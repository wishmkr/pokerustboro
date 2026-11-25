//! Translated from `src/pokeblock.c` by tools/rustport/c2rs.py.
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
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::battle_controller_player::CB2_SetUpReshowBattleScreenAfterMenu2;
use crate::battle_main::gBattleTextBuff1;
use crate::bg::{FillBgTilemapBufferRect_Palette0, ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_Result};
use crate::field_screen_effect::FieldCB_ContinueScriptHandleMusic;
use crate::gpu_regs::SetGpuReg;
use crate::item_menu::gSpecialVar_ItemId;
use crate::lilycove_lady::GivePokeblockToContestLady;
use crate::list_menu::{
    AddScrollIndicatorArrowPairParameterized, DestroyListMenuTask, ListMenu_ProcessInput,
    ListMenuGetScrollAndRow, ListMenuInit, RemoveScrollIndicatorArrowPair,
    gMultiuseListMenuTemplate,
};
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::{
    AddTextPrinterParameterized4, ClearDialogWindowAndFrameToTransparent,
    ClearScheduledBgCopiesToVram, ClearStdWindowAndFrameToTransparent,
    DecompressAndCopyTileDataToVram, DoScheduledBgTilemapCopiesToVram,
    DrawStdFrameWithCustomTileAndPalette, FreeTempTileDataBuffersIfPossible,
    GetPlayerTextSpeedDelay, InitMenuInUpperLeftCornerNormal, Menu_ProcessInputNoWrap,
    PrintMenuActionTextsInUpperLeftCorner, ResetTempTileDataBuffers, ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{
    CreateSwapLineSprites, CreateYesNoMenuWithCallbacks, DisplayMessageAndContinueTask,
    LoadListMenuSwapLineGfx, MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv,
    ResetAllBgsCoordinates, ResetVramOamAndBgCntRegs, SetSwapLineSpritesInvisibility,
    SetVBlankHBlankCallbacksToNull, UpdateSwapLineSpritesPos,
};
use crate::overworld::{CB2_ReturnToField, gFieldCallback};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::pokemon::{GetNature, gEnemyParty};
use crate::safari_zone::SafariZoneActivatePokeblockFeeder;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, FreeOamMatrix, LoadOam,
    ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
#[allow(unused_imports)]
use crate::types::*;
use crate::use_pokeblock::ChooseMonToGivePokeblock;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CopyToBgTilemapBufferRect` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8) {
    unsafe {
        crate::bg::CopyToBgTilemapBufferRect(a0, a1 as _, a2, a3, a4, a5);
    }
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
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `LoadCompressedSpritePalette` with this module's view of its types.
#[inline]
fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette) {
    unsafe {
        crate::decompress::LoadCompressedSpritePalette(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
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
const sState: usize = 0;
const tListTaskId: usize = 0;
const sTimer: usize = 1;
// Data tables (translate with cdata.py): gPokeblockFlavorCompatibilityTable sBgTemplatesForPokeblockMenu gPokeblockNames sPokeblockMenuActions sActionsOnField sActionsInBattle sActionsOnPokeblockFeeder sActionsWhenGivingToLady sTossYesNoFuncTable sContestStatsMonData sOamData_PokeblockCase sSpriteAnim_PokeblockCase sSpriteAnimTable_PokeblockCase sAffineAnim_PokeblockCaseShake sAffineAnims_PokeblockCaseShake gPokeblockCase_SpriteSheet gPokeblockCase_SpritePal sSpriteTemplate_PokeblockCase sTextColor sFavoritePokeblocksTable sWindowTemplates sTossPkblockWindowTemplate sPokeblockListMenuTemplate

/// `struct PokeblockSavedData`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokeblockSavedData {
    pub callback: Option<unsafe fn()>,
    pub selectedRow: u16,
    pub scrollOffset: u16,
}

unsafe impl Sync for PokeblockSavedData {}

/// `struct PokeblockMenuStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PokeblockMenuStruct {
    pub tilemap: CArray<u8, 2048>,
    pub callbackOnUse: Option<unsafe fn()>,
    pub pokeblockActionIds: *mut u8,
    pub numActions: u8,
    pub caseId: u8,
    pub itemsNo: u8,
    pub maxShowed: u8,
    pub items: CArray<ListMenuItem, 41>,
    pub menuItemsStrings: CArray<CArray<u8, 32>, 41>,
    pub pokeblockCaseSpriteId: u8,
    pub swapLineSpriteIds: CArray<u8, 7>,
    pub arrowTaskId: u8,
    pub isSwapping: u8,
    pub gfxState: i16,
    pub unused: CArray<u8, 8>,
}

unsafe impl Sync for PokeblockMenuStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PokeblockSavedData>() == 8);
    assert!(offset_of!(PokeblockSavedData, callback) == 0);
    assert!(offset_of!(PokeblockSavedData, selectedRow) == 4);
    assert!(offset_of!(PokeblockSavedData, scrollOffset) == 6);
    assert!(size_of::<PokeblockMenuStruct>() == 3720);
    assert!(offset_of!(PokeblockMenuStruct, tilemap) == 0);
    assert!(offset_of!(PokeblockMenuStruct, callbackOnUse) == 2048);
    assert!(offset_of!(PokeblockMenuStruct, pokeblockActionIds) == 2052);
    assert!(offset_of!(PokeblockMenuStruct, numActions) == 2056);
    assert!(offset_of!(PokeblockMenuStruct, caseId) == 2057);
    assert!(offset_of!(PokeblockMenuStruct, itemsNo) == 2058);
    assert!(offset_of!(PokeblockMenuStruct, maxShowed) == 2059);
    assert!(offset_of!(PokeblockMenuStruct, items) == 2060);
    assert!(offset_of!(PokeblockMenuStruct, menuItemsStrings) == 2388);
    assert!(offset_of!(PokeblockMenuStruct, pokeblockCaseSpriteId) == 3700);
    assert!(offset_of!(PokeblockMenuStruct, swapLineSpriteIds) == 3701);
    assert!(offset_of!(PokeblockMenuStruct, arrowTaskId) == 3708);
    assert!(offset_of!(PokeblockMenuStruct, isSwapping) == 3709);
    assert!(offset_of!(PokeblockMenuStruct, gfxState) == 3710);
    assert!(offset_of!(PokeblockMenuStruct, unused) == 3712);
};

