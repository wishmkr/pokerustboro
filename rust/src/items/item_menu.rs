//! Translated from `src/item_menu.c` by tools/rustport/c2rs.py.
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
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::apprentice::Apprentice_ScriptContext_Enable;
use crate::battle_controller_player::CB2_SetUpReshowBattleScreenAfterMenu2;
use crate::battle_pike::InBattlePike;
use crate::battle_pyramid::CurrentBattlePyramidLocation;
use crate::battle_pyramid_bag::GoToBattlePyramidBagMenu;
use crate::berry_tag_screen::DoBerryTagScreen;
use crate::bg::{
    ChangeBgY_ScreenOff, FillBgTilemapBufferRect_Palette0, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::FreezeObjectEvents;
use crate::ffi::{gSpecialVar_0x8005, gSpecialVar_Result};
use crate::field_player_avatar::{PlayerFreeze, StopPlayerAvatar, TestPlayerAvatarFlags};
use crate::field_specials::InMultiPartnerRoom;
use crate::gpu_regs::SetGpuReg;
use crate::item::gBagPockets;
use crate::item::{
    AddBagItem, AddPCItem, BagGetItemIdByPocketPosition, BagGetQuantityByPocketPosition,
    CheckBagHasItem, ClearItemSlots, CompactItemsInBagPocket, CopyItemName, GetItemBattleFunc,
    GetItemBattleUsage, GetItemFieldFunc, GetItemImportance, GetItemPrice, GetItemType,
    MoveItemSlotInList, RemoveBagItem, SortBerriesOrTMHMs,
};
use crate::item_menu_icons::{
    AddBagItemIconSprite, AddBagVisualSprite, AddSwitchPocketRotatingBallSprite,
    CreateItemMenuSwapLine, RemoveBagItemIconSprite, RemoveBagSprite, SetBagVisualPocketId,
    SetItemMenuSwapLineInvisibility, ShakeBagSprite, UpdateItemMenuSwapLinePos,
};
use crate::item_use::ItemUseOutOfBattle_Berry;
use crate::lilycove_lady::{
    FieldCallback_FavorLadyEnableScriptContexts, FieldCallback_QuizLadyEnableScriptContexts,
};
use crate::list_menu::{
    AddScrollIndicatorArrowPair, AddScrollIndicatorArrowPairParameterized, DestroyListMenuTask,
    ListMenu_ProcessInput, ListMenuGetScrollAndRow, ListMenuGetYCoordForPrintingArrowCursor,
    ListMenuInit, ListMenuSetTemplateField, RemoveScrollIndicatorArrowPair,
    gMultiuseListMenuTemplate,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::mail_data::ItemIsMail;
use crate::map_name_popup::HideMapNamePopUpWindow;
use crate::menu::{
    AddTextPrinterParameterized4, BlitMenuInfoIcon, ChangeMenuGridCursorPosition,
    ClearDialogWindowAndFrameToTransparent, ClearScheduledBgCopiesToVram,
    ClearStdWindowAndFrameToTransparent, DecompressAndCopyTileDataToVram,
    DoScheduledBgTilemapCopiesToVram, DrawStdFrameWithCustomTileAndPalette,
    FreeTempTileDataBuffersIfPossible, GetPlayerTextSpeedDelay, InitMenuActionGrid,
    InitMenuInUpperLeftCornerNormal, ListMenuLoadStdPalAt, Menu_GetCursorPos,
    Menu_ProcessInputNoWrap, PrintMenuActionGrid, PrintMenuActionTexts, ResetTempTileDataBuffers,
    ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{
    AdjustQuantityAccordingToDPadInput, CreateYesNoMenuWithCallbacks,
    DisplayMessageAndContinueTask, GetLRKeysPressed, IsHoldingItemAllowed, IsWritingMailAllowed,
    LoadListMenuSwapLineGfx, MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv,
    ResetAllBgsCoordinates, ResetVramOamAndBgCntRegs, SetCursorScrollWithinListBounds,
    SetCursorWithinListBounds, SetVBlankHBlankCallbacksToNull,
};
use crate::money::{
    AddMoney, AddMoneyLabelObject, GetMoney, PrintMoneyAmount, PrintMoneyAmountInMoneyBox,
    PrintMoneyAmountInMoneyBoxWithBorder, RemoveMoneyLabelObject,
};
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldContinueScript, CB2_ReturnToFieldWithOpenMenu,
    gFieldCallback,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::party_menu::{CB2_ChooseMonToGiveItem, ItemIdToBattleMoveId};
use crate::player_pc::CB2_PlayerPCExitBagMenu;
use crate::pokemon::CalculatePlayerPartyCount;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::script::LockPlayerFieldControls;
use crate::shop::CB2_ExitSellMenu;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks, SwitchTaskToFollowupFunc};
use crate::task::{gTasks, task_func, task_set, task_set_func};
use crate::text::{DeactivateAllTextPrinters, GetMenuCursorDimensionByFont};
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::InUnionRoom;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    FreeAllWindowBuffers, GetWindowAttribute, PutWindowTilemap, RemoveWindow,
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
/// `BlitBitmapToWindow` with this module's view of its types.
#[inline]
unsafe fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16) {
    unsafe {
        crate::window::BlitBitmapToWindow(a0, a1 as _, a2, a3, a4, a5);
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
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
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `ScriptContext_SetupScript` with this module's view of its types.
#[inline]
unsafe fn ScriptContext_SetupScript(a0: *mut u8) {
    unsafe {
        crate::script::ScriptContext_SetupScript(a0 as _);
    }
}
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
unsafe fn SetTaskFuncWithFollowupFunc(
    a0: u8,
    a1: Option<unsafe fn(u8)>,
    a2: Option<unsafe fn(u8)>,
) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
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
const tListTaskId: usize = 0;
const tNeverRead: usize = 3;
const tUsingRegisteredKeyItem: usize = 3;
const tItemCount: usize = 8;
// Data tables (translate with cdata.py): sBgTemplates_ItemMenu sItemListMenu sItemMenuActions sContextMenuItems_ItemsPocket sContextMenuItems_KeyItemsPocket sContextMenuItems_BallsPocket sContextMenuItems_TmHmPocket sContextMenuItems_BerriesPocket sContextMenuItems_BattleUse sContextMenuItems_Give sContextMenuItems_Cancel sContextMenuItems_BerryBlenderCrush sContextMenuItems_Apprentice sContextMenuItems_FavorLady sContextMenuItems_QuizLady sContextMenuFuncs sYesNoTossFunctions sYesNoSellItemFunctions sBagScrollArrowsTemplate sRegisteredSelect_Gfx sFontColorTable sDefaultBagWindows sContextMenuWindowTemplates

/// `struct ListBuffer1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ListBuffer1 {
    pub subBuffers: CArray<ListMenuItem, 65>,
}

unsafe impl Sync for ListBuffer1 {}

/// `struct ListBuffer2`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ListBuffer2 {
    pub name: CArray<CArray<u8, 24>, 65>,
}

unsafe impl Sync for ListBuffer2 {}

/// `struct TempWallyBag`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TempWallyBag {
    pub bagPocket_Items: CArray<ItemSlot, 30>,
    pub bagPocket_PokeBalls: CArray<ItemSlot, 16>,
    pub cursorPosition: CArray<u16, 5>,
    pub scrollPosition: CArray<u16, 5>,
    pub unused: u16,
    pub pocket: u16,
}

unsafe impl Sync for TempWallyBag {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ListBuffer1>() == 520);
    assert!(offset_of!(ListBuffer1, subBuffers) == 0);
    assert!(size_of::<ListBuffer2>() == 1560);
    assert!(offset_of!(ListBuffer2, name) == 0);
    assert!(size_of::<TempWallyBag>() == 208);
    assert!(offset_of!(TempWallyBag, bagPocket_Items) == 0);
    assert!(offset_of!(TempWallyBag, bagPocket_PokeBalls) == 120);
    assert!(offset_of!(TempWallyBag, cursorPosition) == 184);
    assert!(offset_of!(TempWallyBag, scrollPosition) == 194);
    assert!(offset_of!(TempWallyBag, unused) == 204);
    assert!(offset_of!(TempWallyBag, pocket) == 206);
};

const ACTION_CANCEL: i32 = 4;
const ACTION_CHECK: u8 = 6;
const ACTION_DESELECT: u8 = 8;
const ACTION_DUMMY: u8 = 14;
const ACTION_WALK: u8 = 7;
const COLORID_GRAY_CURSOR: u8 = 2;
const COLORID_NONE: u8 = 255;
const COLORID_NORMAL: u8 = 0;
const COLORID_POCKET_NAME: u8 = 1;
const COLORID_TMHM_INFO: u8 = 4;
const MAX_ITEMS_SHOWN: u8 = 8;
const NOT_SWAPPING: u8 = 255;
const SWITCH_POCKET_LEFT: u8 = 1;
const SWITCH_POCKET_NONE: u8 = 0;
const SWITCH_POCKET_RIGHT: u8 = 2;
const TAG_POCKET_SCROLL_ARROW: i32 = 110;
const WALLY_BAG_DELAY: i16 = 102;
const WIN_DESCRIPTION: u8 = 1;
const WIN_ITEM_LIST: u8 = 0;
const WIN_POCKET_NAME: u8 = 2;
const WIN_TMHM_INFO: u8 = 4;
const WIN_TMHM_INFO_ICONS: u8 = 3;

static sBagScrollArrowsTemplate: Table<ScrollArrowsTemplate> =
    Table((&raw const crate::data::item_menu::sBagScrollArrowsTemplate).cast());
static sBgTemplates_ItemMenu: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::item_menu::sBgTemplates_ItemMenu).cast());
static sContextMenuFuncs: Table<CArray<Option<unsafe fn(u8)>, 12>> =
    Table((&raw const crate::data::item_menu::sContextMenuFuncs).cast());
static sContextMenuItems_Apprentice: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_Apprentice).cast());
static sContextMenuItems_BallsPocket: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_BallsPocket).cast());
static sContextMenuItems_BattleUse: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_BattleUse).cast());
static sContextMenuItems_BerriesPocket: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_BerriesPocket).cast());
static sContextMenuItems_BerryBlenderCrush: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_BerryBlenderCrush).cast());
static sContextMenuItems_Cancel: Table<CArray<u8, 1>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_Cancel).cast());
static sContextMenuItems_FavorLady: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_FavorLady).cast());
static sContextMenuItems_Give: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_Give).cast());
static sContextMenuItems_ItemsPocket: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_ItemsPocket).cast());
static sContextMenuItems_KeyItemsPocket: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_KeyItemsPocket).cast());
static sContextMenuItems_QuizLady: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_QuizLady).cast());
static sContextMenuItems_TmHmPocket: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_menu::sContextMenuItems_TmHmPocket).cast());
static sContextMenuWindowTemplates: Table<CArray<WindowTemplate, 10>> =
    Table((&raw const crate::data::item_menu::sContextMenuWindowTemplates).cast());
static sDefaultBagWindows: Table<CArray<WindowTemplate, 7>> =
    Table((&raw const crate::data::item_menu::sDefaultBagWindows).cast());
static sFontColorTable: Table<CArray<CArray<u8, 3>, 5>> =
    Table((&raw const crate::data::item_menu::sFontColorTable).cast());
static sItemListMenu: Table<ListMenuTemplate> =
    Table((&raw const crate::data::item_menu::sItemListMenu).cast());
static sItemMenuActions: Table<CArray<MenuAction, 15>> =
    Table((&raw const crate::data::item_menu::sItemMenuActions).cast());
static sRegisteredSelect_Gfx: Table<CArray<u8, 192>> =
    Table((&raw const crate::data::item_menu::sRegisteredSelect_Gfx).cast());
static sYesNoSellItemFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::item_menu::sYesNoSellItemFunctions).cast());
static sYesNoTossFunctions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::item_menu::sYesNoTossFunctions).cast());

#[unsafe(link_section = "ewram_data")]
pub static mut gBagMenu: *mut BagMenu = null_mut();
#[unsafe(link_section = "ewram_data")]
pub static mut gBagPosition: BagPosition = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sListBuffer1: *mut ListBuffer1 = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sListBuffer2: *mut ListBuffer2 = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gSpecialVar_ItemId: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTempWallyBag: *mut TempWallyBag = null_mut();

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
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
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

#[unsafe(no_mangle)]
pub unsafe fn ResetBagScrollPositions() {
    gBagPosition.pocket = ITEMS_POCKET;
    memset(gBagPosition.cursorPosition.as_mut_ptr() as *mut u8, 0, 10);
    memset(gBagPosition.scrollPosition.as_mut_ptr() as *mut u8, 0, 10);
}
pub unsafe fn CB2_BagMenuFromStartMenu() {
    GoToBagMenu(
        ITEMMENULOCATION_FIELD,
        POCKETS_COUNT,
        Some(CB2_ReturnToFieldWithOpenMenu),
    );
}
pub unsafe fn CB2_BagMenuFromBattle() {
    if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        GoToBagMenu(
            ITEMMENULOCATION_BATTLE,
            POCKETS_COUNT,
            Some(CB2_SetUpReshowBattleScreenAfterMenu2),
        );
    } else {
        GoToBattlePyramidBagMenu(
            PYRAMIDBAG_LOC_BATTLE,
            Some(CB2_SetUpReshowBattleScreenAfterMenu2),
        );
    }
}
pub unsafe fn CB2_ChooseBerry() {
    GoToBagMenu(
        ITEMMENULOCATION_BERRY_TREE,
        BERRIES_POCKET,
        Some(CB2_ReturnToFieldContinueScript),
    );
}
pub unsafe fn ChooseBerryForMachine(exitCallback: Option<unsafe fn()>) {
    GoToBagMenu(
        ITEMMENULOCATION_BERRY_BLENDER_CRUSH,
        BERRIES_POCKET,
        exitCallback,
    );
}
pub unsafe fn CB2_GoToSellMenu() {
    GoToBagMenu(ITEMMENULOCATION_SHOP, POCKETS_COUNT, Some(CB2_ExitSellMenu));
}
pub unsafe fn CB2_GoToItemDepositMenu() {
    GoToBagMenu(
        ITEMMENULOCATION_ITEMPC,
        POCKETS_COUNT,
        Some(CB2_PlayerPCExitBagMenu),
    );
}
pub unsafe fn ApprenticeOpenBagMenu() {
    GoToBagMenu(
        ITEMMENULOCATION_APPRENTICE,
        POCKETS_COUNT,
        Some(CB2_ApprenticeExitBagMenu),
    );
    gSpecialVar_0x8005 = ITEM_NONE;
    gSpecialVar_Result = FALSE as u16;
}
pub unsafe fn FavorLadyOpenBagMenu() {
    GoToBagMenu(
        ITEMMENULOCATION_FAVOR_LADY,
        POCKETS_COUNT,
        Some(CB2_FavorLadyExitBagMenu),
    );
    gSpecialVar_Result = FALSE as u16;
}
pub unsafe fn QuizLadyOpenBagMenu() {
    GoToBagMenu(
        ITEMMENULOCATION_QUIZ_LADY,
        POCKETS_COUNT,
        Some(CB2_QuizLadyExitBagMenu),
    );
    gSpecialVar_Result = FALSE as u16;
}
pub unsafe fn GoToBagMenu(location: u8, pocket: u8, exitCallback: Option<unsafe fn()>) {
    gBagMenu = AllocZeroed(3144) as *mut BagMenu;
    if gBagMenu.is_null() {
        SetMainCallback2(exitCallback);
    } else {
        if location != ITEMMENULOCATION_LAST {
            gBagPosition.location = location;
        }
        if exitCallback.is_some() {
            gBagPosition.exitCallback = exitCallback;
        }
        if pocket < POCKETS_COUNT {
            gBagPosition.pocket = pocket;
        }
        if gBagPosition.location == ITEMMENULOCATION_BERRY_TREE
            || gBagPosition.location == ITEMMENULOCATION_BERRY_BLENDER_CRUSH
        {
            (*gBagMenu).set_pocketSwitchDisabled(TRUE);
        }
        (*gBagMenu).newScreenCallback = None;
        (*gBagMenu).toSwapPos = NOT_SWAPPING;
        (*gBagMenu).pocketScrollArrowsTask = TASK_NONE;
        (*gBagMenu).pocketSwitchArrowsTask = TASK_NONE;
        memset((*gBagMenu).spriteIds.as_mut_ptr(), SPRITE_NONE as i32, 12);
        memset((*gBagMenu).windowIds.as_mut_ptr(), WINDOW_NONE as i32, 10);
        SetMainCallback2(Some(CB2_Bag));
    }
}
pub unsafe fn CB2_BagMenuRun() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub unsafe fn VBlankCB_BagMenuRun() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_Bag() {
    while MenuHelpers_ShouldWaitForLinkRecv() != TRUE
        && SetupBagMenu() != TRUE
        && MenuHelpers_IsLinkActive() != TRUE
    {}
}
unsafe fn SetupBagMenu() -> u8 {
    let mut taskId: u8 = 0;
    'l1: {
        match gMain.state {
            0 => {
                SetVBlankHBlankCallbacksToNull();
                ClearScheduledBgCopiesToVram();
                gMain.state += 1;
            }
            1 => {
                ScanlineEffect_Stop();
                gMain.state += 1;
            }
            2 => {
                FreeAllSpritePalettes();
                gMain.state += 1;
            }
            3 => {
                ResetPaletteFade();
                gPaletteFade.set_bufferTransferDisabled(TRUE as u16);
                gMain.state += 1;
            }
            4 => {
                ResetSpriteData();
                gMain.state += 1;
            }
            5 => {
                gMain.state += 1;
            }
            6 => {
                if MenuHelpers_IsLinkActive() == 0 {
                    ResetTasks();
                }
                gMain.state += 1;
            }
            7 => {
                BagMenu_InitBGs();
                (*gBagMenu).graphicsLoadState = 0;
                gMain.state += 1;
            }
            8 => {
                if LoadBagMenu_Graphics() == 0 {
                    break 'l1;
                }
                gMain.state += 1;
            }
            9 => {
                LoadBagMenuTextWindows();
                gMain.state += 1;
            }
            10 => {
                UpdatePocketItemLists();
                InitPocketListPositions();
                InitPocketScrollPositions();
                gMain.state += 1;
            }
            11 => {
                AllocateBagItemListBuffers();
                gMain.state += 1;
            }
            12 => {
                LoadBagItemListBuffers(gBagPosition.pocket);
                gMain.state += 1;
            }
            13 => {
                PrintPocketNames(
                    (*(&raw const crate::data::strings::gPocketNamesStringsTable).cast::<CArray<
                        *mut u8,
                        0,
                    >>(
                    ))[gBagPosition.pocket],
                    null_mut(),
                );
                CopyPocketNameToWindow(0);
                DrawPocketIndicatorSquare(gBagPosition.pocket, TRUE);
                gMain.state += 1;
            }
            14 => {
                taskId = CreateBagInputHandlerTask(gBagPosition.location);
                task_set(
                    taskId,
                    tListTaskId,
                    ListMenuInit(
                        &raw mut gMultiuseListMenuTemplate,
                        gBagPosition.scrollPosition[gBagPosition.pocket],
                        gBagPosition.cursorPosition[gBagPosition.pocket],
                    ) as i16,
                );
                task_set(taskId, tNeverRead, 0);
                task_set(taskId, tItemCount, 0);
                gMain.state += 1;
            }
            15 => {
                AddBagVisualSprite(gBagPosition.pocket);
                gMain.state += 1;
            }
            16 => {
                CreateItemMenuSwapLine();
                gMain.state += 1;
            }
            17 => {
                CreatePocketScrollArrowPair();
                CreatePocketSwitchArrowPair();
                gMain.state += 1;
            }
            18 => {
                PrepareTMHMMoveWindow();
                gMain.state += 1;
            }
            19 => {
                BlendPalettes(PALETTES_ALL, 16, 0);
                gMain.state += 1;
            }
            20 => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
                gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
                gMain.state += 1;
            }
            _ => {
                SetVBlankCallback(Some(VBlankCB_BagMenuRun));
                SetMainCallback2(Some(CB2_BagMenuRun));
                return TRUE;
            }
        }
    }
    FALSE
}
unsafe fn BagMenu_InitBGs() {
    ResetVramOamAndBgCntRegs();
    memset((*gBagMenu).tilemapBuffer.as_mut_ptr(), 0, 2048);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates_ItemMenu.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(2, (*gBagMenu).tilemapBuffer.as_mut_ptr() as *mut c_void);
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadBagMenu_Graphics() -> u8 {
    match (*gBagMenu).graphicsLoadState {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                2,
                (*(&raw const crate::data::graphics::gBagScreen_Gfx).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*gBagMenu).graphicsLoadState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gBagScreen_GfxTileMap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    (*gBagMenu).tilemapBuffer.as_mut_ptr() as *mut c_void,
                );
                (*gBagMenu).graphicsLoadState += 1;
            }
        }
        2 => {
            if IsWallysBag() == 0 && (*gSaveBlock2Ptr).playerGender != MALE {
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBagScreenFemale_Pal)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0,
                    64,
                );
            } else {
                LoadCompressedPalette(
                    (*(&raw const crate::data::graphics::gBagScreenMale_Pal)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    0,
                    64,
                );
            }
            (*gBagMenu).graphicsLoadState += 1;
        }
        3 => {
            if IsWallysBag() == TRUE || (*gSaveBlock2Ptr).playerGender == MALE {
                LoadCompressedSpriteSheet(
                    (&raw const (*(&raw const crate::data::item_menu_icons::gBagMaleSpriteSheet)
                        .cast::<CompressedSpriteSheet>()))
                        .cast_mut(),
                );
            } else {
                LoadCompressedSpriteSheet((&raw const (*(&raw const crate::data::item_menu_icons::gBagFemaleSpriteSheet).cast::<CompressedSpriteSheet>())).cast_mut());
            }
            (*gBagMenu).graphicsLoadState += 1;
        }
        4 => {
            LoadCompressedSpritePalette(
                (&raw const (*(&raw const crate::data::item_menu_icons::gBagPaletteTable)
                    .cast::<CompressedSpritePalette>()))
                    .cast_mut(),
            );
            (*gBagMenu).graphicsLoadState += 1;
        }
        _ => {
            LoadListMenuSwapLineGfx();
            (*gBagMenu).graphicsLoadState = 0;
            return TRUE;
        }
    }
    FALSE
}
unsafe fn CreateBagInputHandlerTask(location: u8) -> u8 {
    let mut taskId: u8 = 0;
    if location == ITEMMENULOCATION_WALLY {
        taskId = CreateTask(Some(Task_WallyTutorialBagMenu), 0);
    } else {
        taskId = CreateTask(Some(Task_BagMenu_HandleInput), 0);
    }
    taskId
}
unsafe fn AllocateBagItemListBuffers() {
    sListBuffer1 = Alloc(520) as *mut ListBuffer1;
    sListBuffer2 = Alloc(1560) as *mut ListBuffer2;
}
unsafe fn LoadBagItemListBuffers(pocketId: u8) {
    let mut i: u16 = 0;
    let pocket: *mut BagPocket = &raw mut (*(&raw const crate::item::gBagPockets)
        .cast::<CArray<BagPocket, 0>>()
        .cast_mut())[pocketId];
    let mut subBuffer: *mut ListMenuItem = null_mut();
    if (*gBagMenu).hideCloseBagText() == 0 {
        i = 0;
        while (i as i32) < (*gBagMenu).numItemStacks[pocketId] as i32 - 1 {
            GetItemNameFromPocket(
                (*sListBuffer2).name[i].as_mut_ptr(),
                (*(*pocket).itemSlots.at(i)).itemId,
            );
            subBuffer = (*sListBuffer1).subBuffers.as_mut_ptr();
            (*subBuffer.at(i)).name = (*sListBuffer2).name[i].as_mut_ptr();
            (*subBuffer.at(i)).id = i as i32;
            i += 1;
        }
        StringCopy(
            (*sListBuffer2).name[i].as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_CloseBag).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        subBuffer = (*sListBuffer1).subBuffers.as_mut_ptr();
        (*subBuffer.at(i)).name = (*sListBuffer2).name[i].as_mut_ptr();
        (*subBuffer.at(i)).id = LIST_CANCEL;
    } else {
        i = 0;
        while i < (*gBagMenu).numItemStacks[pocketId] as u16 {
            GetItemNameFromPocket(
                (*sListBuffer2).name[i].as_mut_ptr(),
                (*(*pocket).itemSlots.at(i)).itemId,
            );
            subBuffer = (*sListBuffer1).subBuffers.as_mut_ptr();
            (*subBuffer.at(i)).name = (*sListBuffer2).name[i].as_mut_ptr();
            (*subBuffer.at(i)).id = i as i32;
            i += 1;
        }
    }
    gMultiuseListMenuTemplate = *sItemListMenu;
    gMultiuseListMenuTemplate.totalItems = (*gBagMenu).numItemStacks[pocketId] as u16;
    gMultiuseListMenuTemplate.items = (*sListBuffer1).subBuffers.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = (*gBagMenu).numShownItems[pocketId] as u16;
}
unsafe fn GetItemNameFromPocket(dest: *mut u8, itemId: u16) {
    match gBagPosition.pocket {
        TMHM_POCKET => {
            StringCopy(
                gStringVar2.as_mut_ptr(),
                (*(&raw const crate::data::data_tables::gMoveNames)
                    .cast::<CArray<CArray<u8, 13>, 355>>())[ItemIdToBattleMoveId(itemId)]
                .as_ptr()
                .cast_mut(),
            );
            if itemId >= ITEM_HM01 {
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    itemId as i32 - ITEM_HM01 as i32 + 1,
                    STR_CONV_MODE_LEADING_ZEROS,
                    1,
                );
                StringExpandPlaceholders(
                    dest,
                    (*(&raw const crate::data::strings::gText_NumberItem_HM)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            } else {
                ConvertIntToDecimalStringN(
                    gStringVar1.as_mut_ptr(),
                    itemId as i32 - ITEM_TM01 as i32 + 1,
                    STR_CONV_MODE_LEADING_ZEROS,
                    2,
                );
                StringExpandPlaceholders(
                    dest,
                    (*(&raw const crate::data::strings::gText_NumberItem_TMBerry)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
            }
        }
        BERRIES_POCKET => {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                itemId as i32 - ITEM_CHERI_BERRY as i32 + 1,
                STR_CONV_MODE_LEADING_ZEROS,
                2,
            );
            CopyItemName(itemId, gStringVar2.as_mut_ptr());
            StringExpandPlaceholders(
                dest,
                (*(&raw const crate::data::strings::gText_NumberItem_TMBerry)
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
        }
        _ => {
            CopyItemName(itemId, dest);
        }
    }
}
pub(crate) unsafe fn BagMenu_MoveCursorCallback(itemIndex: i32, onInit: u8, list: *mut ListMenu) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
        ShakeBagSprite();
    }
    if (*gBagMenu).toSwapPos == NOT_SWAPPING {
        RemoveBagItemIconSprite((*gBagMenu).itemIconSlot() ^ 1);
        if itemIndex != LIST_CANCEL {
            AddBagItemIconSprite(
                BagGetItemIdByPocketPosition(gBagPosition.pocket + 1, itemIndex as u16),
                (*gBagMenu).itemIconSlot(),
            );
        } else {
            AddBagItemIconSprite(ITEM_LIST_END, (*gBagMenu).itemIconSlot());
        }
        (*gBagMenu).set_itemIconSlot((*gBagMenu).itemIconSlot() ^ 1);
        if (*gBagMenu).inhibitItemDescriptionPrint() == 0 {
            PrintItemDescription(itemIndex);
        }
    }
}
pub(crate) unsafe fn BagMenu_ItemPrintCallback(windowId: u8, itemIndex: u32, y: u8) {
    let mut itemId: u16 = 0;
    let mut itemQuantity: u16 = 0;
    let mut offset: i32 = 0;
    if itemIndex != LIST_CANCEL as u32 {
        if (*gBagMenu).toSwapPos != NOT_SWAPPING {
            if (*gBagMenu).toSwapPos == itemIndex as u8 {
                BagMenu_PrintCursorAtPos(y, COLORID_GRAY_CURSOR);
            } else {
                BagMenu_PrintCursorAtPos(y, COLORID_NONE);
            }
        }
        itemId = BagGetItemIdByPocketPosition(gBagPosition.pocket + 1, itemIndex as u16);
        itemQuantity = BagGetQuantityByPocketPosition(gBagPosition.pocket + 1, itemIndex as u16);
        if (ITEM_HM01..=ITEM_HM08).contains(&itemId) {
            BlitBitmapToWindow(
                windowId,
                (*(&raw const crate::data::graphics::gBagMenuHMIcon_Gfx).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                8,
                y as u16 - 1,
                16,
                16,
            );
        }
        if gBagPosition.pocket == BERRIES_POCKET {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                itemQuantity as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                BERRY_CAPACITY_DIGITS,
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            offset = GetStringRightAlignXOffset(FONT_NARROW as i32, gStringVar4.as_mut_ptr(), 119);
            BagMenu_Print(
                windowId,
                FONT_NARROW,
                gStringVar4.as_mut_ptr(),
                offset as u8,
                y,
                0,
                0,
                TEXT_SKIP_DRAW,
                COLORID_NORMAL,
            );
        } else if gBagPosition.pocket != KEYITEMS_POCKET && GetItemImportance(itemId) == FALSE {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                itemQuantity as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                BAG_ITEM_CAPACITY_DIGITS,
            );
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            offset = GetStringRightAlignXOffset(FONT_NARROW as i32, gStringVar4.as_mut_ptr(), 119);
            BagMenu_Print(
                windowId,
                FONT_NARROW,
                gStringVar4.as_mut_ptr(),
                offset as u8,
                y,
                0,
                0,
                TEXT_SKIP_DRAW,
                COLORID_NORMAL,
            );
        } else {
            if (*gSaveBlock1Ptr).registeredItem != ITEM_NONE
                && (*gSaveBlock1Ptr).registeredItem == itemId
            {
                BlitBitmapToWindow(
                    windowId,
                    sRegisteredSelect_Gfx.as_ptr().cast_mut(),
                    96,
                    y as u16 - 1,
                    24,
                    16,
                );
            }
        }
    }
}
pub(crate) unsafe fn PrintItemDescription(itemIndex: i32) {
    let mut str: *mut u8 = null_mut();
    if itemIndex != LIST_CANCEL {
        str = GetItemDescription(BagGetItemIdByPocketPosition(
            gBagPosition.pocket + 1,
            itemIndex as u16,
        ));
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gBagMenu_ReturnToStrings)
                .cast::<CArray<*mut u8, 0>>())[gBagPosition.location],
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_ReturnToVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        str = gStringVar4.as_mut_ptr();
    }
    FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
    BagMenu_Print(
        WIN_DESCRIPTION,
        FONT_NORMAL,
        str,
        3,
        1,
        0,
        0,
        0,
        COLORID_NORMAL,
    );
}
unsafe fn BagMenu_PrintCursor(listTaskId: u8, colorIndex: u8) {
    BagMenu_PrintCursorAtPos(
        ListMenuGetYCoordForPrintingArrowCursor(listTaskId) as u8,
        colorIndex,
    );
}
unsafe fn BagMenu_PrintCursorAtPos(y: u8, colorIndex: u8) {
    if colorIndex == COLORID_NONE {
        FillWindowPixelRect(
            WIN_ITEM_LIST,
            0,
            0,
            y as u16,
            GetMenuCursorDimensionByFont(FONT_NORMAL, 0) as u16,
            GetMenuCursorDimensionByFont(FONT_NORMAL, 1) as u16,
        );
    } else {
        BagMenu_Print(
            WIN_ITEM_LIST,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_SelectorArrow2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            y,
            0,
            0,
            0,
            colorIndex,
        );
    }
}
unsafe fn CreatePocketScrollArrowPair() {
    if (*gBagMenu).pocketScrollArrowsTask == TASK_NONE {
        (*gBagMenu).pocketScrollArrowsTask = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            172,
            12,
            148,
            (*gBagMenu).numItemStacks[gBagPosition.pocket] as i32
                - (*gBagMenu).numShownItems[gBagPosition.pocket] as i32,
            TAG_POCKET_SCROLL_ARROW,
            TAG_POCKET_SCROLL_ARROW,
            &raw mut gBagPosition.scrollPosition[gBagPosition.pocket],
        );
    }
}
pub unsafe fn BagDestroyPocketScrollArrowPair() {
    if (*gBagMenu).pocketScrollArrowsTask != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*gBagMenu).pocketScrollArrowsTask);
        (*gBagMenu).pocketScrollArrowsTask = TASK_NONE;
    }
    DestroyPocketSwitchArrowPair();
}
unsafe fn CreatePocketSwitchArrowPair() {
    if (*gBagMenu).pocketSwitchDisabled() != TRUE && (*gBagMenu).pocketSwitchArrowsTask == TASK_NONE
    {
        (*gBagMenu).pocketSwitchArrowsTask = AddScrollIndicatorArrowPair(
            (&raw const *sBagScrollArrowsTemplate).cast_mut(),
            &raw mut gBagPosition.pocketSwitchArrowPos,
        );
    }
}
unsafe fn DestroyPocketSwitchArrowPair() {
    if (*gBagMenu).pocketSwitchArrowsTask != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*gBagMenu).pocketSwitchArrowsTask);
        (*gBagMenu).pocketSwitchArrowsTask = TASK_NONE;
    }
}
unsafe fn FreeBagMenu() {
    Free(sListBuffer2 as *mut c_void);
    Free(sListBuffer1 as *mut c_void);
    FreeAllWindowBuffers();
    Free(gBagMenu as *mut c_void);
}
pub unsafe fn Task_FadeAndCloseBagMenu(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_CloseBagMenu));
}
pub(crate) unsafe fn Task_CloseBagMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        DestroyListMenuTask(
            *data as u8,
            &raw mut gBagPosition.scrollPosition[gBagPosition.pocket],
            &raw mut gBagPosition.cursorPosition[gBagPosition.pocket],
        );
        if (*gBagMenu).newScreenCallback.is_some() {
            SetMainCallback2((*gBagMenu).newScreenCallback);
        } else {
            SetMainCallback2(gBagPosition.exitCallback);
        }
        BagDestroyPocketScrollArrowPair();
        ResetSpriteData();
        FreeAllSpritePalettes();
        FreeBagMenu();
        DestroyTask(taskId);
    }
}
pub unsafe fn UpdatePocketItemList(pocketId: u8) {
    let pocket: *mut BagPocket = &raw mut (*(&raw const crate::item::gBagPockets)
        .cast::<CArray<BagPocket, 0>>()
        .cast_mut())[pocketId];
    match pocketId {
        TMHM_POCKET | BERRIES_POCKET => {
            SortBerriesOrTMHMs(pocket);
        }
        _ => {
            CompactItemsInBagPocket(pocket);
        }
    }
    (*gBagMenu).numItemStacks[pocketId] = 0;
    let mut i: u16 = 0;
    while i < (*pocket).capacity as u16 && (*(*pocket).itemSlots.at(i)).itemId != 0 {
        (*gBagMenu).numItemStacks[pocketId] += 1;
        i += 1;
    }
    if (*gBagMenu).hideCloseBagText() == 0 {
        (*gBagMenu).numItemStacks[pocketId] += 1;
    }
    if (*gBagMenu).numItemStacks[pocketId] > MAX_ITEMS_SHOWN {
        (*gBagMenu).numShownItems[pocketId] = MAX_ITEMS_SHOWN;
    } else {
        (*gBagMenu).numShownItems[pocketId] = (*gBagMenu).numItemStacks[pocketId];
    }
}
unsafe fn UpdatePocketItemLists() {
    for i in 0..POCKETS_COUNT {
        UpdatePocketItemList(i);
    }
}
pub unsafe fn UpdatePocketListPosition(pocketId: u8) {
    SetCursorWithinListBounds(
        &raw mut gBagPosition.scrollPosition[pocketId],
        &raw mut gBagPosition.cursorPosition[pocketId],
        (*gBagMenu).numShownItems[pocketId],
        (*gBagMenu).numItemStacks[pocketId],
    );
}
unsafe fn InitPocketListPositions() {
    for i in 0..POCKETS_COUNT {
        UpdatePocketListPosition(i);
    }
}
unsafe fn InitPocketScrollPositions() {
    for i in 0..POCKETS_COUNT {
        SetCursorScrollWithinListBounds(
            &raw mut gBagPosition.scrollPosition[i],
            &raw mut gBagPosition.cursorPosition[i],
            (*gBagMenu).numShownItems[i],
            (*gBagMenu).numItemStacks[i],
            MAX_ITEMS_SHOWN,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetItemListPosition(pocketId: u8) -> u8 {
    gBagPosition.scrollPosition[pocketId] as u8 + gBagPosition.cursorPosition[pocketId] as u8
}
pub unsafe fn DisplayItemMessage(
    taskId: u8,
    fontId: u8,
    str: *mut u8,
    callback: Option<unsafe fn(u8)>,
) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(10) = AddItemMessageWindow(ITEMWIN_MESSAGE) as i16;
    FillWindowPixelBuffer(*data.at(10) as u8, 17);
    DisplayMessageAndContinueTask(
        taskId,
        *data.at(10) as u8,
        10,
        13,
        fontId,
        GetPlayerTextSpeedDelay(),
        str,
        core::mem::transmute::<Option<unsafe fn(u8)>, *mut c_void>(callback),
    );
    ScheduleBgCopyTilemapToVram(1);
}
pub unsafe fn CloseItemMessage(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    RemoveItemMessageWindow(ITEMWIN_MESSAGE);
    DestroyListMenuTask(*data as u8, scrollPos, cursorPos);
    UpdatePocketItemList(gBagPosition.pocket);
    UpdatePocketListPosition(gBagPosition.pocket);
    LoadBagItemListBuffers(gBagPosition.pocket);
    *data = ListMenuInit(&raw mut gMultiuseListMenuTemplate, *scrollPos, *cursorPos) as i16;
    ScheduleBgCopyTilemapToVram(0);
    ReturnToItemList(taskId);
}
unsafe fn AddItemQuantityWindow(windowType: u8) {
    PrintItemQuantity(BagMenu_AddWindow(windowType), 1);
}
pub(crate) unsafe fn PrintItemQuantity(windowId: u8, quantity: i16) {
    let numDigits: u8 = (if gBagPosition.pocket == 3 {
        3
    } else {
        BAG_ITEM_CAPACITY_DIGITS as i32
    }) as u8;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        quantity as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        numDigits,
    );
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
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x28) as u8,
        2,
        0,
        None,
    );
}
unsafe fn PrintItemSoldAmount(windowId: i32, numSold: i32, moneyEarned: i32) {
    let numDigits: u8 = (if gBagPosition.pocket == 3 {
        3
    } else {
        BAG_ITEM_CAPACITY_DIGITS as i32
    }) as u8;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        numSold,
        STR_CONV_MODE_LEADING_ZEROS,
        numDigits,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized(
        windowId as u8,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    PrintMoneyAmount(windowId as u8, 38, 1, moneyEarned, 0);
}
pub(crate) unsafe fn Task_BagMenu_HandleInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    let mut listPosition: i32 = 0;
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE && gPaletteFade.active() == 0 {
        match GetSwitchBagPocketDirection() {
            SWITCH_POCKET_LEFT => {
                SwitchBagPocket(taskId, MENU_CURSOR_DELTA_LEFT as i16, FALSE as u16);
                return;
            }
            SWITCH_POCKET_RIGHT => {
                SwitchBagPocket(taskId, MENU_CURSOR_DELTA_RIGHT as i16, FALSE as u16);
                return;
            }
            _ => {
                if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
                    if CanSwapItems() == TRUE {
                        ListMenuGetScrollAndRow(*data as u8, scrollPos, cursorPos);
                        if *scrollPos as i32 + *cursorPos as i32
                            != (*gBagMenu).numItemStacks[gBagPosition.pocket] as i32 - 1
                        {
                            PlaySE(SE_SELECT);
                            StartItemSwap(taskId);
                        }
                    }
                    return;
                }
            }
        }
        listPosition = ListMenu_ProcessInput(*data as u8);
        ListMenuGetScrollAndRow(*data as u8, scrollPos, cursorPos);
        'l2: {
            match listPosition {
                LIST_NOTHING_CHOSEN => {}
                LIST_CANCEL => {
                    if gBagPosition.location == ITEMMENULOCATION_BERRY_BLENDER_CRUSH {
                        PlaySE(SE_FAILURE);
                        break 'l2;
                    }
                    PlaySE(SE_SELECT);
                    gSpecialVar_ItemId = ITEM_NONE;
                    task_set_func(taskId, Some(Task_FadeAndCloseBagMenu));
                }
                _ => {
                    PlaySE(SE_SELECT);
                    BagDestroyPocketScrollArrowPair();
                    BagMenu_PrintCursor(*data as u8, COLORID_GRAY_CURSOR);
                    *data.at(1) = listPosition as i16;
                    *data.at(2) = BagGetQuantityByPocketPosition(
                        gBagPosition.pocket + 1,
                        listPosition as u16,
                    ) as i16;
                    gSpecialVar_ItemId =
                        BagGetItemIdByPocketPosition(gBagPosition.pocket + 1, listPosition as u16);
                    sContextMenuFuncs[gBagPosition.location].unwrap_unchecked()(taskId);
                }
            }
        }
    }
}
unsafe fn ReturnToItemList(taskId: u8) {
    CreatePocketScrollArrowPair();
    CreatePocketSwitchArrowPair();
    ClearWindowTilemap(WIN_TMHM_INFO_ICONS);
    ClearWindowTilemap(WIN_TMHM_INFO);
    PutWindowTilemap(WIN_DESCRIPTION);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(Task_BagMenu_HandleInput));
}
unsafe fn GetSwitchBagPocketDirection() -> u8 {
    if (*gBagMenu).pocketSwitchDisabled() != 0 {
        return SWITCH_POCKET_NONE;
    }
    let LRKeys: u8 = GetLRKeysPressed();
    if gMain.newKeys as i32 & DPAD_LEFT != 0 || LRKeys == MENU_L_PRESSED {
        PlaySE(SE_SELECT);
        return SWITCH_POCKET_LEFT;
    }
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 || LRKeys == MENU_R_PRESSED {
        PlaySE(SE_SELECT);
        return SWITCH_POCKET_RIGHT;
    }
    SWITCH_POCKET_NONE
}
unsafe fn ChangeBagPocketId(bagPocketId: *mut u8, deltaBagPocketId: i8) {
    if deltaBagPocketId == 1 && *bagPocketId == 4 {
        *bagPocketId = 0;
    } else if deltaBagPocketId == MENU_CURSOR_DELTA_LEFT && *bagPocketId == 0 {
        *bagPocketId = 4;
    } else {
        *bagPocketId += deltaBagPocketId as u8;
    }
}
unsafe fn SwitchBagPocket(taskId: u8, deltaBagPocketId: i16, skipEraseList: u16) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(13) = 0;
    *data.at(12) = 0;
    *data.at(11) = deltaBagPocketId;
    if skipEraseList == 0 {
        ClearWindowTilemap(WIN_ITEM_LIST);
        ClearWindowTilemap(WIN_DESCRIPTION);
        DestroyListMenuTask(
            *data as u8,
            &raw mut gBagPosition.scrollPosition[gBagPosition.pocket],
            &raw mut gBagPosition.cursorPosition[gBagPosition.pocket],
        );
        ScheduleBgCopyTilemapToVram(0);
        gSprites
            [(*gBagMenu).spriteIds[ITEMMENUSPRITE_ITEM + ((*gBagMenu).itemIconSlot() as i32 ^ 1)]]
            .set_invisible(1);
        BagDestroyPocketScrollArrowPair();
    }
    let mut newPocket: u8 = gBagPosition.pocket;
    ChangeBagPocketId(&raw mut newPocket, deltaBagPocketId as i8);
    if deltaBagPocketId == MENU_CURSOR_DELTA_RIGHT as i16 {
        PrintPocketNames(
            (*(&raw const crate::data::strings::gPocketNamesStringsTable)
                .cast::<CArray<*mut u8, 0>>())[gBagPosition.pocket],
            (*(&raw const crate::data::strings::gPocketNamesStringsTable)
                .cast::<CArray<*mut u8, 0>>())[newPocket],
        );
        CopyPocketNameToWindow(0);
    } else {
        PrintPocketNames(
            (*(&raw const crate::data::strings::gPocketNamesStringsTable)
                .cast::<CArray<*mut u8, 0>>())[newPocket],
            (*(&raw const crate::data::strings::gPocketNamesStringsTable)
                .cast::<CArray<*mut u8, 0>>())[gBagPosition.pocket],
        );
        CopyPocketNameToWindow(8);
    }
    DrawPocketIndicatorSquare(gBagPosition.pocket, FALSE);
    DrawPocketIndicatorSquare(newPocket, TRUE);
    FillBgTilemapBufferRect_Palette0(2, 11, 14, 2, 15, 16);
    ScheduleBgCopyTilemapToVram(2);
    SetBagVisualPocketId(newPocket, TRUE);
    RemoveBagSprite(ITEMMENUSPRITE_BALL);
    AddSwitchPocketRotatingBallSprite(deltaBagPocketId);
    SetTaskFuncWithFollowupFunc(taskId, Some(Task_SwitchBagPocket), task_func(taskId));
}
pub(crate) unsafe fn Task_SwitchBagPocket(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if MenuHelpers_IsLinkActive() == 0 && IsWallysBag() == 0 {
        match GetSwitchBagPocketDirection() {
            SWITCH_POCKET_LEFT => {
                ChangeBagPocketId(&raw mut gBagPosition.pocket, *data.at(11) as i8);
                SwitchTaskToFollowupFunc(taskId);
                SwitchBagPocket(taskId, MENU_CURSOR_DELTA_LEFT as i16, TRUE as u16);
                return;
            }
            SWITCH_POCKET_RIGHT => {
                ChangeBagPocketId(&raw mut gBagPosition.pocket, *data.at(11) as i8);
                SwitchTaskToFollowupFunc(taskId);
                SwitchBagPocket(taskId, 1, 1);
                return;
            }
            _ => {}
        }
    }
    match *data.at(13) {
        0 => {
            DrawItemListBgRow(*data.at(12) as u8);
            if ({
                *data.at(12) += 1;
                *data.at(12)
            }) as i32
                & 1
                == 0
            {
                if *data.at(11) == MENU_CURSOR_DELTA_RIGHT as i16 {
                    CopyPocketNameToWindow((*data.at(12) >> 1) as u8 as u32);
                } else {
                    CopyPocketNameToWindow(8 - (*data.at(12) >> 1) as u8 as u32);
                }
            }
            if *data.at(12) == 16 {
                *data.at(13) += 1;
            }
        }
        1 => {
            ChangeBagPocketId(&raw mut gBagPosition.pocket, *data.at(11) as i8);
            LoadBagItemListBuffers(gBagPosition.pocket);
            *data = ListMenuInit(
                &raw mut gMultiuseListMenuTemplate,
                gBagPosition.scrollPosition[gBagPosition.pocket],
                gBagPosition.cursorPosition[gBagPosition.pocket],
            ) as i16;
            PutWindowTilemap(WIN_DESCRIPTION);
            PutWindowTilemap(WIN_POCKET_NAME);
            ScheduleBgCopyTilemapToVram(0);
            CreatePocketScrollArrowPair();
            CreatePocketSwitchArrowPair();
            SwitchTaskToFollowupFunc(taskId);
        }
        _ => {}
    }
}
unsafe fn DrawItemListBgRow(y: u8) {
    FillBgTilemapBufferRect_Palette0(2, 17, 14, y + 2, 15, 1);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DrawPocketIndicatorSquare(x: u8, isCurrentPocket: u8) {
    if isCurrentPocket == 0 {
        FillBgTilemapBufferRect_Palette0(2, 0x1017, x + 5, 3, 1, 1);
    } else {
        FillBgTilemapBufferRect_Palette0(2, 0x102B, x + 5, 3, 1, 1);
    }
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn CanSwapItems() -> u8 {
    if (gBagPosition.location == ITEMMENULOCATION_FIELD
        || gBagPosition.location == ITEMMENULOCATION_BATTLE)
        && gBagPosition.pocket != TMHM_POCKET
        && gBagPosition.pocket != BERRIES_POCKET
    {
        return TRUE;
    }
    FALSE
}
unsafe fn StartItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ListMenuSetTemplateField(*data as u8, LISTFIELD_CURSORKIND, CURSOR_INVISIBLE);
    *data.at(1) = gBagPosition.scrollPosition[gBagPosition.pocket] as i16
        + gBagPosition.cursorPosition[gBagPosition.pocket] as i16;
    (*gBagMenu).toSwapPos = *data.at(1) as u8;
    CopyItemName(
        BagGetItemIdByPocketPosition(gBagPosition.pocket + 1, *data.at(1) as u16),
        gStringVar1.as_mut_ptr(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_MoveVar1Where).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
    BagMenu_Print(
        WIN_DESCRIPTION,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        3,
        1,
        0,
        0,
        0,
        COLORID_NORMAL,
    );
    UpdateItemMenuSwapLinePos(*data.at(1) as u8);
    DestroyPocketSwitchArrowPair();
    BagMenu_PrintCursor(*data as u8, COLORID_GRAY_CURSOR);
    task_set_func(taskId, Some(Task_HandleSwappingItemsInput));
}
pub(crate) unsafe fn Task_HandleSwappingItemsInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            PlaySE(SE_SELECT);
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut gBagPosition.scrollPosition[gBagPosition.pocket],
                &raw mut gBagPosition.cursorPosition[gBagPosition.pocket],
            );
            DoItemSwap(taskId);
        } else {
            let input: i32 = ListMenu_ProcessInput(*data as u8);
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut gBagPosition.scrollPosition[gBagPosition.pocket],
                &raw mut gBagPosition.cursorPosition[gBagPosition.pocket],
            );
            SetItemMenuSwapLineInvisibility(FALSE);
            UpdateItemMenuSwapLinePos(gBagPosition.cursorPosition[gBagPosition.pocket] as u8);
            match input {
                LIST_NOTHING_CHOSEN => {}
                LIST_CANCEL => {
                    PlaySE(SE_SELECT);
                    if gMain.newKeys as i32 & A_BUTTON != 0 {
                        DoItemSwap(taskId);
                    } else {
                        CancelItemSwap(taskId);
                    }
                }
                _ => {
                    PlaySE(SE_SELECT);
                    DoItemSwap(taskId);
                }
            }
        }
    }
}
unsafe fn DoItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    let realPos: u16 = *scrollPos + *cursorPos;
    if *data.at(1) as i32 == realPos as i32 || *data.at(1) as i32 == realPos as i32 - 1 {
        CancelItemSwap(taskId);
    } else {
        MoveItemSlotInList(
            gBagPockets[gBagPosition.pocket].itemSlots,
            *data.at(1) as u32,
            realPos as u32,
        );
        (*gBagMenu).toSwapPos = NOT_SWAPPING;
        DestroyListMenuTask(*data as u8, scrollPos, cursorPos);
        if (*data.at(1) as i32) < realPos as i32 {
            gBagPosition.cursorPosition[gBagPosition.pocket] -= 1;
        }
        LoadBagItemListBuffers(gBagPosition.pocket);
        *data = ListMenuInit(&raw mut gMultiuseListMenuTemplate, *scrollPos, *cursorPos) as i16;
        SetItemMenuSwapLineInvisibility(TRUE);
        CreatePocketSwitchArrowPair();
        task_set_func(taskId, Some(Task_BagMenu_HandleInput));
    }
}
pub(crate) unsafe fn CancelItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    (*gBagMenu).toSwapPos = NOT_SWAPPING;
    DestroyListMenuTask(*data as u8, scrollPos, cursorPos);
    if (*data.at(1) as i32) < *scrollPos as i32 + *cursorPos as i32 {
        gBagPosition.cursorPosition[gBagPosition.pocket] -= 1;
    }
    LoadBagItemListBuffers(gBagPosition.pocket);
    *data = ListMenuInit(&raw mut gMultiuseListMenuTemplate, *scrollPos, *cursorPos) as i16;
    SetItemMenuSwapLineInvisibility(TRUE);
    CreatePocketSwitchArrowPair();
    task_set_func(taskId, Some(Task_BagMenu_HandleInput));
}
pub(crate) unsafe fn OpenContextMenu(taskId: u8) {
    match gBagPosition.location {
        ITEMMENULOCATION_BATTLE | ITEMMENULOCATION_WALLY => {
            if GetItemBattleUsage(gSpecialVar_ItemId) != ITEM_B_USE_NONE {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_BattleUse.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 2;
            } else {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Cancel.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 1;
            }
        }
        ITEMMENULOCATION_BERRY_BLENDER_CRUSH => {
            (*gBagMenu).contextMenuItemsPtr =
                sContextMenuItems_BerryBlenderCrush.as_ptr().cast_mut();
            (*gBagMenu).contextMenuNumItems = 4;
        }
        ITEMMENULOCATION_APPRENTICE => {
            if GetItemImportance(gSpecialVar_ItemId) == 0 && gSpecialVar_ItemId != ITEM_ENIGMA_BERRY
            {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Apprentice.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 2;
            } else {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Cancel.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 1;
            }
        }
        ITEMMENULOCATION_FAVOR_LADY => {
            if GetItemImportance(gSpecialVar_ItemId) == 0 && gSpecialVar_ItemId != ITEM_ENIGMA_BERRY
            {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_FavorLady.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 2;
            } else {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Cancel.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 1;
            }
        }
        ITEMMENULOCATION_QUIZ_LADY => {
            if GetItemImportance(gSpecialVar_ItemId) == 0 && gSpecialVar_ItemId != ITEM_ENIGMA_BERRY
            {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_QuizLady.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 2;
            } else {
                (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Cancel.as_ptr().cast_mut();
                (*gBagMenu).contextMenuNumItems = 1;
            }
        }
        _ => {
            if MenuHelpers_IsLinkActive() == TRUE || InUnionRoom() == TRUE as u32 {
                if gBagPosition.pocket == KEYITEMS_POCKET
                    || IsHoldingItemAllowed(gSpecialVar_ItemId) == 0
                {
                    (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Cancel.as_ptr().cast_mut();
                    (*gBagMenu).contextMenuNumItems = 1;
                } else {
                    (*gBagMenu).contextMenuItemsPtr = sContextMenuItems_Give.as_ptr().cast_mut();
                    (*gBagMenu).contextMenuNumItems = 2;
                }
            } else {
                match gBagPosition.pocket {
                    ITEMS_POCKET => {
                        (*gBagMenu).contextMenuItemsPtr =
                            (*gBagMenu).contextMenuItemsBuffer.as_mut_ptr();
                        (*gBagMenu).contextMenuNumItems = 4;
                        memcpy(
                            &raw mut (*gBagMenu).contextMenuItemsBuffer as *mut u8,
                            (&raw const *sContextMenuItems_ItemsPocket).cast_mut() as *mut u8,
                            4,
                        );
                        if ItemIsMail(gSpecialVar_ItemId) == TRUE {
                            (*gBagMenu).contextMenuItemsBuffer[0] = ACTION_CHECK;
                        }
                    }
                    KEYITEMS_POCKET => {
                        (*gBagMenu).contextMenuItemsPtr =
                            (*gBagMenu).contextMenuItemsBuffer.as_mut_ptr();
                        (*gBagMenu).contextMenuNumItems = 4;
                        memcpy(
                            &raw mut (*gBagMenu).contextMenuItemsBuffer as *mut u8,
                            (&raw const *sContextMenuItems_KeyItemsPocket).cast_mut() as *mut u8,
                            4,
                        );
                        if (*gSaveBlock1Ptr).registeredItem == gSpecialVar_ItemId {
                            (*gBagMenu).contextMenuItemsBuffer[1] = ACTION_DESELECT;
                        }
                        if (gSpecialVar_ItemId == ITEM_MACH_BIKE
                            || gSpecialVar_ItemId == ITEM_ACRO_BIKE)
                            && TestPlayerAvatarFlags(6) != 0
                        {
                            (*gBagMenu).contextMenuItemsBuffer[0] = ACTION_WALK;
                        }
                    }
                    BALLS_POCKET => {
                        (*gBagMenu).contextMenuItemsPtr =
                            sContextMenuItems_BallsPocket.as_ptr().cast_mut();
                        (*gBagMenu).contextMenuNumItems = 4;
                    }
                    TMHM_POCKET => {
                        (*gBagMenu).contextMenuItemsPtr =
                            sContextMenuItems_TmHmPocket.as_ptr().cast_mut();
                        (*gBagMenu).contextMenuNumItems = 4;
                    }
                    BERRIES_POCKET => {
                        (*gBagMenu).contextMenuItemsPtr =
                            sContextMenuItems_BerriesPocket.as_ptr().cast_mut();
                        (*gBagMenu).contextMenuNumItems = 6;
                    }
                    _ => {}
                }
            }
        }
    }
    if gBagPosition.pocket == TMHM_POCKET {
        ClearWindowTilemap(WIN_DESCRIPTION);
        PrintTMHMMoveData(gSpecialVar_ItemId);
        PutWindowTilemap(WIN_TMHM_INFO_ICONS);
        PutWindowTilemap(WIN_TMHM_INFO);
        ScheduleBgCopyTilemapToVram(0);
    } else {
        CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Var1IsSelected).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
        BagMenu_Print(
            WIN_DESCRIPTION,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            3,
            1,
            0,
            0,
            0,
            COLORID_NORMAL,
        );
    }
    if (*gBagMenu).contextMenuNumItems == 1 {
        PrintContextMenuItems(BagMenu_AddWindow(ITEMWIN_1x1));
    } else if (*gBagMenu).contextMenuNumItems == 2 {
        PrintContextMenuItems(BagMenu_AddWindow(ITEMWIN_1x2));
    } else if (*gBagMenu).contextMenuNumItems == 4 {
        PrintContextMenuItemGrid(BagMenu_AddWindow(ITEMWIN_2x2), 2, 2);
    } else {
        PrintContextMenuItemGrid(BagMenu_AddWindow(ITEMWIN_2x3), 2, 3);
    }
}
unsafe fn PrintContextMenuItems(windowId: u8) {
    PrintMenuActionTexts(
        windowId,
        FONT_NARROW,
        8,
        1,
        0,
        16,
        (*gBagMenu).contextMenuNumItems,
        sItemMenuActions.as_ptr().cast_mut(),
        (*gBagMenu).contextMenuItemsPtr,
    );
    InitMenuInUpperLeftCornerNormal(windowId, (*gBagMenu).contextMenuNumItems, 0);
}
unsafe fn PrintContextMenuItemGrid(windowId: u8, columns: u8, rows: u8) {
    PrintMenuActionGrid(
        windowId,
        FONT_NARROW,
        8,
        1,
        56,
        columns,
        rows,
        sItemMenuActions.as_ptr().cast_mut(),
        (*gBagMenu).contextMenuItemsPtr,
    );
    InitMenuActionGrid(windowId, 56, columns, rows, 0);
}
pub(crate) unsafe fn Task_ItemContext_Normal(taskId: u8) {
    OpenContextMenu(taskId);
    if (*gBagMenu).contextMenuNumItems <= 2 {
        task_set_func(taskId, Some(Task_ItemContext_SingleRow));
    } else {
        task_set_func(taskId, Some(Task_ItemContext_MultipleRows));
    }
}
pub(crate) unsafe fn Task_ItemContext_SingleRow(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let selection: i8 = Menu_ProcessInputNoWrap();
        match selection {
            MENU_NOTHING_CHOSEN => {}
            MENU_B_PRESSED => {
                PlaySE(SE_SELECT);
                sItemMenuActions[4].func.void_u8.unwrap_unchecked()(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                sItemMenuActions[*(*gBagMenu).contextMenuItemsPtr.at(selection)]
                    .func
                    .void_u8
                    .unwrap_unchecked()(taskId);
            }
        }
    }
}
pub(crate) unsafe fn Task_ItemContext_MultipleRows(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let cursorPos: i8 = Menu_GetCursorPos() as i8;
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            if cursorPos > 0 && IsValidContextMenuPos(cursorPos - 2) != 0 {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_UP);
            }
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            if (cursorPos as i32) < (*gBagMenu).contextMenuNumItems as i32 - 2
                && IsValidContextMenuPos(cursorPos + 2) != 0
            {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_DOWN);
            }
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
            if cursorPos as i32 & 1 != 0 && IsValidContextMenuPos(cursorPos - 1) != 0 {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_LEFT, MENU_CURSOR_DELTA_NONE);
            }
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
            if cursorPos as i32 & 1 == 0 && IsValidContextMenuPos(cursorPos + 1) != 0 {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_RIGHT, MENU_CURSOR_DELTA_NONE);
            }
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            PlaySE(SE_SELECT);
            sItemMenuActions[*(*gBagMenu).contextMenuItemsPtr.at(cursorPos)]
                .func
                .void_u8
                .unwrap_unchecked()(taskId);
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            sItemMenuActions[4].func.void_u8.unwrap_unchecked()(taskId);
        }
    }
}
unsafe fn IsValidContextMenuPos(cursorPos: i8) -> u8 {
    if cursorPos < 0 {
        return FALSE;
    }
    if cursorPos as i32 > (*gBagMenu).contextMenuNumItems as i32 {
        return FALSE;
    }
    if *(*gBagMenu).contextMenuItemsPtr.at(cursorPos) == ACTION_DUMMY {
        return FALSE;
    }
    TRUE
}
unsafe fn RemoveContextWindow() {
    if (*gBagMenu).contextMenuNumItems == 1 {
        BagMenu_RemoveWindow(ITEMWIN_1x1);
    } else if (*gBagMenu).contextMenuNumItems == 2 {
        BagMenu_RemoveWindow(ITEMWIN_1x2);
    } else if (*gBagMenu).contextMenuNumItems == 4 {
        BagMenu_RemoveWindow(ITEMWIN_2x2);
    } else {
        BagMenu_RemoveWindow(ITEMWIN_2x3);
    }
}
pub(crate) unsafe fn ItemMenu_UseOutOfBattle(taskId: u8) {
    if GetItemFieldFunc(gSpecialVar_ItemId).is_some() {
        RemoveContextWindow();
        if CalculatePlayerPartyCount() == 0
            && GetItemType(gSpecialVar_ItemId) == ITEM_USE_PARTY_MENU as u8
        {
            PrintThereIsNoPokemon(taskId);
        } else {
            FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
            ScheduleBgCopyTilemapToVram(0);
            if gBagPosition.pocket != BERRIES_POCKET {
                GetItemFieldFunc(gSpecialVar_ItemId).unwrap_unchecked()(taskId);
            } else {
                ItemUseOutOfBattle_Berry(taskId);
            }
        }
    }
}
pub(crate) unsafe fn ItemMenu_Toss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RemoveContextWindow();
    *data.at(8) = 1;
    if *data.at(2) == 1 {
        AskTossItems(taskId);
    } else {
        CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_TossHowManyVar1s).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
        BagMenu_Print(
            WIN_DESCRIPTION,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            3,
            1,
            0,
            0,
            0,
            COLORID_NORMAL,
        );
        AddItemQuantityWindow(ITEMWIN_QUANTITY);
        task_set_func(taskId, Some(Task_ChooseHowManyToToss));
    }
}
unsafe fn AskTossItems(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        *data.at(8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        MAX_ITEM_DIGITS,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ConfirmTossItems).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
    BagMenu_Print(
        WIN_DESCRIPTION,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        3,
        1,
        0,
        0,
        0,
        COLORID_NORMAL,
    );
    BagMenu_YesNo(
        taskId,
        ITEMWIN_YESNO_LOW,
        (&raw const *sYesNoTossFunctions).cast_mut(),
    );
}
pub(crate) unsafe fn CancelToss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    PrintItemDescription(*data.at(1) as i32);
    BagMenu_PrintCursor(*data as u8, COLORID_NORMAL);
    ReturnToItemList(taskId);
}
pub(crate) unsafe fn Task_ChooseHowManyToToss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if AdjustQuantityAccordingToDPadInput(data.at(8), *data.at(2) as u16) == TRUE {
        PrintItemQuantity((*gBagMenu).windowIds[7], *data.at(8));
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BagMenu_RemoveWindow(ITEMWIN_QUANTITY);
        AskTossItems(taskId);
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BagMenu_RemoveWindow(ITEMWIN_QUANTITY);
        CancelToss(taskId);
    }
}
pub(crate) unsafe fn ConfirmToss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        *data.at(8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        MAX_ITEM_DIGITS,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ThrewAwayVar2Var1s).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
    BagMenu_Print(
        WIN_DESCRIPTION,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        3,
        1,
        0,
        0,
        0,
        COLORID_NORMAL,
    );
    task_set_func(taskId, Some(Task_RemoveItemFromBag));
}
pub(crate) unsafe fn Task_RemoveItemFromBag(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    if gMain.newKeys as i32 & 3 != 0 {
        PlaySE(SE_SELECT);
        RemoveBagItem(gSpecialVar_ItemId, *data.at(8) as u16);
        DestroyListMenuTask(*data as u8, scrollPos, cursorPos);
        UpdatePocketItemList(gBagPosition.pocket);
        UpdatePocketListPosition(gBagPosition.pocket);
        LoadBagItemListBuffers(gBagPosition.pocket);
        *data = ListMenuInit(&raw mut gMultiuseListMenuTemplate, *scrollPos, *cursorPos) as i16;
        ScheduleBgCopyTilemapToVram(0);
        ReturnToItemList(taskId);
    }
}
pub(crate) unsafe fn ItemMenu_Register(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    if (*gSaveBlock1Ptr).registeredItem == gSpecialVar_ItemId {
        (*gSaveBlock1Ptr).registeredItem = ITEM_NONE;
    } else {
        (*gSaveBlock1Ptr).registeredItem = gSpecialVar_ItemId;
    }
    DestroyListMenuTask(*data as u8, scrollPos, cursorPos);
    LoadBagItemListBuffers(gBagPosition.pocket);
    *data = ListMenuInit(&raw mut gMultiuseListMenuTemplate, *scrollPos, *cursorPos) as i16;
    ScheduleBgCopyTilemapToVram(0);
    ItemMenu_Cancel(taskId);
}
pub(crate) unsafe fn ItemMenu_Give(taskId: u8) {
    RemoveContextWindow();
    if IsWritingMailAllowed(gSpecialVar_ItemId) == 0 {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_CantWriteMail).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(HandleErrorMessage),
        );
    } else if GetItemImportance(gSpecialVar_ItemId) == 0 {
        if CalculatePlayerPartyCount() == 0 {
            PrintThereIsNoPokemon(taskId);
        } else {
            (*gBagMenu).newScreenCallback = Some(CB2_ChooseMonToGiveItem);
            Task_FadeAndCloseBagMenu(taskId);
        }
    } else {
        PrintItemCantBeHeld(taskId);
    }
}
unsafe fn PrintThereIsNoPokemon(taskId: u8) {
    DisplayItemMessage(
        taskId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_NoPokemon).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(HandleErrorMessage),
    );
}
unsafe fn PrintItemCantBeHeld(taskId: u8) {
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Var1CantBeHeld).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayItemMessage(
        taskId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        Some(HandleErrorMessage),
    );
}
pub(crate) unsafe fn HandleErrorMessage(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        CloseItemMessage(taskId);
    }
}
pub(crate) unsafe fn ItemMenu_CheckTag(taskId: u8) {
    (*gBagMenu).newScreenCallback = Some(DoBerryTagScreen);
    Task_FadeAndCloseBagMenu(taskId);
}
pub(crate) unsafe fn ItemMenu_Cancel(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RemoveContextWindow();
    PrintItemDescription(*data.at(1) as i32);
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    BagMenu_PrintCursor(*data as u8, COLORID_NORMAL);
    ReturnToItemList(taskId);
}
pub(crate) unsafe fn ItemMenu_UseInBattle(taskId: u8) {
    if GetItemBattleFunc(gSpecialVar_ItemId).is_some() {
        RemoveContextWindow();
        GetItemBattleFunc(gSpecialVar_ItemId).unwrap_unchecked()(taskId);
    }
}
pub unsafe fn CB2_ReturnToBagMenuPocket() {
    GoToBagMenu(ITEMMENULOCATION_LAST, POCKETS_COUNT, None);
}
pub(crate) unsafe fn Task_ItemContext_GiveToParty(taskId: u8) {
    if IsWritingMailAllowed(gSpecialVar_ItemId) == 0 {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_CantWriteMail).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(HandleErrorMessage),
        );
    } else if IsHoldingItemAllowed(gSpecialVar_ItemId) == 0 {
        CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Var1CantBeHeldHere).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            Some(HandleErrorMessage),
        );
    } else if gBagPosition.pocket != KEYITEMS_POCKET && GetItemImportance(gSpecialVar_ItemId) == 0 {
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        PrintItemCantBeHeld(taskId);
    }
}
pub(crate) unsafe fn Task_ItemContext_GiveToPC(taskId: u8) {
    if ItemIsMail(gSpecialVar_ItemId) == TRUE {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_CantWriteMail).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(HandleErrorMessage),
        );
    } else if gBagPosition.pocket != KEYITEMS_POCKET && GetItemImportance(gSpecialVar_ItemId) == 0 {
        task_set_func(taskId, Some(Task_FadeAndCloseBagMenu));
    } else {
        PrintItemCantBeHeld(taskId);
    }
}
pub unsafe fn UseRegisteredKeyItemOnField() -> u8 {
    let mut taskId: u8 = 0;
    if InUnionRoom() == TRUE as u32
        || CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE
        || InBattlePike() != 0
        || InMultiPartnerRoom() == TRUE
    {
        return FALSE;
    }
    HideMapNamePopUpWindow();
    ChangeBgY_ScreenOff(0, 0, BG_COORD_SET);
    if (*gSaveBlock1Ptr).registeredItem != ITEM_NONE {
        if CheckBagHasItem((*gSaveBlock1Ptr).registeredItem, 1) == 1 {
            LockPlayerFieldControls();
            FreezeObjectEvents();
            PlayerFreeze();
            StopPlayerAvatar();
            gSpecialVar_ItemId = (*gSaveBlock1Ptr).registeredItem;
            taskId = CreateTask(GetItemFieldFunc((*gSaveBlock1Ptr).registeredItem), 8);
            task_set(taskId, tUsingRegisteredKeyItem, TRUE as i16);
            return TRUE;
        } else {
            (*gSaveBlock1Ptr).registeredItem = ITEM_NONE;
        }
    }
    ScriptContext_SetupScript(
        (*crate::asmdata::EventScript_SelectWithoutRegisteredItem.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    TRUE
}
pub(crate) unsafe fn Task_ItemContext_Sell(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if GetItemPrice(gSpecialVar_ItemId) == 0 {
        CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_CantBuyKeyItem).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            Some(CloseItemMessage),
        );
    } else {
        *data.at(8) = 1;
        if *data.at(2) == 1 {
            DisplayCurrentMoneyWindow();
            DisplaySellItemPriceAndConfirm(taskId);
        } else {
            CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*(&raw const crate::data::strings::gText_HowManyToSell).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                Some(InitSellHowManyInput),
            );
        }
    }
}
unsafe fn DisplaySellItemPriceAndConfirm(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetItemPrice(gSpecialVar_ItemId) as i32 / 2 * *data.at(8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        6,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ICanPayVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayItemMessage(
        taskId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        Some(AskSellItems),
    );
}
pub(crate) unsafe fn AskSellItems(taskId: u8) {
    BagMenu_YesNo(
        taskId,
        ITEMWIN_YESNO_HIGH,
        (&raw const *sYesNoSellItemFunctions).cast_mut(),
    );
}
pub(crate) unsafe fn CancelSell(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RemoveMoneyWindow();
    RemoveItemMessageWindow(ITEMWIN_MESSAGE);
    BagMenu_PrintCursor(*data as u8, COLORID_NORMAL);
    ReturnToItemList(taskId);
}
pub(crate) unsafe fn InitSellHowManyInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let windowId: u8 = BagMenu_AddWindow(ITEMWIN_QUANTITY_WIDE);
    PrintItemSoldAmount(
        windowId as i32,
        1,
        GetItemPrice(gSpecialVar_ItemId) as i32 / 2 * *data.at(8) as i32,
    );
    DisplayCurrentMoneyWindow();
    task_set_func(taskId, Some(Task_ChooseHowManyToSell));
}
pub(crate) unsafe fn Task_ChooseHowManyToSell(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if AdjustQuantityAccordingToDPadInput(data.at(8), *data.at(2) as u16) == TRUE {
        PrintItemSoldAmount(
            (*gBagMenu).windowIds[8] as i32,
            *data.at(8) as i32,
            GetItemPrice(gSpecialVar_ItemId) as i32 / 2 * *data.at(8) as i32,
        );
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BagMenu_RemoveWindow(ITEMWIN_QUANTITY_WIDE);
        DisplaySellItemPriceAndConfirm(taskId);
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BagMenu_PrintCursor(*data as u8, COLORID_NORMAL);
        RemoveMoneyWindow();
        BagMenu_RemoveWindow(ITEMWIN_QUANTITY_WIDE);
        RemoveItemMessageWindow(ITEMWIN_MESSAGE);
        ReturnToItemList(taskId);
    }
}
pub(crate) unsafe fn ConfirmSell(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetItemPrice(gSpecialVar_ItemId) as i32 / 2 * *data.at(8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        6,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_TurnedOverVar1ForVar2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayItemMessage(
        taskId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        Some(SellItem),
    );
}
pub(crate) unsafe fn SellItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[gBagPosition.pocket];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[gBagPosition.pocket];
    PlaySE(SE_SHOP);
    RemoveBagItem(gSpecialVar_ItemId, *data.at(8) as u16);
    AddMoney(
        &raw mut (*gSaveBlock1Ptr).money,
        (GetItemPrice(gSpecialVar_ItemId) as i32 / 2) as u32 * *data.at(8) as u32,
    );
    DestroyListMenuTask(*data as u8, scrollPos, cursorPos);
    UpdatePocketItemList(gBagPosition.pocket);
    UpdatePocketListPosition(gBagPosition.pocket);
    LoadBagItemListBuffers(gBagPosition.pocket);
    *data = ListMenuInit(&raw mut gMultiuseListMenuTemplate, *scrollPos, *cursorPos) as i16;
    BagMenu_PrintCursor(*data as u8, COLORID_GRAY_CURSOR);
    PrintMoneyAmountInMoneyBox(
        (*gBagMenu).windowIds[9],
        GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32,
        0,
    );
    task_set_func(taskId, Some(WaitAfterItemSell));
}
pub(crate) unsafe fn WaitAfterItemSell(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        PlaySE(SE_SELECT);
        RemoveMoneyWindow();
        CloseItemMessage(taskId);
    }
}
pub(crate) unsafe fn Task_ItemContext_Deposit(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(8) = 1;
    if *data.at(2) == 1 {
        TryDepositItem(taskId);
    } else {
        CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_DepositHowManyVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
        BagMenu_Print(
            WIN_DESCRIPTION,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            3,
            1,
            0,
            0,
            0,
            COLORID_NORMAL,
        );
        AddItemQuantityWindow(ITEMWIN_QUANTITY);
        task_set_func(taskId, Some(Task_ChooseHowManyToDeposit));
    }
}
pub(crate) unsafe fn Task_ChooseHowManyToDeposit(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if AdjustQuantityAccordingToDPadInput(data.at(8), *data.at(2) as u16) == TRUE {
        PrintItemQuantity((*gBagMenu).windowIds[7], *data.at(8));
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BagMenu_RemoveWindow(ITEMWIN_QUANTITY);
        TryDepositItem(taskId);
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        PrintItemDescription(*data.at(1) as i32);
        BagMenu_PrintCursor(*data as u8, COLORID_NORMAL);
        BagMenu_RemoveWindow(ITEMWIN_QUANTITY);
        ReturnToItemList(taskId);
    }
}
unsafe fn TryDepositItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    FillWindowPixelBuffer(WIN_DESCRIPTION, 0);
    if GetItemImportance(gSpecialVar_ItemId) != 0 {
        BagMenu_Print(
            WIN_DESCRIPTION,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_CantStoreImportantItems)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            3,
            1,
            0,
            0,
            0,
            COLORID_NORMAL,
        );
        task_set_func(taskId, Some(WaitDepositErrorMessage));
    } else if AddPCItem(gSpecialVar_ItemId, *data.at(8) as u16) == TRUE {
        CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            *data.at(8) as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            MAX_ITEM_DIGITS,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_DepositedVar2Var1s).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        BagMenu_Print(
            WIN_DESCRIPTION,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            3,
            1,
            0,
            0,
            0,
            COLORID_NORMAL,
        );
        task_set_func(taskId, Some(Task_RemoveItemFromBag));
    } else {
        BagMenu_Print(
            WIN_DESCRIPTION,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_NoRoomForItems).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            3,
            1,
            0,
            0,
            0,
            COLORID_NORMAL,
        );
        task_set_func(taskId, Some(WaitDepositErrorMessage));
    }
}
pub(crate) unsafe fn WaitDepositErrorMessage(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & 3 != 0 {
        PlaySE(SE_SELECT);
        PrintItemDescription(*data.at(1) as i32);
        BagMenu_PrintCursor(*data as u8, COLORID_NORMAL);
        ReturnToItemList(taskId);
    }
}
unsafe fn IsWallysBag() -> u8 {
    if gBagPosition.location == ITEMMENULOCATION_WALLY {
        return TRUE;
    }
    FALSE
}
unsafe fn PrepareBagForWallyTutorial() {
    sTempWallyBag = AllocZeroed(208) as *mut TempWallyBag;
    memcpy(
        (*sTempWallyBag).bagPocket_Items.as_mut_ptr() as *mut u8,
        (*gSaveBlock1Ptr).bagPocket_Items.as_mut_ptr() as *mut u8,
        120,
    );
    memcpy(
        (*sTempWallyBag).bagPocket_PokeBalls.as_mut_ptr() as *mut u8,
        (*gSaveBlock1Ptr).bagPocket_PokeBalls.as_mut_ptr() as *mut u8,
        64,
    );
    (*sTempWallyBag).pocket = gBagPosition.pocket as u16;
    for i in 0..(POCKETS_COUNT as u32) {
        (*sTempWallyBag).cursorPosition[i] = gBagPosition.cursorPosition[i];
        (*sTempWallyBag).scrollPosition[i] = gBagPosition.scrollPosition[i];
    }
    ClearItemSlots(
        (*gSaveBlock1Ptr).bagPocket_Items.as_mut_ptr(),
        BAG_ITEMS_COUNT,
    );
    ClearItemSlots(
        (*gSaveBlock1Ptr).bagPocket_PokeBalls.as_mut_ptr(),
        BAG_POKEBALLS_COUNT,
    );
    ResetBagScrollPositions();
}
unsafe fn RestoreBagAfterWallyTutorial() {
    memcpy(
        (*gSaveBlock1Ptr).bagPocket_Items.as_mut_ptr() as *mut u8,
        (*sTempWallyBag).bagPocket_Items.as_mut_ptr() as *mut u8,
        120,
    );
    memcpy(
        (*gSaveBlock1Ptr).bagPocket_PokeBalls.as_mut_ptr() as *mut u8,
        (*sTempWallyBag).bagPocket_PokeBalls.as_mut_ptr() as *mut u8,
        64,
    );
    gBagPosition.pocket = (*sTempWallyBag).pocket as u8;
    for i in 0..(POCKETS_COUNT as u32) {
        gBagPosition.cursorPosition[i] = (*sTempWallyBag).cursorPosition[i];
        gBagPosition.scrollPosition[i] = (*sTempWallyBag).scrollPosition[i];
    }
    Free(sTempWallyBag as *mut c_void);
}
pub unsafe fn DoWallyTutorialBagMenu() {
    PrepareBagForWallyTutorial();
    AddBagItem(ITEM_POTION as u16, 1);
    AddBagItem(ITEM_POKE_BALL, 1);
    GoToBagMenu(
        ITEMMENULOCATION_WALLY,
        ITEMS_POCKET,
        Some(CB2_SetUpReshowBattleScreenAfterMenu2),
    );
}
pub(crate) unsafe fn Task_WallyTutorialBagMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        match *data.at(8) {
            WALLY_BAG_DELAY => {
                PlaySE(SE_SELECT);
                SwitchBagPocket(taskId, MENU_CURSOR_DELTA_RIGHT as i16, FALSE as u16);
                *data.at(8) += 1;
            }
            204 => {
                PlaySE(SE_SELECT);
                BagMenu_PrintCursor(*data as u8, COLORID_GRAY_CURSOR);
                gSpecialVar_ItemId = ITEM_POKE_BALL;
                OpenContextMenu(taskId);
                *data.at(8) += 1;
            }
            306 => {
                PlaySE(SE_SELECT);
                RemoveContextWindow();
                DestroyListMenuTask(*data as u8, null_mut(), null_mut());
                RestoreBagAfterWallyTutorial();
                Task_FadeAndCloseBagMenu(taskId);
            }
            _ => {
                *data.at(8) += 1;
            }
        }
    }
}
pub(crate) unsafe fn ItemMenu_Show(taskId: u8) {
    gSpecialVar_0x8005 = gSpecialVar_ItemId;
    gSpecialVar_Result = TRUE as u16;
    RemoveContextWindow();
    Task_FadeAndCloseBagMenu(taskId);
}
pub(crate) unsafe fn CB2_ApprenticeExitBagMenu() {
    gFieldCallback = Some(Apprentice_ScriptContext_Enable);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn ItemMenu_GiveFavorLady(taskId: u8) {
    RemoveBagItem(gSpecialVar_ItemId, 1);
    gSpecialVar_Result = TRUE as u16;
    RemoveContextWindow();
    Task_FadeAndCloseBagMenu(taskId);
}
pub(crate) unsafe fn CB2_FavorLadyExitBagMenu() {
    gFieldCallback = Some(FieldCallback_FavorLadyEnableScriptContexts);
    SetMainCallback2(Some(CB2_ReturnToField));
}
pub(crate) unsafe fn ItemMenu_ConfirmQuizLady(taskId: u8) {
    gSpecialVar_Result = TRUE as u16;
    RemoveContextWindow();
    Task_FadeAndCloseBagMenu(taskId);
}
pub(crate) unsafe fn CB2_QuizLadyExitBagMenu() {
    gFieldCallback = Some(FieldCallback_QuizLadyEnableScriptContexts);
    SetMainCallback2(Some(CB2_ReturnToField));
}
unsafe fn PrintPocketNames(pocketName1: *mut u8, pocketName2: *mut u8) {
    let mut window: WindowTemplate = zeroed();
    window.bg = 0;
    window.width = 16;
    window.height = 2;
    let windowId: u16 = AddWindow(&raw mut window);
    FillWindowPixelBuffer(windowId as u8, 0);
    let mut offset: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, pocketName1, 0x40);
    BagMenu_Print(
        windowId as u8,
        FONT_NORMAL,
        pocketName1,
        offset as u8,
        1,
        0,
        0,
        TEXT_SKIP_DRAW,
        COLORID_POCKET_NAME,
    );
    if !pocketName2.is_null() {
        offset = GetStringCenterAlignXOffset(FONT_NORMAL as i32, pocketName2, 0x40);
        BagMenu_Print(
            windowId as u8,
            FONT_NORMAL,
            pocketName2,
            offset as u8 + 0x40,
            1,
            0,
            0,
            TEXT_SKIP_DRAW,
            COLORID_POCKET_NAME,
        );
    }
    CpuSet(
        GetWindowAttribute(windowId as u8, WINDOW_TILE_DATA) as usize as *mut u8 as *mut c_void,
        (*gBagMenu).pocketNameBuffer.as_mut_ptr() as *mut c_void,
        0x4000100,
    );
    RemoveWindow(windowId as u8);
}
unsafe fn CopyPocketNameToWindow(mut a: u32) {
    if a > 8 {
        a = 8;
    }
    let tileDataBuffer: *mut CArray<CArray<u8, 32>, 32> = &raw mut (*gBagMenu).pocketNameBuffer;
    let windowTileData: *mut u8 = GetWindowAttribute(2, WINDOW_TILE_DATA) as usize as *mut u8;
    CpuSet(
        &raw mut (*tileDataBuffer)[a] as *mut c_void,
        windowTileData as *mut c_void,
        0x4000040,
    );
    let b: i32 = a as i32 + 16;
    CpuSet(
        &raw mut (*tileDataBuffer)[b] as *mut c_void,
        windowTileData.at(256) as *mut c_void,
        0x4000040,
    );
    CopyWindowToVram(WIN_POCKET_NAME, COPYWIN_GFX);
}
unsafe fn LoadBagMenuTextWindows() {
    InitWindows(sDefaultBagWindows.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 1, 224);
    LoadMessageBoxGfx(0, 10, 208);
    ListMenuLoadStdPalAt(192, 1);
    LoadPalette(
        (&raw const (*(&raw const crate::data::menu::gStandardMenuPalette)
            .cast::<CArray<u16, 0>>()))
            .cast_mut() as *mut c_void,
        240,
        32,
    );
    let mut i: u8 = 0;
    while i <= WIN_POCKET_NAME {
        FillWindowPixelBuffer(i, 0);
        PutWindowTilemap(i);
        i += 1;
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn BagMenu_Print(
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
    AddTextPrinterParameterized4(
        windowId,
        fontId,
        left,
        top,
        letterSpacing,
        lineSpacing,
        sFontColorTable[colorIndex].as_ptr().cast_mut(),
        speed as i8,
        str,
    );
}
unsafe fn BagMenu_GetWindowId(windowType: u8) -> u8 {
    (*gBagMenu).windowIds[windowType]
}
unsafe fn BagMenu_AddWindow(windowType: u8) -> u8 {
    let windowId: *mut u8 = &raw mut (*gBagMenu).windowIds[windowType];
    if *windowId == WINDOW_NONE {
        *windowId =
            AddWindow((&raw const sContextMenuWindowTemplates[windowType]).cast_mut()) as u8;
        DrawStdFrameWithCustomTileAndPalette(*windowId, FALSE, 1, 14);
        ScheduleBgCopyTilemapToVram(1);
    }
    *windowId
}
unsafe fn BagMenu_RemoveWindow(windowType: u8) {
    let windowId: *mut u8 = &raw mut (*gBagMenu).windowIds[windowType];
    if *windowId != WINDOW_NONE {
        ClearStdWindowAndFrameToTransparent(*windowId, FALSE);
        ClearWindowTilemap(*windowId);
        RemoveWindow(*windowId);
        ScheduleBgCopyTilemapToVram(1);
        *windowId = WINDOW_NONE;
    }
}
unsafe fn AddItemMessageWindow(windowType: u8) -> u8 {
    let windowId: *mut u8 = &raw mut (*gBagMenu).windowIds[windowType];
    if *windowId == WINDOW_NONE {
        *windowId =
            AddWindow((&raw const sContextMenuWindowTemplates[windowType]).cast_mut()) as u8;
    }
    *windowId
}
unsafe fn RemoveItemMessageWindow(windowType: u8) {
    let windowId: *mut u8 = &raw mut (*gBagMenu).windowIds[windowType];
    if *windowId != WINDOW_NONE {
        ClearDialogWindowAndFrameToTransparent(*windowId, FALSE);
        ClearWindowTilemap(*windowId);
        RemoveWindow(*windowId);
        ScheduleBgCopyTilemapToVram(1);
        *windowId = WINDOW_NONE;
    }
}
pub unsafe fn BagMenu_YesNo(taskId: u8, windowType: u8, funcTable: *mut YesNoFuncTable) {
    CreateYesNoMenuWithCallbacks(
        taskId,
        (&raw const sContextMenuWindowTemplates[windowType]).cast_mut(),
        1,
        0,
        2,
        1,
        14,
        funcTable,
    );
}
unsafe fn DisplayCurrentMoneyWindow() {
    let windowId: u8 = BagMenu_AddWindow(ITEMWIN_MONEY);
    PrintMoneyAmountInMoneyBoxWithBorder(
        windowId,
        1,
        14,
        GetMoney(&raw mut (*gSaveBlock1Ptr).money) as i32,
    );
    AddMoneyLabelObject(19, 11);
}
unsafe fn RemoveMoneyWindow() {
    BagMenu_RemoveWindow(ITEMWIN_MONEY);
    RemoveMoneyLabelObject();
}
unsafe fn PrepareTMHMMoveWindow() {
    FillWindowPixelBuffer(WIN_TMHM_INFO_ICONS, 0);
    BlitMenuInfoIcon(WIN_TMHM_INFO_ICONS, MENU_INFO_ICON_TYPE, 0, 0);
    BlitMenuInfoIcon(WIN_TMHM_INFO_ICONS, MENU_INFO_ICON_POWER, 0, 12);
    BlitMenuInfoIcon(WIN_TMHM_INFO_ICONS, MENU_INFO_ICON_ACCURACY, 0, 24);
    BlitMenuInfoIcon(WIN_TMHM_INFO_ICONS, MENU_INFO_ICON_PP, 0, 36);
    CopyWindowToVram(WIN_TMHM_INFO_ICONS, COPYWIN_GFX);
}
unsafe fn PrintTMHMMoveData(itemId: u16) {
    let mut r#move: u16 = 0;
    let mut text: *mut u8 = null_mut();
    FillWindowPixelBuffer(WIN_TMHM_INFO, 0);
    if itemId == ITEM_NONE {
        for i in 0..4u8 {
            BagMenu_Print(
                WIN_TMHM_INFO,
                FONT_NORMAL,
                (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                7,
                i * 12,
                0,
                0,
                TEXT_SKIP_DRAW,
                COLORID_TMHM_INFO,
            );
        }
        CopyWindowToVram(WIN_TMHM_INFO, COPYWIN_GFX);
    } else {
        r#move = ItemIdToBattleMoveId(itemId);
        BlitMenuInfoIcon(
            WIN_TMHM_INFO,
            (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .r#type
                + 1,
            0,
            0,
        );
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .power
            <= 1
        {
            text = (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [r#move]
                    .power as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text = gStringVar1.as_mut_ptr();
        }
        BagMenu_Print(
            WIN_TMHM_INFO,
            FONT_NORMAL,
            text,
            7,
            12,
            0,
            0,
            TEXT_SKIP_DRAW,
            COLORID_TMHM_INFO,
        );
        if (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
            [r#move]
            .accuracy
            == 0
        {
            text = (*(&raw const crate::data::strings::gText_ThreeDashes).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        } else {
            ConvertIntToDecimalStringN(
                gStringVar1.as_mut_ptr(),
                (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                    [r#move]
                    .accuracy as i32,
                STR_CONV_MODE_RIGHT_ALIGN,
                3,
            );
            text = gStringVar1.as_mut_ptr();
        }
        BagMenu_Print(
            WIN_TMHM_INFO,
            FONT_NORMAL,
            text,
            7,
            24,
            0,
            0,
            TEXT_SKIP_DRAW,
            COLORID_TMHM_INFO,
        );
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::pokemon::gBattleMoves).cast::<CArray<BattleMove, 0>>())
                [r#move]
                .pp as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            3,
        );
        BagMenu_Print(
            WIN_TMHM_INFO,
            FONT_NORMAL,
            gStringVar1.as_mut_ptr(),
            7,
            36,
            0,
            0,
            TEXT_SKIP_DRAW,
            COLORID_TMHM_INFO,
        );
        CopyWindowToVram(WIN_TMHM_INFO, COPYWIN_GFX);
    }
}
