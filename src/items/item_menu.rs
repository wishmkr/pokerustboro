//! Translated from `src/item_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBgTemplates_ItemMenu sItemListMenu sItemMenuActions sContextMenuItems_ItemsPocket sContextMenuItems_KeyItemsPocket sContextMenuItems_BallsPocket sContextMenuItems_TmHmPocket sContextMenuItems_BerriesPocket sContextMenuItems_BattleUse sContextMenuItems_Give sContextMenuItems_Cancel sContextMenuItems_BerryBlenderCrush sContextMenuItems_Apprentice sContextMenuItems_FavorLady sContextMenuItems_QuizLady sContextMenuFuncs sYesNoTossFunctions sYesNoSellItemFunctions sBagScrollArrowsTemplate sRegisteredSelect_Gfx sFontColorTable sDefaultBagWindows sContextMenuWindowTemplates
#[allow(unused_imports)]
use crate::data::item_menu::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBagMenu: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBagPosition: crate::ffi::Align4<[u8; 28]> = crate::ffi::Align4([0; 28]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sListBuffer1: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sListBuffer2: *mut u8 = core::ptr::null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialVar_ItemId: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTempWallyBag: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut EventScript_SelectWithoutRegisteredItem: u8;
    static mut gBagFemaleSpriteSheet: u8;
    static mut gBagMaleSpriteSheet: u8;
    static mut gBagMenuHMIcon_Gfx: u8;
    static mut gBagMenu_ReturnToStrings: u8;
    static mut gBagPaletteTable: u8;
    static mut gBagPockets: u8;
    static mut gBagScreenFemale_Pal: u8;
    static mut gBagScreenMale_Pal: u8;
    static mut gBagScreen_Gfx: u8;
    static mut gBagScreen_GfxTileMap: u8;
    static mut gBattleMoves: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gMoveNames: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gPocketNamesStringsTable: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8005: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStandardMenuPalette: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_CantBuyKeyItem: u8;
    static mut gText_CantStoreImportantItems: u8;
    static mut gText_CantWriteMail: u8;
    static mut gText_CloseBag: u8;
    static mut gText_ConfirmTossItems: u8;
    static mut gText_DepositHowManyVar1: u8;
    static mut gText_DepositedVar2Var1s: u8;
    static mut gText_HowManyToSell: u8;
    static mut gText_ICanPayVar1: u8;
    static mut gText_MoveVar1Where: u8;
    static mut gText_NoPokemon: u8;
    static mut gText_NoRoomForItems: u8;
    static mut gText_NumberItem_HM: u8;
    static mut gText_NumberItem_TMBerry: u8;
    static mut gText_ReturnToVar1: u8;
    static mut gText_SelectorArrow2: u8;
    static mut gText_ThreeDashes: u8;
    static mut gText_ThrewAwayVar2Var1s: u8;
    static mut gText_TossHowManyVar1s: u8;
    static mut gText_TurnedOverVar1ForVar2: u8;
    static mut gText_Var1CantBeHeld: u8;
    static mut gText_Var1CantBeHeldHere: u8;
    static mut gText_Var1IsSelected: u8;
    static mut gText_xVar1: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddBagItemIconSprite(a0: u16, a1: u8);
    fn AddBagVisualSprite(a0: u8);
    fn AddMoney(a0: *mut u32, a1: u32);
    fn AddMoneyLabelObject(a0: u16, a1: u16);
    fn AddPCItem(a0: u16, a1: u16) -> u8;
    fn AddScrollIndicatorArrowPair(a0: *mut u8, a1: *mut u16) -> u8;
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
    fn AddSwitchPocketRotatingBallSprite(a0: i16);
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
    fn Apprentice_ScriptContext_Enable();
    fn BagGetItemIdByPocketPosition(a0: u8, a1: u16) -> u16;
    fn BagGetQuantityByPocketPosition(a0: u8, a1: u16) -> u16;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn BlitMenuInfoIcon(a0: u8, a1: u8, a2: u16, a3: u16);
    fn BuildOamBuffer();
    fn CB2_ChooseMonToGiveItem();
    fn CB2_ExitSellMenu();
    fn CB2_PlayerPCExitBagMenu();
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldContinueScript();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CB2_SetUpReshowBattleScreenAfterMenu2();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChangeBgY_ScreenOff(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeMenuGridCursorPosition(a0: i8, a1: i8) -> u8;
    fn CheckBagHasItem(a0: u16, a1: u16) -> u8;
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearItemSlots(a0: *mut u8, a1: u8);
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CompactItemsInBagPocket(a0: *mut u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateItemMenuSwapLine();
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
    fn CurrentBattlePyramidLocation() -> u8;
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
    fn DoBerryTagScreen();
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FieldCallback_FavorLadyEnableScriptContexts();
    fn FieldCallback_QuizLadyEnableScriptContexts();
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FreezeObjectEvents();
    fn GetItemBattleFunc(a0: u16) -> Option<unsafe extern "C" fn(u8)>;
    fn GetItemBattleUsage(a0: u16) -> u8;
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemFieldFunc(a0: u16) -> Option<unsafe extern "C" fn(u8)>;
    fn GetItemImportance(a0: u16) -> u8;
    fn GetItemPrice(a0: u16) -> u16;
    fn GetItemType(a0: u16) -> u8;
    fn GetLRKeysPressed() -> u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetMoney(a0: *mut u32) -> u32;
    fn GetPlayerTextSpeedDelay() -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GoToBattlePyramidBagMenu(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn HideMapNamePopUpWindow();
    fn InBattlePike() -> u8;
    fn InMultiPartnerRoom() -> u8;
    fn InUnionRoom() -> u32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuActionGrid(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsHoldingItemAllowed(a0: u16) -> u8;
    fn IsWritingMailAllowed(a0: u16) -> u8;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn ItemIsMail(a0: u16) -> u8;
    fn ItemUseOutOfBattle_Berry(a0: u8);
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuGetYCoordForPrintingArrowCursor(a0: u8) -> u16;
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenuLoadStdPalAt(a0: u8, a1: u8);
    fn ListMenuSetTemplateField(a0: u8, a1: u8, a2: i32);
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadListMenuSwapLineGfx();
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn MoveItemSlotInList(a0: *mut u8, a1: u32, a2: u32);
    fn PlaySE(a0: u16);
    fn PlayerFreeze();
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
    fn PrintMoneyAmount(a0: u8, a1: u8, a2: u8, a3: i32, a4: u8);
    fn PrintMoneyAmountInMoneyBox(a0: u8, a1: i32, a2: u8);
    fn PrintMoneyAmountInMoneyBoxWithBorder(a0: u8, a1: u16, a2: u8, a3: i32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn RemoveBagItemIconSprite(a0: u8);
    fn RemoveBagSprite(a0: u8);
    fn RemoveMoneyLabelObject();
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
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetBagVisualPocketId(a0: u8, a1: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetCursorScrollWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8, a4: u8);
    fn SetCursorWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetItemMenuSwapLineInvisibility(a0: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShakeBagSprite();
    fn ShowBg(a0: u8);
    fn SortBerriesOrTMHMs(a0: *mut u8);
    fn StopPlayerAvatar();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TransferPlttBuffer();
    fn UpdateItemMenuSwapLinePos(a0: u8);
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBagScrollPositions() {
    unsafe {
        (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).write(0u8);
        crate::c::memset(
            ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>()).cast::<u8>(),
            0i32,
            10u32,
        );
        crate::c::memset(
            ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>()).cast::<u8>(),
            0i32,
            10u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_BagMenuFromStartMenu() {
    unsafe {
        GoToBagMenu(0u8, 5u8, Some(CB2_ReturnToFieldWithOpenMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_BagMenuFromBattle() {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
            GoToBagMenu(1u8, 5u8, Some(CB2_SetUpReshowBattleScreenAfterMenu2));
        } else {
            GoToBattlePyramidBagMenu(1u8, Some(CB2_SetUpReshowBattleScreenAfterMenu2));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ChooseBerry() {
    unsafe {
        GoToBagMenu(4u8, 3u8, Some(CB2_ReturnToFieldContinueScript));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChooseBerryForMachine(exitCallback: Option<unsafe extern "C" fn()>) {
    unsafe {
        let mut exitCallback = exitCallback;
        GoToBagMenu(5u8, 3u8, exitCallback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_GoToSellMenu() {
    unsafe {
        GoToBagMenu(3u8, 5u8, Some(CB2_ExitSellMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_GoToItemDepositMenu() {
    unsafe {
        GoToBagMenu(6u8, 5u8, Some(CB2_PlayerPCExitBagMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ApprenticeOpenBagMenu() {
    unsafe {
        GoToBagMenu(9u8, 5u8, Some(CB2_ApprenticeExitBagMenu));
        ((&raw mut gSpecialVar_0x8005).cast::<u16>()).write(0u16);
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FavorLadyOpenBagMenu() {
    unsafe {
        GoToBagMenu(7u8, 5u8, Some(CB2_FavorLadyExitBagMenu));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn QuizLadyOpenBagMenu() {
    unsafe {
        GoToBagMenu(8u8, 5u8, Some(CB2_QuizLadyExitBagMenu));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GoToBagMenu(
    location: u8,
    pocket: u8,
    exitCallback: Option<unsafe extern "C" fn()>,
) {
    unsafe {
        let mut location = location;
        let mut pocket = pocket;
        let mut exitCallback = exitCallback;
        ((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(3144u32));
        if ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()) as usize) == 0usize {
            SetMainCallback2(exitCallback);
        } else {
            if ((location) as i32) != 12i32 {
                (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).write(location);
            }
            if (exitCallback).is_some() {
                (((&raw mut gBagPosition).cast::<u8>()).cast::<Option<unsafe extern "C" fn()>>())
                    .write(exitCallback);
            }
            if ((pocket) as i32) < 5i32 {
                (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).write(pocket);
            }
            if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32) == 4i32)
                || ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    == 5i32)
            {
                crate::c::bf_write(
                    (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2075),
                    0,
                    4,
                    (1u8) as i32,
                );
            }
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(None);
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2074))
                .write(255u8);
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2078))
                .write(255u8);
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2079))
                .write(255u8);
            crate::c::memset(
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2052))
                    .cast::<u8>(),
                255i32,
                12u32,
            );
            crate::c::memset(
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2064))
                    .cast::<u8>(),
                255i32,
                10u32,
            );
            SetMainCallback2(Some(CB2_Bag));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_BagMenuRun() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn VBlankCB_BagMenuRun() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_Bag() {
    unsafe {
        'l1: loop {
            if !(((((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
                && (((SetupBagMenu()) as i32) != 1i32))
                && (((MenuHelpers_IsLinkActive()) as i32) != 1i32))
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetupBagMenu() -> u8 {
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
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32;
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
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                if !((MenuHelpers_IsLinkActive()) != 0) {
                    ResetTasks();
                }
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                BagMenu_InitBGs();
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2100)
                    .cast::<i16>())
                .write(0i16);
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                if !((LoadBagMenu_Graphics()) != 0) {
                    break 'l1;
                }
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                LoadBagMenuTextWindows();
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                UpdatePocketItemLists();
                InitPocketListPositions();
                InitPocketScrollPositions();
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                AllocateBagItemListBuffers();
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                LoadBagItemListBuffers(
                    (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read(),
                );
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                PrintPocketNames(
                    ((((&raw mut gPocketNamesStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ))
                    .read(),
                    core::ptr::null_mut(),
                );
                CopyPocketNameToWindow(0u32);
                DrawPocketIndicatorSquare(
                    (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read(),
                    1u8,
                );
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                taskId = CreateBagInputHandlerTask(
                    (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read(),
                );
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(
                    ((ListMenuInit(
                        (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                    as i32) as isize,
                            ))
                        .read(),
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                    as i32) as isize,
                            ))
                        .read(),
                    )) as i16),
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .write(0i16);
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                AddBagVisualSprite((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 16i32 {
                CreateItemMenuSwapLine();
                let __p18 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p18).write(((__p18).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 17i32 {
                CreatePocketScrollArrowPair();
                CreatePocketSwitchArrowPair();
                let __p19 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 18i32 {
                PrepareTMHMMoveWindow();
                let __p20 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 19i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p21 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p21).write(((__p21).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 20i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p22 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p22).write(((__p22).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VBlankCB_BagMenuRun));
                SetMainCallback2(Some(CB2_BagMenuRun));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BagMenu_InitBGs() {
    unsafe {
        ResetVramOamAndBgCntRegs();
        crate::c::memset(
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<u8>(),
            0i32,
            2048u32,
        );
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates_ItemMenu).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            2u8,
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(4))
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
pub(crate) unsafe extern "C" fn LoadBagMenu_Graphics() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2100)
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
                let __p2 = (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2100)
                    .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LZDecompressWram(
                        ((&raw mut gBagScreen_GfxTileMap).cast::<u32>()).cast::<u32>(),
                        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(4))
                        .cast::<u8>(),
                    );
                    let __p3 = (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2100)
                        .cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (!((IsWallysBag()) != 0))
                    && (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                        .read()) as i32)
                        != 0i32)
                {
                    LoadCompressedPalette(
                        ((&raw mut gBagScreenFemale_Pal).cast::<u32>()).cast::<u32>(),
                        0u16,
                        64u16,
                    );
                } else {
                    LoadCompressedPalette(
                        ((&raw mut gBagScreenMale_Pal).cast::<u32>()).cast::<u32>(),
                        0u16,
                        64u16,
                    );
                }
                let __p4 = (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2100)
                    .cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((IsWallysBag()) as i32) == 1i32)
                    || (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                        .read()) as i32)
                        == 0i32)
                {
                    LoadCompressedSpriteSheet((&raw mut gBagMaleSpriteSheet).cast::<u8>());
                } else {
                    LoadCompressedSpriteSheet((&raw mut gBagFemaleSpriteSheet).cast::<u8>());
                }
                let __p5 = (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2100)
                    .cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadCompressedSpritePalette((&raw mut gBagPaletteTable).cast::<u8>());
                let __p6 = (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2100)
                    .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                LoadListMenuSwapLineGfx();
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2100)
                    .cast::<i16>())
                .write(0i16);
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CreateBagInputHandlerTask(location: u8) -> u8 {
    unsafe {
        let mut location = location;
        let mut taskId: u8 = 0u8;
        if ((location) as i32) == 10i32 {
            taskId = CreateTask(Some(Task_WallyTutorialBagMenu), 0u8);
        } else {
            taskId = CreateTask(Some(Task_BagMenu_HandleInput), 0u8);
        }
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn AllocateBagItemListBuffers() {
    unsafe {
        ((&raw mut sListBuffer1).cast::<u8>().cast::<*mut u8>()).write(Alloc(520u32));
        ((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).write(Alloc(1560u32));
    }
}
pub(crate) unsafe extern "C" fn LoadBagItemListBuffers(pocketId: u8) {
    unsafe {
        let mut pocketId = pocketId;
        let mut i: u16 = 0u16;
        let mut pocket: *mut u8 =
            ((&raw mut gBagPockets).cast::<u8>()).wrapping_offset(((pocketId) as i32) as isize * 8);
        let mut subBuffer: *mut u8 = core::ptr::null_mut();
        if !((crate::c::bf_read(
            (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2075),
            7,
            1,
            false,
        ) as u8)
            != 0)
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2089))
                        .cast::<u8>())
                        .wrapping_offset(((pocketId) as i32) as isize))
                        .read()) as i32)
                            .wrapping_sub(1i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        GetItemNameFromPocket(
                            (((((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .cast::<u8>(),
                            (((((pocket).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                        );
                        subBuffer = (((&raw mut sListBuffer1).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<u8>();
                        (((subBuffer).wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<*mut u8>())
                        .write(
                            (((((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .cast::<u8>(),
                        );
                        (((subBuffer).wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<i32>())
                        .write(((i) as i32));
                    }
                    i = (i).wrapping_add(1);
                }
            }
            StringCopy(
                (((((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                .cast::<u8>(),
                (&raw mut gText_CloseBag).cast::<u8>(),
            );
            subBuffer =
                (((&raw mut sListBuffer1).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>();
            (((subBuffer).wrapping_offset(((i) as i32) as isize * 8)).cast::<*mut u8>()).write(
                (((((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 24))
                .cast::<u8>(),
            );
            (((subBuffer).wrapping_offset(((i) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<i32>())
            .write((-2i32));
        } else {
            {
                i = 0u16;
                'l3: loop {
                    if !(((i) as i32)
                        < ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2089))
                        .cast::<u8>())
                        .wrapping_offset(((pocketId) as i32) as isize))
                        .read()) as i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        GetItemNameFromPocket(
                            (((((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .cast::<u8>(),
                            (((((pocket).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .read(),
                        );
                        subBuffer = (((&raw mut sListBuffer1).cast::<u8>().cast::<*mut u8>())
                            .read())
                        .cast::<u8>();
                        (((subBuffer).wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<*mut u8>())
                        .write(
                            (((((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read())
                                .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 24))
                            .cast::<u8>(),
                        );
                        (((subBuffer).wrapping_offset(((i) as i32) as isize * 8))
                            .wrapping_add(4)
                            .cast::<i32>())
                        .write(((i) as i32));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sItemListMenu)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(
            ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2089))
            .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize))
            .read()) as u16),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>())
            .write((((&raw mut sListBuffer1).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>());
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(
            ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2094))
            .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize))
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn GetItemNameFromPocket(dest: *mut u8, itemId: u16) {
    unsafe {
        let mut dest = dest;
        let mut itemId = itemId;
        'l1: {
            let __sw1 = (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32);
            let __matched = __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 2i32 {
                StringCopy(
                    (&raw mut gStringVar2).cast::<u8>(),
                    (((&raw mut gMoveNames).cast::<u8>())
                        .wrapping_offset(((ItemIdToBattleMoveId(itemId)) as i32) as isize * 13))
                    .cast::<u8>(),
                );
                if ((itemId) as i32) >= 339i32 {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((itemId) as i32).wrapping_sub(339i32)).wrapping_add(1i32),
                        2i32,
                        1u8,
                    );
                    StringExpandPlaceholders(dest, (&raw mut gText_NumberItem_HM).cast::<u8>());
                } else {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        (((itemId) as i32).wrapping_sub(289i32)).wrapping_add(1i32),
                        2i32,
                        2u8,
                    );
                    StringExpandPlaceholders(
                        dest,
                        (&raw mut gText_NumberItem_TMBerry).cast::<u8>(),
                    );
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    (((itemId) as i32).wrapping_sub(133i32)).wrapping_add(1i32),
                    2i32,
                    2u8,
                );
                CopyItemName(itemId, (&raw mut gStringVar2).cast::<u8>());
                StringExpandPlaceholders(dest, (&raw mut gText_NumberItem_TMBerry).cast::<u8>());
                break 'l1;
            }
            if !__matched {
                CopyItemName(itemId, dest);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BagMenu_MoveCursorCallback(
    itemIndex: i32,
    onInit: u8,
    list: *mut u8,
) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut onInit = onInit;
        let mut list = list;
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
            ShakeBagSprite();
        }
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2074))
            .read()) as i32)
            == 255i32
        {
            RemoveBagItemIconSprite(
                ((((crate::c::bf_read(
                    (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2075),
                    4,
                    2,
                    false,
                ) as u8) as i32)
                    ^ 1i32) as u8),
            );
            if itemIndex != (-2i32) {
                AddBagItemIconSprite(
                    BagGetItemIdByPocketPosition(
                        (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                            as i32)
                            .wrapping_add(1i32)) as u8),
                        ((itemIndex) as u16),
                    ),
                    (crate::c::bf_read(
                        (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2075),
                        4,
                        2,
                        false,
                    ) as u8),
                );
            } else {
                AddBagItemIconSprite(
                    65535u16,
                    (crate::c::bf_read(
                        (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2075),
                        4,
                        2,
                        false,
                    ) as u8),
                );
            }
            crate::c::bf_write(
                (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2075),
                4,
                2,
                ((((crate::c::bf_read(
                    (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2075),
                    4,
                    2,
                    false,
                ) as u8) as i32)
                    ^ 1i32) as u8) as i32,
            );
            if !((crate::c::bf_read(
                (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2075),
                6,
                1,
                false,
            ) as u8)
                != 0)
            {
                PrintItemDescription(itemIndex);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn BagMenu_ItemPrintCallback(windowId: u8, itemIndex: u32, y: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemIndex = itemIndex;
        let mut y = y;
        let mut itemId: u16 = 0u16;
        let mut itemQuantity: u16 = 0u16;
        let mut offset: i32 = 0i32;
        if itemIndex != 4294967294u32 {
            if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2074))
                .read()) as i32)
                != 255i32
            {
                if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2074))
                .read()) as i32)
                    == (((itemIndex) as u8) as i32)
                {
                    BagMenu_PrintCursorAtPos(y, 2u8);
                } else {
                    BagMenu_PrintCursorAtPos(y, 255u8);
                }
            }
            itemId = BagGetItemIdByPocketPosition(
                (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    .wrapping_add(1i32)) as u8),
                ((itemIndex) as u16),
            );
            itemQuantity = BagGetQuantityByPocketPosition(
                (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    .wrapping_add(1i32)) as u8),
                ((itemIndex) as u16),
            );
            if (((itemId) as i32) >= 339i32) && (((itemId) as i32) <= 346i32) {
                BlitBitmapToWindow(
                    windowId,
                    (&raw mut gBagMenuHMIcon_Gfx).cast::<u8>(),
                    8u16,
                    ((((y) as i32).wrapping_sub(1i32)) as u16),
                    16u16,
                    16u16,
                );
            }
            if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) == 3i32 {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((itemQuantity) as i32),
                    1i32,
                    3u8,
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_xVar1).cast::<u8>(),
                );
                offset =
                    GetStringRightAlignXOffset(7i32, (&raw mut gStringVar4).cast::<u8>(), 119i32);
                BagMenu_Print(
                    windowId,
                    7u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    ((offset) as u8),
                    y,
                    0u8,
                    0u8,
                    255u8,
                    0u8,
                );
            } else {
                if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    != 4i32)
                    && (((GetItemImportance(itemId)) as i32) == 0i32)
                {
                    ConvertIntToDecimalStringN(
                        (&raw mut gStringVar1).cast::<u8>(),
                        ((itemQuantity) as i32),
                        1i32,
                        2u8,
                    );
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (&raw mut gText_xVar1).cast::<u8>(),
                    );
                    offset = GetStringRightAlignXOffset(
                        7i32,
                        (&raw mut gStringVar4).cast::<u8>(),
                        119i32,
                    );
                    BagMenu_Print(
                        windowId,
                        7u8,
                        (&raw mut gStringVar4).cast::<u8>(),
                        ((offset) as u8),
                        y,
                        0u8,
                        0u8,
                        255u8,
                        0u8,
                    );
                } else {
                    if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1174)
                        .cast::<u16>())
                    .read()) as i32)
                        != 0i32)
                        && (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1174)
                            .cast::<u16>())
                        .read()) as i32)
                            == ((itemId) as i32))
                    {
                        BlitBitmapToWindow(
                            windowId,
                            ((&raw const sRegisteredSelect_Gfx).cast::<u8>().cast_mut())
                                .cast::<u8>(),
                            96u16,
                            ((((y) as i32).wrapping_sub(1i32)) as u16),
                            24u16,
                            16u16,
                        );
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintItemDescription(itemIndex: i32) {
    unsafe {
        let mut itemIndex = itemIndex;
        let mut str: *mut u8 = core::ptr::null_mut();
        if itemIndex != (-2i32) {
            str = GetItemDescription(BagGetItemIdByPocketPosition(
                (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    .wrapping_add(1i32)) as u8),
                ((itemIndex) as u16),
            ));
        } else {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((&raw mut gBagMenu_ReturnToStrings).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32)
                            as isize,
                    ))
                .read(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_ReturnToVar1).cast::<u8>(),
            );
            str = (&raw mut gStringVar4).cast::<u8>();
        }
        FillWindowPixelBuffer(1u8, 0u8);
        BagMenu_Print(1u8, 1u8, str, 3u8, 1u8, 0u8, 0u8, 0u8, 0u8);
    }
}
pub(crate) unsafe extern "C" fn BagMenu_PrintCursor(listTaskId: u8, colorIndex: u8) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut colorIndex = colorIndex;
        BagMenu_PrintCursorAtPos(
            ((ListMenuGetYCoordForPrintingArrowCursor(listTaskId)) as u8),
            colorIndex,
        );
    }
}
pub(crate) unsafe extern "C" fn BagMenu_PrintCursorAtPos(y: u8, colorIndex: u8) {
    unsafe {
        let mut y = y;
        let mut colorIndex = colorIndex;
        if ((colorIndex) as i32) == 255i32 {
            FillWindowPixelRect(
                0u8,
                0u8,
                0u16,
                ((y) as u16),
                ((GetMenuCursorDimensionByFont(1u8, 0u8)) as u16),
                ((GetMenuCursorDimensionByFont(1u8, 1u8)) as u16),
            );
        } else {
            BagMenu_Print(
                0u8,
                1u8,
                (&raw mut gText_SelectorArrow2).cast::<u8>(),
                0u8,
                y,
                0u8,
                0u8,
                0u8,
                colorIndex,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePocketScrollArrowPair() {
    unsafe {
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2078))
            .read()) as i32)
            == 255i32
        {
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2078))
                .write(AddScrollIndicatorArrowPairParameterized(
                    2u32,
                    172i32,
                    12i32,
                    148i32,
                    ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2089))
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ))
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2094))
                            .cast::<u8>())
                            .wrapping_offset(
                                (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                    as i32) as isize,
                            ))
                            .read()) as i32),
                        ),
                    110i32,
                    110i32,
                    ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ),
                ));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BagDestroyPocketScrollArrowPair() {
    unsafe {
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2078))
            .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2078))
                    .read(),
            );
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2078))
                .write(255u8);
        }
        DestroyPocketSwitchArrowPair();
    }
}
pub(crate) unsafe extern "C" fn CreatePocketSwitchArrowPair() {
    unsafe {
        if (((crate::c::bf_read(
            (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2075),
            0,
            4,
            false,
        ) as u8) as i32)
            != 1i32)
            && (((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2079))
            .read()) as i32)
                == 255i32)
        {
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2079))
                .write(AddScrollIndicatorArrowPair(
                    (&raw const sBagScrollArrowsTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    ((&raw mut gBagPosition).cast::<u8>())
                        .wrapping_add(6)
                        .cast::<u16>(),
                ));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyPocketSwitchArrowPair() {
    unsafe {
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2079))
            .read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2079))
                    .read(),
            );
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2079))
                .write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn FreeBagMenu() {
    unsafe {
        Free(((&raw mut sListBuffer2).cast::<u8>().cast::<*mut u8>()).read());
        Free(((&raw mut sListBuffer1).cast::<u8>().cast::<*mut u8>()).read());
        FreeAllWindowBuffers();
        Free(((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_FadeAndCloseBagMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CloseBagMenu));
    }
}
pub(crate) unsafe extern "C" fn Task_CloseBagMenu(taskId: u8) {
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
                ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ),
                ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ),
            );
            if core::mem::transmute::<_, usize>(
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            ) != 0usize
            {
                SetMainCallback2(
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            } else {
                SetMainCallback2(
                    (((&raw mut gBagPosition).cast::<u8>())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .read(),
                );
            }
            BagDestroyPocketScrollArrowPair();
            ResetSpriteData();
            FreeAllSpritePalettes();
            FreeBagMenu();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePocketItemList(pocketId: u8) {
    unsafe {
        let mut pocketId = pocketId;
        let mut i: u16 = 0u16;
        let mut pocket: *mut u8 =
            ((&raw mut gBagPockets).cast::<u8>()).wrapping_offset(((pocketId) as i32) as isize * 8);
        'l1: {
            let __sw1 = ((pocketId) as i32);
            let __matched = __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 2i32 || __sw1 == 3i32 {
                SortBerriesOrTMHMs(pocket);
                break 'l1;
            }
            if !__matched {
                CompactItemsInBagPocket(pocket);
                break 'l1;
            }
        }
        ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2089))
            .cast::<u8>())
        .wrapping_offset(((pocketId) as i32) as isize))
        .write(0u8);
        {
            i = 0u16;
            'l2: loop {
                if !((((i) as i32) < ((((pocket).wrapping_add(4)).read()) as i32))
                    && (((((((pocket).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read())
                        != 0))
                {
                    break 'l2;
                }
                'l3: {
                    let __p2 = (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2089))
                    .cast::<u8>())
                    .wrapping_offset(((pocketId) as i32) as isize);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                i = (i).wrapping_add(1);
            }
        }
        if !((crate::c::bf_read(
            (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2075),
            7,
            1,
            false,
        ) as u8)
            != 0)
        {
            let __p3 = (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2089))
            .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2089))
            .cast::<u8>())
        .wrapping_offset(((pocketId) as i32) as isize))
        .read()) as i32)
            > 8i32
        {
            ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2094))
                .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize))
            .write(8u8);
        } else {
            ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2094))
                .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize))
            .write(
                ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2089))
                .cast::<u8>())
                .wrapping_offset(((pocketId) as i32) as isize))
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UpdatePocketItemLists() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    UpdatePocketItemList(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePocketListPosition(pocketId: u8) {
    unsafe {
        let mut pocketId = pocketId;
        SetCursorWithinListBounds(
            ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                .wrapping_offset(((pocketId) as i32) as isize),
            ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                .wrapping_offset(((pocketId) as i32) as isize),
            ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2094))
                .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize))
            .read(),
            ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2089))
                .cast::<u8>())
            .wrapping_offset(((pocketId) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn InitPocketListPositions() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    UpdatePocketListPosition(i);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPocketScrollPositions() {
    unsafe {
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    SetCursorScrollWithinListBounds(
                        ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize),
                        ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize),
                        ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2094))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2089))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        8u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetItemListPosition(pocketId: u8) -> u8 {
    unsafe {
        let mut pocketId = pocketId;
        return (((((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
            .wrapping_offset(((pocketId) as i32) as isize))
        .read()) as i32)
            .wrapping_add(
                (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                    .wrapping_offset(((pocketId) as i32) as isize))
                .read()) as i32),
            )) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayItemMessage(
    taskId: u8,
    fontId: u8,
    str: *mut u8,
    callback: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut fontId = fontId;
        let mut str = str;
        let mut callback = callback;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(10)).write(((AddItemMessageWindow(4u8)) as i16));
        FillWindowPixelBuffer(((((data).wrapping_offset(10)).read()) as u8), 17u8);
        DisplayMessageAndContinueTask(
            taskId,
            ((((data).wrapping_offset(10)).read()) as u8),
            10u16,
            13u8,
            fontId,
            GetPlayerTextSpeedDelay(),
            str,
            core::mem::transmute::<_, *mut u8>(callback),
        );
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseItemMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        RemoveItemMessageWindow(4u8);
        DestroyListMenuTask((((data).read()) as u8), scrollPos, cursorPos);
        UpdatePocketItemList((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        UpdatePocketListPosition((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        LoadBagItemListBuffers((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (scrollPos).read(),
                (cursorPos).read(),
            )) as i16),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ReturnToItemList(taskId);
    }
}
pub(crate) unsafe extern "C" fn AddItemQuantityWindow(windowType: u8) {
    unsafe {
        let mut windowType = windowType;
        PrintItemQuantity(BagMenu_AddWindow(windowType), 1i16);
    }
}
pub(crate) unsafe extern "C" fn PrintItemQuantity(windowId: u8, quantity: i16) {
    unsafe {
        let mut windowId = windowId;
        let mut quantity = quantity;
        let mut numDigits: u8 =
            ((if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) == 3i32
            {
                3i32
            } else {
                2i32
            }) as u8);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((quantity) as i32),
            2i32,
            numDigits,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 40i32)) as u8),
            2u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintItemSoldAmount(windowId: i32, numSold: i32, moneyEarned: i32) {
    unsafe {
        let mut windowId = windowId;
        let mut numSold = numSold;
        let mut moneyEarned = moneyEarned;
        let mut numDigits: u8 =
            ((if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) == 3i32
            {
                3i32
            } else {
                2i32
            }) as u8);
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            numSold,
            2i32,
            numDigits,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((windowId) as u8),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        PrintMoneyAmount(((windowId) as u8), 38u8, 1u8, moneyEarned, 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_BagMenu_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut listPosition: i32 = 0i32;
        if (((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32)
            && (!((crate::c::bf_read(
                ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                7,
                1,
                false,
            ) as u16)
                != 0))
        {
            'l1: {
                let __sw1 = ((GetSwitchBagPocketDirection()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32;
                if __sw1 == 1i32 {
                    SwitchBagPocket(taskId, (-1i16), 0u16);
                    return;
                }
                if __sw1 == 2i32 {
                    SwitchBagPocket(taskId, 1i16, 0u16);
                    return;
                }
                if !__matched {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 4i32)
                        != 0
                    {
                        if ((CanSwapItems()) as i32) == 1i32 {
                            ListMenuGetScrollAndRow((((data).read()) as u8), scrollPos, cursorPos);
                            if (((scrollPos).read()) as i32)
                                .wrapping_add((((cursorPos).read()) as i32))
                                != ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .wrapping_add(2089))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5))
                                        .read()) as i32)
                                        as isize,
                                ))
                                .read()) as i32)
                                    .wrapping_sub(1i32)
                            {
                                PlaySE(5u16);
                                StartItemSwap(taskId);
                            }
                        }
                        return;
                    }
                    break 'l1;
                }
            }
            listPosition = ListMenu_ProcessInput((((data).read()) as u8));
            ListMenuGetScrollAndRow((((data).read()) as u8), scrollPos, cursorPos);
            'l2: {
                let __sw2 = listPosition;
                let __matched = __sw2 == (-1i32) || __sw2 == (-2i32);
                if __sw2 == (-1i32) {
                    break 'l2;
                }
                if __sw2 == (-2i32) {
                    if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        == 5i32
                    {
                        PlaySE(32u16);
                        break 'l2;
                    }
                    PlaySE(5u16);
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).write(0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_FadeAndCloseBagMenu));
                    break 'l2;
                }
                if !__matched {
                    PlaySE(5u16);
                    BagDestroyPocketScrollArrowPair();
                    BagMenu_PrintCursor((((data).read()) as u8), 2u8);
                    ((data).wrapping_offset(1)).write(((listPosition) as i16));
                    ((data).wrapping_offset(2)).write(
                        ((BagGetQuantityByPocketPosition(
                            (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32)
                                .wrapping_add(1i32)) as u8),
                            ((listPosition) as u16),
                        )) as i16),
                    );
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).write(
                        BagGetItemIdByPocketPosition(
                            (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32)
                                .wrapping_add(1i32)) as u8),
                            ((listPosition) as u16),
                        ),
                    );
                    (((((&raw const sContextMenuFuncs)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32)
                            as isize,
                    ))
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l2;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ReturnToItemList(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CreatePocketScrollArrowPair();
        CreatePocketSwitchArrowPair();
        ClearWindowTilemap(3u8);
        ClearWindowTilemap(4u8);
        PutWindowTilemap(1u8);
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_BagMenu_HandleInput));
    }
}
pub(crate) unsafe extern "C" fn GetSwitchBagPocketDirection() -> u8 {
    unsafe {
        let mut LRKeys: u8 = 0u8;
        if (crate::c::bf_read(
            (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2075),
            0,
            4,
            false,
        ) as u8)
            != 0
        {
            return 0u8;
        }
        LRKeys = GetLRKeysPressed();
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0)
            || (((LRKeys) as i32) == 1i32)
        {
            PlaySE(5u16);
            return 1u8;
        }
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0)
            || (((LRKeys) as i32) == 2i32)
        {
            PlaySE(5u16);
            return 2u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ChangeBagPocketId(bagPocketId: *mut u8, deltaBagPocketId: i8) {
    unsafe {
        let mut bagPocketId = bagPocketId;
        let mut deltaBagPocketId = deltaBagPocketId;
        if (((deltaBagPocketId) as i32) == 1i32) && ((((bagPocketId).read()) as i32) == 4i32) {
            (bagPocketId).write(0u8);
        } else {
            if (((deltaBagPocketId) as i32) == (-1i32)) && ((((bagPocketId).read()) as i32) == 0i32)
            {
                (bagPocketId).write(4u8);
            } else {
                (bagPocketId).write(
                    (((((bagPocketId).read()) as i32).wrapping_add(((deltaBagPocketId) as i32)))
                        as u8),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SwitchBagPocket(
    taskId: u8,
    deltaBagPocketId: i16,
    skipEraseList: u16,
) {
    unsafe {
        let mut taskId = taskId;
        let mut deltaBagPocketId = deltaBagPocketId;
        let mut skipEraseList = skipEraseList;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut newPocket: u8 = 0u8;
        ((data).wrapping_offset(13)).write(0i16);
        ((data).wrapping_offset(12)).write(0i16);
        ((data).wrapping_offset(11)).write(deltaBagPocketId);
        if !((skipEraseList) != 0) {
            ClearWindowTilemap(0u8);
            ClearWindowTilemap(1u8);
            DestroyListMenuTask(
                (((data).read()) as u8),
                ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ),
                ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ),
            );
            ScheduleBgCopyTilemapToVram(0u8);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2052))
                    .cast::<u8>())
                    .wrapping_offset(
                        ((2i32).wrapping_add(
                            (((crate::c::bf_read(
                                (((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2075),
                                4,
                                2,
                                false,
                            ) as u8) as i32)
                                ^ 1i32),
                        )) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            BagDestroyPocketScrollArrowPair();
        }
        newPocket = (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read();
        ChangeBagPocketId(&raw mut newPocket, ((deltaBagPocketId) as i8));
        if ((deltaBagPocketId) as i32) == 1i32 {
            PrintPocketNames(
                ((((&raw mut gPocketNamesStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ))
                .read(),
                ((((&raw mut gPocketNamesStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(((newPocket) as i32) as isize))
                .read(),
            );
            CopyPocketNameToWindow(0u32);
        } else {
            PrintPocketNames(
                ((((&raw mut gPocketNamesStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(((newPocket) as i32) as isize))
                .read(),
                ((((&raw mut gPocketNamesStringsTable).cast::<*mut u8>()).cast::<*mut u8>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    ))
                .read(),
            );
            CopyPocketNameToWindow(8u32);
        }
        DrawPocketIndicatorSquare(
            (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read(),
            0u8,
        );
        DrawPocketIndicatorSquare(newPocket, 1u8);
        FillBgTilemapBufferRect_Palette0(2u8, 11u16, 14u8, 2u8, 15u8, 16u8);
        ScheduleBgCopyTilemapToVram(2u8);
        SetBagVisualPocketId(newPocket, 1u8);
        RemoveBagSprite(1u8);
        AddSwitchPocketRotatingBallSprite(deltaBagPocketId);
        SetTaskFuncWithFollowupFunc(
            taskId,
            Some(Task_SwitchBagPocket),
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_SwitchBagPocket(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (!((MenuHelpers_IsLinkActive()) != 0)) && (!((IsWallysBag()) != 0)) {
            'l1: {
                let __sw1 = ((GetSwitchBagPocketDirection()) as i32);
                if __sw1 == 1i32 {
                    ChangeBagPocketId(
                        ((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5),
                        ((((data).wrapping_offset(11)).read()) as i8),
                    );
                    SwitchTaskToFollowupFunc(taskId);
                    SwitchBagPocket(taskId, (-1i16), 1u16);
                    return;
                }
                if __sw1 == 2i32 {
                    ChangeBagPocketId(
                        ((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5),
                        ((((data).wrapping_offset(11)).read()) as i8),
                    );
                    SwitchTaskToFollowupFunc(taskId);
                    SwitchBagPocket(taskId, 1i16, 1u16);
                    return;
                }
            }
        }
        'l2: {
            let __sw2 = ((((data).wrapping_offset(13)).read()) as i32);
            if __sw2 == 0i32 {
                DrawItemListBgRow(((((data).wrapping_offset(12)).read()) as u8));
                if !(((({
                    let __p3 = (data).wrapping_offset(12);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    & 1i32)
                    != 0)
                {
                    if ((((data).wrapping_offset(11)).read()) as i32) == 1i32 {
                        CopyPocketNameToWindow(
                            (((((((data).wrapping_offset(12)).read()) as i32) >> 1) as u8) as u32),
                        );
                    } else {
                        CopyPocketNameToWindow(
                            ((((8i32).wrapping_sub(
                                (((((data).wrapping_offset(12)).read()) as i32) >> 1),
                            )) as u8) as u32),
                        );
                    }
                }
                if ((((data).wrapping_offset(12)).read()) as i32) == 16i32 {
                    let __p5 = (data).wrapping_offset(13);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l2;
            }
            if __sw2 == 1i32 {
                ChangeBagPocketId(
                    ((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5),
                    ((((data).wrapping_offset(11)).read()) as i8),
                );
                LoadBagItemListBuffers(
                    (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read(),
                );
                (data).write(
                    ((ListMenuInit(
                        (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                    as i32) as isize,
                            ))
                        .read(),
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                            .wrapping_offset(
                                (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                    as i32) as isize,
                            ))
                        .read(),
                    )) as i16),
                );
                PutWindowTilemap(1u8);
                PutWindowTilemap(2u8);
                ScheduleBgCopyTilemapToVram(0u8);
                CreatePocketScrollArrowPair();
                CreatePocketSwitchArrowPair();
                SwitchTaskToFollowupFunc(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DrawItemListBgRow(y: u8) {
    unsafe {
        let mut y = y;
        FillBgTilemapBufferRect_Palette0(
            2u8,
            17u16,
            14u8,
            ((((y) as i32).wrapping_add(2i32)) as u8),
            15u8,
            1u8,
        );
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn DrawPocketIndicatorSquare(x: u8, isCurrentPocket: u8) {
    unsafe {
        let mut x = x;
        let mut isCurrentPocket = isCurrentPocket;
        if !((isCurrentPocket) != 0) {
            FillBgTilemapBufferRect_Palette0(
                2u8,
                4119u16,
                ((((x) as i32).wrapping_add(5i32)) as u8),
                3u8,
                1u8,
                1u8,
            );
        } else {
            FillBgTilemapBufferRect_Palette0(
                2u8,
                4139u16,
                ((((x) as i32).wrapping_add(5i32)) as u8),
                3u8,
                1u8,
                1u8,
            );
        }
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn CanSwapItems() -> u8 {
    unsafe {
        if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32) == 0i32)
            || ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32) == 1i32)
        {
            if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) != 2i32)
                && ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    != 3i32)
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartItemSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ListMenuSetTemplateField((((data).read()) as u8), 16u8, 1i32);
        ((data).wrapping_offset(1)).write(
            (((((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                .wrapping_offset(
                    (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize,
                ))
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ))
                    .read()) as i32),
                )) as i16),
        );
        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2074))
            .write(((((data).wrapping_offset(1)).read()) as u8));
        CopyItemName(
            BagGetItemIdByPocketPosition(
                (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    .wrapping_add(1i32)) as u8),
                ((((data).wrapping_offset(1)).read()) as u16),
            ),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_MoveVar1Where).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        BagMenu_Print(
            1u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            1u8,
            0u8,
            0u8,
            0u8,
            0u8,
        );
        UpdateItemMenuSwapLinePos(((((data).wrapping_offset(1)).read()) as u8));
        DestroyPocketSwitchArrowPair();
        BagMenu_PrintCursor((((data).read()) as u8), 2u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleSwappingItemsInput));
    }
}
pub(crate) unsafe extern "C" fn Task_HandleSwappingItemsInput(taskId: u8) {
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
                    ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ),
                    ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ),
                );
                DoItemSwap(taskId);
            } else {
                let mut input: i32 = ListMenu_ProcessInput((((data).read()) as u8));
                ListMenuGetScrollAndRow(
                    (((data).read()) as u8),
                    ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ),
                    ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ),
                );
                SetItemMenuSwapLineInvisibility(0u8);
                UpdateItemMenuSwapLinePos(
                    (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                        .wrapping_offset(
                            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize,
                        ))
                    .read()) as u8),
                );
                'l1: {
                    let __sw1 = input;
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
                            DoItemSwap(taskId);
                        } else {
                            CancelItemSwap(taskId);
                        }
                        break 'l1;
                    }
                    if !__matched {
                        PlaySE(5u16);
                        DoItemSwap(taskId);
                        break 'l1;
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoItemSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut realPos: u16 =
            (((((scrollPos).read()) as i32).wrapping_add((((cursorPos).read()) as i32))) as u16);
        if (((((data).wrapping_offset(1)).read()) as i32) == ((realPos) as i32))
            || (((((data).wrapping_offset(1)).read()) as i32)
                == ((realPos) as i32).wrapping_sub(1i32))
        {
            CancelItemSwap(taskId);
        } else {
            MoveItemSlotInList(
                ((((&raw mut gBagPockets).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 8,
                ))
                .cast::<*mut u8>())
                .read(),
                ((((data).wrapping_offset(1)).read()) as u32),
                ((realPos) as u32),
            );
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2074))
                .write(255u8);
            DestroyListMenuTask((((data).read()) as u8), scrollPos, cursorPos);
            if ((((data).wrapping_offset(1)).read()) as i32) < ((realPos) as i32) {
                let __p1 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                    .wrapping_offset(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize,
                    );
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            LoadBagItemListBuffers((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
            (data).write(
                ((ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    (scrollPos).read(),
                    (cursorPos).read(),
                )) as i16),
            );
            SetItemMenuSwapLineInvisibility(1u8);
            CreatePocketSwitchArrowPair();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_BagMenu_HandleInput));
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
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2074))
            .write(255u8);
        DestroyListMenuTask((((data).read()) as u8), scrollPos, cursorPos);
        if ((((data).wrapping_offset(1)).read()) as i32)
            < (((scrollPos).read()) as i32).wrapping_add((((cursorPos).read()) as i32))
        {
            let __p1 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                .wrapping_offset(
                    (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize,
                );
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        LoadBagItemListBuffers((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (scrollPos).read(),
                (cursorPos).read(),
            )) as i16),
        );
        SetItemMenuSwapLineInvisibility(1u8);
        CreatePocketSwitchArrowPair();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_BagMenu_HandleInput));
    }
}
pub(crate) unsafe extern "C" fn OpenContextMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32);
            let __matched = __sw1 == 1i32
                || __sw1 == 10i32
                || __sw1 == 5i32
                || __sw1 == 9i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 6i32;
            if __sw1 == 1i32 || __sw1 == 10i32 {
                if ((GetItemBattleUsage(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                )) as i32)
                    != 0i32
                {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_BattleUse)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                } else {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_Cancel)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(1u32, 1u32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2080)
                    .cast::<*mut u8>())
                .write(
                    ((&raw const sContextMenuItems_BerryBlenderCrush)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
                    .write(((crate::c::div_u32(4u32, 1u32)) as u8));
                break 'l1;
            }
            if __sw1 == 9i32 {
                if (!((GetItemImportance(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                )) != 0))
                    && (((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read())
                        as i32)
                        != 175i32)
                {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_Apprentice)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                } else {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_Cancel)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(1u32, 1u32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (!((GetItemImportance(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                )) != 0))
                    && (((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read())
                        as i32)
                        != 175i32)
                {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_FavorLady)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                } else {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_Cancel)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(1u32, 1u32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                if (!((GetItemImportance(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                )) != 0))
                    && (((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read())
                        as i32)
                        != 175i32)
                {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_QuizLady)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                } else {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2080)
                        .cast::<*mut u8>())
                    .write(
                        ((&raw const sContextMenuItems_Cancel)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                    );
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(2088))
                    .write(((crate::c::div_u32(1u32, 1u32)) as u8));
                }
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 6i32 || !__matched {
                if (((MenuHelpers_IsLinkActive()) as i32) == 1i32) || (InUnionRoom() == 1u32) {
                    if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        == 4i32)
                        || (!((IsHoldingItemAllowed(
                            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                        )) != 0))
                    {
                        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2080)
                            .cast::<*mut u8>())
                        .write(
                            ((&raw const sContextMenuItems_Cancel)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2088))
                        .write(((crate::c::div_u32(1u32, 1u32)) as u8));
                    } else {
                        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2080)
                            .cast::<*mut u8>())
                        .write(
                            ((&raw const sContextMenuItems_Give).cast::<u8>().cast_mut())
                                .cast::<u8>(),
                        );
                        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2088))
                        .write(((crate::c::div_u32(2u32, 1u32)) as u8));
                    }
                } else {
                    'l2: {
                        let __sw2 = (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5))
                            .read()) as i32);
                        if __sw2 == 0i32 {
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2080)
                                .cast::<*mut u8>())
                            .write(
                                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2084))
                                .cast::<u8>(),
                            );
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2088))
                            .write(((crate::c::div_u32(4u32, 1u32)) as u8));
                            crate::c::memcpy(
                                (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2084))
                                .cast::<u8>())
                                .cast::<u8>(),
                                (((&raw const sContextMenuItems_ItemsPocket)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .cast::<u8>(),
                                4u32,
                            );
                            if ((ItemIsMail(
                                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                            )) as i32)
                                == 1i32
                            {
                                (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2084))
                                .cast::<u8>())
                                .write(6u8);
                            }
                            break 'l2;
                        }
                        if __sw2 == 4i32 {
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2080)
                                .cast::<*mut u8>())
                            .write(
                                ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2084))
                                .cast::<u8>(),
                            );
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2088))
                            .write(((crate::c::div_u32(4u32, 1u32)) as u8));
                            crate::c::memcpy(
                                (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2084))
                                .cast::<u8>())
                                .cast::<u8>(),
                                (((&raw const sContextMenuItems_KeyItemsPocket)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .cast::<u8>(),
                                4u32,
                            );
                            if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(1174)
                                .cast::<u16>())
                            .read()) as i32)
                                == ((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>())
                                    .read()) as i32)
                            {
                                ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                    .wrapping_add(2084))
                                .cast::<u8>())
                                .wrapping_offset(1))
                                .write(8u8);
                            }
                            if (((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read())
                                as i32)
                                == 259i32)
                                || (((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>())
                                    .read()) as i32)
                                    == 272i32)
                            {
                                if (TestPlayerAvatarFlags(6u8)) != 0 {
                                    (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(2084))
                                    .cast::<u8>())
                                    .write(7u8);
                                }
                            }
                            break 'l2;
                        }
                        if __sw2 == 1i32 {
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2080)
                                .cast::<*mut u8>())
                            .write(
                                ((&raw const sContextMenuItems_BallsPocket)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>(),
                            );
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2088))
                            .write(((crate::c::div_u32(4u32, 1u32)) as u8));
                            break 'l2;
                        }
                        if __sw2 == 2i32 {
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2080)
                                .cast::<*mut u8>())
                            .write(
                                ((&raw const sContextMenuItems_TmHmPocket)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>(),
                            );
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2088))
                            .write(((crate::c::div_u32(4u32, 1u32)) as u8));
                            break 'l2;
                        }
                        if __sw2 == 3i32 {
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2080)
                                .cast::<*mut u8>())
                            .write(
                                ((&raw const sContextMenuItems_BerriesPocket)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>(),
                            );
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2088))
                            .write(((crate::c::div_u32(6u32, 1u32)) as u8));
                            break 'l2;
                        }
                    }
                }
            }
        }
        if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) == 2i32 {
            ClearWindowTilemap(1u8);
            PrintTMHMMoveData(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read());
            PutWindowTilemap(3u8);
            PutWindowTilemap(4u8);
            ScheduleBgCopyTilemapToVram(0u8);
        } else {
            CopyItemName(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Var1IsSelected).cast::<u8>(),
            );
            FillWindowPixelBuffer(1u8, 0u8);
            BagMenu_Print(
                1u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                3u8,
                1u8,
                0u8,
                0u8,
                0u8,
                0u8,
            );
        }
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
            .read()) as i32)
            == 1i32
        {
            PrintContextMenuItems(BagMenu_AddWindow(0u8));
        } else {
            if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
                .read()) as i32)
                == 2i32
            {
                PrintContextMenuItems(BagMenu_AddWindow(1u8));
            } else {
                if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2088))
                .read()) as i32)
                    == 4i32
                {
                    PrintContextMenuItemGrid(BagMenu_AddWindow(2u8), 2u8, 2u8);
                } else {
                    PrintContextMenuItemGrid(BagMenu_AddWindow(3u8), 2u8, 3u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintContextMenuItems(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        PrintMenuActionTexts(
            windowId,
            7u8,
            8u8,
            1u8,
            0u8,
            16u8,
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
                .read(),
            ((&raw const sItemMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2080)
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuInUpperLeftCornerNormal(
            windowId,
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
                .read(),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintContextMenuItemGrid(windowId: u8, columns: u8, rows: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut columns = columns;
        let mut rows = rows;
        PrintMenuActionGrid(
            windowId,
            7u8,
            8u8,
            1u8,
            56u8,
            columns,
            rows,
            ((&raw const sItemMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(2080)
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuActionGrid(windowId, 56u8, columns, rows, 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_Normal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        OpenContextMenu(taskId);
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
            .read()) as i32)
            <= 2i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ItemContext_SingleRow));
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ItemContext_MultipleRows));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_SingleRow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            let mut selection: i8 = Menu_ProcessInputNoWrap();
            'l1: {
                let __sw1 = ((selection) as i32);
                let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
                if __sw1 == (-2i32) {
                    break 'l1;
                }
                if __sw1 == (-1i32) {
                    PlaySE(5u16);
                    (((((((&raw const sItemMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(32))
                    .wrapping_add(4))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    (((((((&raw const sItemMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2080)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((selection) as i32) as isize))
                            .read()) as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(4))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read())
                    .unwrap_unchecked()(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_MultipleRows(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) != 1i32 {
            let mut cursorPos: i8 = ((Menu_GetCursorPos()) as i8);
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 64i32)
                != 0
            {
                if (((cursorPos) as i32) > 0i32)
                    && ((IsValidContextMenuPos(((((cursorPos) as i32).wrapping_sub(2i32)) as i8)))
                        != 0)
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
                    if (((cursorPos) as i32)
                        < ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(2088))
                        .read()) as i32)
                            .wrapping_sub(2i32))
                        && ((IsValidContextMenuPos(
                            ((((cursorPos) as i32).wrapping_add(2i32)) as i8),
                        )) != 0)
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
                        if ((((cursorPos) as i32) & 1i32) != 0)
                            && ((IsValidContextMenuPos(
                                ((((cursorPos) as i32).wrapping_sub(1i32)) as i8),
                            )) != 0)
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
                            if (!((((cursorPos) as i32) & 1i32) != 0))
                                && ((IsValidContextMenuPos(
                                    ((((cursorPos) as i32).wrapping_add(1i32)) as i8),
                                )) != 0)
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
                                (((((((&raw const sItemMenuActions).cast::<u8>().cast_mut())
                                    .cast::<u8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(2080)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((cursorPos) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 8,
                                ))
                                .wrapping_add(4))
                                .cast::<Option<unsafe extern "C" fn(u8)>>())
                                .read())
                                .unwrap_unchecked()(taskId);
                            } else {
                                if ((((((&raw mut gMain).cast::<u8>())
                                    .wrapping_add(46)
                                    .cast::<u16>())
                                .read()) as i32)
                                    & 2i32)
                                    != 0
                                {
                                    PlaySE(5u16);
                                    (((((((&raw const sItemMenuActions)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(32))
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
pub(crate) unsafe extern "C" fn IsValidContextMenuPos(cursorPos: i8) -> u8 {
    unsafe {
        let mut cursorPos = cursorPos;
        if ((cursorPos) as i32) < 0i32 {
            return 0u8;
        }
        if ((cursorPos) as i32)
            > ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
                .read()) as i32)
        {
            return 0u8;
        }
        if ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2080)
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((cursorPos) as i32) as isize))
        .read()) as i32)
            == 14i32
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn RemoveContextWindow() {
    unsafe {
        if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
            .read()) as i32)
            == 1i32
        {
            BagMenu_RemoveWindow(0u8);
        } else {
            if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2088))
                .read()) as i32)
                == 2i32
            {
                BagMenu_RemoveWindow(1u8);
            } else {
                if ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2088))
                .read()) as i32)
                    == 4i32
                {
                    BagMenu_RemoveWindow(2u8);
                } else {
                    BagMenu_RemoveWindow(3u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_UseOutOfBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (GetItemFieldFunc(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
            .is_some()
        {
            RemoveContextWindow();
            if (((CalculatePlayerPartyCount()) as i32) == 0i32)
                && (((GetItemType(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                )) as i32)
                    == 1i32)
            {
                PrintThereIsNoPokemon(taskId);
            } else {
                FillWindowPixelBuffer(1u8, 0u8);
                ScheduleBgCopyTilemapToVram(0u8);
                if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    != 3i32
                {
                    (GetItemFieldFunc(
                        ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                    ))
                    .unwrap_unchecked()(taskId);
                } else {
                    ItemUseOutOfBattle_Berry(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_Toss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RemoveContextWindow();
        ((data).wrapping_offset(8)).write(1i16);
        if ((((data).wrapping_offset(2)).read()) as i32) == 1i32 {
            AskTossItems(taskId);
        } else {
            CopyItemName(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_TossHowManyVar1s).cast::<u8>(),
            );
            FillWindowPixelBuffer(1u8, 0u8);
            BagMenu_Print(
                1u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                3u8,
                1u8,
                0u8,
                0u8,
                0u8,
                0u8,
            );
            AddItemQuantityWindow(7u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ChooseHowManyToToss));
        }
    }
}
pub(crate) unsafe extern "C" fn AskTossItems(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((data).wrapping_offset(8)).read()) as i32),
            0i32,
            3u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ConfirmTossItems).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        BagMenu_Print(
            1u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            1u8,
            0u8,
            0u8,
            0u8,
            0u8,
        );
        BagMenu_YesNo(
            taskId,
            5u8,
            (&raw const sYesNoTossFunctions).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn CancelToss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
        BagMenu_PrintCursor((((data).read()) as u8), 0u8);
        ReturnToItemList(taskId);
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
            PrintItemQuantity(
                ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2064))
                .cast::<u8>())
                .wrapping_offset(7))
                .read(),
                ((data).wrapping_offset(8)).read(),
            );
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                PlaySE(5u16);
                BagMenu_RemoveWindow(7u8);
                AskTossItems(taskId);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(5u16);
                    BagMenu_RemoveWindow(7u8);
                    CancelToss(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ConfirmToss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar2).cast::<u8>(),
            ((((data).wrapping_offset(8)).read()) as i32),
            0i32,
            3u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ThrewAwayVar2Var1s).cast::<u8>(),
        );
        FillWindowPixelBuffer(1u8, 0u8);
        BagMenu_Print(
            1u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            3u8,
            1u8,
            0u8,
            0u8,
            0u8,
            0u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_RemoveItemFromBag));
    }
}
pub(crate) unsafe extern "C" fn Task_RemoveItemFromBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            PlaySE(5u16);
            RemoveBagItem(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                ((((data).wrapping_offset(8)).read()) as u16),
            );
            DestroyListMenuTask((((data).read()) as u8), scrollPos, cursorPos);
            UpdatePocketItemList((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
            UpdatePocketListPosition(
                (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read(),
            );
            LoadBagItemListBuffers((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
            (data).write(
                ((ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    (scrollPos).read(),
                    (cursorPos).read(),
                )) as i16),
            );
            ScheduleBgCopyTilemapToVram(0u8);
            ReturnToItemList(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_Register(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1174)
            .cast::<u16>())
        .read()) as i32)
            == ((((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()) as i32)
        {
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1174)
                .cast::<u16>())
            .write(0u16);
        } else {
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1174)
                .cast::<u16>())
            .write(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read());
        }
        DestroyListMenuTask((((data).read()) as u8), scrollPos, cursorPos);
        LoadBagItemListBuffers((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (scrollPos).read(),
                (cursorPos).read(),
            )) as i16),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ItemMenu_Cancel(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_Give(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RemoveContextWindow();
        if !((IsWritingMailAllowed(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
        )) != 0)
        {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gText_CantWriteMail).cast::<u8>(),
                Some(HandleErrorMessage),
            );
        } else {
            if !((GetItemImportance(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            )) != 0)
            {
                if ((CalculatePlayerPartyCount()) as i32) == 0i32 {
                    PrintThereIsNoPokemon(taskId);
                } else {
                    ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_ChooseMonToGiveItem));
                    Task_FadeAndCloseBagMenu(taskId);
                }
            } else {
                PrintItemCantBeHeld(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrintThereIsNoPokemon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayItemMessage(
            taskId,
            1u8,
            (&raw mut gText_NoPokemon).cast::<u8>(),
            Some(HandleErrorMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn PrintItemCantBeHeld(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Var1CantBeHeld).cast::<u8>(),
        );
        DisplayItemMessage(
            taskId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(HandleErrorMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn HandleErrorMessage(taskId: u8) {
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
            CloseItemMessage(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_CheckTag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .cast::<Option<unsafe extern "C" fn()>>())
        .write(Some(DoBerryTagScreen));
        Task_FadeAndCloseBagMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_Cancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RemoveContextWindow();
        PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
        BagMenu_PrintCursor((((data).read()) as u8), 0u8);
        ReturnToItemList(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_UseInBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (GetItemBattleFunc(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
            .is_some()
        {
            RemoveContextWindow();
            (GetItemBattleFunc(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
                .unwrap_unchecked()(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReturnToBagMenuPocket() {
    unsafe {
        GoToBagMenu(12u8, 5u8, None);
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_GiveToParty(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsWritingMailAllowed(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
        )) != 0)
        {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gText_CantWriteMail).cast::<u8>(),
                Some(HandleErrorMessage),
            );
        } else {
            if !((IsHoldingItemAllowed(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            )) != 0)
            {
                CopyItemName(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_Var1CantBeHeldHere).cast::<u8>(),
                );
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(HandleErrorMessage),
                );
            } else {
                if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    != 4i32)
                    && (!((GetItemImportance(
                        ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                    )) != 0))
                {
                    Task_FadeAndCloseBagMenu(taskId);
                } else {
                    PrintItemCantBeHeld(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_GiveToPC(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((ItemIsMail(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read())) as i32)
            == 1i32
        {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gText_CantWriteMail).cast::<u8>(),
                Some(HandleErrorMessage),
            );
        } else {
            if ((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) != 4i32)
                && (!((GetItemImportance(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                )) != 0))
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_FadeAndCloseBagMenu));
            } else {
                PrintItemCantBeHeld(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UseRegisteredKeyItemOnField() -> u8 {
    unsafe {
        let mut taskId: u8 = 0u8;
        if (((InUnionRoom() == 1u32) || (((CurrentBattlePyramidLocation()) as i32) != 0i32))
            || ((InBattlePike()) != 0))
            || (((InMultiPartnerRoom()) as i32) == 1i32)
        {
            return 0u8;
        }
        HideMapNamePopUpWindow();
        ChangeBgY_ScreenOff(0u8, 0i32, 0u8);
        if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(1174)
            .cast::<u16>())
        .read()) as i32)
            != 0i32
        {
            if ((CheckBagHasItem(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1174)
                    .cast::<u16>())
                .read(),
                1u16,
            )) as i32)
                == 1i32
            {
                LockPlayerFieldControls();
                FreezeObjectEvents();
                PlayerFreeze();
                StopPlayerAvatar();
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).write(
                    ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1174)
                        .cast::<u16>())
                    .read(),
                );
                taskId = CreateTask(
                    GetItemFieldFunc(
                        ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1174)
                            .cast::<u16>())
                        .read(),
                    ),
                    8u8,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(1i16);
                return 1u8;
            } else {
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1174)
                    .cast::<u16>())
                .write(0u16);
            }
        }
        ScriptContext_SetupScript((&raw mut EventScript_SelectWithoutRegisteredItem).cast::<u8>());
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_Sell(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((GetItemPrice(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
            as i32)
            == 0i32
        {
            CopyItemName(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                (&raw mut gStringVar2).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_CantBuyKeyItem).cast::<u8>(),
            );
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(CloseItemMessage),
            );
        } else {
            ((data).wrapping_offset(8)).write(1i16);
            if ((((data).wrapping_offset(2)).read()) as i32) == 1i32 {
                DisplayCurrentMoneyWindow();
                DisplaySellItemPriceAndConfirm(taskId);
            } else {
                CopyItemName(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                    (&raw mut gStringVar2).cast::<u8>(),
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_HowManyToSell).cast::<u8>(),
                );
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(InitSellHowManyInput),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DisplaySellItemPriceAndConfirm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (crate::c::div_i32(
                ((GetItemPrice(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
                    as i32),
                2i32,
            ))
            .wrapping_mul(((((data).wrapping_offset(8)).read()) as i32)),
            0i32,
            6u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ICanPayVar1).cast::<u8>(),
        );
        DisplayItemMessage(
            taskId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(AskSellItems),
        );
    }
}
pub(crate) unsafe extern "C" fn AskSellItems(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BagMenu_YesNo(
            taskId,
            6u8,
            (&raw const sYesNoSellItemFunctions).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn CancelSell(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RemoveMoneyWindow();
        RemoveItemMessageWindow(4u8);
        BagMenu_PrintCursor((((data).read()) as u8), 0u8);
        ReturnToItemList(taskId);
    }
}
pub(crate) unsafe extern "C" fn InitSellHowManyInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut windowId: u8 = BagMenu_AddWindow(8u8);
        PrintItemSoldAmount(
            ((windowId) as i32),
            1i32,
            (crate::c::div_i32(
                ((GetItemPrice(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
                    as i32),
                2i32,
            ))
            .wrapping_mul(((((data).wrapping_offset(8)).read()) as i32)),
        );
        DisplayCurrentMoneyWindow();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ChooseHowManyToSell));
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseHowManyToSell(taskId: u8) {
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
            PrintItemSoldAmount(
                ((((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2064))
                .cast::<u8>())
                .wrapping_offset(8))
                .read()) as i32),
                ((((data).wrapping_offset(8)).read()) as i32),
                (crate::c::div_i32(
                    ((GetItemPrice(
                        ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                    )) as i32),
                    2i32,
                ))
                .wrapping_mul(((((data).wrapping_offset(8)).read()) as i32)),
            );
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                PlaySE(5u16);
                BagMenu_RemoveWindow(8u8);
                DisplaySellItemPriceAndConfirm(taskId);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(5u16);
                    BagMenu_PrintCursor((((data).read()) as u8), 0u8);
                    RemoveMoneyWindow();
                    BagMenu_RemoveWindow(8u8);
                    RemoveItemMessageWindow(4u8);
                    ReturnToItemList(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ConfirmSell(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            (crate::c::div_i32(
                ((GetItemPrice(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
                    as i32),
                2i32,
            ))
            .wrapping_mul(((((data).wrapping_offset(8)).read()) as i32)),
            0i32,
            6u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_TurnedOverVar1ForVar2).cast::<u8>(),
        );
        DisplayItemMessage(
            taskId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(SellItem),
        );
    }
}
pub(crate) unsafe extern "C" fn SellItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(
            (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize,
        );
        PlaySE(95u16);
        RemoveBagItem(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            ((((data).wrapping_offset(8)).read()) as u16),
        );
        AddMoney(
            (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(1168)
                .cast::<u32>(),
            (((crate::c::div_i32(
                ((GetItemPrice(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
                    as i32),
                2i32,
            ))
            .wrapping_mul(((((data).wrapping_offset(8)).read()) as i32))) as u32),
        );
        DestroyListMenuTask((((data).read()) as u8), scrollPos, cursorPos);
        UpdatePocketItemList((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        UpdatePocketListPosition((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        LoadBagItemListBuffers((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read());
        (data).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (scrollPos).read(),
                (cursorPos).read(),
            )) as i16),
        );
        BagMenu_PrintCursor((((data).read()) as u8), 2u8);
        PrintMoneyAmountInMoneyBox(
            ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2064))
                .cast::<u8>())
            .wrapping_offset(9))
            .read(),
            ((GetMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
            )) as i32),
            0u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(WaitAfterItemSell));
    }
}
pub(crate) unsafe extern "C" fn WaitAfterItemSell(taskId: u8) {
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
            RemoveMoneyWindow();
            CloseItemMessage(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ItemContext_Deposit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(8)).write(1i16);
        if ((((data).wrapping_offset(2)).read()) as i32) == 1i32 {
            TryDepositItem(taskId);
        } else {
            CopyItemName(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_DepositHowManyVar1).cast::<u8>(),
            );
            FillWindowPixelBuffer(1u8, 0u8);
            BagMenu_Print(
                1u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                3u8,
                1u8,
                0u8,
                0u8,
                0u8,
                0u8,
            );
            AddItemQuantityWindow(7u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ChooseHowManyToDeposit));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ChooseHowManyToDeposit(taskId: u8) {
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
            PrintItemQuantity(
                ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(2064))
                .cast::<u8>())
                .wrapping_offset(7))
                .read(),
                ((data).wrapping_offset(8)).read(),
            );
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                PlaySE(5u16);
                BagMenu_RemoveWindow(7u8);
                TryDepositItem(taskId);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    PlaySE(5u16);
                    PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
                    BagMenu_PrintCursor((((data).read()) as u8), 0u8);
                    BagMenu_RemoveWindow(7u8);
                    ReturnToItemList(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryDepositItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        FillWindowPixelBuffer(1u8, 0u8);
        if (GetItemImportance(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read()))
            != 0
        {
            BagMenu_Print(
                1u8,
                1u8,
                (&raw mut gText_CantStoreImportantItems).cast::<u8>(),
                3u8,
                1u8,
                0u8,
                0u8,
                0u8,
                0u8,
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(WaitDepositErrorMessage));
        } else {
            if ((AddPCItem(
                ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                ((((data).wrapping_offset(8)).read()) as u16),
            )) as i32)
                == 1i32
            {
                CopyItemName(
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
                    (&raw mut gStringVar1).cast::<u8>(),
                );
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar2).cast::<u8>(),
                    ((((data).wrapping_offset(8)).read()) as i32),
                    0i32,
                    3u8,
                );
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_DepositedVar2Var1s).cast::<u8>(),
                );
                BagMenu_Print(
                    1u8,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    3u8,
                    1u8,
                    0u8,
                    0u8,
                    0u8,
                    0u8,
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_RemoveItemFromBag));
            } else {
                BagMenu_Print(
                    1u8,
                    1u8,
                    (&raw mut gText_NoRoomForItems).cast::<u8>(),
                    3u8,
                    1u8,
                    0u8,
                    0u8,
                    0u8,
                    0u8,
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(WaitDepositErrorMessage));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WaitDepositErrorMessage(taskId: u8) {
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
            PrintItemDescription(((((data).wrapping_offset(1)).read()) as i32));
            BagMenu_PrintCursor((((data).read()) as u8), 0u8);
            ReturnToItemList(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn IsWallysBag() -> u8 {
    unsafe {
        if (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(4)).read()) as i32) == 10i32 {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PrepareBagForWallyTutorial() {
    unsafe {
        let mut i: u32 = 0u32;
        ((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(208u32));
        crate::c::memcpy(
            (((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1376))
                .cast::<u8>(),
            120u32,
        );
        crate::c::memcpy(
            ((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
                .cast::<u8>(),
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1616))
                .cast::<u8>(),
            64u32,
        );
        ((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(206)
            .cast::<u16>())
        .write((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).read()) as u16));
        {
            i = 0u32;
            'l1: loop {
                if !(i < 5u32) {
                    break 'l1;
                }
                'l2: {
                    ((((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(184))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    ((((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(194))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ClearItemSlots(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1376))
                .cast::<u8>(),
            30u8,
        );
        ClearItemSlots(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1616))
                .cast::<u8>(),
            16u8,
        );
        ResetBagScrollPositions();
    }
}
pub(crate) unsafe extern "C" fn RestoreBagAfterWallyTutorial() {
    unsafe {
        let mut i: u32 = 0u32;
        crate::c::memcpy(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1376))
                .cast::<u8>(),
            (((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
            120u32,
        );
        crate::c::memcpy(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1616))
                .cast::<u8>(),
            ((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(120))
                .cast::<u8>(),
            64u32,
        );
        (((&raw mut gBagPosition).cast::<u8>()).wrapping_add(5)).write(
            ((((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(206)
                .cast::<u16>())
            .read()) as u8),
        );
        {
            i = 0u32;
            'l1: loop {
                if !(i < 5u32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(184))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                    (((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(194))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(((&raw mut sTempWallyBag).cast::<u8>().cast::<*mut u8>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoWallyTutorialBagMenu() {
    unsafe {
        PrepareBagForWallyTutorial();
        AddBagItem(13u16, 1u16);
        AddBagItem(4u16, 1u16);
        GoToBagMenu(10u8, 0u8, Some(CB2_SetUpReshowBattleScreenAfterMenu2));
    }
}
pub(crate) unsafe extern "C" fn Task_WallyTutorialBagMenu(taskId: u8) {
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
            'l1: {
                let __sw1 = ((((data).wrapping_offset(8)).read()) as i32);
                let __matched = __sw1 == 102i32 || __sw1 == 204i32 || __sw1 == 306i32;
                if __sw1 == 102i32 {
                    PlaySE(5u16);
                    SwitchBagPocket(taskId, 1i16, 0u16);
                    let __p2 = (data).wrapping_offset(8);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    break 'l1;
                }
                if __sw1 == 204i32 {
                    PlaySE(5u16);
                    BagMenu_PrintCursor((((data).read()) as u8), 2u8);
                    ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).write(4u16);
                    OpenContextMenu(taskId);
                    let __p3 = (data).wrapping_offset(8);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    break 'l1;
                }
                if __sw1 == 306i32 {
                    PlaySE(5u16);
                    RemoveContextWindow();
                    DestroyListMenuTask(
                        (((data).read()) as u8),
                        core::ptr::null_mut(),
                        core::ptr::null_mut(),
                    );
                    RestoreBagAfterWallyTutorial();
                    Task_FadeAndCloseBagMenu(taskId);
                    break 'l1;
                }
                if !__matched {
                    let __p4 = (data).wrapping_offset(8);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_Show(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gSpecialVar_0x8005).cast::<u16>())
            .write(((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read());
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        RemoveContextWindow();
        Task_FadeAndCloseBagMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_ApprenticeExitBagMenu() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(Apprentice_ScriptContext_Enable));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_GiveFavorLady(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RemoveBagItem(
            ((&raw mut gSpecialVar_ItemId).cast::<u8>().cast::<u16>()).read(),
            1u16,
        );
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        RemoveContextWindow();
        Task_FadeAndCloseBagMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_FavorLadyExitBagMenu() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCallback_FavorLadyEnableScriptContexts));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn ItemMenu_ConfirmQuizLady(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        RemoveContextWindow();
        Task_FadeAndCloseBagMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_QuizLadyExitBagMenu() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(FieldCallback_QuizLadyEnableScriptContexts));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn PrintPocketNames(pocketName1: *mut u8, pocketName2: *mut u8) {
    unsafe {
        let mut pocketName1 = pocketName1;
        let mut pocketName2 = pocketName2;
        let mut window = crate::ffi::Align4([0u8; 8]);
        (&raw mut window).cast::<u8>().wrapping_add(0).write(0u8);
        let mut windowId: u16 = 0u16;
        let mut offset: i32 = 0i32;
        (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(16u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(2u8);
        windowId = AddWindow((&raw mut window).cast::<u8>());
        FillWindowPixelBuffer(((windowId) as u8), 0u8);
        offset = GetStringCenterAlignXOffset(1i32, pocketName1, 64i32);
        BagMenu_Print(
            ((windowId) as u8),
            1u8,
            pocketName1,
            ((offset) as u8),
            1u8,
            0u8,
            0u8,
            255u8,
            1u8,
        );
        if !(pocketName2).is_null() {
            offset = GetStringCenterAlignXOffset(1i32, pocketName2, 64i32);
            BagMenu_Print(
                ((windowId) as u8),
                1u8,
                pocketName2,
                (((offset).wrapping_add(64i32)) as u8),
                1u8,
                0u8,
                0u8,
                255u8,
                1u8,
            );
        }
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((GetWindowAttribute(((windowId) as u8), 7u8)) as usize as *mut u8),
                            ((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(2116))
                            .cast::<u8>(),
                            (67108864u32
                                | (crate::c::div_u32(
                                    1024u32,
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
        RemoveWindow(((windowId) as u8));
    }
}
pub(crate) unsafe extern "C" fn CopyPocketNameToWindow(a: u32) {
    unsafe {
        let mut a = a;
        let mut tileDataBuffer: *mut u8 = core::ptr::null_mut();
        let mut windowTileData: *mut u8 = core::ptr::null_mut();
        let mut b: i32 = 0i32;
        if a > 8u32 {
            a = 8u32;
        }
        tileDataBuffer = (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2116))
        .cast::<u8>())
        .cast::<u8>();
        windowTileData = ((GetWindowAttribute(2u8, 7u8)) as usize as *mut u8);
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            ((((tileDataBuffer).cast::<u8>())
                                .wrapping_offset(((a) as i32) as isize * 32))
                            .cast::<u8>())
                            .cast::<u8>(),
                            windowTileData,
                            ((67108864i32
                                | (crate::c::div_i32(256i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32)) as u32),
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
        b = (((a).wrapping_add(16u32)) as i32);
        'l5: loop {
            'l6: {
                'l7: loop {
                    'l8: {
                        CpuSet(
                            ((((tileDataBuffer).cast::<u8>()).wrapping_offset((b) as isize * 32))
                                .cast::<u8>())
                            .cast::<u8>(),
                            (windowTileData).wrapping_offset(256),
                            ((67108864i32
                                | (crate::c::div_i32(256i32, crate::c::div_i32(32i32, 8i32))
                                    & 2097151i32)) as u32),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l7;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        CopyWindowToVram(2u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn LoadBagMenuTextWindows() {
    unsafe {
        let mut i: u8 = 0u8;
        InitWindows(((&raw const sDefaultBagWindows).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 1u16, 224u8);
        LoadMessageBoxGfx(0u8, 10u16, 208u8);
        ListMenuLoadStdPalAt(192u8, 1u8);
        LoadPalette(
            (((&raw mut gStandardMenuPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            240u16,
            32u16,
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) <= 2i32) {
                    break 'l1;
                }
                'l2: {
                    FillWindowPixelBuffer(i, 0u8);
                    PutWindowTilemap(i);
                }
                i = (i).wrapping_add(1);
            }
        }
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn BagMenu_Print(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
    letterSpacing: u8,
    lineSpacing: u8,
    speed: u8,
    colorIndex: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut left = left;
        let mut top = top;
        let mut letterSpacing = letterSpacing;
        let mut lineSpacing = lineSpacing;
        let mut speed = speed;
        let mut colorIndex = colorIndex;
        AddTextPrinterParameterized4(
            windowId,
            fontId,
            left,
            top,
            letterSpacing,
            lineSpacing,
            ((((&raw const sFontColorTable).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorIndex) as i32) as isize * 3))
            .cast::<u8>(),
            ((speed) as i8),
            str,
        );
    }
}
pub(crate) unsafe extern "C" fn BagMenu_GetWindowId(windowType: u8) -> u8 {
    unsafe {
        let mut windowType = windowType;
        return ((((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(2064))
        .cast::<u8>())
        .wrapping_offset(((windowType) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn BagMenu_AddWindow(windowType: u8) -> u8 {
    unsafe {
        let mut windowType = windowType;
        let mut windowId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2064))
                .cast::<u8>())
            .wrapping_offset(((windowType) as i32) as isize);
        if (((windowId).read()) as i32) == 255i32 {
            (windowId).write(
                ((AddWindow(
                    (((&raw const sContextMenuWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((windowType) as i32) as isize * 8),
                )) as u8),
            );
            DrawStdFrameWithCustomTileAndPalette((windowId).read(), 0u8, 1u16, 14u8);
            ScheduleBgCopyTilemapToVram(1u8);
        }
        return (windowId).read();
    }
}
pub(crate) unsafe extern "C" fn BagMenu_RemoveWindow(windowType: u8) {
    unsafe {
        let mut windowType = windowType;
        let mut windowId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2064))
                .cast::<u8>())
            .wrapping_offset(((windowType) as i32) as isize);
        if (((windowId).read()) as i32) != 255i32 {
            ClearStdWindowAndFrameToTransparent((windowId).read(), 0u8);
            ClearWindowTilemap((windowId).read());
            RemoveWindow((windowId).read());
            ScheduleBgCopyTilemapToVram(1u8);
            (windowId).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn AddItemMessageWindow(windowType: u8) -> u8 {
    unsafe {
        let mut windowType = windowType;
        let mut windowId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2064))
                .cast::<u8>())
            .wrapping_offset(((windowType) as i32) as isize);
        if (((windowId).read()) as i32) == 255i32 {
            (windowId).write(
                ((AddWindow(
                    (((&raw const sContextMenuWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((windowType) as i32) as isize * 8),
                )) as u8),
            );
        }
        return (windowId).read();
    }
}
pub(crate) unsafe extern "C" fn RemoveItemMessageWindow(windowType: u8) {
    unsafe {
        let mut windowType = windowType;
        let mut windowId: *mut u8 =
            (((((&raw mut gBagMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(2064))
                .cast::<u8>())
            .wrapping_offset(((windowType) as i32) as isize);
        if (((windowId).read()) as i32) != 255i32 {
            ClearDialogWindowAndFrameToTransparent((windowId).read(), 0u8);
            ClearWindowTilemap((windowId).read());
            RemoveWindow((windowId).read());
            ScheduleBgCopyTilemapToVram(1u8);
            (windowId).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BagMenu_YesNo(taskId: u8, windowType: u8, funcTable: *mut u8) {
    unsafe {
        let mut taskId = taskId;
        let mut windowType = windowType;
        let mut funcTable = funcTable;
        CreateYesNoMenuWithCallbacks(
            taskId,
            (((&raw const sContextMenuWindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((windowType) as i32) as isize * 8),
            1u8,
            0u8,
            2u8,
            1u16,
            14u8,
            funcTable,
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayCurrentMoneyWindow() {
    unsafe {
        let mut windowId: u8 = BagMenu_AddWindow(9u8);
        PrintMoneyAmountInMoneyBoxWithBorder(
            windowId,
            1u16,
            14u8,
            ((GetMoney(
                (((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(1168)
                    .cast::<u32>(),
            )) as i32),
        );
        AddMoneyLabelObject(19u16, 11u16);
    }
}
pub(crate) unsafe extern "C" fn RemoveMoneyWindow() {
    unsafe {
        BagMenu_RemoveWindow(9u8);
        RemoveMoneyLabelObject();
    }
}
pub(crate) unsafe extern "C" fn PrepareTMHMMoveWindow() {
    unsafe {
        FillWindowPixelBuffer(3u8, 0u8);
        BlitMenuInfoIcon(3u8, 19u8, 0u16, 0u16);
        BlitMenuInfoIcon(3u8, 20u8, 0u16, 12u16);
        BlitMenuInfoIcon(3u8, 21u8, 0u16, 24u16);
        BlitMenuInfoIcon(3u8, 22u8, 0u16, 36u16);
        CopyWindowToVram(3u8, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintTMHMMoveData(itemId: u16) {
    unsafe {
        let mut itemId = itemId;
        let mut i: u8 = 0u8;
        let mut r#move: u16 = 0u16;
        let mut text: *mut u8 = core::ptr::null_mut();
        FillWindowPixelBuffer(4u8, 0u8);
        if ((itemId) as i32) == 0i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        BagMenu_Print(
                            4u8,
                            1u8,
                            (&raw mut gText_ThreeDashes).cast::<u8>(),
                            7u8,
                            ((((i) as i32).wrapping_mul(12i32)) as u8),
                            0u8,
                            0u8,
                            255u8,
                            4u8,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            CopyWindowToVram(4u8, 2u8);
        } else {
            r#move = ItemIdToBattleMoveId(itemId);
            BlitMenuInfoIcon(
                4u8,
                ((((((((&raw mut gBattleMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(2))
                .read()) as i32)
                    .wrapping_add(1i32)) as u8),
                0u16,
                0u16,
            );
            if ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(1))
            .read()) as i32)
                <= 1i32
            {
                text = (&raw mut gText_ThreeDashes).cast::<u8>();
            } else {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 12))
                    .wrapping_add(1))
                    .read()) as i32),
                    1i32,
                    3u8,
                );
                text = (&raw mut gStringVar1).cast::<u8>();
            }
            BagMenu_Print(4u8, 1u8, text, 7u8, 12u8, 0u8, 0u8, 255u8, 4u8);
            if ((((((&raw mut gBattleMoves).cast::<u8>())
                .wrapping_offset(((r#move) as i32) as isize * 12))
            .wrapping_add(3))
            .read()) as i32)
                == 0i32
            {
                text = (&raw mut gText_ThreeDashes).cast::<u8>();
            } else {
                ConvertIntToDecimalStringN(
                    (&raw mut gStringVar1).cast::<u8>(),
                    ((((((&raw mut gBattleMoves).cast::<u8>())
                        .wrapping_offset(((r#move) as i32) as isize * 12))
                    .wrapping_add(3))
                    .read()) as i32),
                    1i32,
                    3u8,
                );
                text = (&raw mut gStringVar1).cast::<u8>();
            }
            BagMenu_Print(4u8, 1u8, text, 7u8, 24u8, 0u8, 0u8, 255u8, 4u8);
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((((((&raw mut gBattleMoves).cast::<u8>())
                    .wrapping_offset(((r#move) as i32) as isize * 12))
                .wrapping_add(4))
                .read()) as i32),
                1i32,
                3u8,
            );
            BagMenu_Print(
                4u8,
                1u8,
                (&raw mut gStringVar1).cast::<u8>(),
                7u8,
                36u8,
                0u8,
                0u8,
                255u8,
                4u8,
            );
            CopyWindowToVram(4u8, 2u8);
        }
    }
}
