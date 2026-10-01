//! Translated from `src/battle_pyramid_bag.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_controller_player::CB2_SetUpReshowBattleScreenAfterMenu2;
use crate::bg::{ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ffi::gSpecialVar_Result;
use crate::field_weather::FadeScreen;
use crate::gpu_regs::SetGpuReg;
use crate::item::{
    AddBagItem, CopyItemName, GetItemBattleFunc, GetItemBattleUsage, GetItemFieldFunc,
    GetItemImportance, GetItemPocket, RemovePyramidBagItem,
};
use crate::item_icon::AddItemIconSprite;
use crate::item_menu::gSpecialVar_ItemId;
use crate::list_menu::{
    AddScrollIndicatorArrowPairParameterized, DestroyListMenuTask, ListMenu_ProcessInput,
    ListMenuGetScrollAndRow, ListMenuGetYCoordForPrintingArrowCursor, ListMenuInit,
    ListMenuSetTemplateField, RemoveScrollIndicatorArrowPair, gMultiuseListMenuTemplate,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::mail_data::ItemIsMail;
use crate::menu::{
    AddTextPrinterParameterized4, ChangeMenuGridCursorPosition,
    ClearDialogWindowAndFrameToTransparent, ClearScheduledBgCopiesToVram,
    ClearStdWindowAndFrameToTransparent, DecompressAndCopyTileDataToVram,
    DoScheduledBgTilemapCopiesToVram, DrawStdFrameWithCustomTileAndPalette,
    FreeTempTileDataBuffersIfPossible, GetPlayerTextSpeedDelay, InitMenuActionGrid,
    InitMenuInUpperLeftCornerNormal, Menu_GetCursorPos, Menu_ProcessInputNoWrap,
    PrintMenuActionGrid, PrintMenuActionTexts, ResetTempTileDataBuffers,
    ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{
    AdjustQuantityAccordingToDPadInput, CreateSwapLineSprites, CreateYesNoMenuWithCallbacks,
    DisplayMessageAndContinueTask, GetLRKeysPressed, IsWritingMailAllowed, LoadListMenuSwapLineGfx,
    MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv, ResetAllBgsCoordinates,
    ResetVramOamAndBgCntRegs, SetSwapLineSpritesInvisibility, SetVBlankHBlankCallbacksToNull,
    UpdateSwapLineSpritesPos,
};
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldWithOpenMenu, CleanupOverworldWindowsAndTilemaps,
    gFieldCallback2,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::party_menu::{CB2_ChooseMonToGiveItem, CB2_FadeFromPartyMenu};
use crate::pokemon::{GetMonData2, SetMonData, gPlayerParty};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::script::LockPlayerFieldControls;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeSpritePaletteByTag,
    FreeSpriteTilesByTag, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_set_func};
use crate::text::{DeactivateAllTextPrinters, GetMenuCursorDimensionByFont};
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, FillWindowPixelBuffer, FillWindowPixelRect, FreeAllWindowBuffers,
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
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
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
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
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
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
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
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
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
// Data tables (translate with cdata.py): sBgTemplates sListMenuTemplate sMenuActions sMenuActionIds_Field sMenuActionIds_ChooseToss sMenuActionIds_Battle sMenuActionIds_BattleCannotUse sYesNoTossFuncions sTextColors sWindowTemplates sWindowTemplates_MenuActions sOamData_PyramidBag sAnim_PyramidBag sAnims_PyramidBag sAffineAnim_PyramidBag_Still sAffineAnim_PyramidBag_Shake sAffineAnims_PyramidBag sSpriteSheet_PyramidBag sSpriteTemplate_PyramidBag

const ACTION_CANCEL: i32 = 3;
const ACTION_DUMMY: u8 = 5;
const ANIM_BAG_SHAKE: u8 = 1;
const ANIM_BAG_STILL: u8 = 0;
const COLORID_DARK_GRAY: u8 = 0;
const COLORID_LIGHT_GRAY: u8 = 1;
const COLORID_NONE: u8 = 255;
const MENU_WIN_1x1: u8 = 0;
const MENU_WIN_1x2: u8 = 1;
const MENU_WIN_2x2: u8 = 2;
const MENU_WIN_YESNO: i32 = 4;
const POS_NONE: i32 = -1;
const TAG_ITEM_ICON: u16 = 4133;
const TAG_PYRAMID_BAG: u16 = 4132;
const TAG_SCROLL_ARROW: i32 = 2910;
const WIN_INFO: u8 = 1;
const WIN_LIST: u8 = 0;
const WIN_MSG: u8 = 2;
const WIN_TOSS_NUM: u8 = 3;

static sBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::battle_pyramid_bag::sBgTemplates).cast());
static sListMenuTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::battle_pyramid_bag::sListMenuTemplate).cast());
static sMenuActionIds_Battle: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_pyramid_bag::sMenuActionIds_Battle).cast());
static sMenuActionIds_BattleCannotUse: Table<CArray<u8, 1>> =
    Table((&raw const crate::data::battle_pyramid_bag::sMenuActionIds_BattleCannotUse).cast());
static sMenuActionIds_ChooseToss: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::battle_pyramid_bag::sMenuActionIds_ChooseToss).cast());
static sMenuActionIds_Field: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::battle_pyramid_bag::sMenuActionIds_Field).cast());
static sMenuActions: Table<CArray<MenuAction, 6>> =
    Table((&raw const crate::data::battle_pyramid_bag::sMenuActions).cast());
static sSpriteSheet_PyramidBag: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::battle_pyramid_bag::sSpriteSheet_PyramidBag).cast());
static sSpriteTemplate_PyramidBag: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_pyramid_bag::sSpriteTemplate_PyramidBag).cast());
static sTextColors: Table<CArray<CArray<u8, 3>, 3>> =
    Table((&raw const crate::data::battle_pyramid_bag::sTextColors).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::battle_pyramid_bag::sWindowTemplates).cast());
static sWindowTemplates_MenuActions: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::battle_pyramid_bag::sWindowTemplates_MenuActions).cast());
static sYesNoTossFuncions: Table<YesNoFuncTable> =
    Table((&raw const crate::data::battle_pyramid_bag::sYesNoTossFuncions).cast());

#[unsafe(link_section = "ewram_data")]
pub static mut gPyramidBagMenu: *mut PyramidBagMenu = null_mut();
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPyramidBagMenuState: PyramidBagMenuState = unsafe { zeroed() };

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

