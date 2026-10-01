//! Translated from `src/shop.c` by tools/rustport/c2rs.py.
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

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{FillBgTilemapBufferRect_Palette0, ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::decoration::AddDecorationIconObject;
use crate::decoration_inventory::DecorationAdd;
use crate::event_object_movement::{
    CreateObjectGraphicsSprite, GetObjectEventGraphicsInfo, GetObjectEventIdByXY,
};
use crate::field_player_avatar::{GetXYCoordsOneStepInFrontOfPlayer, gObjectEvents};
use crate::field_screen_effect::FadeInFromBlack;
use crate::field_weather::{FadeScreen, IsWeatherNotFadingIn};
use crate::fieldmap::{MapGridGetMetatileIdAt, MapGridGetMetatileLayerTypeAt, gMapHeader};
use crate::gpu_regs::SetGpuReg;
use crate::item::{
    AddBagItem, CopyItemName, CountTotalItemQuantityInBag, GetItemPocket, GetItemPrice,
};
use crate::item_icon::AddItemIconSprite;
use crate::item_menu::CB2_GoToSellMenu;
use crate::list_menu::{
    AddScrollIndicatorArrowPairParameterized, ListMenu_ProcessInput, ListMenuGetScrollAndRow,
    ListMenuGetYCoordForPrintingArrowCursor, ListMenuInit, RemoveScrollIndicatorArrowPair,
    gMultiuseListMenuTemplate,
};
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::{
    AddTextPrinterParameterized4, ClearDialogWindowAndFrameToTransparent,
    ClearScheduledBgCopiesToVram, ClearStdWindowAndFrameToTransparent,
    DecompressAndCopyTileDataToVram, DisplayItemMessageOnField, DoScheduledBgTilemapCopiesToVram,
    DrawStdFrameWithCustomTileAndPalette, FreeTempTileDataBuffersIfPossible,
    GetPlayerTextSpeedDelay, InitMenuInUpperLeftCornerNormal, Menu_ProcessInputNoWrap,
    PrintMenuTable, ResetTempTileDataBuffers, ScheduleBgCopyTilemapToVram,
    SetStandardWindowBorderStyle,
};
use crate::menu_helpers::{
    AdjustQuantityAccordingToDPadInput, CreateYesNoMenuWithCallbacks,
    DisplayMessageAndContinueTask, SetVBlankHBlankCallbacksToNull,
};
use crate::money::{
    AddMoneyLabelObject, GetMoney, IsEnoughMoney, PrintMoneyAmount, PrintMoneyAmountInMoneyBox,
    PrintMoneyAmountInMoneyBoxWithBorder, RemoveMoney, RemoveMoneyLabelObject,
};
use crate::overworld::{CB2_ReturnToField, IncrementGameStat, gFieldCallback};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::party_menu::ItemIdToBattleMoveId;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeSpritePaletteByTag,
    FreeSpriteTilesByTag, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar3, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
use crate::tv::{IsPokeNewsActive, TryPutSmartShopperOnAir};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers,
    PutWindowTilemap, RemoveWindow,
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
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
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
const tListTaskId: usize = 7;
// Data tables (translate with cdata.py): sShopPurchaseYesNoFuncs sShopMenuActions_BuySellQuit sShopMenuActions_BuyQuit sShopMenuWindowTemplates sShopBuyMenuListTemplate sShopBuyMenuBgTemplates sShopBuyMenuWindowTemplates sShopBuyMenuYesNoWindowTemplates sShopBuyMenuTextColors

/// `struct MartInfo`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MartInfo {
    pub callback: Option<unsafe fn()>,
    pub menuActions: *mut MenuAction,
    pub itemList: *mut u16,
    pub itemCount: u16,
    pub windowId: u8,
    pub martType: u8,
}

unsafe impl Sync for MartInfo {}

/// `struct ShopData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShopData {
    pub tilemapBuffers: CArray<CArray<u16, 1024>, 4>,
    pub totalCost: u32,
    pub itemsShowed: u16,
    pub selectedRow: u16,
    pub scrollOffset: u16,
    pub maxQuantity: u8,
    pub scrollIndicatorsTaskId: u8,
    pub iconSlot: u8,
    pub itemSpriteIds: CArray<u8, 2>,
    pub viewportObjects: CArray<CArray<i16, 5>, 16>,
}

unsafe impl Sync for ShopData {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<MartInfo>() == 16);
    assert!(offset_of!(MartInfo, callback) == 0);
    assert!(offset_of!(MartInfo, menuActions) == 4);
    assert!(offset_of!(MartInfo, itemList) == 8);
    assert!(offset_of!(MartInfo, itemCount) == 12);
    assert!(offset_of!(MartInfo, windowId) == 14);
    assert!(offset_of!(MartInfo, martType) == 15);
    assert!(size_of::<ShopData>() == 8368);
    assert!(offset_of!(ShopData, tilemapBuffers) == 0);
    assert!(offset_of!(ShopData, totalCost) == 8192);
    assert!(offset_of!(ShopData, itemsShowed) == 8196);
    assert!(offset_of!(ShopData, selectedRow) == 8198);
    assert!(offset_of!(ShopData, scrollOffset) == 8200);
    assert!(offset_of!(ShopData, maxQuantity) == 8202);
    assert!(offset_of!(ShopData, scrollIndicatorsTaskId) == 8203);
    assert!(offset_of!(ShopData, iconSlot) == 8204);
    assert!(offset_of!(ShopData, itemSpriteIds) == 8205);
    assert!(offset_of!(ShopData, viewportObjects) == 8208);
};

