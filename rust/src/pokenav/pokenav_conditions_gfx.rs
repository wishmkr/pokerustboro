//! Translated from `src/pokenav_conditions_gfx.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_labels
)]

use crate::bg::CopyToBgTilemapBufferRect;
use crate::bg::{ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, HideBg, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::menu::{BgDmaFill, DecompressAndCopyTileDataToVram, FreeTempTileDataBuffersIfPossible};
use crate::menu_specialized::{
    ConditionGraph_Draw, ConditionGraph_InitResetScanline, ConditionGraph_InitWindow,
    ConditionGraph_ResetScanline, ConditionGraph_SetNewPositions, ConditionGraph_TryUpdate,
    ConditionMenu_UpdateMonEnter, ConditionMenu_UpdateMonExit, CreateConditionSparkleSprites,
    DestroyConditionSparkleSprites, FreeConditionSparkles, LoadConditionMonPicTemplate,
    LoadConditionSelectionIcons, LoadConditionSparkle, MoveConditionMonOffscreen,
    ResetConditionSparkleSprites,
};
use crate::mon_markings::{
    BufferMonMarkingsMenuTiles, CreateMonMarkingAllCombosSprite, FreeMonMarkingsMenu,
    InitMonMarkingsMenu, OpenMonMarkingsMenu,
};
use crate::palette::{LoadPalette, TransferPlttBuffer};
use crate::pokenav::CreateLoopedTask;
use crate::pokenav::{
    AllocSubstruct, FreePokenavSubstruct, GetSubstructPtr, IsLoopedTaskActive,
    SetPokenavVBlankCallback, SetVBlankCallback_,
};
use crate::pokenav_conditions::{
    GetConditionGraphCurrentListIndex, GetConditionGraphPtr, GetConditionMonDataBuffer,
    GetConditionMonLocationText, GetConditionMonNameText, GetConditionMonPal,
    GetConditionMonPicGfx, GetMonListCount, GetNumConditionMonSparkles, IsConditionMenuSearchMode,
    LoadConditionGraphMenuGfx, LoadNextConditionMenuMonData, TryGetMonMarkId,
};
use crate::pokenav_main_menu::{
    AreLeftHeaderSpritesMoving, CopyPaletteIntoBufferUnfaded, InitBgTemplates, IsPaletteFadeActive,
    LoadLeftHeaderGfxForIndex, MainMenuLoopedTaskIsBusy, Pokenav_AllocAndLoadPalettes,
    PokenavFadeScreen, PokenavFillPalette, PrintHelpBarText, SetLeftHeaderSpritesInvisibility,
    ShowLeftHeaderGfx, SlideMenuHeaderDown, WaitForHelpBar,
};
use crate::scanline_effect::ScanlineEffect_InitHBlankDmaTransfer;
use crate::sprite::gSprites;
use crate::sprite::{
    FreeSpritePaletteByTag, FreeSpriteTilesByTag, IndexOfSpritePaletteTag, LoadOam,
    ProcessSpriteCopyRequests,
};
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy};
use crate::text::DeactivateAllTextPrinters;
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
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `LoadSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16 {
    unsafe { crate::sprite::LoadSpriteSheet(a0 as _) }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
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
// Data tables (translate with cdata.py): gConditionGraphData_Pal gConditionText_Pal sConditionGraphData_Gfx sConditionGraphData_Tilemap sMonMarkings_Pal sMenuBgTemplates sMonNameGenderWindowTemplate sListIndexWindowTemplate sUnusedWindowTemplate1 sUnusedWindowTemplate2 sLoopedTaskFuncs

/// `struct Pokenav_ConditionMenuGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_ConditionMenuGfx {
    pub loopedTaskId: u32,
    pub tilemapBuffers: CArray<CArray<u8, 2048>, 3>,
    pub filler: CArray<u8, 2>,
    pub partyPokeballSpriteIds: CArray<u8, 7>,
    pub callback: Option<unsafe fn() -> u32>,
    pub monTransitionX: i16,
    pub monPicSpriteId: u8,
    pub monPalIndex: u16,
    pub monGfxTileStart: u16,
    pub monGfxPtr: *mut core::ffi::c_void,
    pub nameGenderWindowId: u8,
    pub listIndexWindowId: u8,
    pub unusedWindowId1: u8,
    pub unusedWindowId2: u8,
    pub marksMenu: MonMarkingsMenu,
    pub monMarksSprite: *mut Sprite,
    pub conditionSparkleSprites: CArray<*mut Sprite, 10>,
    pub windowModeState: u8,
    pub filler2: CArray<u8, 4003>,
}

