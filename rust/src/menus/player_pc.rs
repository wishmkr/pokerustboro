//! Translated from `src/player_pc.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sItemStorage_OptionDescriptions sPlayerPCMenuActions sBedroomPC_OptionOrder sPlayerPC_OptionOrder sItemStorage_MenuActions sNewGamePCItems gMailboxMailOptions sWindowTemplates_MainMenus ItemTossYesNoFuncs sListMenuTemplate_ItemStorage sWindowTemplates_ItemStorage sSwapArrowTextColors
#[allow(unused_imports)]
use crate::data::player_pc::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTopMenuOptionOrder: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTopMenuNumOptions: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPCItemPageInfo: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemStorageMenu: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut LittlerootTown_BrendansHouse_2F_EventScript_TurnOffPlayerPC: u8;
    static mut LittlerootTown_MaysHouse_2F_EventScript_TurnOffPlayerPC: u8;
    static mut gFieldCallback: u8;
    static mut gMain: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BagIsFull: u8;
    static mut gText_Cancel2: u8;
    static mut gText_ConfirmTossItems: u8;
    static mut gText_GoBackPrevMenu: u8;
    static mut gText_MailToBagMessageErased: u8;
    static mut gText_Mailbox: u8;
    static mut gText_MessageWillBeLost: u8;
    static mut gText_MoveVar1Where: u8;
    static mut gText_NoItems: u8;
    static mut gText_NoMailHere: u8;
    static mut gText_NoPokemon: u8;
    static mut gText_NoRoomInBag: u8;
    static mut gText_SelectorArrow2: u8;
    static mut gText_ThrewAwayVar2Var1s: u8;
    static mut gText_TooImportantToToss: u8;
    static mut gText_TossHowManyVar1s: u8;
    static mut gText_TossItem: u8;
    static mut gText_WhatToDoWithVar1sMail: u8;
    static mut gText_WhatWouldYouLike: u8;
    static mut gText_WithdrawHowManyItems: u8;
    static mut gText_WithdrawItem: u8;
    static mut gText_WithdrawXItems: u8;
    static mut gText_xVar1: u8;
    fn AddBagItem(a0: u16, a1: u16) -> u8;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn AddPCItem(a0: u16, a1: u16) -> u8;
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
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CB2_GoToItemDepositMenu();
    fn CB2_ReturnToField();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChooseMonToGiveMailFromMailbox();
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearItemSlots(a0: *mut u8, a1: u8);
    fn ClearMail(a0: *mut u8);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn CompactPCItems();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalPlayerNameStripChar(a0: *mut u8, a1: u8);
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountUsedPCItemSlots() -> u8;
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
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySprite(a0: *mut u8);
    fn DestroySwapLineSprites(a0: *mut u8, a1: u8);
    fn DestroyTask(a0: u8);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn DisplayYesNoMenuDefaultYes();
    fn DoPlayerRoomDecorationMenu(a0: u8);
    fn DrawDialogueFrame(a0: u8, a1: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn Free(a0: *mut u8);
    fn FreeAndReserveObjectSpritePalettes();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemImportance(a0: u16) -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut u8, a1: i32) -> i32;
    fn GetMaxWidthInSubsetOfMenuTable(a0: *mut u8, a1: *mut u8, a2: i32) -> i32;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuGetYCoordForPrintingArrowCursor(a0: u8) -> u16;
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
    fn ListMenuSetTemplateField(a0: u8, a1: u8, a2: i32);
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadListMenuSwapLineGfx();
    fn LoadMessageBoxAndBorderGfx();
    fn MailboxMenu_AddScrollArrows(a0: *mut u8);
    fn MailboxMenu_AddWindow(a0: u8) -> u8;
    fn MailboxMenu_Alloc(a0: u8) -> u8;
    fn MailboxMenu_CreateList(a0: *mut u8) -> u8;
    fn MailboxMenu_Free();
    fn MailboxMenu_RemoveWindow(a0: u8);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MoveItemSlotInList(a0: *mut u8, a1: u32, a2: u32);
    fn PlaySE(a0: u16);
    fn PrintMenuActionTextsInUpperLeftCorner(a0: u8, a1: u8, a2: *mut u8, a3: *mut u8);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
    fn ProcessMenuInput_other() -> i8;
    fn ReadMail(a0: *mut u8, a1: Option<unsafe extern "C" fn()>, a2: u8);
    fn RemovePCItem(a0: u8, a1: u16);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetCursorWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8);
    fn SetItemListPerPageCount(a0: *mut u8, a1: u8, a2: *mut u8, a3: *mut u8, a4: u8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetSwapLineSpritesInvisibility(a0: *mut u8, a1: u8, a2: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TaskDummy(a0: u8);
    fn UpdateSwapLineSpritesPos(a0: *mut u8, a1: u8, a2: i16, a3: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn NewGameInitPCItems() {
    unsafe {
        let mut i: u8 = 0u8;
        ClearItemSlots(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>(),
            50u8,
        );
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((((((((&raw const sNewGamePCItems).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((i) as i32) as isize * 4))
            .cast::<u16>())
            .read()) as i32)
                == 0i32)
                || (((((((((&raw const sNewGamePCItems).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32)
            {
                break 'l1;
            }
            if ((AddPCItem(
                (((((&raw const sNewGamePCItems).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
                ((((((&raw const sNewGamePCItems).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                .cast::<u16>())
                .wrapping_offset(1))
                .read(),
            )) as i32)
                != 1i32
            {
                break 'l1;
            }
            i = (i).wrapping_add(1);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BedroomPC() {
    unsafe {
        ((&raw mut sTopMenuOptionOrder)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(((&raw const sBedroomPC_OptionOrder).cast::<u8>().cast_mut()).cast::<u8>());
        ((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>())
            .write(((crate::c::div_u32(4u32, 1u32)) as u8));
        DisplayItemMessageOnField(
            CreateTask(Some(TaskDummy), 0u8),
            (&raw mut gText_WhatWouldYouLike).cast::<u8>(),
            Some(InitPlayerPCMenu),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerPC() {
    unsafe {
        ((&raw mut sTopMenuOptionOrder)
            .cast::<u8>()
            .cast::<*mut u8>())
        .write(((&raw const sPlayerPC_OptionOrder).cast::<u8>().cast_mut()).cast::<u8>());
        ((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>())
            .write(((crate::c::div_u32(3u32, 1u32)) as u8));
        DisplayItemMessageOnField(
            CreateTask(Some(TaskDummy), 0u8),
            (&raw mut gText_WhatWouldYouLike).cast::<u8>(),
            Some(InitPlayerPCMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn InitPlayerPCMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u16 = core::ptr::null_mut();
        let mut windowTemplate = crate::ffi::Align4([0u8; 8]);
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        if ((((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>()).read()) as u32)
            == crate::c::div_u32(3u32, 1u32)
        {
            (&raw mut windowTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    ((&raw const sWindowTemplates_MainMenus)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
                );
        } else {
            (&raw mut windowTemplate)
                .cast::<u8>()
                .cast::<crate::c::Rec4<8>>()
                .write_unaligned(
                    (((&raw const sWindowTemplates_MainMenus)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(8)
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
                );
        }
        (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(3)).write(
            ((GetMaxWidthInSubsetOfMenuTable(
                ((&raw const sPlayerPCMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
                ((&raw mut sTopMenuOptionOrder)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .read(),
                ((((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>()).read()) as i32),
            )) as u8),
        );
        ((data).wrapping_offset(4)).write(AddWindow((&raw mut windowTemplate).cast::<u8>()));
        SetStandardWindowBorderStyle(((((data).wrapping_offset(4)).read()) as u8), 0u8);
        PrintMenuActionTextsInUpperLeftCorner(
            ((((data).wrapping_offset(4)).read()) as u8),
            ((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>()).read(),
            ((&raw const sPlayerPCMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
            ((&raw mut sTopMenuOptionOrder)
                .cast::<u8>()
                .cast::<*mut u8>())
            .read(),
        );
        InitMenuInUpperLeftCornerNormal(
            ((((data).wrapping_offset(4)).read()) as u8),
            ((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(PlayerPCProcessMenuInput));
    }
}
pub(crate) unsafe extern "C" fn PlayerPCProcessMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u16 = core::ptr::null_mut();
        let mut inputOptionId: i8 = 0i8;
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        if ((((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>()).read()) as i32) > 3i32 {
            inputOptionId = Menu_ProcessInput();
        } else {
            inputOptionId = Menu_ProcessInputNoWrap();
        }
        'l1: {
            let __sw1 = ((inputOptionId) as i32);
            let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
            if __sw1 == (-2i32) {
                break 'l1;
            }
            if __sw1 == (-1i32) {
                PlaySE(5u16);
                ClearStdWindowAndFrameToTransparent(
                    ((((data).wrapping_offset(4)).read()) as u8),
                    0u8,
                );
                ClearWindowTilemap(((((data).wrapping_offset(4)).read()) as u8));
                RemoveWindow(((((data).wrapping_offset(4)).read()) as u8));
                ScheduleBgCopyTilemapToVram(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(PlayerPC_TurnOff));
                break 'l1;
            }
            if !__matched {
                ClearStdWindowAndFrameToTransparent(
                    ((((data).wrapping_offset(4)).read()) as u8),
                    0u8,
                );
                ClearWindowTilemap(((((data).wrapping_offset(4)).read()) as u8));
                RemoveWindow(((((data).wrapping_offset(4)).read()) as u8));
                ScheduleBgCopyTilemapToVram(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(
                    ((((((&raw const sPlayerPCMenuActions).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sTopMenuOptionOrder)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((inputOptionId) as i32) as isize))
                        .read()) as i32) as isize
                            * 8,
                    ))
                    .wrapping_add(4))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .read(),
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReshowPlayerPC(var: u8) {
    unsafe {
        let mut var = var;
        DisplayItemMessageOnField(
            var,
            (&raw mut gText_WhatWouldYouLike).cast::<u8>(),
            Some(InitPlayerPCMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn PlayerPC_ItemStorage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        InitItemStorageMenu(taskId, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorageMenuProcessInput));
    }
}
pub(crate) unsafe extern "C" fn PlayerPC_Mailbox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5))
            .write(GetMailboxMailCount());
        if (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read()) as i32)
            == 0i32
        {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_NoMailHere).cast::<u8>(),
                Some(ReshowPlayerPC),
            );
        } else {
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).write(0u16);
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .write(0u16);
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).write(255u8);
            Mailbox_CompactMailList();
            SetPlayerPCListCount(taskId);
            if ((MailboxMenu_Alloc(
                (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read(),
            )) as i32)
                == 1i32
            {
                ClearDialogWindowAndFrame(0u8, 0u8);
                Mailbox_DrawMailboxMenu(taskId);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Mailbox_ProcessInput));
            } else {
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gText_NoMailHere).cast::<u8>(),
                    Some(ReshowPlayerPC),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerPC_Decoration(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DoPlayerRoomDecorationMenu(taskId);
    }
}
pub(crate) unsafe extern "C" fn PlayerPC_TurnOff(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((&raw mut sTopMenuNumOptions).cast::<u8>().cast::<u8>()).read()) as u32)
            == crate::c::div_u32(4u32, 1u32)
        {
            if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
                == 0i32
            {
                ScriptContext_SetupScript(
                    (&raw mut LittlerootTown_BrendansHouse_2F_EventScript_TurnOffPlayerPC)
                        .cast::<u8>(),
                );
            } else {
                ScriptContext_SetupScript(
                    (&raw mut LittlerootTown_MaysHouse_2F_EventScript_TurnOffPlayerPC).cast::<u8>(),
                );
            }
        } else {
            ScriptContext_Enable();
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn InitItemStorageMenu(taskId: u8, var: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var = var;
        let mut data: *mut u16 = core::ptr::null_mut();
        let mut windowTemplate = crate::ffi::Align4([0u8; 8]);
        data = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        (&raw mut windowTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw const sWindowTemplates_MainMenus)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(16)
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
            );
        (((&raw mut windowTemplate).cast::<u8>()).wrapping_add(3)).write(
            ((GetMaxWidthInMenuTable(
                ((&raw const sItemStorage_MenuActions)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
                ((crate::c::div_u32(32u32, 8u32)) as i32),
            )) as u8),
        );
        ((data).wrapping_offset(4)).write(AddWindow((&raw mut windowTemplate).cast::<u8>()));
        SetStandardWindowBorderStyle(((((data).wrapping_offset(4)).read()) as u8), 0u8);
        PrintMenuTable(
            ((((data).wrapping_offset(4)).read()) as u8),
            ((crate::c::div_u32(32u32, 8u32)) as u8),
            ((&raw const sItemStorage_MenuActions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(
            ((((data).wrapping_offset(4)).read()) as u8),
            ((crate::c::div_u32(32u32, 8u32)) as u8),
            var,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ItemStorageMenuPrint(
            ((((&raw const sItemStorage_OptionDescriptions)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((var) as i32) as isize))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorageMenuPrint(textPtr: *mut u8) {
    unsafe {
        let mut textPtr = textPtr;
        DrawDialogueFrame(0u8, 0u8);
        AddTextPrinterParameterized(0u8, 1u8, textPtr, 0u8, 1u8, 0u8, None);
    }
}
pub(crate) unsafe extern "C" fn ItemStorageMenuProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut oldPos: i8 = 0i8;
        let mut newPos: i8 = 0i8;
        let mut inputOptionId: i8 = 0i8;
        oldPos = ((Menu_GetCursorPos()) as i8);
        inputOptionId = Menu_ProcessInput();
        newPos = ((Menu_GetCursorPos()) as i8);
        'l1: {
            let __sw1 = ((inputOptionId) as i32);
            let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
            if __sw1 == (-2i32) {
                if ((oldPos) as i32) != ((newPos) as i32) {
                    ItemStorageMenuPrint(
                        ((((&raw const sItemStorage_OptionDescriptions)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((newPos) as i32) as isize))
                        .read(),
                    );
                }
                break 'l1;
            }
            if __sw1 == (-1i32) {
                PlaySE(5u16);
                ItemStorage_Exit(taskId);
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                (((((((&raw const sItemStorage_MenuActions)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((inputOptionId) as i32) as isize * 8))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Deposit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ItemStorage_Deposit));
        FadeScreen(1u8, 0i8);
    }
}
pub(crate) unsafe extern "C" fn Task_ItemStorage_Deposit(taskId: u8) {
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
            CB2_GoToItemDepositMenu();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_PlayerPCExitBagMenu() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(ItemStorage_ReshowAfterBagMenu));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ReshowAfterBagMenu() {
    unsafe {
        LoadMessageBoxAndBorderGfx();
        DrawDialogueFrame(0u8, 1u8);
        InitItemStorageMenu(
            CreateTask(Some(ItemStorage_HandleReturnToProcessInput), 0u8),
            1u8,
        );
        FadeInFromBlack();
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleReturnToProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemStorageMenuProcessInput));
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Withdraw(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(1)).write(((CountUsedPCItemSlots()) as i16));
        if ((((data).wrapping_offset(1)).read()) as i32) != 0i32 {
            ItemStorage_Enter(taskId, 0u8);
        } else {
            ItemStorage_EraseMainMenu(taskId);
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_NoItems).cast::<u8>(),
                Some(PlayerPC_ItemStorage),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Toss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(1)).write(((CountUsedPCItemSlots()) as i16));
        if ((((data).wrapping_offset(1)).read()) as i32) != 0i32 {
            ItemStorage_Enter(taskId, 1u8);
        } else {
            ItemStorage_EraseMainMenu(taskId);
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_NoItems).cast::<u8>(),
                Some(PlayerPC_ItemStorage),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Enter(taskId: u8, toss: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut toss = toss;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        ((data).wrapping_offset(3)).write(((toss) as u16));
        ItemStorage_EraseMainMenu(taskId);
        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).write(0u16);
        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).write(255u8);
        SetPlayerPCListCount(taskId);
        ItemStorage_Init();
        FreeAndReserveObjectSpritePalettes();
        LoadListMenuSwapLineGfx();
        CreateSwapLineSprites(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1640))
            .cast::<u8>(),
            7u8,
        );
        ClearDialogWindowAndFrame(0u8, 0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_CreateListMenu));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Exit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ItemStorage_EraseMainMenu(taskId);
        ReshowPlayerPC(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetPlayerPCListCount(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read()) as i32)
            > 7i32
        {
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4)).write(8u8);
        } else {
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4)).write(
                (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read())
                    as i32)
                    .wrapping_add(1i32)) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_EraseMainMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        ClearStdWindowAndFrameToTransparent(((((data).wrapping_offset(4)).read()) as u8), 0u8);
        ClearWindowTilemap(((((data).wrapping_offset(4)).read()) as u8));
        RemoveWindow(((((data).wrapping_offset(4)).read()) as u8));
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn GetMailboxMailCount() -> u8 {
    unsafe {
        let mut mailInPC: u8 = 0u8;
        let mut i: u8 = 0u8;
        {
            mailInPC = 0u8;
            i = 6u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(11232))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 36))
                    .wrapping_add(32)
                    .cast::<u16>())
                    .read()) as i32)
                        != 0i32
                    {
                        mailInPC = (mailInPC).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return mailInPC;
    }
}
pub(crate) unsafe extern "C" fn Mailbox_CompactMailList() {
    unsafe {
        let mut temp = crate::ffi::Align4([0u8; 36]);
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 6u8;
            'l1: loop {
                if !(((i) as i32) < 15i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u8);
                        'l3: loop {
                            if !(((j) as i32) < 16i32) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(11232))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 36))
                                .wrapping_add(32)
                                .cast::<u16>())
                                .read()) as i32)
                                    == 0i32
                                {
                                    (&raw mut temp)
                                        .cast::<u8>()
                                        .cast::<crate::c::Rec4<36>>()
                                        .write_unaligned(
                                            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(11232))
                                            .cast::<u8>())
                                            .wrapping_offset(((i) as i32) as isize * 36)
                                            .cast::<crate::c::Rec4<36>>()
                                            .read_unaligned(),
                                        );
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(11232))
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 36)
                                    .cast::<crate::c::Rec4<36>>()
                                    .write_unaligned(
                                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                            .wrapping_add(11232))
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 36)
                                        .cast::<crate::c::Rec4<36>>()
                                        .read_unaligned(),
                                    );
                                    (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                        .wrapping_add(11232))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 36)
                                    .cast::<crate::c::Rec4<36>>()
                                    .write_unaligned(
                                        (&raw mut temp)
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<36>>()
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
pub(crate) unsafe extern "C" fn Mailbox_DrawMailboxMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut windowId: u8 = MailboxMenu_AddWindow(0u8);
        MailboxMenu_AddWindow(1u8);
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_Mailbox).cast::<u8>(),
            ((GetStringCenterAlignXOffset(1i32, (&raw mut gText_Mailbox).cast::<u8>(), 64i32))
                as u8),
            1u8,
            0u8,
            None,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((MailboxMenu_CreateList((&raw mut gPlayerPCItemPageInfo).cast::<u8>())) as i16));
        MailboxMenu_AddScrollArrows((&raw mut gPlayerPCItemPageInfo).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn Mailbox_ProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut u16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .cast::<u16>();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut inputOptionId: i32 =
                ListMenu_ProcessInput(((((data).wrapping_offset(5)).read()) as u8));
            ListMenuGetScrollAndRow(
                ((((data).wrapping_offset(5)).read()) as u8),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>(),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
            );
            'l1: {
                let __sw1 = inputOptionId;
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    PlaySE(5u16);
                    RemoveScrollIndicatorArrowPair(
                        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).read(),
                    );
                    Mailbox_ReturnToPlayerPC(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    MailboxMenu_RemoveWindow(0u8);
                    MailboxMenu_RemoveWindow(1u8);
                    DestroyListMenuTask(
                        ((((data).wrapping_offset(5)).read()) as u8),
                        ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>(),
                        ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
                    );
                    ScheduleBgCopyTilemapToVram(0u8);
                    RemoveScrollIndicatorArrowPair(
                        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).read(),
                    );
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Mailbox_PrintWhatToDoWithPlayerMailText));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_PrintWhatToDoWithPlayerMailText(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                .cast::<u8>())
            .wrapping_offset(
                (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(6i32))
                .wrapping_add(
                    (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read())
                        as i32),
                )) as isize
                    * 36,
            ))
            .wrapping_add(18))
            .cast::<u8>(),
        );
        ConvertInternationalPlayerNameStripChar((&raw mut gStringVar1).cast::<u8>(), 0u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_WhatToDoWithVar1sMail).cast::<u8>(),
        );
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(Mailbox_PrintMailOptions),
        );
    }
}
pub(crate) unsafe extern "C" fn Mailbox_ReturnToPlayerPC(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        MailboxMenu_RemoveWindow(0u8);
        MailboxMenu_RemoveWindow(1u8);
        DestroyListMenuTask(
            ((((data).wrapping_offset(5)).read()) as u8),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        MailboxMenu_Free();
        ReshowPlayerPC(taskId);
    }
}
pub(crate) unsafe extern "C" fn Mailbox_PrintMailOptions(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut windowId: u8 = MailboxMenu_AddWindow(2u8);
        PrintMenuTable(
            windowId,
            ((crate::c::div_u32(32u32, 8u32)) as u8),
            ((&raw const gMailboxMailOptions).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(windowId, ((crate::c::div_u32(32u32, 8u32)) as u8), 0u8);
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Mailbox_MailOptionsProcessInput));
    }
}
pub(crate) unsafe extern "C" fn Mailbox_MailOptionsProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut inputOptionId: i8 = ProcessMenuInput_other();
        'l1: {
            let __sw1 = ((inputOptionId) as i32);
            let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
            if __sw1 == (-2i32) {
                break 'l1;
            }
            if __sw1 == (-1i32) {
                PlaySE(5u16);
                Mailbox_Cancel(taskId);
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                (((((((&raw const gMailboxMailOptions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((inputOptionId) as i32) as isize * 8))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_DoMailRead(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        FadeScreen(1u8, 0i8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Mailbox_FadeAndReadMail));
    }
}
pub(crate) unsafe extern "C" fn Mailbox_FadeAndReadMail(taskId: u8) {
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
            MailboxMenu_Free();
            CleanupOverworldWindowsAndTilemaps();
            ReadMail(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(11232))
                    .cast::<u8>())
                .wrapping_offset(
                    (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(6i32))
                    .wrapping_add(
                        (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read())
                            as i32),
                    )) as isize
                        * 36,
                ),
                Some(Mailbox_ReturnToFieldFromReadMail),
                1u8,
            );
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_ReturnToFieldFromReadMail() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(Mailbox_ReshowAfterMail));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn Mailbox_ReshowAfterMail() {
    unsafe {
        let mut taskId: u8 = 0u8;
        LoadMessageBoxAndBorderGfx();
        taskId = CreateTask(Some(Mailbox_HandleReturnToProcessInput), 0u8);
        if ((MailboxMenu_Alloc(
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read(),
        )) as i32)
            == 1i32
        {
            Mailbox_DrawMailboxMenu(taskId);
        } else {
            DestroyTask(taskId);
        }
        FadeInFromBlack();
    }
}
pub(crate) unsafe extern "C" fn Mailbox_HandleReturnToProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Mailbox_ProcessInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_MoveToBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gText_MessageWillBeLost).cast::<u8>(),
            Some(Mailbox_AskConfirmMoveToBag),
        );
    }
}
pub(crate) unsafe extern "C" fn Mailbox_AskConfirmMoveToBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Mailbox_HandleConfirmMoveToBag));
    }
}
pub(crate) unsafe extern "C" fn Mailbox_HandleConfirmMoveToBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == (-1i32) || __sw1 == 1i32 || __sw1 == (-2i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                Mailbox_DoMailMoveToBag(taskId);
                break 'l1;
            }
            if __sw1 == (-1i32) {
                __fall = true;
                PlaySE(5u16);
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                Mailbox_CancelMoveToBag(taskId);
                break 'l1;
            }
            if __sw1 == (-2i32) || !__matched {
                __fall = true;
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_DoMailMoveToBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut mail: *mut u8 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(11232))
        .cast::<u8>())
        .wrapping_offset(
            (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(6i32))
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read()) as i32),
            )) as isize
                * 36,
        );
        if !((AddBagItem(((mail).wrapping_add(32).cast::<u16>()).read(), 1u16)) != 0) {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_BagIsFull).cast::<u8>(),
                Some(Mailbox_Cancel),
            );
        } else {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_MailToBagMessageErased).cast::<u8>(),
                Some(Mailbox_Cancel),
            );
            ClearMail(mail);
            Mailbox_CompactMailList();
            let __p1 = ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if ((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read()) as i32)
                < (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4)).read())
                    as i32)
                    .wrapping_add(
                        (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32),
                    ))
                && ((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    != 0i32)
            {
                let __p2 = ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_sub(1));
            }
            SetPlayerPCListCount(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_CancelMoveToBag(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        Mailbox_Cancel(taskId);
    }
}
pub(crate) unsafe extern "C" fn Mailbox_Give(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((CalculatePlayerPartyCount()) as i32) == 0i32 {
            Mailbox_NoPokemonForMail(taskId);
        } else {
            FadeScreen(1u8, 0i8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Mailbox_DoGiveMailPokeMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_DoGiveMailPokeMenu(taskId: u8) {
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
            MailboxMenu_Free();
            CleanupOverworldWindowsAndTilemaps();
            ChooseMonToGiveMailFromMailbox();
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Mailbox_ReturnToMailListAfterDeposit() {
    unsafe {
        ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(Mailbox_UpdateMailListAfterDeposit));
        SetMainCallback2(Some(CB2_ReturnToField));
    }
}
pub(crate) unsafe extern "C" fn Mailbox_UpdateMailListAfterDeposit() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut prevCount: u8 = 0u8;
        taskId = CreateTask(Some(Mailbox_HandleReturnToProcessInput), 0u8);
        prevCount = (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read();
        (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5))
            .write(GetMailboxMailCount());
        Mailbox_CompactMailList();
        if ((((prevCount) as i32)
            != (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read()) as i32))
            && ((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read())
                as i32)
                < (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4)).read())
                    as i32)
                    .wrapping_add(
                        (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32),
                    )))
            && ((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                != 0i32)
        {
            let __p1 = ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        SetPlayerPCListCount(taskId);
        LoadMessageBoxAndBorderGfx();
        if ((MailboxMenu_Alloc(
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read(),
        )) as i32)
            == 1i32
        {
            Mailbox_DrawMailboxMenu(taskId);
        } else {
            DestroyTask(taskId);
        }
        FadeInFromBlack();
    }
}
pub(crate) unsafe extern "C" fn Mailbox_NoPokemonForMail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gText_NoPokemon).cast::<u8>(),
            Some(Mailbox_Cancel),
        );
    }
}
pub(crate) unsafe extern "C" fn Mailbox_Cancel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        MailboxMenu_RemoveWindow(2u8);
        ClearDialogWindowAndFrame(0u8, 0u8);
        Mailbox_DrawMailboxMenu(taskId);
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Mailbox_ProcessInput));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Init() {
    unsafe {
        ((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(1648u32));
        crate::c::memset(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1632))
            .cast::<u8>(),
            255i32,
            6u32,
        );
        ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1638))
            .write(255u8);
        ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1639))
            .write(255u8);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Free() {
    unsafe {
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 6u32) {
                    break 'l1;
                }
                'l2: {
                    ItemStorage_RemoveWindow(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        Free(((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_AddWindow(i: u8) -> u8 {
    unsafe {
        let mut i = i;
        let mut windowIdLoc: *mut u8 =
            (((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1632))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize);
        if (((windowIdLoc).read()) as i32) == 255i32 {
            (windowIdLoc).write(
                ((AddWindow(
                    (((&raw const sWindowTemplates_ItemStorage)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8),
                )) as u8),
            );
            DrawStdFrameWithCustomTileAndPalette((windowIdLoc).read(), 0u8, 532u16, 14u8);
            ScheduleBgCopyTilemapToVram(0u8);
        }
        return (windowIdLoc).read();
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_RemoveWindow(i: u8) {
    unsafe {
        let mut i = i;
        let mut windowIdLoc: *mut u8 =
            (((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1632))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize);
        if (((windowIdLoc).read()) as i32) != 255i32 {
            ClearStdWindowAndFrameToTransparent((windowIdLoc).read(), 0u8);
            ClearWindowTilemap((windowIdLoc).read());
            ScheduleBgCopyTilemapToVram(0u8);
            RemoveWindow((windowIdLoc).read());
            (windowIdLoc).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemStorage_RefreshListMenu() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read())
                        as i32)
                        .wrapping_sub(1i32))
                {
                    break 'l1;
                }
                'l2: {
                    CopyItemName_PlayerPC(
                        ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(408))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u8>(),
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1176))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 4))
                        .cast::<u16>())
                        .read(),
                    );
                    ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<*mut u8>())
                    .write(
                        ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(408))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 24))
                        .cast::<u8>(),
                    );
                    ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
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
            ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(408))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 24))
            .cast::<u8>(),
            (&raw mut gText_Cancel2).cast::<u8>(),
        );
        ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write(
            ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(408))
            .cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 24))
            .cast::<u8>(),
        );
        ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
            .wrapping_offset(((i) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sListMenuTemplate_ItemStorage)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
            .write(ItemStorage_AddWindow(0u8));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read()) as u16));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>()).write(
            (((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>(),
        );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4)).read()) as u16));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemName_PlayerPC(string: *mut u8, itemId: u16) {
    unsafe {
        let mut string = string;
        let mut itemId = itemId;
        CopyItemName(itemId, string);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_MoveCursor(id: i32, onInit: u8, list: *mut u8) {
    unsafe {
        let mut id = id;
        let mut onInit = onInit;
        let mut list = list;
        if ((onInit) as i32) != 1i32 {
            PlaySE(5u16);
        }
        if ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1638))
        .read()) as i32)
            == 255i32
        {
            ItemStorage_EraseItemIcon();
            if id != (-2i32) {
                ItemStorage_DrawItemIcon(
                    (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(1176))
                    .cast::<u8>())
                    .wrapping_offset((id) as isize * 4))
                    .cast::<u16>())
                    .read(),
                );
            } else {
                ItemStorage_DrawItemIcon(65535u16);
            }
            ItemStorage_PrintDescription(id);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_PrintMenuItem(windowId: u8, id: u32, yOffset: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut id = id;
        let mut yOffset = yOffset;
        if id != 4294967294u32 {
            if ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1638))
            .read()) as i32)
                != 255i32
            {
                if ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1638))
                .read()) as i32)
                    == (((id) as u8) as i32)
                {
                    ItemStorage_DrawSwapArrow(yOffset, 0u8, 255u8);
                } else {
                    ItemStorage_DrawSwapArrow(yOffset, 255u8, 255u8);
                }
            }
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset(((id) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32),
                1i32,
                3u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_xVar1).cast::<u8>(),
            );
            AddTextPrinterParameterized(
                windowId,
                7u8,
                (&raw mut gStringVar4).cast::<u8>(),
                ((GetStringRightAlignXOffset(7i32, (&raw mut gStringVar4).cast::<u8>(), 104i32))
                    as u8),
                yOffset,
                255u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_PrintDescription(id: i32) {
    unsafe {
        let mut id = id;
        let mut description: *mut u8 = core::ptr::null_mut();
        let mut windowId: u8 = ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(1632))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        if id != (-2i32) {
            description = GetItemDescription(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset((id) as isize * 4))
                .cast::<u16>())
                .read(),
            );
        } else {
            description = ItemStorage_GetMessage(65535u16);
        }
        FillWindowPixelBuffer(windowId, 17u8);
        AddTextPrinterParameterized(windowId, 1u8, description, 0u8, 1u8, 0u8, None);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_AddScrollIndicator() {
    unsafe {
        if (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).read()) as i32)
            == 255i32
        {
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).write(
                AddScrollIndicatorArrowPairParameterized(
                    2u32,
                    176i32,
                    12i32,
                    148i32,
                    (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read())
                        as i32)
                        .wrapping_sub(
                            (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4))
                                .read()) as i32),
                        ),
                    5112i32,
                    5112i32,
                    ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>(),
                ),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_RemoveScrollIndicator() {
    unsafe {
        if (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).read()) as i32)
            != 255i32
        {
            RemoveScrollIndicatorArrowPair(
                (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).read(),
            );
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(9)).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_SetSwapArrow(listTaskId: u8, b: u8, speed: u8) {
    unsafe {
        let mut listTaskId = listTaskId;
        let mut b = b;
        let mut speed = speed;
        ItemStorage_DrawSwapArrow(
            ((ListMenuGetYCoordForPrintingArrowCursor(listTaskId)) as u8),
            b,
            speed,
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DrawSwapArrow(y: u8, b: u8, speed: u8) {
    unsafe {
        let mut y = y;
        let mut b = b;
        let mut speed = speed;
        let mut windowId: u8 = (((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(1632))
        .cast::<u8>())
        .read();
        if ((b) as i32) == 255i32 {
            FillWindowPixelRect(
                windowId,
                17u8,
                0u16,
                ((y) as u16),
                ((GetMenuCursorDimensionByFont(1u8, 0u8)) as u16),
                ((GetMenuCursorDimensionByFont(1u8, 1u8)) as u16),
            );
        } else {
            AddTextPrinterParameterized4(
                windowId,
                1u8,
                0u8,
                y,
                0u8,
                0u8,
                ((&raw const sSwapArrowTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                ((speed) as i8),
                (&raw mut gText_SelectorArrow2).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DrawItemIcon(itemId: u16) {
    unsafe {
        let mut itemId = itemId;
        let mut spriteId: u8 = 0u8;
        let mut spriteIdLoc: *mut u8 =
            (((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1639);
        if (((spriteIdLoc).read()) as i32) == 255i32 {
            FreeSpriteTilesByTag(5110u16);
            FreeSpritePaletteByTag(5110u16);
            spriteId = AddItemIconSprite(5110u16, 5110u16, itemId);
            if ((spriteId) as i32) != 64i32 {
                (spriteIdLoc).write(spriteId);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(24i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(80i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_EraseItemIcon() {
    unsafe {
        let mut spriteIdLoc: *mut u8 =
            (((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1639);
        if (((spriteIdLoc).read()) as i32) != 255i32 {
            FreeSpriteTilesByTag(5110u16);
            FreeSpritePaletteByTag(5110u16);
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset((((spriteIdLoc).read()) as i32) as isize * 68),
            );
            (spriteIdLoc).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_CompactList() {
    unsafe {
        CompactPCItems();
        SetItemListPerPageCount(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>(),
            50u8,
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4),
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5),
            8u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_CompactCursor() {
    unsafe {
        SetCursorWithinListBounds(
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>(),
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(4)).read(),
            (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_CreateListMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut toss: u32 = 0u32;
        let mut i: u32 = 0u32;
        let mut x: u32 = 0u32;
        let mut text: *mut u8 = core::ptr::null_mut();
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        {
            i = 0u32;
            'l1: loop {
                if !(i <= 3u32) {
                    break 'l1;
                }
                'l2: {
                    ItemStorage_AddWindow(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        toss = ((((data).wrapping_offset(3)).read()) as u32);
        text = (&raw mut gText_TossItem).cast::<u8>();
        if !((toss) != 0) {
            text = (&raw mut gText_WithdrawItem).cast::<u8>();
        }
        x = ((GetStringCenterAlignXOffset(1i32, text, 104i32)) as u32);
        AddTextPrinterParameterized(
            ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1632))
            .cast::<u8>())
            .wrapping_offset(3))
            .read(),
            1u8,
            text,
            ((x) as u8),
            1u8,
            0u8,
            None,
        );
        CopyWindowToVram(
            ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1632))
            .cast::<u8>())
            .wrapping_offset(2))
            .read(),
            2u8,
        );
        ItemStorage_CompactList();
        ItemStorage_CompactCursor();
        ItemStorage_RefreshListMenu();
        ((data).wrapping_offset(5)).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read(),
                (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read(),
            )) as i16),
        );
        ItemStorage_AddScrollIndicator();
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_ProcessInput));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_GetMessage(itemId: u16) -> *mut u8 {
    unsafe {
        let mut itemId = itemId;
        let mut string: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((itemId) as i32);
            let __matched = __sw1 == 65535i32
                || __sw1 == 65534i32
                || __sw1 == 65533i32
                || __sw1 == 65532i32
                || __sw1 == 65531i32
                || __sw1 == 65530i32
                || __sw1 == 65529i32
                || __sw1 == 65528i32
                || __sw1 == 65527i32;
            if __sw1 == 65535i32 {
                string = (&raw mut gText_GoBackPrevMenu).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65534i32 {
                string = (&raw mut gText_WithdrawHowManyItems).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65533i32 {
                string = (&raw mut gText_WithdrawXItems).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65532i32 {
                string = (&raw mut gText_TossHowManyVar1s).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65531i32 {
                string = (&raw mut gText_ThrewAwayVar2Var1s).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65530i32 {
                string = (&raw mut gText_NoRoomInBag).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65529i32 {
                string = (&raw mut gText_TooImportantToToss).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65528i32 {
                string = (&raw mut gText_ConfirmTossItems).cast::<u8>();
                break 'l1;
            }
            if __sw1 == 65527i32 {
                string = (&raw mut gText_MoveVar1Where).cast::<u8>();
                break 'l1;
            }
            if !__matched {
                string = GetItemDescription(itemId);
                break 'l1;
            }
        }
        return string;
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_PrintMessage(string: *mut u8) {
    unsafe {
        let mut string = string;
        let mut windowId: u8 = ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_add(1632))
        .cast::<u8>())
        .wrapping_offset(1))
        .read();
        FillWindowPixelBuffer(windowId, 17u8);
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), string);
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ProcessInput(taskId: u8) {
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
            & 4i32)
            != 0
        {
            ListMenuGetScrollAndRow(
                ((((data).wrapping_offset(5)).read()) as u8),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>(),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
            );
            if (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>())
            .read()) as i32)
                .wrapping_add(
                    (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read())
                        as i32),
                )
                != (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).wrapping_add(5)).read())
                    as i32)
                    .wrapping_sub(1i32)
            {
                PlaySE(5u16);
                ItemStorage_StartItemSwap(taskId);
            }
        } else {
            let mut id: i32 = ListMenu_ProcessInput(((((data).wrapping_offset(5)).read()) as u8));
            ListMenuGetScrollAndRow(
                ((((data).wrapping_offset(5)).read()) as u8),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>(),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
            );
            'l1: {
                let __sw1 = id;
                let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
                if __sw1 == (-1i32) {
                    break 'l1;
                }
                if __sw1 == (-2i32) {
                    PlaySE(5u16);
                    ItemStorage_ExitItemList(taskId);
                    break 'l1;
                }
                if !__matched {
                    PlaySE(5u16);
                    ItemStorage_DoItemAction(taskId);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ReturnToMenuSelect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            DrawDialogueFrame(0u8, 0u8);
            if !((((data).wrapping_offset(3)).read()) != 0) {
                InitItemStorageMenu(taskId, 0u8);
            } else {
                InitItemStorageMenu(taskId, 2u8);
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemStorageMenuProcessInput));
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ExitItemList(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ItemStorage_EraseItemIcon();
        ItemStorage_RemoveScrollIndicator();
        DestroyListMenuTask(
            ((((data).wrapping_offset(5)).read()) as u8),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        DestroySwapLineSprites(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1640))
            .cast::<u8>(),
            7u8,
        );
        ItemStorage_Free();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_ReturnToMenuSelect));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_StartItemSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ListMenuSetTemplateField(((((data).wrapping_offset(5)).read()) as u8), 16u8, 1i32);
        ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1638))
            .write(
                (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(
                        (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read())
                            as i32),
                    )) as u8),
            );
        ItemStorage_SetSwapArrow(((((data).wrapping_offset(5)).read()) as u8), 0u8, 0u8);
        ItemStorage_UpdateSwapLinePos(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1638))
            .read(),
        );
        CopyItemName(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1638))
                .read()) as i32) as isize
                    * 4,
            ))
            .cast::<u16>())
            .read(),
            (&raw mut gStringVar1).cast::<u8>(),
        );
        ItemStorage_PrintMessage(ItemStorage_GetMessage(65527u16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_ProcessItemSwapInput));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ProcessItemSwapInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut id: i32 = 0i32;
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 4i32)
            != 0
        {
            ListMenuGetScrollAndRow(
                ((((data).wrapping_offset(5)).read()) as u8),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>(),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
            );
            ItemStorage_FinishItemSwap(taskId, 0u8);
            return;
        }
        id = ListMenu_ProcessInput(((((data).wrapping_offset(5)).read()) as u8));
        ListMenuGetScrollAndRow(
            ((((data).wrapping_offset(5)).read()) as u8),
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>(),
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
        );
        SetSwapLineSpritesInvisibility(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1640))
            .cast::<u8>(),
            7u8,
            0u8,
        );
        ItemStorage_UpdateSwapLinePos(
            (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read()) as u8),
        );
        'l1: {
            let __sw1 = id;
            let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
            if __sw1 == (-1i32) {
                break 'l1;
            }
            if __sw1 == (-2i32) {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    ItemStorage_FinishItemSwap(taskId, 0u8);
                } else {
                    ItemStorage_FinishItemSwap(taskId, 1u8);
                }
                break 'l1;
            }
            if !__matched {
                ItemStorage_FinishItemSwap(taskId, 0u8);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_FinishItemSwap(taskId: u8, canceled: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut canceled = canceled;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut newPos: u16 = (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
            .wrapping_add(2)
            .cast::<u16>())
        .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read()) as i32),
            )) as u16);
        PlaySE(5u16);
        DestroyListMenuTask(
            ((((data).wrapping_offset(5)).read()) as u8),
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                .wrapping_add(2)
                .cast::<u16>(),
            ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
        );
        if ((!((canceled) != 0))
            && (((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1638))
            .read()) as i32)
                != ((newPos) as i32)))
            && (((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1638))
            .read()) as i32)
                != ((newPos) as i32).wrapping_sub(1i32))
        {
            MoveItemSlotInList(
                ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>(),
                ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(1638))
                .read()) as u32),
                ((newPos) as u32),
            );
            ItemStorage_RefreshListMenu();
        }
        if ((((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(1638))
        .read()) as i32)
            < ((newPos) as i32)
        {
            let __p1 = ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
        SetSwapLineSpritesInvisibility(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1640))
            .cast::<u8>(),
            7u8,
            1u8,
        );
        ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(1638))
            .write(255u8);
        ((data).wrapping_offset(5)).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                (((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read(),
                (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read(),
            )) as i16),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_ProcessInput));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_UpdateSwapLinePos(y: u8) {
    unsafe {
        let mut y = y;
        UpdateSwapLineSpritesPos(
            ((((&raw mut sItemStorageMenu).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(1640))
            .cast::<u8>(),
            7u8,
            128i16,
            (((((y) as i32).wrapping_add(1i32)).wrapping_mul(16i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_PrintItemQuantity(
    windowId: u8,
    value: u16,
    mode: u32,
    x: u8,
    y: u8,
    n: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut value = value;
        let mut mode = mode;
        let mut x = x;
        let mut y = y;
        let mut n = n;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((value) as i32),
            ((mode) as i32),
            n,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_xVar1).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((GetStringCenterAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 48i32)) as u8),
            y,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DoItemAction(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut pos: u16 = (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>())
            .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16);
        ItemStorage_RemoveScrollIndicator();
        ((data).wrapping_offset(2)).write(1i16);
        if !((((data).wrapping_offset(3)).read()) != 0) {
            if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(((pos) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                == 1i32
            {
                ItemStorage_DoItemWithdraw(taskId);
                return;
            }
            CopyItemName(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset(((pos) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            ItemStorage_PrintMessage(ItemStorage_GetMessage(65534u16));
        } else {
            if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(((pos) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read()) as i32)
                == 1i32
            {
                ItemStorage_DoItemToss(taskId);
                return;
            }
            CopyItemName(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset(((pos) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            ItemStorage_PrintMessage(ItemStorage_GetMessage(65532u16));
        }
        ItemStorage_PrintItemQuantity(
            ItemStorage_AddWindow(4u8),
            ((((data).wrapping_offset(2)).read()) as u16),
            2u32,
            8u8,
            1u8,
            3u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_HandleQuantityRolling));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleQuantityRolling(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut pos: u16 = (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>())
            .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16);
        if ((AdjustQuantityAccordingToDPadInput(
            (data).wrapping_offset(2),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(((pos) as i32) as isize * 4))
            .wrapping_add(2)
            .cast::<u16>())
            .read(),
        )) as i32)
            == 1i32
        {
            ItemStorage_PrintItemQuantity(
                ItemStorage_AddWindow(4u8),
                ((((data).wrapping_offset(2)).read()) as u16),
                2u32,
                8u8,
                1u8,
                3u8,
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
                ItemStorage_RemoveWindow(4u8);
                if !((((data).wrapping_offset(3)).read()) != 0) {
                    ItemStorage_DoItemWithdraw(taskId);
                } else {
                    ItemStorage_DoItemToss(taskId);
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
                    ItemStorage_RemoveWindow(4u8);
                    ItemStorage_PrintMessage(ItemStorage_GetMessage(
                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(1176))
                        .cast::<u8>())
                        .wrapping_offset(((pos) as i32) as isize * 4))
                        .cast::<u16>())
                        .read(),
                    ));
                    ItemStorage_ReturnToListInput(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DoItemWithdraw(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut pos: u16 = (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>())
            .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16);
        if ((AddBagItem(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(((pos) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
            ((((data).wrapping_offset(2)).read()) as u16),
        )) as i32)
            == 1i32
        {
            CopyItemName(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset(((pos) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((((data).wrapping_offset(2)).read()) as i32),
                0i32,
                3u8,
            );
            ItemStorage_PrintMessage(ItemStorage_GetMessage(65533u16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemStorage_HandleRemoveItem));
        } else {
            ((data).wrapping_offset(2)).write(0i16);
            ItemStorage_PrintMessage(ItemStorage_GetMessage(65530u16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemStorage_HandleErrorMessageInput));
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DoItemToss(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut pos: u16 = (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>())
            .read()) as i32)
            .wrapping_add(
                (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32),
            )) as u16);
        if !((GetItemImportance(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(((pos) as i32) as isize * 4))
            .cast::<u16>())
            .read(),
        )) != 0)
        {
            CopyItemName(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset(((pos) as i32) as isize * 4))
                .cast::<u16>())
                .read(),
                (&raw mut gStringVar1).cast::<u8>(),
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((((data).wrapping_offset(2)).read()) as i32),
                0i32,
                3u8,
            );
            ItemStorage_PrintMessage(ItemStorage_GetMessage(65528u16));
            CreateYesNoMenuWithCallbacks(
                taskId,
                (((&raw const sWindowTemplates_ItemStorage)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(40),
                1u8,
                0u8,
                1u8,
                532u16,
                14u8,
                (&raw const ItemTossYesNoFuncs).cast::<u8>().cast_mut(),
            );
        } else {
            ((data).wrapping_offset(2)).write(0i16);
            ItemStorage_PrintMessage(ItemStorage_GetMessage(65529u16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemStorage_HandleErrorMessageInput));
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_TossItemYes(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ItemStorage_PrintMessage(ItemStorage_GetMessage(65531u16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_HandleRemoveItem));
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_TossItemNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ItemStorage_PrintMessage(ItemStorage_GetMessage(
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                .cast::<u8>())
            .wrapping_offset(
                ((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_add(
                        (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read())
                            as i32),
                    )) as isize
                    * 4,
            ))
            .cast::<u16>())
            .read(),
        ));
        ItemStorage_ReturnToListInput(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleRemoveItem(taskId: u8) {
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
            RemovePCItem(
                (((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read()) as i32)
                    .wrapping_add(
                        (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                            .wrapping_add(2)
                            .cast::<u16>())
                        .read()) as i32),
                    )) as u8),
                ((((data).wrapping_offset(2)).read()) as u16),
            );
            DestroyListMenuTask(
                ((((data).wrapping_offset(5)).read()) as u8),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<u16>(),
                ((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>(),
            );
            ItemStorage_CompactList();
            ItemStorage_CompactCursor();
            ItemStorage_RefreshListMenu();
            ((data).wrapping_offset(5)).write(
                ((ListMenuInit(
                    (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                    (((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read(),
                    (((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>()).read(),
                )) as i16),
            );
            ItemStorage_ReturnToListInput(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleErrorMessageInput(taskId: u8) {
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
            ItemStorage_PrintMessage(ItemStorage_GetMessage(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(1176))
                    .cast::<u8>())
                .wrapping_offset(
                    ((((((&raw mut gPlayerPCItemPageInfo).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<u16>())
                    .read()) as i32)
                        .wrapping_add(
                            (((((&raw mut gPlayerPCItemPageInfo).cast::<u8>()).cast::<u16>())
                                .read()) as i32),
                        )) as isize
                        * 4,
                ))
                .cast::<u16>())
                .read(),
            ));
            ItemStorage_ReturnToListInput(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ReturnToListInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ItemStorage_AddScrollIndicator();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemStorage_ProcessInput));
    }
}