const ANIM_NUM: i32 = 3;
const COLORID_GRAY_CURSOR: u8 = 2;
const COLORID_ITEM_LIST: u8 = 1;
const COLORID_NORMAL: u8 = 0;
const LAYER_TYPE: i32 = 4;
const MART_TYPE_DECOR: u8 = 1;
const MART_TYPE_DECOR2: u8 = 2;
const MART_TYPE_NORMAL: u8 = 0;
const MAX_ITEMS_SHOWN: u16 = 8;
const OBJ_EVENT_ID: i32 = 0;
const SHOP_MENU_PALETTE_ID: i32 = 12;
const TAG_ITEM_ICON_BASE: u16 = 2110;
const TAG_SCROLL_ARROW: i32 = 2100;
const WIN_BUY_QUIT: i32 = 1;
const WIN_BUY_SELL_QUIT: i32 = 0;
const WIN_ITEM_DESCRIPTION: u8 = 2;
const WIN_ITEM_LIST: u8 = 1;
const WIN_MESSAGE: u8 = 5;
const WIN_MONEY: u8 = 0;
const WIN_QUANTITY_IN_BAG: u8 = 3;
const WIN_QUANTITY_PRICE: u8 = 4;
const X_COORD: i32 = 1;
const Y_COORD: i32 = 2;

static sShopBuyMenuBgTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::shop::sShopBuyMenuBgTemplates).cast());
static sShopBuyMenuListTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::shop::sShopBuyMenuListTemplate).cast());
static sShopBuyMenuTextColors: Table<CArray<CArray<u8, 3>, 3>> =
    Table((&raw const crate::data::shop::sShopBuyMenuTextColors).cast());
static sShopBuyMenuWindowTemplates: Table<CArray<WindowTemplate, 7>> =
    Table((&raw const crate::data::shop::sShopBuyMenuWindowTemplates).cast());
static sShopBuyMenuYesNoWindowTemplates: Table<WindowTemplate> =
    Table((&raw const crate::data::shop::sShopBuyMenuYesNoWindowTemplates).cast());
static sShopMenuActions_BuyQuit: Table<CArray<MenuAction, 2>> =
    Table((&raw const crate::data::shop::sShopMenuActions_BuyQuit).cast());
static sShopMenuActions_BuySellQuit: Table<CArray<MenuAction, 3>> =
    Table((&raw const crate::data::shop::sShopMenuActions_BuySellQuit).cast());
static sShopMenuWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::shop::sShopMenuWindowTemplates).cast());
static sShopPurchaseYesNoFuncs: Table<YesNoFuncTable> =
    Table((&raw const crate::data::shop::sShopPurchaseYesNoFuncs).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMartInfo: MartInfo = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sShopData: *mut ShopData = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sListMenuItems: *mut ListMenuItem = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemNames: *mut CArray<u8, 16> = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sPurchaseHistoryId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gMartPurchaseHistory: CArray<ItemSlot, 3> = unsafe { zeroed() };

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
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

unsafe fn CreateShopMenu(martType: u8) -> u8 {
    let mut numMenuItems: i32 = 0;
    LockPlayerFieldControls();
    sMartInfo.martType = martType;
    if martType == MART_TYPE_NORMAL {
        let mut winTemplate: WindowTemplate = sShopMenuWindowTemplates[0];
        winTemplate.width =
            GetMaxWidthInMenuTable(sShopMenuActions_BuySellQuit.as_ptr().cast_mut(), 3) as u8;
        sMartInfo.windowId = AddWindow(&raw mut winTemplate) as u8;
        sMartInfo.menuActions = sShopMenuActions_BuySellQuit.as_ptr().cast_mut();
        numMenuItems = 3;
    } else {
        let mut winTemplate: WindowTemplate = sShopMenuWindowTemplates[1];
        winTemplate.width =
            GetMaxWidthInMenuTable(sShopMenuActions_BuyQuit.as_ptr().cast_mut(), 2) as u8;
        sMartInfo.windowId = AddWindow(&raw mut winTemplate) as u8;
        sMartInfo.menuActions = sShopMenuActions_BuyQuit.as_ptr().cast_mut();
        numMenuItems = 2;
    }
    SetStandardWindowBorderStyle(sMartInfo.windowId, FALSE);
    PrintMenuTable(
        sMartInfo.windowId,
        numMenuItems as u8,
        sMartInfo.menuActions,
    );
    InitMenuInUpperLeftCornerNormal(sMartInfo.windowId, numMenuItems as u8, 0);
    PutWindowTilemap(sMartInfo.windowId);
    CopyWindowToVram(sMartInfo.windowId, COPYWIN_MAP);
    CreateTask(Some(Task_ShopMenu), 8)
}
unsafe fn SetShopMenuCallback(callback: Option<unsafe fn()>) {
    sMartInfo.callback = callback;
}
unsafe fn SetShopItemsForSale(items: *mut u16) {
    let mut i: u16 = 0;
    sMartInfo.itemList = items;
    sMartInfo.itemCount = 0;
    while *sMartInfo.itemList.at(i) != 0 {
        sMartInfo.itemCount += 1;
        i += 1;
    }
}
pub(crate) unsafe fn Task_ShopMenu(taskId: u8) {
    let inputCode: i8 = Menu_ProcessInputNoWrap();
    match inputCode {
        MENU_NOTHING_CHOSEN => {}
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            Task_HandleShopMenuQuit(taskId);
        }
        _ => {
            (*sMartInfo.menuActions.at(inputCode))
                .func
                .void_u8
                .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe fn Task_HandleShopMenuBuy(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(8) = (CB2_InitBuyMenu as *const () as usize as u32 >> 16) as i16;
    *data.at(9) = CB2_InitBuyMenu as *const () as usize as u32 as i16;
    task_set_func(taskId, Some(Task_GoToBuyOrSellMenu));
    FadeScreen(FADE_TO_BLACK, 0);
}
pub(crate) unsafe fn Task_HandleShopMenuSell(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(8) = (CB2_GoToSellMenu as *const () as usize as u32 >> 16) as i16;
    *data.at(9) = CB2_GoToSellMenu as *const () as usize as u32 as i16;
    task_set_func(taskId, Some(Task_GoToBuyOrSellMenu));
    FadeScreen(FADE_TO_BLACK, 0);
}
pub unsafe fn CB2_ExitSellMenu() {
    gFieldCallback = Some(MapPostLoadHook_ReturnToShopMenu);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn Task_HandleShopMenuQuit(taskId: u8) {
    ClearStdWindowAndFrameToTransparent(sMartInfo.windowId, 2);
    RemoveWindow(sMartInfo.windowId);
    TryPutSmartShopperOnAir();
    UnlockPlayerFieldControls();
    DestroyTask(taskId);
    if sMartInfo.callback.is_some() {
        sMartInfo.callback.unwrap_unchecked()();
    }
}
pub(crate) unsafe fn Task_GoToBuyOrSellMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        SetMainCallback2(core::mem::transmute::<usize, Option<unsafe fn()>>(
            ((*data.at(8) as u16 as i32) << 16 | *data.at(9) as u16 as i32) as usize,
        ));
    }
}
pub(crate) unsafe fn MapPostLoadHook_ReturnToShopMenu() {
    FadeInFromBlack();
    CreateTask(Some(Task_ReturnToShopMenu), 8);
}
pub(crate) unsafe fn Task_ReturnToShopMenu(taskId: u8) {
    if IsWeatherNotFadingIn() == TRUE {
        if sMartInfo.martType == MART_TYPE_DECOR2 {
            DisplayItemMessageOnField(
                taskId,
                (*(&raw const crate::data::strings::gText_CanIHelpWithAnythingElse)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(ShowShopMenuAfterExitingBuyOrSellMenu),
            );
        } else {
            DisplayItemMessageOnField(
                taskId,
                (*(&raw const crate::data::strings::gText_AnythingElseICanHelp)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(ShowShopMenuAfterExitingBuyOrSellMenu),
            );
        }
    }
}
pub(crate) unsafe fn ShowShopMenuAfterExitingBuyOrSellMenu(taskId: u8) {
    CreateShopMenu(sMartInfo.martType);
    DestroyTask(taskId);
}
pub(crate) unsafe fn CB2_BuyMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_BuyMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_InitBuyMenu() {
    let mut taskId: u8 = 0;
    match gMain.state {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            {
                let mut tmp: u32 = 0;
                volatile_write(&raw mut tmp, 0);
                CpuFastSet(
                    &raw mut tmp as *mut c_void,
                    OAM as i32 as usize as *mut c_void,
                    0x1000100,
                );
            }
            ScanlineEffect_Stop();
            ResetTempTileDataBuffers();
            FreeAllSpritePalettes();
            ResetPaletteFade();
            ResetSpriteData();
            ResetTasks();
            ClearScheduledBgCopiesToVram();
            sShopData = AllocZeroed(8368) as *mut ShopData;
            (*sShopData).scrollIndicatorsTaskId = TASK_NONE;
            (*sShopData).itemSpriteIds[0] = SPRITE_NONE;
            (*sShopData).itemSpriteIds[1] = SPRITE_NONE;
            BuyMenuBuildListMenuTemplate();
            BuyMenuInitBgs();
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 0x20, 0x20);
            FillBgTilemapBufferRect_Palette0(1, 0, 0, 0, 0x20, 0x20);
            FillBgTilemapBufferRect_Palette0(2, 0, 0, 0, 0x20, 0x20);
            FillBgTilemapBufferRect_Palette0(3, 0, 0, 0, 0x20, 0x20);
            BuyMenuInitWindows();
            BuyMenuDecompressBgGraphics();
            gMain.state += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() == 0 {
                gMain.state += 1;
            }
        }
        _ => {
            BuyMenuDrawGraphics();
            BuyMenuAddScrollIndicatorArrows();
            taskId = CreateTask(Some(Task_BuyMenu), 8);
            task_set(
                taskId,
                tListTaskId,
                ListMenuInit(&raw mut gMultiuseListMenuTemplate, 0, 0) as i16,
            );
            BlendPalettes(PALETTES_ALL, 16, 0);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetVBlankCallback(Some(VBlankCB_BuyMenu));
            SetMainCallback2(Some(CB2_BuyMenu));
        }
    }
}
unsafe fn BuyMenuFreeMemory() {
    Free(sShopData as *mut c_void);
    Free(sListMenuItems as *mut c_void);
    Free(sItemNames as *mut c_void);
    FreeAllWindowBuffers();
}
unsafe fn BuyMenuBuildListMenuTemplate() {
    sListMenuItems = Alloc((sMartInfo.itemCount as u32 + 1) * 8) as *mut ListMenuItem;
    sItemNames = Alloc((sMartInfo.itemCount as u32 + 1) * 16) as *mut CArray<u8, 16>;
    let mut i: u16 = 0;
    while i < sMartInfo.itemCount {
        BuyMenuSetListEntry(
            sListMenuItems.at(i),
            *sMartInfo.itemList.at(i),
            (*sItemNames.at(i)).as_mut_ptr(),
        );
        i += 1;
    }
    StringCopy(
        (*sItemNames.at(i)).as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Cancel2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*sListMenuItems.at(i)).name = (*sItemNames.at(i)).as_mut_ptr();
    (*sListMenuItems.at(i)).id = LIST_CANCEL;
    gMultiuseListMenuTemplate = *sShopBuyMenuListTemplate;
    gMultiuseListMenuTemplate.items = sListMenuItems;
    gMultiuseListMenuTemplate.totalItems = sMartInfo.itemCount + 1;
    if gMultiuseListMenuTemplate.totalItems > MAX_ITEMS_SHOWN {
        gMultiuseListMenuTemplate.maxShowed = MAX_ITEMS_SHOWN;
    } else {
        gMultiuseListMenuTemplate.maxShowed = gMultiuseListMenuTemplate.totalItems;
    }
    (*sShopData).itemsShowed = gMultiuseListMenuTemplate.maxShowed;
}
unsafe fn BuyMenuSetListEntry(menuItem: *mut ListMenuItem, item: u16, name: *mut u8) {
    if sMartInfo.martType == MART_TYPE_NORMAL {
        CopyItemName(item, name);
    } else {
        StringCopy(
            name,
            (*(&raw const crate::data::decoration::gDecorations).cast::<CArray<Decoration, 0>>())
                [item]
                .name
                .as_ptr()
                .cast_mut(),
        );
    }
    (*menuItem).name = name;
    (*menuItem).id = item as i32;
}
pub(crate) unsafe fn BuyMenuPrintItemDescriptionAndShowItemIcon(
    item: i32,
    onInit: u8,
    list: *mut ListMenu,
) {
    let mut description: *mut u8 = null_mut();
    if onInit != TRUE {
        PlaySE(SE_SELECT);
    }
    if item != LIST_CANCEL {
        BuyMenuAddItemIcon(item as u16, (*sShopData).iconSlot);
    } else {
        BuyMenuAddItemIcon(ITEM_LIST_END, (*sShopData).iconSlot);
    }
    BuyMenuRemoveItemIcon(item as u16, (*sShopData).iconSlot ^ 1);
    (*sShopData).iconSlot ^= 1;
    if item != LIST_CANCEL {
        if sMartInfo.martType == MART_TYPE_NORMAL {
            description = GetItemDescription(item as u16);
        } else {
            description = (*(&raw const crate::data::decoration::gDecorations)
                .cast::<CArray<Decoration, 0>>())[item]
                .description;
        }
    } else {
        description = (*(&raw const crate::data::strings::gText_QuitShopping)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    }
    FillWindowPixelBuffer(WIN_ITEM_DESCRIPTION, 0);
    BuyMenuPrint(WIN_ITEM_DESCRIPTION, description, 3, 1, 0, COLORID_NORMAL);
}
pub(crate) unsafe fn BuyMenuPrintPriceInList(windowId: u8, itemId: u32, y: u8) {
    let mut x: u8 = 0;
    if itemId != LIST_CANCEL as u32 {
        if sMartInfo.martType == MART_TYPE_NORMAL {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                shr_i32(
                    GetItemPrice(itemId as u16) as i32,
                    IsPokeNewsActive(POKENEWS_SLATEPORT) as u32,
                ),
                STR_CONV_MODE_LEFT_ALIGN,
                5,
            );
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::decoration::gDecorations)
                    .cast::<CArray<Decoration, 0>>())[itemId]
                    .price as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                5,
            );
        }
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_PokedollarVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        x = GetStringRightAlignXOffset(FONT_NARROW as i32, gStringVar4.as_mut_ptr(), 120) as u8;
        AddTextPrinterParameterized4(
            windowId,
            FONT_NARROW,
            x,
            y,
            0,
            0,
            sShopBuyMenuTextColors[1].as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gStringVar4.as_mut_ptr(),
        );
    }
}
unsafe fn BuyMenuAddScrollIndicatorArrows() {
    if (*sShopData).scrollIndicatorsTaskId == TASK_NONE
        && sMartInfo.itemCount as i32 + 1 > MAX_ITEMS_SHOWN as i32
    {
        (*sShopData).scrollIndicatorsTaskId = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            172,
            12,
            148,
            sMartInfo.itemCount as i32 - 7,
            TAG_SCROLL_ARROW,
            TAG_SCROLL_ARROW,
            &raw mut (*sShopData).scrollOffset,
        );
    }
}
unsafe fn BuyMenuRemoveScrollIndicatorArrows() {
    if (*sShopData).scrollIndicatorsTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*sShopData).scrollIndicatorsTaskId);
        (*sShopData).scrollIndicatorsTaskId = TASK_NONE;
    }
}
unsafe fn BuyMenuPrintCursor(scrollIndicatorsTaskId: u8, colorSet: u8) {
    let y: u8 = ListMenuGetYCoordForPrintingArrowCursor(scrollIndicatorsTaskId) as u8;
    BuyMenuPrint(
        WIN_ITEM_LIST,
        (*(&raw const crate::data::strings::gText_SelectorArrow2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        y,
        0,
        colorSet,
    );
}
unsafe fn BuyMenuAddItemIcon(item: u16, iconSlot: u8) {
    let mut spriteId: u8 = 0;
    let spriteIdPtr: *mut u8 = &raw mut (*sShopData).itemSpriteIds[iconSlot];
    if *spriteIdPtr != SPRITE_NONE {
        return;
    }
    if sMartInfo.martType == MART_TYPE_NORMAL || item == ITEM_LIST_END {
        spriteId = AddItemIconSprite(
            iconSlot as u16 + TAG_ITEM_ICON_BASE,
            iconSlot as u16 + TAG_ITEM_ICON_BASE,
            item,
        );
        if spriteId != MAX_SPRITES {
            *spriteIdPtr = spriteId;
            gSprites[spriteId].x2 = 24;
            gSprites[spriteId].y2 = 88;
        }
    } else {
        spriteId = AddDecorationIconObject(
            item as u8,
            20,
            84,
            1,
            iconSlot as u16 + TAG_ITEM_ICON_BASE,
            iconSlot as u16 + TAG_ITEM_ICON_BASE,
        );
        if spriteId != MAX_SPRITES {
            *spriteIdPtr = spriteId;
        }
    }
}
unsafe fn BuyMenuRemoveItemIcon(item: u16, iconSlot: u8) {
    let spriteIdPtr: *mut u8 = &raw mut (*sShopData).itemSpriteIds[iconSlot];
    if *spriteIdPtr == SPRITE_NONE {
        return;
    }
    FreeSpriteTilesByTag(iconSlot as u16 + TAG_ITEM_ICON_BASE);
    FreeSpritePaletteByTag(iconSlot as u16 + TAG_ITEM_ICON_BASE);
    DestroySprite(&raw mut gSprites[*spriteIdPtr]);
    *spriteIdPtr = SPRITE_NONE;
}
unsafe fn BuyMenuInitBgs() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sShopBuyMenuBgTemplates.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        1,
        (*sShopData).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        2,
        (*sShopData).tilemapBuffers[3].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sShopData).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
    );
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG3HOFS, 0);
    SetGpuReg(REG_OFFSET_BG3VOFS, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(0x0, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
unsafe fn BuyMenuDecompressBgGraphics() {
    DecompressAndCopyTileDataToVram(
        1,
        (*(&raw const crate::data::graphics::gShopMenu_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        0x3A0,
        0x3E3,
        0,
    );
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gShopMenu_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*sShopData).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gShopMenu_Pal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        192,
        32,
    );
}
unsafe fn BuyMenuInitWindows() {
    InitWindows(sShopBuyMenuWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(WIN_MONEY, 1, 208);
    LoadMessageBoxGfx(WIN_MONEY, 0xA, 224);
    PutWindowTilemap(WIN_MONEY);
    PutWindowTilemap(WIN_ITEM_LIST);
    PutWindowTilemap(WIN_ITEM_DESCRIPTION);
}
unsafe fn BuyMenuPrint(windowId: u8, text: *mut u8, x: u8, y: u8, speed: i8, colorSet: u8) {
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x,
        y,
        0,
        0,
        sShopBuyMenuTextColors[colorSet].as_ptr().cast_mut(),
        speed,
        text,
    );
}
unsafe fn BuyMenuDisplayMessage(taskId: u8, text: *mut u8, callback: Option<unsafe fn(u8)>) {
    DisplayMessageAndContinueTask(
        taskId,
        WIN_MESSAGE,
        10,
        14,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        text,
        core::mem::transmute::<Option<unsafe fn(u8)>, *mut c_void>(callback),
    );
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn BuyMenuDrawGraphics() {
    BuyMenuDrawMapGraphics();
    BuyMenuCopyMenuBgToBg1TilemapBuffer();
    AddMoneyLabelObject(19, 11);
    PrintMoneyAmountInMoneyBoxWithBorder(
        WIN_MONEY,
        1,
        13,
        GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32,
    );
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
}
unsafe fn BuyMenuDrawMapGraphics() {
    BuyMenuCollectObjectEventData();
    BuyMenuDrawObjectEvents();
    BuyMenuDrawMapBg();
}
unsafe fn BuyMenuDrawMapBg() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut mapLayout: *mut MapLayout = null_mut();
    let mut metatile: u16 = 0;
    let mut metatileLayerType: u8 = 0;
    mapLayout = gMapHeader.mapLayout;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    x -= 4;
    y -= 4;
    for j in 0..10i16 {
        for i in 0..15i16 {
            metatile = MapGridGetMetatileIdAt(x as i32 + i as i32, y as i32 + j as i32) as u16;
            if BuyMenuCheckForOverlapWithMenuBg(i as i32, j as i32) == TRUE {
                metatileLayerType =
                    MapGridGetMetatileLayerTypeAt(x as i32 + i as i32, y as i32 + j as i32);
            } else {
                metatileLayerType = METATILE_LAYER_TYPE_COVERED as u8;
            }
            if metatile < NUM_METATILES_IN_PRIMARY {
                BuyMenuDrawMapMetatile(
                    i,
                    j,
                    (*(*mapLayout).primaryTileset)
                        .metatiles
                        .at(metatile as i32 * NUM_TILES_PER_METATILE),
                    metatileLayerType,
                );
            } else {
                BuyMenuDrawMapMetatile(
                    i,
                    j,
                    (*(*mapLayout).secondaryTileset)
                        .metatiles
                        .at((metatile as i32 - NUM_METATILES_IN_PRIMARY as i32)
                            * NUM_TILES_PER_METATILE),
                    metatileLayerType,
                );
            }
        }
    }
}
unsafe fn BuyMenuDrawMapMetatile(x: i16, y: i16, src: *mut u16, metatileLayerType: u8) {
    let offset1: u16 = x as u16 * 2;
    let offset2: u16 = y as u16 * 64;
    match metatileLayerType {
        0 => {
            BuyMenuDrawMapMetatileLayer(
                (*sShopData).tilemapBuffers[3].as_mut_ptr(),
                offset1 as i16,
                offset2 as i16,
                src,
            );
            BuyMenuDrawMapMetatileLayer(
                (*sShopData).tilemapBuffers[1].as_mut_ptr(),
                offset1 as i16,
                offset2 as i16,
                src.at(4),
            );
        }
        1 => {
            BuyMenuDrawMapMetatileLayer(
                (*sShopData).tilemapBuffers[2].as_mut_ptr(),
                offset1 as i16,
                offset2 as i16,
                src,
            );
            BuyMenuDrawMapMetatileLayer(
                (*sShopData).tilemapBuffers[3].as_mut_ptr(),
                offset1 as i16,
                offset2 as i16,
                src.at(4),
            );
        }
        2 => {
            BuyMenuDrawMapMetatileLayer(
                (*sShopData).tilemapBuffers[2].as_mut_ptr(),
                offset1 as i16,
                offset2 as i16,
                src,
            );
            BuyMenuDrawMapMetatileLayer(
                (*sShopData).tilemapBuffers[1].as_mut_ptr(),
                offset1 as i16,
                offset2 as i16,
                src.at(4),
            );
        }
        _ => {}
    }
}
unsafe fn BuyMenuDrawMapMetatileLayer(dest: *mut u16, offset1: i16, offset2: i16, src: *mut u16) {
    *dest.at(offset1 as i32 + offset2 as i32) = *src;
    *dest.at(offset1 as i32 + offset2 as i32 + 1) = *src.at(1);
    *dest.at(offset1 as i32 + offset2 as i32 + 32) = *src.at(2);
    *dest.at(offset1 as i32 + offset2 as i32 + 33) = *src.at(3);
}
unsafe fn BuyMenuCollectObjectEventData() {
    let mut facingX: i16 = 0;
    let mut facingY: i16 = 0;
    let mut numObjects: u8 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut facingX, &raw mut facingY);
    for y in 0..OBJECT_EVENTS_COUNT {
        (*sShopData).viewportObjects[y][0] = OBJECT_EVENTS_COUNT as i16;
    }
    for y in 0..5u8 {
        for x in 0..7u8 {
            let objEventId: u8 =
                GetObjectEventIdByXY(facingX - 4 + x as i16, facingY - 2 + y as i16);
            if objEventId != OBJECT_EVENTS_COUNT {
                (*sShopData).viewportObjects[numObjects][0] = objEventId as i16;
                (*sShopData).viewportObjects[numObjects][1] = x as i16;
                (*sShopData).viewportObjects[numObjects][2] = y as i16;
                (*sShopData).viewportObjects[numObjects][4] = MapGridGetMetatileLayerTypeAt(
                    facingX as i32 - 4 + x as i32,
                    facingY as i32 - 2 + y as i32,
                ) as i16;
                match gObjectEvents[objEventId].facingDirection() {
                    1 => {
                        (*sShopData).viewportObjects[numObjects][3] = ANIM_STD_FACE_SOUTH;
                    }
                    2 => {
                        (*sShopData).viewportObjects[numObjects][3] = ANIM_STD_FACE_NORTH;
                    }
                    3 => {
                        (*sShopData).viewportObjects[numObjects][3] = ANIM_STD_FACE_WEST;
                    }
                    _ => {
                        (*sShopData).viewportObjects[numObjects][3] = ANIM_STD_FACE_EAST;
                    }
                }
                numObjects += 1;
            }
        }
    }
}
unsafe fn BuyMenuDrawObjectEvents() {
    let mut spriteId: u8 = 0;
    let mut graphicsInfo: *mut ObjectEventGraphicsInfo = null_mut();
    for i in 0..OBJECT_EVENTS_COUNT {
        'l1: {
            if (*sShopData).viewportObjects[i][0] == OBJECT_EVENTS_COUNT as i16 {
                break 'l1;
            }
            graphicsInfo = GetObjectEventGraphicsInfo(
                gObjectEvents[(*sShopData).viewportObjects[i][0]].graphicsId,
            );
            spriteId = CreateObjectGraphicsSprite(
                gObjectEvents[(*sShopData).viewportObjects[i][0]].graphicsId as u16,
                Some(SpriteCallbackDummy),
                (*sShopData).viewportObjects[i][1] as u16 as i16 * 16 + 8,
                (*sShopData).viewportObjects[i][2] as u16 as i16 * 16 + 48
                    - (*graphicsInfo).height / 2,
                2,
            );
            if BuyMenuCheckIfObjectEventOverlapsMenuBg((*sShopData).viewportObjects[i].as_mut_ptr())
                == TRUE
            {
                gSprites[spriteId].set_subspriteTableNum(4);
                gSprites[spriteId].set_subspriteMode(SUBSPRITES_ON);
            }
            StartSpriteAnim(
                &raw mut gSprites[spriteId],
                (*sShopData).viewportObjects[i][3] as u8,
            );
        }
    }
}
unsafe fn BuyMenuCheckIfObjectEventOverlapsMenuBg(object: *mut i16) -> u8 {
    if BuyMenuCheckForOverlapWithMenuBg(*object.at(1) as i32, *object.at(2) as i32 + 2) == 0
        && *object.at(4) != METATILE_LAYER_TYPE_COVERED as i16
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn BuyMenuCopyMenuBgToBg1TilemapBuffer() {
    let dest: *mut u16 = (*sShopData).tilemapBuffers[1].as_mut_ptr();
    let src: *mut u16 = (*sShopData).tilemapBuffers[0].as_mut_ptr();
    for i in 0..1024i16 {
        if *src.at(i) != 0 {
            *dest.at(i) = *src.at(i) + 50147;
        }
    }
}
unsafe fn BuyMenuCheckForOverlapWithMenuBg(x: i32, y: i32) -> u8 {
    let metatile: *mut u16 = (*sShopData).tilemapBuffers[0].as_mut_ptr();
    let offset1: i32 = x * 2;
    let offset2: i32 = y * 64;
    if *metatile.at(offset2 + offset1) == 0
        && *metatile.at(offset2 + offset1 + 32) == 0
        && *metatile.at(offset2 + offset1 + 1) == 0
        && *metatile.at(offset2 + offset1 + 33) == 0
    {
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn Task_BuyMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        let itemId: i32 = ListMenu_ProcessInput(*data.at(7) as u8);
        ListMenuGetScrollAndRow(
            *data.at(7) as u8,
            &raw mut (*sShopData).scrollOffset,
            &raw mut (*sShopData).selectedRow,
        );
        match itemId {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                ExitBuyMenu(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                *data.at(5) = itemId as i16;
                ClearWindowTilemap(WIN_ITEM_DESCRIPTION);
                BuyMenuRemoveScrollIndicatorArrows();
                BuyMenuPrintCursor(*data.at(7) as u8, COLORID_GRAY_CURSOR);
                if sMartInfo.martType == MART_TYPE_NORMAL {
                    (*sShopData).totalCost = shr_i32(
                        GetItemPrice(itemId as u16) as i32,
                        IsPokeNewsActive(POKENEWS_SLATEPORT) as u32,
                    ) as u32;
                } else {
                    (*sShopData).totalCost = (*(&raw const crate::data::decoration::gDecorations)
                        .cast::<CArray<Decoration, 0>>())[itemId]
                        .price as u32;
                }
                if IsEnoughMoney(&raw mut (*gSaveBlock1Ptr).money, (*sShopData).totalCost) == 0 {
                    BuyMenuDisplayMessage(
                        taskId,
                        (*(&raw const crate::data::strings::gText_YouDontHaveMoney)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                        Some(BuyMenuReturnToItemList),
                    );
                } else {
                    if sMartInfo.martType == MART_TYPE_NORMAL {
                        CopyItemName(itemId as u16, gStringVar1.as_mut_ptr());
                        if GetItemPocket(itemId as u16) == POCKET_TM_HM {
                            StringCopy(
                                gStringVar2.as_mut_ptr(),
                                (*(&raw const crate::data::data_tables::gMoveNames).cast::<CArray<
                                    CArray<u8, 13>,
                                    355,
                                >>(
                                ))[ItemIdToBattleMoveId(itemId as u16)]
                                .as_ptr()
                                .cast_mut(),
                            );
                            BuyMenuDisplayMessage(
                                taskId,
                                (*(&raw const crate::data::strings::gText_Var1CertainlyHowMany2)
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                                Some(Task_BuyHowManyDialogueInit),
                            );
                        } else {
                            BuyMenuDisplayMessage(
                                taskId,
                                (*(&raw const crate::data::strings::gText_Var1CertainlyHowMany)
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                                Some(Task_BuyHowManyDialogueInit),
                            );
                        }
                    } else {
                        StringCopy(
                            gStringVar1.as_mut_ptr(),
                            (*(&raw const crate::data::decoration::gDecorations).cast::<CArray<
                                Decoration,
                                0,
                            >>(
                            ))[itemId]
                                .name
                                .as_ptr()
                                .cast_mut(),
                        );
                        ConvertIntToDecimalStringN(
                            gStringVar2.as_mut_ptr(),
                            (*sShopData).totalCost as i32,
                            STR_CONV_MODE_LEFT_ALIGN,
                            6,
                        );
                        if sMartInfo.martType == MART_TYPE_DECOR {
                            StringExpandPlaceholders(
                                gStringVar4.as_mut_ptr(),
                                (*(&raw const crate::data::strings::gText_Var1IsItThatllBeVar2)
                                    .cast::<CArray<u8, 0>>())
                                .as_ptr()
                                .cast_mut(),
                            );
                        } else {
                            StringExpandPlaceholders(
                                gStringVar4.as_mut_ptr(),
                                (*(&raw const crate::data::strings::gText_YouWantedVar1ThatllBeVar2).cast::<CArray<u8, 0>>()).as_ptr().cast_mut(),
                            );
                        }
                        BuyMenuDisplayMessage(
                            taskId,
                            gStringVar4.as_mut_ptr(),
                            Some(BuyMenuConfirmPurchase),
                        );
                    }
                }
            }
        }
    }
}
pub(crate) unsafe fn Task_BuyHowManyDialogueInit(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let quantityInBag: u16 = CountTotalItemQuantityInBag(*data.at(5) as u16);
    DrawStdFrameWithCustomTileAndPalette(WIN_QUANTITY_IN_BAG, FALSE, 1, 13);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        quantityInBag as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_InBagVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BuyMenuPrint(
        WIN_QUANTITY_IN_BAG,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        0,
        COLORID_NORMAL,
    );
    *data.at(1) = 1;
    DrawStdFrameWithCustomTileAndPalette(WIN_QUANTITY_PRICE, FALSE, 1, 13);
    BuyMenuPrintItemQuantityAndPrice(taskId);
    ScheduleBgCopyTilemapToVram(0);
    let maxQuantity: u16 = div_u32(
        GetMoney(&raw mut (*gSaveBlock1Ptr).money),
        (*sShopData).totalCost,
    ) as u16;
    if maxQuantity > MAX_BAG_ITEM_CAPACITY {
        (*sShopData).maxQuantity = MAX_BAG_ITEM_CAPACITY as u8;
    } else {
        (*sShopData).maxQuantity = maxQuantity as u8;
    }
    task_set_func(taskId, Some(Task_BuyHowManyDialogueHandleInput));
}
pub(crate) unsafe fn Task_BuyHowManyDialogueHandleInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if AdjustQuantityAccordingToDPadInput(data.at(1), (*sShopData).maxQuantity as u16) == TRUE {
        (*sShopData).totalCost = shr_i32(
            GetItemPrice(*data.at(5) as u16) as i32,
            IsPokeNewsActive(POKENEWS_SLATEPORT) as u32,
        ) as u32
            * *data.at(1) as u32;
        BuyMenuPrintItemQuantityAndPrice(taskId);
    } else {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            PlaySE(SE_SELECT);
            ClearStdWindowAndFrameToTransparent(WIN_QUANTITY_PRICE, FALSE);
            ClearStdWindowAndFrameToTransparent(WIN_QUANTITY_IN_BAG, FALSE);
            ClearWindowTilemap(WIN_QUANTITY_PRICE);
            ClearWindowTilemap(WIN_QUANTITY_IN_BAG);
            PutWindowTilemap(WIN_ITEM_LIST);
            CopyItemName(*data.at(5) as u16, gStringVar1.as_mut_ptr());
            ConvertIntToDecimalStringN(
                gStringVar2.as_mut_ptr(),
                *data.at(1) as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                BAG_ITEM_CAPACITY_DIGITS,
            );
            ConvertIntToDecimalStringN(
                gStringVar3.as_mut_ptr(),
                (*sShopData).totalCost as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                6,
            );
            BuyMenuDisplayMessage(
                taskId,
                (*(&raw const crate::data::strings::gText_Var1AndYouWantedVar2)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(BuyMenuConfirmPurchase),
            );
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            ClearStdWindowAndFrameToTransparent(WIN_QUANTITY_PRICE, FALSE);
            ClearStdWindowAndFrameToTransparent(WIN_QUANTITY_IN_BAG, FALSE);
            ClearWindowTilemap(WIN_QUANTITY_PRICE);
            ClearWindowTilemap(WIN_QUANTITY_IN_BAG);
            BuyMenuReturnToItemList(taskId);
        }
    }
}
pub(crate) unsafe fn BuyMenuConfirmPurchase(taskId: u8) {
    CreateYesNoMenuWithCallbacks(
        taskId,
        (&raw const *sShopBuyMenuYesNoWindowTemplates).cast_mut(),
        1,
        0,
        0,
        1,
        13,
        (&raw const *sShopPurchaseYesNoFuncs).cast_mut(),
    );
}
pub(crate) unsafe fn BuyMenuTryMakePurchase(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    PutWindowTilemap(WIN_ITEM_LIST);
    if sMartInfo.martType == MART_TYPE_NORMAL {
        if AddBagItem(*data.at(5) as u16, *data.at(1) as u16) == TRUE {
            BuyMenuDisplayMessage(
                taskId,
                (*(&raw const crate::data::strings::gText_HereYouGoThankYou)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(BuyMenuSubtractMoney),
            );
            RecordItemPurchase(taskId);
        } else {
            BuyMenuDisplayMessage(
                taskId,
                (*(&raw const crate::data::strings::gText_NoMoreRoomForThis)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(BuyMenuReturnToItemList),
            );
        }
    } else {
        if DecorationAdd(*data.at(5) as u8) != 0 {
            if sMartInfo.martType == MART_TYPE_DECOR {
                BuyMenuDisplayMessage(
                    taskId,
                    (*(&raw const crate::data::strings::gText_ThankYouIllSendItHome)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    Some(BuyMenuSubtractMoney),
                );
            } else {
                BuyMenuDisplayMessage(
                    taskId,
                    (*(&raw const crate::data::strings::gText_ThanksIllSendItHome)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    Some(BuyMenuSubtractMoney),
                );
            }
        } else {
            BuyMenuDisplayMessage(
                taskId,
                (*(&raw const crate::data::strings::gText_SpaceForVar1Full)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(BuyMenuReturnToItemList),
            );
        }
    }
}
pub(crate) unsafe fn BuyMenuSubtractMoney(taskId: u8) {
    IncrementGameStat(GAME_STAT_SHOPPED);
    RemoveMoney(&raw mut (*gSaveBlock1Ptr).money, (*sShopData).totalCost);
    PlaySE(SE_SHOP);
    PrintMoneyAmountInMoneyBox(
        WIN_MONEY,
        GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32,
        0,
    );
    if sMartInfo.martType == MART_TYPE_NORMAL {
        task_set_func(taskId, Some(Task_ReturnToItemListAfterItemPurchase));
    } else {
        task_set_func(taskId, Some(Task_ReturnToItemListAfterDecorationPurchase));
    }
}
pub(crate) unsafe fn Task_ReturnToItemListAfterItemPurchase(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & 3 != 0 {
        PlaySE(SE_SELECT);
        if *data.at(5) == ITEM_POKE_BALL as i16
            && *data.at(1) >= 10
            && AddBagItem(ITEM_PREMIER_BALL, 1) == 1
        {
            BuyMenuDisplayMessage(
                taskId,
                (*(&raw const crate::data::strings::gText_ThrowInPremierBall)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
                Some(BuyMenuReturnToItemList),
            );
        } else {
            BuyMenuReturnToItemList(taskId);
        }
    }
}
pub(crate) unsafe fn Task_ReturnToItemListAfterDecorationPurchase(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        PlaySE(SE_SELECT);
        BuyMenuReturnToItemList(taskId);
    }
}
pub(crate) unsafe fn BuyMenuReturnToItemList(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ClearDialogWindowAndFrameToTransparent(WIN_MESSAGE, FALSE);
    BuyMenuPrintCursor(*data.at(7) as u8, COLORID_ITEM_LIST);
    PutWindowTilemap(WIN_ITEM_LIST);
    PutWindowTilemap(WIN_ITEM_DESCRIPTION);
    ScheduleBgCopyTilemapToVram(0);
    BuyMenuAddScrollIndicatorArrows();
    task_set_func(taskId, Some(Task_BuyMenu));
}
unsafe fn BuyMenuPrintItemQuantityAndPrice(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    FillWindowPixelBuffer(WIN_QUANTITY_PRICE, 17);
    PrintMoneyAmount(
        WIN_QUANTITY_PRICE,
        38,
        1,
        (*sShopData).totalCost as i32,
        TEXT_SKIP_DRAW,
    );
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        *data.at(1) as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        BAG_ITEM_CAPACITY_DIGITS,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    BuyMenuPrint(
        WIN_QUANTITY_PRICE,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        0,
        COLORID_NORMAL,
    );
}
unsafe fn ExitBuyMenu(taskId: u8) {
    gFieldCallback = Some(MapPostLoadHook_ReturnToShopMenu);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_ExitBuyMenu));
}
pub(crate) unsafe fn Task_ExitBuyMenu(taskId: u8) {
    if gPaletteFade.active() == 0 {
        RemoveMoneyLabelObject();
        BuyMenuFreeMemory();
        SetMainCallback2(Some(CB2_ReturnToField));
        DestroyTask(taskId);
    }
}
unsafe fn ClearItemPurchases() {
    sPurchaseHistoryId.set(0);
    memset(gMartPurchaseHistory.as_mut_ptr() as *mut u8, 0, 12);
}
unsafe fn RecordItemPurchase(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    for i in 0..3u16 {
        if gMartPurchaseHistory[i].itemId as i32 == *data.at(5) as i32
            && gMartPurchaseHistory[i].quantity != 0
        {
            if gMartPurchaseHistory[i].quantity as i32 + *data.at(1) as i32 > 255 {
                gMartPurchaseHistory[i].quantity = 255;
            } else {
                gMartPurchaseHistory[i].quantity += *data.at(1) as u16;
            }
            return;
        }
    }
    if sPurchaseHistoryId.get() < 3 {
        gMartPurchaseHistory[sPurchaseHistoryId.get()].itemId = *data.at(5) as u16;
        gMartPurchaseHistory[sPurchaseHistoryId.get()].quantity = *data.at(1) as u16;
        sPurchaseHistoryId.set(sPurchaseHistoryId.get() + 1);
    }
}
pub unsafe fn CreatePokemartMenu(itemsForSale: *mut u16) {
    CreateShopMenu(MART_TYPE_NORMAL);
    SetShopItemsForSale(itemsForSale);
    ClearItemPurchases();
    SetShopMenuCallback(Some(ScriptContext_Enable));
}
pub unsafe fn CreateDecorationShop1Menu(itemsForSale: *mut u16) {
    CreateShopMenu(MART_TYPE_DECOR);
    SetShopItemsForSale(itemsForSale);
    SetShopMenuCallback(Some(ScriptContext_Enable));
}
pub unsafe fn CreateDecorationShop2Menu(itemsForSale: *mut u16) {
    CreateShopMenu(MART_TYPE_DECOR2);
    SetShopItemsForSale(itemsForSale);
    SetShopMenuCallback(Some(ScriptContext_Enable));
}
