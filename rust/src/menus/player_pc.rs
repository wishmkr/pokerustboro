//! Translated from `src/player_pc.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::bg::IsDma3ManagerBusyWithBgCopy;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::decoration::DoPlayerRoomDecorationMenu;
use crate::event_object_movement::FreeAndReserveObjectSpritePalettes;
use crate::field_screen_effect::FadeInFromBlack;
use crate::field_weather::{FadeScreen, IsWeatherNotFadingIn};
use crate::international_string_util::ConvertInternationalPlayerNameStripChar;
use crate::item::{
    AddBagItem, AddPCItem, ClearItemSlots, CompactPCItems, CopyItemName, CountUsedPCItemSlots,
    GetItemImportance, MoveItemSlotInList, RemovePCItem,
};
use crate::item_icon::AddItemIconSprite;
use crate::item_menu::CB2_GoToItemDepositMenu;
use crate::list_menu::{
    AddScrollIndicatorArrowPairParameterized, DestroyListMenuTask, ListMenu_ProcessInput,
    ListMenuGetScrollAndRow, ListMenuGetYCoordForPrintingArrowCursor, ListMenuInit,
    ListMenuSetTemplateField, RemoveScrollIndicatorArrowPair, gMultiuseListMenuTemplate,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::mail::ReadMail;
use crate::menu::{
    AddTextPrinterParameterized4, ClearDialogWindowAndFrame, ClearStdWindowAndFrameToTransparent,
    DisplayItemMessageOnField, DisplayYesNoMenuDefaultYes, DrawDialogueFrame,
    DrawStdFrameWithCustomTileAndPalette, InitMenuInUpperLeftCornerNormal,
    LoadMessageBoxAndBorderGfx, Menu_GetCursorPos, Menu_ProcessInput, Menu_ProcessInputNoWrap,
    Menu_ProcessInputNoWrapClearOnChoose, PrintMenuActionTextsInUpperLeftCorner, PrintMenuTable,
    ProcessMenuInput_other, ScheduleBgCopyTilemapToVram, SetStandardWindowBorderStyle,
};
use crate::menu_helpers::{
    AdjustQuantityAccordingToDPadInput, CreateSwapLineSprites, CreateYesNoMenuWithCallbacks,
    DestroySwapLineSprites, LoadListMenuSwapLineGfx, SetCursorWithinListBounds,
    SetItemListPerPageCount, SetSwapLineSpritesInvisibility, UpdateSwapLineSpritesPos,
};
use crate::menu_specialized::{
    MailboxMenu_AddScrollArrows, MailboxMenu_AddWindow, MailboxMenu_Alloc, MailboxMenu_CreateList,
    MailboxMenu_Free, MailboxMenu_RemoveWindow,
};
use crate::overworld::{CB2_ReturnToField, CleanupOverworldWindowsAndTilemaps, gFieldCallback};
use crate::palette::gPaletteFade;
use crate::party_menu::ChooseMonToGiveMailFromMailbox;
use crate::pokemon::CalculatePlayerPartyCount;
use crate::script::ScriptContext_Enable;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::{DestroyTask, TaskDummy};
use crate::task::{gTasks, task_set, task_set_func};
use crate::text::GetMenuCursorDimensionByFont;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect, RemoveWindow,
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
/// `ClearMail` with this module's view of its types.
#[inline]
unsafe fn ClearMail(a0: *mut Mail) {
    unsafe {
        crate::mail_data::ClearMail(a0 as _);
    }
}
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `GetMaxWidthInMenuTable` with this module's view of its types.
#[inline]
unsafe fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32 {
    unsafe { crate::international_string_util::GetMaxWidthInMenuTable(a0 as _, a1) }
}
/// `GetMaxWidthInSubsetOfMenuTable` with this module's view of its types.
#[inline]
unsafe fn GetMaxWidthInSubsetOfMenuTable(a0: *mut MenuAction, a1: *mut u8, a2: i32) -> i32 {
    unsafe {
        crate::international_string_util::GetMaxWidthInSubsetOfMenuTable(a0 as _, a1 as _, a2)
    }
}
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *mut u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringExpandPlaceholders` with this module's view of its types.
#[inline]
unsafe fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringExpandPlaceholders(a0 as _, a1 as _) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tListTaskId: usize = 5;
// Data tables (translate with cdata.py): sItemStorage_OptionDescriptions sPlayerPCMenuActions sBedroomPC_OptionOrder sPlayerPC_OptionOrder sItemStorage_MenuActions sNewGamePCItems gMailboxMailOptions sWindowTemplates_MainMenus ItemTossYesNoFuncs sListMenuTemplate_ItemStorage sWindowTemplates_ItemStorage sSwapArrowTextColors

/// `struct ItemStorageMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ItemStorageMenu {
    pub listItems: CArray<ListMenuItem, 51>,
    pub itemNames: CArray<CArray<u8, 24>, 51>,
    pub windowIds: CArray<u8, 6>,
    pub toSwapPos: u8,
    pub spriteId: u8,
    pub swapLineSpriteIds: CArray<u8, 7>,
}

unsafe impl Sync for ItemStorageMenu {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ItemStorageMenu>() == 1648);
    assert!(offset_of!(ItemStorageMenu, listItems) == 0);
    assert!(offset_of!(ItemStorageMenu, itemNames) == 408);
    assert!(offset_of!(ItemStorageMenu, windowIds) == 1632);
    assert!(offset_of!(ItemStorageMenu, toSwapPos) == 1638);
    assert!(offset_of!(ItemStorageMenu, spriteId) == 1639);
    assert!(offset_of!(ItemStorageMenu, swapLineSpriteIds) == 1640);
};

const ITEMPC_WIN_COUNT: u32 = 6;
const ITEMPC_WIN_ICON: i32 = 2;
const ITEMPC_WIN_LIST: u8 = 0;
const ITEMPC_WIN_MESSAGE: i32 = 1;
const ITEMPC_WIN_QUANTITY: u8 = 4;
const ITEMPC_WIN_TITLE: u32 = 3;
const ITEMPC_WIN_YESNO: i32 = 5;
const MENU_TOSS: u8 = 2;
const MENU_WITHDRAW: u8 = 0;
const MSG_GO_BACK_TO_PREV: u16 = 65535;
const MSG_HOW_MANY_TO_TOSS: u16 = 65532;
const MSG_HOW_MANY_TO_WITHDRAW: u16 = 65534;
const MSG_NO_MORE_ROOM: u16 = 65530;
const MSG_OKAY_TO_THROW_AWAY: u16 = 65528;
const MSG_SWITCH_WHICH_ITEM: u16 = 65527;
const MSG_THREW_AWAY_ITEM: u16 = 65531;
const MSG_TOO_IMPORTANT: u16 = 65529;
const MSG_WITHDREW_ITEM: u16 = 65533;
const NOT_SWAPPING: u8 = 255;
const SWAP_LINE_LENGTH: u8 = 7;
const TAG_ITEM_ICON: u16 = 5110;
const TAG_SCROLL_ARROW: i32 = 5112;
const WIN_ITEM_STORAGE_MENU: i32 = 2;
const WIN_MAIN_MENU: i32 = 0;
const WIN_MAIN_MENU_BEDROOM: i32 = 1;