const MAX_MENU_ITEMS: u8 = 9;
const MENU_MIDPOINT: u16 = 4;
const POKEBLOCK_MAX_FEEL: u8 = 99;
const TAG_SCROLL_ARROW: i32 = 1110;
const TILE_HIGHLIGHT_BLUE: u16 = 4101;
const TILE_HIGHLIGHT_NONE: u16 = 5;
const TILE_HIGHLIGHT_RED: u16 = 8197;
const WIN_ACTIONS: i16 = 9;
const WIN_ACTIONS_TALL: u8 = 8;
const WIN_BITTER: u8 = 5;
const WIN_DRY: u8 = 3;
const WIN_FEEL: u8 = 7;
const WIN_SOUR: u8 = 6;
const WIN_SPICY: u8 = 2;
const WIN_SWEET: u8 = 4;
const WIN_TITLE: u8 = 0;
const WIN_TOSS_MSG: u8 = 10;

static gPokeblockCase_SpritePal: Table<CompressedSpritePalette> =
    Table((&raw const crate::data::pokeblock::gPokeblockCase_SpritePal).cast());
static gPokeblockCase_SpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokeblock::gPokeblockCase_SpriteSheet).cast());
static gPokeblockFlavorCompatibilityTable: Table<CArray<i8, 125>> =
    Table((&raw const crate::data::pokeblock::gPokeblockFlavorCompatibilityTable).cast());
static gPokeblockNames: Table<CArray<*mut u8, 15>> =
    Table((&raw const crate::data::pokeblock::gPokeblockNames).cast());
static sActionsInBattle: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokeblock::sActionsInBattle).cast());
static sActionsOnField: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokeblock::sActionsOnField).cast());
static sActionsOnPokeblockFeeder: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokeblock::sActionsOnPokeblockFeeder).cast());
static sActionsWhenGivingToLady: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::pokeblock::sActionsWhenGivingToLady).cast());
static sAffineAnims_PokeblockCaseShake: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::pokeblock::sAffineAnims_PokeblockCaseShake).cast());
static sBgTemplatesForPokeblockMenu: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::pokeblock::sBgTemplatesForPokeblockMenu).cast());
static sFavoritePokeblocksTable: Table<CArray<Pokeblock, 5>> =
    Table((&raw const crate::data::pokeblock::sFavoritePokeblocksTable).cast());
static sPokeblockListMenuTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::pokeblock::sPokeblockListMenuTemplate).cast());
static sPokeblockMenuActions: Table<CArray<MenuAction, 6>> =
    Table((&raw const crate::data::pokeblock::sPokeblockMenuActions).cast());
