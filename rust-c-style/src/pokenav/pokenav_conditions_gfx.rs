//! Translated from `src/pokenav_conditions_gfx.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): gConditionGraphData_Pal gConditionText_Pal sConditionGraphData_Gfx sConditionGraphData_Tilemap sMonMarkings_Pal sMenuBgTemplates sMonNameGenderWindowTemplate sListIndexWindowTemplate sUnusedWindowTemplate1 sUnusedWindowTemplate2 sLoopedTaskFuncs

/// `struct Pokenav_ConditionMenuGfx`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_ConditionMenuGfx {
    pub loopedTaskId: u32,
    pub tilemapBuffers: CArray<CArray<u8, 2048>, 3>,
    pub filler: CArray<u8, 2>,
    pub partyPokeballSpriteIds: CArray<u8, 7>,
    pub callback: Option<unsafe extern "C" fn() -> u32>,
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
static sLoopedTaskFuncs: Table<CArray<Option<unsafe extern "C" fn(i32) -> u32>, 7>> =
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

pub(crate) static mut sInitialLoadId: u8 = 0;

unsafe extern "C" {
    static gPokenavCondition_Gfx: CArray<u32, 0>;
    static gPokenavCondition_Pal: CArray<u16, 0>;
    static gPokenavCondition_Tilemap: CArray<u32, 0>;
    static gPokenavOptions_Tilemap: CArray<u16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static gText_Number2: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut c_void;
    fn AreLeftHeaderSpritesMoving() -> u32;
    fn BgDmaFill(a0: u32, a1: u8, a2: i32, a3: i32);
    fn BufferMonMarkingsMenuTiles();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConditionGraph_Draw(a0: *mut ConditionGraph);
    fn ConditionGraph_InitResetScanline(a0: *mut ConditionGraph);
    fn ConditionGraph_InitWindow(a0: u8);
    fn ConditionGraph_ResetScanline(a0: *mut ConditionGraph) -> u8;
    fn ConditionGraph_SetNewPositions(
        a0: *mut ConditionGraph,
        a1: *mut UCoords16,
        a2: *mut UCoords16,
    );
    fn ConditionGraph_TryUpdate(a0: *mut ConditionGraph) -> u8;
    fn ConditionMenu_UpdateMonEnter(a0: *mut ConditionGraph, a1: *mut i16) -> u8;
    fn ConditionMenu_UpdateMonExit(a0: *mut ConditionGraph, a1: *mut i16) -> u8;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyPaletteIntoBufferUnfaded(a0: *mut u16, a1: u32, a2: u32);
    fn CopyToBgTilemapBufferRect(a0: u8, a1: *mut c_void, a2: u8, a3: u8, a4: u8, a5: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateConditionSparkleSprites(a0: *mut *mut Sprite, a1: u8, a2: u8);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateMonMarkingAllCombosSprite(a0: u16, a1: u16, a2: *mut u16) -> *mut Sprite;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroyConditionSparkleSprites(a0: *mut *mut Sprite);
    fn DestroySprite(a0: *mut Sprite);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeConditionSparkles(a0: *mut *mut Sprite);
    fn FreeMonMarkingsMenu();
    fn FreePokenavSubstruct(a0: u32);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetConditionGraphCurrentListIndex() -> u16;
    fn GetConditionGraphMenuCurrentLoadIndex() -> i8;
    fn GetConditionGraphPtr() -> *mut ConditionGraph;
    fn GetConditionMonDataBuffer() -> u16;
    fn GetConditionMonLocationText(a0: u8) -> *mut u8;
    fn GetConditionMonNameText(a0: u8) -> *mut u8;
    fn GetConditionMonPal(a0: u8) -> *mut c_void;
    fn GetConditionMonPicGfx(a0: u8) -> *mut c_void;
    fn GetMonListCount() -> u16;
    fn GetNumConditionMonSparkles() -> u8;
    fn GetSubstructPtr(a0: u32) -> *mut c_void;
    fn HideBg(a0: u8);
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgTemplates(a0: *mut BgTemplate, a1: i32);
    fn InitMonMarkingsMenu(a0: *mut MonMarkingsMenu);
    fn IsConditionMenuSearchMode() -> u32;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn IsPaletteFadeActive() -> u32;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadConditionGraphMenuGfx() -> u32;
    fn LoadConditionMonPicTemplate(
        a0: *mut SpriteSheet,
        a1: *mut SpriteTemplate,
        a2: *mut SpritePalette,
    );
    fn LoadConditionSelectionIcons(
        a0: *mut SpriteSheet,
        a1: *mut SpriteTemplate,
        a2: *mut SpritePalette,
    );
    fn LoadConditionSparkle(a0: *mut SpriteSheet, a1: *mut SpritePalette);
    fn LoadLeftHeaderGfxForIndex(a0: u32);
    fn LoadNextConditionMenuMonData(a0: u8) -> u32;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn LoadSpriteSheet(a0: *mut SpriteSheet) -> u16;
    fn LoadSpriteSheets(a0: *mut SpriteSheet);
    fn MainMenuLoopedTaskIsBusy() -> u32;
    fn MoveConditionMonOffscreen(a0: *mut i16) -> u8;
    fn OpenMonMarkingsMenu(a0: u8, a1: i16, a2: i16);
    fn PokenavFadeScreen(a0: i32);
    fn PokenavFillPalette(a0: u32, a1: u16);
    fn Pokenav_AllocAndLoadPalettes(a0: *mut SpritePalette);
    fn PrintHelpBarText(a0: u32);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetConditionSparkleSprites(a0: *mut *mut Sprite);
    fn ScanlineEffect_InitHBlankDmaTransfer();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetLeftHeaderSpritesInvisibility();
    fn SetPokenavVBlankCallback();
    fn SetVBlankCallback_(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowLeftHeaderGfx(a0: u32, a1: u32, a2: u32);
    fn SlideMenuHeaderDown();
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TryGetMonMarkId() -> u8;
    fn WaitForHelpBar() -> u32;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn OpenConditionGraphMenu() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        AllocSubstruct(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX, 14508)
            as *mut Pokenav_ConditionMenuGfx;
    if menu.is_null() {
        return FALSE as u32;
    }
    (*menu).monPicSpriteId = SPRITE_NONE;
    (*menu).loopedTaskId = CreateLoopedTask(Some(LoopedTask_OpenConditionGraphMenu), 1);
    (*menu).callback = Some(GetConditionGraphMenuLoopedTaskActive);
    (*menu).windowModeState = 0;
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateConditionGraphMenuLoopedTask(id: i32) {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    (*menu).loopedTaskId = CreateLoopedTask(sLoopedTaskFuncs[id], 1);
    (*menu).callback = Some(GetConditionGraphMenuLoopedTaskActive);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsConditionGraphMenuLoopedTaskActive() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    return (*menu).callback.unwrap_unchecked()();
}
pub(crate) unsafe extern "C" fn GetConditionGraphMenuLoopedTaskActive() -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    return IsLoopedTaskActive((*menu).loopedTaskId);
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenConditionGraphMenu(state: i32) -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
                gPokenavCondition_Gfx.as_ptr().cast_mut() as *mut c_void,
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
                gPokenavCondition_Tilemap.as_ptr().cast_mut(),
                (*menu).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
            );
            SetBgTilemapBuffer(3, (*menu).tilemapBuffers[0].as_mut_ptr() as *mut c_void);
            if IsConditionMenuSearchMode() == TRUE as u32 {
                CopyToBgTilemapBufferRect(
                    3,
                    gPokenavOptions_Tilemap.as_ptr().cast_mut() as *mut c_void,
                    0,
                    5,
                    9,
                    4,
                );
            }
            CopyBgTilemapBufferToVram(3);
            CopyPaletteIntoBufferUnfaded(gPokenavCondition_Pal.as_ptr().cast_mut(), 16, 32);
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_ExitConditionGraphMenu(state: i32) -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_TransitionMons(state: i32) -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_MoveCursorNoTransition(state: i32) -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_SlideMonOut(state: i32) -> u32 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_OpenMonMarkingsWindow(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn LoopedTask_CloseMonMarkingsWindow(state: i32) -> u32 {
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
    return LT_FINISH;
}
pub(crate) unsafe extern "C" fn UnusedPrintNumberString(dst: *mut u8, num: u16) -> *mut u8 {
    let mut txtPtr: *mut u8 =
        ConvertIntToDecimalStringN(dst, num as i32, STR_CONV_MODE_RIGHT_ALIGN, 4);
    txtPtr = StringCopy(txtPtr, gText_Number2.as_ptr().cast_mut());
    return txtPtr;
}
pub(crate) unsafe extern "C" fn UpdateConditionGraphMenuWindows(
    mode: u8,
    bufferIndex: u16,
    winMode: u8,
) -> u32 {
    let mut text: CArray<u8, 32> = zeroed();
    let mut str: *mut u8 = null_mut();
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
                StringCopy(&raw mut text[5], gText_Number2.as_ptr().cast_mut());
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
            let mut fall = false;
            if sw1 == 0 {
                fall = true;
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
                fall = true;
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
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn CopyUnusedConditionWindowsToVram() {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    CopyWindowToVram((*menu).unusedWindowId1, COPYWIN_FULL);
    CopyWindowToVram((*menu).unusedWindowId2, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn SpriteCB_PartyPokeball(sprite: *mut Sprite) {
    if (*sprite).data[0] as i32 == GetConditionGraphCurrentListIndex() as i32 {
        StartSpriteAnim(sprite, CONDITION_ICON_SELECTED);
    } else {
        StartSpriteAnim(sprite, CONDITION_ICON_UNSELECTED);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HighlightCurrentPartyIndexPokeball(sprite: *mut Sprite) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonMarkingsCallback(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, TryGetMonMarkId());
}
pub(crate) unsafe extern "C" fn CreateMonMarkingsOrPokeballIndicators() {
    let mut sprSheets: CArray<SpriteSheet, 4> = zeroed();
    let mut sprTemplate: SpriteTemplate = zeroed();
    let mut sprPals: CArray<SpritePalette, 3> = zeroed();
    let mut sprSheet: SpriteSheet = zeroed();
    let mut sprite: *mut Sprite = null_mut();
    let mut i: u16 = 0;
    let mut spriteId: u16 = 0;
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
pub(crate) unsafe extern "C" fn FreeConditionMenuGfx(menu: *mut Pokenav_ConditionMenuGfx) {
    let mut i: u8 = 0;
    if IsConditionMenuSearchMode() == TRUE as u32 {
        DestroySprite((*menu).monMarksSprite);
        FreeSpriteTilesByTag(TAG_CONDITION_MARKINGS_MENU);
        FreeSpriteTilesByTag(TAG_CONDITION_MON_MARKINGS);
        FreeSpritePaletteByTag(TAG_CONDITION_MARKINGS_MENU);
        FreeSpritePaletteByTag(TAG_CONDITION_MON_MARKINGS);
    } else {
        i = 0;
        while i < 7 {
            DestroySprite(&raw mut gSprites[(*menu).partyPokeballSpriteIds[i]]);
            i += 1;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeConditionGraphMenuSubstruct2() {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonPicGfxSpriteCallback(sprite: *mut Sprite) {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    (*sprite).x = (*menu).monTransitionX + 38;
}
pub(crate) unsafe extern "C" fn CreateConditionMonPic(id: u8) {
    let mut sprTemplate: SpriteTemplate = zeroed();
    let mut sprSheet: SpriteSheet = zeroed();
    let mut sprPal: SpritePalette = zeroed();
    let mut spriteId: u8 = 0;
    let mut menu: *mut Pokenav_ConditionMenuGfx =
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
                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                        volatile_write(dmaRegs, _src as usize as u32);
                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                        volatile_write(dmaRegs.at(2), 0x80000000 | _size / 2);
                        let _ = (dmaRegs.at(2)).read_volatile();
                    }
                }
            }
        }
        LoadPalette(GetConditionMonPal(id), (*menu).monPalIndex, 32);
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_PokenavConditionGraph() {
    let mut graph: *mut ConditionGraph = GetConditionGraphPtr();
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
    ConditionGraph_Draw(graph);
    ScanlineEffect_InitHBlankDmaTransfer();
}
pub(crate) unsafe extern "C" fn SetExitVBlank() {
    SetPokenavVBlankCallback();
}
pub(crate) unsafe extern "C" fn ToggleGraphData(showBg: u8) {
    if showBg != 0 {
        ShowBg(2);
    } else {
        HideBg(2);
    }
}
pub(crate) unsafe extern "C" fn DoConditionGraphEnterTransition() {
    let mut graph: *mut ConditionGraph = GetConditionGraphPtr();
    let mut id: u8 = GetConditionGraphMenuCurrentLoadIndex() as u8;
    sInitialLoadId = id;
    ConditionGraph_SetNewPositions(
        graph,
        (*graph).savedPositions[3].as_mut_ptr(),
        (*graph).savedPositions[id].as_mut_ptr(),
    );
    ConditionGraph_TryUpdate(graph);
}
pub(crate) unsafe extern "C" fn DoConditionGraphExitTransition() {
    let mut graph: *mut ConditionGraph = GetConditionGraphPtr();
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMonMarkingsData() -> u8 {
    let mut menu: *mut Pokenav_ConditionMenuGfx =
        GetSubstructPtr(POKENAV_SUBSTRUCT_CONDITION_GRAPH_MENU_GFX)
            as *mut Pokenav_ConditionMenuGfx;
    if IsConditionMenuSearchMode() == 1 {
        return (*menu).marksMenu.markings;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
