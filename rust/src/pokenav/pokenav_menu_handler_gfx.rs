//! Translated from `src/pokenav_menu_handler_gfx.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    unused_assignments,
    unused_variables
)]

use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, IsDma3ManagerBusyWithBgCopy, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::fieldmap::gMapHeader;
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::load_save::gSaveBlock1Ptr;
use crate::menu::{
    AddTextPrinterParameterized3, DecompressAndCopyTileDataToVram,
    FreeTempTileDataBuffersIfPossible,
};
use crate::palette::{LoadPalette, TransferPlttBuffer};
use crate::pokenav::{
    AllocSubstruct, FreePokenavSubstruct, GetSubstructPtr, IsLoopedTaskActive,
    SetPokenavVBlankCallback, SetVBlankCallback_,
};
use crate::pokenav_main_menu::{
    AreLeftHeaderSpritesMoving, CopyPaletteIntoBufferUnfaded, HideMainOrSubMenuLeftHeader,
    InitBgTemplates, IsPaletteFadeActive, LoadLeftHeaderGfxForIndex, Pokenav_AllocAndLoadPalettes,
    PokenavCopyPalette, PokenavFadeScreen, PrintHelpBarText, ShowLeftHeaderGfx, SlideMenuHeaderUp,
    WaitForHelpBar,
};
use crate::pokenav_match_call_list::{GetMatchTableMapSectionId, IsRematchEntryRegistered};
use crate::pokenav_menu_handler::{
    GetCurrentMenuItemId, GetHelpBarTextId, GetPokenavCursorPos, GetPokenavMenuType,
};
use crate::scanline_effect::{ScanlineEffect_InitHBlankDmaTransfer, ScanlineEffect_Stop};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    FreeOamMatrix, FreeSpritePaletteByTag, FreeSpriteTilesByTag, GetSpriteTileStartByTag,
    IndexOfSpritePaletteTag, LoadOam, ProcessSpriteCopyRequests,
};
use crate::task::{DestroyTask, GetWordTaskArg, SetWordTaskArg};
use crate::task::{gTasks, task_set};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
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
/// `CalcCenterToCornerVec` with this module's view of its types.
#[inline]
unsafe fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8) {
    unsafe {
        crate::sprite::CalcCenterToCornerVec(a0 as _, a1, a2, a3);
    }
}
/// `CopyToBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16) {
    unsafe {
        crate::bg::CopyToBgTilemapBuffer(a0, a1 as _, a2, a3);
    }
}
/// `CreateLoopedTask` with this module's view of its types.
#[inline]
unsafe fn CreateLoopedTask(a0: Option<unsafe fn(i32) -> u32>, a1: u32) -> u32 {
    unsafe { crate::pokenav::CreateLoopedTask(a0, a1) }
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
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
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
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sSlideTime: usize = 0;
const sZoomDelay: usize = 0;
const sSlideAccel: usize = 1;
const sZoomSetAffine: usize = 1;
const sSlideSpeed: usize = 2;
const sZoomSpeed: usize = 2;
const sSlideEndX: usize = 7;
const sZoomSubspriteId: usize = 7;
// Data tables (translate with cdata.py): sPokenavBgDotsPal sPokenavBgDotsTiles sPokenavBgDotsTilemap sPokenavDeviceBgPal sPokenavDeviceBgTiles sPokenavDeviceBgTilemap sMatchCallBlueLightPal sMatchCallBlueLightTiles sPokenavMainMenuBgTemplates sMenuHandlerLoopTaskFuncs sPokenavOptionsSpriteSheets sPokenavOptionsSpritePalettes sOptionsLabelGfx_RegionMap sOptionsLabelGfx_Condition sOptionsLabelGfx_MatchCall sOptionsLabelGfx_Ribbons sOptionsLabelGfx_SwitchOff sOptionsLabelGfx_Party sOptionsLabelGfx_Search sOptionsLabelGfx_Cool sOptionsLabelGfx_Beauty sOptionsLabelGfx_Cute sOptionsLabelGfx_Smart sOptionsLabelGfx_Tough sOptionsLabelGfx_Cancel sPokenavMenuOptionLabelGfx sOptionDescWindowTemplate sPageDescriptions sOptionDescTextColors sOptionDescTextColors2 sOamData_MenuOption sAffineAnim_MenuOption_Normal sAffineAnim_MenuOption_Zoom sAffineAnims_MenuOption sMenuOptionSpriteTemplate sBlueLightOamData sMatchCallBlueLightSpriteTemplate sPokenavMainMenuScanlineEffectParams

/// `struct Pokenav_MenuGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_MenuGfx {
    pub isTaskActiveCB: Option<unsafe fn() -> u32>,
    pub loopedTaskId: u32,
    pub optionDescWindowId: u16,
    pub bg3ScrollTaskId: u8,
    pub cursorPos: u8,
    pub numIconsBlending: u8,
    pub pokenavAlreadyOpen: u8,
    pub iconVisible: CArray<u32, 6>,
    pub blueLightSprite: *mut Sprite,
    pub iconSprites: CArray<CArray<*mut Sprite, 4>, 6>,
    pub bg1TilemapBuffer: CArray<u8, 2048>,
}