unsafe impl Sync for Pokenav_ConditionMenuGfx {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_ConditionMenuGfx>() == 14508);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, loopedTaskId) == 0);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, tilemapBuffers) == 4);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, filler) == 6148);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, partyPokeballSpriteIds) == 6150);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, callback) == 6160);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, monTransitionX) == 6164);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, monPicSpriteId) == 6166);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, monPalIndex) == 6168);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, monGfxTileStart) == 6170);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, monGfxPtr) == 6172);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, nameGenderWindowId) == 6176);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, listIndexWindowId) == 6177);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, unusedWindowId1) == 6178);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, unusedWindowId2) == 6179);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, marksMenu) == 6180);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, monMarksSprite) == 10460);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, conditionSparkleSprites) == 10464);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, windowModeState) == 10504);
    assert!(offset_of!(Pokenav_ConditionMenuGfx, filler2) == 10505);
};

static gConditionGraphData_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::gConditionGraphData_Pal).cast());
static gConditionText_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::gConditionText_Pal).cast());
static sConditionGraphData_Gfx: Table<CArray<u32, 5>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sConditionGraphData_Gfx).cast());
static sConditionGraphData_Tilemap: Table<CArray<u32, 63>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sConditionGraphData_Tilemap).cast());
static sListIndexWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sListIndexWindowTemplate).cast());
static sLoopedTaskFuncs: Table<CArray<Option<unsafe fn(i32) -> u32>, 7>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sLoopedTaskFuncs).cast());
static sMenuBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sMenuBgTemplates).cast());
static sMonMarkings_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sMonMarkings_Pal).cast());
static sMonNameGenderWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sMonNameGenderWindowTemplate).cast());
static sUnusedWindowTemplate1: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sUnusedWindowTemplate1).cast());
static sUnusedWindowTemplate2: Table<WindowTemplate> =
    Table((&raw const crate::data::pokenav_conditions_gfx::sUnusedWindowTemplate2).cast());

pub(crate) static sInitialLoadId: crate::global::Global<u8> = crate::global::Global::new(0);

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
/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `GetConditionGraphMenuCurrentLoadIndex` with this module's view of its types.
#[inline]
unsafe fn GetConditionGraphMenuCurrentLoadIndex() -> i8 {
    unsafe { crate::pokenav_conditions::GetConditionGraphMenuCurrentLoadIndex() as i8 }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}

