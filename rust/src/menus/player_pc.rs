//! Translated from `src/player_pc.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
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
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
pub(crate) static mut sTopMenuNumOptions: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerPCItemPageInfo: PlayerPCItemPageStruct = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemStorageMenu: *mut ItemStorageMenu = null_mut();

unsafe extern "C" {
    static LittlerootTown_BrendansHouse_2F_EventScript_TurnOffPlayerPC: CArray<u8, 0>;
    static LittlerootTown_MaysHouse_2F_EventScript_TurnOffPlayerPC: CArray<u8, 0>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gMain: Main;
    static mut gMultiuseListMenuTemplate: ListMenuTemplate;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BagIsFull: CArray<u8, 0>;
    static gText_Cancel2: CArray<u8, 0>;
    static gText_ConfirmTossItems: CArray<u8, 0>;
    static gText_GoBackPrevMenu: CArray<u8, 0>;
    static gText_MailToBagMessageErased: CArray<u8, 0>;
    static gText_Mailbox: CArray<u8, 0>;
    static gText_MessageWillBeLost: CArray<u8, 0>;
    static gText_MoveVar1Where: CArray<u8, 0>;
    static gText_NoItems: CArray<u8, 0>;
    static gText_NoMailHere: CArray<u8, 0>;
    static gText_NoPokemon: CArray<u8, 0>;
    static gText_NoRoomInBag: CArray<u8, 0>;
    static gText_SelectorArrow2: CArray<u8, 0>;
    static gText_ThrewAwayVar2Var1s: CArray<u8, 0>;
    static gText_TooImportantToToss: CArray<u8, 0>;
    static gText_TossHowManyVar1s: CArray<u8, 0>;
    static gText_TossItem: CArray<u8, 0>;
    static gText_WhatToDoWithVar1sMail: CArray<u8, 0>;
    static gText_WhatWouldYouLike: CArray<u8, 0>;
    static gText_WithdrawHowManyItems: CArray<u8, 0>;
    static gText_WithdrawItem: CArray<u8, 0>;
    static gText_WithdrawXItems: CArray<u8, 0>;
    static gText_xVar1: CArray<u8, 0>;
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
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AdjustQuantityAccordingToDPadInput(a0: *mut i16, a1: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn CB2_GoToItemDepositMenu();
    fn CB2_ReturnToField();
    fn CalculatePlayerPartyCount() -> u8;
    fn ChooseMonToGiveMailFromMailbox();
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearItemSlots(a0: *mut ItemSlot, a1: u8);
    fn ClearMail(a0: *mut Mail);
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
        a1: *mut WindowTemplate,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u16,
        a6: u8,
        a7: *mut YesNoFuncTable,
    );
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySprite(a0: *mut Sprite);
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
    fn Free(a0: *mut c_void);
    fn FreeAndReserveObjectSpritePalettes();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetItemDescription(a0: u16) -> *mut u8;
    fn GetItemImportance(a0: u16) -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32;
    fn GetMaxWidthInSubsetOfMenuTable(a0: *mut MenuAction, a1: *mut u8, a2: i32) -> i32;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuGetYCoordForPrintingArrowCursor(a0: u8) -> u16;
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenuSetTemplateField(a0: u8, a1: u8, a2: i32);
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadListMenuSwapLineGfx();
    fn LoadMessageBoxAndBorderGfx();
    fn MailboxMenu_AddScrollArrows(a0: *mut PlayerPCItemPageStruct);
    fn MailboxMenu_AddWindow(a0: u8) -> u8;
    fn MailboxMenu_Alloc(a0: u8) -> u8;
    fn MailboxMenu_CreateList(a0: *mut PlayerPCItemPageStruct) -> u8;
    fn MailboxMenu_Free();
    fn MailboxMenu_RemoveWindow(a0: u8);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn MoveItemSlotInList(a0: *mut ItemSlot, a1: u32, a2: u32);
    fn PlaySE(a0: u16);
    fn PrintMenuActionTextsInUpperLeftCorner(a0: u8, a1: u8, a2: *mut MenuAction, a3: *mut u8);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
    fn ProcessMenuInput_other() -> i8;
    fn ReadMail(a0: *mut Mail, a1: Option<unsafe extern "C" fn()>, a2: u8);
    fn RemovePCItem(a0: u8, a1: u16);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetCursorWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8);
    fn SetItemListPerPageCount(a0: *mut ItemSlot, a1: u8, a2: *mut u8, a3: *mut u8, a4: u8);
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
pub unsafe extern "C" fn BedroomPC() {
    sTopMenuOptionOrder = sBedroomPC_OptionOrder.as_ptr().cast_mut();
    sTopMenuNumOptions = 4;
    DisplayItemMessageOnField(
        CreateTask(Some(TaskDummy), 0),
        gText_WhatWouldYouLike.as_ptr().cast_mut(),
        Some(InitPlayerPCMenu),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerPC() {
    sTopMenuOptionOrder = sPlayerPC_OptionOrder.as_ptr().cast_mut();
    sTopMenuNumOptions = 3;
    DisplayItemMessageOnField(
        CreateTask(Some(TaskDummy), 0),
        gText_WhatWouldYouLike.as_ptr().cast_mut(),
        Some(InitPlayerPCMenu),
    );
}
pub(crate) unsafe extern "C" fn InitPlayerPCMenu(taskId: u8) {
    let mut data: *mut u16 = null_mut();
    let mut windowTemplate: WindowTemplate = zeroed();
    data = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    if sTopMenuNumOptions == 3 {
        windowTemplate = sWindowTemplates_MainMenus[0];
    } else {
        windowTemplate = sWindowTemplates_MainMenus[1];
    }
    windowTemplate.width = GetMaxWidthInSubsetOfMenuTable(
        sPlayerPCMenuActions.as_ptr().cast_mut(),
        sTopMenuOptionOrder,
        sTopMenuNumOptions as i32,
    ) as u8;
    *data.at(4) = AddWindow(&raw mut windowTemplate);
    SetStandardWindowBorderStyle(*data.at(4) as u8, FALSE);
    PrintMenuActionTextsInUpperLeftCorner(
        *data.at(4) as u8,
        sTopMenuNumOptions,
        sPlayerPCMenuActions.as_ptr().cast_mut(),
        sTopMenuOptionOrder,
    );
    InitMenuInUpperLeftCornerNormal(*data.at(4) as u8, sTopMenuNumOptions, 0);
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].func = Some(PlayerPCProcessMenuInput);
}
pub(crate) unsafe extern "C" fn PlayerPCProcessMenuInput(taskId: u8) {
    let mut data: *mut u16 = null_mut();
    let mut inputOptionId: i8 = 0;
    data = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    if sTopMenuNumOptions > 3 {
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
            gTasks[taskId].func = Some(PlayerPC_TurnOff);
        }
        _ => {
            ClearStdWindowAndFrameToTransparent(*data.at(4) as u8, FALSE);
            ClearWindowTilemap(*data.at(4) as u8);
            RemoveWindow(*data.at(4) as u8);
            ScheduleBgCopyTilemapToVram(0);
            gTasks[taskId].func = sPlayerPCMenuActions[*sTopMenuOptionOrder.at(inputOptionId)]
                .func
                .void_u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReshowPlayerPC(var: u8) {
    DisplayItemMessageOnField(
        var,
        gText_WhatWouldYouLike.as_ptr().cast_mut(),
        Some(InitPlayerPCMenu),
    );
}
pub(crate) unsafe extern "C" fn PlayerPC_ItemStorage(taskId: u8) {
    InitItemStorageMenu(taskId, MENU_WITHDRAW);
    gTasks[taskId].func = Some(ItemStorageMenuProcessInput);
}
pub(crate) unsafe extern "C" fn PlayerPC_Mailbox(taskId: u8) {
    gPlayerPCItemPageInfo.count = GetMailboxMailCount();
    if gPlayerPCItemPageInfo.count == 0 {
        DisplayItemMessageOnField(
            taskId,
            gText_NoMailHere.as_ptr().cast_mut(),
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
            gTasks[taskId].func = Some(Mailbox_ProcessInput);
        } else {
            DisplayItemMessageOnField(
                taskId,
                gText_NoMailHere.as_ptr().cast_mut(),
                Some(ReshowPlayerPC),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerPC_Decoration(taskId: u8) {
    DoPlayerRoomDecorationMenu(taskId);
}
pub(crate) unsafe extern "C" fn PlayerPC_TurnOff(taskId: u8) {
    if sTopMenuNumOptions == 4 {
        if (*gSaveBlock2Ptr).playerGender == MALE {
            ScriptContext_SetupScript(
                LittlerootTown_BrendansHouse_2F_EventScript_TurnOffPlayerPC
                    .as_ptr()
                    .cast_mut(),
            );
        } else {
            ScriptContext_SetupScript(
                LittlerootTown_MaysHouse_2F_EventScript_TurnOffPlayerPC
                    .as_ptr()
                    .cast_mut(),
            );
        }
    } else {
        ScriptContext_Enable();
    }
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn InitItemStorageMenu(taskId: u8, var: u8) {
    let mut data: *mut u16 = null_mut();
    let mut windowTemplate: WindowTemplate = zeroed();
    data = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    windowTemplate = sWindowTemplates_MainMenus[2];
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
pub(crate) unsafe extern "C" fn ItemStorageMenuPrint(textPtr: *mut u8) {
    DrawDialogueFrame(0, 0);
    AddTextPrinterParameterized(0, FONT_NORMAL, textPtr, 0, 1, 0, None);
}
pub(crate) unsafe extern "C" fn ItemStorageMenuProcessInput(taskId: u8) {
    let mut oldPos: i8 = 0;
    let mut newPos: i8 = 0;
    let mut inputOptionId: i8 = 0;
    oldPos = Menu_GetCursorPos() as i8;
    inputOptionId = Menu_ProcessInput();
    newPos = Menu_GetCursorPos() as i8;
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
pub(crate) unsafe extern "C" fn ItemStorage_Deposit(taskId: u8) {
    gTasks[taskId].func = Some(Task_ItemStorage_Deposit);
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe extern "C" fn Task_ItemStorage_Deposit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        CB2_GoToItemDepositMenu();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_PlayerPCExitBagMenu() {
    gFieldCallback = Some(ItemStorage_ReshowAfterBagMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe extern "C" fn ItemStorage_ReshowAfterBagMenu() {
    LoadMessageBoxAndBorderGfx();
    DrawDialogueFrame(0, TRUE);
    InitItemStorageMenu(
        CreateTask(Some(ItemStorage_HandleReturnToProcessInput), 0),
        1,
    );
    FadeInFromBlack();
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleReturnToProcessInput(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        gTasks[taskId].func = Some(ItemStorageMenuProcessInput);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Withdraw(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(1) = CountUsedPCItemSlots() as i16;
    if *data.at(1) != 0 {
        ItemStorage_Enter(taskId, FALSE);
    } else {
        ItemStorage_EraseMainMenu(taskId);
        DisplayItemMessageOnField(
            taskId,
            gText_NoItems.as_ptr().cast_mut(),
            Some(PlayerPC_ItemStorage),
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Toss(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(1) = CountUsedPCItemSlots() as i16;
    if *data.at(1) != 0 {
        ItemStorage_Enter(taskId, TRUE);
    } else {
        ItemStorage_EraseMainMenu(taskId);
        DisplayItemMessageOnField(
            taskId,
            gText_NoItems.as_ptr().cast_mut(),
            Some(PlayerPC_ItemStorage),
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_Enter(taskId: u8, toss: u8) {
    let mut data: *mut u16 = gTasks[taskId].data.as_mut_ptr() as *mut u16;
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
    gTasks[taskId].func = Some(ItemStorage_CreateListMenu);
}
pub(crate) unsafe extern "C" fn ItemStorage_Exit(taskId: u8) {
    ItemStorage_EraseMainMenu(taskId);
    ReshowPlayerPC(taskId);
}
pub(crate) unsafe extern "C" fn SetPlayerPCListCount(taskId: u8) {
    if gPlayerPCItemPageInfo.count > 7 {
        gPlayerPCItemPageInfo.pageItems = 8;
    } else {
        gPlayerPCItemPageInfo.pageItems = gPlayerPCItemPageInfo.count + 1;
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_EraseMainMenu(taskId: u8) {
    let mut data: *mut u16 = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    ClearStdWindowAndFrameToTransparent(*data.at(4) as u8, FALSE);
    ClearWindowTilemap(*data.at(4) as u8);
    RemoveWindow(*data.at(4) as u8);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn GetMailboxMailCount() -> u8 {
    let mut mailInPC: u8 = 0;
    let mut i: u8 = 0;
    mailInPC = 0;
    i = PARTY_SIZE as u8;
    while i < MAIL_COUNT {
        if (*gSaveBlock1Ptr).mail[i].itemId != ITEM_NONE {
            mailInPC += 1;
        }
        i += 1;
    }
    return mailInPC;
}
pub(crate) unsafe extern "C" fn Mailbox_CompactMailList() {
    let mut temp: Mail = zeroed();
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    i = PARTY_SIZE as u8;
    while i < 15 {
        j = i + 1;
        while j < MAIL_COUNT {
            if (*gSaveBlock1Ptr).mail[i].itemId == ITEM_NONE {
                temp = (*gSaveBlock1Ptr).mail[i];
                (*gSaveBlock1Ptr).mail[i] = (*gSaveBlock1Ptr).mail[j];
                (*gSaveBlock1Ptr).mail[j] = temp;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn Mailbox_DrawMailboxMenu(taskId: u8) {
    let mut windowId: u8 = MailboxMenu_AddWindow(MAILBOXWIN_TITLE);
    MailboxMenu_AddWindow(MAILBOXWIN_LIST);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_Mailbox.as_ptr().cast_mut(),
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_Mailbox.as_ptr().cast_mut(), 0x40)
            as u8,
        1,
        0,
        None,
    );
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].data[5] = MailboxMenu_CreateList(&raw mut gPlayerPCItemPageInfo) as i16;
    MailboxMenu_AddScrollArrows(&raw mut gPlayerPCItemPageInfo);
}
pub(crate) unsafe extern "C" fn Mailbox_ProcessInput(taskId: u8) {
    let mut data: *mut u16 = gTasks[taskId].data.as_mut_ptr() as *mut u16;
    if gPaletteFade.active() == 0 {
        let mut inputOptionId: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
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
                gTasks[taskId].func = Some(Mailbox_PrintWhatToDoWithPlayerMailText);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_PrintWhatToDoWithPlayerMailText(taskId: u8) {
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
        gText_WhatToDoWithVar1sMail.as_ptr().cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(Mailbox_PrintMailOptions),
    );
}
pub(crate) unsafe extern "C" fn Mailbox_ReturnToPlayerPC(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    MailboxMenu_RemoveWindow(MAILBOXWIN_TITLE);
    MailboxMenu_RemoveWindow(MAILBOXWIN_LIST);
    DestroyListMenuTask(*data.at(5) as u8, null_mut(), null_mut());
    ScheduleBgCopyTilemapToVram(0);
    MailboxMenu_Free();
    ReshowPlayerPC(taskId);
}
pub(crate) unsafe extern "C" fn Mailbox_PrintMailOptions(taskId: u8) {
    let mut windowId: u8 = MailboxMenu_AddWindow(MAILBOXWIN_OPTIONS);
    PrintMenuTable(windowId, 4, gMailboxMailOptions.as_ptr().cast_mut());
    InitMenuInUpperLeftCornerNormal(windowId, 4, 0);
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].func = Some(Mailbox_MailOptionsProcessInput);
}
pub(crate) unsafe extern "C" fn Mailbox_MailOptionsProcessInput(taskId: u8) {
    let mut inputOptionId: i8 = ProcessMenuInput_other();
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
pub(crate) unsafe extern "C" fn Mailbox_DoMailRead(taskId: u8) {
    FadeScreen(FADE_TO_BLACK, 0);
    gTasks[taskId].func = Some(Mailbox_FadeAndReadMail);
}
pub(crate) unsafe extern "C" fn Mailbox_FadeAndReadMail(taskId: u8) {
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
pub(crate) unsafe extern "C" fn Mailbox_ReturnToFieldFromReadMail() {
    gFieldCallback = Some(Mailbox_ReshowAfterMail);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe extern "C" fn Mailbox_ReshowAfterMail() {
    let mut taskId: u8 = 0;
    LoadMessageBoxAndBorderGfx();
    taskId = CreateTask(Some(Mailbox_HandleReturnToProcessInput), 0);
    if MailboxMenu_Alloc(gPlayerPCItemPageInfo.count) == TRUE {
        Mailbox_DrawMailboxMenu(taskId);
    } else {
        DestroyTask(taskId);
    }
    FadeInFromBlack();
}
pub(crate) unsafe extern "C" fn Mailbox_HandleReturnToProcessInput(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        gTasks[taskId].func = Some(Mailbox_ProcessInput);
    }
}
pub(crate) unsafe extern "C" fn Mailbox_MoveToBag(taskId: u8) {
    DisplayItemMessageOnField(
        taskId,
        gText_MessageWillBeLost.as_ptr().cast_mut(),
        Some(Mailbox_AskConfirmMoveToBag),
    );
}
pub(crate) unsafe extern "C" fn Mailbox_AskConfirmMoveToBag(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    gTasks[taskId].func = Some(Mailbox_HandleConfirmMoveToBag);
}
pub(crate) unsafe extern "C" fn Mailbox_HandleConfirmMoveToBag(taskId: u8) {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let matched = sw1 == 0 || sw1 == MENU_B_PRESSED || sw1 == 1 || sw1 == MENU_NOTHING_CHOSEN;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            Mailbox_DoMailMoveToBag(taskId);
            break 'l1;
        }
        if sw1 == MENU_B_PRESSED {
            fall = true;
            PlaySE(SE_SELECT);
        }
        if fall || sw1 == 1 {
            fall = true;
            Mailbox_CancelMoveToBag(taskId);
            break 'l1;
        }
        if sw1 == MENU_NOTHING_CHOSEN || !matched {
            fall = true;
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Mailbox_DoMailMoveToBag(taskId: u8) {
    let mut mail: *mut Mail = &raw mut (*gSaveBlock1Ptr).mail[gPlayerPCItemPageInfo.itemsAbove
        as i32
        + PARTY_SIZE
        + gPlayerPCItemPageInfo.cursorPos as i32];
    if AddBagItem((*mail).itemId, 1) == 0 {
        DisplayItemMessageOnField(
            taskId,
            gText_BagIsFull.as_ptr().cast_mut(),
            Some(Mailbox_Cancel),
        );
    } else {
        DisplayItemMessageOnField(
            taskId,
            gText_MailToBagMessageErased.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn Mailbox_CancelMoveToBag(taskId: u8) {
    Mailbox_Cancel(taskId);
}
pub(crate) unsafe extern "C" fn Mailbox_Give(taskId: u8) {
    if CalculatePlayerPartyCount() == 0 {
        Mailbox_NoPokemonForMail(taskId);
    } else {
        FadeScreen(FADE_TO_BLACK, 0);
        gTasks[taskId].func = Some(Mailbox_DoGiveMailPokeMenu);
    }
}
pub(crate) unsafe extern "C" fn Mailbox_DoGiveMailPokeMenu(taskId: u8) {
    if gPaletteFade.active() == 0 {
        MailboxMenu_Free();
        CleanupOverworldWindowsAndTilemaps();
        ChooseMonToGiveMailFromMailbox();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Mailbox_ReturnToMailListAfterDeposit() {
    gFieldCallback = Some(Mailbox_UpdateMailListAfterDeposit);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe extern "C" fn Mailbox_UpdateMailListAfterDeposit() {
    let mut taskId: u8 = 0;
    let mut prevCount: u8 = 0;
    taskId = CreateTask(Some(Mailbox_HandleReturnToProcessInput), 0);
    prevCount = gPlayerPCItemPageInfo.count;
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
pub(crate) unsafe extern "C" fn Mailbox_NoPokemonForMail(taskId: u8) {
    DisplayItemMessageOnField(
        taskId,
        gText_NoPokemon.as_ptr().cast_mut(),
        Some(Mailbox_Cancel),
    );
}
pub(crate) unsafe extern "C" fn Mailbox_Cancel(taskId: u8) {
    MailboxMenu_RemoveWindow(MAILBOXWIN_OPTIONS);
    ClearDialogWindowAndFrame(0, 0);
    Mailbox_DrawMailboxMenu(taskId);
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].func = Some(Mailbox_ProcessInput);
}
pub(crate) unsafe extern "C" fn ItemStorage_Init() {
    sItemStorageMenu = AllocZeroed(1648) as *mut ItemStorageMenu;
    memset(
        (*sItemStorageMenu).windowIds.as_mut_ptr(),
        WINDOW_NONE as i32,
        ITEMPC_WIN_COUNT,
    );
    (*sItemStorageMenu).toSwapPos = NOT_SWAPPING;
    (*sItemStorageMenu).spriteId = SPRITE_NONE;
}
pub(crate) unsafe extern "C" fn ItemStorage_Free() {
    let mut i: u32 = 0;
    i = 0;
    while i < ITEMPC_WIN_COUNT {
        ItemStorage_RemoveWindow(i as u8);
        i += 1;
    }
    Free(sItemStorageMenu as *mut c_void);
}
pub(crate) unsafe extern "C" fn ItemStorage_AddWindow(i: u8) -> u8 {
    let mut windowIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).windowIds[i];
    if *windowIdLoc == WINDOW_NONE {
        *windowIdLoc = AddWindow((&raw const sWindowTemplates_ItemStorage[i]).cast_mut()) as u8;
        DrawStdFrameWithCustomTileAndPalette(*windowIdLoc, FALSE, 0x214, 0xE);
        ScheduleBgCopyTilemapToVram(0);
    }
    return *windowIdLoc;
}
pub(crate) unsafe extern "C" fn ItemStorage_RemoveWindow(i: u8) {
    let mut windowIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).windowIds[i];
    if *windowIdLoc != WINDOW_NONE {
        ClearStdWindowAndFrameToTransparent(*windowIdLoc, FALSE);
        ClearWindowTilemap(*windowIdLoc);
        ScheduleBgCopyTilemapToVram(0);
        RemoveWindow(*windowIdLoc);
        *windowIdLoc = WINDOW_NONE;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemStorage_RefreshListMenu() {
    let mut i: u16 = 0;
    i = 0;
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
        gText_Cancel2.as_ptr().cast_mut(),
    );
    (*sItemStorageMenu).listItems[i].name = &raw mut (*sItemStorageMenu).itemNames[i][0];
    (*sItemStorageMenu).listItems[i].id = LIST_CANCEL;
    gMultiuseListMenuTemplate = *sListMenuTemplate_ItemStorage;
    gMultiuseListMenuTemplate.windowId = ItemStorage_AddWindow(ITEMPC_WIN_LIST);
    gMultiuseListMenuTemplate.totalItems = gPlayerPCItemPageInfo.count as u16;
    gMultiuseListMenuTemplate.items = (*sItemStorageMenu).listItems.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = gPlayerPCItemPageInfo.pageItems as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyItemName_PlayerPC(string: *mut u8, itemId: u16) {
    CopyItemName(itemId, string);
}
pub(crate) unsafe extern "C" fn ItemStorage_MoveCursor(id: i32, onInit: u8, list: *mut ListMenu) {
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
pub(crate) unsafe extern "C" fn ItemStorage_PrintMenuItem(windowId: u8, id: u32, yOffset: u8) {
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
        StringExpandPlaceholders(gStringVar4.as_mut_ptr(), gText_xVar1.as_ptr().cast_mut());
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
pub(crate) unsafe extern "C" fn ItemStorage_PrintDescription(id: i32) {
    let mut description: *mut u8 = null_mut();
    let mut windowId: u8 = (*sItemStorageMenu).windowIds[1];
    if id != LIST_CANCEL {
        description = GetItemDescription((*gSaveBlock1Ptr).pcItems[id].itemId);
    } else {
        description = ItemStorage_GetMessage(MSG_GO_BACK_TO_PREV);
    }
    FillWindowPixelBuffer(windowId, 17);
    AddTextPrinterParameterized(windowId, FONT_NORMAL, description, 0, 1, 0, None);
}
pub(crate) unsafe extern "C" fn ItemStorage_AddScrollIndicator() {
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
pub(crate) unsafe extern "C" fn ItemStorage_RemoveScrollIndicator() {
    if gPlayerPCItemPageInfo.scrollIndicatorTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair(gPlayerPCItemPageInfo.scrollIndicatorTaskId);
        gPlayerPCItemPageInfo.scrollIndicatorTaskId = TASK_NONE;
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_SetSwapArrow(listTaskId: u8, b: u8, speed: u8) {
    ItemStorage_DrawSwapArrow(
        ListMenuGetYCoordForPrintingArrowCursor(listTaskId) as u8,
        b,
        speed,
    );
}
pub(crate) unsafe extern "C" fn ItemStorage_DrawSwapArrow(y: u8, b: u8, speed: u8) {
    let mut windowId: u8 = (*sItemStorageMenu).windowIds[0];
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
            gText_SelectorArrow2.as_ptr().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DrawItemIcon(itemId: u16) {
    let mut spriteId: u8 = 0;
    let mut spriteIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).spriteId;
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
pub(crate) unsafe extern "C" fn ItemStorage_EraseItemIcon() {
    let mut spriteIdLoc: *mut u8 = &raw mut (*sItemStorageMenu).spriteId;
    if *spriteIdLoc != SPRITE_NONE {
        FreeSpriteTilesByTag(TAG_ITEM_ICON);
        FreeSpritePaletteByTag(TAG_ITEM_ICON);
        DestroySprite(&raw mut gSprites[*spriteIdLoc]);
        *spriteIdLoc = SPRITE_NONE;
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_CompactList() {
    CompactPCItems();
    SetItemListPerPageCount(
        (*gSaveBlock1Ptr).pcItems.as_mut_ptr(),
        PC_ITEMS_COUNT,
        &raw mut gPlayerPCItemPageInfo.pageItems,
        &raw mut gPlayerPCItemPageInfo.count,
        8,
    );
}
pub(crate) unsafe extern "C" fn ItemStorage_CompactCursor() {
    SetCursorWithinListBounds(
        &raw mut gPlayerPCItemPageInfo.itemsAbove,
        &raw mut gPlayerPCItemPageInfo.cursorPos,
        gPlayerPCItemPageInfo.pageItems,
        gPlayerPCItemPageInfo.count,
    );
}
pub(crate) unsafe extern "C" fn ItemStorage_CreateListMenu(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut toss: u32 = 0;
    let mut i: u32 = 0;
    let mut x: u32 = 0;
    let mut text: *mut u8 = null_mut();
    data = gTasks[taskId].data.as_mut_ptr();
    i = 0;
    while i <= ITEMPC_WIN_TITLE {
        ItemStorage_AddWindow(i as u8);
        i += 1;
    }
    toss = *data.at(3) as u32;
    text = gText_TossItem.as_ptr().cast_mut();
    if toss == 0 {
        text = gText_WithdrawItem.as_ptr().cast_mut();
    }
    x = GetStringCenterAlignXOffset(FONT_NORMAL as i32, text, 104) as u32;
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
    gTasks[taskId].func = Some(ItemStorage_ProcessInput);
}
pub(crate) unsafe extern "C" fn ItemStorage_GetMessage(itemId: u16) -> *mut u8 {
    let mut string: *mut u8 = null_mut();
    match itemId {
        MSG_GO_BACK_TO_PREV => {
            string = gText_GoBackPrevMenu.as_ptr().cast_mut();
        }
        MSG_HOW_MANY_TO_WITHDRAW => {
            string = gText_WithdrawHowManyItems.as_ptr().cast_mut();
        }
        MSG_WITHDREW_ITEM => {
            string = gText_WithdrawXItems.as_ptr().cast_mut();
        }
        MSG_HOW_MANY_TO_TOSS => {
            string = gText_TossHowManyVar1s.as_ptr().cast_mut();
        }
        MSG_THREW_AWAY_ITEM => {
            string = gText_ThrewAwayVar2Var1s.as_ptr().cast_mut();
        }
        MSG_NO_MORE_ROOM => {
            string = gText_NoRoomInBag.as_ptr().cast_mut();
        }
        MSG_TOO_IMPORTANT => {
            string = gText_TooImportantToToss.as_ptr().cast_mut();
        }
        MSG_OKAY_TO_THROW_AWAY => {
            string = gText_ConfirmTossItems.as_ptr().cast_mut();
        }
        MSG_SWITCH_WHICH_ITEM => {
            string = gText_MoveVar1Where.as_ptr().cast_mut();
        }
        _ => {
            string = GetItemDescription(itemId);
        }
    }
    return string;
}
pub(crate) unsafe extern "C" fn ItemStorage_PrintMessage(string: *mut u8) {
    let mut windowId: u8 = (*sItemStorageMenu).windowIds[1];
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
pub(crate) unsafe extern "C" fn ItemStorage_ProcessInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
        let mut id: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
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
pub(crate) unsafe extern "C" fn ItemStorage_ReturnToMenuSelect(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        DrawDialogueFrame(0, 0);
        if *data.at(3) == 0 {
            InitItemStorageMenu(taskId, MENU_WITHDRAW);
        } else {
            InitItemStorageMenu(taskId, MENU_TOSS);
        }
        gTasks[taskId].func = Some(ItemStorageMenuProcessInput);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ExitItemList(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    ItemStorage_EraseItemIcon();
    ItemStorage_RemoveScrollIndicator();
    DestroyListMenuTask(*data.at(5) as u8, null_mut(), null_mut());
    DestroySwapLineSprites(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
    );
    ItemStorage_Free();
    gTasks[taskId].func = Some(ItemStorage_ReturnToMenuSelect);
}
pub(crate) unsafe extern "C" fn ItemStorage_StartItemSwap(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
    gTasks[taskId].func = Some(ItemStorage_ProcessItemSwapInput);
}
pub(crate) unsafe extern "C" fn ItemStorage_ProcessItemSwapInput(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut id: i32 = 0;
    data = gTasks[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        ListMenuGetScrollAndRow(
            *data.at(5) as u8,
            &raw mut gPlayerPCItemPageInfo.itemsAbove,
            &raw mut gPlayerPCItemPageInfo.cursorPos,
        );
        ItemStorage_FinishItemSwap(taskId, FALSE);
        return;
    }
    id = ListMenu_ProcessInput(*data.at(5) as u8);
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
pub(crate) unsafe extern "C" fn ItemStorage_FinishItemSwap(taskId: u8, canceled: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut newPos: u16 = gPlayerPCItemPageInfo.itemsAbove + gPlayerPCItemPageInfo.cursorPos;
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
    gTasks[taskId].func = Some(ItemStorage_ProcessInput);
}
pub(crate) unsafe extern "C" fn ItemStorage_UpdateSwapLinePos(y: u8) {
    UpdateSwapLineSpritesPos(
        (*sItemStorageMenu).swapLineSpriteIds.as_mut_ptr(),
        SWAP_LINE_LENGTH,
        128,
        (y as u16 + 1) * 16,
    );
}
pub(crate) unsafe extern "C" fn ItemStorage_PrintItemQuantity(
    windowId: u8,
    value: u16,
    mode: u32,
    x: u8,
    y: u8,
    n: u8,
) {
    ConvertIntToDecimalStringN(gStringVar1.as_mut_ptr(), value as i32, mode as i32, n);
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), gText_xVar1.as_ptr().cast_mut());
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
pub(crate) unsafe extern "C" fn ItemStorage_DoItemAction(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
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
    gTasks[taskId].func = Some(ItemStorage_HandleQuantityRolling);
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleQuantityRolling(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
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
pub(crate) unsafe extern "C" fn ItemStorage_DoItemWithdraw(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
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
        gTasks[taskId].func = Some(ItemStorage_HandleRemoveItem);
    } else {
        *data.at(2) = 0;
        ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_NO_MORE_ROOM));
        gTasks[taskId].func = Some(ItemStorage_HandleErrorMessageInput);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_DoItemToss(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut pos: u16 = gPlayerPCItemPageInfo.cursorPos + gPlayerPCItemPageInfo.itemsAbove;
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
        gTasks[taskId].func = Some(ItemStorage_HandleErrorMessageInput);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_TossItemYes(taskId: u8) {
    ItemStorage_PrintMessage(ItemStorage_GetMessage(MSG_THREW_AWAY_ITEM));
    gTasks[taskId].func = Some(ItemStorage_HandleRemoveItem);
}
pub(crate) unsafe extern "C" fn ItemStorage_TossItemNo(taskId: u8) {
    ItemStorage_PrintMessage(ItemStorage_GetMessage(
        (*gSaveBlock1Ptr).pcItems
            [gPlayerPCItemPageInfo.itemsAbove as i32 + gPlayerPCItemPageInfo.cursorPos as i32]
            .itemId,
    ));
    ItemStorage_ReturnToListInput(taskId);
}
pub(crate) unsafe extern "C" fn ItemStorage_HandleRemoveItem(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn ItemStorage_HandleErrorMessageInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & 3 != 0 {
        ItemStorage_PrintMessage(ItemStorage_GetMessage(
            (*gSaveBlock1Ptr).pcItems
                [gPlayerPCItemPageInfo.itemsAbove as i32 + gPlayerPCItemPageInfo.cursorPos as i32]
                .itemId,
        ));
        ItemStorage_ReturnToListInput(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemStorage_ReturnToListInput(taskId: u8) {
    ItemStorage_AddScrollIndicator();
    gTasks[taskId].func = Some(ItemStorage_ProcessInput);
}
