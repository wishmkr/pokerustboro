//! Translated from `src/pokeblock.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gPokeblockFlavorCompatibilityTable sBgTemplatesForPokeblockMenu gPokeblockNames sPokeblockMenuActions sActionsOnField sActionsInBattle sActionsOnPokeblockFeeder sActionsWhenGivingToLady sTossYesNoFuncTable sContestStatsMonData sOamData_PokeblockCase sSpriteAnim_PokeblockCase sSpriteAnimTable_PokeblockCase sAffineAnim_PokeblockCaseShake sAffineAnims_PokeblockCaseShake gPokeblockCase_SpriteSheet gPokeblockCase_SpritePal sSpriteTemplate_PokeblockCase sTextColor sFavoritePokeblocksTable sWindowTemplates sTossPkblockWindowTemplate sPokeblockListMenuTemplate
#[allow(unused_imports)]
use crate::data::pokeblock::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavedPokeblockData: crate::ffi::Align4<[u8; 8]> = crate::ffi::Align4([0; 8]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPokeblockMenu: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBattleTextBuff1: u8;
    static mut gEnemyParty: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gMenuPokeblock_Gfx: u8;
    static mut gMenuPokeblock_Pal: u8;
    static mut gMenuPokeblock_Tilemap: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_Bitter: u8;
    static mut gText_Dry: u8;
    static mut gText_LvVar1: u8;
    static mut gText_Sour: u8;
    static mut gText_Spicy: u8;
    static mut gText_StowCase: u8;
    static mut gText_Sweet: u8;
    static mut gText_ThrowAwayVar1: u8;
    static mut gText_Var1ThrownAway: u8;
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
    fn Alloc(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToField();
    fn CB2_SetUpReshowBattleScreenAfterMenu2();
    fn ChooseMonToGivePokeblock(a0: *mut u8, a1: Option<unsafe extern "C" fn()>);
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut u8, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
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
    fn FieldCB_ContinueScriptHandleMusic();
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeOamMatrix(a0: u8);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetItemName(a0: u16) -> *mut u8;
    fn GetNature(a0: *mut u8) -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GivePokeblockToContestLady(a0: *mut u8) -> u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadListMenuSwapLineGfx();
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuActionTextsInUpperLeftCorner(a0: u8, a1: u8, a2: *mut u8, a3: *mut u8);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn SafariZoneActivatePokeblockFeeder(a0: u8);
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetSwapLineSpritesInvisibility(a0: *mut u8, a1: u8, a2: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn UpdateSwapLineSpritesPos(a0: *mut u8, a1: u8, a2: i16, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCase(caseId: u8, callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut caseId = caseId;
        let mut callback = callback;
        ((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).write(Alloc(3720u32));
        ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2057))
            .write(caseId);
        ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2048)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(None);
        ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3708))
            .write(255u8);
        ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3709))
            .write(0u8);
        (((&raw mut sSavedPokeblockData).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
            .write(callback);
        'l1: {
            let __sw1 = ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2057))
            .read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 1i32 {
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2052)
                    .cast::<*mut u8>())
                .write(((&raw const sActionsInBattle).cast::<u8>().cast_mut()).cast::<u8>());
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2056))
                .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2052)
                    .cast::<*mut u8>())
                .write(
                    ((&raw const sActionsOnPokeblockFeeder)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2056))
                .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2052)
                    .cast::<*mut u8>())
                .write(
                    ((&raw const sActionsWhenGivingToLady)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2056))
                .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                break 'l1;
            }
            if !__matched {
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2052)
                    .cast::<*mut u8>())
                .write(((&raw const sActionsOnField).cast::<u8>().cast_mut()).cast::<u8>());
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2056))
                .write(((crate::c::div_u32(3u32, 1u32)) as u8));
                break 'l1;
            }
        }
        SetMainCallback2(Some(CB2_InitPokeblockMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCaseInBattle() {
    unsafe {
        OpenPokeblockCase(1u8, Some(CB2_SetUpReshowBattleScreenAfterMenu2));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokeblockCaseOnFeeder() {
    unsafe {
        OpenPokeblockCase(2u8, Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn CB2_PokeblockMenu() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokeblockMenu() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_InitPokeblockMenu() {
    unsafe {
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32 {
                break 'l1;
            }
            if ((InitPokeblockMenu()) as i32) == 1i32 {
                break 'l1;
            }
            if ((MenuHelpers_IsLinkActive()) as i32) == 1i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPokeblockMenu() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
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
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32;
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
                if ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2057))
                .read()) as i32)
                    != 1i32
                {
                    ResetTasks();
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                HandleInitBackgrounds();
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3710)
                    .cast::<i16>())
                .write(0i16);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if !((LoadPokeblockMenuGfx()) != 0) {
                    return 0u8;
                }
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                SetMenuItemsCountAndMaxShowed();
                LimitMenuScrollAndRow();
                SetInitialScroll();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3700))
                .write(CreatePokeblockCaseSprite(56i16, 64i16, 0u8));
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                CreateSwapLineSprites(
                    ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3701))
                    .cast::<u8>(),
                    ((crate::c::div_u32(7u32, 1u32)) as u8),
                );
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                DrawPokeblockMenuHighlight(
                    (((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read(),
                    4101u16,
                );
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                HandleInitWindows();
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                UpdatePokeblockList();
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                CreateScrollArrows();
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                taskId = CreateTask(Some(Task_HandlePokeblockMenuInput), 0u8);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(
                    ((ListMenuInit(
                        (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                        (((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .read(),
                        (((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read(),
                    )) as i16),
                );
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 16i32 {
                DrawPokeblockMenuTitleText();
                let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p18).write(((__p18).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p19 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 18i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p20 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VBlankCB_PokeblockMenu));
                SetMainCallback2(Some(CB2_PokeblockMenu));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HandleInitBackgrounds() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplatesForPokeblockMenu)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            2u8,
            (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
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
pub(crate) unsafe extern "C" fn LoadPokeblockMenuGfx() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3710)
                .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ResetTempTileDataBuffers();
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw mut gMenuPokeblock_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                let __p2 = (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3710)
                    .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LZDecompressWram(
                        ((&raw mut gMenuPokeblock_Tilemap).cast::<u32>()).cast::<u32>(),
                        (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>(),
                    );
                    let __p3 = (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3710)
                        .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadCompressedPalette(
                    ((&raw mut gMenuPokeblock_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    192u16,
                );
                let __p4 = (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3710)
                    .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                LoadCompressedSpriteSheet(
                    (&raw const gPokeblockCase_SpriteSheet)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p5 = (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3710)
                    .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadCompressedSpritePalette(
                    (&raw const gPokeblockCase_SpritePal)
                        .cast::<u8>()
                        .cast_mut(),
                );
                let __p6 = (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3710)
                    .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadListMenuSwapLineGfx();
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3710)
                    .cast::<i16>())
                .write(0i16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HandleInitWindows() {
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
                if !(((i) as u32) < (crate::c::div_u32(96u32, 8u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(i, 0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn PrintOnPokeblockWindow(windowId: u8, string: *mut u8, x: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut string = string;
        let mut x = x;
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            ((x) as u8),
            1u8,
            0u8,
            0u8,
            ((&raw const sTextColor).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            string,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawPokeblockMenuTitleText() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut itemName: *mut u8 = GetItemName(273u16);
        PrintOnPokeblockWindow(
            0u8,
            itemName,
            GetStringCenterAlignXOffset(1i32, itemName, 72i32),
        );
        PrintOnPokeblockWindow(2u8, (&raw mut gText_Spicy).cast::<u8>(), 0i32);
        PrintOnPokeblockWindow(3u8, (&raw mut gText_Dry).cast::<u8>(), 0i32);
        PrintOnPokeblockWindow(4u8, (&raw mut gText_Sweet).cast::<u8>(), 0i32);
        PrintOnPokeblockWindow(5u8, (&raw mut gText_Bitter).cast::<u8>(), 0i32);
        PrintOnPokeblockWindow(6u8, (&raw mut gText_Sour).cast::<u8>(), 0i32);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 8i32) {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePokeblockList() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2058))
                    .read()) as i32)
                        .wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    PutPokeblockListMenuString(
                        ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2388))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                        .cast::<u8>(),
                        i,
                    );
                    (((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2060))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2388))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 32))
                        .cast::<u8>(),
                    );
                    (((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2060))
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
            ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2388))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 32))
            .cast::<u8>(),
            (&raw mut gText_StowCase).cast::<u8>(),
        );
        (((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2060))
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write(
            ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2388))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 32))
            .cast::<u8>(),
        );
        (((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2060))
        .cast::<u8>())
        .wrapping_offset(((i) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sPokeblockListMenuTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        crate::c::bf_write(
            ((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(23),
            0,
            6,
            (7u8) as i32,
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(
            ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2058))
            .read()) as u16),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2060))
            .cast::<u8>(),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(
            ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2059))
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn PutPokeblockListMenuString(dst: *mut u8, pkblId: u16) {
    unsafe {
        let mut dst = dst;
        let mut pkblId = pkblId;
        let mut pkblock: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2120))
        .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8);
        let mut txtPtr: *mut u8 = StringCopy(
            dst,
            ((((&raw const gPokeblockNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset((((pkblock).read()) as i32) as isize))
            .read(),
        );
        ({
            let __t1 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t1
        })
        .write(252u8);
        ({
            let __t2 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t2
        })
        .write(18u8);
        ({
            let __t3 = txtPtr;
            txtPtr = (txtPtr).wrapping_offset(1);
            __t3
        })
        .write(87u8);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((GetHighestPokeblocksFlavorLevel(pkblock)) as i32),
            0i32,
            3u8,
        );
        StringExpandPlaceholders(txtPtr, (&raw mut gText_LvVar1).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn MovePokeblockMenuCursor(pkblId: i32, onInit: u8, list: *mut u8) {
    unsafe {
        let mut pkblId = pkblId;
        let mut onInit = onInit;
        let mut list = list;
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3700))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ShakePokeblockCase));
        }
        if !((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3709))
        .read())
            != 0)
        {
            DrawPokeblockInfo(pkblId);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawPokeblockInfo(pkblId: i32) {
    unsafe {
        let mut pkblId = pkblId;
        let mut i: u8 = 0u8;
        let mut pokeblock: *mut u8 = core::ptr::null_mut();
        let mut rectTilemapSrc = crate::ffi::Align4([0u8; 4]);
        FillWindowPixelBuffer(7u8, 0u8);
        if pkblId != (-2i32) {
            pokeblock = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(2120))
            .cast::<u8>())
            .wrapping_offset((pkblId) as isize * 8);
            ((&raw mut rectTilemapSrc).cast::<u16>()).write(23u16);
            (((&raw mut rectTilemapSrc).cast::<u16>()).wrapping_offset(1)).write(24u16);
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((GetPokeblockData(
                            pokeblock,
                            (((1i32).wrapping_add(((i) as i32))) as u8),
                        )) as i32)
                            > 0i32
                        {
                            ((&raw mut rectTilemapSrc).cast::<u16>())
                                .write((((((i) as i32) << 12).wrapping_add(23i32)) as u16));
                            (((&raw mut rectTilemapSrc).cast::<u16>()).wrapping_offset(1))
                                .write((((((i) as i32) << 12).wrapping_add(24i32)) as u16));
                        } else {
                            ((&raw mut rectTilemapSrc).cast::<u16>()).write(15u16);
                            (((&raw mut rectTilemapSrc).cast::<u16>()).wrapping_offset(1))
                                .write(15u16);
                        }
                        CopyToBgTilemapBufferRect(
                            2u8,
                            ((&raw mut rectTilemapSrc).cast::<u16>()).cast::<u8>(),
                            ((((crate::c::div_i32(((i) as i32), 3i32)).wrapping_mul(6i32))
                                .wrapping_add(1i32)) as u8),
                            ((((crate::c::rem_i32(((i) as i32), 3i32)).wrapping_mul(2i32))
                                .wrapping_add(13i32)) as u8),
                            1u8,
                            2u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((GetPokeblocksFeel(pokeblock)) as i32),
                1i32,
                2u8,
            );
            PrintOnPokeblockWindow(7u8, (&raw mut gStringVar1).cast::<u8>(), 4i32);
        } else {
            ((&raw mut rectTilemapSrc).cast::<u16>()).write(15u16);
            (((&raw mut rectTilemapSrc).cast::<u16>()).wrapping_offset(1)).write(15u16);
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 5i32) {
                        break 'l3;
                    }
                    'l4: {
                        CopyToBgTilemapBufferRect(
                            2u8,
                            ((&raw mut rectTilemapSrc).cast::<u16>()).cast::<u8>(),
                            ((((crate::c::div_i32(((i) as i32), 3i32)).wrapping_mul(6i32))
                                .wrapping_add(1i32)) as u8),
                            ((((crate::c::rem_i32(((i) as i32), 3i32)).wrapping_mul(2i32))
                                .wrapping_add(13i32)) as u8),
                            1u8,
                            2u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            CopyWindowToVram(7u8, 2u8);
        }
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DrawPokeblockMenuHighlight(cursorPos: u16, tileNum: u16) {
    unsafe {
        let mut cursorPos = cursorPos;
        let mut tileNum = tileNum;
        FillBgTilemapBufferRect_Palette0(
            2u8,
            tileNum,
            15u8,
            (((((cursorPos) as i32).wrapping_mul(2i32)).wrapping_add(1i32)) as u8),
            14u8,
            2u8,
        );
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn CompactPokeblockSlots() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 39i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u16);
                        'l3: loop {
                            if !(((j) as i32) < 40i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(2120))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 8))
                                .read()) as i32)
                                    == 0i32
                                {
                                    let mut temp = crate::ffi::Align4([0u8; 8]);
                                    (&raw mut temp)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<8>>()
                                        .write_unaligned(
                                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(2120))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 8)
                                            .cast::<crate::c::Rec4<8>>()
                                            .read_unaligned(),
                                        );
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(2120))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 8)
                                    .cast::<crate::c::Rec4<8>>()
                                    .write_unaligned(
                                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(2120))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 8)
                                        .cast::<crate::c::Rec4<8>>()
                                        .read_unaligned(),
                                    );
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(2120))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 8)
                                    .cast::<crate::c::Rec4<8>>()
                                    .write_unaligned(
                                        (&raw mut temp)
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<8>>()
                                            .read_unaligned(),
                                    );
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
pub(crate) unsafe extern "C" fn SwapPokeblockMenuItems(id1: u32, id2: u32) {
    unsafe {
        let mut id1 = id1;
        let mut id2 = id2;
        let mut i: i16 = 0i16;
        let mut count: i16 = 0i16;
        let mut pokeblocks: *mut u8 = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2120))
        .cast::<u8>();
        let mut copyPokeblock1: *mut u8 = core::ptr::null_mut();
        if id1 == id2 {
            return;
        }
        copyPokeblock1 = Alloc(8u32);
        copyPokeblock1.cast::<crate::c::Rec4<8>>().write_unaligned(
            (pokeblocks)
                .wrapping_offset(((id1) as i32) as isize * 8)
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
        );
        if id2 > id1 {
            id2 = (id2).wrapping_sub(1);
            {
                count = ((id2) as i16);
                i = ((id1) as i16);
                'l1: loop {
                    if !(((i) as i32) < ((count) as i32)) {
                        break 'l1;
                    }
                    'l2: {
                        (pokeblocks)
                            .wrapping_offset(((i) as i32) as isize * 8)
                            .cast::<crate::c::Rec4<8>>()
                            .write_unaligned(
                                (pokeblocks)
                                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize * 8)
                                    .cast::<crate::c::Rec4<8>>()
                                    .read_unaligned(),
                            );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            {
                count = ((id2) as i16);
                i = ((id1) as i16);
                'l3: loop {
                    if !(((i) as i32) > ((count) as i32)) {
                        break 'l3;
                    }
                    'l4: {
                        (pokeblocks)
                            .wrapping_offset(((i) as i32) as isize * 8)
                            .cast::<crate::c::Rec4<8>>()
                            .write_unaligned(
                                (pokeblocks)
                                    .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize * 8)
                                    .cast::<crate::c::Rec4<8>>()
                                    .read_unaligned(),
                            );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
        }
        (pokeblocks)
            .wrapping_offset(((id2) as i32) as isize * 8)
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(copyPokeblock1.cast::<crate::c::Rec4<8>>().read_unaligned());
        Free(copyPokeblock1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetPokeblockScrollPositions() {
    unsafe {
        (((&raw mut sSavedPokeblockData).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut sSavedPokeblockData).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(0u16);
    }
}
pub(crate) unsafe extern "C" fn SetMenuItemsCountAndMaxShowed() {
    unsafe {
        let mut i: u16 = 0u16;
        CompactPokeblockSlots();
        {
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2058))
            .write(0u8);
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 40i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(2120))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .read()) as i32)
                        != 0i32
                    {
                        let __p1 = (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(2058);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p2 =
            (((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2058);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2058))
        .read()) as i32)
            > 9i32
        {
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2059))
            .write(9u8);
        } else {
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2059))
            .write(
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2058))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LimitMenuScrollAndRow() {
    unsafe {
        if (((((&raw mut sSavedPokeblockData).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            if (((((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2059))
                    .read()) as i32),
                )
                > ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2058))
                .read()) as i32)
            {
                (((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .write(
                    ((((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2058))
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2059))
                            .read()) as i32),
                        )) as u16),
                );
            }
        }
        if (((((&raw mut sSavedPokeblockData).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(
                (((((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as i32),
            )
            >= ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2058))
            .read()) as i32)
        {
            if ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2058))
            .read()) as i32)
                == 0i32
            {
                (((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(0u16);
            } else {
                (((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .write(
                    ((((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2058))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetInitialScroll() {
    unsafe {
        if (((((&raw mut sSavedPokeblockData).cast::<u8>())
            .wrapping_add(4)
            .cast::<u16>())
        .read()) as i32)
            > crate::c::div_i32(9i32, 2i32)
        {
            let mut i: u8 = 0u8;
            {
                i = 0u8;
                'l1: loop {
                    if !((((i) as i32)
                        < (((((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_sub(crate::c::div_i32(9i32, 2i32)))
                        && ((((((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(
                                ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2059))
                                .read()) as i32),
                            )
                            != ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(2058))
                            .read()) as i32)))
                    {
                        break 'l1;
                    }
                    'l2: {}
                    let __p1 = ((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>();
                    (__p1).write(((__p1).read()).wrapping_sub(1));
                    let __p2 = ((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateScrollArrows() {
    unsafe {
        if ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3708))
        .read()) as i32)
            == 255i32
        {
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3708))
            .write(AddScrollIndicatorArrowPairParameterized(
                2u32,
                176i32,
                8i32,
                152i32,
                ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2058))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2059))
                        .read()) as i32),
                    ),
                1110i32,
                1110i32,
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>(),
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyScrollArrows() {
    unsafe {
        if ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(3708))
        .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3708))
                .read(),
            );
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3708))
            .write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePokeblockCaseSprite(x: i16, y: i16, subpriority: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut subpriority = subpriority;
        return CreateSprite(
            (&raw const sSpriteTemplate_PokeblockCase)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            subpriority,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ShakePokeblockCase(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 1i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (1u32) as i32);
                ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
                    ((&raw const sAffineAnims_PokeblockCaseShake)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>(),
                );
                InitSpriteAffineAnim(sprite);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 11i32
                {
                    crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    FreeOamMatrix(
                        ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                    );
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
pub(crate) unsafe extern "C" fn FadePaletteAndSetTaskToClosePokeblockCase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_FreeDataAndExitPokeblockCase));
    }
}
pub(crate) unsafe extern "C" fn Task_FreeDataAndExitPokeblockCase(taskId: u8) {
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
            if (((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2057))
            .read()) as i32)
                == 2i32)
                || (((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2057))
                .read()) as i32)
                    == 3i32)
            {
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCB_ContinueScriptHandleMusic));
            }
            DestroyListMenuTask(
                (((data).read()) as u8),
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>(),
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>(),
            );
            DestroyScrollArrows();
            ResetSpriteData();
            FreeAllSpritePalettes();
            if core::mem::transmute::<_, usize>(
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2048)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                SetMainCallback2(
                    ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2048)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            } else {
                SetMainCallback2(
                    (((&raw mut sSavedPokeblockData).cast::<u8>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            }
            FreeAllWindowBuffers();
            Free(((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read());
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblockMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
        {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 4i32)
                != 0
            {
                ListMenuGetScrollAndRow(
                    (((data).read()) as u8),
                    ((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>(),
                    ((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>(),
                );
                if (((((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(
                        (((((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read()) as i32),
                    )
                    != ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2058))
                    .read()) as i32)
                        .wrapping_sub(1i32)
                {
                    PlaySE(5u16);
                    DrawPokeblockMenuHighlight(
                        (((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read(),
                        8197u16,
                    );
                    ((data).wrapping_offset(2)).write(
                        (((((((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(6)
                            .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(
                                (((((&raw mut sSavedPokeblockData).cast::<u8>())
                                    .wrapping_add(4)
                                    .cast::<u16>())
                                .read()) as i32),
                            )) as i16),
                    );
                    ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(3709))
                    .write(1u8);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandlePokeblocksSwapInput));
                }
            } else {
                let mut oldPosition: u16 = (((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read();
                let mut input: i32 = ListMenu_ProcessInput((((data).read()) as u8));
                ListMenuGetScrollAndRow(
                    (((data).read()) as u8),
                    ((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>(),
                    ((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>(),
                );
                if ((oldPosition) as i32)
                    != (((((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32)
                {
                    DrawPokeblockMenuHighlight(oldPosition, 5u16);
                    DrawPokeblockMenuHighlight(
                        (((&raw mut sSavedPokeblockData).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<u16>())
                        .read(),
                        4101u16,
                    );
                }
                'l1: {
                    let __sw1 = input;
                    let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                    if __sw1 == (-1i32) {
                        break 'l1;
                    }
                    if __sw1 == (-2i32) {
                        PlaySE(5u16);
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(65535u16);
                        ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(0u16);
                        FadePaletteAndSetTaskToClosePokeblockCase(taskId);
                        break 'l1;
                    }
                    if !__matched {
                        PlaySE(5u16);
                        ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(((input) as u16));
                        ShowPokeblockActionsWindow(taskId);
                        break 'l1;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblocksSwapInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32 {
            return;
        }
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
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>(),
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>(),
            );
            UpdatePokeblockSwapMenu(taskId, 0u8);
        } else {
            let mut i: u16 = (((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>())
            .read();
            let mut row: u16 = (((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .read();
            let mut input: i32 = ListMenu_ProcessInput((((data).read()) as u8));
            ListMenuGetScrollAndRow(
                (((data).read()) as u8),
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>(),
                ((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>(),
            );
            if (((i) as i32)
                != (((((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read()) as i32))
                || (((row) as i32)
                    != (((((&raw mut sSavedPokeblockData).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u16>())
                    .read()) as i32))
            {
                {
                    i = 0u16;
                    'l1: loop {
                        if !(((i) as i32) < 9i32) {
                            break 'l1;
                        }
                        'l2: {
                            row = ((((i) as i32).wrapping_add(
                                (((((&raw mut sSavedPokeblockData).cast::<u8>())
                                    .wrapping_add(6)
                                    .cast::<u16>())
                                .read()) as i32),
                            )) as u16);
                            if ((row) as i32) == ((((data).wrapping_offset(2)).read()) as i32) {
                                DrawPokeblockMenuHighlight(i, 8197u16);
                            } else {
                                DrawPokeblockMenuHighlight(i, 5u16);
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            SetSwapLineSpritesInvisibility(
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3701))
                .cast::<u8>(),
                ((crate::c::div_u32(7u32, 1u32)) as u8),
                0u8,
            );
            UpdateSwapLineSpritesPos(
                ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(3701))
                .cast::<u8>(),
                ((crate::c::div_u32(7u32, 1u32)) as u8),
                128i16,
                ((((((((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(16i32))
                .wrapping_add(8i32)) as u16),
            );
            'l3: {
                let __sw1 = input;
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l3;
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
                        UpdatePokeblockSwapMenu(taskId, 0u8);
                    } else {
                        UpdatePokeblockSwapMenu(taskId, 1u8);
                    }
                    break 'l3;
                }
                if !__matched {
                    PlaySE(5u16);
                    UpdatePokeblockSwapMenu(taskId, 0u8);
                    break 'l3;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePokeblockSwapMenu(taskId: u8, noSwap: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut noSwap = noSwap;
        let mut i: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut swappedFromId: u16 = (((((((&raw mut sSavedPokeblockData).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(
                (((((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16);
        ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(3709))
            .write(0u8);
        DestroyListMenuTask(
            (((data).read()) as u8),
            ((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>(),
            ((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>(),
        );
        if ((!((noSwap) != 0))
            && (((((data).wrapping_offset(2)).read()) as i32) != ((swappedFromId) as i32)))
            && (((((data).wrapping_offset(2)).read()) as i32)
                != ((swappedFromId) as i32).wrapping_sub(1i32))
        {
            SwapPokeblockMenuItems(
                ((((data).wrapping_offset(2)).read()) as u32),
                ((swappedFromId) as u32),
            );
            UpdatePokeblockList();
        }
        if ((((data).wrapping_offset(2)).read()) as i32) < ((swappedFromId) as i32) {
            let __p1 = ((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(6)
                    .cast::<u16>())
                .read(),
                (((&raw mut sSavedPokeblockData).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u16>())
                .read(),
            )) as i16),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        SetSwapLineSpritesInvisibility(
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(3701))
            .cast::<u8>(),
            ((crate::c::div_u32(7u32, 1u32)) as u8),
            1u8,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 9i32) {
                    break 'l1;
                }
                'l2: {
                    DrawPokeblockMenuHighlight(((i) as u16), 5u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        DrawPokeblockMenuHighlight(
            (((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
            4101u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandlePokeblockMenuInput));
    }
}
pub(crate) unsafe extern "C" fn ShowPokeblockActionsWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2056))
        .read()) as i32)
            == 3i32
        {
            ((data).wrapping_offset(1)).write(8i16);
        } else {
            ((data).wrapping_offset(1)).write(9i16);
        }
        DestroyScrollArrows();
        DrawStdFrameWithCustomTileAndPalette(
            ((((data).wrapping_offset(1)).read()) as u8),
            0u8,
            1u16,
            14u8,
        );
        PrintMenuActionTextsInUpperLeftCorner(
            ((((data).wrapping_offset(1)).read()) as u8),
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2056))
            .read(),
            ((&raw const sPokeblockMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2052)
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuInUpperLeftCornerNormal(
            ((((data).wrapping_offset(1)).read()) as u8),
            ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2056))
            .read(),
            0u8,
        );
        PutWindowTilemap(((((data).wrapping_offset(1)).read()) as u8));
        ScheduleBgCopyTilemapToVram(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandlePokeblockActionsInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandlePokeblockActionsInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut itemId: i8 = 0i8;
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32 {
            return;
        }
        itemId = Menu_ProcessInputNoWrap();
        if ((itemId) as i32) == (-2i32) {
            return;
        } else {
            if ((itemId) as i32) == (-1i32) {
                PlaySE(5u16);
                PokeblockAction_Cancel(taskId);
            } else {
                PlaySE(5u16);
                (((((((&raw const sPokeblockMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2052)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((itemId) as i32) as isize))
                        .read()) as i32) as isize
                            * 8,
                    ))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_UseOnField(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut sPokeblockMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2048)
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(UsePokeblockOnField));
        FadePaletteAndSetTaskToClosePokeblockCase(taskId);
    }
}
pub(crate) unsafe extern "C" fn UsePokeblockOnField() {
    unsafe {
        ChooseMonToGivePokeblock(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
            ),
            Some(ReturnToPokeblockCaseOnField),
        );
    }
}
pub(crate) unsafe extern "C" fn ReturnToPokeblockCaseOnField() {
    unsafe {
        OpenPokeblockCase(
            0u8,
            (((&raw mut sSavedPokeblockData).cast::<u8>())
                .cast::<Option<unsafe extern "C" fn()>>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_Toss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ClearStdWindowAndFrameToTransparent(((((data).wrapping_offset(1)).read()) as u8), 0u8);
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw const gPokeblockNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
                ))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ThrowAwayVar1).cast::<u8>(),
        );
        DisplayMessageAndContinueTask(
            taskId,
            10u8,
            10u16,
            13u8,
            1u8,
            GetPlayerTextSpeedDelay(),
            (&raw mut gStringVar4).cast::<u8>(),
            core::mem::transmute::<Option<unsafe extern "C" fn(u8)>, *mut u8>(Some(
                CreateTossPokeblockYesNoMenu,
            )),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateTossPokeblockYesNoMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CreateYesNoMenuWithCallbacks(
            taskId,
            (&raw const sTossPkblockWindowTemplate)
                .cast::<u8>()
                .cast_mut(),
            1u8,
            0u8,
            2u8,
            1u16,
            14u8,
            (&raw const sTossYesNoFuncTable).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn TossedPokeblockMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Var1ThrownAway).cast::<u8>(),
        );
        DisplayMessageAndContinueTask(
            taskId,
            10u8,
            10u16,
            13u8,
            1u8,
            GetPlayerTextSpeedDelay(),
            (&raw mut gStringVar4).cast::<u8>(),
            core::mem::transmute::<Option<unsafe extern "C" fn(u8)>, *mut u8>(Some(TossPokeblock)),
        );
    }
}
pub(crate) unsafe extern "C" fn TossPokeblock(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            let mut data: *mut i16 = core::ptr::null_mut();
            let mut scrollOffset: *mut u16 = core::ptr::null_mut();
            let mut selectedRow: *mut u16 = core::ptr::null_mut();
            TryClearPokeblock(((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8));
            PlaySE(5u16);
            scrollOffset = ((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(6)
                .cast::<u16>();
            selectedRow = ((&raw mut sSavedPokeblockData).cast::<u8>())
                .wrapping_add(4)
                .cast::<u16>();
            data = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            DestroyListMenuTask((((data).read()) as u8), scrollOffset, selectedRow);
            DrawPokeblockMenuHighlight((selectedRow).read(), 5u16);
            SetMenuItemsCountAndMaxShowed();
            LimitMenuScrollAndRow();
            UpdatePokeblockList();
            (data).write(
                ((ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    (scrollOffset).read(),
                    (selectedRow).read(),
                )) as i16),
            );
            DrawPokeblockMenuHighlight((selectedRow).read(), 4101u16);
            ScheduleBgCopyTilemapToVram(0u8);
            ScheduleBgCopyTilemapToVram(1u8);
            CloseTossPokeblockWindow(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CloseTossPokeblockWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrameToTransparent(10u8, 0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        CreateScrollArrows();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandlePokeblockMenuInput));
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_UseInBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut nature: u8 = GetNature((&raw mut gEnemyParty).cast::<u8>());
        let mut gain: i16 = PokeblockGetGain(
            nature,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
            ),
        );
        StringCopy(
            (&raw mut gBattleTextBuff1).cast::<u8>(),
            ((((&raw const gPokeblockNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
                ))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        TryClearPokeblock(((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8));
        ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(
            ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                .cast::<u8>())
            .wrapping_offset(
                ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
            ))
            .read()) as i32)
                << 8) as u16),
        );
        if ((gain) as i32) == 0i32 {
            let __p1 = (&raw mut gSpecialVar_ItemId).cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(1i32)) as u16));
        } else {
            if ((gain) as i32) > 0i32 {
                let __p2 = (&raw mut gSpecialVar_ItemId).cast::<u16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(2i32)) as u16));
            } else {
                let __p3 = (&raw mut gSpecialVar_ItemId).cast::<u16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(3i32)) as u16));
            }
        }
        FadePaletteAndSetTaskToClosePokeblockCase(taskId);
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_UseOnPokeblockFeeder(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SafariZoneActivatePokeblockFeeder(
            ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8),
        );
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw const gPokeblockNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
                ))
                .read()) as i32) as isize,
            ))
            .read(),
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
        TryClearPokeblock(((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8));
        ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(0u16);
        FadePaletteAndSetTaskToClosePokeblockCase(taskId);
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_GiveToContestLady(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(
            ((GivePokeblockToContestLady(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) as isize * 8,
                ),
            )) as u16),
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>())
            .write(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
        TryClearPokeblock(((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as u8));
        ((&raw mut gSpecialVar_ItemId).cast::<u16>()).write(0u16);
        FadePaletteAndSetTaskToClosePokeblockCase(taskId);
    }
}
pub(crate) unsafe extern "C" fn PokeblockAction_Cancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ClearStdWindowAndFrameToTransparent(((((data).wrapping_offset(1)).read()) as u8), 0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        CreateScrollArrows();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandlePokeblockMenuInput));
    }
}
pub(crate) unsafe extern "C" fn ClearPokeblock(pkblId: u8) {
    unsafe {
        let mut pkblId = pkblId;
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .wrapping_add(1))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .wrapping_add(2))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .wrapping_add(3))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .wrapping_add(4))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .wrapping_add(5))
        .write(0u8);
        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .wrapping_add(6))
        .write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPokeblocks() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 40i32) {
                    break 'l1;
                }
                'l2: {
                    ClearPokeblock(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetHighestPokeblocksFlavorLevel(pokeblock: *mut u8) -> u8 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut i: u8 = 0u8;
        let mut maxFlavor: u8 = ((GetPokeblockData(pokeblock, 1u8)) as u8);
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut currFlavor: u8 =
                        ((GetPokeblockData(pokeblock, (((1i32).wrapping_add(((i) as i32))) as u8)))
                            as u8);
                    if ((maxFlavor) as i32) < ((currFlavor) as i32) {
                        maxFlavor = currFlavor;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return maxFlavor;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblocksFeel(pokeblock: *mut u8) -> u8 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut feel: u8 = ((GetPokeblockData(pokeblock, 6u8)) as u8);
        if ((feel) as i32) > 99i32 {
            feel = 99u8;
        }
        return feel;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFirstFreePokeblockSlot() -> i8 {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 40i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(2120))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .read()) as i32)
                        == 0i32
                    {
                        return ((i) as i8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddPokeblock(pokeblock: *mut u8) -> u32 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut slot: i8 = GetFirstFreePokeblockSlot();
        if ((slot) as i32) == (-1i32) {
            return 0u32;
        } else {
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
                .cast::<u8>())
            .wrapping_offset(((slot) as i32) as isize * 8)
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(pokeblock.cast::<crate::c::Rec4<8>>().read_unaligned());
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryClearPokeblock(pkblId: u8) -> u32 {
    unsafe {
        let mut pkblId = pkblId;
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(2120))
            .cast::<u8>())
        .wrapping_offset(((pkblId) as i32) as isize * 8))
        .read()) as i32)
            == 0i32
        {
            return 0u32;
        } else {
            ClearPokeblock(pkblId);
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblockData(pokeblock: *mut u8, field: u8) -> i16 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut field = field;
        if ((field) as i32) == 0i32 {
            return (((pokeblock).read()) as i16);
        }
        if ((field) as i32) == 1i32 {
            return ((((pokeblock).wrapping_add(1)).read()) as i16);
        }
        if ((field) as i32) == 2i32 {
            return ((((pokeblock).wrapping_add(2)).read()) as i16);
        }
        if ((field) as i32) == 3i32 {
            return ((((pokeblock).wrapping_add(3)).read()) as i16);
        }
        if ((field) as i32) == 4i32 {
            return ((((pokeblock).wrapping_add(4)).read()) as i16);
        }
        if ((field) as i32) == 5i32 {
            return ((((pokeblock).wrapping_add(5)).read()) as i16);
        }
        if ((field) as i32) == 6i32 {
            return ((((pokeblock).wrapping_add(6)).read()) as i16);
        }
        return 0i16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokeblockGetGain(nature: u8, pokeblock: *mut u8) -> i16 {
    unsafe {
        let mut nature = nature;
        let mut pokeblock = pokeblock;
        let mut flavor: u8 = 0u8;
        let mut curGain: i16 = 0i16;
        let mut totalGain: i16 = 0i16;
        {
            flavor = 0u8;
            'l1: loop {
                if !(((flavor) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    curGain =
                        GetPokeblockData(pokeblock, ((((flavor) as i32).wrapping_add(1i32)) as u8));
                    if ((curGain) as i32) > 0i32 {
                        totalGain = ((((totalGain) as i32).wrapping_add(
                            ((curGain) as i32).wrapping_mul(
                                ((((((&raw const gPokeblockFlavorCompatibilityTable)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<i8>())
                                .cast::<i8>())
                                .wrapping_offset(
                                    (((5i32).wrapping_mul(((nature) as i32)))
                                        .wrapping_add(((flavor) as i32)))
                                        as isize,
                                ))
                                .read()) as i32),
                            ),
                        )) as i16);
                    }
                }
                flavor = (flavor).wrapping_add(1);
            }
        }
        return totalGain;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokeblockCopyName(pokeblock: *mut u8, dest: *mut u8) {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut dest = dest;
        let mut color: u8 = ((GetPokeblockData(pokeblock, 0u8)) as u8);
        StringCopy(
            dest,
            ((((&raw const gPokeblockNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((color) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyMonFavoritePokeblockName(nature: u8, dest: *mut u8) -> u8 {
    unsafe {
        let mut nature = nature;
        let mut dest = dest;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((PokeblockGetGain(
                        nature,
                        (((&raw const sFavoritePokeblocksTable)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 8),
                    )) as i32)
                        > 0i32
                    {
                        StringCopy(
                            dest,
                            ((((&raw const gPokeblockNames)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                            .read(),
                        );
                        return 1u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblocksFlavor(pokeblock: *mut u8) -> u8 {
    unsafe {
        let mut pokeblock = pokeblock;
        let mut bestFlavor: i16 = 0i16;
        let mut i: i16 = 0i16;
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((GetPokeblockData(
                        pokeblock,
                        ((((bestFlavor) as i32).wrapping_add(1i32)) as u8),
                    )) as i32)
                        < ((GetPokeblockData(pokeblock, ((((i) as i32).wrapping_add(1i32)) as u8)))
                            as i32)
                    {
                        bestFlavor = i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return ((bestFlavor) as u8);
    }
}