static ItemTossYesNoFuncs: Table<YesNoFuncTable> =
    Table((&raw const crate::data::player_pc::ItemTossYesNoFuncs).cast());
static gMailboxMailOptions: Table<CArray<MenuAction, 4>> =
    Table((&raw const crate::data::player_pc::gMailboxMailOptions).cast());
static sBedroomPC_OptionOrder: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::player_pc::sBedroomPC_OptionOrder).cast());
static sItemStorage_MenuActions: Table<CArray<MenuAction, 4>> =
    Table((&raw const crate::data::player_pc::sItemStorage_MenuActions).cast());
static sItemStorage_OptionDescriptions: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::player_pc::sItemStorage_OptionDescriptions).cast());
static sListMenuTemplate_ItemStorage: Table<ListMenuTemplate> =
    Table((&raw const crate::data::player_pc::sListMenuTemplate_ItemStorage).cast());
static sNewGamePCItems: Table<CArray<CArray<u16, 2>, 2>> =
    Table((&raw const crate::data::player_pc::sNewGamePCItems).cast());
static sPlayerPCMenuActions: Table<CArray<MenuAction, 4>> =
    Table((&raw const crate::data::player_pc::sPlayerPCMenuActions).cast());
static sPlayerPC_OptionOrder: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::player_pc::sPlayerPC_OptionOrder).cast());
static sSwapArrowTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::player_pc::sSwapArrowTextColors).cast());
static sWindowTemplates_ItemStorage: Table<CArray<WindowTemplate, 6>> =
    Table((&raw const crate::data::player_pc::sWindowTemplates_ItemStorage).cast());
