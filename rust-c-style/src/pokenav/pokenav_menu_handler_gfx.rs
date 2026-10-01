//! Translated from `src/pokenav_menu_handler_gfx.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sPokenavBgDotsPal sPokenavBgDotsTiles sPokenavBgDotsTilemap sPokenavDeviceBgPal sPokenavDeviceBgTiles sPokenavDeviceBgTilemap sMatchCallBlueLightPal sMatchCallBlueLightTiles sPokenavMainMenuBgTemplates sMenuHandlerLoopTaskFuncs sPokenavOptionsSpriteSheets sPokenavOptionsSpritePalettes sOptionsLabelGfx_RegionMap sOptionsLabelGfx_Condition sOptionsLabelGfx_MatchCall sOptionsLabelGfx_Ribbons sOptionsLabelGfx_SwitchOff sOptionsLabelGfx_Party sOptionsLabelGfx_Search sOptionsLabelGfx_Cool sOptionsLabelGfx_Beauty sOptionsLabelGfx_Cute sOptionsLabelGfx_Smart sOptionsLabelGfx_Tough sOptionsLabelGfx_Cancel sPokenavMenuOptionLabelGfx sOptionDescWindowTemplate sPageDescriptions sOptionDescTextColors sOptionDescTextColors2 sOamData_MenuOption sAffineAnim_MenuOption_Normal sAffineAnim_MenuOption_Zoom sAffineAnims_MenuOption sMenuOptionSpriteTemplate sBlueLightOamData sMatchCallBlueLightSpriteTemplate sPokenavMainMenuScanlineEffectParams

/// `struct Pokenav_MenuGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_MenuGfx {
    pub isTaskActiveCB: Option<unsafe extern "C" fn() -> u32>,
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
static sMenuHandlerLoopTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(i32) -> u32>, 9>> =
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

unsafe extern "C" {
    static mut gMapHeader: MapHeader;
    static gPokenavMessageBox_Gfx: CArray<u32, 0>;
    static gPokenavMessageBox_Pal: CArray<u16, 0>;
    static gPokenavMessageBox_Tilemap: CArray<u32, 0>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_NoRibbonWinners: CArray<u8, 0>;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FreeOamMatrix(a0: u8);
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetCurrentMenuItemId() -> i32;
    fn GetHelpBarTextId() -> u16;
    fn GetMatchTableMapSectionId(a0: i32) -> u8;
    fn GetPokenavCursorPos() -> i32;
    fn GetPokenavMenuType() -> i32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn HideMainOrSubMenuLeftHeader(a0: u32, a1: u32);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgTemplates(a0: *mut BgTemplate, a1: i32);
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn IsRematchEntryRegistered(a0: i32) -> u32;
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PokenavCopyPalette(a0: *mut u16, a1: *mut u16, a2: i32, a3: i32, a4: i32, a5: *mut u16);
    fn PokenavFadeScreen(a0: i32);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut SpritePalette);
    fn PrintHelpBarText(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetPokenavVBlankCallback();
    fn SetVBlankCallback_(a0: Option<unsafe extern "C" fn()>);
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderUp();
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn TransferPlttBuffer();
    fn WaitForHelpBar() -> u32;
}