unsafe impl Sync for Pokenav_MenuGfx {}

/// `__typeof__(sPokenavMenuOptionLabelGfx[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPokenavMenuOptionLabelGfx_0_t {
    pub yStart: u16,
    pub deltaY: u16,
    pub gfx: CArray<*mut u16, 6>,
}

unsafe impl Sync for sPokenavMenuOptionLabelGfx_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_MenuGfx>() == 2188);
    assert!(offset_of!(Pokenav_MenuGfx, isTaskActiveCB) == 0);
    assert!(offset_of!(Pokenav_MenuGfx, loopedTaskId) == 4);
    assert!(offset_of!(Pokenav_MenuGfx, optionDescWindowId) == 8);
    assert!(offset_of!(Pokenav_MenuGfx, bg3ScrollTaskId) == 10);
    assert!(offset_of!(Pokenav_MenuGfx, cursorPos) == 11);
    assert!(offset_of!(Pokenav_MenuGfx, numIconsBlending) == 12);
    assert!(offset_of!(Pokenav_MenuGfx, pokenavAlreadyOpen) == 13);
    assert!(offset_of!(Pokenav_MenuGfx, iconVisible) == 16);
    assert!(offset_of!(Pokenav_MenuGfx, blueLightSprite) == 40);
    assert!(offset_of!(Pokenav_MenuGfx, iconSprites) == 44);
    assert!(offset_of!(Pokenav_MenuGfx, bg1TilemapBuffer) == 140);
    assert!(size_of::<sPokenavMenuOptionLabelGfx_0_t>() == 28);
    assert!(offset_of!(sPokenavMenuOptionLabelGfx_0_t, yStart) == 0);
    assert!(offset_of!(sPokenavMenuOptionLabelGfx_0_t, deltaY) == 2);
    assert!(offset_of!(sPokenavMenuOptionLabelGfx_0_t, gfx) == 4);
};

const GFXTAG_BLUE_LIGHT: u16 = 1;
const GFXTAG_OPTIONS: u16 = 3;
const NUM_OPTION_SUBSPRITES: i32 = 4;
const OPTION_DEFAULT_X: i32 = 140;
const OPTION_EXIT_X: i32 = 256;
const OPTION_SELECTED_X: i32 = 130;
const PALTAG_BLUE_LIGHT: u16 = 3;
const PALTAG_OPTIONS_BEIGE: u16 = 7;
const PALTAG_OPTIONS_BLUE: u16 = 5;
const PALTAG_OPTIONS_DEFAULT: u16 = 4;
const PALTAG_OPTIONS_PINK: u16 = 6;
const PALTAG_OPTIONS_RED: u16 = 8;
const PALTAG_OPTIONS_START: u16 = 4;

static sMatchCallBlueLightSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::pokenav_menu_handler_gfx::sMatchCallBlueLightSpriteTemplate).cast(),
);
static sMenuHandlerLoopTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 9>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sMenuHandlerLoopTaskFuncs).cast());
static sMenuOptionSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sMenuOptionSpriteTemplate).cast());
static sOptionDescTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sOptionDescTextColors).cast());
static sOptionDescTextColors2: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sOptionDescTextColors2).cast());
static sOptionDescWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sOptionDescWindowTemplate).cast());
static sPageDescriptions: Table<CArray<*mut u8, 14>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPageDescriptions).cast());
static sPokenavBgDotsPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavBgDotsPal).cast());
static sPokenavBgDotsTilemap: Table<CArray<u32, 40>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavBgDotsTilemap).cast());
static sPokenavBgDotsTiles: Table<CArray<u32, 5>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavBgDotsTiles).cast());
static sPokenavDeviceBgPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavDeviceBgPal).cast());
static sPokenavDeviceBgTilemap: Table<CArray<u32, 68>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavDeviceBgTilemap).cast());
static sPokenavDeviceBgTiles: Table<CArray<u32, 162>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavDeviceBgTiles).cast());
static sPokenavMainMenuBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavMainMenuBgTemplates).cast());
static sPokenavMainMenuScanlineEffectParams: Table<ScanlineEffectParams> = Table(
    (&raw const crate::data::pokenav_menu_handler_gfx::sPokenavMainMenuScanlineEffectParams).cast(),
);
static sPokenavMenuOptionLabelGfx: Table<CArray<sPokenavMenuOptionLabelGfx_0_t, 5>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavMenuOptionLabelGfx).cast());
static sPokenavOptionsSpritePalettes: Table<CArray<SpritePalette, 7>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavOptionsSpritePalettes).cast());
static sPokenavOptionsSpriteSheets: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::pokenav_menu_handler_gfx::sPokenavOptionsSpriteSheets).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

