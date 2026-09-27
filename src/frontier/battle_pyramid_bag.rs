//! Translated from `src/battle_pyramid_bag.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBgTemplates sListMenuTemplate sMenuActions sMenuActionIds_Field sMenuActionIds_ChooseToss sMenuActionIds_Battle sMenuActionIds_BattleCannotUse sYesNoTossFuncions sTextColors sWindowTemplates sWindowTemplates_MenuActions sOamData_PyramidBag sAnim_PyramidBag sAnims_PyramidBag sAffineAnim_PyramidBag_Still sAffineAnim_PyramidBag_Shake sAffineAnims_PyramidBag sSpriteSheet_PyramidBag sSpriteTemplate_PyramidBag
#[allow(unused_imports)]
use crate::data::battle_pyramid_bag::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPyramidBagMenu: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPyramidBagMenuState: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);

unsafe extern "C" {
    static mut gBagScreen_Gfx: u8;
    static mut gBattlePyramidBagInterface_Pal: u8;
    static mut gBattlePyramidBagTilemap: u8;
    static mut gBattlePyramidBag_Pal: u8;
    static mut gFieldCallback2: u8;
    static mut gMain: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPyramidBagMenu_ReturnToStrings: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_CantWriteMail: u8;
    static mut gText_CloseBag: u8;
    static mut gText_ConfirmTossItems: u8;
    static mut gText_DadsAdvice: u8;
    static mut gText_MoveVar1Where: u8;
    static mut gText_NumberItem_TMBerry: u8;
    static mut gText_ReturnToVar1: u8;
    static mut gText_SelectorArrow2: u8;
    static mut gText_ThrewAwayVar2Var1s: u8;
    static mut gText_TossHowManyVar1s: u8;
    static mut gText_Var1CantBeHeld: u8;
    static mut gText_Var1IsSelected: u8;
    static mut gText_xVar1: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn AddScrollIndicatorArrowPairParameterized(
        a0: u32,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: *mut u16,
    ) -> u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AddWindow(a0: *mut u8) -> u16;
    fn AdjustQuantityAccordingToDPadInput(a0: *mut i16, a1: u16) -> u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ChooseMonToGiveItem();
    fn CB2_FadeFromPartyMenu() -> u8;
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CB2_SetUpReshowBattleScreenAfterMenu2();
    fn ChangeMenuGridCursorPosition(a0: i8, a1: i8) -> u8;
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSwapLineSprites(a0: *mut u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenuWithCallbacks(
        a0: u8,
        a1: *mut u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u16,
        a6: u8,
        a7: *mut u8,
    );
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisplayMessageAndContinueTask(
        a0: u8,
        a1: u8,
        a2: u16,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: *mut u8,
    );
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FadeScreen(a0: u8, a1: i8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetItemBattleFunc(a0: u16) -> Option<unsafe extern "C" fn(u8)>;
    fn GetItemBattleUsage(a0: u16) -> u8;
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemFieldFunc(a0: u16) -> Option<unsafe extern "C" fn(u8)>;
    fn GetItemImportance(a0: u16) -> u8;
    fn GetItemPocket(a0: u16) -> u8;
    fn GetLRKeysPressed() -> u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuActionGrid(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsWritingMailAllowed(a0: u16) -> u8;
    fn ItemIsMail(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuGetYCoordForPrintingArrowCursor(a0: u8) -> u16;
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenuSetTemplateField(a0: u8, a1: u8, a2: i32);
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadListMenuSwapLineGfx();
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuActionGrid(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: *mut u8,
        a8: *mut u8,
    );
    fn PrintMenuActionTexts(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: *mut u8,
        a8: *mut u8,
    );
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemovePyramidBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut u8, a1: i32, a2: *mut u8);
    fn SetSwapLineSpritesInvisibility(a0: *mut u8, a1: u8, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn UpdateSwapLineSpritesPos(a0: *mut u8, a1: u8, a2: i16, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBattlePyramidBagCursorPosition() {
    unsafe {
        (((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>())
        .write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_PyramidBagMenuFromStartMenu() {
    unsafe {
        GoToBattlePyramidBagMenu(0u8, Some(CB2_ReturnToFieldWithOpenMenu));
    }
}
pub(crate) unsafe extern "C" fn OpenBattlePyramidBagInBattle() {
    unsafe {
        GoToBattlePyramidBagMenu(1u8, Some(CB2_SetUpReshowBattleScreenAfterMenu2));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseItemsToTossFromPyramidBag() {
    unsafe {
        LockPlayerFieldControls();
        FadeScreen(1u8, 0i8);
        CreateTask(Some(Task_ChooseItemsToTossFromPyramidBag), 10u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseItemsToTossFromPyramidBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            CleanupOverworldWindowsAndTilemaps();
            ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(CB2_FadeFromPartyMenu));
            GoToBattlePyramidBagMenu(3u8, Some(CB2_ReturnToField));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToPyramidBagMenu() {
    unsafe {
        GoToBattlePyramidBagMenu(
            4u8,
            (((&raw mut gPyramidBagMenuState).cast::<u8>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GoToBattlePyramidBagMenu(
    location: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut location = location;
        let mut exitCallback = exitCallback;
        ((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(2444u32));
        if ((location) as i32) != 4i32 {
            (((&raw mut gPyramidBagMenuState).cast::<u8>()).wrapping_add(4)).write(location);
        }
        if core::mem::transmute::<_, usize>(exitCallback) != 0usize {
            (((&raw mut gPyramidBagMenuState).cast::<u8>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(exitCallback);
        }
        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(None);
        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2068))
            .write(255u8);
        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2070))
            .write(255u8);
        crate::c::memset(
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>(),
            255i32,
            11u32,
        );
        crate::c::memset(
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2063))
            .cast::<u8>(),
            255i32,
            5u32,
        );
        SetMainCallback2(Some(CB2_LoadPyramidBagMenu));
    }
}
pub(crate) unsafe extern "C" fn CB2_PyramidBag() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PyramidBag() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_LoadPyramidBagMenu() {
    unsafe {
        'l1: loop {
            if !(((((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
                && (((LoadPyramidBagMenu()) as i32) != 1i32))
                && (((MenuHelpers_IsLinkActive()) as i32) != 1i32))
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPyramidBagMenu() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32;
            if __sw1 == 0i32 {
                SetVBlankHBlankCallbacksToNull();
                ClearScheduledBgCopiesToVram();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScanlineEffect_Stop();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                FreeAllSpritePalettes();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetPaletteFade();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ResetSpriteData();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((MenuHelpers_IsLinkActive()) != 0) {
                    ResetTasks();
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                InitPyramidBagBgs();
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2436)
                    .cast::<i16>())
                .write(0i16);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (LoadPyramidBagGfx()) != 0 {
                    let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                InitPyramidBagWindows();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                UpdatePyramidBagList();
                UpdatePyramidBagCursorPos();
                InitPyramidBagScroll();
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                SetBagItemsListTemplate();
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                CreatePyramidBagInputTask();
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                CreatePyramidBagSprite();
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                AddScrollArrows();
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                CreateSwapLine();
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 16i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p18).write(((__p18).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VBlankCB_PyramidBag));
                SetMainCallback2(Some(CB2_PyramidBag));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitPyramidBagBgs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(2u8);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        SetGpuReg(80u8, 0u16);
    }
}
pub(crate) unsafe extern "C" fn LoadPyramidBagGfx() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2436)
                .cast::<i16>())
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                ResetTempTileDataBuffers();
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw mut gBagScreen_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                let __p2 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2436)
                    .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LZDecompressWram(
                        ((&raw mut gBattlePyramidBagTilemap).cast::<u32>()).cast::<u32>(),
                        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u8>(),
                    );
                    let __p3 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2436)
                        .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadCompressedPalette(
                    ((&raw mut gBattlePyramidBagInterface_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    32u16,
                );
                let __p4 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2436)
                    .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadCompressedSpriteSheet(
                    (&raw const sSpriteSheet_PyramidBag).cast::<u8>().cast_mut(),
                );
                let __p5 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2436)
                    .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadPyramidBagPalette();
                let __p6 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2436)
                    .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                LoadListMenuSwapLineGfx();
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2436)
                    .cast::<i16>())
                .write(0i16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetBagItemsListTemplate() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut itemIds: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2081))
                    .read()) as i32)
                        .wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    CopyBagItemName(
                        ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2172))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u8>(),
                        ((itemIds).wrapping_offset(((i) as i32) as isize)).read(),
                    );
                    (((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2172))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u8>(),
                    );
                    (((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2084))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(4)
                    .cast::<i32>())
                    .write(((i) as i32));
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2172))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 24))
            .cast::<u8>(),
            (&raw mut gText_CloseBag).cast::<u8>(),
        );
        (((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2084))
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write(
            ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2172))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 24))
            .cast::<u8>(),
        );
        (((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2084))
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sListMenuTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(
            ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2081))
            .read()) as u16),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2084))
            .cast::<u8>(),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(
            ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2082))
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CopyBagItemName(dst: *mut u8, itemId: u16) {
    unsafe {
        let mut dst = dst;
        let mut itemId = itemId;
        if ((GetItemPocket(itemId)) as i32) == 4i32 {
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                (((itemId) as i32).wrapping_sub(133i32)).wrapping_add(1i32),
                2i32,
                2u8,
            );
            CopyItemName(itemId, (&raw mut gStringVar2).cast::<u8>());
            StringExpandPlaceholders(dst, (&raw mut gText_NumberItem_TMBerry).cast::<u8>());
        } else {
            CopyItemName(itemId, dst);
        }
    }
}
pub(crate) unsafe extern "C" fn BagCursorMoved(itemIndex: i32, onInit: u8, list: *mut u8) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut onInit = onInit;
        let mut list = list;
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
            ShakePyramidBag();
        }
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2068))
        .read()) as i32)
            == 255i32
        {
            FreeItemIconSpriteByAltId(
                ((((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2069))
                .read()) as i32)
                    ^ 1i32) as u8),
            );
            if itemIndex != (-2i32) {
                ShowItemIcon(
                    (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1612))
                    .wrapping_add(2016))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read(
                            ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(1629),
                            0,
                            2,
                            false,
                        ) as u8) as i32) as isize
                            * 20,
                    ))
                    .cast::<u16>())
                    .wrapping_offset((itemIndex) as isize))
                    .read(),
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2069))
                    .read(),
                );
            } else {
                ShowItemIcon(
                    65535u16,
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2069))
                    .read(),
                );
            }
            let __p1 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2069);
            (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
            PrintItemDescription(itemIndex);
        }
    }
}
pub(crate) unsafe extern "C" fn PrintItemQuantity(windowId: u8, itemIndex: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemIndex = itemIndex;
        let mut y = y;
        let mut xAlign: i32 = 0i32;
        if itemIndex == 4294967294u32 {
            return;
        }
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2068))
        .read()) as i32)
            != 255i32
        {
            if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2068))
            .read()) as i32)
                == (((itemIndex) as u8) as i32)
            {
                PrintSelectorArrowAtPos(y, 1u8);
            } else {
                PrintSelectorArrowAtPos(y, 255u8);
            }
        }
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1612))
            .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>())
            .wrapping_offset(((itemIndex) as i32) as isize))
            .read()) as i32),
            1i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        xAlign = GetStringRightAlignXOffset(7i32, (&raw mut gStringVar4).cast::<u8>(), 119i32);
        PyramidBagPrint_Quantity(
            windowId,
            (&raw mut gStringVar4).cast::<u8>(),
            ((xAlign) as u8),
            y,
            0u8,
            0u8,
            255u8,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintItemDescription(listMenuId: i32) {
    unsafe {
        let mut listMenuId = listMenuId;
        let mut desc: *mut u8 = core::ptr::null_mut();
        if listMenuId != (-2i32) {
            desc = GetItemDescription(
                (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1612))
                .wrapping_add(2016))
                .cast::<u8>())
                .wrapping_offset(
                    ((crate::c::bf_read(
                        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                            .wrapping_add(1629),
                        0,
                        2,
                        false,
                    ) as u8) as i32) as isize
                        * 20,
                ))
                .cast::<u16>())
                .wrapping_offset((listMenuId) as isize))
                .read(),
            );
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut gPyramidBagMenu_ReturnToStrings).cast::<*mut u8>())
                    .cast::<*mut u8>())
                .wrapping_offset(
                    (((((&raw mut gPyramidBagMenuState).cast::<u8>()).wrapping_add(4)).read())
                        as i32) as isize,
                ))
                .read(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_ReturnToVar1).cast::<u8>(),
            );
            desc = (&raw mut gStringVar4).cast::<u8>();
        }
        FillWindowPixelBuffer(1u8, 0u8);
        PyramidBagPrint(1u8, desc, 3u8, 0u8, 0u8, 1u8, 0u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn AddScrollArrows() {
    unsafe {
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2070))
        .read()) as i32)
            == 255i32
        {
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2070))
            .write(AddScrollIndicatorArrowPairParameterized(
                2u32,
                172i32,
                12i32,
                148i32,
                ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2081))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2082))
                        .read()) as i32),
                    ),
                2910i32,
                2910i32,
                ((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>(),
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn RemoveScrollArrow() {
    unsafe {
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2070))
        .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2070))
                .read(),
            );
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2070))
            .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePyramidBagInputTask() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_HandlePyramidBagInput), 0u8);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>())
                .read(),
                (((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read(),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SwapItems(id1: u8, id2: u8) {
    unsafe {
        let mut id1 = id1;
        let mut id2 = id2;
        let mut temp: u16 = 0u16;
        let mut itemIds: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        {
            temp = ((itemIds).wrapping_offset(((id1) as i32) as isize)).read();
            ((itemIds).wrapping_offset(((id1) as i32) as isize))
                .write(((itemIds).wrapping_offset(((id2) as i32) as isize)).read());
            ((itemIds).wrapping_offset(((id2) as i32) as isize)).write(temp);
        }
        {
            temp = ((((quantities).wrapping_offset(((id1) as i32) as isize)).read()) as u16);
            ((quantities).wrapping_offset(((id1) as i32) as isize))
                .write(((quantities).wrapping_offset(((id2) as i32) as isize)).read());
            ((quantities).wrapping_offset(((id2) as i32) as isize)).write(((temp) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn MovePyramidBagItemSlotInList(from: u8, to: u8) {
    unsafe {
        let mut from = from;
        let mut to = to;
        let mut itemIds: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        if ((from) as i32) != ((to) as i32) {
            let mut i: i16 = 0i16;
            let mut firstSlotItemId: u16 =
                ((itemIds).wrapping_offset(((from) as i32) as isize)).read();
            let mut firstSlotQuantity: u8 =
                ((quantities).wrapping_offset(((from) as i32) as isize)).read();
            if ((to) as i32) > ((from) as i32) {
                to = (to).wrapping_sub(1);
                {
                    i = ((from) as i16);
                    'l1: loop {
                        if !(((i) as i32) < ((to) as i32)) {
                            break 'l1;
                        }
                        'l2: {
                            ((itemIds).wrapping_offset(((i) as i32) as isize)).write(
                                ((itemIds)
                                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                                .read(),
                            );
                            ((quantities).wrapping_offset(((i) as i32) as isize)).write(
                                ((quantities)
                                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            } else {
                {
                    i = ((from) as i16);
                    'l3: loop {
                        if !(((i) as i32) > ((to) as i32)) {
                            break 'l3;
                        }
                        'l4: {
                            ((itemIds).wrapping_offset(((i) as i32) as isize)).write(
                                ((itemIds)
                                    .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                                .read(),
                            );
                            ((quantities).wrapping_offset(((i) as i32) as isize)).write(
                                ((quantities)
                                    .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                                .read(),
                            );
                        }
                        i = (i).wrapping_sub(1);
                    }
                }
            }
            ((itemIds).wrapping_offset(((to) as i32) as isize)).write(firstSlotItemId);
            ((quantities).wrapping_offset(((to) as i32) as isize)).write(firstSlotQuantity);
        }
    }
}
pub(crate) unsafe extern "C" fn CompactItems() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut itemIds: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        let mut quantities: *mut u8 =
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((itemIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        == 0i32)
                        || (((((quantities).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            == 0i32)
                    {
                        ((itemIds).wrapping_offset(((i) as i32) as isize)).write(0u16);
                        ((quantities).wrapping_offset(((i) as i32) as isize)).write(0u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u8);
                        'l5: loop {
                            if !(((j) as i32) < 10i32) {
                                break 'l5;
                            }
                            'l6: {
                                if (((((itemIds).wrapping_offset(((i) as i32) as isize)).read())
                                    as i32)
                                    == 0i32)
                                    || (((((quantities).wrapping_offset(((i) as i32) as isize))
                                        .read()) as i32)
                                        == 0i32)
                                {
                                    SwapItems(i, j);
                                }
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePyramidBagList() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut itemIds: *mut u16 = (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1612))
        .wrapping_add(2016))
        .cast::<u8>())
        .wrapping_offset(
            ((crate::c::bf_read(
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1629),
                0,
                2,
                false,
            ) as u8) as i32) as isize
                * 20,
        ))
        .cast::<u16>();
        CompactItems();
        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2081))
            .write(0u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((itemIds).wrapping_offset(((i) as i32) as isize)).read()) as i32) != 0i32
                    {
                        let __p1 = (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2081);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 =
            (((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2081);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2081))
        .read()) as i32)
            > 8i32
        {
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2082))
            .write(8u8);
        } else {
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2082))
            .write(
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2081))
                .read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePyramidBagCursorPos() {
    unsafe {
        if ((((((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>())
        .read()) as i32)
            != 0i32)
            && ((((((&raw mut gPyramidBagMenuState).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2082))
                    .read()) as i32),
                )
                > ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2081))
                .read()) as i32))
        {
            (((&raw mut gPyramidBagMenuState).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .write(
                ((((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2081))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2082))
                        .read()) as i32),
                    )) as u16),
            );
        }
        if (((((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32),
            )
            >= ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2081))
            .read()) as i32)
        {
            if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2081))
            .read()) as i32)
                == 0i32
            {
                (((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(0u16);
            } else {
                (((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(
                    ((((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2081))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPyramidBagScroll() {
    unsafe {
        let mut i: u8 = 0u8;
        if (((((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            > 4i32
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32)
                        <= (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(4i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        if (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2082))
                                .read()) as i32),
                            )
                            == ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2081))
                            .read()) as i32)
                        {
                            break 'l1;
                        }
                        let __p1 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>();
                        (__p1).write(((__p1).read()).wrapping_sub(1));
                        let __p2 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<u16>();
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintSelectorArrow(listMenuTaskId: u8, colorId: u8) {
    unsafe {
        let mut listMenuTaskId = listMenuTaskId;
        let mut colorId = colorId;
        let mut y: u8 = ((ListMenuGetYCoordForPrintingArrowCursor(listMenuTaskId)) as u8);
        PrintSelectorArrowAtPos(y, colorId);
    }
}
pub(crate) unsafe extern "C" fn PrintSelectorArrowAtPos(y: u8, colorId: u8) {
    unsafe {
        let mut y = y;
        let mut colorId = colorId;
        if ((colorId) as i32) == 255i32 {
            FillWindowPixelRect(
                0u8,
                0u8,
                0u16,
                ((y) as u16),
                ((GetMenuCursorDimensionByFont(1u8, 0u8)) as u16),
                ((GetMenuCursorDimensionByFont(1u8, 1u8)) as u16),
            );
        } else {
            PyramidBagPrint(
                0u8,
                (&raw mut gText_SelectorArrow2).cast::<u8>(),
                0u8,
                y,
                0u8,
                0u8,
                0u8,
                colorId,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseBattlePyramidBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ClosePyramidBag));
    }
}
pub(crate) unsafe extern "C" fn Task_ClosePyramidBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyListMenuTask(
                (((data).read()) as u8),
                ((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>(),
                ((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>(),
            );
            if core::mem::transmute::<_, usize>(
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                SetMainCallback2(
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            } else {
                SetMainCallback2(
                    (((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            }
            RemoveScrollArrow();
            ResetSpriteData();
            FreeAllSpritePalettes();
            FreeAllWindowBuffers();
            Free(((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read());
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePyramidBagInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32)
            || ((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0)
        {
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 4i32)
            != 0
        {
            if (((((&raw mut gPyramidBagMenuState).cast::<u8>()).wrapping_add(4)).read()) as i32)
                != 2i32
            {
                ListMenuGetScrollAndRow(
                    (((data).read()) as u8),
                    ((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u16>(),
                    ((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>(),
                );
                if (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(
                        (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .read()) as i32),
                    )
                    != ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2081))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    PlaySE(5u16);
                    Task_BeginItemSwap(taskId);
                }
            }
        } else {
            let mut listId: i32 = ListMenu_ProcessInput((((data).read()) as u8));
            ListMenuGetScrollAndRow(
                (((data).read()) as u8),
                ((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<u16>(),
                ((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>(),
            );
            'l1: {
                let __sw1 = listId;
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    PlaySE(5u16);
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(0u16);
                    CloseBattlePyramidBag(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(
                        (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2016))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1629),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 20,
                        ))
                        .cast::<u16>())
                        .wrapping_offset((listId) as isize))
                        .read(),
                    );
                    ((data).wrapping_offset(1)).write(((listId) as i16));
                    ((data).wrapping_offset(2)).write(
                        ((((((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1612))
                        .wrapping_add(2016))
                        .wrapping_add(40))
                        .cast::<u8>())
                        .wrapping_offset(
                            ((crate::c::bf_read(
                                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(1612))
                                .wrapping_add(1629),
                                0,
                                2,
                                false,
                            ) as u8) as i32) as isize
                                * 10,
                        ))
                        .cast::<u8>())
                        .wrapping_offset((listId) as isize))
                        .read()) as i16),
                    );
                    if (((((&raw mut gPyramidBagMenuState).cast::<u8>()).wrapping_add(4)).read())
                        as i32)
                        == 2i32
                    {
                        TryCloseBagToGiveItem(taskId);
                    } else {
                        OpenContextMenu(taskId);
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn OpenContextMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RemoveScrollArrow();
        PrintSelectorArrow((((data).read()) as u8), 1u8);
        'l1: {
            let __sw1 =
                (((((&raw mut gPyramidBagMenuState).cast::<u8>()).wrapping_add(4)).read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 3i32;
            if !__matched {
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2072)
                    .cast::<*mut u8>())
                .write(((&raw const sMenuActionIds_Field).cast::<u8>().cast_mut()).cast::<u8>());
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2080))
                .write(((crate::c::div_u32(4u32, 1u32)) as u8));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((GetItemBattleUsage(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()))
                    as i32)
                    != 0i32
                {
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2072)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sMenuActionIds_Battle).cast::<u8>().cast_mut()).cast::<u8>(),
                    );
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080))
                    .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                } else {
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2072)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sMenuActionIds_BattleCannotUse)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080))
                    .write(((crate::c::div_u32(1u32, 1u32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2072)
                    .cast::<*mut u8>())
                .write(
                    ((&raw const sMenuActionIds_ChooseToss)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2080))
                .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                break 'l1;
            }
        }
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Var1IsSelected).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        PyramidBagPrint(
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            0u8,
            0u8,
            1u8,
            0u8,
            0u8,
        );
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2080))
        .read()) as i32)
            == 1i32
        {
            PrintMenuActionText_SingleRow(OpenMenuActionWindowById(0u8));
        } else {
            if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2080))
            .read()) as i32)
                == 2i32
            {
                PrintMenuActionText_SingleRow(OpenMenuActionWindowById(1u8));
            } else {
                PrintMenuActionText_MultiRow(OpenMenuActionWindowById(2u8), 2u8, 2u8);
            }
        }
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2080))
        .read()) as i32)
            == 4i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(HandleMenuActionInput_2x2));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(HandleMenuActionInput_SingleRow));
        }
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionText_SingleRow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        PrintMenuActionTexts(
            windowId,
            7u8,
            8u8,
            1u8,
            0u8,
            16u8,
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2080))
            .read(),
            ((&raw const sMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2072)
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuInUpperLeftCornerNormal(
            windowId,
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2080))
            .read(),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionText_MultiRow(
    windowId: u8,
    horizontalCount: u8,
    verticalCount: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut horizontalCount = horizontalCount;
        let mut verticalCount = verticalCount;
        PrintMenuActionGrid(
            windowId,
            7u8,
            8u8,
            1u8,
            56u8,
            horizontalCount,
            verticalCount,
            ((&raw const sMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2072)
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuActionGrid(windowId, 56u8, horizontalCount, verticalCount, 0u8);
    }
}
pub(crate) unsafe extern "C" fn HandleMenuActionInput_SingleRow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            let mut id: i32 = ((Menu_ProcessInputNoWrap()) as i32);
            'l1: {
                let __sw1 = id;
                let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
                if __sw1 == (-2i32) {
                    break 'l1;
                }
                if __sw1 == (-1i32) {
                    PlaySE(5u16);
                    (((((((&raw const sMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(24))
                    .wrapping_add(4))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    if core::mem::transmute::<_, usize>(
                        ((((((&raw const sMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut gPyramidBagMenu)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2072)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((id) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .wrapping_add(4))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .read(),
                    ) != 0usize
                    {
                        (((((((&raw const sMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut gPyramidBagMenu)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(2072)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset((id) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .wrapping_add(4))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .read())
                        .unwrap_unchecked()(taskId);
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HandleMenuActionInput_2x2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            let mut id: i8 = ((Menu_GetCursorPos()) as i8);
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                if (((id) as i32) > 0i32)
                    && ((IsValidMenuAction(((((id) as i32).wrapping_sub(2i32)) as i8))) != 0)
                {
                    PlaySE(5u16);
                    ChangeMenuGridCursorPosition(0i8, (-1i8));
                }
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 128i32)
                    != 0
                {
                    if (((id) as i32)
                        < ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2080))
                        .read()) as i32)
                            .wrapping_sub(2i32))
                        && ((IsValidMenuAction(((((id) as i32).wrapping_add(2i32)) as i8))) != 0)
                    {
                        PlaySE(5u16);
                        ChangeMenuGridCursorPosition(0i8, 1i8);
                    }
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 32i32)
                        != 0)
                        || (((GetLRKeysPressed()) as i32) == 1i32)
                    {
                        if ((((id) as i32) & 1i32) != 0)
                            && ((IsValidMenuAction(((((id) as i32).wrapping_sub(1i32)) as i8)))
                                != 0)
                        {
                            PlaySE(5u16);
                            ChangeMenuGridCursorPosition((-1i8), 0i8);
                        }
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 16i32)
                            != 0)
                            || (((GetLRKeysPressed()) as i32) == 2i32)
                        {
                            if (!((((id) as i32) & 1i32) != 0))
                                && ((IsValidMenuAction(((((id) as i32).wrapping_add(1i32)) as i8)))
                                    != 0)
                            {
                                PlaySE(5u16);
                                ChangeMenuGridCursorPosition(1i8, 0i8);
                            }
                        } else {
                            if ((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 1i32)
                                != 0
                            {
                                PlaySE(5u16);
                                if core::mem::transmute::<_, usize>(
                                    ((((((&raw const sMenuActions).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((((&raw mut gPyramidBagMenu)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(2072)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((id) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .wrapping_add(4))
                                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                                    .read(),
                                ) != 0usize
                                {
                                    (((((((&raw const sMenuActions).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(
                                        ((((((((&raw mut gPyramidBagMenu)
                                            .cast::<u8>()
                                            .cast::<*mut u8>())
                                        .read())
                                        .wrapping_add(2072)
                                        .cast::<*mut u8>())
                                        .read())
                                        .wrapping_offset(((id) as i32) as isize))
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .wrapping_add(4))
                                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                                    .read())
                                    .unwrap_unchecked()(taskId);
                                }
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 2i32)
                                    != 0
                                {
                                    PlaySE(5u16);
                                    (((((((&raw const sMenuActions).cast::<u8>().cast_mut())
                                        .cast::<u8>())
                                    .wrapping_offset(24))
                                    .wrapping_add(4))
                                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                                    .read())
                                    .unwrap_unchecked()(taskId);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn IsValidMenuAction(actionTableId: i8) -> u8 {
    unsafe {
        let mut actionTableId = actionTableId;
        if ((actionTableId) as i32) < 0i32 {
            return 0u8;
        } else {
            if ((actionTableId) as i32)
                > ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2080))
                .read()) as i32)
            {
                return 0u8;
            } else {
                if ((((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2072)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((actionTableId) as i32) as isize))
                .read()) as i32)
                    == 5i32
                {
                    return 0u8;
                } else {
                    return 1u8;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CloseMenuActionWindow() {
    unsafe {
        if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2080))
        .read()) as i32)
            == 1i32
        {
            CloseMenuActionWindowById(0u8);
        } else {
            if ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2080))
            .read()) as i32)
                == 2i32
            {
                CloseMenuActionWindowById(1u8);
            } else {
                CloseMenuActionWindowById(2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BagAction_UseOnField(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut pocketId: u8 = GetItemPocket(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
        if (((((pocketId) as i32) == 5i32) || (((pocketId) as i32) == 2i32))
            || (((pocketId) as i32) == 3i32))
            || (((ItemIsMail(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32) == 1i32)
        {
            CloseMenuActionWindow();
            DisplayItemMessageInBattlePyramid(
                taskId,
                (&raw mut gText_DadsAdvice).cast::<u8>(),
                Some(Task_CloseBattlePyramidBagMessage),
            );
        } else {
            if core::mem::transmute::<_, usize>(GetItemFieldFunc(
                ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            )) != 0usize
            {
                CloseMenuActionWindow();
                FillWindowPixelBuffer(1u8, 0u8);
                ScheduleBgCopyTilemapToVram(0u8);
                (GetItemFieldFunc(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()))
                    .unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BagAction_Cancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CloseMenuActionWindow();
        PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        PrintSelectorArrow((((data).read()) as u8), 0u8);
        SetTaskToMainPyramidBagInputHandler(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetTaskToMainPyramidBagInputHandler(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        AddScrollArrows();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandlePyramidBagInput));
    }
}
pub(crate) unsafe extern "C" fn BagAction_Toss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CloseMenuActionWindow();
        ((data).wrapping_offset(8)).write(1i16);
        if ((((data).wrapping_offset(2)).read()) as i32) == 1i32 {
            AskConfirmToss(taskId);
        } else {
            CopyItemName(
                ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_TossHowManyVar1s).cast::<u8>(),
            );
            FillWindowPixelBuffer(1u8, 0u8);
            PyramidBagPrint(
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                3u8,
                0u8,
                0u8,
                1u8,
                0u8,
                0u8,
            );
            ShowNumToToss();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ChooseHowManyToToss));
        }
    }
}
pub(crate) unsafe extern "C" fn AskConfirmToss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((data).wrapping_offset(8)).read()) as i32),
            0i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ConfirmTossItems).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        PyramidBagPrint(
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            0u8,
            0u8,
            1u8,
            0u8,
            0u8,
        );
        CreatePyramidBagYesNo(
            taskId,
            (&raw const sYesNoTossFuncions).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn DontTossItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
        PrintSelectorArrow((((data).read()) as u8), 0u8);
        SetTaskToMainPyramidBagInputHandler(taskId);
    }
}
pub(crate) unsafe extern "C" fn ShowNumToToss() {
    unsafe {
        let mut x: i32 = 0i32;
        ConvertIntToDecimalStringN((&raw mut gStringVar1).cast::<u8>(), 1i32, 2i32, 2u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        DrawTossNumberWindow(3u8);
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 40i32);
        AddTextPrinterParameterized(
            3u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            2u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateNumToToss(num: i16) {
    unsafe {
        let mut num = num;
        let mut x: i32 = 0i32;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((num) as i32),
            2i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        x = GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 40i32);
        AddTextPrinterParameterized(
            3u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            2u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseHowManyToToss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((AdjustQuantityAccordingToDPadInput(
            (data).wrapping_offset(8),
            ((((data).wrapping_offset(2)).read()) as u16),
        )) as i32)
            == 1i32
        {
            UpdateNumToToss(((data).wrapping_offset(8)).read());
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                PlaySE(5u16);
                ClearStdWindowAndFrameToTransparent(3u8, 0u8);
                ClearWindowTilemap(3u8);
                ScheduleBgCopyTilemapToVram(1u8);
                AskConfirmToss(taskId);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(5u16);
                    ClearStdWindowAndFrameToTransparent(3u8, 0u8);
                    ClearWindowTilemap(3u8);
                    ScheduleBgCopyTilemapToVram(1u8);
                    DontTossItem(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TossItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((data).wrapping_offset(8)).read()) as i32),
            0i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ThrewAwayVar2Var1s).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        PyramidBagPrint(
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            0u8,
            0u8,
            1u8,
            0u8,
            0u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_TossItem));
    }
}
pub(crate) unsafe extern "C" fn Task_TossItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollOffset: *mut u16 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>();
        let mut selectedRow: *mut u16 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            PlaySE(5u16);
            RemovePyramidBagItem(
                ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
                ((((data).wrapping_offset(8)).read()) as u16),
            );
            DestroyListMenuTask((((data).read()) as u8), scrollOffset, selectedRow);
            UpdatePyramidBagList();
            UpdatePyramidBagCursorPos();
            SetBagItemsListTemplate();
            (data).write(
                ((ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    (scrollOffset).read(),
                    (selectedRow).read(),
                )) as i16),
            );
            ScheduleBgCopyTilemapToVram(0u8);
            SetTaskToMainPyramidBagInputHandler(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn BagAction_Give(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CloseMenuActionWindow();
        if ((ItemIsMail(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32) == 1i32 {
            DisplayItemMessageInBattlePyramid(
                taskId,
                (&raw mut gText_CantWriteMail).cast::<u8>(),
                Some(Task_WaitCloseErrorMessage),
            );
        } else {
            if !((GetItemImportance(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) != 0) {
                ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_ChooseMonToGiveItem));
                CloseBattlePyramidBag(taskId);
            } else {
                ShowCantHoldMessage(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowCantHoldMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Var1CantBeHeld).cast::<u8>(),
        );
        DisplayItemMessageInBattlePyramid(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(Task_WaitCloseErrorMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_WaitCloseErrorMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            Task_CloseBattlePyramidBagMessage(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_CloseBattlePyramidBagMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CloseBattlePyramidBagTextWindow();
        PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
        PrintSelectorArrow((((data).read()) as u8), 0u8);
        SetTaskToMainPyramidBagInputHandler(taskId);
    }
}
pub(crate) unsafe extern "C" fn TryCloseBagToGiveItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsWritingMailAllowed(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) != 0) {
            DisplayItemMessageInBattlePyramid(
                taskId,
                (&raw mut gText_CantWriteMail).cast::<u8>(),
                Some(Task_WaitCloseErrorMessage),
            );
        } else {
            if !((GetItemImportance(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) != 0) {
                CloseBattlePyramidBag(taskId);
            } else {
                ShowCantHoldMessage(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BagAction_UseInBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if core::mem::transmute::<_, usize>(GetItemBattleFunc(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
        )) != 0usize
        {
            CloseMenuActionWindow();
            (GetItemBattleFunc(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()))
                .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BeginItemSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(1)).write(
            (((((((&raw mut gPyramidBagMenuState).cast::<u8>())
                .wrapping_add(8)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(
                    (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .read()) as i32),
                )) as i16),
        );
        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2068))
            .write(((((data).wrapping_offset(1)).read()) as u8));
        ListMenuSetTemplateField((((data).read()) as u8), 16u8, 1i32);
        CopyItemName(
            (((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 20,
            ))
            .cast::<u16>())
            .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize))
            .read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_MoveVar1Where).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        PyramidBagPrint(
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            0u8,
            0u8,
            1u8,
            0u8,
            0u8,
        );
        PrintSelectorArrow((((data).read()) as u8), 1u8);
        UpdateSwapLinePos(((((data).wrapping_offset(1)).read()) as u8));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ItemSwapHandleInput));
    }
}
pub(crate) unsafe extern "C" fn Task_ItemSwapHandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 4i32)
                != 0
            {
                PlaySE(5u16);
                ListMenuGetScrollAndRow(
                    (((data).read()) as u8),
                    ((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u16>(),
                    ((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>(),
                );
                PerformItemSwap(taskId);
            } else {
                let mut id: i32 = ListMenu_ProcessInput((((data).read()) as u8));
                ListMenuGetScrollAndRow(
                    (((data).read()) as u8),
                    ((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u16>(),
                    ((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>(),
                );
                SetSwapLineInvisibility(0u8);
                UpdateSwapLinePos(
                    (((((&raw mut gPyramidBagMenuState).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>())
                    .read()) as u8),
                );
                'l1: {
                    let __sw1 = id;
                    let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                    if __sw1 == (-1i32) {
                        break 'l1;
                    }
                    if __sw1 == (-2i32) {
                        PlaySE(5u16);
                        if ((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 1i32)
                            != 0
                        {
                            PerformItemSwap(taskId);
                        } else {
                            CancelItemSwap(taskId);
                        }
                        break 'l1;
                    }
                    if !__matched {
                        PlaySE(5u16);
                        PerformItemSwap(taskId);
                        break 'l1;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PerformItemSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollOffset: *mut u16 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>();
        let mut selectedRow: *mut u16 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>();
        let mut swapPos: u16 = (((((scrollOffset).read()) as i32)
            .wrapping_add((((selectedRow).read()) as i32))) as u16);
        if (((((data).wrapping_offset(1)).read()) as i32) == ((swapPos) as i32))
            || (((((data).wrapping_offset(1)).read()) as i32)
                == ((swapPos) as i32).wrapping_sub(1i32))
        {
            CancelItemSwap(taskId);
        } else {
            MovePyramidBagItemSlotInList(
                ((((data).wrapping_offset(1)).read()) as u8),
                ((swapPos) as u8),
            );
            ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2068))
            .write(255u8);
            SetSwapLineInvisibility(1u8);
            DestroyListMenuTask((((data).read()) as u8), scrollOffset, selectedRow);
            if ((((data).wrapping_offset(1)).read()) as i32) < ((swapPos) as i32) {
                let __p1 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            SetBagItemsListTemplate();
            (data).write(
                ((ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    (scrollOffset).read(),
                    (selectedRow).read(),
                )) as i16),
            );
            SetTaskToMainPyramidBagInputHandler(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CancelItemSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollOffset: *mut u16 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(8)
            .cast::<u16>();
        let mut selectedRow: *mut u16 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>();
        ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2068))
            .write(255u8);
        SetSwapLineInvisibility(1u8);
        DestroyListMenuTask((((data).read()) as u8), scrollOffset, selectedRow);
        if ((((data).wrapping_offset(1)).read()) as i32)
            < (((scrollOffset).read()) as i32).wrapping_add((((selectedRow).read()) as i32))
        {
            let __p1 = ((&raw mut gPyramidBagMenuState).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        SetBagItemsListTemplate();
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (scrollOffset).read(),
                (selectedRow).read(),
            )) as i16),
        );
        SetTaskToMainPyramidBagInputHandler(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryStoreHeldItemsInPyramidBag() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut party: *mut u8 = (&raw mut gPlayerParty).cast::<u8>();
        let mut newItems: *mut u16 = (Alloc(20u32)).cast::<u16>();
        let mut newQuantities: *mut u8 = Alloc(10u32);
        let mut heldItem: u16 = 0u16;
        crate::c::memcpy(
            (newItems).cast::<u8>(),
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 20,
            ))
            .cast::<u16>())
            .cast::<u8>(),
            20u32,
        );
        crate::c::memcpy(
            newQuantities,
            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                .wrapping_add(2016))
            .wrapping_add(40))
            .cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32) as isize
                    * 10,
            ))
            .cast::<u8>(),
            10u32,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    heldItem =
                        ((GetMonData2((party).wrapping_offset(((i) as i32) as isize * 100), 12i32))
                            as u16);
                    if (((heldItem) as i32) != 0i32) && (!((AddBagItem(heldItem, 1u16)) != 0)) {
                        crate::c::memcpy(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2016))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1629),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 20,
                            ))
                            .cast::<u16>())
                            .cast::<u8>(),
                            (newItems).cast::<u8>(),
                            20u32,
                        );
                        crate::c::memcpy(
                            ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1612))
                            .wrapping_add(2016))
                            .wrapping_add(40))
                            .cast::<u8>())
                            .wrapping_offset(
                                ((crate::c::bf_read(
                                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(1612))
                                    .wrapping_add(1629),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32) as isize
                                    * 10,
                            ))
                            .cast::<u8>(),
                            newQuantities,
                            10u32,
                        );
                        Free((newItems).cast::<u8>());
                        Free(newQuantities);
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        heldItem = 0u16;
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l3;
                }
                'l4: {
                    SetMonData(
                        (party).wrapping_offset(((i) as i32) as isize * 100),
                        12i32,
                        (&raw mut heldItem).cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        Free((newItems).cast::<u8>());
        Free(newQuantities);
    }
}
pub(crate) unsafe extern "C" fn InitPyramidBagWindows() {
    unsafe {
        let mut i: u8 = 0u8;
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 1u16, 224u8);
        LoadMessageBoxGfx(0u8, 10u16, 208u8);
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            240u16,
            32u16,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(40u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        PutWindowTilemap(0u8);
        PutWindowTilemap(1u8);
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn PyramidBagPrint(
    windowId: u8,
    src: *mut u8,
    x: u8,
    y: u8,
    letterSpacing: u8,
    lineSpacing: u8,
    speed: u8,
    colorTableId: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut src = src;
        let mut x = x;
        let mut y = y;
        let mut letterSpacing = letterSpacing;
        let mut lineSpacing = lineSpacing;
        let mut speed = speed;
        let mut colorTableId = colorTableId;
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            x,
            y,
            letterSpacing,
            lineSpacing,
            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorTableId) as i32) as isize * 3))
            .cast::<u8>(),
            ((speed) as i8),
            src,
        );
    }
}
pub(crate) unsafe extern "C" fn PyramidBagPrint_Quantity(
    windowId: u8,
    src: *mut u8,
    x: u8,
    y: u8,
    letterSpacing: u8,
    lineSpacing: u8,
    speed: u8,
    colorTableId: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut src = src;
        let mut x = x;
        let mut y = y;
        let mut letterSpacing = letterSpacing;
        let mut lineSpacing = lineSpacing;
        let mut speed = speed;
        let mut colorTableId = colorTableId;
        AddTextPrinterParameterized4(
            windowId,
            7u8,
            x,
            y,
            letterSpacing,
            lineSpacing,
            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorTableId) as i32) as isize * 3))
            .cast::<u8>(),
            ((speed) as i8),
            src,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawTossNumberWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        DrawStdFrameWithCustomTileAndPalette(windowId, 0u8, 1u16, 14u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn GetMenuActionWindowId(windowArrayId: u8) -> u8 {
    unsafe {
        let mut windowArrayId = windowArrayId;
        return ((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2063))
        .cast::<u8>())
        .wrapping_offset(((windowArrayId) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn OpenMenuActionWindowById(windowArrayId: u8) -> u8 {
    unsafe {
        let mut windowArrayId = windowArrayId;
        let mut windowId: *mut u8 =
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2063))
            .cast::<u8>())
            .wrapping_offset(((windowArrayId) as i32) as isize);
        if (((windowId).read()) as i32) == 255i32 {
            (windowId).write(
                ((AddWindow(
                    (((&raw const sWindowTemplates_MenuActions)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((windowArrayId) as i32) as isize * 8),
                )) as u8),
            );
            DrawStdFrameWithCustomTileAndPalette((windowId).read(), 0u8, 1u16, 14u8);
            ScheduleBgCopyTilemapToVram(1u8);
        }
        return (windowId).read();
    }
}
pub(crate) unsafe extern "C" fn CloseMenuActionWindowById(windowArrayId: u8) {
    unsafe {
        let mut windowArrayId = windowArrayId;
        let mut windowId: *mut u8 =
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2063))
            .cast::<u8>())
            .wrapping_offset(((windowArrayId) as i32) as isize);
        if (((windowId).read()) as i32) != 255i32 {
            ClearStdWindowAndFrameToTransparent((windowId).read(), 0u8);
            ClearWindowTilemap((windowId).read());
            RemoveWindow((windowId).read());
            ScheduleBgCopyTilemapToVram(1u8);
            (windowId).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePyramidBagYesNo(taskId: u8, yesNoTable: *mut u8) {
    unsafe {
        let mut taskId = taskId;
        let mut yesNoTable = yesNoTable;
        CreateYesNoMenuWithCallbacks(
            taskId,
            (((&raw const sWindowTemplates_MenuActions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(32),
            1u8,
            0u8,
            2u8,
            1u16,
            14u8,
            yesNoTable,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayItemMessageInBattlePyramid(
    taskId: u8,
    str: *mut u8,
    callback: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut str = str;
        let mut callback = callback;
        FillWindowPixelBuffer(2u8, 17u8);
        DisplayMessageAndContinueTask(
            taskId,
            2u8,
            10u16,
            13u8,
            1u8,
            GetPlayerTextSpeedDelay(),
            str,
            core::mem::transmute::<_, *mut u8>(callback),
        );
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn CloseBattlePyramidBagTextWindow() {
    unsafe {
        ClearDialogWindowAndFrameToTransparent(2u8, 0u8);
        ClearWindowTilemap(2u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn FreeItemIconSprite(spriteArrId: u8) {
    unsafe {
        let mut spriteArrId = spriteArrId;
        let mut spriteId: *mut u8 =
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>())
            .wrapping_offset(((spriteArrId) as i32) as isize);
        if (((spriteId).read()) as i32) != 255i32 {
            FreeSpriteTilesByTag((((4132i32).wrapping_add(((spriteArrId) as i32))) as u16));
            FreeSpritePaletteByTag((((4132i32).wrapping_add(((spriteArrId) as i32))) as u16));
            FreeSpriteOamMatrix(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((spriteId).read()) as i32) as isize * 68),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((spriteId).read()) as i32) as isize * 68),
            );
            (spriteId).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadPyramidBagPalette() {
    unsafe {
        let mut spritePalette = crate::ffi::Align4([0u8; 8]);
        let mut palPtr: *mut u16 = (Alloc(64u32)).cast::<u16>();
        LZDecompressWram(
            ((&raw mut gBattlePyramidBag_Pal).cast::<u32>()).cast::<u32>(),
            (palPtr).cast::<u8>(),
        );
        (((&raw mut spritePalette).cast::<u8>()).cast::<*mut u16>()).write(
            (palPtr).wrapping_offset(
                (((crate::c::bf_read(
                    ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                        .wrapping_add(1629),
                    0,
                    2,
                    false,
                ) as u8) as i32)
                    .wrapping_mul(16i32)) as isize,
            ),
        );
        (((&raw mut spritePalette).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(4132u16);
        LoadSpritePalette((&raw mut spritePalette).cast::<u8>());
        Free((palPtr).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreatePyramidBagSprite() {
    unsafe {
        let mut spriteId: *mut u8 = ((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(2052))
        .cast::<u8>();
        (spriteId).write(CreateSprite(
            (&raw const sSpriteTemplate_PyramidBag)
                .cast::<u8>()
                .cast_mut(),
            68i16,
            56i16,
            0u8,
        ));
    }
}
pub(crate) unsafe extern "C" fn ShakePyramidBag() {
    unsafe {
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>())
            .read()) as i32) as isize
                * 68,
        );
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            StartSpriteAffineAnim(sprite, 1u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BagWaitForShake));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BagWaitForShake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            StartSpriteAffineAnim(sprite, 0u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn ShowItemIcon(itemId: u16, isAlt: u8) {
    unsafe {
        let mut itemId = itemId;
        let mut isAlt = isAlt;
        let mut itemSpriteId: u8 = 0u8;
        let mut spriteId: *mut u8 =
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>())
            .wrapping_offset((((isAlt) as i32).wrapping_add(1i32)) as isize);
        if (((spriteId).read()) as i32) == 255i32 {
            FreeSpriteTilesByTag((((4133i32).wrapping_add(((isAlt) as i32))) as u16));
            FreeSpritePaletteByTag((((4133i32).wrapping_add(((isAlt) as i32))) as u16));
            itemSpriteId = AddItemIconSprite(
                (((4133i32).wrapping_add(((isAlt) as i32))) as u16),
                (((4133i32).wrapping_add(((isAlt) as i32))) as u16),
                itemId,
            );
            if ((itemSpriteId) as i32) != 64i32 {
                (spriteId).write(itemSpriteId);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((itemSpriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(24i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((itemSpriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(88i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn FreeItemIconSpriteByAltId(isAlt: u8) {
    unsafe {
        let mut isAlt = isAlt;
        FreeItemIconSprite(((((isAlt) as i32).wrapping_add(1i32)) as u8));
    }
}
pub(crate) unsafe extern "C" fn CreateSwapLine() {
    unsafe {
        CreateSwapLineSprites(
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>())
            .wrapping_offset(3),
            8u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetSwapLineInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        SetSwapLineSpritesInvisibility(
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>())
            .wrapping_offset(3),
            8u8,
            invisible,
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateSwapLinePos(y: u8) {
    unsafe {
        let mut y = y;
        UpdateSwapLineSpritesPos(
            (((((&raw mut gPyramidBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052))
            .cast::<u8>())
            .wrapping_offset(3),
            136u8,
            120i16,
            (((((y) as i32).wrapping_add(1i32)).wrapping_mul(16i32)) as u16),
        );
    }
}