pub(crate) unsafe extern "C" fn AreAnyTrainerRematchesNearby() -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < REMATCH_TABLE_ENTRIES {
        if GetMatchTableMapSectionId(i) == gMapHeader.regionMapSectionId
            && IsRematchEntryRegistered(i) != 0
            && (*gSaveBlock1Ptr).trainerRematches[i] != 0
        {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavMenuInitial() -> u32 {
    let mut gfx: *mut Pokenav_MenuGfx = OpenPokenavMenu();
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).pokenavAlreadyOpen = FALSE;
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenPokenavMenuNotInitial() -> u32 {
    let mut gfx: *mut Pokenav_MenuGfx = OpenPokenavMenu();
    if gfx.is_null() {
        return FALSE as u32;
    }
    (*gfx).pokenavAlreadyOpen = TRUE;
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn OpenPokenavMenu() -> *mut Pokenav_MenuGfx {
    let mut gfx: *mut Pokenav_MenuGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_MENU_GFX, 2188) as *mut Pokenav_MenuGfx;
    if !gfx.is_null() {
        (*gfx).numIconsBlending = 0;
        (*gfx).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenMenu), 1);
        (*gfx).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
    }
    return gfx;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateMenuHandlerLoopedTask(ltIdx: i32) {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).loopedTaskId = CreateLoopedTask(sMenuHandlerLoopTaskFuncs[ltIdx], 1);
    (*gfx).isTaskActiveCB = Some(GetCurrentLoopedTaskActive);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMenuHandlerLoopedTaskActive() -> u32 {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    return (*gfx).isTaskActiveCB.unwrap_unchecked()();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeMenuHandlerSubstruct2() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    DestroyMovingDotsBgTask();
    RemoveWindow((*gfx).optionDescWindowId as u8);
    FreeAndDestroyMainMenuSprites();
    DestroyMenuOptionGlowTask();
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_MENU_GFX);
}
pub(crate) unsafe extern "C" fn GetCurrentLoopedTaskActive() -> u32 {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    return IsLoopedTaskActive((*gfx).loopedTaskId);
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenMenu(state: i32) -> u32 {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    match state {
        0 => {
            InitBgTemplates(sPokenavMainMenuBgTemplates.as_ptr().cast_mut(), 3);
            DecompressAndCopyTileDataToVram(
                1,
                gPokenavMessageBox_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(1, (*gfx).bg1TilemapBuffer.as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                1,
                gPokenavMessageBox_Tilemap.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyBgTilemapBufferToVram(1);
            CopyPaletteIntoBufferUnfaded(gPokenavMessageBox_Pal.as_ptr().cast_mut(), 16, 32);
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
                    fall = true;
                    LoadLeftHeaderGfxForIndex(1);
                    break 'l2;
                }
                if !matched {
                    fall = true;
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
                    fall = true;
                    ShowLeftHeaderGfx(1, FALSE as u32, FALSE as u32);
                    break 'l3;
                }
                if !matched {
                    fall = true;
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveMenuCursor(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenConditionMenu(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ReturnToMainMenu(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenConditionSearchMenu(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ReturnToConditionMenu(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_SelectRibbonsNoWinners(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_FAILURE);
            PrintNoRibbonWinners();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ReShowDescription(state: i32) -> u32 {
    match state {
        0 => {
            PlaySE(SE_SELECT);
            PrintCurrentOptionDescription();
            return LT_INC_AND_PAUSE;
        }
        1 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenPokenavFeature(state: i32) -> u32 {
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
                    fall = true;
                    HideMainOrSubMenuLeftHeader(POKENAV_GFX_CONDITION_MENU, FALSE as u32);
                    break 'l2;
                }
                if !matched {
                    fall = true;
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
        3 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
        }
        _ => {}
    }
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoadPokenavOptionPalettes() {
    let mut i: i32 = 0;
    i = 0;
    while i < 2 {
        LoadCompressedSpriteSheet((&raw const sPokenavOptionsSpriteSheets[i]).cast_mut());
        i += 1;
    }
    Pokenav_AllocAndLoadPalettes(sPokenavOptionsSpritePalettes.as_ptr().cast_mut());
}
pub(crate) unsafe extern "C" fn FreeAndDestroyMainMenuSprites() {
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
pub(crate) unsafe extern "C" fn CreateMenuOptionSprites() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    i = 0;
    while i < MAX_POKENAV_MENUITEMS {
        j = 0;
        while j < NUM_OPTION_SUBSPRITES {
            let mut spriteId: u8 = CreateSprite(
                (&raw const *sMenuOptionSpriteTemplate).cast_mut(),
                0x8c,
                20 * i as i16 + 40,
                3,
            );
            (*gfx).iconSprites[i][j] = &raw mut gSprites[spriteId];
            gSprites[spriteId].x2 = 32 * j as i16;
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DestroyMenuOptionSprites() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    i = 0;
    while i < MAX_POKENAV_MENUITEMS {
        j = 0;
        while j < NUM_OPTION_SUBSPRITES {
            FreeSpriteOamMatrix((*gfx).iconSprites[i][j]);
            DestroySprite((*gfx).iconSprites[i][j]);
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn DrawCurrentMenuOptionLabels() {
    let mut menuType: i32 = GetPokenavMenuType();
    DrawOptionLabelGfx(
        sPokenavMenuOptionLabelGfx[menuType].gfx.as_ptr().cast_mut(),
        sPokenavMenuOptionLabelGfx[menuType].yStart as i32,
        sPokenavMenuOptionLabelGfx[menuType].deltaY as i32,
    );
}
pub(crate) unsafe extern "C" fn DrawOptionLabelGfx(
    mut optionGfx: *mut *mut u16,
    mut yPos: i32,
    deltaY: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut baseTile: i32 = GetSpriteTileStartByTag(GFXTAG_OPTIONS) as i32;
    i = 0;
    while i < MAX_POKENAV_MENUITEMS {
        if !(*optionGfx).is_null() {
            j = 0;
            while j < NUM_OPTION_SUBSPRITES {
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
                j += 1;
            }
            (*gfx).iconVisible[i] = TRUE as u32;
        } else {
            j = 0;
            while j < NUM_OPTION_SUBSPRITES {
                (*(*gfx).iconSprites[i][j]).set_invisible(TRUE as u16);
                j += 1;
            }
            (*gfx).iconVisible[i] = FALSE as u32;
        }
        optionGfx = optionGfx.at(1);
        yPos += deltaY;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartOptionAnimations_Enter() {
    let mut i: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut cursorPos: i32 = GetPokenavCursorPos();
    let mut iconCount: i32 = 0;
    let mut x: i32 = 0;
    i = 0;
    while i < MAX_POKENAV_MENUITEMS {
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
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartOptionAnimations_CursorMoved() {
    let mut i: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut prevPos: i32 = GetPokenavCursorPos();
    let mut newPos: i32 = 0;
    i = 0;
    newPos = 0;
    while i < MAX_POKENAV_MENUITEMS {
        if (*gfx).iconVisible[i] != 0 {
            if newPos == prevPos {
                newPos = i;
                break;
            }
            newPos += 1;
        }
        i += 1;
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
pub(crate) unsafe extern "C" fn StartOptionAnimations_Exit() {
    let mut i: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    i = 0;
    while i < MAX_POKENAV_MENUITEMS {
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
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AreMenuOptionSpritesMoving() -> u32 {
    let mut i: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    i = 0;
    while i < MAX_POKENAV_MENUITEMS {
        if (*(*gfx).iconSprites[i][0]).callback
            != Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
        {
            return TRUE as u32;
        }
        i += 1;
    }
    if (*gfx).numIconsBlending != 0 {
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn StartOptionSlide(
    mut sprites: *mut *mut Sprite,
    startX: i32,
    endX: i32,
    time: i32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_OPTION_SUBSPRITES {
        (*(*sprites)).x = startX as i16;
        (*(*sprites)).data[0] = time as i16;
        (*(*sprites)).data[1] = div_i32(16 * (endX - startX), time) as i16;
        (*(*sprites)).data[2] = 16 * startX as i16;
        (*(*sprites)).data[7] = endX as i16;
        (*(*sprites)).callback = Some(SpriteCB_OptionSlide);
        sprites = sprites.at(1);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn StartOptionZoom(mut sprites: *mut *mut Sprite) {
    let mut i: i32 = 0;
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut taskId: u8 = 0;
    i = 0;
    while i < NUM_OPTION_SUBSPRITES {
        (*(*sprites)).oam.set_objMode(ST_OAM_OBJ_BLEND);
        (*(*sprites)).oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
        (*(*sprites)).callback = Some(SpriteCB_OptionZoom);
        (*(*sprites)).data[0] = 8;
        (*(*sprites)).data[1] = FALSE as i16;
        (*(*sprites)).data[7] = i as i16;
        InitSpriteAffineAnim(*sprites);
        StartSpriteAffineAnim(*sprites, 0);
        sprites = sprites.at(1);
        i += 1;
    }
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    taskId = CreateTask(Some(Task_OptionBlend), 3);
    gTasks[taskId].data[0] = 8;
    (*gfx).numIconsBlending += 1;
}
pub(crate) unsafe extern "C" fn SetOptionInvisibility(
    mut sprites: *mut *mut Sprite,
    invisible: u32,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_OPTION_SUBSPRITES {
        (*(*sprites)).set_invisible(invisible as u16);
        sprites = sprites.at(1);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OptionSlide(sprite: *mut Sprite) {
    (*sprite).data[0] -= 1;
    if (*sprite).data[0] != -1 {
        (*sprite).data[2] += (*sprite).data[1];
        (*sprite).x = (*sprite).data[2] >> 4;
    } else {
        (*sprite).x = (*sprite).data[7];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OptionZoom(sprite: *mut Sprite) {
    let mut temp: i32 = 0;
    let mut x: i32 = 0;
    if (*sprite).data[0] == 0 {
        if (*sprite).data[1] == 0 {
            StartSpriteAffineAnim(sprite, 1);
            (*sprite).data[1] += 1;
            (*sprite).data[2] = 0x100;
            (*sprite).x += (*sprite).x2;
            (*sprite).x2 = 0;
        } else {
            (*sprite).data[2] += 16;
            temp = (*sprite).data[2] as i32;
            x = temp >> 3;
            x = (x - 32) / 2;
            match (*sprite).data[7] {
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
        (*sprite).data[0] -= 1;
    }
}
pub(crate) unsafe extern "C" fn Task_OptionBlend(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn CreateMatchCallBlueLightSprite() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut spriteId: u8 = CreateSprite(
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
pub(crate) unsafe extern "C" fn DestroyRematchBlueLightSprite() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    DestroySprite((*gfx).blueLightSprite);
}
pub(crate) unsafe extern "C" fn SpriteCB_BlinkingBlueLight(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if (*sprite).data[0] > 8 {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
}
pub(crate) unsafe extern "C" fn AddOptionDescriptionWindow() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).optionDescWindowId = AddWindow((&raw const *sOptionDescWindowTemplate).cast_mut());
    PutWindowTilemap((*gfx).optionDescWindowId as u8);
    FillWindowPixelBuffer((*gfx).optionDescWindowId as u8, 102);
    CopyWindowToVram((*gfx).optionDescWindowId as u8, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn PrintCurrentOptionDescription() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut menuItem: i32 = GetCurrentMenuItemId();
    let mut desc: *mut u8 = sPageDescriptions[menuItem];
    let mut width: u32 = GetStringWidth(FONT_NORMAL, desc, -1) as u32;
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
pub(crate) unsafe extern "C" fn PrintNoRibbonWinners() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    let mut s: *mut u8 = gText_NoRibbonWinners.as_ptr().cast_mut();
    let mut width: u32 = GetStringWidth(FONT_NORMAL, s, -1) as u32;
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
pub(crate) unsafe extern "C" fn IsDma3ManagerBusyWithBgCopy_() -> u32 {
    return IsDma3ManagerBusyWithBgCopy() as u32;
}
pub(crate) unsafe extern "C" fn CreateMovingBgDotsTask() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    (*gfx).bg3ScrollTaskId = CreateTask(Some(Task_MoveBgDots), 2);
}
pub(crate) unsafe extern "C" fn DestroyMovingDotsBgTask() {
    let mut gfx: *mut Pokenav_MenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MENU_GFX) as *mut Pokenav_MenuGfx;
    DestroyTask((*gfx).bg3ScrollTaskId);
}
pub(crate) unsafe extern "C" fn Task_MoveBgDots(taskId: u8) {
    ChangeBgX(3, 0x80, BG_COORD_ADD);
}
pub(crate) unsafe extern "C" fn CreateBgDotPurplePalTask() {
    let mut taskId: u8 = CreateTask(Some(Task_UpdateBgDotsPalette), 3);
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
pub(crate) unsafe extern "C" fn ChangeBgDotsColorToPurple() {
    CopyPaletteIntoBufferUnfaded(sPokenavBgDotsPal.as_ptr().cast_mut().at(7), 49, 4);
}
pub(crate) unsafe extern "C" fn CreateBgDotLightBluePalTask() {
    let mut taskId: u8 = CreateTask(Some(Task_UpdateBgDotsPalette), 3);
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
pub(crate) unsafe extern "C" fn IsTaskActive_UpdateBgDotsPalette() -> u32 {
    return FuncIsActiveTask(Some(Task_UpdateBgDotsPalette)) as u32;
}
pub(crate) unsafe extern "C" fn Task_UpdateBgDotsPalette(taskId: u8) {
    let mut sp8: CArray<u16, 2> = zeroed();
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut pal1: *mut u16 = GetWordTaskArg(taskId, 1) as usize as *mut u16;
    let mut pal2: *mut u16 = GetWordTaskArg(taskId, 3) as usize as *mut u16;
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
pub(crate) unsafe extern "C" fn VBlankCB_PokenavMainMenu() {
    TransferPlttBuffer();
    LoadOam();
    ProcessSpriteCopyRequests();
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn SetupPokenavMenuScanlineEffects() {
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
pub(crate) unsafe extern "C" fn DestroyMenuOptionGlowTask() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN0_ON);
    ScanlineEffect_Stop();
    DestroyTask(FindTaskIdByFunc(Some(Task_CurrentMenuOptionGlow)));
    SetPokenavVBlankCallback();
}
pub(crate) unsafe extern "C" fn ResetBldCnt() {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
}
pub(crate) unsafe extern "C" fn InitMenuOptionGlow() {
    SetMenuOptionGlow();
    SetGpuReg(REG_OFFSET_BLDCNT, 144);
}
pub(crate) unsafe extern "C" fn Task_CurrentMenuOptionGlow(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data += 1;
    if *data > 0 {
        *data = 0;
        *data.at(1) += 3;
        *data.at(1) &= 0x7F;
        SetGpuReg(REG_OFFSET_BLDY, (gSineTable[*data.at(1)] >> 5) as u16);
    }
}
pub(crate) unsafe extern "C" fn SetMenuOptionGlow() {
    let mut menuType: i32 = GetPokenavMenuType();
    let mut cursorPos: i32 = GetPokenavCursorPos();
    let mut r4: i32 = sPokenavMenuOptionLabelGfx[menuType].deltaY as i32 * cursorPos
        + sPokenavMenuOptionLabelGfx[menuType].yStart as i32
        - 8;
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            CpuSet(
                &raw mut tmp as *mut c_void,
                gScanlineEffectRegBuffers[0].as_mut_ptr() as *mut c_void,
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
                gScanlineEffectRegBuffers[1].as_mut_ptr() as *mut c_void,
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
                &raw mut gScanlineEffectRegBuffers[0][r4] as *mut c_void,
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
                &raw mut gScanlineEffectRegBuffers[1][r4] as *mut c_void,
                0x1000010,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBldCnt_() {
    ResetBldCnt();
}