unsafe fn AreAnyTrainerRematchesNearby() -> u32 {
    for i in 0..REMATCH_TABLE_ENTRIES {
        if GetMatchTableMapSectionId(i) == gMapHeader.regionMapSectionId
            && IsRematchEntryRegistered(i) != 0
            && (*gSaveBlock1Ptr).trainerRematches[i] != 0
        {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
pub unsafe fn OpenPokenavMenuInitial() -> u32 {
    let gfx: *mut Pokenav_MenuGfx = OpenPokenavMenu();
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).pokenavAlreadyOpen = FALSE;
    TRUE as u32
}
pub unsafe fn OpenPokenavMenuNotInitial() -> u32 {
    let gfx: *mut Pokenav_MenuGfx = OpenPokenavMenu();
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).pokenavAlreadyOpen = TRUE;
    TRUE as u32
}
unsafe fn OpenPokenavMenu() -> *mut Pokenav_MenuGfx {
    let gfx: *mut Pokenav_MenuGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_MENU_GFX, 2188) as *mut Pokenav_MenuGfx;
    if !gfx.is_null() {
        (*gfx).numIconsBlending = 0;
        (*gfx).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenMenu), 1);
        (*gfx).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
    }
    gfx
}
pub unsafe fn CreateMenuHandlerLoopedTask(ltIdx: i32) {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).loopedTaskId = CreateLoopedTask(sMenuHandlerLoopTaskFuncs[ltIdx], 1);
    (*gfx).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
}
pub unsafe fn IsMenuHandlerLoopedTaskActive() -> u32 {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).isTaskActiveCB.unwrap_unchecked()()
}
pub unsafe fn FreeMenuHandlerSubstruct2() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    DestroyMovingDotsBgTask();
    RemoveWindow((*gfx).optionDescWindowId as u8);
    FreeAndDestroyMainMenuSprites();
    DestroyMenuOptionGlowTask();
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_MENU_GFX);
}
pub(crate) unsafe fn GetCurrentLoopedTaskActive() -> u32 {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    IsLoopedTaskActive((*gfx).loopedTaskId)
}
pub(crate) unsafe fn LoopedTask_OpenMenu(state: i32) -> u32 {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    match state {
        0 => {
            InitBgTemplates(sPokenavMainMenuBgTemplates.as_ptr().cast_mut(), 3);
            DecompressAndCopyTileDataToVram(
                1,
                (*(&raw const crate::data::graphics::gPokenavMessageBox_Gfx)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(1, (*gfx).bg1TilemapBuffer.as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                1,
                (*(&raw const crate::data::graphics::gPokenavMessageBox_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyPaletteIntoBufferUnfaded(
                (*(&raw const crate::data::graphics::gPokenavMessageBox_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut(),
                16,
                32,
            );
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            DecompressAndCopyTileDataToVram(
                2,
                sPokenavDeviceBgTiles.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                2,
                sPokenavDeviceBgTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            CopyPaletteIntoBufferUnfaded(sPokenavDeviceBgPal.as_ptr().cast_mut(), 32, 32);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            DecompressAndCopyTileDataToVram(
                3,
                sPokenavBgDotsTiles.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            DecompressAndCopyTileDataToVram(
                3,
                sPokenavBgDotsTilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                1,
            );
            CopyPaletteIntoBufferUnfaded(sPokenavBgDotsPal.as_ptr().cast_mut(), 48, 32);
            if GetPokenavMenuType() == POKENAV_MENU_TYPE_CONDITION as i32
                || GetPokenavMenuType() == POKENAV_MENU_TYPE_CONDITION_SEARCH
            {
                ChangeBgDotsColorToPurple();
            }
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            AddOptionDescriptionWindow();
            CreateMovingBgDotsTask();
            return LT_INC_AND_CONTINUE;
        }
        4 => {
            LoadPokenavOptionPalettes();
            return LT_INC_AND_CONTINUE;
        }
        5 => {
            PrintCurrentOptionDescription();
            CreateMenuOptionSprites();
            CreateMatchCallBlueLightSprite();
            DrawCurrentMenuOptionLabels();
            return LT_INC_AND_PAUSE;
        }
        6 => {
            if IsDma3ManagerBusyWithBgCopy_() != 0 {
                return LT_PAUSE;
            }
            return LT_INC_AND_CONTINUE;
        }
        7 => {
            ShowBg(1);
            ShowBg(2);
            ShowBg(3);
            if (*gfx).pokenavAlreadyOpen != 0 {
                PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
            } else {
                PlaySE(SE_POKENAV_ON);
                PokenavFadeScreen(POKENAV_FADE_FROM_BLACK_ALL);
            }
            'l2: {
                let sw1: i32 = GetPokenavMenuType();
                let matched = sw1 == POKENAV_MENU_TYPE_CONDITION_SEARCH || sw1 == 3;
                let mut fall = false;
                if sw1 == POKENAV_MENU_TYPE_CONDITION_SEARCH {
                    fall = true;
                    LoadLeftHeaderGfxForIndex(7);
                }
                if fall || sw1 == 3 {
                    LoadLeftHeaderGfxForIndex(1);
                    break 'l2;
                }
                if !matched {
                    LoadLeftHeaderGfxForIndex(0);
                    break 'l2;
                }
            }
            return LT_INC_AND_PAUSE;
        }
        8 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
            'l3: {
                let sw2: i32 = GetPokenavMenuType();
                let matched = sw2 == POKENAV_MENU_TYPE_CONDITION_SEARCH || sw2 == 3;
                let mut fall = false;
                if sw2 == POKENAV_MENU_TYPE_CONDITION_SEARCH {
                    fall = true;
                    ShowLeftHeaderGfx(7, FALSE as u32, FALSE as u32);
                }
                if fall || sw2 == 3 {
                    ShowLeftHeaderGfx(1, FALSE as u32, FALSE as u32);
                    break 'l3;
                }
                if !matched {
                    ShowLeftHeaderGfx(0, 0, 0);
                    break 'l3;
                }
            }
            StartOptionAnimations_Enter();
            SetupPokenavMenuScanlineEffects();
            return LT_INC_AND_CONTINUE;
        }
        9 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_MoveMenuCursor(state: i32) -> u32 {
    match state {
        0 => {
            SetMenuOptionGlow();
            StartOptionAnimations_CursorMoved();
            PrintCurrentOptionDescription();
            PlaySE(SE_SELECT);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if IsDma3ManagerBusyWithBgCopy_() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_OpenConditionMenu(state: i32) -> u32 {
    match state {
        0 => {
            ResetBldCnt();
            StartOptionAnimations_Exit();
            HideMainOrSubMenuLeftHeader(POKENAV_GFX_MAIN_MENU, FALSE as u32);
            PlaySE(SE_SELECT);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            DrawCurrentMenuOptionLabels();
            LoadLeftHeaderGfxForIndex(1);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            StartOptionAnimations_Enter();
            ShowLeftHeaderGfx(1, FALSE as u32, FALSE as u32);
            CreateBgDotPurplePalTask();
            PrintCurrentOptionDescription();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if IsTaskActive_UpdateBgDotsPalette() != 0 {
                return LT_PAUSE;
            }
            if IsDma3ManagerBusyWithBgCopy_() != 0 {
                return LT_PAUSE;
            }
            InitMenuOptionGlow();
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_ReturnToMainMenu(state: i32) -> u32 {
    match state {
        0 => {
            ResetBldCnt();
            StartOptionAnimations_Exit();
            HideMainOrSubMenuLeftHeader(POKENAV_GFX_CONDITION_MENU, FALSE as u32);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            DrawCurrentMenuOptionLabels();
            LoadLeftHeaderGfxForIndex(0);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            StartOptionAnimations_Enter();
            ShowLeftHeaderGfx(0, 0, 0);
            CreateBgDotLightBluePalTask();
            PrintCurrentOptionDescription();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if IsTaskActive_UpdateBgDotsPalette() != 0 {
                return LT_PAUSE;
            }
            if IsDma3ManagerBusyWithBgCopy_() != 0 {
                return LT_PAUSE;
            }
            InitMenuOptionGlow();
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_OpenConditionSearchMenu(state: i32) -> u32 {
    match state {
        0 => {
            ResetBldCnt();
            StartOptionAnimations_Exit();
            PlaySE(SE_SELECT);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            LoadLeftHeaderGfxForIndex(7);
            DrawCurrentMenuOptionLabels();
            return LT_INC_AND_PAUSE;
        }
        2 => {
            StartOptionAnimations_Enter();
            ShowLeftHeaderGfx(7, FALSE as u32, FALSE as u32);
            PrintCurrentOptionDescription();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if IsTaskActive_UpdateBgDotsPalette() != 0 {
                return LT_PAUSE;
            }
            InitMenuOptionGlow();
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_ReturnToConditionMenu(state: i32) -> u32 {
    match state {
        0 => {
            ResetBldCnt();
            StartOptionAnimations_Exit();
            HideMainOrSubMenuLeftHeader(POKENAV_GFX_SEARCH_MENU, FALSE as u32);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            DrawCurrentMenuOptionLabels();
            return LT_INC_AND_PAUSE;
        }
        2 => {
            StartOptionAnimations_Enter();
            PrintCurrentOptionDescription();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if IsTaskActive_UpdateBgDotsPalette() != 0 {
                return LT_PAUSE;
            }
            InitMenuOptionGlow();
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_SelectRibbonsNoWinners(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_FAILURE);
            PrintNoRibbonWinners();
            return LT_INC_AND_PAUSE;
        }
        1 if IsDma3ManagerBusyWithBgCopy() != 0 => {
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_ReShowDescription(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PrintCurrentOptionDescription();
            return LT_INC_AND_PAUSE;
        }
        1 if IsDma3ManagerBusyWithBgCopy() != 0 => {
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_OpenPokenavFeature(state: i32) -> u32 {
    match state {
        0 => {
            PrintHelpBarText(GetHelpBarTextId() as u32);
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if WaitForHelpBar() != 0 {
                return LT_PAUSE;
            }
            SlideMenuHeaderUp();
            ResetBldCnt();
            StartOptionAnimations_Exit();
            'l2: {
                let sw1: i32 = GetPokenavMenuType();
                let matched = sw1 == POKENAV_MENU_TYPE_CONDITION_SEARCH || sw1 == 3;
                let mut fall = false;
                if sw1 == POKENAV_MENU_TYPE_CONDITION_SEARCH {
                    fall = true;
                    HideMainOrSubMenuLeftHeader(POKENAV_GFX_SEARCH_MENU, FALSE as u32);
                }
                if fall || sw1 == 3 {
                    HideMainOrSubMenuLeftHeader(POKENAV_GFX_CONDITION_MENU, FALSE as u32);
                    break 'l2;
                }
                if !matched {
                    HideMainOrSubMenuLeftHeader(POKENAV_GFX_MAIN_MENU, FALSE as u32);
                    break 'l2;
                }
            }
            PlaySE(SE_SELECT);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if AreMenuOptionSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            if AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            return LT_INC_AND_PAUSE;
        }
        3 if IsPaletteFadeActive() != 0 => {
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
unsafe fn LoadPokenavOptionPalettes() {
    for i in 0..2i32 {
        LoadCompressedSpriteSheet((&raw const sPokenavOptionsSpriteSheets[i]).cast_mut());
    }
    Pokenav_AllocAndLoadPalettes(sPokenavOptionsSpritePalettes.as_ptr().cast_mut());
}
unsafe fn FreeAndDestroyMainMenuSprites() {
    FreeSpriteTilesByTag(GFXTAG_OPTIONS);
    FreeSpriteTilesByTag(GFXTAG_BLUE_LIGHT);
    FreeSpritePaletteByTag(PALTAG_OPTIONS_DEFAULT);
    FreeSpritePaletteByTag(PALTAG_OPTIONS_BLUE);
    FreeSpritePaletteByTag(PALTAG_OPTIONS_PINK);
    FreeSpritePaletteByTag(PALTAG_OPTIONS_BEIGE);
    FreeSpritePaletteByTag(PALTAG_OPTIONS_RED);
    FreeSpritePaletteByTag(PALTAG_BLUE_LIGHT);
    DestroyMenuOptionSprites();
    DestroyRematchBlueLightSprite();
}
unsafe fn CreateMenuOptionSprites() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    for i in 0..MAX_POKENAV_MENUITEMS {
        for j in 0..NUM_OPTION_SUBSPRITES {
            let spriteId: u8 = CreateSprite(
                (&raw const *sMenuOptionSpriteTemplate).cast_mut(),
                0x8c,
                20 * i as i16 + 40,
                3,
            );
            (*gfx).iconSprites[i][j] = &raw mut gSprites[spriteId];
            gSprites[spriteId].x2 = 32 * j as i16;
        }
    }
}
unsafe fn DestroyMenuOptionSprites() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    for i in 0..MAX_POKENAV_MENUITEMS {
        for j in 0..NUM_OPTION_SUBSPRITES {
            FreeSpriteOamMatrix((*gfx).iconSprites[i][j]);
            DestroySprite((*gfx).iconSprites[i][j]);
        }
    }
}
unsafe fn DrawCurrentMenuOptionLabels() {
    let menuType: i32 = GetPokenavMenuType();
    DrawOptionLabelGfx(
        sPokenavMenuOptionLabelGfx[menuType].gfx.as_ptr().cast_mut(),
        sPokenavMenuOptionLabelGfx[menuType].yStart as i32,
        sPokenavMenuOptionLabelGfx[menuType].deltaY as i32,
    );
}
unsafe fn DrawOptionLabelGfx(mut optionGfx: *mut *mut u16, mut yPos: i32, deltaY: i32) {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let baseTile: i32 = GetSpriteTileStartByTag(GFXTAG_OPTIONS) as i32;
    for i in 0..MAX_POKENAV_MENUITEMS {
        if !(*optionGfx).is_null() {
            for j in 0..NUM_OPTION_SUBSPRITES {
                (*(*gfx).iconSprites[i][j])
                    .oam
                    .set_tileNum(*(*optionGfx) + baseTile as u16 + 8 * j as u16);
                (*(*gfx).iconSprites[i][j])
                    .oam
                    .set_paletteNum(IndexOfSpritePaletteTag(
                        *(*optionGfx).at(1) + PALTAG_OPTIONS_START,
                    ) as u16);
                (*(*gfx).iconSprites[i][j]).set_invisible(TRUE as u16);
                (*(*gfx).iconSprites[i][j]).y = yPos as i16;
                (*(*gfx).iconSprites[i][j]).x = OPTION_DEFAULT_X as i16;
                (*(*gfx).iconSprites[i][j]).x2 = 32 * j as i16;
            }
            (*gfx).iconVisible[i] = TRUE as u32;
        } else {
            for j in 0..NUM_OPTION_SUBSPRITES {
                (*(*gfx).iconSprites[i][j]).set_invisible(TRUE as u16);
            }
            (*gfx).iconVisible[i] = FALSE as u32;
        }
        optionGfx = optionGfx.at(1);
        yPos += deltaY;
    }
}
unsafe fn StartOptionAnimations_Enter() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let cursorPos: i32 = GetPokenavCursorPos();
    let mut iconCount: i32 = 0;
    let mut x: i32 = 0;
    for i in 0..MAX_POKENAV_MENUITEMS {
        if (*gfx).iconVisible[i] != 0 {
            if ({
                let t1 = iconCount;
                iconCount += 1;
                t1
            }) == cursorPos
            {
                x = OPTION_SELECTED_X;
                (*gfx).cursorPos = i as u8;
            } else {
                x = OPTION_DEFAULT_X;
            }
            StartOptionSlide((*gfx).iconSprites[i].as_mut_ptr(), OPTION_EXIT_X, x, 12);
            SetOptionInvisibility((*gfx).iconSprites[i].as_mut_ptr(), FALSE as u32);
        } else {
            SetOptionInvisibility((*gfx).iconSprites[i].as_mut_ptr(), TRUE as u32);
        }
    }
}
unsafe fn StartOptionAnimations_CursorMoved() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let prevPos: i32 = GetPokenavCursorPos();
    let mut newPos: i32 = 0;
    for i in 0..MAX_POKENAV_MENUITEMS {
        if (*gfx).iconVisible[i] != 0 {
            if newPos == prevPos {
                newPos = i;
                break;
            }
            newPos += 1;
        }
    }
    StartOptionSlide(
        (*gfx).iconSprites[(*gfx).cursorPos].as_mut_ptr(),
        OPTION_SELECTED_X,
        OPTION_DEFAULT_X,
        4,
    );
    StartOptionSlide(
        (*gfx).iconSprites[newPos].as_mut_ptr(),
        OPTION_DEFAULT_X,
        OPTION_SELECTED_X,
        4,
    );
    (*gfx).cursorPos = newPos as u8;
}
unsafe fn StartOptionAnimations_Exit() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    for i in 0..MAX_POKENAV_MENUITEMS {
        if (*gfx).iconVisible[i] != 0 {
            if (*gfx).cursorPos as i32 != i {
                StartOptionSlide(
                    (*gfx).iconSprites[i].as_mut_ptr(),
                    OPTION_DEFAULT_X,
                    OPTION_EXIT_X,
                    8,
                );
            } else {
                StartOptionZoom((*gfx).iconSprites[i].as_mut_ptr());
            }
        }
    }
}
unsafe fn AreMenuOptionSpritesMoving() -> u32 {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    for i in 0..MAX_POKENAV_MENUITEMS {
        if (*(*gfx).iconSprites[i][0]).callback
            != Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        {
            return TRUE as u32;
        }
    }
    if (*gfx).numIconsBlending != 0 {
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn StartOptionSlide(mut sprites: *mut *mut Sprite, startX: i32, endX: i32, time: i32) {
    for i in 0..NUM_OPTION_SUBSPRITES {
        (*(*sprites)).x = startX as i16;
        (*(*sprites)).data[sSlideTime] = time as i16;
        (*(*sprites)).data[sSlideAccel] = div_i32(16 * (endX - startX), time) as i16;
        (*(*sprites)).data[sSlideSpeed] = 16 * startX as i16;
        (*(*sprites)).data[sSlideEndX] = endX as i16;
        (*(*sprites)).callback = Some(SpriteCB_OptionSlide);
        sprites = sprites.at(1);
    }
}
unsafe fn StartOptionZoom(mut sprites: *mut *mut Sprite) {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    for i in 0..NUM_OPTION_SUBSPRITES {
        (*(*sprites)).oam.set_objMode(ST_OAM_OBJ_BLEND);
        (*(*sprites)).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
        (*(*sprites)).callback = Some(SpriteCB_OptionZoom);
        (*(*sprites)).data[0] = 8;
        (*(*sprites)).data[sZoomSetAffine] = FALSE as i16;
        (*(*sprites)).data[sZoomSubspriteId] = i as i16;
        InitSpriteAffineAnim(*sprites);
        StartSpriteAffineAnim(*sprites, 0);
        sprites = sprites.at(1);
    }
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    let taskId: u8 = CreateTask(Some(Task_OptionBlend), 3);
    task_set(taskId, 0, 8);
    (*gfx).numIconsBlending += 1;
}
unsafe fn SetOptionInvisibility(mut sprites: *mut *mut Sprite, invisible: u32) {
    for i in 0..NUM_OPTION_SUBSPRITES {
        (*(*sprites)).set_invisible(invisible as u16);
        sprites = sprites.at(1);
    }
}
pub(crate) unsafe fn SpriteCB_OptionSlide(sprite: *mut Sprite) {
    (*sprite).data[sSlideTime] -= 1;
    if (*sprite).data[sSlideTime] != -1 {
        (*sprite).data[sSlideSpeed] += (*sprite).data[sSlideAccel];
        (*sprite).x = (*sprite).data[sSlideSpeed] >> 4;
    } else {
        (*sprite).x = (*sprite).data[sSlideEndX];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe fn SpriteCB_OptionZoom(sprite: *mut Sprite) {
    let mut temp: i32 = 0;
    let mut x: i32 = 0;
    if (*sprite).data[sZoomDelay] == 0 {
        if (*sprite).data[sZoomSetAffine] == 0 {
            StartSpriteAffineAnim(sprite, 1);
            (*sprite).data[sZoomSetAffine] += 1;
            (*sprite).data[sZoomSpeed] = 0x100;
            (*sprite).x += (*sprite).x2;
            (*sprite).x2 = 0;
        } else {
            (*sprite).data[sZoomSpeed] += 16;
            temp = (*sprite).data[sZoomSpeed] as i32;
            x = temp >> 3;
            x = (x - 32) / 2;
            match (*sprite).data[sZoomSubspriteId] {
                0 => {
                    (*sprite).x2 = -(x as i16) * 3;
                }
                1 => {
                    (*sprite).x2 = -(x as i16);
                }
                2 => {
                    (*sprite).x2 = x as i16;
                }
                3 => {
                    (*sprite).x2 = x as i16 * 3;
                }
                _ => {}
            }
            if (*sprite).affineAnimEnded() != 0 {
                (*sprite).set_invisible(TRUE as u16);
                FreeOamMatrix((*sprite).oam.matrixNum() as u8);
                CalcCenterToCornerVec(
                    sprite,
                    (*sprite).oam.shape() as u8,
                    (*sprite).oam.size() as u8,
                    ST_OAM_AFFINE_OFF as u8,
                );
                (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
                (*sprite).oam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
                (*sprite).callback = Some(SpriteCallbackDummy);
            }
        }
    } else {
        (*sprite).data[sZoomDelay] -= 1;
    }
}
pub(crate) unsafe fn Task_OptionBlend(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data == 0 {
        match *data.at(1) {
            0 => {
                *data.at(2) = 16;
                *data.at(3) = 0;
                SetGpuReg(REG_OFFSET_BLDCNT, BLDCNT_TGT2_ALL);
                SetGpuReg(REG_OFFSET_BLDALPHA, 16);
                *data.at(1) += 1;
            }
            1 => {
                if *data.at(4) as i32 & 1 != 0 {
                    *data.at(2) -= 3;
                    if *data.at(2) < 0 {
                        *data.at(2) = 0;
                    }
                } else {
                    *data.at(3) += 3;
                    if *data.at(3) > 16 {
                        *data.at(3) = 16;
                    }
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (*data.at(3) as u16) << 8 | *data.at(2) as u16,
                );
                *data.at(4) += 1;
                if *data.at(4) == 12 {
                    let p1 = &raw mut (*(GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX)
                        as *mut Pokenav_MenuGfx))
                        .numIconsBlending;
                    *p1 -= 1;
                    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
                    DestroyTask(taskId);
                }
            }
            _ => {}
        }
    } else {
        *data -= 1;
    }
}
unsafe fn CreateMatchCallBlueLightSprite() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let spriteId: u8 = CreateSprite(
        (&raw const *sMatchCallBlueLightSpriteTemplate).cast_mut(),
        0x10,
        0x60,
        4,
    );
    (*gfx).blueLightSprite = &raw mut gSprites[spriteId];
    if AreAnyTrainerRematchesNearby() != 0 {
        (*(*gfx).blueLightSprite).callback = Some(SpriteCB_BlinkingBlueLight);
    } else {
        (*(*gfx).blueLightSprite).set_invisible(TRUE as u16);
    }
}
unsafe fn DestroyRematchBlueLightSprite() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    DestroySprite((*gfx).blueLightSprite);
}
pub(crate) unsafe fn SpriteCB_BlinkingBlueLight(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if (*sprite).data[0] > 8 {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
}
unsafe fn AddOptionDescriptionWindow() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).optionDescWindowId = AddWindow((&raw const *sOptionDescWindowTemplate).cast_mut());
    PutWindowTilemap((*gfx).optionDescWindowId as u8);
    FillWindowPixelBuffer((*gfx).optionDescWindowId as u8, 102);
    CopyWindowToVram((*gfx).optionDescWindowId as u8, COPYWIN_FULL);
}
unsafe fn PrintCurrentOptionDescription() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let menuItem: i32 = GetCurrentMenuItemId();
    let desc: *mut u8 = sPageDescriptions[menuItem];
    let width: u32 = GetStringWidth(FONT_NORMAL, desc, -1) as u32;
    FillWindowPixelBuffer((*gfx).optionDescWindowId as u8, 102);
    AddTextPrinterParameterized3(
        (*gfx).optionDescWindowId as u8,
        FONT_NORMAL,
        ((192 - width) / 2) as u8,
        1,
        sOptionDescTextColors.as_ptr().cast_mut(),
        0,
        desc,
    );
}
unsafe fn PrintNoRibbonWinners() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let s: *mut u8 = (*(&raw const crate::data::strings::gText_NoRibbonWinners)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    let width: u32 = GetStringWidth(FONT_NORMAL, s, -1) as u32;
    FillWindowPixelBuffer((*gfx).optionDescWindowId as u8, 102);
    AddTextPrinterParameterized3(
        (*gfx).optionDescWindowId as u8,
        FONT_NORMAL,
        ((192 - width) / 2) as u8,
        1,
        sOptionDescTextColors2.as_ptr().cast_mut(),
        0,
        s,
    );
}
pub(crate) unsafe fn IsDma3ManagerBusyWithBgCopy_() -> u32 {
    IsDma3ManagerBusyWithBgCopy() as u32
}
unsafe fn CreateMovingBgDotsTask() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).bg3ScrollTaskId = CreateTask(Some(Task_MoveBgDots), 2);
}
unsafe fn DestroyMovingDotsBgTask() {
    let gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    DestroyTask((*gfx).bg3ScrollTaskId);
}
pub(crate) unsafe fn Task_MoveBgDots(taskId: u8) {
    ChangeBgX(3, 0x80, BG_COORD_ADD);
}
unsafe fn CreateBgDotPurplePalTask() {
    let taskId: u8 = CreateTask(Some(Task_UpdateBgDotsPalette), 3);
    SetWordTaskArg(
        taskId,
        1,
        sPokenavBgDotsPal.as_ptr().cast_mut().at(1) as usize as u32,
    );
    SetWordTaskArg(
        taskId,
        3,
        sPokenavBgDotsPal.as_ptr().cast_mut().at(7) as usize as u32,
    );
}
unsafe fn ChangeBgDotsColorToPurple() {
    CopyPaletteIntoBufferUnfaded(sPokenavBgDotsPal.as_ptr().cast_mut().at(7), 49, 4);
}
unsafe fn CreateBgDotLightBluePalTask() {
    let taskId: u8 = CreateTask(Some(Task_UpdateBgDotsPalette), 3);
    SetWordTaskArg(
        taskId,
        1,
        sPokenavBgDotsPal.as_ptr().cast_mut().at(7) as usize as u32,
    );
    SetWordTaskArg(
        taskId,
        3,
        sPokenavBgDotsPal.as_ptr().cast_mut().at(1) as usize as u32,
    );
}
unsafe fn IsTaskActive_UpdateBgDotsPalette() -> u32 {
    FuncIsActiveTask(Some(Task_UpdateBgDotsPalette)) as u32
}
pub(crate) unsafe fn Task_UpdateBgDotsPalette(taskId: u8) {
    let mut sp8: CArray<u16, 2> = zeroed();
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let pal1: *mut u16 = GetWordTaskArg(taskId, 1) as usize as *mut u16;
    let pal2: *mut u16 = GetWordTaskArg(taskId, 3) as usize as *mut u16;
    PokenavCopyPalette(
        pal1,
        pal2,
        2,
        12,
        ({
            *data += 1;
            *data
        }) as i32,
        sp8.as_mut_ptr(),
    );
    LoadPalette(sp8.as_mut_ptr() as *mut c_void, 49, 4);
    if *data == 12 {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn VBlankCB_PokenavMainMenu() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
    ScanlineEffect_InitHBlankDmaTransfer();
}
unsafe fn SetupPokenavMenuScanlineEffects() {
    SetGpuReg(REG_OFFSET_BLDCNT, 144);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
    SetGpuRegBits(REG_OFFSET_WININ, WININ_WIN0_ALL);
    SetGpuRegBits(REG_OFFSET_WINOUT, 31);
    SetGpuRegBits(REG_OFFSET_WIN0V, DISPLAY_HEIGHT);
    ScanlineEffect_Stop();
    SetMenuOptionGlow();
    ScanlineEffect_SetParams(*sPokenavMainMenuScanlineEffectParams);
    SetVBlankCallback_(Some(VBlankCB_PokenavMainMenu));
    CreateTask(Some(Task_CurrentMenuOptionGlow), 3);
}
unsafe fn DestroyMenuOptionGlowTask() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
    ScanlineEffect_Stop();
    DestroyTask(FindTaskIdByFunc(Some(Task_CurrentMenuOptionGlow)));
    SetPokenavVBlankCallback();
}
unsafe fn ResetBldCnt() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
unsafe fn InitMenuOptionGlow() {
    SetMenuOptionGlow();
    SetGpuReg(REG_OFFSET_BLDCNT, 144);
}
pub(crate) unsafe fn Task_CurrentMenuOptionGlow(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data += 1;
    if *data > 0 {
        *data = 0;
        *data.at(1) += 3;
        *data.at(1) &= 0x7F;
        SetGpuReg(
            REG_OFFSET_BLDY,
            ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[*data.at(1)] >> 5)
                as u16,
        );
    }
}
unsafe fn SetMenuOptionGlow() {
    let menuType: i32 = GetPokenavMenuType();
    let cursorPos: i32 = GetPokenavCursorPos();
    let r4: i32 = sPokenavMenuOptionLabelGfx[menuType].deltaY as i32 * cursorPos
        + sPokenavMenuOptionLabelGfx[menuType].yStart as i32
        - 8;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0]
                    .as_mut_ptr() as *mut c_void,
                0x10000a0,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[1]
                    .as_mut_ptr() as *mut c_void,
                0x10000a0,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 29424);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0][r4] as *mut c_void,
                0x1000010,
            );
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 29424);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[1][r4] as *mut c_void,
                0x1000010,
            );
        }
    }
}
pub unsafe fn ResetBldCnt_() {
    ResetBldCnt();
}