pub unsafe fn InitBattlePyramidBagCursorPosition() {
    gPyramidBagMenuState.cursorPosition = 0;
    gPyramidBagMenuState.scrollPosition = 0;
}
pub unsafe fn CB2_PyramidBagMenuFromStartMenu() {
    GoToBattlePyramidBagMenu(PYRAMIDBAG_LOC_FIELD, Some(CB2_ReturnToFieldWithOpenMenu));
}
unsafe fn OpenBattlePyramidBagInBattle() {
    GoToBattlePyramidBagMenu(
        PYRAMIDBAG_LOC_BATTLE,
        Some(CB2_SetUpReshowBattleScreenAfterMenu2),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ChooseItemsToTossFromPyramidBag() {
    LockPlayerFieldControls();
    FadeScreen(FADE_TO_BLACK, 0);
    CreateTask(Some(Task_ChooseItemsToTossFromPyramidBag), 10);
}
pub(crate) unsafe fn Task_ChooseItemsToTossFromPyramidBag(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        gFieldCallback2 = Some(CB2_FadeFromPartyMenu);
        GoToBattlePyramidBagMenu(PYRAMIDBAG_LOC_CHOOSE_TOSS, Some(CB2_ReturnToField));
        DestroyTask(taskId);
    }
}
pub unsafe fn CB2_ReturnToPyramidBagMenu() {
    GoToBattlePyramidBagMenu(PYRAMIDBAG_LOC_PREV, gPyramidBagMenuState.exitCallback);
}
pub unsafe fn GoToBattlePyramidBagMenu(location: u8, exitCallback: Option<unsafe fn()>) {
    gPyramidBagMenu = AllocZeroed(2444) as *mut PyramidBagMenu;
    if location != PYRAMIDBAG_LOC_PREV {
        gPyramidBagMenuState.location = location;
    }
    if exitCallback.is_some() {
        gPyramidBagMenuState.exitCallback = exitCallback;
    }
    (*gPyramidBagMenu).newScreenCallback = None;
    (*gPyramidBagMenu).toSwapPos = POS_NONE as u8;
    (*gPyramidBagMenu).scrollIndicatorsTaskId = TASK_NONE;
    memset(
        (*gPyramidBagMenu).spriteIds.as_mut_ptr(),
        SPRITE_NONE as i32,
        11,
    );
    memset(
        (*gPyramidBagMenu).windowIds.as_mut_ptr(),
        WINDOW_NONE as i32,
        5,
    );
    SetMainCallback2(Some(CB2_LoadPyramidBagMenu));
}
pub(crate) unsafe fn CB2_PyramidBag() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_PyramidBag() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_LoadPyramidBagMenu() {
    while MenuHelpers_ShouldWaitForLinkRecv() != TRUE
        && LoadPyramidBagMenu() != TRUE
        && MenuHelpers_IsLinkActive() != TRUE
    {}
}
unsafe fn LoadPyramidBagMenu() -> u8 {
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
            if MenuHelpers_IsLinkActive() == 0 {
                ResetTasks();
            }
            gMain.state += 1;
        }
        6 => {
            InitPyramidBagBgs();
            (*gPyramidBagMenu).state = 0;
            gMain.state += 1;
        }
        7 => {
            if LoadPyramidBagGfx() != 0 {
                gMain.state += 1;
            }
        }
        8 => {
            InitPyramidBagWindows();
            gMain.state += 1;
        }
        9 => {
            UpdatePyramidBagList();
            UpdatePyramidBagCursorPos();
            InitPyramidBagScroll();
            gMain.state += 1;
        }
        10 => {
            SetBagItemsListTemplate();
            gMain.state += 1;
        }
        11 => {
            CreatePyramidBagInputTask();
            gMain.state += 1;
        }
        12 => {
            CreatePyramidBagSprite();
            gMain.state += 1;
        }
        13 => {
            AddScrollArrows();
            gMain.state += 1;
        }
        14 => {
            CreateSwapLine();
            gMain.state += 1;
        }
        15 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            gMain.state += 1;
        }
        16 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            gPaletteFade.set_bufferTransferDisabled(FALSE as u16);
            gMain.state += 1;
        }
        _ => {
            SetVBlankCallback(Some(VBlankCB_PyramidBag));
            SetMainCallback2(Some(CB2_PyramidBag));
            return TRUE;
        }
    }
    FALSE
}
unsafe fn InitPyramidBagBgs() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        2,
        (*gPyramidBagMenu).tilemapBuffer.as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(0x0, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadPyramidBagGfx() -> u8 {
    match (*gPyramidBagMenu).state {
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
            (*gPyramidBagMenu).state += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gBattlePyramidBagTilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    (*gPyramidBagMenu).tilemapBuffer.as_mut_ptr() as *mut c_void,
                );
                (*gPyramidBagMenu).state += 1;
            }
        }
        2 => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBattlePyramidBagInterface_Pal)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                0,
                32,
            );
            (*gPyramidBagMenu).state += 1;
        }
        3 => {
            LoadCompressedSpriteSheet((&raw const *sSpriteSheet_PyramidBag).cast_mut());
            (*gPyramidBagMenu).state += 1;
        }
        4 => {
            LoadPyramidBagPalette();
            (*gPyramidBagMenu).state += 1;
        }
        _ => {
            LoadListMenuSwapLineGfx();
            (*gPyramidBagMenu).state = 0;
            return TRUE;
        }
    }
    FALSE
}
unsafe fn SetBagItemsListTemplate() {
    let itemIds: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut i: u16 = 0;
    while (i as i32) < (*gPyramidBagMenu).listMenuCount as i32 - 1 {
        CopyBagItemName(
            (*gPyramidBagMenu).itemStrings[i].as_mut_ptr(),
            *itemIds.at(i),
        );
        (*gPyramidBagMenu).bagListItems[i].name = (*gPyramidBagMenu).itemStrings[i].as_mut_ptr();
        (*gPyramidBagMenu).bagListItems[i].id = i as i32;
        i += 1;
    }
    StringCopy(
        (*gPyramidBagMenu).itemStrings[i].as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_CloseBag).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*gPyramidBagMenu).bagListItems[i].name = (*gPyramidBagMenu).itemStrings[i].as_mut_ptr();
    (*gPyramidBagMenu).bagListItems[i].id = LIST_CANCEL;
    gMultiuseListMenuTemplate = *sListMenuTemplate;
    gMultiuseListMenuTemplate.totalItems = (*gPyramidBagMenu).listMenuCount as u16;
    gMultiuseListMenuTemplate.items = (*gPyramidBagMenu).bagListItems.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = (*gPyramidBagMenu).listMenuMaxShown as u16;
}
unsafe fn CopyBagItemName(dst: *mut u8, itemId: u16) {
    if GetItemPocket(itemId) == POCKET_BERRIES {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            itemId as i32 - ITEM_CHERI_BERRY as i32 + 1,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        CopyItemName(itemId, gStringVar2.as_mut_ptr());
        StringExpandPlaceholders(
            dst,
            (*(&raw const crate::data::strings::gText_NumberItem_TMBerry).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        CopyItemName(itemId, dst);
    }
}
pub(crate) unsafe fn BagCursorMoved(itemIndex: i32, onInit: u8, list: *mut ListMenu) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
        ShakePyramidBag();
    }
    if (*gPyramidBagMenu).toSwapPos == POS_NONE as u8 {
        FreeItemIconSpriteByAltId((*gPyramidBagMenu).isAltIcon ^ 1);
        if itemIndex != LIST_CANCEL {
            ShowItemIcon(
                (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[(*gSaveBlock2Ptr).frontier.lvlMode()]
                    [itemIndex],
                (*gPyramidBagMenu).isAltIcon,
            );
        } else {
            ShowItemIcon(ITEM_LIST_END, (*gPyramidBagMenu).isAltIcon);
        }
        (*gPyramidBagMenu).isAltIcon ^= 1;
        PrintItemDescription(itemIndex);
    }
}
pub(crate) unsafe fn PrintItemQuantity(windowId: u8, itemIndex: u32, y: u8) {
    if itemIndex == LIST_CANCEL as u32 {
        return;
    }
    if (*gPyramidBagMenu).toSwapPos != POS_NONE as u8 {
        if (*gPyramidBagMenu).toSwapPos == itemIndex as u8 {
            PrintSelectorArrowAtPos(y, COLORID_LIGHT_GRAY);
        } else {
            PrintSelectorArrowAtPos(y, COLORID_NONE);
        }
    }
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock2Ptr).frontier.pyramidBag.quantity[(*gSaveBlock2Ptr).frontier.lvlMode()]
            [itemIndex] as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let xAlign: i32 = GetStringRightAlignXOffset(FONT_NARROW as i32, gStringVar4.as_mut_ptr(), 119);
    PyramidBagPrint_Quantity(
        windowId,
        gStringVar4.as_mut_ptr(),
        xAlign as u8,
        y,
        0,
        0,
        TEXT_SKIP_DRAW,
        COLORID_DARK_GRAY,
    );
}
pub(crate) unsafe fn PrintItemDescription(listMenuId: i32) {
    let mut desc: *mut u8 = null_mut();
    if listMenuId != LIST_CANCEL {
        desc = GetItemDescription(
            (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[(*gSaveBlock2Ptr).frontier.lvlMode()]
                [listMenuId],
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gPyramidBagMenu_ReturnToStrings)
                .cast::<CArray<*mut u8, 0>>())[gPyramidBagMenuState.location],
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_ReturnToVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        desc = gStringVar4.as_mut_ptr();
    }
    FillWindowPixelBuffer(WIN_INFO, 0);
    PyramidBagPrint(WIN_INFO, desc, 3, 0, 0, 1, 0, COLORID_DARK_GRAY);
}
pub(crate) unsafe fn AddScrollArrows() {
    if (*gPyramidBagMenu).scrollIndicatorsTaskId == TASK_NONE {
        (*gPyramidBagMenu).scrollIndicatorsTaskId = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            172,
            12,
            148,
            (*gPyramidBagMenu).listMenuCount as i32 - (*gPyramidBagMenu).listMenuMaxShown as i32,
            TAG_SCROLL_ARROW,
            TAG_SCROLL_ARROW,
            &raw mut gPyramidBagMenuState.scrollPosition,
        );
    }
}
unsafe fn RemoveScrollArrow() {
    if (*gPyramidBagMenu).scrollIndicatorsTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*gPyramidBagMenu).scrollIndicatorsTaskId);
        (*gPyramidBagMenu).scrollIndicatorsTaskId = TASK_NONE;
    }
}
unsafe fn CreatePyramidBagInputTask() {
    let taskId: u8 = CreateTask(Some(Task_HandlePyramidBagInput), 0);
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        gPyramidBagMenuState.scrollPosition,
        gPyramidBagMenuState.cursorPosition,
    ) as i16;
}
unsafe fn SwapItems(id1: u8, id2: u8) {
    let itemIds: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let mut temp: u16 = *itemIds.at(id1);
    *itemIds.at(id1) = *itemIds.at(id2);
    *itemIds.at(id2) = temp;
    temp = *quantities.at(id1) as u16;
    *quantities.at(id1) = *quantities.at(id2);
    *quantities.at(id2) = temp as u8;
}
unsafe fn MovePyramidBagItemSlotInList(from: u8, mut to: u8) {
    let itemIds: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    if from != to {
        let mut i: i16 = 0;
        let firstSlotItemId: u16 = *itemIds.at(from);
        let firstSlotQuantity: u8 = *quantities.at(from);
        if to > from {
            to -= 1;
            for i in (from as i16)..(to as i16) {
                *itemIds.at(i) = *itemIds.at(i as i32 + 1);
                *quantities.at(i) = *quantities.at(i as i32 + 1);
            }
        } else {
            i = from as i16;
            while i > to as i16 {
                *itemIds.at(i) = *itemIds.at(i as i32 - 1);
                *quantities.at(i) = *quantities.at(i as i32 - 1);
                i -= 1;
            }
        }
        *itemIds.at(to) = firstSlotItemId;
        *quantities.at(to) = firstSlotQuantity;
    }
}
unsafe fn CompactItems() {
    let itemIds: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    let quantities: *mut u8 = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    for i in 0..(PYRAMID_BAG_ITEMS_COUNT as u8) {
        if *itemIds.at(i) == ITEM_NONE || *quantities.at(i) == 0 {
            *itemIds.at(i) = ITEM_NONE;
            *quantities.at(i) = 0;
        }
    }
    for i in 0..9u8 {
        for j in (i + 1)..(PYRAMID_BAG_ITEMS_COUNT as u8) {
            if *itemIds.at(i) == ITEM_NONE || *quantities.at(i) == 0 {
                SwapItems(i, j);
            }
        }
    }
}
pub unsafe fn UpdatePyramidBagList() {
    let itemIds: *mut u16 = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
        [(*gSaveBlock2Ptr).frontier.lvlMode()]
    .as_mut_ptr();
    CompactItems();
    (*gPyramidBagMenu).listMenuCount = 0;
    for i in 0..(PYRAMID_BAG_ITEMS_COUNT as u16) {
        if *itemIds.at(i) != ITEM_NONE {
            (*gPyramidBagMenu).listMenuCount += 1;
        }
    }
    (*gPyramidBagMenu).listMenuCount += 1;
    if (*gPyramidBagMenu).listMenuCount > 8 {
        (*gPyramidBagMenu).listMenuMaxShown = 8;
    } else {
        (*gPyramidBagMenu).listMenuMaxShown = (*gPyramidBagMenu).listMenuCount;
    }
}
pub unsafe fn UpdatePyramidBagCursorPos() {
    if gPyramidBagMenuState.scrollPosition != 0
        && gPyramidBagMenuState.scrollPosition as i32 + (*gPyramidBagMenu).listMenuMaxShown as i32
            > (*gPyramidBagMenu).listMenuCount as i32
    {
        gPyramidBagMenuState.scrollPosition =
            (*gPyramidBagMenu).listMenuCount as u16 - (*gPyramidBagMenu).listMenuMaxShown as u16;
    }
    if gPyramidBagMenuState.scrollPosition as i32 + gPyramidBagMenuState.cursorPosition as i32
        >= (*gPyramidBagMenu).listMenuCount as i32
    {
        if (*gPyramidBagMenu).listMenuCount == 0 {
            gPyramidBagMenuState.cursorPosition = 0;
        } else {
            gPyramidBagMenuState.cursorPosition = (*gPyramidBagMenu).listMenuCount as u16 - 1;
        }
    }
}
unsafe fn InitPyramidBagScroll() {
    let mut i: u8 = 0;
    if gPyramidBagMenuState.cursorPosition > 4 {
        i = 0;
        while i as i32 <= gPyramidBagMenuState.cursorPosition as i32 - 4 {
            if gPyramidBagMenuState.scrollPosition as i32
                + (*gPyramidBagMenu).listMenuMaxShown as i32
                == (*gPyramidBagMenu).listMenuCount as i32
            {
                break;
            }
            gPyramidBagMenuState.cursorPosition -= 1;
            gPyramidBagMenuState.scrollPosition += 1;
            i += 1;
        }
    }
}
pub(crate) unsafe fn PrintSelectorArrow(listMenuTaskId: u8, colorId: u8) {
    let y: u8 = ListMenuGetYCoordForPrintingArrowCursor(listMenuTaskId) as u8;
    PrintSelectorArrowAtPos(y, colorId);
}
unsafe fn PrintSelectorArrowAtPos(y: u8, colorId: u8) {
    if colorId == COLORID_NONE {
        FillWindowPixelRect(
            WIN_LIST,
            0,
            0,
            y as u16,
            GetMenuCursorDimensionByFont(FONT_NORMAL, 0) as u16,
            GetMenuCursorDimensionByFont(FONT_NORMAL, 1) as u16,
        );
    } else {
        PyramidBagPrint(
            WIN_LIST,
            (*(&raw const crate::data::strings::gText_SelectorArrow2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            y,
            0,
            0,
            0,
            colorId,
        );
    }
}
pub unsafe fn CloseBattlePyramidBag(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_ClosePyramidBag));
}
pub(crate) unsafe fn Task_ClosePyramidBag(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        DestroyListMenuTask(
            *data as u8,
            &raw mut gPyramidBagMenuState.scrollPosition,
            &raw mut gPyramidBagMenuState.cursorPosition,
        );
        if (*gPyramidBagMenu).newScreenCallback.is_some() {
            SetMainCallback2((*gPyramidBagMenu).newScreenCallback);
        } else {
            SetMainCallback2(gPyramidBagMenuState.exitCallback);
        }
        RemoveScrollArrow();
        ResetSpriteData();
        FreeAllSpritePalettes();
        FreeAllWindowBuffers();
        Free(gPyramidBagMenu as *mut c_void);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_HandlePyramidBagInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() == TRUE || gPaletteFade.active() != 0 {
        return;
    }
    if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
        if gPyramidBagMenuState.location != PYRAMIDBAG_LOC_PARTY {
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut gPyramidBagMenuState.scrollPosition,
                &raw mut gPyramidBagMenuState.cursorPosition,
            );
            if gPyramidBagMenuState.scrollPosition as i32
                + gPyramidBagMenuState.cursorPosition as i32
                != (*gPyramidBagMenu).listMenuCount as i32 - 1
            {
                PlaySE(SE_SELECT);
                Task_BeginItemSwap(taskId);
            }
        }
    } else {
        let listId: i32 = ListMenu_ProcessInput(*data as u8);
        ListMenuGetScrollAndRow(
            *data as u8,
            &raw mut gPyramidBagMenuState.scrollPosition,
            &raw mut gPyramidBagMenuState.cursorPosition,
        );
        match listId {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                gSpecialVar_ItemId = ITEM_NONE;
                CloseBattlePyramidBag(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                gSpecialVar_ItemId = (*gSaveBlock2Ptr).frontier.pyramidBag.itemId
                    [(*gSaveBlock2Ptr).frontier.lvlMode()][listId];
                *data.at(1) = listId as i16;
                *data.at(2) = (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
                    [(*gSaveBlock2Ptr).frontier.lvlMode()][listId]
                    as i16;
                if gPyramidBagMenuState.location == PYRAMIDBAG_LOC_PARTY {
                    TryCloseBagToGiveItem(taskId);
                } else {
                    OpenContextMenu(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe fn OpenContextMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RemoveScrollArrow();
    PrintSelectorArrow(*data as u8, COLORID_LIGHT_GRAY);
    match gPyramidBagMenuState.location {
        PYRAMIDBAG_LOC_BATTLE => {
            if GetItemBattleUsage(gSpecialVar_ItemId) != ITEM_B_USE_NONE {
                (*gPyramidBagMenu).menuActionIds = sMenuActionIds_Battle.as_ptr().cast_mut();
                (*gPyramidBagMenu).menuActionsCount = 2;
            } else {
                (*gPyramidBagMenu).menuActionIds =
                    sMenuActionIds_BattleCannotUse.as_ptr().cast_mut();
                (*gPyramidBagMenu).menuActionsCount = 1;
            }
        }
        PYRAMIDBAG_LOC_CHOOSE_TOSS => {
            (*gPyramidBagMenu).menuActionIds = sMenuActionIds_ChooseToss.as_ptr().cast_mut();
            (*gPyramidBagMenu).menuActionsCount = 2;
        }
        _ => {
            (*gPyramidBagMenu).menuActionIds = sMenuActionIds_Field.as_ptr().cast_mut();
            (*gPyramidBagMenu).menuActionsCount = 4;
        }
    }
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Var1IsSelected).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_INFO, 0);
    PyramidBagPrint(
        WIN_INFO,
        gStringVar4.as_mut_ptr(),
        3,
        0,
        0,
        1,
        0,
        COLORID_DARK_GRAY,
    );
    if (*gPyramidBagMenu).menuActionsCount == 1 {
        PrintMenuActionText_SingleRow(OpenMenuActionWindowById(MENU_WIN_1x1));
    } else if (*gPyramidBagMenu).menuActionsCount == 2 {
        PrintMenuActionText_SingleRow(OpenMenuActionWindowById(MENU_WIN_1x2));
    } else {
        PrintMenuActionText_MultiRow(OpenMenuActionWindowById(MENU_WIN_2x2), 2, 2);
    }
    if (*gPyramidBagMenu).menuActionsCount == 4 {
        task_set_func(taskId, Some(HandleMenuActionInput_2x2));
    } else {
        task_set_func(taskId, Some(HandleMenuActionInput_SingleRow));
    }
}
unsafe fn PrintMenuActionText_SingleRow(windowId: u8) {
    PrintMenuActionTexts(
        windowId,
        FONT_NARROW,
        8,
        1,
        0,
        0x10,
        (*gPyramidBagMenu).menuActionsCount,
        sMenuActions.as_ptr().cast_mut(),
        (*gPyramidBagMenu).menuActionIds,
    );
    InitMenuInUpperLeftCornerNormal(windowId, (*gPyramidBagMenu).menuActionsCount, 0);
}
unsafe fn PrintMenuActionText_MultiRow(windowId: u8, horizontalCount: u8, verticalCount: u8) {
    PrintMenuActionGrid(
        windowId,
        FONT_NARROW,
        8,
        1,
        56,
        horizontalCount,
        verticalCount,
        sMenuActions.as_ptr().cast_mut(),
        (*gPyramidBagMenu).menuActionIds,
    );
    InitMenuActionGrid(windowId, 56, horizontalCount, verticalCount, 0);
}
pub(crate) unsafe fn HandleMenuActionInput_SingleRow(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let id: i32 = Menu_ProcessInputNoWrap() as i32;
        match id {
            -2 => {}
            -1 => {
                PlaySE(SE_SELECT);
                sMenuActions[3].func.void_u8.unwrap_unchecked()(taskId);
            }
            _ => {
                PlaySE(SE_SELECT);
                if sMenuActions[*(*gPyramidBagMenu).menuActionIds.at(id)]
                    .func
                    .void_u8
                    .is_some()
                {
                    sMenuActions[*(*gPyramidBagMenu).menuActionIds.at(id)]
                        .func
                        .void_u8
                        .unwrap_unchecked()(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe fn HandleMenuActionInput_2x2(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        let id: i8 = Menu_GetCursorPos() as i8;
        if gMain.newKeys as i32 & DPAD_UP != 0 {
            if id > 0 && IsValidMenuAction(id - 2) != 0 {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_UP);
            }
        } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
            if (id as i32) < (*gPyramidBagMenu).menuActionsCount as i32 - 2
                && IsValidMenuAction(id + 2) != 0
            {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_DOWN);
            }
        } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
            if id as i32 & 1 != 0 && IsValidMenuAction(id - 1) != 0 {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_LEFT, MENU_CURSOR_DELTA_NONE);
            }
        } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
            if id as i32 & 1 == 0 && IsValidMenuAction(id + 1) != 0 {
                PlaySE(SE_SELECT);
                ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_RIGHT, MENU_CURSOR_DELTA_NONE);
            }
        } else if gMain.newKeys as i32 & A_BUTTON != 0 {
            PlaySE(SE_SELECT);
            if sMenuActions[*(*gPyramidBagMenu).menuActionIds.at(id)]
                .func
                .void_u8
                .is_some()
            {
                sMenuActions[*(*gPyramidBagMenu).menuActionIds.at(id)]
                    .func
                    .void_u8
                    .unwrap_unchecked()(taskId);
            }
        } else if gMain.newKeys as i32 & B_BUTTON != 0 {
            PlaySE(SE_SELECT);
            sMenuActions[3].func.void_u8.unwrap_unchecked()(taskId);
        }
    }
}
unsafe fn IsValidMenuAction(actionTableId: i8) -> u8 {
    if actionTableId < 0 {
        return FALSE;
    } else if actionTableId as i32 > (*gPyramidBagMenu).menuActionsCount as i32 {
        return FALSE;
    } else if *(*gPyramidBagMenu).menuActionIds.at(actionTableId) == ACTION_DUMMY {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CloseMenuActionWindow() {
    if (*gPyramidBagMenu).menuActionsCount == 1 {
        CloseMenuActionWindowById(MENU_WIN_1x1);
    } else if (*gPyramidBagMenu).menuActionsCount == 2 {
        CloseMenuActionWindowById(MENU_WIN_1x2);
    } else {
        CloseMenuActionWindowById(MENU_WIN_2x2);
    }
}
pub(crate) unsafe fn BagAction_UseOnField(taskId: u8) {
    let pocketId: u8 = GetItemPocket(gSpecialVar_ItemId);
    if pocketId == POCKET_KEY_ITEMS
        || pocketId == POCKET_POKE_BALLS
        || pocketId == POCKET_TM_HM
        || ItemIsMail(gSpecialVar_ItemId) == TRUE
    {
        CloseMenuActionWindow();
        DisplayItemMessageInBattlePyramid(
            taskId,
            (*(&raw const crate::data::strings::gText_DadsAdvice).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Task_CloseBattlePyramidBagMessage),
        );
    } else if GetItemFieldFunc(gSpecialVar_ItemId).is_some() {
        CloseMenuActionWindow();
        FillWindowPixelBuffer(WIN_INFO, 0);
        ScheduleBgCopyTilemapToVram(0);
        GetItemFieldFunc(gSpecialVar_ItemId).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe fn BagAction_Cancel(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CloseMenuActionWindow();
    PrintItemDescription(*data.at(1) as i32);
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
    PrintSelectorArrow(*data as u8, COLORID_DARK_GRAY);
    SetTaskToMainPyramidBagInputHandler(taskId);
}
unsafe fn SetTaskToMainPyramidBagInputHandler(taskId: u8) {
    AddScrollArrows();
    task_set_func(taskId, Some(Task_HandlePyramidBagInput));
}
pub(crate) unsafe fn BagAction_Toss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CloseMenuActionWindow();
    *data.at(8) = 1;
    if *data.at(2) == 1 {
        AskConfirmToss(taskId);
    } else {
        CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_TossHowManyVar1s).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        FillWindowPixelBuffer(WIN_INFO, 0);
        PyramidBagPrint(
            WIN_INFO,
            gStringVar4.as_mut_ptr(),
            3,
            0,
            0,
            1,
            0,
            COLORID_DARK_GRAY,
        );
        ShowNumToToss();
        task_set_func(taskId, Some(Task_ChooseHowManyToToss));
    }
}
unsafe fn AskConfirmToss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        *data.at(8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ConfirmTossItems).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_INFO, 0);
    PyramidBagPrint(
        WIN_INFO,
        gStringVar4.as_mut_ptr(),
        3,
        0,
        0,
        1,
        0,
        COLORID_DARK_GRAY,
    );
    CreatePyramidBagYesNo(taskId, (&raw const *sYesNoTossFuncions).cast_mut());
}
pub(crate) unsafe fn DontTossItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    PrintItemDescription(*data.at(1) as i32);
    PrintSelectorArrow(*data as u8, COLORID_DARK_GRAY);
    SetTaskToMainPyramidBagInputHandler(taskId);
}
unsafe fn ShowNumToToss() {
    ConvertIntToDecimalStringN(gStringVar1.as_mut_ptr(), 1, STR_CONV_MODE_LEADING_ZEROS, 2);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DrawTossNumberWindow(WIN_TOSS_NUM);
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x28);
    AddTextPrinterParameterized(
        WIN_TOSS_NUM,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        2,
        0,
        None,
    );
}
unsafe fn UpdateNumToToss(num: i16) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        num as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_xVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let x: i32 = GetStringCenterAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x28);
    AddTextPrinterParameterized(
        WIN_TOSS_NUM,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x as u8,
        2,
        0,
        None,
    );
}
pub(crate) unsafe fn Task_ChooseHowManyToToss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if AdjustQuantityAccordingToDPadInput(data.at(8), *data.at(2) as u16) == TRUE {
        UpdateNumToToss(*data.at(8));
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        ClearStdWindowAndFrameToTransparent(WIN_TOSS_NUM, FALSE);
        ClearWindowTilemap(WIN_TOSS_NUM);
        ScheduleBgCopyTilemapToVram(1);
        AskConfirmToss(taskId);
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        ClearStdWindowAndFrameToTransparent(WIN_TOSS_NUM, FALSE);
        ClearWindowTilemap(WIN_TOSS_NUM);
        ScheduleBgCopyTilemapToVram(1);
        DontTossItem(taskId);
    }
}
pub(crate) unsafe fn TossItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    ConvertIntToDecimalStringN(
        gStringVar2.as_mut_ptr(),
        *data.at(8) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        2,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ThrewAwayVar2Var1s).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_INFO, 0);
    PyramidBagPrint(
        WIN_INFO,
        gStringVar4.as_mut_ptr(),
        3,
        0,
        0,
        1,
        0,
        COLORID_DARK_GRAY,
    );
    task_set_func(taskId, Some(Task_TossItem));
}
pub(crate) unsafe fn Task_TossItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollOffset: *mut u16 = &raw mut gPyramidBagMenuState.scrollPosition;
    let selectedRow: *mut u16 = &raw mut gPyramidBagMenuState.cursorPosition;
    if gMain.newKeys as i32 & 3 != 0 {
        PlaySE(SE_SELECT);
        RemovePyramidBagItem(gSpecialVar_ItemId, *data.at(8) as u16);
        DestroyListMenuTask(*data as u8, scrollOffset, selectedRow);
        UpdatePyramidBagList();
        UpdatePyramidBagCursorPos();
        SetBagItemsListTemplate();
        *data = ListMenuInit(
            &raw mut gMultiuseListMenuTemplate,
            *scrollOffset,
            *selectedRow,
        ) as i16;
        ScheduleBgCopyTilemapToVram(0);
        SetTaskToMainPyramidBagInputHandler(taskId);
    }
}
pub(crate) unsafe fn BagAction_Give(taskId: u8) {
    CloseMenuActionWindow();
    if ItemIsMail(gSpecialVar_ItemId) == TRUE {
        DisplayItemMessageInBattlePyramid(
            taskId,
            (*(&raw const crate::data::strings::gText_CantWriteMail).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Task_WaitCloseErrorMessage),
        );
    } else if GetItemImportance(gSpecialVar_ItemId) == 0 {
        (*gPyramidBagMenu).newScreenCallback = Some(CB2_ChooseMonToGiveItem);
        CloseBattlePyramidBag(taskId);
    } else {
        ShowCantHoldMessage(taskId);
    }
}
unsafe fn ShowCantHoldMessage(taskId: u8) {
    CopyItemName(gSpecialVar_ItemId, gStringVar1.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Var1CantBeHeld).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayItemMessageInBattlePyramid(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(Task_WaitCloseErrorMessage),
    );
}
pub(crate) unsafe fn Task_WaitCloseErrorMessage(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        Task_CloseBattlePyramidBagMessage(taskId);
    }
}
pub unsafe fn Task_CloseBattlePyramidBagMessage(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    CloseBattlePyramidBagTextWindow();
    PrintItemDescription(*data.at(1) as i32);
    PrintSelectorArrow(*data as u8, COLORID_DARK_GRAY);
    SetTaskToMainPyramidBagInputHandler(taskId);
}
unsafe fn TryCloseBagToGiveItem(taskId: u8) {
    if IsWritingMailAllowed(gSpecialVar_ItemId) == 0 {
        DisplayItemMessageInBattlePyramid(
            taskId,
            (*(&raw const crate::data::strings::gText_CantWriteMail).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Task_WaitCloseErrorMessage),
        );
    } else if GetItemImportance(gSpecialVar_ItemId) == 0 {
        CloseBattlePyramidBag(taskId);
    } else {
        ShowCantHoldMessage(taskId);
    }
}
pub(crate) unsafe fn BagAction_UseInBattle(taskId: u8) {
    if GetItemBattleFunc(gSpecialVar_ItemId).is_some() {
        CloseMenuActionWindow();
        GetItemBattleFunc(gSpecialVar_ItemId).unwrap_unchecked()(taskId);
    }
}
unsafe fn Task_BeginItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) =
        gPyramidBagMenuState.scrollPosition as i16 + gPyramidBagMenuState.cursorPosition as i16;
    (*gPyramidBagMenu).toSwapPos = *data.at(1) as u8;
    ListMenuSetTemplateField(*data as u8, LISTFIELD_CURSORKIND, CURSOR_INVISIBLE);
    CopyItemName(
        (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[(*gSaveBlock2Ptr).frontier.lvlMode()]
            [*data.at(1)],
        gStringVar1.as_mut_ptr(),
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_MoveVar1Where).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    FillWindowPixelBuffer(WIN_INFO, 0);
    PyramidBagPrint(
        WIN_INFO,
        gStringVar4.as_mut_ptr(),
        3,
        0,
        0,
        1,
        0,
        COLORID_DARK_GRAY,
    );
    PrintSelectorArrow(*data as u8, COLORID_LIGHT_GRAY);
    UpdateSwapLinePos(*data.at(1) as u8);
    task_set_func(taskId, Some(Task_ItemSwapHandleInput));
}
pub(crate) unsafe fn Task_ItemSwapHandleInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if gMain.newKeys as i32 & SELECT_BUTTON != 0 {
            PlaySE(SE_SELECT);
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut gPyramidBagMenuState.scrollPosition,
                &raw mut gPyramidBagMenuState.cursorPosition,
            );
            PerformItemSwap(taskId);
        } else {
            let id: i32 = ListMenu_ProcessInput(*data as u8);
            ListMenuGetScrollAndRow(
                *data as u8,
                &raw mut gPyramidBagMenuState.scrollPosition,
                &raw mut gPyramidBagMenuState.cursorPosition,
            );
            SetSwapLineInvisibility(FALSE);
            UpdateSwapLinePos(gPyramidBagMenuState.cursorPosition as u8);
            match id {
                LIST_NOTHING_CHOSEN => {}
                LIST_CANCEL => {
                    PlaySE(SE_SELECT);
                    if gMain.newKeys as i32 & A_BUTTON != 0 {
                        PerformItemSwap(taskId);
                    } else {
                        CancelItemSwap(taskId);
                    }
                }
                _ => {
                    PlaySE(SE_SELECT);
                    PerformItemSwap(taskId);
                }
            }
        }
    }
}
unsafe fn PerformItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollOffset: *mut u16 = &raw mut gPyramidBagMenuState.scrollPosition;
    let selectedRow: *mut u16 = &raw mut gPyramidBagMenuState.cursorPosition;
    let swapPos: u16 = *scrollOffset + *selectedRow;
    if *data.at(1) as i32 == swapPos as i32 || *data.at(1) as i32 == swapPos as i32 - 1 {
        CancelItemSwap(taskId);
    } else {
        MovePyramidBagItemSlotInList(*data.at(1) as u8, swapPos as u8);
        (*gPyramidBagMenu).toSwapPos = POS_NONE as u8;
        SetSwapLineInvisibility(TRUE);
        DestroyListMenuTask(*data as u8, scrollOffset, selectedRow);
        if (*data.at(1) as i32) < swapPos as i32 {
            gPyramidBagMenuState.cursorPosition -= 1;
        }
        SetBagItemsListTemplate();
        *data = ListMenuInit(
            &raw mut gMultiuseListMenuTemplate,
            *scrollOffset,
            *selectedRow,
        ) as i16;
        SetTaskToMainPyramidBagInputHandler(taskId);
    }
}
pub(crate) unsafe fn CancelItemSwap(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let scrollOffset: *mut u16 = &raw mut gPyramidBagMenuState.scrollPosition;
    let selectedRow: *mut u16 = &raw mut gPyramidBagMenuState.cursorPosition;
    (*gPyramidBagMenu).toSwapPos = POS_NONE as u8;
    SetSwapLineInvisibility(TRUE);
    DestroyListMenuTask(*data as u8, scrollOffset, selectedRow);
    if (*data.at(1) as i32) < *scrollOffset as i32 + *selectedRow as i32 {
        gPyramidBagMenuState.cursorPosition -= 1;
    }
    SetBagItemsListTemplate();
    *data = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        *scrollOffset,
        *selectedRow,
    ) as i16;
    SetTaskToMainPyramidBagInputHandler(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn TryStoreHeldItemsInPyramidBag() {
    let party: *mut Pokemon = gPlayerParty.as_mut_ptr();
    let newItems: *mut u16 = Alloc(20) as *mut u16;
    let newQuantities: *mut u8 = Alloc(PYRAMID_BAG_ITEMS_COUNT) as *mut u8;
    let mut heldItem: u16 = 0;
    memcpy(
        newItems as *mut u8,
        (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[(*gSaveBlock2Ptr).frontier.lvlMode()]
            .as_mut_ptr() as *mut u8,
        20,
    );
    memcpy(
        newQuantities,
        (*gSaveBlock2Ptr).frontier.pyramidBag.quantity[(*gSaveBlock2Ptr).frontier.lvlMode()]
            .as_mut_ptr(),
        PYRAMID_BAG_ITEMS_COUNT,
    );
    let mut i: u8 = 0;
    while i < FRONTIER_PARTY_SIZE as u8 {
        heldItem = GetMonData2(party.at(i), MON_DATA_HELD_ITEM) as u16;
        if heldItem != ITEM_NONE && AddBagItem(heldItem, 1) == 0 {
            memcpy(
                (*gSaveBlock2Ptr).frontier.pyramidBag.itemId[(*gSaveBlock2Ptr).frontier.lvlMode()]
                    .as_mut_ptr() as *mut u8,
                newItems as *mut u8,
                20,
            );
            memcpy(
                (*gSaveBlock2Ptr).frontier.pyramidBag.quantity
                    [(*gSaveBlock2Ptr).frontier.lvlMode()]
                .as_mut_ptr(),
                newQuantities,
                PYRAMID_BAG_ITEMS_COUNT,
            );
            Free(newItems as *mut c_void);
            Free(newQuantities as *mut c_void);
            gSpecialVar_Result = 1;
            return;
        }
        i += 1;
    }
    heldItem = ITEM_NONE;
    for i in 0..(FRONTIER_PARTY_SIZE as u8) {
        SetMonData(
            party.at(i),
            MON_DATA_HELD_ITEM,
            &raw mut heldItem as *mut c_void,
        );
    }
    gSpecialVar_Result = 0;
    Free(newItems as *mut c_void);
    Free(newQuantities as *mut c_void);
}
unsafe fn InitPyramidBagWindows() {
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 0x1, 224);
    LoadMessageBoxGfx(0, 0xA, 208);
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        240,
        32,
    );
    for i in 0..5u8 {
        FillWindowPixelBuffer(i, 0);
    }
    PutWindowTilemap(WIN_LIST);
    PutWindowTilemap(WIN_INFO);
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn PyramidBagPrint(
    windowId: u8,
    src: *mut u8,
    x: u8,
    y: u8,
    letterSpacing: u8,
    lineSpacing: u8,
    speed: u8,
    colorTableId: u8,
) {
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x,
        y,
        letterSpacing,
        lineSpacing,
        sTextColors[colorTableId].as_ptr().cast_mut(),
        speed as i8,
        src,
    );
}
unsafe fn PyramidBagPrint_Quantity(
    windowId: u8,
    src: *mut u8,
    x: u8,
    y: u8,
    letterSpacing: u8,
    lineSpacing: u8,
    speed: u8,
    colorTableId: u8,
) {
    AddTextPrinterParameterized4(
        windowId,
        FONT_NARROW,
        x,
        y,
        letterSpacing,
        lineSpacing,
        sTextColors[colorTableId].as_ptr().cast_mut(),
        speed as i8,
        src,
    );
}
unsafe fn DrawTossNumberWindow(windowId: u8) {
    DrawStdFrameWithCustomTileAndPalette(windowId, FALSE, 1, 0xE);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn GetMenuActionWindowId(windowArrayId: u8) -> u8 {
    (*gPyramidBagMenu).windowIds[windowArrayId]
}
unsafe fn OpenMenuActionWindowById(windowArrayId: u8) -> u8 {
    let windowId: *mut u8 = &raw mut (*gPyramidBagMenu).windowIds[windowArrayId];
    if *windowId == WINDOW_NONE {
        *windowId =
            AddWindow((&raw const sWindowTemplates_MenuActions[windowArrayId]).cast_mut()) as u8;
        DrawStdFrameWithCustomTileAndPalette(*windowId, FALSE, 1, 0xE);
        ScheduleBgCopyTilemapToVram(1);
    }
    *windowId
}
unsafe fn CloseMenuActionWindowById(windowArrayId: u8) {
    let windowId: *mut u8 = &raw mut (*gPyramidBagMenu).windowIds[windowArrayId];
    if *windowId != WINDOW_NONE {
        ClearStdWindowAndFrameToTransparent(*windowId, FALSE);
        ClearWindowTilemap(*windowId);
        RemoveWindow(*windowId);
        ScheduleBgCopyTilemapToVram(1);
        *windowId = WINDOW_NONE;
    }
}
unsafe fn CreatePyramidBagYesNo(taskId: u8, yesNoTable: *mut YesNoFuncTable) {
    CreateYesNoMenuWithCallbacks(
        taskId,
        (&raw const sWindowTemplates_MenuActions[4]).cast_mut(),
        1,
        0,
        2,
        1,
        0xE,
        yesNoTable,
    );
}
pub unsafe fn DisplayItemMessageInBattlePyramid(
    taskId: u8,
    str: *mut u8,
    callback: Option<unsafe fn(u8)>,
) {
    FillWindowPixelBuffer(WIN_MSG, 17);
    DisplayMessageAndContinueTask(
        taskId,
        WIN_MSG,
        0xA,
        0xD,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        str,
        core::mem::transmute::<Option<unsafe fn(u8)>, *mut c_void>(callback),
    );
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn CloseBattlePyramidBagTextWindow() {
    ClearDialogWindowAndFrameToTransparent(WIN_MSG, FALSE);
    ClearWindowTilemap(WIN_MSG);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn FreeItemIconSprite(spriteArrId: u8) {
    let spriteId: *mut u8 = &raw mut (*gPyramidBagMenu).spriteIds[spriteArrId];
    if *spriteId != SPRITE_NONE {
        FreeSpriteTilesByTag(4132 + spriteArrId as u16);
        FreeSpritePaletteByTag(4132 + spriteArrId as u16);
        FreeSpriteOamMatrix(&raw mut gSprites[*spriteId]);
        DestroySprite(&raw mut gSprites[*spriteId]);
        *spriteId = SPRITE_NONE;
    }
}
unsafe fn LoadPyramidBagPalette() {
    let mut spritePalette: SpritePalette = zeroed();
    let palPtr: *mut u16 = Alloc(64) as *mut u16;
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gBattlePyramidBag_Pal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        palPtr as *mut c_void,
    );
    spritePalette.data = palPtr.at((*gSaveBlock2Ptr).frontier.lvlMode() as i32 * 16);
    spritePalette.tag = TAG_PYRAMID_BAG;
    LoadSpritePalette(&raw mut spritePalette);
    Free(palPtr as *mut c_void);
}
unsafe fn CreatePyramidBagSprite() {
    let spriteId: *mut u8 = &raw mut (*gPyramidBagMenu).spriteIds[0];
    *spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_PyramidBag).cast_mut(),
        68,
        56,
        0,
    );
}
unsafe fn ShakePyramidBag() {
    let sprite: *mut Sprite = &raw mut gSprites[(*gPyramidBagMenu).spriteIds[0]];
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, ANIM_BAG_SHAKE);
        (*sprite).callback = Some(SpriteCB_BagWaitForShake);
    }
}
pub(crate) unsafe fn SpriteCB_BagWaitForShake(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, ANIM_BAG_STILL);
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
unsafe fn ShowItemIcon(itemId: u16, isAlt: u8) {
    let mut itemSpriteId: u8 = 0;
    let spriteId: *mut u8 =
        &raw mut (*gPyramidBagMenu).spriteIds[isAlt as i32 + PBAG_SPRITE_ITEM_ICON];
    if *spriteId == SPRITE_NONE {
        FreeSpriteTilesByTag(TAG_ITEM_ICON + isAlt as u16);
        FreeSpritePaletteByTag(TAG_ITEM_ICON + isAlt as u16);
        itemSpriteId = AddItemIconSprite(
            TAG_ITEM_ICON + isAlt as u16,
            TAG_ITEM_ICON + isAlt as u16,
            itemId,
        );
        if itemSpriteId != MAX_SPRITES {
            *spriteId = itemSpriteId;
            gSprites[itemSpriteId].x2 = 24;
            gSprites[itemSpriteId].y2 = 88;
        }
    }
}
unsafe fn FreeItemIconSpriteByAltId(isAlt: u8) {
    FreeItemIconSprite(isAlt + PBAG_SPRITE_ITEM_ICON as u8);
}
unsafe fn CreateSwapLine() {
    CreateSwapLineSprites(&raw mut (*gPyramidBagMenu).spriteIds[3], 8);
}
unsafe fn SetSwapLineInvisibility(invisible: u8) {
    SetSwapLineSpritesInvisibility(&raw mut (*gPyramidBagMenu).spriteIds[3], 8, invisible);
}
unsafe fn UpdateSwapLinePos(y: u8) {
    UpdateSwapLineSpritesPos(
        &raw mut (*gPyramidBagMenu).spriteIds[3],
        136,
        120,
        (y as u16 + 1) * 16,
    );
}