pub unsafe fn OpenConditionGraphMenu() -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX, 14508)
            as *mut Pokenav_ConditionMenuGfx;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).monPicSpriteId = SPRITE_NONE;
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenConditionGraphMenu), 1);
    (*menu).callback = Some(GetConditionGraphMenuLoopedTaskActive);
    (*menu).windowModeState = 0;
    TRUE as u32
}
pub unsafe fn CreateConditionGraphMenuLoopedTask(id: i32) {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    (*menu).loopedTaskId = CreateLoopedTask(sLoopedTaskFuncs[id], 1);
    (*menu).callback = Some(GetConditionGraphMenuLoopedTaskActive);
}
pub unsafe fn IsConditionGraphMenuLoopedTaskActive() -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    (*menu).callback.unwrap_unchecked()()
}
pub(crate) unsafe fn GetConditionGraphMenuLoopedTaskActive() -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    IsLoopedTaskActive((*menu).loopedTaskId)
}
pub(crate) unsafe fn LoopedTask_OpenConditionGraphMenu(state: i32) -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    match state {
        0 => {
            if LoadConditionGraphMenuGfx() != TRUE as u32 {
                return LT_PAUSE;
            }
            return LT_INC_AND_PAUSE;
        }
        1 => {
            InitBgTemplates(sMenuBgTemplates.as_ptr().cast_mut(), 3);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            SetGpuReg(REG_OFFSET_DISPCNT, 31040);
            SetGpuReg(REG_OFFSET_BLDCNT, 2116);
            SetGpuReg(REG_OFFSET_BLDALPHA, 1035);
            DecompressAndCopyTileDataToVram(
                3,
                (*(&raw const crate::data::graphics::gPokenavCondition_Gfx)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            DecompressAndCopyTileDataToVram(
                2,
                sConditionGraphData_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            LZ77UnCompVram(
                (*(&raw const crate::data::graphics::gPokenavCondition_Tilemap)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                (*menu).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(3, (*menu).tilemapBuffers[0].as_mut_ptr() as *mut c_void);
            if IsConditionMenuSearchMode() == TRUE as u32 {
                CopyToBgTilemapBufferRect(
                    3,
                    (*(&raw const crate::data::graphics::gPokenavOptions_Tilemap)
                        .cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                    0,
                    5,
                    9,
                    4,
                );
            }
            CopyBgTilemapBufferToVram(3);
            CopyPaletteIntoBufferUnfaded(
                (*(&raw const crate::data::graphics::gPokenavCondition_Pal)
                    .cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut(),
                16,
                32,
            );
            CopyPaletteIntoBufferUnfaded(gConditionText_Pal.as_ptr().cast_mut(), 240, 32);
            (*menu).monTransitionX = -80;
            return LT_INC_AND_PAUSE;
        }
        4 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            LZ77UnCompVram(
                sConditionGraphData_Tilemap.as_ptr().cast_mut(),
                (*menu).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(2, (*menu).tilemapBuffers[2].as_mut_ptr() as *mut c_void);
            CopyBgTilemapBufferToVram(2);
            CopyPaletteIntoBufferUnfaded(gConditionGraphData_Pal.as_ptr().cast_mut(), 48, 32);
            ConditionGraph_InitWindow(2);
            return LT_INC_AND_PAUSE;
        }
        5 => {
            BgDmaFill(1, 0, 0, 1);
            BgDmaFill(1, 17, 1, 1);
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    CpuSet(
                        &raw mut tmp as *mut c_void,
                        (*menu).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
                        0x5000200,
                    );
                }
            }
            SetBgTilemapBuffer(1, (*menu).tilemapBuffers[1].as_mut_ptr() as *mut c_void);
            return LT_INC_AND_PAUSE;
        }
        6 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            (*menu).nameGenderWindowId =
                AddWindow((&raw const *sMonNameGenderWindowTemplate).cast_mut()) as u8;
            if IsConditionMenuSearchMode() == TRUE as u32 {
                (*menu).listIndexWindowId =
                    AddWindow((&raw const *sListIndexWindowTemplate).cast_mut()) as u8;
                (*menu).unusedWindowId1 =
                    AddWindow((&raw const *sUnusedWindowTemplate1).cast_mut()) as u8;
                (*menu).unusedWindowId2 =
                    AddWindow((&raw const *sUnusedWindowTemplate2).cast_mut()) as u8;
            }
            DeactivateAllTextPrinters();
            return LT_INC_AND_PAUSE;
        }
        7 => {
            CreateConditionMonPic(0);
            return LT_INC_AND_PAUSE;
        }
        8 => {
            CreateMonMarkingsOrPokeballIndicators();
            return LT_INC_AND_PAUSE;
        }
        9 => {
            if IsConditionMenuSearchMode() == TRUE as u32 {
                CopyUnusedConditionWindowsToVram();
            }
            return LT_INC_AND_PAUSE;
        }
        10 => {
            UpdateConditionGraphMenuWindows(
                0,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                TRUE,
            );
            return LT_INC_AND_PAUSE;
        }
        11 => {
            UpdateConditionGraphMenuWindows(1, GetConditionGraphMenuCurrentLoadIndex() as u16, 1);
            return LT_INC_AND_PAUSE;
        }
        12 => {
            UpdateConditionGraphMenuWindows(
                2,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                TRUE,
            );
            return LT_INC_AND_PAUSE;
        }
        13 => {
            if UpdateConditionGraphMenuWindows(
                3,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                TRUE,
            ) != TRUE as u32
            {
                return LT_PAUSE;
            }
            PutWindowTilemap((*menu).nameGenderWindowId);
            if IsConditionMenuSearchMode() == TRUE as u32 {
                PutWindowTilemap((*menu).listIndexWindowId);
                PutWindowTilemap((*menu).unusedWindowId1);
                PutWindowTilemap((*menu).unusedWindowId2);
            }
            return LT_INC_AND_PAUSE;
        }
        14 => {
            ShowBg(1);
            HideBg(2);
            ShowBg(3);
            if IsConditionMenuSearchMode() == TRUE as u32 {
                PrintHelpBarText(HELPBAR_CONDITION_MON_STATUS);
            }
            return LT_INC_AND_PAUSE;
        }
        15 => {
            PokenavFadeScreen(POKENAV_FADE_FROM_BLACK);
            if IsConditionMenuSearchMode() == 0 {
                LoadLeftHeaderGfxForIndex(POKENAV_GFX_PARTY_MENU);
                ShowLeftHeaderGfx(POKENAV_GFX_CONDITION_MENU, TRUE as u32, FALSE as u32);
                ShowLeftHeaderGfx(POKENAV_GFX_PARTY_MENU, TRUE as u32, FALSE as u32);
            }
            return LT_INC_AND_PAUSE;
        }
        16 => {
            if IsPaletteFadeActive() != 0 {
                return LT_PAUSE;
            }
            if IsConditionMenuSearchMode() == 0 && AreLeftHeaderSpritesMoving() != 0 {
                return LT_PAUSE;
            }
            SetVBlankCallback_(Some(VBlankCB_PokenavConditionGraph));
            return LT_INC_AND_PAUSE;
        }
        17 => {
            DoConditionGraphEnterTransition();
            ConditionGraph_InitResetScanline(GetConditionGraphPtr());
            return LT_INC_AND_PAUSE;
        }
        18 => {
            if ConditionGraph_ResetScanline(GetConditionGraphPtr()) != 0 {
                return LT_PAUSE;
            }
            return LT_INC_AND_PAUSE;
        }
        19 => {
            ToggleGraphData(TRUE);
            return LT_INC_AND_PAUSE;
        }
        20 => {
            if ConditionMenu_UpdateMonEnter(GetConditionGraphPtr(), &raw mut (*menu).monTransitionX)
                == 0
            {
                ResetConditionSparkleSprites((*menu).conditionSparkleSprites.as_mut_ptr());
                if IsConditionMenuSearchMode() == TRUE as u32
                    || GetConditionGraphCurrentListIndex() != GetMonListCount()
                {
                    CreateConditionSparkleSprites(
                        (*menu).conditionSparkleSprites.as_mut_ptr(),
                        (*menu).monPicSpriteId,
                        GetNumConditionMonSparkles(),
                    );
                }
                return LT_FINISH;
            }
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_ExitConditionGraphMenu(state: i32) -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    match state {
        0 => {
            DoConditionGraphExitTransition();
            DestroyConditionSparkleSprites((*menu).conditionSparkleSprites.as_mut_ptr());
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            if ConditionMenu_UpdateMonExit(GetConditionGraphPtr(), &raw mut (*menu).monTransitionX)
                != 0
            {
                return 2;
            }
            ToggleGraphData(FALSE);
            return LT_INC_AND_CONTINUE;
        }
        2 => {
            PokenavFadeScreen(POKENAV_FADE_TO_BLACK);
            if IsConditionMenuSearchMode() == 0 {
                SlideMenuHeaderDown();
            }
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsPaletteFadeActive() != 0 || MainMenuLoopedTaskIsBusy() != 0 {
                return LT_PAUSE;
            }
            FreeConditionSparkles((*menu).conditionSparkleSprites.as_mut_ptr());
            HideBg(1);
            HideBg(2);
            HideBg(3);
            return LT_INC_AND_CONTINUE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_TransitionMons(state: i32) -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    let mut graph: *mut ConditionGraph = GetConditionGraphPtr();
    match state {
        0 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_MON_INFO);
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_GRAPH);
            return LT_INC_AND_CONTINUE;
        }
        2 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_MON_PIC);
            DestroyConditionSparkleSprites((*menu).conditionSparkleSprites.as_mut_ptr());
            return LT_INC_AND_CONTINUE;
        }
        3 => {
            ConditionGraph_TryUpdate(graph);
            return LT_INC_AND_CONTINUE;
        }
        4 => {
            if MoveConditionMonOffscreen(&raw mut (*menu).monTransitionX) == 0 {
                CreateConditionMonPic(GetConditionGraphMenuCurrentLoadIndex() as u8);
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        5 => {
            UpdateConditionGraphMenuWindows(0, GetConditionGraphMenuCurrentLoadIndex() as u16, 0);
            return LT_INC_AND_CONTINUE;
        }
        6 => {
            UpdateConditionGraphMenuWindows(
                1,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            );
            return LT_INC_AND_CONTINUE;
        }
        7 => {
            UpdateConditionGraphMenuWindows(
                2,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            );
            return LT_INC_AND_CONTINUE;
        }
        8 => {
            if UpdateConditionGraphMenuWindows(
                3,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            ) == TRUE as u32
            {
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        9 => {
            graph = GetConditionGraphPtr();
            if ConditionMenu_UpdateMonEnter(graph, &raw mut (*menu).monTransitionX) == 0 {
                ResetConditionSparkleSprites((*menu).conditionSparkleSprites.as_mut_ptr());
                if IsConditionMenuSearchMode() != TRUE as u32
                    && GetConditionGraphCurrentListIndex() == GetMonListCount()
                {
                    return LT_INC_AND_CONTINUE;
                }
                CreateConditionSparkleSprites(
                    (*menu).conditionSparkleSprites.as_mut_ptr(),
                    (*menu).monPicSpriteId,
                    GetNumConditionMonSparkles(),
                );
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_MoveCursorNoTransition(state: i32) -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    match state {
        0 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_MON_INFO);
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_GRAPH);
            return LT_INC_AND_CONTINUE;
        }
        2 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_MON_PIC);
            return LT_INC_AND_CONTINUE;
        }
        3 => {
            CreateConditionMonPic(GetConditionGraphMenuCurrentLoadIndex() as u8);
            return LT_INC_AND_CONTINUE;
        }
        4 => {
            UpdateConditionGraphMenuWindows(0, GetConditionGraphMenuCurrentLoadIndex() as u16, 0);
            return LT_INC_AND_CONTINUE;
        }
        5 => {
            UpdateConditionGraphMenuWindows(
                1,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            );
            return LT_INC_AND_CONTINUE;
        }
        6 => {
            UpdateConditionGraphMenuWindows(
                2,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            );
            return LT_INC_AND_CONTINUE;
        }
        7 => {
            if UpdateConditionGraphMenuWindows(
                3,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            ) == TRUE as u32
            {
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        8 => {
            if ConditionMenu_UpdateMonEnter(GetConditionGraphPtr(), &raw mut (*menu).monTransitionX)
                == 0
            {
                ResetConditionSparkleSprites((*menu).conditionSparkleSprites.as_mut_ptr());
                CreateConditionSparkleSprites(
                    (*menu).conditionSparkleSprites.as_mut_ptr(),
                    (*menu).monPicSpriteId,
                    GetNumConditionMonSparkles(),
                );
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_SlideMonOut(state: i32) -> u32 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    match state {
        0 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_MON_INFO);
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_GRAPH);
            return LT_INC_AND_CONTINUE;
        }
        2 => {
            LoadNextConditionMenuMonData(CONDITION_LOAD_MON_PIC);
            DestroyConditionSparkleSprites((*menu).conditionSparkleSprites.as_mut_ptr());
            return LT_INC_AND_CONTINUE;
        }
        3 => {
            if ConditionMenu_UpdateMonExit(GetConditionGraphPtr(), &raw mut (*menu).monTransitionX)
                == 0
            {
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        4 => {
            UpdateConditionGraphMenuWindows(0, GetConditionGraphMenuCurrentLoadIndex() as u16, 0);
            return LT_INC_AND_CONTINUE;
        }
        5 => {
            UpdateConditionGraphMenuWindows(
                1,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            );
            return LT_INC_AND_CONTINUE;
        }
        6 => {
            UpdateConditionGraphMenuWindows(
                2,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            );
            return LT_INC_AND_CONTINUE;
        }
        7 => {
            if UpdateConditionGraphMenuWindows(
                3,
                GetConditionGraphMenuCurrentLoadIndex() as u16,
                FALSE,
            ) == TRUE as u32
            {
                return LT_INC_AND_CONTINUE;
            }
            return LT_PAUSE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_OpenMonMarkingsWindow(state: i32) -> u32 {
    match state {
        0 => {
            OpenMonMarkingsMenu(TryGetMonMarkId(), 176, 32);
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            PrintHelpBarText(HELPBAR_CONDITION_MARKINGS);
            return LT_INC_AND_CONTINUE;
        }
        2 => {
            if WaitForHelpBar() == TRUE as u32 {
                return LT_PAUSE;
            }
            return LT_INC_AND_CONTINUE;
        }
        _ => {}
    }
    LT_FINISH
}
pub(crate) unsafe fn LoopedTask_CloseMonMarkingsWindow(state: i32) -> u32 {
    match state {
        0 => {
            FreeMonMarkingsMenu();
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            PrintHelpBarText(HELPBAR_CONDITION_MON_STATUS);
            return LT_INC_AND_CONTINUE;
        }
        2 => {
            if WaitForHelpBar() == TRUE as u32 {
                return LT_PAUSE;
            }
            return LT_INC_AND_CONTINUE;
        }
        _ => {}
    }
    LT_FINISH
}
unsafe fn UnusedPrintNumberString(dst: *mut u8, num: u16) -> *mut u8 {
    let mut txtPtr: *mut u8 =
        ConvertIntToDecimalStringN(dst, num as i32, STR_CONV_MODE_RIGHT_ALIGN, 4);
    txtPtr = StringCopy(
        txtPtr,
        (*(&raw const crate::data::strings::gText_Number2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    txtPtr
}
unsafe fn UpdateConditionGraphMenuWindows(mode: u8, bufferIndex: u16, winMode: u8) -> u32 {
    let mut text: CArray<u8, 32> = zeroed();
    let mut str: *mut u8 = null_mut();
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    match mode {
        0 => {
            FillWindowPixelBuffer((*menu).nameGenderWindowId, 0);
            if IsConditionMenuSearchMode() == TRUE as u32 {
                FillWindowPixelBuffer((*menu).listIndexWindowId, 0);
            }
        }
        1 => {
            if GetConditionGraphCurrentListIndex() as i32 != GetMonListCount() as i32 - 1
                || IsConditionMenuSearchMode() == 1
            {
                str = GetConditionMonNameText(bufferIndex as u8);
                AddTextPrinterParameterized(
                    (*menu).nameGenderWindowId,
                    FONT_NORMAL,
                    str,
                    0,
                    1,
                    0,
                    None,
                );
            }
        }
        2 => {
            if IsConditionMenuSearchMode() == TRUE as u32 {
                str = GetConditionMonLocationText(bufferIndex as u8);
                AddTextPrinterParameterized(
                    (*menu).nameGenderWindowId,
                    FONT_NORMAL,
                    str,
                    0,
                    17,
                    0,
                    None,
                );
                text[0] = EXT_CTRL_CODE_BEGIN;
                text[1] = EXT_CTRL_CODE_COLOR_HIGHLIGHT_SHADOW;
                text[2] = TEXT_COLOR_BLUE;
                text[3] = TEXT_COLOR_TRANSPARENT;
                text[4] = TEXT_COLOR_LIGHT_BLUE;
                StringCopy(
                    &raw mut text[5],
                    (*(&raw const crate::data::strings::gText_Number2).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
                AddTextPrinterParameterized(
                    (*menu).listIndexWindowId,
                    FONT_NORMAL,
                    text.as_mut_ptr(),
                    4,
                    1,
                    0,
                    None,
                );
                ConvertIntToDecimalStringN(
                    &raw mut text[5],
                    GetConditionMonDataBuffer() as i32,
                    STR_CONV_MODE_RIGHT_ALIGN,
                    4,
                );
                AddTextPrinterParameterized(
                    (*menu).listIndexWindowId,
                    FONT_NORMAL,
                    text.as_mut_ptr(),
                    28,
                    1,
                    0,
                    None,
                );
            }
        }
        3 => 'l2: {
            let sw1: u8 = (*menu).windowModeState;
            let fall = false;
            if sw1 == 0 {
                if winMode != 0 {
                    CopyWindowToVram((*menu).nameGenderWindowId, COPYWIN_FULL);
                } else {
                    CopyWindowToVram((*menu).nameGenderWindowId, COPYWIN_GFX);
                }
                if IsConditionMenuSearchMode() == TRUE as u32 {
                    (*menu).windowModeState += 1;
                    return FALSE as u32;
                } else {
                    (*menu).windowModeState = 0;
                    return TRUE as u32;
                }
            }
            if fall || sw1 == 1 {
                if winMode != 0 {
                    CopyWindowToVram((*menu).listIndexWindowId, COPYWIN_FULL);
                } else {
                    CopyWindowToVram((*menu).listIndexWindowId, COPYWIN_GFX);
                }
                (*menu).windowModeState = 0;
                return TRUE as u32;
            }
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn CopyUnusedConditionWindowsToVram() {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    CopyWindowToVram((*menu).unusedWindowId1, COPYWIN_FULL);
    CopyWindowToVram((*menu).unusedWindowId2, COPYWIN_FULL);
}
pub(crate) unsafe fn SpriteCB_PartyPokeball(sprite: *mut Sprite) {
    if (*sprite).data[0] as i32 == GetConditionGraphCurrentListIndex() as i32 {
        StartSpriteAnim(sprite, CONDITION_ICON_SELECTED);
    } else {
        StartSpriteAnim(sprite, CONDITION_ICON_UNSELECTED);
    }
}
pub unsafe fn HighlightCurrentPartyIndexPokeball(sprite: *mut Sprite) {
    if GetConditionGraphCurrentListIndex() as i32 == GetMonListCount() as i32 - 1 {
        (*sprite)
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(TAG_CONDITION_BALL) as u16);
    } else {
        (*sprite)
            .oam
            .set_paletteNum(IndexOfSpritePaletteTag(TAG_CONDITION_CANCEL) as u16);
    }
}
pub unsafe fn MonMarkingsCallback(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, TryGetMonMarkId());
}
unsafe fn CreateMonMarkingsOrPokeballIndicators() {
    let mut sprSheets: CArray<SpriteSheet, 4> = zeroed();
    let mut sprTemplate: SpriteTemplate = zeroed();
    let mut sprPals: CArray<SpritePalette, 3> = zeroed();
    let mut sprSheet: SpriteSheet = zeroed();
    let mut sprite: *mut Sprite = null_mut();
    let mut i: u16 = 0;
    let mut spriteId: u16 = 0;
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    LoadConditionSelectionIcons(
        sprSheets.as_mut_ptr(),
        &raw mut sprTemplate,
        sprPals.as_mut_ptr(),
    );
    if IsConditionMenuSearchMode() == TRUE as u32 {
        (*menu).marksMenu.baseTileTag = TAG_CONDITION_MARKINGS_MENU;
        (*menu).marksMenu.basePaletteTag = TAG_CONDITION_MARKINGS_MENU;
        InitMonMarkingsMenu(&raw mut (*menu).marksMenu);
        BufferMonMarkingsMenuTiles();
        sprite = CreateMonMarkingAllCombosSprite(
            TAG_CONDITION_MON_MARKINGS,
            TAG_CONDITION_MON_MARKINGS,
            sMonMarkings_Pal.as_ptr().cast_mut(),
        );
        (*sprite).oam.set_priority(3);
        (*sprite).x = 192;
        (*sprite).y = 32;
        (*sprite).callback = Some(MonMarkingsCallback);
        (*menu).monMarksSprite = sprite;
        PokenavFillPalette(
            IndexOfSpritePaletteTag(TAG_CONDITION_MON_MARKINGS) as u32,
            0,
        );
    } else {
        LoadSpriteSheets(sprSheets.as_mut_ptr());
        Pokenav_AllocAndLoadPalettes(sprPals.as_mut_ptr());
        i = 0;
        while (i as i32) < GetMonListCount() as i32 - 1 {
            spriteId = CreateSprite(&raw mut sprTemplate, 226, i as i16 * 20 + 8, 0) as u16;
            if spriteId != MAX_SPRITES as u16 {
                (*menu).partyPokeballSpriteIds[i] = spriteId as u8;
                gSprites[spriteId].data[0] = i as i16;
                gSprites[spriteId].callback = Some(SpriteCB_PartyPokeball);
            } else {
                (*menu).partyPokeballSpriteIds[i] = SPRITE_NONE;
            }
            i += 1;
        }
        sprTemplate.tileTag = TAG_CONDITION_BALL_PLACEHOLDER;
        sprTemplate.callback = Some(SpriteCallbackDummy);
        while i < PARTY_SIZE as u16 {
            spriteId = CreateSprite(&raw mut sprTemplate, 230, i as i16 * 20 + 8, 0) as u16;
            if spriteId != MAX_SPRITES as u16 {
                (*menu).partyPokeballSpriteIds[i] = spriteId as u8;
                gSprites[spriteId].oam.set_size(0);
            } else {
                (*menu).partyPokeballSpriteIds[i] = SPRITE_NONE;
            }
            i += 1;
        }
        sprTemplate.tileTag = TAG_CONDITION_CANCEL;
        sprTemplate.callback = Some(HighlightCurrentPartyIndexPokeball);
        spriteId = CreateSprite(&raw mut sprTemplate, 222, i as i16 * 20 + 8, 0) as u16;
        if spriteId != MAX_SPRITES as u16 {
            (*menu).partyPokeballSpriteIds[i] = spriteId as u8;
            gSprites[spriteId].oam.set_shape(1);
            gSprites[spriteId].oam.set_size(2);
        } else {
            (*menu).partyPokeballSpriteIds[i] = SPRITE_NONE;
        }
    }
    LoadConditionSparkle(&raw mut sprSheet, &raw mut sprPals[0]);
    LoadSpriteSheet(&raw mut sprSheet);
    sprPals[1].data = null_mut();
    Pokenav_AllocAndLoadPalettes(sprPals.as_mut_ptr());
}
unsafe fn FreeConditionMenuGfx(menu: *mut Pokenav_ConditionMenuGfx) {
    if IsConditionMenuSearchMode() == TRUE as u32 {
        DestroySprite((*menu).monMarksSprite);
        FreeSpriteTilesByTag(TAG_CONDITION_MARKINGS_MENU);
        FreeSpriteTilesByTag(TAG_CONDITION_MON_MARKINGS);
        FreeSpritePaletteByTag(TAG_CONDITION_MARKINGS_MENU);
        FreeSpritePaletteByTag(TAG_CONDITION_MON_MARKINGS);
    } else {
        for i in 0..7u8 {
            DestroySprite(&raw mut gSprites[(*menu).partyPokeballSpriteIds[i]]);
        }
        FreeSpriteTilesByTag(TAG_CONDITION_BALL);
        FreeSpriteTilesByTag(TAG_CONDITION_CANCEL);
        FreeSpriteTilesByTag(TAG_CONDITION_BALL_PLACEHOLDER);
        FreeSpritePaletteByTag(TAG_CONDITION_BALL);
        FreeSpritePaletteByTag(TAG_CONDITION_CANCEL);
    }
    if (*menu).monPicSpriteId != SPRITE_NONE {
        DestroySprite(&raw mut gSprites[(*menu).monPicSpriteId]);
        FreeSpriteTilesByTag(TAG_CONDITION_MON);
        FreeSpritePaletteByTag(TAG_CONDITION_MON);
    }
}
pub unsafe fn FreeConditionGraphMenuSubstruct2() {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    RemoveWindow((*menu).nameGenderWindowId);
    if IsConditionMenuSearchMode() == TRUE as u32 {
        RemoveWindow((*menu).listIndexWindowId);
        RemoveWindow((*menu).unusedWindowId1);
        RemoveWindow((*menu).unusedWindowId2);
    } else {
        SetLeftHeaderSpritesInvisibility();
    }
    SetGpuReg(REG_OFFSET_DISPCNT, 4416);
    FreeConditionMenuGfx(menu);
    SetExitVBlank();
    FreePokenavSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX);
}
pub unsafe fn MonPicGfxSpriteCallback(sprite: *mut Sprite) {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    (*sprite).x = (*menu).monTransitionX + 38;
}
unsafe fn CreateConditionMonPic(id: u8) {
    let mut sprTemplate: SpriteTemplate = zeroed();
    let mut sprSheet: SpriteSheet = zeroed();
    let mut sprPal: SpritePalette = zeroed();
    let mut spriteId: u8 = 0;
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    if (*menu).monPicSpriteId == SPRITE_NONE {
        LoadConditionMonPicTemplate(&raw mut sprSheet, &raw mut sprTemplate, &raw mut sprPal);
        sprSheet.data = GetConditionMonPicGfx(id);
        sprPal.data = GetConditionMonPal(id) as *mut u16;
        (*menu).monPalIndex = LoadSpritePalette(&raw mut sprPal) as u16;
        (*menu).monGfxTileStart = LoadSpriteSheet(&raw mut sprSheet);
        spriteId = CreateSprite(&raw mut sprTemplate, 38, 104, 0);
        (*menu).monPicSpriteId = spriteId;
        if spriteId == MAX_SPRITES {
            FreeSpriteTilesByTag(TAG_CONDITION_MON);
            FreeSpritePaletteByTag(TAG_CONDITION_MON);
            (*menu).monPicSpriteId = SPRITE_NONE;
        } else {
            (*menu).monPicSpriteId = spriteId;
            gSprites[(*menu).monPicSpriteId].callback = Some(MonPicGfxSpriteCallback);
            (*menu).monGfxPtr =
                ((VRAM as usize as *mut c_void as *mut u8).at(0x10000) as *mut c_void as *mut u8)
                    .at((*menu).monGfxTileStart as i32 * 32) as *mut c_void;
            (*menu).monPalIndex = 0x100 + (*menu).monPalIndex * 16;
        }
    } else {
        {
            let mut _src: *mut c_void = GetConditionMonPicGfx(id);
            let mut _dest: *mut c_void = (*menu).monGfxPtr;
            let mut _size: u32 = MON_PIC_SIZE as u32;
            {
                {
                    {
                        let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x80000000 | (_size / 2));
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
        }
        LoadPalette(GetConditionMonPal(id), (*menu).monPalIndex, 32);
    }
}
pub(crate) unsafe fn VBlankCB_PokenavConditionGraph() {
    let graph: *mut ConditionGraph = GetConditionGraphPtr();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ConditionGraph_Draw(graph);
    ScanlineEffect_InitHBlankDmaTransfer();
}
unsafe fn SetExitVBlank() {
    SetPokenavVBlankCallback();
}
unsafe fn ToggleGraphData(showBg: u8) {
    if showBg != 0 {
        ShowBg(2);
    } else {
        HideBg(2);
    }
}
unsafe fn DoConditionGraphEnterTransition() {
    let graph: *mut ConditionGraph = GetConditionGraphPtr();
    let id: u8 = GetConditionGraphMenuCurrentLoadIndex() as u8;
    sInitialLoadId.set(id);
    ConditionGraph_SetNewPositions(
        graph,
        (*graph).savedPositions[3].as_mut_ptr(),
        (*graph).savedPositions[id].as_mut_ptr(),
    );
    ConditionGraph_TryUpdate(graph);
}
unsafe fn DoConditionGraphExitTransition() {
    let graph: *mut ConditionGraph = GetConditionGraphPtr();
    if IsConditionMenuSearchMode() != 0
        || GetConditionGraphCurrentListIndex() as i32 != GetMonListCount() as i32 - 1
    {
        ConditionGraph_SetNewPositions(
            graph,
            (*graph).savedPositions[GetConditionGraphMenuCurrentLoadIndex()].as_mut_ptr(),
            (*graph).savedPositions[3].as_mut_ptr(),
        );
    }
}
pub unsafe fn GetMonMarkingsData() -> u8 {
    let menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    if IsConditionMenuSearchMode() == 1 {
        return (*menu).marksMenu.markings;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
