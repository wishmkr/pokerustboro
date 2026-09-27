//! Translated from `src/shop.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sShopPurchaseYesNoFuncs sShopMenuActions_BuySellQuit sShopMenuActions_BuyQuit sShopMenuWindowTemplates sShopBuyMenuListTemplate sShopBuyMenuBgTemplates sShopBuyMenuWindowTemplates sShopBuyMenuYesNoWindowTemplates sShopBuyMenuTextColors
#[allow(unused_imports)]
use crate::data::shop::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMartInfo: crate::ffi::Align4<[u8; 16]> = crate::ffi::Align4([0; 16]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sShopData: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sListMenuItems: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemNames: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPurchaseHistoryId: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gMartPurchaseHistory: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);

unsafe extern "C" {
    static mut gDecorations: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMoveNames: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gShopMenu_Gfx: u8;
    static mut gShopMenu_Pal: u8;
    static mut gShopMenu_Tilemap: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar3: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AnythingElseICanHelp: u8;
    static mut gText_CanIHelpWithAnythingElse: u8;
    static mut gText_Cancel2: u8;
    static mut gText_HereYouGoThankYou: u8;
    static mut gText_InBagVar1: u8;
    static mut gText_NoMoreRoomForThis: u8;
    static mut gText_PokedollarVar1: u8;
    static mut gText_QuitShopping: u8;
    static mut gText_SelectorArrow2: u8;
    static mut gText_SpaceForVar1Full: u8;
    static mut gText_ThankYouIllSendItHome: u8;
    static mut gText_ThanksIllSendItHome: u8;
    static mut gText_ThrowInPremierBall: u8;
    static mut gText_Var1AndYouWantedVar2: u8;
    static mut gText_Var1CertainlyHowMany: u8;
    static mut gText_Var1CertainlyHowMany2: u8;
    static mut gText_Var1IsItThatllBeVar2: u8;
    static mut gText_YouDontHaveMoney: u8;
    static mut gText_YouWantedVar1ThatllBeVar2: u8;
    static mut gText_xVar1: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddDecorationIconObject(a0: u8, a1: i16, a2: i16, a3: u8, a4: u16, a5: u16) -> u8;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn AddMoneyLabelObject(a0: u16, a1: u16);
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn AdjustQuantityAccordingToDPadInput(a0: *mut i16, a1: u16) -> u8;
    fn Alloc(a0: u32) -> *mut u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_GoToSellMenu();
    fn CB2_ReturnToField();
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountTotalItemQuantityInBag(a0: u16) -> u16;
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateObjectGraphicsSprite(
        a0: u16,
        a1: Option<unsafe extern "C" fn(*mut u8)>,
        a2: i16,
        a3: i16,
        a4: u8,
    ) -> u8;
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
    fn DecorationAdd(a0: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
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
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemPocket(a0: u16) -> u8;
    fn GetItemPrice(a0: u16) -> u16;
    fn GetMaxWidthInMenuTable(a0: *mut u8, a1: i32) -> i32;
    fn GetMoney(a0: *mut u32) -> u32;
    fn GetObjectEventGraphicsInfo(a0: u8) -> *mut u8;
    fn GetObjectEventIdByXY(a0: i16, a1: i16) -> u8;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsEnoughMoney(a0: *mut u32, a1: u32) -> u8;
    fn IsPokeNewsActive(a0: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuGetYCoordForPrintingArrowCursor(a0: u8) -> u16;
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileLayerTypeAt(a0: i32, a1: i32) -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
    fn PrintMoneyAmount(a0: u8, a1: u8, a2: u8, a3: i32, a4: u8);
    fn PrintMoneyAmountInMoneyBox(a0: u8, a1: i32, a2: u8);
    fn PrintMoneyAmountInMoneyBoxWithBorder(a0: u8, a1: u16, a2: u8, a3: i32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveMoney(a0: *mut u32, a1: u32);
    fn RemoveMoneyLabelObject();
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryPutSmartShopperOnAir();
    fn UnlockPlayerFieldControls();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn CreateShopMenu(martType: u8) -> u8 {
    unsafe {
        let mut martType = martType;
        let mut numMenuItems: i32 = 0i32;
        LockPlayerFieldControls();
        (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).write(martType);
        if ((martType) as i32) == 0i32 {
            let mut winTemplate = crate::ffi::Align4([0u8; 8]);
            (&raw mut winTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    ((&raw const sShopMenuWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
                );
            (((&raw mut winTemplate).cast::<u8>()).wrapping_add(3)).write(
                ((GetMaxWidthInMenuTable(
                    ((&raw const sShopMenuActions_BuySellQuit)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(24u32, 8u32)) as i32),
                )) as u8),
            );
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14))
                .write(((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8));
            (((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(
                ((&raw const sShopMenuActions_BuySellQuit)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            numMenuItems = ((crate::c::div_u32(24u32, 8u32)) as i32);
        } else {
            let mut winTemplate = crate::ffi::Align4([0u8; 8]);
            (&raw mut winTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    (((&raw const sShopMenuWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(8)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
                );
            (((&raw mut winTemplate).cast::<u8>()).wrapping_add(3)).write(
                ((GetMaxWidthInMenuTable(
                    ((&raw const sShopMenuActions_BuyQuit)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(16u32, 8u32)) as i32),
                )) as u8),
            );
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14))
                .write(((AddWindow((&raw mut winTemplate).cast::<u8>())) as u8));
            (((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .write(
                ((&raw const sShopMenuActions_BuyQuit)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            numMenuItems = ((crate::c::div_u32(16u32, 8u32)) as i32);
        }
        SetStandardWindowBorderStyle(
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read(),
            0u8,
        );
        PrintMenuTable(
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read(),
            ((numMenuItems) as u8),
            (((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuInUpperLeftCornerNormal(
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read(),
            ((numMenuItems) as u8),
            0u8,
        );
        PutWindowTilemap((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read());
        CopyWindowToVram(
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read(),
            1u8,
        );
        return CreateTask(Some(Task_ShopMenu), 8u8);
    }
}
pub(crate) unsafe extern "C" fn SetShopMenuCallback(callback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut callback = callback;
        (((&raw mut sMartInfo).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
            .write(callback);
    }
}
pub(crate) unsafe extern "C" fn SetShopItemsForSale(items: *mut u16) {
    unsafe {
        let mut items = items;
        let mut i: u16 = 0u16;
        (((&raw mut sMartInfo).cast::<u8>())
            .wrapping_add(8)
            .cast::<*mut u16>())
        .write(items);
        (((&raw mut sMartInfo).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(0u16);
        'l1: loop {
            if !(((((((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(8)
                .cast::<*mut u16>())
            .read())
            .wrapping_offset(((i) as i32) as isize))
            .read())
                != 0)
            {
                break 'l1;
            }
            let __p1 = ((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            i = (i).wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShopMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut inputCode: i8 = Menu_ProcessInputNoWrap();
        'l1: {
            let __sw1 = ((inputCode) as i32);
            let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
            if __sw1 == (-2i32) {
                break 'l1;
            }
            if __sw1 == (-1i32) {
                PlaySE(5u16);
                Task_HandleShopMenuQuit(taskId);
                break 'l1;
            }
            if !__matched {
                ((((((((&raw mut sMartInfo).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((inputCode) as i32) as isize * 8))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleShopMenuBuy(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(8))
            .write((((CB2_InitBuyMenu as *const () as usize as u32) >> 16) as i16));
        ((data).wrapping_offset(9)).write(((CB2_InitBuyMenu as *const () as usize as u32) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_GoToBuyOrSellMenu));
        FadeScreen(1u8, 0i8);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleShopMenuSell(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(8))
            .write((((CB2_GoToSellMenu as *const () as usize as u32) >> 16) as i16));
        ((data).wrapping_offset(9)).write(((CB2_GoToSellMenu as *const () as usize as u32) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_GoToBuyOrSellMenu));
        FadeScreen(1u8, 0i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ExitSellMenu() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(MapPostLoadHook_ReturnToShopMenu));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleShopMenuQuit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearStdWindowAndFrameToTransparent(
            (((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read(),
            2u8,
        );
        RemoveWindow((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(14)).read());
        TryPutSmartShopperOnAir();
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
        if ((((&raw mut sMartInfo).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read())
            .is_some()
        {
            ((((&raw mut sMartInfo).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>()).read())
                .unwrap_unchecked()();
        }
    }
}
pub(crate) unsafe extern "C" fn Task_GoToBuyOrSellMenu(taskId: u8) {
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
            DestroyTask(taskId);
            SetMainCallback2(
                (core::mem::transmute::<usize, Option<unsafe extern "C" fn()>>(
                    (((((((data).wrapping_offset(8)).read()) as u16) as i32) << 16)
                        | (((((data).wrapping_offset(9)).read()) as u16) as i32))
                        as usize,
                )),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MapPostLoadHook_ReturnToShopMenu() {
    unsafe {
        FadeInFromBlack();
        CreateTask(Some(Task_ReturnToShopMenu), 8u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToShopMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
            if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 2i32 {
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gText_CanIHelpWithAnythingElse).cast::<u8>(),
                    Some(ShowShopMenuAfterExitingBuyOrSellMenu),
                );
            } else {
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gText_AnythingElseICanHelp).cast::<u8>(),
                    Some(ShowShopMenuAfterExitingBuyOrSellMenu),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowShopMenuAfterExitingBuyOrSellMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CreateShopMenu((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read());
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_BuyMenu() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_BuyMenu() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_InitBuyMenu() {
    unsafe {
        let mut taskId: u8 = 0u8;
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                SetVBlankHBlankCallbacksToNull();
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l2: loop {
                        'l3: {
                            CpuFastSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((117440512i32) as usize as *mut u8),
                                ((16777216i32
                                    | (crate::c::div_i32(1024i32, crate::c::div_i32(32i32, 8i32))
                                        & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l2;
                        }
                    }
                }
                ScanlineEffect_Stop();
                ResetTempTileDataBuffers();
                FreeAllSpritePalettes();
                ResetPaletteFade();
                ResetSpriteData();
                ResetTasks();
                ClearScheduledBgCopiesToVram();
                ((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(8368u32));
                ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8203))
                    .write(255u8);
                (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8205))
                .cast::<u8>())
                .write(255u8);
                ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8205))
                .cast::<u8>())
                .wrapping_offset(1))
                .write(255u8);
                BuyMenuBuildListMenuTemplate();
                BuyMenuInitBgs();
                FillBgTilemapBufferRect_Palette0(0u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(1u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(2u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                FillBgTilemapBufferRect_Palette0(3u8, 0u16, 0u8, 0u8, 32u8, 32u8);
                BuyMenuInitWindows();
                BuyMenuDecompressBgGraphics();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((FreeTempTileDataBuffersIfPossible()) != 0) {
                    let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                BuyMenuDrawGraphics();
                BuyMenuAddScrollIndicatorArrows();
                taskId = CreateTask(Some(Task_BuyMenu), 8u8);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(
                    ((ListMenuInit(
                        (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                        0u16,
                        0u16,
                    )) as i16),
                );
                BlendPalettes(4294967295u32, 16u8, 0u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(Some(VBlankCB_BuyMenu));
                SetMainCallback2(Some(CB2_BuyMenu));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuFreeMemory() {
    unsafe {
        Free(((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read());
        Free(((&raw mut sListMenuItems).cast::<u8>().cast::<*mut u8>()).read());
        Free(((&raw mut sItemNames).cast::<u8>().cast::<*mut u8>()).read());
        FreeAllWindowBuffers();
    }
}
pub(crate) unsafe extern "C" fn BuyMenuBuildListMenuTemplate() {
    unsafe {
        let mut i: u16 = 0u16;
        ((&raw mut sListMenuItems).cast::<u8>().cast::<*mut u8>()).write(Alloc(
            (((((((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)) as u32)
                .wrapping_mul(8u32),
        ));
        ((&raw mut sItemNames).cast::<u8>().cast::<*mut u8>()).write(Alloc(
            (((((((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)) as u32)
                .wrapping_mul(16u32),
        ));
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut sMartInfo).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    BuyMenuSetListEntry(
                        (((&raw mut sListMenuItems).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 8),
                        (((((&raw mut sMartInfo).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((((&raw mut sItemNames).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 16))
                        .cast::<u8>(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        StringCopy(
            ((((&raw mut sItemNames).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((i) as i32) as isize * 16))
            .cast::<u8>(),
            (&raw mut gText_Cancel2).cast::<u8>(),
        );
        (((((&raw mut sListMenuItems).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((i) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write(
            ((((&raw mut sItemNames).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_offset(((i) as i32) as isize * 16))
            .cast::<u8>(),
        );
        (((((&raw mut sListMenuItems).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_offset(((i) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sShopBuyMenuListTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>())
            .write(((&raw mut sListMenuItems).cast::<u8>().cast::<*mut u8>()).read());
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(
            (((((((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)) as u16),
        );
        if (((((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .read()) as i32)
            > 8i32
        {
            (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
            .write(8u16);
        } else {
            (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
            .write(
                (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
                    .wrapping_add(12)
                    .cast::<u16>())
                .read(),
            );
        }
        ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(8196)
            .cast::<u16>())
        .write(
            (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn BuyMenuSetListEntry(menuItem: *mut u8, item: u16, name: *mut u8) {
    unsafe {
        let mut menuItem = menuItem;
        let mut item = item;
        let mut name = name;
        if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 0i32 {
            CopyItemName(item, name);
        } else {
            StringCopy(
                name,
                ((((&raw mut gDecorations).cast::<u8>())
                    .wrapping_offset(((item) as i32) as isize * 32))
                .wrapping_add(1))
                .cast::<u8>(),
            );
        }
        ((menuItem).cast::<*mut u8>()).write(name);
        ((menuItem).wrapping_add(4).cast::<i32>()).write(((item) as i32));
    }
}
pub(crate) unsafe extern "C" fn BuyMenuPrintItemDescriptionAndShowItemIcon(
    item: i32,
    onInit: u8,
    list: *mut u8,
) {
    unsafe {
        let mut item = item;
        let mut onInit = onInit;
        let mut list = list;
        let mut description: *mut u8 = core::ptr::null_mut();
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
        }
        if item != (-2i32) {
            BuyMenuAddItemIcon(
                ((item) as u16),
                ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8204))
                    .read(),
            );
        } else {
            BuyMenuAddItemIcon(
                65535u16,
                ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8204))
                    .read(),
            );
        }
        BuyMenuRemoveItemIcon(
            ((item) as u16),
            ((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8204))
                .read()) as i32)
                ^ 1i32) as u8),
        );
        let __p1 =
            (((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8204);
        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as u8));
        if item != (-2i32) {
            if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 0i32 {
                description = GetItemDescription(((item) as u16));
            } else {
                description = ((((&raw mut gDecorations).cast::<u8>())
                    .wrapping_offset((item) as isize * 32))
                .wrapping_add(24)
                .cast::<*mut u8>())
                .read();
            }
        } else {
            description = (&raw mut gText_QuitShopping).cast::<u8>();
        }
        FillWindowPixelBuffer(2u8, 0u8);
        BuyMenuPrint(2u8, description, 3u8, 1u8, 0i8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn BuyMenuPrintPriceInList(windowId: u8, itemId: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemId = itemId;
        let mut y = y;
        let mut x: u8 = 0u8;
        if itemId != 4294967294u32 {
            if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 0i32 {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    crate::c::shr_i32(
                        ((GetItemPrice(((itemId) as u16))) as i32),
                        ((IsPokeNewsActive(1u8)) as u32),
                    ),
                    0i32,
                    5u8,
                );
            } else {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((((&raw mut gDecorations).cast::<u8>())
                        .wrapping_offset(((itemId) as i32) as isize * 32))
                    .wrapping_add(20)
                    .cast::<u16>())
                    .read()) as i32),
                    0i32,
                    5u8,
                );
            }
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_PokedollarVar1).cast::<u8>(),
            );
            x = ((GetStringRightAlignXOffset(7i32, (&raw mut gStringVar4).cast::<u8>(), 120i32))
                as u8);
            AddTextPrinterParameterized4(
                windowId,
                7u8,
                x,
                y,
                0u8,
                0u8,
                ((((&raw const sShopBuyMenuTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(3))
                .cast::<u8>(),
                (-1i8),
                (&raw mut gStringVar4).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuAddScrollIndicatorArrows() {
    unsafe {
        if (((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8203))
            .read()) as i32)
            == 255i32)
            && ((((((&raw mut sMartInfo).cast::<u8>())
                .wrapping_add(12)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(1i32)
                > 8i32)
        {
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8203))
                .write(AddScrollIndicatorArrowPairParameterized(
                    2u32,
                    172i32,
                    12i32,
                    148i32,
                    (((((&raw mut sMartInfo).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_sub(7i32),
                    2100i32,
                    2100i32,
                    (((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8200)
                        .cast::<u16>(),
                ));
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuRemoveScrollIndicatorArrows() {
    unsafe {
        if ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8203))
            .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8203))
                    .read(),
            );
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8203))
                .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuPrintCursor(scrollIndicatorsTaskId: u8, colorSet: u8) {
    unsafe {
        let mut scrollIndicatorsTaskId = scrollIndicatorsTaskId;
        let mut colorSet = colorSet;
        let mut y: u8 = ((ListMenuGetYCoordForPrintingArrowCursor(scrollIndicatorsTaskId)) as u8);
        BuyMenuPrint(
            1u8,
            (&raw mut gText_SelectorArrow2).cast::<u8>(),
            0u8,
            y,
            0i8,
            colorSet,
        );
    }
}
pub(crate) unsafe extern "C" fn BuyMenuAddItemIcon(item: u16, iconSlot: u8) {
    unsafe {
        let mut item = item;
        let mut iconSlot = iconSlot;
        let mut spriteId: u8 = 0u8;
        let mut spriteIdPtr: *mut u8 =
            (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8205))
                .cast::<u8>())
            .wrapping_offset(((iconSlot) as i32) as isize);
        if (((spriteIdPtr).read()) as i32) != 255i32 {
            return;
        }
        if ((((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 0i32)
            || (((item) as i32) == 65535i32)
        {
            spriteId = AddItemIconSprite(
                ((((iconSlot) as i32).wrapping_add(2110i32)) as u16),
                ((((iconSlot) as i32).wrapping_add(2110i32)) as u16),
                item,
            );
            if ((spriteId) as i32) != 64i32 {
                (spriteIdPtr).write(spriteId);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(24i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(88i16);
            }
        } else {
            spriteId = AddDecorationIconObject(
                ((item) as u8),
                20i16,
                84i16,
                1u8,
                ((((iconSlot) as i32).wrapping_add(2110i32)) as u16),
                ((((iconSlot) as i32).wrapping_add(2110i32)) as u16),
            );
            if ((spriteId) as i32) != 64i32 {
                (spriteIdPtr).write(spriteId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuRemoveItemIcon(item: u16, iconSlot: u8) {
    unsafe {
        let mut item = item;
        let mut iconSlot = iconSlot;
        let mut spriteIdPtr: *mut u8 =
            (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8205))
                .cast::<u8>())
            .wrapping_offset(((iconSlot) as i32) as isize);
        if (((spriteIdPtr).read()) as i32) == 255i32 {
            return;
        }
        FreeSpriteTilesByTag(((((iconSlot) as i32).wrapping_add(2110i32)) as u16));
        FreeSpritePaletteByTag(((((iconSlot) as i32).wrapping_add(2110i32)) as u16));
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset((((spriteIdPtr).read()) as i32) as isize * 68),
        );
        (spriteIdPtr).write(255u8);
    }
}
pub(crate) unsafe extern "C" fn BuyMenuInitBgs() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sShopBuyMenuBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            1u8,
            ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(2048))
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(6144))
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(4096))
            .cast::<u16>())
            .cast::<u8>(),
        );
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(28u8, 0u16);
        SetGpuReg(30u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDecompressBgGraphics() {
    unsafe {
        DecompressAndCopyTileDataToVram(
            1u8,
            (((&raw mut gShopMenu_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            928u32,
            995u16,
            0u8,
        );
        LZDecompressWram(
            ((&raw mut gShopMenu_Tilemap).cast::<u32>()).cast::<u32>(),
            (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .cast::<u16>())
            .cast::<u8>(),
        );
        LoadCompressedPalette(
            ((&raw mut gShopMenu_Pal).cast::<u32>()).cast::<u32>(),
            192u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn BuyMenuInitWindows() {
    unsafe {
        InitWindows(
            ((&raw const sShopBuyMenuWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 1u16, 208u8);
        LoadMessageBoxGfx(0u8, 10u16, 224u8);
        PutWindowTilemap(0u8);
        PutWindowTilemap(1u8);
        PutWindowTilemap(2u8);
    }
}
pub(crate) unsafe extern "C" fn BuyMenuPrint(
    windowId: u8,
    text: *mut u8,
    x: u8,
    y: u8,
    speed: i8,
    colorSet: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut text = text;
        let mut x = x;
        let mut y = y;
        let mut speed = speed;
        let mut colorSet = colorSet;
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            x,
            y,
            0u8,
            0u8,
            ((((&raw const sShopBuyMenuTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorSet) as i32) as isize * 3))
            .cast::<u8>(),
            speed,
            text,
        );
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDisplayMessage(
    taskId: u8,
    text: *mut u8,
    callback: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut text = text;
        let mut callback = callback;
        DisplayMessageAndContinueTask(
            taskId,
            5u8,
            10u16,
            14u8,
            1u8,
            GetPlayerTextSpeedDelay(),
            text,
            core::mem::transmute::<_, *mut u8>(callback),
        );
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDrawGraphics() {
    unsafe {
        BuyMenuDrawMapGraphics();
        BuyMenuCopyMenuBgToBg1TilemapBuffer();
        AddMoneyLabelObject(19u16, 11u16);
        PrintMoneyAmountInMoneyBoxWithBorder(
            0u8,
            1u16,
            13u8,
            ((GetMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
            )) as i32),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDrawMapGraphics() {
    unsafe {
        BuyMenuCollectObjectEventData();
        BuyMenuDrawObjectEvents();
        BuyMenuDrawMapBg();
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDrawMapBg() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut mapLayout: *mut u8 = core::ptr::null_mut();
        let mut metatile: u16 = 0u16;
        let mut metatileLayerType: u8 = 0u8;
        mapLayout = (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read();
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        x = ((((x) as i32).wrapping_sub(4i32)) as i16);
        y = ((((y) as i32).wrapping_sub(4i32)) as i16);
        {
            j = 0i16;
            'l1: loop {
                if !(((j) as i32) < 10i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        i = 0i16;
                        'l3: loop {
                            if !(((i) as i32) < 15i32) {
                                break 'l3;
                            }
                            'l4: {
                                metatile = ((MapGridGetMetatileIdAt(
                                    ((x) as i32).wrapping_add(((i) as i32)),
                                    ((y) as i32).wrapping_add(((j) as i32)),
                                )) as u16);
                                if ((BuyMenuCheckForOverlapWithMenuBg(((i) as i32), ((j) as i32)))
                                    as i32)
                                    == 1i32
                                {
                                    metatileLayerType = MapGridGetMetatileLayerTypeAt(
                                        ((x) as i32).wrapping_add(((i) as i32)),
                                        ((y) as i32).wrapping_add(((j) as i32)),
                                    );
                                } else {
                                    metatileLayerType = 1u8;
                                }
                                if ((metatile) as i32) < 512i32 {
                                    BuyMenuDrawMapMetatile(
                                        i,
                                        j,
                                        (((((mapLayout).wrapping_add(16).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(12)
                                        .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(
                                            (((metatile) as i32).wrapping_mul(8i32)) as isize,
                                        ),
                                        metatileLayerType,
                                    );
                                } else {
                                    BuyMenuDrawMapMetatile(
                                        i,
                                        j,
                                        (((((mapLayout).wrapping_add(20).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(12)
                                        .cast::<*mut u16>())
                                        .read())
                                        .wrapping_offset(
                                            ((((metatile) as i32).wrapping_sub(512i32))
                                                .wrapping_mul(8i32))
                                                as isize,
                                        ),
                                        metatileLayerType,
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDrawMapMetatile(
    x: i16,
    y: i16,
    src: *mut u16,
    metatileLayerType: u8,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut src = src;
        let mut metatileLayerType = metatileLayerType;
        let mut offset1: u16 = ((((x) as i32).wrapping_mul(2i32)) as u16);
        let mut offset2: u16 = ((((y) as i32).wrapping_mul(64i32)) as u16);
        'l1: {
            let __sw1 = ((metatileLayerType) as i32);
            if __sw1 == 0i32 {
                BuyMenuDrawMapMetatileLayer(
                    (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(6144))
                    .cast::<u16>(),
                    ((offset1) as i16),
                    ((offset2) as i16),
                    src,
                );
                BuyMenuDrawMapMetatileLayer(
                    (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(2048))
                    .cast::<u16>(),
                    ((offset1) as i16),
                    ((offset2) as i16),
                    (src).wrapping_offset(4),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                BuyMenuDrawMapMetatileLayer(
                    (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(4096))
                    .cast::<u16>(),
                    ((offset1) as i16),
                    ((offset2) as i16),
                    src,
                );
                BuyMenuDrawMapMetatileLayer(
                    (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(6144))
                    .cast::<u16>(),
                    ((offset1) as i16),
                    ((offset2) as i16),
                    (src).wrapping_offset(4),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                BuyMenuDrawMapMetatileLayer(
                    (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(4096))
                    .cast::<u16>(),
                    ((offset1) as i16),
                    ((offset2) as i16),
                    src,
                );
                BuyMenuDrawMapMetatileLayer(
                    (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(2048))
                    .cast::<u16>(),
                    ((offset1) as i16),
                    ((offset2) as i16),
                    (src).wrapping_offset(4),
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDrawMapMetatileLayer(
    dest: *mut u16,
    offset1: i16,
    offset2: i16,
    src: *mut u16,
) {
    unsafe {
        let mut dest = dest;
        let mut offset1 = offset1;
        let mut offset2 = offset2;
        let mut src = src;
        ((dest).wrapping_offset((((offset1) as i32).wrapping_add(((offset2) as i32))) as isize))
            .write((src).read());
        ((dest).wrapping_offset(
            ((((offset1) as i32).wrapping_add(((offset2) as i32))).wrapping_add(1i32)) as isize,
        ))
        .write(((src).wrapping_offset(1)).read());
        ((dest).wrapping_offset(
            ((((offset1) as i32).wrapping_add(((offset2) as i32))).wrapping_add(32i32)) as isize,
        ))
        .write(((src).wrapping_offset(2)).read());
        ((dest).wrapping_offset(
            ((((offset1) as i32).wrapping_add(((offset2) as i32))).wrapping_add(33i32)) as isize,
        ))
        .write(((src).wrapping_offset(3)).read());
    }
}
pub(crate) unsafe extern "C" fn BuyMenuCollectObjectEventData() {
    unsafe {
        let mut facingX: i16 = 0i16;
        let mut facingY: i16 = 0i16;
        let mut y: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut numObjects: u8 = 0u8;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut facingX, &raw mut facingY);
        {
            y = 0u8;
            'l1: loop {
                if !(((y) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    (((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8208))
                    .cast::<u8>())
                    .wrapping_offset(((y) as i32) as isize * 10))
                    .cast::<i16>())
                    .write(16i16);
                }
                y = (y).wrapping_add(1);
            }
        }
        {
            y = 0u8;
            'l3: loop {
                if !(((y) as i32) < 5i32) {
                    break 'l3;
                }
                'l4: {
                    {
                        x = 0u8;
                        'l5: loop {
                            if !(((x) as i32) < 7i32) {
                                break 'l5;
                            }
                            'l6: {
                                let mut objEventId: u8 = GetObjectEventIdByXY(
                                    (((((facingX) as i32).wrapping_sub(4i32))
                                        .wrapping_add(((x) as i32)))
                                        as i16),
                                    (((((facingY) as i32).wrapping_sub(2i32))
                                        .wrapping_add(((y) as i32)))
                                        as i16),
                                );
                                if ((objEventId) as i32) != 16i32 {
                                    (((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(8208))
                                    .cast::<u8>())
                                    .wrapping_offset(((numObjects) as i32) as isize * 10))
                                    .cast::<i16>())
                                    .write(((objEventId) as i16));
                                    ((((((((&raw mut sShopData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8208))
                                    .cast::<u8>())
                                    .wrapping_offset(((numObjects) as i32) as isize * 10))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .write(((x) as i16));
                                    ((((((((&raw mut sShopData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8208))
                                    .cast::<u8>())
                                    .wrapping_offset(((numObjects) as i32) as isize * 10))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .write(((y) as i16));
                                    ((((((((&raw mut sShopData)
                                        .cast::<u8>()
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(8208))
                                    .cast::<u8>())
                                    .wrapping_offset(((numObjects) as i32) as isize * 10))
                                    .cast::<i16>())
                                    .wrapping_offset(4))
                                    .write(
                                        ((MapGridGetMetatileLayerTypeAt(
                                            (((facingX) as i32).wrapping_sub(4i32))
                                                .wrapping_add(((x) as i32)),
                                            (((facingY) as i32).wrapping_sub(2i32))
                                                .wrapping_add(((y) as i32)),
                                        )) as i16),
                                    );
                                    'l7: {
                                        let __sw1 = ((crate::c::bf_read(
                                            (((&raw mut gObjectEvents).cast::<u8>())
                                                .wrapping_offset(
                                                    ((objEventId) as i32) as isize * 36,
                                                ))
                                            .wrapping_add(24),
                                            0,
                                            4,
                                            false,
                                        )
                                            as u16)
                                            as i32);
                                        let __matched = __sw1 == 1i32
                                            || __sw1 == 2i32
                                            || __sw1 == 3i32
                                            || __sw1 == 4i32;
                                        if __sw1 == 1i32 {
                                            ((((((((&raw mut sShopData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8208))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((numObjects) as i32) as isize * 10,
                                            ))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(0i16);
                                            break 'l7;
                                        }
                                        if __sw1 == 2i32 {
                                            ((((((((&raw mut sShopData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8208))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((numObjects) as i32) as isize * 10,
                                            ))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(1i16);
                                            break 'l7;
                                        }
                                        if __sw1 == 3i32 {
                                            ((((((((&raw mut sShopData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8208))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((numObjects) as i32) as isize * 10,
                                            ))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(2i16);
                                            break 'l7;
                                        }
                                        if __sw1 == 4i32 || !__matched {
                                            ((((((((&raw mut sShopData)
                                                .cast::<u8>()
                                                .cast::<*mut u8>())
                                            .read())
                                            .wrapping_add(8208))
                                            .cast::<u8>())
                                            .wrapping_offset(
                                                ((numObjects) as i32) as isize * 10,
                                            ))
                                            .cast::<i16>())
                                            .wrapping_offset(3))
                                            .write(3i16);
                                            break 'l7;
                                        }
                                    }
                                    numObjects = (numObjects).wrapping_add(1);
                                }
                            }
                            x = (x).wrapping_add(1);
                        }
                    }
                }
                y = (y).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuDrawObjectEvents() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut graphicsInfo: *mut u8 = core::ptr::null_mut();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8208))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 10))
                    .cast::<i16>())
                    .read()) as i32)
                        == 16i32
                    {
                        break 'l2;
                    }
                    graphicsInfo = GetObjectEventGraphicsInfo(
                        ((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(8208))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 10))
                            .cast::<i16>())
                            .read()) as i32) as isize
                                * 36,
                        ))
                        .wrapping_add(5))
                        .read(),
                    );
                    spriteId = CreateObjectGraphicsSprite(
                        ((((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                            (((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(8208))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 10))
                            .cast::<i16>())
                            .read()) as i32) as isize
                                * 36,
                        ))
                        .wrapping_add(5))
                        .read()) as u16),
                        Some(SpriteCallbackDummy),
                        ((((((((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8208))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 10))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u16) as i32)
                            .wrapping_mul(16i32))
                        .wrapping_add(8i32)) as i16),
                        (((((((((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .wrapping_add(8208))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 10))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u16) as i32)
                            .wrapping_mul(16i32))
                        .wrapping_add(48i32))
                        .wrapping_sub(crate::c::div_i32(
                            ((((graphicsInfo).wrapping_add(10).cast::<i16>()).read()) as i32),
                            2i32,
                        ))) as i16),
                        2u8,
                    );
                    if ((BuyMenuCheckIfObjectEventOverlapsMenuBg(
                        ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8208))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 10))
                        .cast::<i16>(),
                    )) as i32)
                        == 1i32
                    {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(66),
                            0,
                            6,
                            (4u8) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(66),
                            6,
                            2,
                            (1u8) as i32,
                        );
                    }
                    StartSpriteAnim(
                        ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68),
                        ((((((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8208))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 10))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as u8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuCheckIfObjectEventOverlapsMenuBg(object: *mut i16) -> u8 {
    unsafe {
        let mut object = object;
        if (!((BuyMenuCheckForOverlapWithMenuBg(
            ((((object).wrapping_offset(1)).read()) as i32),
            ((((object).wrapping_offset(2)).read()) as i32).wrapping_add(2i32),
        )) != 0))
            && (((((object).wrapping_offset(4)).read()) as i32) != 1i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuCopyMenuBgToBg1TilemapBuffer() {
    unsafe {
        let mut i: i16 = 0i16;
        let mut dest: *mut u16 = (((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<u8>())
        .wrapping_offset(2048))
        .cast::<u16>();
        let mut src: *mut u16 = ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<u8>())
        .cast::<u16>();
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 1024i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((src).wrapping_offset(((i) as i32) as isize)).read()) as i32) != 0i32 {
                        ((dest).wrapping_offset(((i) as i32) as isize)).write(
                            ((((((src).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                .wrapping_add(50147i32)) as u16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuCheckForOverlapWithMenuBg(x: i32, y: i32) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatile: *mut u16 =
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .cast::<u16>();
        let mut offset1: i32 = (x).wrapping_mul(2i32);
        let mut offset2: i32 = (y).wrapping_mul(64i32);
        if (((((((metatile).wrapping_offset(((offset2).wrapping_add(offset1)) as isize)).read())
            as i32)
            == 0i32)
            && (((((metatile)
                .wrapping_offset((((offset2).wrapping_add(offset1)).wrapping_add(32i32)) as isize))
            .read()) as i32)
                == 0i32))
            && (((((metatile)
                .wrapping_offset((((offset2).wrapping_add(offset1)).wrapping_add(1i32)) as isize))
            .read()) as i32)
                == 0i32))
            && (((((metatile)
                .wrapping_offset((((offset2).wrapping_add(offset1)).wrapping_add(33i32)) as isize))
            .read()) as i32)
                == 0i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_BuyMenu(taskId: u8) {
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
            let mut itemId: i32 =
                ListMenu_ProcessInput(((((data).wrapping_offset(7)).read()) as u8));
            ListMenuGetScrollAndRow(
                ((((data).wrapping_offset(7)).read()) as u8),
                (((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8200)
                    .cast::<u16>(),
                (((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(8198)
                    .cast::<u16>(),
            );
            'l1: {
                let __sw1 = itemId;
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    PlaySE(5u16);
                    ExitBuyMenu(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    ((data).wrapping_offset(5)).write(((itemId) as i16));
                    ClearWindowTilemap(2u8);
                    BuyMenuRemoveScrollIndicatorArrows();
                    BuyMenuPrintCursor(((((data).wrapping_offset(7)).read()) as u8), 2u8);
                    if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32)
                        == 0i32
                    {
                        ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8192)
                            .cast::<u32>())
                        .write(
                            ((crate::c::shr_i32(
                                ((GetItemPrice(((itemId) as u16))) as i32),
                                ((IsPokeNewsActive(1u8)) as u32),
                            )) as u32),
                        );
                    } else {
                        ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8192)
                            .cast::<u32>())
                        .write(
                            ((((((&raw mut gDecorations).cast::<u8>())
                                .wrapping_offset((itemId) as isize * 32))
                            .wrapping_add(20)
                            .cast::<u16>())
                            .read()) as u32),
                        );
                    }
                    if !((IsEnoughMoney(
                        (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1168)
                            .cast::<u32>(),
                        ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(8192)
                            .cast::<u32>())
                        .read(),
                    )) != 0)
                    {
                        BuyMenuDisplayMessage(
                            taskId,
                            (&raw mut gText_YouDontHaveMoney).cast::<u8>(),
                            Some(BuyMenuReturnToItemList),
                        );
                    } else {
                        if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32)
                            == 0i32
                        {
                            CopyItemName(((itemId) as u16), (&raw mut gStringVar1).cast::<u8>());
                            if ((GetItemPocket(((itemId) as u16))) as i32) == 3i32 {
                                StringCopy(
                                    (&raw mut gStringVar2).cast::<u8>(),
                                    (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                                        ((ItemIdToBattleMoveId(((itemId) as u16))) as i32) as isize
                                            * 13,
                                    ))
                                    .cast::<u8>(),
                                );
                                BuyMenuDisplayMessage(
                                    taskId,
                                    (&raw mut gText_Var1CertainlyHowMany2).cast::<u8>(),
                                    Some(Task_BuyHowManyDialogueInit),
                                );
                            } else {
                                BuyMenuDisplayMessage(
                                    taskId,
                                    (&raw mut gText_Var1CertainlyHowMany).cast::<u8>(),
                                    Some(Task_BuyHowManyDialogueInit),
                                );
                            }
                        } else {
                            StringCopy(
                                (&raw mut gStringVar1).cast::<u8>(),
                                ((((&raw mut gDecorations).cast::<u8>())
                                    .wrapping_offset((itemId) as isize * 32))
                                .wrapping_add(1))
                                .cast::<u8>(),
                            );
                            ConvertIntToDecimalStringN(
                                (&raw mut gStringVar2).cast::<u8>(),
                                ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(8192)
                                    .cast::<u32>())
                                .read()) as i32),
                                0i32,
                                6u8,
                            );
                            if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read())
                                as i32)
                                == 1i32
                            {
                                StringExpandPlaceholders(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    (&raw mut gText_Var1IsItThatllBeVar2).cast::<u8>(),
                                );
                            } else {
                                StringExpandPlaceholders(
                                    (&raw mut gStringVar4).cast::<u8>(),
                                    (&raw mut gText_YouWantedVar1ThatllBeVar2).cast::<u8>(),
                                );
                            }
                            BuyMenuDisplayMessage(
                                taskId,
                                (&raw mut gStringVar4).cast::<u8>(),
                                Some(BuyMenuConfirmPurchase),
                            );
                        }
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_BuyHowManyDialogueInit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut quantityInBag: u16 =
            CountTotalItemQuantityInBag(((((data).wrapping_offset(5)).read()) as u16));
        let mut maxQuantity: u16 = 0u16;
        DrawStdFrameWithCustomTileAndPalette(3u8, 0u8, 1u16, 13u8);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((quantityInBag) as i32),
            1i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_InBagVar1).cast::<u8>(),
        );
        BuyMenuPrint(3u8, (&raw mut gStringVar4).cast::<u8>(), 0u8, 1u8, 0i8, 0u8);
        ((data).wrapping_offset(1)).write(1i16);
        DrawStdFrameWithCustomTileAndPalette(4u8, 0u8, 1u16, 13u8);
        BuyMenuPrintItemQuantityAndPrice(taskId);
        ScheduleBgCopyTilemapToVram(0u8);
        maxQuantity = ((crate::c::div_u32(
            GetMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
            ),
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8192)
                .cast::<u32>())
            .read(),
        )) as u16);
        if ((maxQuantity) as i32) > 99i32 {
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8202))
                .write(99u8);
        } else {
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8202))
                .write(((maxQuantity) as u8));
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_BuyHowManyDialogueHandleInput));
    }
}
pub(crate) unsafe extern "C" fn Task_BuyHowManyDialogueHandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((AdjustQuantityAccordingToDPadInput(
            (data).wrapping_offset(1),
            ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(8202))
                .read()) as u16),
        )) as i32)
            == 1i32
        {
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8192)
                .cast::<u32>())
            .write(
                (((crate::c::shr_i32(
                    ((GetItemPrice(((((data).wrapping_offset(5)).read()) as u16))) as i32),
                    ((IsPokeNewsActive(1u8)) as u32),
                ))
                .wrapping_mul(((((data).wrapping_offset(1)).read()) as i32)))
                    as u32),
            );
            BuyMenuPrintItemQuantityAndPrice(taskId);
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                PlaySE(5u16);
                ClearStdWindowAndFrameToTransparent(4u8, 0u8);
                ClearStdWindowAndFrameToTransparent(3u8, 0u8);
                ClearWindowTilemap(4u8);
                ClearWindowTilemap(3u8);
                PutWindowTilemap(1u8);
                CopyItemName(
                    ((((data).wrapping_offset(5)).read()) as u16),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((data).wrapping_offset(1)).read()) as i32),
                    0i32,
                    2u8,
                );
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar3).cast::<u8>(),
                    ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(8192)
                        .cast::<u32>())
                    .read()) as i32),
                    0i32,
                    6u8,
                );
                BuyMenuDisplayMessage(
                    taskId,
                    (&raw mut gText_Var1AndYouWantedVar2).cast::<u8>(),
                    Some(BuyMenuConfirmPurchase),
                );
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(5u16);
                    ClearStdWindowAndFrameToTransparent(4u8, 0u8);
                    ClearStdWindowAndFrameToTransparent(3u8, 0u8);
                    ClearWindowTilemap(4u8);
                    ClearWindowTilemap(3u8);
                    BuyMenuReturnToItemList(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuConfirmPurchase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CreateYesNoMenuWithCallbacks(
            taskId,
            (&raw const sShopBuyMenuYesNoWindowTemplates)
                .cast::<u8>()
                .cast_mut(),
            1u8,
            0u8,
            0u8,
            1u16,
            13u8,
            (&raw const sShopPurchaseYesNoFuncs).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn BuyMenuTryMakePurchase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PutWindowTilemap(1u8);
        if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 0i32 {
            if ((AddBagItem(
                ((((data).wrapping_offset(5)).read()) as u16),
                ((((data).wrapping_offset(1)).read()) as u16),
            )) as i32)
                == 1i32
            {
                BuyMenuDisplayMessage(
                    taskId,
                    (&raw mut gText_HereYouGoThankYou).cast::<u8>(),
                    Some(BuyMenuSubtractMoney),
                );
                RecordItemPurchase(taskId);
            } else {
                BuyMenuDisplayMessage(
                    taskId,
                    (&raw mut gText_NoMoreRoomForThis).cast::<u8>(),
                    Some(BuyMenuReturnToItemList),
                );
            }
        } else {
            if (DecorationAdd(((((data).wrapping_offset(5)).read()) as u8))) != 0 {
                if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 1i32
                {
                    BuyMenuDisplayMessage(
                        taskId,
                        (&raw mut gText_ThankYouIllSendItHome).cast::<u8>(),
                        Some(BuyMenuSubtractMoney),
                    );
                } else {
                    BuyMenuDisplayMessage(
                        taskId,
                        (&raw mut gText_ThanksIllSendItHome).cast::<u8>(),
                        Some(BuyMenuSubtractMoney),
                    );
                }
            } else {
                BuyMenuDisplayMessage(
                    taskId,
                    (&raw mut gText_SpaceForVar1Full).cast::<u8>(),
                    Some(BuyMenuReturnToItemList),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuSubtractMoney(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        IncrementGameStat(38u8);
        RemoveMoney(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1168)
                .cast::<u32>(),
            ((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8192)
                .cast::<u32>())
            .read(),
        );
        PlaySE(95u16);
        PrintMoneyAmountInMoneyBox(
            0u8,
            ((GetMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
            )) as i32),
            0u8,
        );
        if (((((&raw mut sMartInfo).cast::<u8>()).wrapping_add(15)).read()) as i32) == 0i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReturnToItemListAfterItemPurchase));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ReturnToItemListAfterDecorationPurchase));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToItemListAfterItemPurchase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            PlaySE(5u16);
            if ((((((data).wrapping_offset(5)).read()) as i32) == 4i32)
                && (((((data).wrapping_offset(1)).read()) as i32) >= 10i32))
                && (((AddBagItem(12u16, 1u16)) as i32) == 1i32)
            {
                BuyMenuDisplayMessage(
                    taskId,
                    (&raw mut gText_ThrowInPremierBall).cast::<u8>(),
                    Some(BuyMenuReturnToItemList),
                );
            } else {
                BuyMenuReturnToItemList(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ReturnToItemListAfterDecorationPurchase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            PlaySE(5u16);
            BuyMenuReturnToItemList(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn BuyMenuReturnToItemList(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ClearDialogWindowAndFrameToTransparent(5u8, 0u8);
        BuyMenuPrintCursor(((((data).wrapping_offset(7)).read()) as u8), 1u8);
        PutWindowTilemap(1u8);
        PutWindowTilemap(2u8);
        ScheduleBgCopyTilemapToVram(0u8);
        BuyMenuAddScrollIndicatorArrows();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_BuyMenu));
    }
}
pub(crate) unsafe extern "C" fn BuyMenuPrintItemQuantityAndPrice(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        FillWindowPixelBuffer(4u8, 17u8);
        PrintMoneyAmount(
            4u8,
            38u8,
            1u8,
            ((((((&raw mut sShopData).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(8192)
                .cast::<u32>())
            .read()) as i32),
            255u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((data).wrapping_offset(1)).read()) as i32),
            2i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        BuyMenuPrint(4u8, (&raw mut gStringVar4).cast::<u8>(), 0u8, 1u8, 0i8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn ExitBuyMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(MapPostLoadHook_ReturnToShopMenu));
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ExitBuyMenu));
    }
}
pub(crate) unsafe extern "C" fn Task_ExitBuyMenu(taskId: u8) {
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
            RemoveMoneyLabelObject();
            BuyMenuFreeMemory();
            SetMainCallback2(Some(CB2_ReturnToField));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ClearItemPurchases() {
    unsafe {
        ((&raw mut sPurchaseHistoryId).cast::<u8>().cast::<u8>()).write(0u8);
        crate::c::memset(
            ((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>(),
            0i32,
            12u32,
        );
    }
}
pub(crate) unsafe extern "C" fn RecordItemPurchase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(12u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((((data).wrapping_offset(5)).read()) as i32))
                        && ((((((((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            != 0i32)
                    {
                        if (((((((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(((((data).wrapping_offset(1)).read()) as i32))
                            > 255i32
                        {
                            (((((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .write(255u16);
                        } else {
                            let __p1 = ((((&raw mut gMartPurchaseHistory).cast::<u8>())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>();
                            (__p1).write(
                                (((((__p1).read()) as i32)
                                    .wrapping_add(((((data).wrapping_offset(1)).read()) as i32)))
                                    as u16),
                            );
                        }
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut sPurchaseHistoryId).cast::<u8>().cast::<u8>()).read()) as u32)
            < crate::c::div_u32(12u32, 4u32)
        {
            (((((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut sPurchaseHistoryId).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 4,
            ))
            .cast::<u16>())
            .write(((((data).wrapping_offset(5)).read()) as u16));
            (((((&raw mut gMartPurchaseHistory).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut sPurchaseHistoryId).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 4,
            ))
            .wrapping_add(2)
            .cast::<u16>())
            .write(((((data).wrapping_offset(1)).read()) as u16));
            let __p2 = (&raw mut sPurchaseHistoryId).cast::<u8>().cast::<u8>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePokemartMenu(itemsForSale: *mut u16) {
    unsafe {
        let mut itemsForSale = itemsForSale;
        CreateShopMenu(0u8);
        SetShopItemsForSale(itemsForSale);
        ClearItemPurchases();
        SetShopMenuCallback(Some(ScriptContext_Enable));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateDecorationShop1Menu(itemsForSale: *mut u16) {
    unsafe {
        let mut itemsForSale = itemsForSale;
        CreateShopMenu(1u8);
        SetShopItemsForSale(itemsForSale);
        SetShopMenuCallback(Some(ScriptContext_Enable));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateDecorationShop2Menu(itemsForSale: *mut u16) {
    unsafe {
        let mut itemsForSale = itemsForSale;
        CreateShopMenu(2u8);
        SetShopItemsForSale(itemsForSale);
        SetShopMenuCallback(Some(ScriptContext_Enable));
    }
}