static sSpriteTemplate_PokeblockCase: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokeblock::sSpriteTemplate_PokeblockCase).cast());
static sTextColor: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokeblock::sTextColor).cast());
static sTossPkblockWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokeblock::sTossPkblockWindowTemplate).cast());
static sTossYesNoFuncTable: Table<YesNoFuncTable> =
    Table((&raw const crate::data::pokeblock::sTossYesNoFuncTable).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 12>> =
    Table((&raw const crate::data::pokeblock::sWindowTemplates).cast());

#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sSavedPokeblockData: crate::global::Global<PokeblockSavedData> =
    crate::global::Global::new(unsafe { zeroed() });
#[cfg_attr(target_os = "none", unsafe(link_section = "ewram_data"))]
pub(crate) static sPokeblockMenu: crate::global::Global<*mut PokeblockMenuStruct> =
    crate::global::Global::new(null_mut());

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `GetItemName` with this module's view of its types.
#[inline]
unsafe fn GetItemName(a0: u16) -> *mut u8 {
    crate::item::GetItemName(a0) as *mut u8
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn OpenPokeblockCase(caseId: u8, callback: Option<unsafe fn()>) {
    crate::gl!(sPokeblockMenu = Alloc(3720) as *mut PokeblockMenuStruct);
    (*crate::gl!(sPokeblockMenu)).caseId = caseId;
    (*crate::gl!(sPokeblockMenu)).callbackOnUse = None;
    (*crate::gl!(sPokeblockMenu)).arrowTaskId = TASK_NONE;
    (*crate::gl!(sPokeblockMenu)).isSwapping = FALSE;
    crate::gl!(sSavedPokeblockData.callback = callback);
    match (*crate::gl!(sPokeblockMenu)).caseId {
        PBLOCK_CASE_BATTLE => {
            (*crate::gl!(sPokeblockMenu)).pokeblockActionIds = sActionsInBattle.as_ptr().cast_mut();
            (*crate::gl!(sPokeblockMenu)).numActions = 2;
        }
        PBLOCK_CASE_FEEDER => {
            (*crate::gl!(sPokeblockMenu)).pokeblockActionIds =
                sActionsOnPokeblockFeeder.as_ptr().cast_mut();
            (*crate::gl!(sPokeblockMenu)).numActions = 2;
        }
        PBLOCK_CASE_GIVE => {
            (*crate::gl!(sPokeblockMenu)).pokeblockActionIds =
                sActionsWhenGivingToLady.as_ptr().cast_mut();
            (*crate::gl!(sPokeblockMenu)).numActions = 2;
        }
        _ => {
            (*crate::gl!(sPokeblockMenu)).pokeblockActionIds = sActionsOnField.as_ptr().cast_mut();
            (*crate::gl!(sPokeblockMenu)).numActions = 3;
        }
    }
    SetMainCallback2(Some(CB2_InitPokeblockMenu));
}
pub unsafe fn OpenPokeblockCaseInBattle() {
    OpenPokeblockCase(
        PBLOCK_CASE_BATTLE,
        Some(CB2_SetUpReshowBattleScreenAfterMenu2),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn OpenPokeblockCaseOnFeeder() {
    OpenPokeblockCase(PBLOCK_CASE_FEEDER, Some(CB2_ReturnToField));
}
pub(crate) unsafe fn CB2_PokeblockMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_PokeblockMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_InitPokeblockMenu() {
    loop {
        if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
            break;
        }
        if InitPokeblockMenu() == TRUE {
            break;
        }
        if MenuHelpers_IsLinkActive() == TRUE {
            break;
        }
    }
}
unsafe fn InitPokeblockMenu() -> u8 {
    let mut taskId: u8 = 0;
    match crate::gl!(gMain.state) {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            ClearScheduledBgCopiesToVram();
            crate::gl!(gMain.state += 1);
        }
        1 => {
            ScanlineEffect_Stop();
            crate::gl!(gMain.state += 1);
        }
        2 => {
            FreeAllSpritePalettes();
            crate::gl!(gMain.state += 1);
        }
        3 => {
            ResetPaletteFade();
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(TRUE as u16);
            crate::gl!(gMain.state += 1);
        }
        4 => {
            ResetSpriteData();
            crate::gl!(gMain.state += 1);
        }
        5 => {
            if (*crate::gl!(sPokeblockMenu)).caseId != PBLOCK_CASE_BATTLE {
                ResetTasks();
            }
            crate::gl!(gMain.state += 1);
        }
        6 => {
            HandleInitBackgrounds();
            (*crate::gl!(sPokeblockMenu)).gfxState = 0;
            crate::gl!(gMain.state += 1);
        }
        7 => {
            if LoadPokeblockMenuGfx() == 0 {
                return FALSE;
            }
            crate::gl!(gMain.state += 1);
        }
        8 => {
            SetMenuItemsCountAndMaxShowed();
            LimitMenuScrollAndRow();
            SetInitialScroll();
            crate::gl!(gMain.state += 1);
        }
        9 => {
            (*crate::gl!(sPokeblockMenu)).pokeblockCaseSpriteId =
                CreatePokeblockCaseSprite(56, 64, 0);
            crate::gl!(gMain.state += 1);
        }
        10 => {
            CreateSwapLineSprites(
                (*crate::gl!(sPokeblockMenu)).swapLineSpriteIds.as_mut_ptr(),
                7,
            );
            crate::gl!(gMain.state += 1);
        }
        11 => {
            DrawPokeblockMenuHighlight(
                crate::gl!(sSavedPokeblockData.selectedRow),
                TILE_HIGHLIGHT_BLUE,
            );
            crate::gl!(gMain.state += 1);
        }
        12 => {
            HandleInitWindows();
            crate::gl!(gMain.state += 1);
        }
        13 => {
            UpdatePokeblockList();
            crate::gl!(gMain.state += 1);
        }
        14 => {
            CreateScrollArrows();
            crate::gl!(gMain.state += 1);
        }
        15 => {
            taskId = CreateTask(Some(Task_HandlePokeblockMenuInput), 0);
            task_set(
                taskId,
                tListTaskId,
                ListMenuInit(
                    crate::gl!(&raw gMultiuseListMenuTemplate),
                    crate::gl!(sSavedPokeblockData.scrollOffset),
                    crate::gl!(sSavedPokeblockData.selectedRow),
                ) as i16,
            );
            crate::gl!(gMain.state += 1);
        }
        16 => {
            DrawPokeblockMenuTitleText();
            crate::gl!(gMain.state += 1);
        }
        17 => {
            BlendPalettes(PALETTES_ALL, 16, 0);
            crate::gl!(gMain.state += 1);
        }
        18 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            (*gPaletteFade.as_ptr()).set_bufferTransferDisabled(FALSE as u16);
            crate::gl!(gMain.state += 1);
        }
        _ => {
            SetVBlankCallback(Some(VBlankCB_PokeblockMenu));
            SetMainCallback2(Some(CB2_PokeblockMenu));
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn HandleInitBackgrounds() {
    ResetVramOamAndBgCntRegs();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplatesForPokeblockMenu.as_ptr().cast_mut(), 3);
    SetBgTilemapBuffer(
        2,
        (*crate::gl!(sPokeblockMenu)).tilemap.as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(2);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn LoadPokeblockMenuGfx() -> u8 {
    match (*crate::gl!(sPokeblockMenu)).gfxState {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                2,
                (*(&raw const crate::data::graphics::gMenuPokeblock_Gfx).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*crate::gl!(sPokeblockMenu)).gfxState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gMenuPokeblock_Tilemap)
                        .cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                    (*crate::gl!(sPokeblockMenu)).tilemap.as_mut_ptr() as *mut c_void,
                );
                (*crate::gl!(sPokeblockMenu)).gfxState += 1;
            }
        }
        2 => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gMenuPokeblock_Pal).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                192,
            );
            (*crate::gl!(sPokeblockMenu)).gfxState += 1;
        }
        3 => {
            LoadCompressedSpriteSheet((&raw const *gPokeblockCase_SpriteSheet).cast_mut());
            (*crate::gl!(sPokeblockMenu)).gfxState += 1;
        }
        4 => {
            LoadCompressedSpritePalette((&raw const *gPokeblockCase_SpritePal).cast_mut());
            (*crate::gl!(sPokeblockMenu)).gfxState += 1;
        }
        5 => {
            LoadListMenuSwapLineGfx();
            (*crate::gl!(sPokeblockMenu)).gfxState = 0;
            return TRUE;
        }
        _ => {}
    }
    FALSE
}
pub(crate) unsafe fn HandleInitWindows() {
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 1, 224);
    LoadMessageBoxGfx(0, 0xA, 208);
    LoadPalette(
        (*(&raw const crate::data::menu::gStandardMenuPalette).cast::<CArray<u16, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        240,
        32,
    );
    for i in 0..11u8 {
        FillWindowPixelBuffer(i, 0);
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn PrintOnPokeblockWindow(windowId: u8, string: *mut u8, x: i32) {
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x as u8,
        1,
        0,
        0,
        sTextColor.as_ptr().cast_mut(),
        0,
        string,
    );
}
unsafe fn DrawPokeblockMenuTitleText() {
    let itemName: *mut u8 = GetItemName(ITEM_POKEBLOCK_CASE);
    PrintOnPokeblockWindow(
        WIN_TITLE,
        itemName,
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, itemName, 0x48),
    );
    PrintOnPokeblockWindow(
        WIN_SPICY,
        (*(&raw const crate::data::strings::gText_Spicy).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    PrintOnPokeblockWindow(
        WIN_DRY,
        (*(&raw const crate::data::strings::gText_Dry).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    PrintOnPokeblockWindow(
        WIN_SWEET,
        (*(&raw const crate::data::strings::gText_Sweet).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    PrintOnPokeblockWindow(
        WIN_BITTER,
        (*(&raw const crate::data::strings::gText_Bitter).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    PrintOnPokeblockWindow(
        WIN_SOUR,
        (*(&raw const crate::data::strings::gText_Sour).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    for i in 0..WIN_ACTIONS_TALL {
        PutWindowTilemap(i);
    }
}
unsafe fn UpdatePokeblockList() {
    let mut i: u16 = 0;
    while (i as i32) < (*crate::gl!(sPokeblockMenu)).itemsNo as i32 - 1 {
        PutPokeblockListMenuString(
            (*crate::gl!(sPokeblockMenu)).menuItemsStrings[i].as_mut_ptr(),
            i,
        );
        (*crate::gl!(sPokeblockMenu)).items[i].name =
            (*crate::gl!(sPokeblockMenu)).menuItemsStrings[i].as_mut_ptr();
        (*crate::gl!(sPokeblockMenu)).items[i].id = i as i32;
        i += 1;
    }
    StringCopy(
        (*crate::gl!(sPokeblockMenu)).menuItemsStrings[i].as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_StowCase).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    (*crate::gl!(sPokeblockMenu)).items[i].name =
        (*crate::gl!(sPokeblockMenu)).menuItemsStrings[i].as_mut_ptr();
    (*crate::gl!(sPokeblockMenu)).items[i].id = LIST_CANCEL;
    crate::gl!(gMultiuseListMenuTemplate = *sPokeblockListMenuTemplate);
    (*gMultiuseListMenuTemplate.as_ptr()).set_fontId(FONT_NARROW);
    crate::gl!(gMultiuseListMenuTemplate.totalItems = (*crate::gl!(sPokeblockMenu)).itemsNo as u16);
    crate::gl!(gMultiuseListMenuTemplate.items = (*crate::gl!(sPokeblockMenu)).items.as_mut_ptr());
    crate::gl!(
        gMultiuseListMenuTemplate.maxShowed = (*crate::gl!(sPokeblockMenu)).maxShowed as u16
    );
}
unsafe fn PutPokeblockListMenuString(dst: *mut u8, pkblId: u16) {
    let pkblock: *mut Pokeblock = &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId];
    let mut txtPtr: *mut u8 = StringCopy(dst, gPokeblockNames[(*pkblock).color]);
    *({
        let t1 = txtPtr;
        txtPtr = txtPtr.at(1);
        t1
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t2 = txtPtr;
        txtPtr = txtPtr.at(1);
        t2
    }) = EXT_CTRL_CODE_SKIP_TO;
    *({
        let t3 = txtPtr;
        txtPtr = txtPtr.at(1);
        t3
    }) = CHAR_BLOCK_1;
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetHighestPokeblocksFlavorLevel(pkblock) as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    StringExpandPlaceholders(
        txtPtr,
        (*(&raw const crate::data::strings::gText_LvVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
pub(crate) unsafe fn MovePokeblockMenuCursor(pkblId: i32, onInit: u8, list: *mut ListMenu) {
    if onInit != TRUE {
        PlaySE(SE_SELECT);
        {
            let gl_i0 = (*crate::gl!(sPokeblockMenu)).pokeblockCaseSpriteId;
            crate::gl!(gSprites[gl_i0].callback = Some(SpriteCB_ShakePokeblockCase))
        };
    }
    if (*crate::gl!(sPokeblockMenu)).isSwapping == 0 {
        DrawPokeblockInfo(pkblId);
    }
}
unsafe fn DrawPokeblockInfo(pkblId: i32) {
    let mut pokeblock: *mut Pokeblock = null_mut();
    let mut rectTilemapSrc: CArray<u16, 2> = zeroed();
    FillWindowPixelBuffer(WIN_FEEL, 0);
    if pkblId != LIST_CANCEL {
        pokeblock = &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId];
        rectTilemapSrc[0] = 0x17;
        rectTilemapSrc[1] = 0x18;
        for i in 0..(FLAVOR_COUNT as u8) {
            if GetPokeblockData(pokeblock, PBLOCK_SPICY + i) > 0 {
                rectTilemapSrc[0] = ((i as u16) << 12) + 0x17;
                rectTilemapSrc[1] = ((i as u16) << 12) + 0x18;
            } else {
                rectTilemapSrc[0] = 0xF;
                rectTilemapSrc[1] = 0xF;
            }
            CopyToBgTilemapBufferRect(
                2,
                rectTilemapSrc.as_mut_ptr() as *mut c_void,
                (i as i32 / 3) as u8 * 6 + 1,
                (i as i32 % 3) as u8 * 2 + 13,
                1,
                2,
            );
        }
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            GetPokeblocksFeel(pokeblock) as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        PrintOnPokeblockWindow(WIN_FEEL, gStringVar1.as_mut_ptr(), 4);
    } else {
        rectTilemapSrc[0] = 0xF;
        rectTilemapSrc[1] = 0xF;
        for i in 0..(FLAVOR_COUNT as u8) {
            CopyToBgTilemapBufferRect(
                2,
                rectTilemapSrc.as_mut_ptr() as *mut c_void,
                (i as i32 / 3) as u8 * 6 + 1,
                (i as i32 % 3) as u8 * 2 + 13,
                1,
                2,
            );
        }
        CopyWindowToVram(WIN_FEEL, COPYWIN_GFX);
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn DrawPokeblockMenuHighlight(cursorPos: u16, tileNum: u16) {
    FillBgTilemapBufferRect_Palette0(2, tileNum, 0xF, cursorPos as u8 * 2 + 1, 0xE, 2);
    ScheduleBgCopyTilemapToVram(2);
}
unsafe fn CompactPokeblockSlots() {
    for i in 0..39u16 {
        for j in (i + 1)..POKEBLOCKS_COUNT {
            if (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[i].color == PBLOCK_CLR_NONE {
                let temp: Pokeblock = (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[i];
                (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[i] =
                    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[j];
                (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[j] = temp;
            }
        }
    }
}
unsafe fn SwapPokeblockMenuItems(id1: u32, mut id2: u32) {
    let mut i: i16 = 0;
    let mut count: i16 = 0;
    let pokeblocks: *mut Pokeblock = (*crate::gl!(gSaveBlock1Ptr)).pokeblocks.as_mut_ptr();
    if id1 == id2 {
        return;
    }
    let copyPokeblock1: *mut Pokeblock = Alloc(8) as *mut Pokeblock;
    *copyPokeblock1 = *pokeblocks.at(id1);
    if id2 > id1 {
        id2 -= 1;
        count = id2 as i16;
        for i in (id1 as i16)..count {
            *pokeblocks.at(i) = *pokeblocks.at(i as i32 + 1);
        }
    } else {
        count = id2 as i16;
        i = id1 as i16;
        while i > count {
            *pokeblocks.at(i) = *pokeblocks.at(i as i32 - 1);
            i -= 1;
        }
    }
    *pokeblocks.at(id2) = *copyPokeblock1;
    Free(copyPokeblock1 as *mut c_void);
}
#[unsafe(no_mangle)]
pub fn ResetPokeblockScrollPositions() {
    crate::gl!(sSavedPokeblockData.selectedRow = 0);
    crate::gl!(sSavedPokeblockData.scrollOffset = 0);
}
unsafe fn SetMenuItemsCountAndMaxShowed() {
    CompactPokeblockSlots();
    (*crate::gl!(sPokeblockMenu)).itemsNo = 0;
    for i in 0..POKEBLOCKS_COUNT {
        if (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[i].color != PBLOCK_CLR_NONE {
            (*crate::gl!(sPokeblockMenu)).itemsNo += 1;
        }
    }
    (*crate::gl!(sPokeblockMenu)).itemsNo += 1;
    if (*crate::gl!(sPokeblockMenu)).itemsNo > MAX_MENU_ITEMS {
        (*crate::gl!(sPokeblockMenu)).maxShowed = MAX_MENU_ITEMS;
    } else {
        (*crate::gl!(sPokeblockMenu)).maxShowed = (*crate::gl!(sPokeblockMenu)).itemsNo;
    }
}
unsafe fn LimitMenuScrollAndRow() {
    if crate::gl!(sSavedPokeblockData.scrollOffset) != 0
        && crate::gl!(sSavedPokeblockData.scrollOffset) as i32
            + (*crate::gl!(sPokeblockMenu)).maxShowed as i32
            > (*crate::gl!(sPokeblockMenu)).itemsNo as i32
    {
        crate::gl!(
            sSavedPokeblockData.scrollOffset = (*crate::gl!(sPokeblockMenu)).itemsNo as u16
                - (*crate::gl!(sPokeblockMenu)).maxShowed as u16
        );
    }
    if crate::gl!(sSavedPokeblockData.scrollOffset) as i32
        + crate::gl!(sSavedPokeblockData.selectedRow) as i32
        >= (*crate::gl!(sPokeblockMenu)).itemsNo as i32
    {
        if (*crate::gl!(sPokeblockMenu)).itemsNo == 0 {
            crate::gl!(sSavedPokeblockData.selectedRow = 0);
        } else {
            crate::gl!(
                sSavedPokeblockData.selectedRow = (*crate::gl!(sPokeblockMenu)).itemsNo as u16 - 1
            );
        }
    }
}
unsafe fn SetInitialScroll() {
    if crate::gl!(sSavedPokeblockData.selectedRow) > MENU_MIDPOINT {
        let mut i: u8 = 0;
        while (i as i32) < crate::gl!(sSavedPokeblockData.selectedRow) as i32 - MENU_MIDPOINT as i32
            && crate::gl!(sSavedPokeblockData.scrollOffset) as i32
                + (*crate::gl!(sPokeblockMenu)).maxShowed as i32
                != (*crate::gl!(sPokeblockMenu)).itemsNo as i32
        {
            crate::gl!(sSavedPokeblockData.selectedRow -= 1);
            crate::gl!(sSavedPokeblockData.scrollOffset += 1);
            i += 1;
        }
    }
}
unsafe fn CreateScrollArrows() {
    if (*crate::gl!(sPokeblockMenu)).arrowTaskId == TASK_NONE {
        (*crate::gl!(sPokeblockMenu)).arrowTaskId = AddScrollIndicatorArrowPairParameterized(
            SCROLL_ARROW_UP,
            0xB0,
            8,
            0x98,
            (*crate::gl!(sPokeblockMenu)).itemsNo as i32
                - (*crate::gl!(sPokeblockMenu)).maxShowed as i32,
            TAG_SCROLL_ARROW,
            TAG_SCROLL_ARROW,
            crate::gl!(&raw sSavedPokeblockData.scrollOffset),
        );
    }
}
unsafe fn DestroyScrollArrows() {
    if (*crate::gl!(sPokeblockMenu)).arrowTaskId != TASK_NONE {
        RemoveScrollIndicatorArrowPair((*crate::gl!(sPokeblockMenu)).arrowTaskId);
        (*crate::gl!(sPokeblockMenu)).arrowTaskId = TASK_NONE;
    }
}
pub unsafe fn CreatePokeblockCaseSprite(x: i16, y: i16, subpriority: u8) -> u8 {
    CreateSprite(
        (&raw const *sSpriteTemplate_PokeblockCase).cast_mut(),
        x,
        y,
        subpriority,
    )
}
pub(crate) unsafe fn SpriteCB_ShakePokeblockCase(sprite: *mut Sprite) {
    if (*sprite).data[sState] > 1 {
        (*sprite).data[sState] = 0;
    }
    match (*sprite).data[sState] {
        0 => {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            (*sprite).affineAnims = sAffineAnims_PokeblockCaseShake.as_ptr().cast_mut();
            InitSpriteAffineAnim(sprite);
            (*sprite).data[sState] = 1;
            (*sprite).data[sTimer] = 0;
        }
        1 if ({
            (*sprite).data[sTimer] += 1;
            (*sprite).data[sTimer]
        }) > 11 =>
        {
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
            (*sprite).data[sState] = 0;
            (*sprite).data[sTimer] = 0;
            FreeOamMatrix((*sprite).oam.matrixNum() as u8);
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
        _ => {}
    }
}
unsafe fn FadePaletteAndSetTaskToClosePokeblockCase(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_FreeDataAndExitPokeblockCase));
}
pub(crate) unsafe fn Task_FreeDataAndExitPokeblockCase(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if (*gPaletteFade.as_ptr()).active() == 0 {
        if (*crate::gl!(sPokeblockMenu)).caseId == PBLOCK_CASE_FEEDER
            || (*crate::gl!(sPokeblockMenu)).caseId == PBLOCK_CASE_GIVE
        {
            crate::gl!(gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic));
        }
        DestroyListMenuTask(
            *data as u8,
            crate::gl!(&raw sSavedPokeblockData.scrollOffset),
            crate::gl!(&raw sSavedPokeblockData.selectedRow),
        );
        DestroyScrollArrows();
        ResetSpriteData();
        FreeAllSpritePalettes();
        if (*crate::gl!(sPokeblockMenu)).callbackOnUse.is_some() {
            SetMainCallback2((*crate::gl!(sPokeblockMenu)).callbackOnUse);
        } else {
            SetMainCallback2(crate::gl!(sSavedPokeblockData.callback));
        }
        FreeAllWindowBuffers();
        Free(crate::gl!(sPokeblockMenu) as *mut c_void);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_HandlePokeblockMenuInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if (*gPaletteFade.as_ptr()).active() == 0 && MenuHelpers_ShouldWaitForLinkRecv() != TRUE {
        if crate::gl!(gMain.newKeys) as i32 & SELECT_BUTTON != 0 {
            ListMenuGetScrollAndRow(
                *data as u8,
                crate::gl!(&raw sSavedPokeblockData.scrollOffset),
                crate::gl!(&raw sSavedPokeblockData.selectedRow),
            );
            if crate::gl!(sSavedPokeblockData.scrollOffset) as i32
                + crate::gl!(sSavedPokeblockData.selectedRow) as i32
                != (*crate::gl!(sPokeblockMenu)).itemsNo as i32 - 1
            {
                PlaySE(SE_SELECT);
                DrawPokeblockMenuHighlight(
                    crate::gl!(sSavedPokeblockData.selectedRow),
                    TILE_HIGHLIGHT_RED,
                );
                *data.at(2) = crate::gl!(sSavedPokeblockData.scrollOffset) as i16
                    + crate::gl!(sSavedPokeblockData.selectedRow) as i16;
                (*crate::gl!(sPokeblockMenu)).isSwapping = TRUE;
                task_set_func(taskId, Some(Task_HandlePokeblocksSwapInput));
            }
        } else {
            let oldPosition: u16 = crate::gl!(sSavedPokeblockData.selectedRow);
            let input: i32 = ListMenu_ProcessInput(*data as u8);
            ListMenuGetScrollAndRow(
                *data as u8,
                crate::gl!(&raw sSavedPokeblockData.scrollOffset),
                crate::gl!(&raw sSavedPokeblockData.selectedRow),
            );
            if oldPosition != crate::gl!(sSavedPokeblockData.selectedRow) {
                DrawPokeblockMenuHighlight(oldPosition, TILE_HIGHLIGHT_NONE);
                DrawPokeblockMenuHighlight(
                    crate::gl!(sSavedPokeblockData.selectedRow),
                    TILE_HIGHLIGHT_BLUE,
                );
            }
            match input {
                LIST_NOTHING_CHOSEN => {}
                LIST_CANCEL => {
                    PlaySE(SE_SELECT);
                    gSpecialVar_Result = 0xFFFF;
                    gSpecialVar_ItemId.set(0);
                    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
                }
                _ => {
                    PlaySE(SE_SELECT);
                    gSpecialVar_ItemId.set(input as u16);
                    ShowPokeblockActionsWindow(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe fn Task_HandlePokeblocksSwapInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
        return;
    }
    if crate::gl!(gMain.newKeys) as i32 & SELECT_BUTTON != 0 {
        PlaySE(SE_SELECT);
        ListMenuGetScrollAndRow(
            *data as u8,
            crate::gl!(&raw sSavedPokeblockData.scrollOffset),
            crate::gl!(&raw sSavedPokeblockData.selectedRow),
        );
        UpdatePokeblockSwapMenu(taskId, FALSE);
    } else {
        let i: u16 = crate::gl!(sSavedPokeblockData.scrollOffset);
        let mut row: u16 = crate::gl!(sSavedPokeblockData.selectedRow);
        let input: i32 = ListMenu_ProcessInput(*data as u8);
        ListMenuGetScrollAndRow(
            *data as u8,
            crate::gl!(&raw sSavedPokeblockData.scrollOffset),
            crate::gl!(&raw sSavedPokeblockData.selectedRow),
        );
        if i != crate::gl!(sSavedPokeblockData.scrollOffset)
            || row != crate::gl!(sSavedPokeblockData.selectedRow)
        {
            for i in 0..(MAX_MENU_ITEMS as u16) {
                row = i + crate::gl!(sSavedPokeblockData.scrollOffset);
                if row as i32 == *data.at(2) as i32 {
                    DrawPokeblockMenuHighlight(i, TILE_HIGHLIGHT_RED);
                } else {
                    DrawPokeblockMenuHighlight(i, TILE_HIGHLIGHT_NONE);
                }
            }
        }
        SetSwapLineSpritesInvisibility(
            (*crate::gl!(sPokeblockMenu)).swapLineSpriteIds.as_mut_ptr(),
            7,
            FALSE,
        );
        UpdateSwapLineSpritesPos(
            (*crate::gl!(sPokeblockMenu)).swapLineSpriteIds.as_mut_ptr(),
            7,
            128,
            crate::gl!(sSavedPokeblockData.selectedRow) * 16 + 8,
        );
        match input {
            LIST_NOTHING_CHOSEN => {}
            LIST_CANCEL => {
                PlaySE(SE_SELECT);
                if crate::gl!(gMain.newKeys) as i32 & A_BUTTON != 0 {
                    UpdatePokeblockSwapMenu(taskId, FALSE);
                } else {
                    UpdatePokeblockSwapMenu(taskId, TRUE);
                }
            }
            _ => {
                PlaySE(SE_SELECT);
                UpdatePokeblockSwapMenu(taskId, FALSE);
            }
        }
    }
}
unsafe fn UpdatePokeblockSwapMenu(taskId: u8, noSwap: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let swappedFromId: u16 =
        crate::gl!(sSavedPokeblockData.scrollOffset) + crate::gl!(sSavedPokeblockData.selectedRow);
    (*crate::gl!(sPokeblockMenu)).isSwapping = FALSE;
    DestroyListMenuTask(
        *data as u8,
        crate::gl!(&raw sSavedPokeblockData.scrollOffset),
        crate::gl!(&raw sSavedPokeblockData.selectedRow),
    );
    if noSwap == 0
        && *data.at(2) as i32 != swappedFromId as i32
        && *data.at(2) as i32 != swappedFromId as i32 - 1
    {
        SwapPokeblockMenuItems(*data.at(2) as u32, swappedFromId as u32);
        UpdatePokeblockList();
    }
    if (*data.at(2) as i32) < swappedFromId as i32 {
        crate::gl!(sSavedPokeblockData.selectedRow -= 1);
    }
    *data = ListMenuInit(
        crate::gl!(&raw gMultiuseListMenuTemplate),
        crate::gl!(sSavedPokeblockData.scrollOffset),
        crate::gl!(sSavedPokeblockData.selectedRow),
    ) as i16;
    ScheduleBgCopyTilemapToVram(0);
    SetSwapLineSpritesInvisibility(
        (*crate::gl!(sPokeblockMenu)).swapLineSpriteIds.as_mut_ptr(),
        7,
        TRUE,
    );
    for i in 0..MAX_MENU_ITEMS {
        DrawPokeblockMenuHighlight(i as u16, TILE_HIGHLIGHT_NONE);
    }
    DrawPokeblockMenuHighlight(
        crate::gl!(sSavedPokeblockData.selectedRow),
        TILE_HIGHLIGHT_BLUE,
    );
    task_set_func(taskId, Some(Task_HandlePokeblockMenuInput));
}
unsafe fn ShowPokeblockActionsWindow(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if (*crate::gl!(sPokeblockMenu)).numActions == 3 {
        *data.at(1) = WIN_ACTIONS_TALL as i16;
    } else {
        *data.at(1) = WIN_ACTIONS;
    }
    DestroyScrollArrows();
    DrawStdFrameWithCustomTileAndPalette(*data.at(1) as u8, FALSE, 1, 0xE);
    PrintMenuActionTextsInUpperLeftCorner(
        *data.at(1) as u8,
        (*crate::gl!(sPokeblockMenu)).numActions,
        sPokeblockMenuActions.as_ptr().cast_mut(),
        (*crate::gl!(sPokeblockMenu)).pokeblockActionIds,
    );
    InitMenuInUpperLeftCornerNormal(
        *data.at(1) as u8,
        (*crate::gl!(sPokeblockMenu)).numActions,
        0,
    );
    PutWindowTilemap(*data.at(1) as u8);
    ScheduleBgCopyTilemapToVram(1);
    task_set_func(taskId, Some(Task_HandlePokeblockActionsInput));
}
pub(crate) unsafe fn Task_HandlePokeblockActionsInput(taskId: u8) {
    if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
        return;
    }
    let itemId: i8 = Menu_ProcessInputNoWrap();
    if itemId == MENU_NOTHING_CHOSEN {
    } else if itemId == MENU_B_PRESSED {
        PlaySE(SE_SELECT);
        PokeblockAction_Cancel(taskId);
    } else {
        PlaySE(SE_SELECT);
        sPokeblockMenuActions[*(*crate::gl!(sPokeblockMenu)).pokeblockActionIds.at(itemId)]
            .func
            .void_u8
            .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe fn PokeblockAction_UseOnField(taskId: u8) {
    (*crate::gl!(sPokeblockMenu)).callbackOnUse = Some(UsePokeblockOnField);
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe fn UsePokeblockOnField() {
    ChooseMonToGivePokeblock(
        &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()],
        Some(ReturnToPokeblockCaseOnField),
    );
}
pub(crate) unsafe fn ReturnToPokeblockCaseOnField() {
    OpenPokeblockCase(PBLOCK_CASE_FIELD, crate::gl!(sSavedPokeblockData.callback));
}
pub(crate) unsafe fn PokeblockAction_Toss(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ClearStdWindowAndFrameToTransparent(*data.at(1) as u8, FALSE);
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gPokeblockNames[(*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()].color],
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ThrowAwayVar1).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayMessageAndContinueTask(
        taskId,
        WIN_TOSS_MSG,
        10,
        13,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        gStringVar4.as_mut_ptr(),
        core::mem::transmute::<Option<unsafe fn(u8)>, *mut c_void>(Some(
            CreateTossPokeblockYesNoMenu,
        )),
    );
}
pub(crate) unsafe fn CreateTossPokeblockYesNoMenu(taskId: u8) {
    CreateYesNoMenuWithCallbacks(
        taskId,
        (&raw const *sTossPkblockWindowTemplate).cast_mut(),
        1,
        0,
        2,
        1,
        0xE,
        (&raw const *sTossYesNoFuncTable).cast_mut(),
    );
}
pub(crate) unsafe fn TossedPokeblockMessage(taskId: u8) {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_Var1ThrownAway).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DisplayMessageAndContinueTask(
        taskId,
        WIN_TOSS_MSG,
        10,
        13,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        gStringVar4.as_mut_ptr(),
        core::mem::transmute::<Option<unsafe fn(u8)>, *mut c_void>(Some(TossPokeblock)),
    );
}
pub(crate) unsafe fn TossPokeblock(taskId: u8) {
    if crate::gl!(gMain.newKeys) as i32 & 3 != 0 {
        let mut data: *mut i16 = null_mut();
        let mut scrollOffset: *mut u16 = null_mut();
        let mut selectedRow: *mut u16 = null_mut();
        TryClearPokeblock(gSpecialVar_ItemId.get() as u8);
        PlaySE(SE_SELECT);
        scrollOffset = crate::gl!(&raw sSavedPokeblockData.scrollOffset);
        selectedRow = crate::gl!(&raw sSavedPokeblockData.selectedRow);
        data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
        DestroyListMenuTask(*data as u8, scrollOffset, selectedRow);
        DrawPokeblockMenuHighlight(*selectedRow, TILE_HIGHLIGHT_NONE);
        SetMenuItemsCountAndMaxShowed();
        LimitMenuScrollAndRow();
        UpdatePokeblockList();
        *data = ListMenuInit(
            crate::gl!(&raw gMultiuseListMenuTemplate),
            *scrollOffset,
            *selectedRow,
        ) as i16;
        DrawPokeblockMenuHighlight(*selectedRow, TILE_HIGHLIGHT_BLUE);
        ScheduleBgCopyTilemapToVram(0);
        ScheduleBgCopyTilemapToVram(1);
        CloseTossPokeblockWindow(taskId);
    }
}
pub(crate) unsafe fn CloseTossPokeblockWindow(taskId: u8) {
    ClearDialogWindowAndFrameToTransparent(WIN_TOSS_MSG, FALSE);
    ScheduleBgCopyTilemapToVram(1);
    CreateScrollArrows();
    task_set_func(taskId, Some(Task_HandlePokeblockMenuInput));
}
pub(crate) unsafe fn PokeblockAction_UseInBattle(taskId: u8) {
    let nature: u8 = GetNature(crate::gl!(&raw gEnemyParty[0]));
    let gain: i16 = PokeblockGetGain(
        nature,
        &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()],
    );
    StringCopy(
        (*gBattleTextBuff1.as_ptr()).as_mut_ptr(),
        gPokeblockNames[(*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()].color],
    );
    TryClearPokeblock(gSpecialVar_ItemId.get() as u8);
    gSpecialVar_ItemId.set(
        ((*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()].color as u16) << 8,
    );
    if gain == 0 {
        gSpecialVar_ItemId.set(gSpecialVar_ItemId.get() + (1));
    } else if gain > 0 {
        gSpecialVar_ItemId.set(gSpecialVar_ItemId.get() + (2));
    } else {
        gSpecialVar_ItemId.set(gSpecialVar_ItemId.get() + (3));
    }
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe fn PokeblockAction_UseOnPokeblockFeeder(taskId: u8) {
    SafariZoneActivatePokeblockFeeder(gSpecialVar_ItemId.get() as u8);
    StringCopy(
        gStringVar1.as_mut_ptr(),
        gPokeblockNames[(*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()].color],
    );
    gSpecialVar_Result = gSpecialVar_ItemId.get();
    TryClearPokeblock(gSpecialVar_ItemId.get() as u8);
    gSpecialVar_ItemId.set(0);
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe fn PokeblockAction_GiveToContestLady(taskId: u8) {
    gSpecialVar_0x8004 = GivePokeblockToContestLady(
        &raw mut (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[gSpecialVar_ItemId.get()],
    ) as u16;
    gSpecialVar_Result = gSpecialVar_ItemId.get();
    TryClearPokeblock(gSpecialVar_ItemId.get() as u8);
    gSpecialVar_ItemId.set(0);
    FadePaletteAndSetTaskToClosePokeblockCase(taskId);
}
pub(crate) unsafe fn PokeblockAction_Cancel(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ClearStdWindowAndFrameToTransparent(*data.at(1) as u8, FALSE);
    ScheduleBgCopyTilemapToVram(1);
    CreateScrollArrows();
    task_set_func(taskId, Some(Task_HandlePokeblockMenuInput));
}
unsafe fn ClearPokeblock(pkblId: u8) {
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].color = 0;
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].spicy = 0;
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].dry = 0;
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].sweet = 0;
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].bitter = 0;
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].sour = 0;
    (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].feel = 0;
}
#[unsafe(no_mangle)]
pub unsafe fn ClearPokeblocks() {
    for i in 0..(POKEBLOCKS_COUNT as u8) {
        ClearPokeblock(i);
    }
}
pub unsafe fn GetHighestPokeblocksFlavorLevel(pokeblock: *mut Pokeblock) -> u8 {
    let mut maxFlavor: u8 = GetPokeblockData(pokeblock, PBLOCK_SPICY) as u8;
    for i in PBLOCK_SPICY..(FLAVOR_COUNT as u8) {
        let currFlavor: u8 = GetPokeblockData(pokeblock, PBLOCK_SPICY + i) as u8;
        if maxFlavor < currFlavor {
            maxFlavor = currFlavor;
        }
    }
    maxFlavor
}
pub unsafe fn GetPokeblocksFeel(pokeblock: *mut Pokeblock) -> u8 {
    let mut feel: u8 = GetPokeblockData(pokeblock, PBLOCK_FEEL) as u8;
    if feel > POKEBLOCK_MAX_FEEL {
        feel = POKEBLOCK_MAX_FEEL;
    }
    feel
}
#[unsafe(no_mangle)]
pub unsafe fn GetFirstFreePokeblockSlot() -> i8 {
    for i in 0..(POKEBLOCKS_COUNT as u8) {
        if (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[i].color == PBLOCK_CLR_NONE {
            return i as i8;
        }
    }
    -1
}
pub unsafe fn AddPokeblock(pokeblock: *mut Pokeblock) -> u32 {
    let slot: i8 = GetFirstFreePokeblockSlot();
    if slot == -1 {
        return FALSE as u32;
    } else {
        (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[slot] = *pokeblock;
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn TryClearPokeblock(pkblId: u8) -> u32 {
    if (*crate::gl!(gSaveBlock1Ptr)).pokeblocks[pkblId].color == PBLOCK_CLR_NONE {
        return FALSE as u32;
    } else {
        ClearPokeblock(pkblId);
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetPokeblockData(pokeblock: *mut Pokeblock, field: u8) -> i16 {
    if field == PBLOCK_COLOR {
        return (*pokeblock).color as i16;
    }
    if field == PBLOCK_SPICY {
        return (*pokeblock).spicy as i16;
    }
    if field == PBLOCK_DRY {
        return (*pokeblock).dry as i16;
    }
    if field == PBLOCK_SWEET {
        return (*pokeblock).sweet as i16;
    }
    if field == PBLOCK_BITTER {
        return (*pokeblock).bitter as i16;
    }
    if field == PBLOCK_SOUR {
        return (*pokeblock).sour as i16;
    }
    if field == PBLOCK_FEEL {
        return (*pokeblock).feel as i16;
    }
    0
}
pub unsafe fn PokeblockGetGain(nature: u8, pokeblock: *mut Pokeblock) -> i16 {
    let mut curGain: i16 = 0;
    let mut totalGain: i16 = 0;
    for flavor in 0..(FLAVOR_COUNT as u8) {
        curGain = GetPokeblockData(pokeblock, flavor + PBLOCK_SPICY);
        if curGain > 0 {
            totalGain += curGain
                * gPokeblockFlavorCompatibilityTable[FLAVOR_COUNT * nature as i32 + flavor as i32]
                    as i16;
        }
    }
    totalGain
}
pub unsafe fn PokeblockCopyName(pokeblock: *mut Pokeblock, dest: *mut u8) {
    let color: u8 = GetPokeblockData(pokeblock, PBLOCK_COLOR) as u8;
    StringCopy(dest, gPokeblockNames[color]);
}
pub unsafe fn CopyMonFavoritePokeblockName(nature: u8, dest: *mut u8) -> u8 {
    for i in 0..(FLAVOR_COUNT as u8) {
        if PokeblockGetGain(nature, (&raw const sFavoritePokeblocksTable[i]).cast_mut()) > 0 {
            StringCopy(dest, gPokeblockNames[i as i32 + 1]);
            return TRUE;
        }
    }
    FALSE
}
pub unsafe fn GetPokeblocksFlavor(pokeblock: *mut Pokeblock) -> u8 {
    let mut bestFlavor: i16 = 0;
    for i in 0..(FLAVOR_COUNT as i16) {
        if GetPokeblockData(pokeblock, bestFlavor as u8 + 1)
            < GetPokeblockData(pokeblock, i as u8 + 1)
        {
            bestFlavor = i;
        }
    }
    bestFlavor as u8
}