static sWindowTemplates_MainMenus: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::player_pc::sWindowTemplates_MainMenus).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTopMenuOptionOrder: *mut u8 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sTopMenuNumOptions: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPCItemPageInfo: PlayerPCItemPageStruct = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemStorageMenu: *mut ItemStorageMenu = null_mut();

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
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetItemDescription` with this module's view of its types.
#[inline]
unsafe fn GetItemDescription(a0: u16) -> *mut u8 {
    crate::item::GetItemDescription(a0) as *mut u8
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn NewGameInitPCItems() {
    let mut i: u8 = 0;
    ClearItemSlots((*gSaveBlock1Ptr).pcItems.as_mut_ptr(), PC_ITEMS_COUNT);
    loop {
        if sNewGamePCItems[i][0] == ITEM_NONE || sNewGamePCItems[i][1] == 0 {
            break;
        }
        if AddPCItem(sNewGamePCItems[i][0], sNewGamePCItems[i][1]) != 1 {
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BedroomPC() {
    sTopMenuOptionOrder = sBedroomPC_OptionOrder.as_ptr().cast_mut();
    sTopMenuNumOptions.set(4);
    DisplayItemMessageOnField(
        CreateTask(Some(TaskDummy), 0),
        (*(&raw const crate::data::strings::gText_WhatWouldYouLike).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(InitPlayerPCMenu),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerPC() {
    sTopMenuOptionOrder = sPlayerPC_OptionOrder.as_ptr().cast_mut();
    sTopMenuNumOptions.set(3);
    DisplayItemMessageOnField(
        CreateTask(Some(TaskDummy), 0),
        (*(&raw const crate::data::strings::gText_WhatWouldYouLike).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(InitPlayerPCMenu),
    );
}
pub(crate) unsafe fn InitPlayerPCMenu(taskId: u8) {
    let mut data: *mut u16 = null_mut();
    let mut windowTemplate: WindowTemplate = zeroed();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    if sTopMenuNumOptions.get() == 3 {
        windowTemplate = sWindowTemplates_MainMenus[0];
    } else {
        windowTemplate = sWindowTemplates_MainMenus[1];
    }
    windowTemplate.width = GetMaxWidthInSubsetOfMenuTable(
        sPlayerPCMenuActions.as_ptr().cast_mut(),
        sTopMenuOptionOrder,
        sTopMenuNumOptions.get() as i32,
    ) as u8;
    *data.at(4) = AddWindow(&raw mut windowTemplate);
    SetStandardWindowBorderStyle(*data.at(4) as u8, FALSE);
    PrintMenuActionTextsInUpperLeftCorner(
        *data.at(4) as u8,
        sTopMenuNumOptions.get(),
        sPlayerPCMenuActions.as_ptr().cast_mut(),
        sTopMenuOptionOrder,
    );
    InitMenuInUpperLeftCornerNormal(*data.at(4) as u8, sTopMenuNumOptions.get(), 0);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(PlayerPCProcessMenuInput));
}
pub(crate) unsafe fn PlayerPCProcessMenuInput(taskId: u8) {
    let mut data: *mut u16 = null_mut();
    let mut inputOptionId: i8 = 0;
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    if sTopMenuNumOptions.get() > 3 {
        inputOptionId = Menu_ProcessInput();
    } else {
        inputOptionId = Menu_ProcessInputNoWrap();
    }
    match inputOptionId {
        MENU_NOTHING_CHOSEN => {}
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            ClearStdWindowAndFrameToTransparent(*data.at(4) as u8, FALSE);
            ClearWindowTilemap(*data.at(4) as u8);
            RemoveWindow(*data.at(4) as u8);
            ScheduleBgCopyTilemapToVram(0);
            task_set_func(taskId, Some(PlayerPC_TurnOff));
        }
        _ => {
            ClearStdWindowAndFrameToTransparent(*data.at(4) as u8, FALSE);
            ClearWindowTilemap(*data.at(4) as u8);
            RemoveWindow(*data.at(4) as u8);
            ScheduleBgCopyTilemapToVram(0);
            task_set_func(
                taskId,
                sPlayerPCMenuActions[*sTopMenuOptionOrder.at(inputOptionId)]
                    .func
                    .void_u8,
            );
        }
    }
}
pub unsafe fn ReshowPlayerPC(var: u8) {
    DisplayItemMessageOnField(
        var,
        (*(&raw const crate::data::strings::gText_WhatWouldYouLike).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(InitPlayerPCMenu),
    );
}
pub(crate) unsafe fn PlayerPC_ItemStorage(taskId: u8) {
    InitItemStorageMenu(taskId, MENU_WITHDRAW);
    task_set_func(taskId, Some(ItemStorageMenuProcessInput));
}
pub(crate) unsafe fn PlayerPC_Mailbox(taskId: u8) {
    gPlayerPCItemPageInfo.count = GetMailboxMailCount();
    if gPlayerPCItemPageInfo.count == 0 {
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_NoMailHere).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(ReshowPlayerPC),
        );
    } else {
        gPlayerPCItemPageInfo.cursorPos = 0;
        gPlayerPCItemPageInfo.itemsAbove = 0;
        gPlayerPCItemPageInfo.scrollIndicatorTaskId = TASK_NONE;
        Mailbox_CompactMailList();
        SetPlayerPCListCount(taskId);
        if MailboxMenu_Alloc(gPlayerPCItemPageInfo.count) == TRUE {
            ClearDialogWindowAndFrame(0, 0);
            Mailbox_DrawMailboxMenu(taskId);
            task_set_func(taskId, Some(Mailbox_ProcessInput));
        } else {
            DisplayItemMessageOnField(
                taskId,
                (*(&raw const crate::data::strings::gText_NoMailHere).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                Some(ReshowPlayerPC),
            );
        }
    }
}
pub(crate) unsafe fn PlayerPC_Decoration(taskId: u8) {
    DoPlayerRoomDecorationMenu(taskId);
}
pub(crate) unsafe fn PlayerPC_TurnOff(taskId: u8) {
    if sTopMenuNumOptions.get() == 4 {
        if (*gSaveBlock2Ptr).playerGender == MALE {
            ScriptContext_SetupScript(
                (*crate::asmdata::LittlerootTown_BrendansHouse_2F_EventScript_TurnOffPlayerPC
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        } else {
            ScriptContext_SetupScript(
                (*crate::asmdata::LittlerootTown_MaysHouse_2F_EventScript_TurnOffPlayerPC
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
    } else {
        ScriptContext_Enable();
    }
    DestroyTask(taskId);
}
unsafe fn InitItemStorageMenu(taskId: u8, var: u8) {
    let mut data: *mut u16 = null_mut();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    let mut windowTemplate: WindowTemplate = sWindowTemplates_MainMenus[2];
    windowTemplate.width =
        GetMaxWidthInMenuTable(sItemStorage_MenuActions.as_ptr().cast_mut(), 4) as u8;
    *data.at(4) = AddWindow(&raw mut windowTemplate);
    SetStandardWindowBorderStyle(*data.at(4) as u8, FALSE);
    PrintMenuTable(
        *data.at(4) as u8,
        4,
        sItemStorage_MenuActions.as_ptr().cast_mut(),
    );
    InitMenuInUpperLeftCornerNormal(*data.at(4) as u8, 4, var);
    ScheduleBgCopyTilemapToVram(0);
    ItemStorageMenuPrint(sItemStorage_OptionDescriptions[var]);
}
unsafe fn ItemStorageMenuPrint(textPtr: *mut u8) {
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized(0, FONT_NORMAL, textPtr, 0, 1, 0, None);
}
pub(crate) unsafe fn ItemStorageMenuProcessInput(taskId: u8) {
    let oldPos: i8 = Menu_GetCursorPos() as i8;
    let inputOptionId: i8 = Menu_ProcessInput();
    let newPos: i8 = Menu_GetCursorPos() as i8;
    match inputOptionId {
        MENU_NOTHING_CHOSEN => {
            if oldPos != newPos {
                ItemStorageMenuPrint(sItemStorage_OptionDescriptions[newPos]);
            }
        }
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            ItemStorage_Exit(taskId);
        }
        _ => {
            PlaySE(SE_SELECT);
            sItemStorage_MenuActions[inputOptionId]
                .func
                .void_u8
                .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe fn ItemStorage_Deposit(taskId: u8) {
    task_set_func(taskId, Some(Task_ItemStorage_Deposit));
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe fn Task_ItemStorage_Deposit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        CB2_GoToItemDepositMenu();
        DestroyTask(taskId);
    }
}
pub unsafe fn CB2_PlayerPCExitBagMenu() {
    gFieldCallback = Some(ItemStorage_ReshowAfterBagMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn ItemStorage_ReshowAfterBagMenu() {
    LoadMessageBoxAndBorderGfx();
    DrawDialogueFrame(0, TRUE);
    InitItemStorageMenu(
        CreateTask(Some(ItemStorage_HandleReturnToProcessInput), 0),
        1,
    );
    FadeInFromBlack();
}
pub(crate) unsafe fn ItemStorage_HandleReturnToProcessInput(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        task_set_func(taskId, Some(ItemStorageMenuProcessInput));
    }
}
pub(crate) unsafe fn ItemStorage_Withdraw(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) = CountUsedPCItemSlots() as i16;
    if *data.at(1) != 0 {
        ItemStorage_Enter(taskId, FALSE);
    } else {
        ItemStorage_EraseMainMenu(taskId);
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_NoItems).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(PlayerPC_ItemStorage),
        );
    }
}
pub(crate) unsafe fn ItemStorage_Toss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) = CountUsedPCItemSlots() as i16;
    if *data.at(1) != 0 {
        ItemStorage_Enter(taskId, TRUE);
    } else {
        ItemStorage_EraseMainMenu(taskId);
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_NoItems).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(PlayerPC_ItemStorage),
        );
    }
}
unsafe fn ItemStorage_Enter(taskId: u8, toss: u8) {
    let data: *mut u16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    *data.at(3) = toss as u16;
    ItemStorage_EraseMainMenu(taskId);
    gPlayerPCItemPageInfo.cursorPos = 0;
    gPlayerPCItemPageInfo.itemsAbove = 0;
    gPlayerPCItemPageInfo.scrollIndicatorTaskId = TASK_NONE;
    SetPlayerPCListCount(taskId);
    ItemStorage_Init();
    FreeAndReserveObjectSpritePalettes();
    LoadListMenuSwapLineGfx();
    CreateSwapLineSprites(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
    );
    ClearDialogWindowAndFrame(0, 0);
    task_set_func(taskId, Some(ItemStorage_CreateListMenu));
}
pub(crate) unsafe fn ItemStorage_Exit(taskId: u8) {
    ItemStorage_EraseMainMenu(taskId);
    ReshowPlayerPC(taskId);
}
unsafe fn SetPlayerPCListCount(taskId: u8) {
    if gPlayerPCItemPageInfo.count > 7 {
        gPlayerPCItemPageInfo.pageItems = 8;
    } else {
        gPlayerPCItemPageInfo.pageItems = gPlayerPCItemPageInfo.count + 1;
    }
}
unsafe fn ItemStorage_EraseMainMenu(taskId: u8) {
    let data: *mut u16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    ClearStdWindowAndFrameToTransparent(*data.at(4) as u8, FALSE);
    ClearWindowTilemap(*data.at(4) as u8);
    RemoveWindow(*data.at(4) as u8);
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn GetMailboxMailCount() -> u8 {
    let mut mailInPC: u8 = 0;
    for i in (PARTY_SIZE as u8)..MAIL_COUNT {
        if (*gSaveBlock1Ptr).mail[i].itemId != ITEM_NONE {
            mailInPC += 1;
        }
    }
    mailInPC
}
unsafe fn Mailbox_CompactMailList() {
    let mut temp: Mail = zeroed();
    for i in (PARTY_SIZE as u8)..15 {
        for j in (i + 1)..MAIL_COUNT {
            if (*gSaveBlock1Ptr).mail[i].itemId == ITEM_NONE {
                temp = (*gSaveBlock1Ptr).mail[i];
                (*gSaveBlock1Ptr).mail[i] = (*gSaveBlock1Ptr).mail[j];
                (*gSaveBlock1Ptr).mail[j] = temp;
            }
        }
    }
}
unsafe fn Mailbox_DrawMailboxMenu(taskId: u8) {
    let windowId: u8 = MailboxMenu_AddWindow(MAILBOXWIN_TITLE);
    MailboxMenu_AddWindow(MAILBOXWIN_LIST);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Mailbox).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_Mailbox).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0x40,
        ) as u8,
        1,
        0,
        None,
    );
    ScheduleBgCopyTilemapToVram(0);
    task_set(
        taskId,
        tListTaskId,
        MailboxMenu_CreateList(&raw mut gPlayerPCItemPageInfo) as i16,
    );
    MailboxMenu_AddScrollArrows(&raw mut gPlayerPCItemPageInfo);
}
pub(crate) unsafe fn Mailbox_ProcessInput(taskId: u8) {
    let data: *mut u16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut u16;
    if gPaletteFade.active() == 0 {
        let inputOptionId: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
        ListMenuGetScrollAndRow(
            *data.at(5) as u8,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
            &raw mut gPlayerPCItemPageInfo.cursorPos,
        );
        match inputOptionId {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                RemoveScrollIndicatorArrowPair(gPlayerPCItemPageInfo.scrollIndicatorTaskId);
                Mailbox_ReturnToPlayerPC(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                MailboxMenu_RemoveWindow(MAILBOXWIN_TITLE);
                MailboxMenu_RemoveWindow(MAILBOXWIN_LIST);
                DestroyListMenuTask(
                    *data.at(5) as u8,
                    &raw mut gPlayerPCItemPageInfo.itemsAbove,
                    &raw mut gPlayerPCItemPageInfo.cursorPos,
                );
                ScheduleBgCopyTilemapToVram(0);
                RemoveScrollIndicatorArrowPair(gPlayerPCItemPageInfo.scrollIndicatorTaskId);
                task_set_func(taskId, Some(Mailbox_PrintWhatToDoWithPlayerMailText));
            }
        }
    }
}
pub(crate) unsafe fn Mailbox_PrintWhatToDoWithPlayerMailText(taskId: u8) {
    StringCopy(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock1Ptr).mail[gPlayerPCItemPageInfo.itemsAbove as i32
            + PARTY_SIZE
            + gPlayerPCItemPageInfo.cursorPos as i32]
            .playerName
            .as_mut_ptr(),
    );
    ConvertInternationalPlayerNameStripChar(gStringVar1.as_mut_ptr(), CHAR_SPACE);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_WhatToDoWithVar1sMail).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(Mailbox_PrintMailOptions),
    );
}
unsafe fn Mailbox_ReturnToPlayerPC(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    MailboxMenu_RemoveWindow(MAILBOXWIN_TITLE);
    MailboxMenu_RemoveWindow(MAILBOXWIN_LIST);
    DestroyListMenuTask(*data.at(5) as u8, null_mut(), null_mut());
    ScheduleBgCopyTilemapToVram(0);
    MailboxMenu_Free();
    ReshowPlayerPC(taskId);
}
pub(crate) unsafe fn Mailbox_PrintMailOptions(taskId: u8) {
    let windowId: u8 = MailboxMenu_AddWindow(MAILBOXWIN_OPTIONS);
    PrintMenuTable(windowId, 4, gMailboxMailOptions.as_ptr().cast_mut());
    InitMenuInUpperLeftCornerNormal(windowId, 4, 0);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(Mailbox_MailOptionsProcessInput));
}
pub(crate) unsafe fn Mailbox_MailOptionsProcessInput(taskId: u8) {
    let inputOptionId: i8 = ProcessMenuInput_other();
    match inputOptionId {
        MENU_NOTHING_CHOSEN => {}
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            Mailbox_Cancel(taskId);
        }
        _ => {
            PlaySE(SE_SELECT);
            gMailboxMailOptions[inputOptionId]
                .func
                .void_u8
                .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe fn Mailbox_DoMailRead(taskId: u8) {
    FadeScreen(FADE_TO_BLACK, 0);
    task_set_func(taskId, Some(Mailbox_FadeAndReadMail));
}
pub(crate) unsafe fn Mailbox_FadeAndReadMail(taskId: u8) {
    if gPaletteFade.active() == 0 {
        MailboxMenu_Free();
        CleanupOverworldWindowsAndTilemaps();
        ReadMail(
            &raw mut (*gSaveBlock1Ptr).mail[gPlayerPCItemPageInfo.itemsAbove as i32
                + PARTY_SIZE
                + gPlayerPCItemPageInfo.cursorPos as i32],
            Some(Mailbox_ReturnToFieldFromReadMail),
            TRUE,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Mailbox_ReturnToFieldFromReadMail() {
    gFieldCallback = Some(Mailbox_ReshowAfterMail);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn Mailbox_ReshowAfterMail() {
    LoadMessageBoxAndBorderGfx();
    let taskId: u8 = CreateTask(Some(Mailbox_HandleReturnToProcessInput), 0);
    if MailboxMenu_Alloc(gPlayerPCItemPageInfo.count) == TRUE {
        Mailbox_DrawMailboxMenu(taskId);
    } else {
        DestroyTask(taskId);
    }
    FadeInFromBlack();
}
pub(crate) unsafe fn Mailbox_HandleReturnToProcessInput(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        task_set_func(taskId, Some(Mailbox_ProcessInput));
    }
}
pub(crate) unsafe fn Mailbox_MoveToBag(taskId: u8) {
    DisplayItemMessageOnField(
        taskId,
        (*(&raw const crate::data::strings::gText_MessageWillBeLost).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(Mailbox_AskConfirmMoveToBag),
    );
}
pub(crate) unsafe fn Mailbox_AskConfirmMoveToBag(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    task_set_func(taskId, Some(Mailbox_HandleConfirmMoveToBag));
}
pub(crate) unsafe fn Mailbox_HandleConfirmMoveToBag(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let matched = sw1 == 0 || sw1 == MENU_B_PRESSED || sw1 == 1 || sw1 == MENU_NOTHING_CHOSEN;
        let mut fall = false;
        if sw1 == 0 {
            Mailbox_DoMailMoveToBag(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            Mailbox_CancelMoveToBag(taskId);
            break 'l1;
        }
        if sw1 == MENU_NOTHING_CHOSEN || !matched {
            break 'l1;
        }
    }
}
unsafe fn Mailbox_DoMailMoveToBag(taskId: u8) {
    let mail: *mut Mail = &raw mut (*gSaveBlock1Ptr).mail[gPlayerPCItemPageInfo.itemsAbove as i32
        + PARTY_SIZE
        + gPlayerPCItemPageInfo.cursorPos as i32];
    if AddBagItem((*mail).itemId, 1) == 0 {
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_BagIsFull).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Mailbox_Cancel),
        );
    } else {
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_MailToBagMessageErased)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            Some(Mailbox_Cancel),
        );
        ClearMail(mail);
        Mailbox_CompactMailList();
        gPlayerPCItemPageInfo.count -= 1;
        if (gPlayerPCItemPageInfo.count as i32)
            < gPlayerPCItemPageInfo.pageItems as i32 + gPlayerPCItemPageInfo.itemsAbove as i32
            && gPlayerPCItemPageInfo.itemsAbove != 0
        {
            gPlayerPCItemPageInfo.itemsAbove -= 1;
        }
        SetPlayerPCListCount(taskId);
    }
}
unsafe fn Mailbox_CancelMoveToBag(taskId: u8) {
    Mailbox_Cancel(taskId);
}
pub(crate) unsafe fn Mailbox_Give(taskId: u8) {
    if CalculatePlayerPartyCount() == 0 {
        Mailbox_NoPokemonForMail(taskId);
    } else {
        FadeScreen(FADE_TO_BLACK, 0);
        task_set_func(taskId, Some(Mailbox_DoGiveMailPokeMenu));
    }
}
pub(crate) unsafe fn Mailbox_DoGiveMailPokeMenu(taskId: u8) {
    if gPaletteFade.active() == 0 {
        MailboxMenu_Free();
        CleanupOverworldWindowsAndTilemaps();
        ChooseMonToGiveMailFromMailbox();
        DestroyTask(taskId);
    }
}
pub unsafe fn Mailbox_ReturnToMailListAfterDeposit() {
    gFieldCallback = Some(Mailbox_UpdateMailListAfterDeposit);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn Mailbox_UpdateMailListAfterDeposit() {
    let taskId: u8 = CreateTask(Some(Mailbox_HandleReturnToProcessInput), 0);
    let prevCount: u8 = gPlayerPCItemPageInfo.count;
    gPlayerPCItemPageInfo.count = GetMailboxMailCount();
    Mailbox_CompactMailList();
    if prevCount != gPlayerPCItemPageInfo.count
        && (gPlayerPCItemPageInfo.count as i32)
            < gPlayerPCItemPageInfo.pageItems as i32 + gPlayerPCItemPageInfo.itemsAbove as i32
        && gPlayerPCItemPageInfo.itemsAbove != 0
    {
        gPlayerPCItemPageInfo.itemsAbove -= 1;
    }
    SetPlayerPCListCount(taskId);
    LoadMessageBoxAndBorderGfx();
    if MailboxMenu_Alloc(gPlayerPCItemPageInfo.count) == TRUE {
        Mailbox_DrawMailboxMenu(taskId);
    } else {
        DestroyTask(taskId);
    }
    FadeInFromBlack();
}
unsafe fn Mailbox_NoPokemonForMail(taskId: u8) {
    DisplayItemMessageOnField(
        taskId,
        (*(&raw const crate::data::strings::gText_NoPokemon).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(Mailbox_Cancel),
    );
}
pub(crate) unsafe fn Mailbox_Cancel(taskId: u8) {
    MailboxMenu_RemoveWindow(MAILBOXWIN_OPTIONS);
    ClearDialogWindowAndFrame(0, 0);
    Mailbox_DrawMailboxMenu(taskId);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(Mailbox_ProcessInput));
}
unsafe fn ItemStorage_Init() {
    sItemStorageMenu = AllocZeroed(1648) as *mut ItemStorageMenu;
    memset(
        (*sItemStorageMenu).windowIds.as_mut_ptr(),
        WINDOW_NONE as i32,
        ITEMPC_WIN_COUNT,
    );
    (*sItemStorageMenu).toSwapPos = NOT_SWAPPING;
    (*sItemStorageMenu).spriteId = SPRITE_NONE;
}
unsafe fn ItemStorage_Free() {
    for i in 0..ITEMPC_WIN_COUNT {
        ItemStorage_RemoveWindow(i as u8);
    }
    Free(sItemStorageMenu as *mut c_void);
}
unsafe fn ItemStorage_AddWindow(i: u8) -> u8 {
    let windowIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).windowIds[i];
    if *windowIdLoc == WINDOW_NONE {
        *windowIdLoc = AddWindow((&raw const sWindowTemplates_ItemStorage[i]).cast_mut()) as u8;
        DrawStdFrameWithCustomTileAndPalette(*windowIdLoc, FALSE, 0x214, 0xE);
        ScheduleBgCopyTilemapToVram(0);
    }
    *windowIdLoc
}
unsafe fn ItemStorage_RemoveWindow(i: u8) {
    let windowIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).windowIds[i];
    if *windowIdLoc != WINDOW_NONE {
        ClearStdWindowAndFrameToTransparent(*windowIdLoc, FALSE);
        ClearWindowTilemap(*windowIdLoc);
        ScheduleBgCopyTilemapToVram(0);
        RemoveWindow(*windowIdLoc);
        *windowIdLoc = WINDOW_NONE;
    }
}
pub unsafe fn ItemStorage_RefreshListMenu() {
    let mut i: u16 = 0;
    while (i as i32) < gPlayerPCItemPageInfo.count as i32 - 1 {
        CopyItemName_PlayerPC(
            &raw mut (*sItemStorageMenu).itemNames[i][0],
            (*gSaveBlock1Ptr).pcItems[i].itemId,
        );
        (*sItemStorageMenu).listItems[i].name = &raw mut (*sItemStorageMenu).itemNames[i][0];
        (*sItemStorageMenu).listItems[i].id = i as i32;
        i += 1;
    }
    StringCopy(
        &raw mut (*sItemStorageMenu).itemNames[i][0],
        (*(&raw const crate::data::strings::gText_Cancel2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*sItemStorageMenu).listItems[i].name = &raw mut (*sItemStorageMenu).itemNames[i][0];
    (*sItemStorageMenu).listItems[i].id = LIST_CANCEL;
    gMultiuseListMenuTemplate = *sListMenuTemplate_ItemStorage;
    gMultiuseListMenuTemplate.windowId = ItemStorage_AddWindow(ITEMPC_WIN_LIST);
    gMultiuseListMenuTemplate.totalItems = gPlayerPCItemPageInfo.count as u16;
    gMultiuseListMenuTemplate.items = (*sItemStorageMenu).listItems.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = gPlayerPCItemPageInfo.pageItems as u16;
}
pub unsafe fn CopyItemName_PlayerPC(string: *mut u8, itemId: u16) {
    CopyItemName(itemId, string);
}
pub(crate) unsafe fn ItemStorage_MoveCursor(id: i32, onInit: u8, list: *mut ListMenu) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
    }
    if (*sItemStorageMenu).toSwapPos == NOT_SWAPPING {
        ItemStorage_EraseItemIcon();
        if id != LIST_CANCEL {
            ItemStorage_DrawItemIcon((*gSaveBlock1Ptr).pcItems[id].itemId);
        } else {
            ItemStorage_DrawItemIcon(ITEM_LIST_END);
        }
        ItemStorage_PrintDescription(id);
    }
}
pub(crate) unsafe fn ItemStorage_PrintMenuItem(windowId: u8, id: u32, yOffset: u8) {
    if id != LIST_CANCEL as u32 {
        if (*sItemStorageMenu).toSwapPos != NOT_SWAPPING {
            if (*sItemStorageMenu).toSwapPos == id as u8 {
                ItemStorage_DrawSwapArrow(yOffset, 0, TEXT_SKIP_DRAW);
            } else {
                ItemStorage_DrawSwapArrow(yOffset, 0xFF, 0xFF);
            }
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*gSaveBlock1Ptr).pcItems[id].quantity as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            3,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterParameterized(
            windowId,
            FONT_NARROW,
            gStringVar4.as_mut_ptr(),
            GetStringRightAlignXOffset(FONT_NARROW as i32, gStringVar4.as_mut_ptr(), 104) as u8,
            yOffset,
            TEXT_SKIP_DRAW,
            None,
        );
    }
}
unsafe fn ItemStorage_PrintDescription(id: i32) {
    let mut description: *mut u8 = null_mut();
    let windowId: u8 = (*sItemStorageMenu).windowIds[1];
    if id != LIST_CANCEL {
        description = GetItemDescription((*gSaveBlock1Ptr).pcItems[id].itemId);
    } else {
        description = ItemStorage_GetMessage(MSG_GO_BACK_TO_PREV);
    }
    FillWindowPixelBuffer(windowId, 17);
    AddTextPrinterParameterized(windowId, FONT_NORMAL, description, 0, 1, 0, None);
}
unsafe fn ItemStorage_AddScrollIndicator() {
    if gPlayerPCItemPageInfo.scrollIndicatorTaskId == TASK_NONE {
        gPlayerPCItemPageInfo.scrollIndicatorTaskId = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            176,
            12,
            148,
            gPlayerPCItemPageInfo.count as i32 - gPlayerPCItemPageInfo.pageItems as i32,
            TAG_SCROLL_ARROW,
            TAG_SCROLL_ARROW,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
        );
    }
}
unsafe fn ItemStorage_RemoveScrollIndicator() {
    if gPlayerPCItemPageInfo.scrollIndicatorTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair(gPlayerPCItemPageInfo.scrollIndicatorTaskId);
        gPlayerPCItemPageInfo.scrollIndicatorTaskId = TASK_NONE;
    }
}
unsafe fn ItemStorage_SetSwapArrow(listTaskId: u8, b: u8, speed: u8) {
    ItemStorage_DrawSwapArrow(
        ListMenuGetYCoordForPrintingArrowCursor(listTaskId) as u8,
        b,
        speed,
    );
}
unsafe fn ItemStorage_DrawSwapArrow(y: u8, b: u8, speed: u8) {
    let windowId: u8 = (*sItemStorageMenu).windowIds[0];
    if b == 0xFF {
        FillWindowPixelRect(
            windowId,
            17,
            0,
            y as u16,
            GetMenuCursorDimensionByFont(FONT_NORMAL, 0) as u16,
            GetMenuCursorDimensionByFont(FONT_NORMAL, 1) as u16,
        );
    } else {
        AddTextPrinterParameterized4(
            windowId,
            FONT_NORMAL,
            0,
            y,
            0,
            0,
            sSwapArrowTextColors.as_ptr().cast_mut(),
            speed as i8,
            (*(&raw const crate::data::strings::gText_SelectorArrow2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
unsafe fn ItemStorage_DrawItemIcon(itemId: u16) {
    let mut spriteId: u8 = 0;
    let spriteIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).spriteId;
    if *spriteIdLoc == SPRITE_NONE {
        FreeSpriteTilesByTag(TAG_ITEM_ICON);
        FreeSpritePaletteByTag(TAG_ITEM_ICON);
        spriteId = AddItemIconSprite(TAG_ITEM_ICON, TAG_ITEM_ICON, itemId);
        if spriteId != MAX_SPRITES {
            *spriteIdLoc = spriteId;
            gSprites[spriteId].oam.set_priority(0);
            gSprites[spriteId].x2 = 24;
            gSprites[spriteId].y2 = 80;
        }
    }
}
unsafe fn ItemStorage_EraseItemIcon() {
    let spriteIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).spriteId;
    if *spriteIdLoc != SPRITE_NONE {
        FreeSpriteTilesByTag(TAG_ITEM_ICON);
        FreeSpritePaletteByTag(TAG_ITEM_ICON);
        DestroySprite(&raw mut gSprites[*spriteIdLoc]);
        *spriteIdLoc = SPRITE_NONE;
    }
}
unsafe fn ItemStorage_CompactList() {
    CompactPCItems();
    SetItemListPerPageCount(
        (*gSaveBlock1Ptr).pcItems.as_mut_ptr(),
        PC_ITEMS_COUNT,
        &raw mut gPlayerPCItemPageInfo.pageItems,
        &raw mut gPlayerPCItemPageInfo.count,
        8,
    );
}
unsafe fn ItemStorage_CompactCursor() {
    SetCursorWithinListBounds(
        &raw mut gPlayerPCItemPageInfo.itemsAbove,
        &raw mut gPlayerPCItemPageInfo.cursorPos,
        gPlayerPCItemPageInfo.pageItems,
        gPlayerPCItemPageInfo.count,
    );
}
pub(crate) unsafe fn ItemStorage_CreateListMenu(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut i: u32 = 0;
    while i <= ITEMPC_WIN_TITLE {
        ItemStorage_AddWindow(i as u8);
        i += 1;
    }
    let toss: u32 = *data.at(3) as u32;
    let mut text: *mut u8 = (*(&raw const crate::data::strings::gText_TossItem)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    if toss == 0 {
        text = (*(&raw const crate::data::strings::gText_WithdrawItem).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    let x: u32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 104) as u32;
    AddTextPrinterParameterized(
        (*sItemStorageMenu).windowIds[3],
        FONT_NORMAL,
        text,
        x as u8,
        1,
        0,
        None,
    );
    CopyWindowToVram((*sItemStorageMenu).windowIds[2], COPYWIN_GFX);
    ItemStorage_CompactList();
    ItemStorage_CompactCursor();
    ItemStorage_RefreshListMenu();
    *data.at(5) = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        gPlayerPCItemPageInfo.itemsAbove,
        gPlayerPCItemPageInfo.cursorPos,
    ) as i16;
    ItemStorage_AddScrollIndicator();
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(ItemStorage_ProcessInput));
}
unsafe fn ItemStorage_GetMessage(itemId: u16) -> *mut u8 {
    let mut string: *mut u8 = null_mut();
    match itemId {
        MSG_GO_BACK_TO_PREV => {
            string = (*(&raw const crate::data::strings::gText_GoBackPrevMenu)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_HOW_MANY_TO_WITHDRAW => {
            string = (*(&raw const crate::data::strings::gText_WithdrawHowManyItems)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_WITHDREW_ITEM => {
            string = (*(&raw const crate::data::strings::gText_WithdrawXItems)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_HOW_MANY_TO_TOSS => {
            string = (*(&raw const crate::data::strings::gText_TossHowManyVar1s)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_THREW_AWAY_ITEM => {
            string = (*(&raw const crate::data::strings::gText_ThrewAwayVar2Var1s)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_NO_MORE_ROOM => {
            string = (*(&raw const crate::data::strings::gText_NoRoomInBag)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_TOO_IMPORTANT => {
            string = (*(&raw const crate::data::strings::gText_TooImportantToToss)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_OKAY_TO_THROW_AWAY => {
            string = (*(&raw const crate::data::strings::gText_ConfirmTossItems)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        MSG_SWITCH_WHICH_ITEM => {
            string = (*(&raw const crate::data::strings::gText_MoveVar1Where)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        _ => {
            string = GetItemDescription(itemId);
        }
    }
    string
}
unsafe fn ItemStorage_PrintMessage(string: *mut u8) {
    let windowId: u8 = (*sItemStorageMenu).windowIds[1];
    FillWindowPixelBuffer(windowId, 17);
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), string);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        0,
        None,
    );
}
pub(crate) unsafe fn ItemStorage_ProcessInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        ListMenuGetScrollAndRow(
            *data.at(5) as u8,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
            &raw mut gPlayerPCItemPageInfo.cursorPos,
        );
        if gPlayerPCItemPageInfo.itemsAbove as i32 + gPlayerPCItemPageInfo.cursorPos as i32
            != gPlayerPCItemPageInfo.count as i32 - 1
        {
            PlaySE(SE_SELECT);
            ItemStorage_StartItemSwap(taskId);
        }
    } else {
        let id: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
        ListMenuGetScrollAndRow(
            *data.at(5) as u8,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
            &raw mut gPlayerPCItemPageInfo.cursorPos,
        );
        match id {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                ItemStorage_ExitItemList(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                ItemStorage_DoItemAction(taskId);
            }
        }
    }
}
pub(crate) unsafe fn ItemStorage_ReturnToMenuSelect(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        DrawDialogueFrame(0, 0);
        if *data.at(3) == 0 {
            InitItemStorageMenu(taskId, MENU_WITHDRAW);
        } else {
            InitItemStorageMenu(taskId, MENU_TOSS);
        }
        task_set_func(taskId, Some(ItemStorageMenuProcessInput));
    }
}
unsafe fn ItemStorage_ExitItemList(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ItemStorage_EraseItemIcon();
    ItemStorage_RemoveScrollIndicator();
    DestroyListMenuTask(*data.at(5) as u8, null_mut(), null_mut());
    DestroySwapLineSprites(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
    );
    ItemStorage_Free();
    task_set_func(taskId, Some(ItemStorage_ReturnToMenuSelect));
}
unsafe fn ItemStorage_StartItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ListMenuSetTemplateField(*data.at(5) as u8, LISTFIELD_CURSORKIND, CURSOR_INVISIBLE);
    (*sItemStorageMenu).toSwapPos =
        gPlayerPCItemPageInfo.itemsAbove as u8 + gPlayerPCItemPageInfo.cursorPos as u8;
    ItemStorage_SetSwapArrow(*data.at(5) as u8, 0, 0);
    ItemStorage_UpdateSwapLinePos((*sItemStorageMenu).toSwapPos);
    CopyItemName(
        (*gSaveBlock1Ptr).pcItems[(*sItemStorageMenu).toSwapPos].itemId,
        gStringVar1.as_mut_ptr(),
    );
    ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_SWITCH_WHICH_ITEM));
    task_set_func(taskId, Some(ItemStorage_ProcessItemSwapInput));
}
pub(crate) unsafe fn ItemStorage_ProcessItemSwapInput(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        ListMenuGetScrollAndRow(
            *data.at(5) as u8,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
            &raw mut gPlayerPCItemPageInfo.cursorPos,
        );
        ItemStorage_FinishItemSwap(taskId, FALSE);
        return;
    }
    let id: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
    ListMenuGetScrollAndRow(
        *data.at(5) as u8,
        &raw mut gPlayerPCItemPageInfo.itemsAbove,
        &raw mut gPlayerPCItemPageInfo.cursorPos,
    );
    SetSwapLineSpritesInvisibility(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
        FALSE,
    );
    ItemStorage_UpdateSwapLinePos(gPlayerPCItemPageInfo.cursorPos as u8);
    match id {
        LIST_NOTHING_CHOSEN => {}
        LIST_CANCEL => {
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                ItemStorage_FinishItemSwap(taskId, FALSE);
            } else {
                ItemStorage_FinishItemSwap(taskId, TRUE);
            }
        }
        _ => {
            ItemStorage_FinishItemSwap(taskId, FALSE);
        }
    }
}
unsafe fn ItemStorage_FinishItemSwap(taskId: u8, canceled: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let newPos: u16 = gPlayerPCItemPageInfo.itemsAbove + gPlayerPCItemPageInfo.cursorPos;
    PlaySE(SE_SELECT);
    DestroyListMenuTask(
        *data.at(5) as u8,
        &raw mut gPlayerPCItemPageInfo.itemsAbove,
        &raw mut gPlayerPCItemPageInfo.cursorPos,
    );
    if canceled == 0
        && (*sItemStorageMenu).toSwapPos as u16 != newPos
        && (*sItemStorageMenu).toSwapPos as i32 != newPos as i32 - 1
    {
        MoveItemSlotInList(
            (*gSaveBlock1Ptr).pcItems.as_mut_ptr(),
            (*sItemStorageMenu).toSwapPos as u32,
            newPos as u32,
        );
        ItemStorage_RefreshListMenu();
    }
    if ((*sItemStorageMenu).toSwapPos as u16) < newPos {
        gPlayerPCItemPageInfo.cursorPos -= 1;
    }
    SetSwapLineSpritesInvisibility(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
        TRUE,
    );
    (*sItemStorageMenu).toSwapPos = NOT_SWAPPING;
    *data.at(5) = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        gPlayerPCItemPageInfo.itemsAbove,
        gPlayerPCItemPageInfo.cursorPos,
    ) as i16;
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(ItemStorage_ProcessInput));
}
unsafe fn ItemStorage_UpdateSwapLinePos(y: u8) {
    UpdateSwapLineSpritesPos(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
        128,
        (y as u16 + 1) * 16,
    );
}
unsafe fn ItemStorage_PrintItemQuantity(windowId: u8, value: u16, mode: u32, x: u8, y: u8, n: u8) {
    ConvertIntToDecimalStringN(gStringVar1.as_mut_ptr(), value as i32, mode as i32, n);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 48) as u8,
        y,
        0,
        None,
    );
}
unsafe fn ItemStorage_DoItemAction(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
    ItemStorage_RemoveScrollIndicator();
    *data.at(2) = 1;
    if *data.at(3) == 0 {
        if (*gSaveBlock1Ptr).pcItems[pos].quantity == 1 {
            ItemStorage_DoItemWithdraw(taskId);
            return;
        }
        CopyItemName(
            (*gSaveBlock1Ptr).pcItems[pos].itemId,
            gStringVar1.as_mut_ptr(),
        );
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_HOW_MANY_TO_WITHDRAW));
    } else {
        if (*gSaveBlock1Ptr).pcItems[pos].quantity == 1 {
            ItemStorage_DoItemToss(taskId);
            return;
        }
        CopyItemName(
            (*gSaveBlock1Ptr).pcItems[pos].itemId,
            gStringVar1.as_mut_ptr(),
        );
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_HOW_MANY_TO_TOSS));
    }
    ItemStorage_PrintItemQuantity(
        ItemStorage_AddWindow(ITEMPC_WIN_QUANTITY),
        *data.at(2) as u16,
        STR_CONV_MODE_LEADING_ZEROS as u32,
        8,
        1,
        3,
    );
    task_set_func(taskId, Some(ItemStorage_HandleQuantityRolling));
}
pub(crate) unsafe fn ItemStorage_HandleQuantityRolling(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
    if AdjustQuantityAccordingToDPadInput(data.at(2), (*gSaveBlock1Ptr).pcItems[pos].quantity)
        == TRUE
    {
        ItemStorage_PrintItemQuantity(
            ItemStorage_AddWindow(ITEMPC_WIN_QUANTITY),
            *data.at(2) as u16,
            STR_CONV_MODE_LEADING_ZEROS as u32,
            8,
            1,
            3,
        );
    } else {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            PlaySE(SE_SELECT);
            ItemStorage_RemoveWindow(ITEMPC_WIN_QUANTITY);
            if *data.at(3) == 0 {
                ItemStorage_DoItemWithdraw(taskId);
            } else {
                ItemStorage_DoItemToss(taskId);
            }
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            ItemStorage_RemoveWindow(ITEMPC_WIN_QUANTITY);
            ItemStorage_PrintMessage(ItemStorage_GetMessage(
                (*gSaveBlock1Ptr).pcItems[pos].itemId,
            ));
            ItemStorage_ReturnToListInput(taskId);
        }
    }
}
unsafe fn ItemStorage_DoItemWithdraw(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
    if AddBagItem((*gSaveBlock1Ptr).pcItems[pos].itemId, *data.at(2) as u16) == TRUE {
        CopyItemName(
            (*gSaveBlock1Ptr).pcItems[pos].itemId,
            gStringVar1.as_mut_ptr(),
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            *data.at(2) as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_WITHDREW_ITEM));
        task_set_func(taskId, Some(ItemStorage_HandleRemoveItem));
    } else {
        *data.at(2) = 0;
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_NO_MORE_ROOM));
        task_set_func(taskId, Some(ItemStorage_HandleErrorMessageInput));
    }
}
unsafe fn ItemStorage_DoItemToss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
    if GetItemImportance((*gSaveBlock1Ptr).pcItems[pos].itemId) == 0 {
        CopyItemName(
            (*gSaveBlock1Ptr).pcItems[pos].itemId,
            gStringVar1.as_mut_ptr(),
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            *data.at(2) as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_OKAY_TO_THROW_AWAY));
        CreateYesNoMenuWithCallbacks(
            taskId,
            (&raw const sWindowTemplates_ItemStorage[5]).cast_mut(),
            1,
            0,
            1,
            0x214,
            0xE,
            (&raw const *ItemTossYesNoFuncs).cast_mut(),
        );
    } else {
        *data.at(2) = 0;
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_TOO_IMPORTANT));
        task_set_func(taskId, Some(ItemStorage_HandleErrorMessageInput));
    }
}
pub(crate) unsafe fn ItemStorage_TossItemYes(taskId: u8) {
    ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_THREW_AWAY_ITEM));
    task_set_func(taskId, Some(ItemStorage_HandleRemoveItem));
}
pub(crate) unsafe fn ItemStorage_TossItemNo(taskId: u8) {
    ItemStorage_PrintMessage(ItemStorage_GetMessage(
        (*gSaveBlock1Ptr).pcItems
            [gPlayerPCItemPageInfo.itemsAbove as i32 + gPlayerPCItemPageInfo.cursorPos as i32]
            .itemId,
    ));
    ItemStorage_ReturnToListInput(taskId);
}
pub(crate) unsafe fn ItemStorage_HandleRemoveItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & 3 != 0 {
        RemovePCItem(
            gPlayerPCItemPageInfo.cursorPos as u8 + gPlayerPCItemPageInfo.itemsAbove as u8,
            *data.at(2) as u16,
        );
        DestroyListMenuTask(
            *data.at(5) as u8,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
            &raw mut gPlayerPCItemPageInfo.cursorPos,
        );
        ItemStorage_CompactList();
        ItemStorage_CompactCursor();
        ItemStorage_RefreshListMenu();
        *data.at(5) = ListMenuInit(
            &raw mut gMultiuseListMenuTemplate,
            gPlayerPCItemPageInfo.itemsAbove,
            gPlayerPCItemPageInfo.cursorPos,
        ) as i16;
        ItemStorage_ReturnToListInput(taskId);
    }
}
pub(crate) unsafe fn ItemStorage_HandleErrorMessageInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & 3 != 0 {
        ItemStorage_PrintMessage(ItemStorage_GetMessage(
            (*gSaveBlock1Ptr).pcItems
                [gPlayerPCItemPageInfo.itemsAbove as i32 + gPlayerPCItemPageInfo.cursorPos as i32]
                .itemId,
        ));
        ItemStorage_ReturnToListInput(taskId);
    }
}
unsafe fn ItemStorage_ReturnToListInput(taskId: u8) {
    ItemStorage_AddScrollIndicator();
    task_set_func(taskId, Some(ItemStorage_ProcessInput));
}
