//! Translated from `src/pokenav_main_menu.c` by tools/rustport/c2rs.py.
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
    unused_assignments,
    unused_variables
)]

use crate::bg::{
    ChangeBgY, CopyBgTilemapBufferToVram, GetBgY, IsDma3ManagerBusyWithBgCopy,
    ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::menu::{
    AddTextPrinterParameterized3, DecompressAndCopyTileDataToVram,
    FreeTempTileDataBuffersIfPossible, ResetBgPositions, ResetTempTileDataBuffers,
};
use crate::palette::{BeginNormalPaletteFade, BlendPalettes, LoadPalette, gPaletteFade};
use crate::pokenav::{AllocSubstruct, GetSubstructPtr, IsLoopedTaskActive};
use crate::pokenav_menu_handler_gfx::{FreeMenuHandlerSubstruct2, ResetBldCnt_};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AllocSpritePalette, FreeAllSpritePalettes, FreeSpritePaletteByTag, FreeSpriteTilesByTag,
    GetSpriteTileStartByTag, IndexOfSpritePaletteTag, ResetSpriteData,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect, FreeAllWindowBuffers,
    PutWindowTilemap,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `GetDecompressedDataSize` with this module's view of its types.
#[inline]
unsafe fn GetDecompressedDataSize(a0: *mut u32) -> u32 {
    unsafe { crate::decompress::GetDecompressedDataSize(a0 as _) }
}
/// `InitBgFromTemplate` with this module's view of its types.
#[inline]
unsafe fn InitBgFromTemplate(a0: *mut BgTemplate) {
    unsafe {
        crate::bg::InitBgFromTemplate(a0 as _);
    }
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
/// `LoadCompressedSpriteSheet` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16 {
    unsafe { crate::decompress::LoadCompressedSpriteSheet(a0 as _) }
}
/// `RequestDma3Copy` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Copy(a0: *mut c_void, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Copy(a0 as _, a1 as _, a2, a3) }
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
// Data tables (translate with cdata.py): sSpinningPokenav_Pal sSpinningPokenav_Gfx sBlueLightCopy gPokenavMainMenuBgTemplates sHelpBarWindowTemplate sHelpBarTexts sHelpBarTextColors sSpinningPokenavSpriteSheet sSpinningNavgearPalettes sMenuLeftHeaderSpriteSheet sMenuLeftHeaderSpriteSheets sPokenavSubMenuLeftHeaderSpriteSheets sSpinningPokenavSpriteOam sSpinningPokenavAnims sSpinningPokenavAnimTable sSpinningPokenavSpriteTemplate sOamData_LeftHeader sOamData_SubmenuLeftHeader sLeftHeaderSpriteTemplate sSubmenuLeftHeaderSpriteTemplate

/// `struct Pokenav_MainMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Pokenav_MainMenu {
    pub loopTask: Option<unsafe fn(u32)>,
    pub isLoopTaskActiveFunc: Option<unsafe fn() -> u32>,
    pub unused: u32,
    pub currentTaskId: u32,
    pub helpBarWindowId: u32,
    pub palettes: u32,
    pub spinningPokenav: *mut Sprite,
    pub leftHeaderSprites: CArray<*mut Sprite, 2>,
    pub submenuLeftHeaderSprites: CArray<*mut Sprite, 2>,
    pub tilemapBuffer: CArray<u8, 2048>,
}

unsafe impl Sync for Pokenav_MainMenu {}

/// `struct CompressedSpriteSheetNoSize`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CompressedSpriteSheetNoSize {
    pub data: *mut u32,
    pub tag: u32,
}

unsafe impl Sync for CompressedSpriteSheetNoSize {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Pokenav_MainMenu>() == 2092);
    assert!(offset_of!(Pokenav_MainMenu, loopTask) == 0);
    assert!(offset_of!(Pokenav_MainMenu, isLoopTaskActiveFunc) == 4);
    assert!(offset_of!(Pokenav_MainMenu, unused) == 8);
    assert!(offset_of!(Pokenav_MainMenu, currentTaskId) == 12);
    assert!(offset_of!(Pokenav_MainMenu, helpBarWindowId) == 16);
    assert!(offset_of!(Pokenav_MainMenu, palettes) == 20);
    assert!(offset_of!(Pokenav_MainMenu, spinningPokenav) == 24);
    assert!(offset_of!(Pokenav_MainMenu, leftHeaderSprites) == 28);
    assert!(offset_of!(Pokenav_MainMenu, submenuLeftHeaderSprites) == 36);
    assert!(offset_of!(Pokenav_MainMenu, tilemapBuffer) == 44);
    assert!(size_of::<CompressedSpriteSheetNoSize>() == 8);
    assert!(offset_of!(CompressedSpriteSheetNoSize, data) == 0);
    assert!(offset_of!(CompressedSpriteSheetNoSize, tag) == 4);
};

static gPokenavMainMenuBgTemplates: Table<CArray<BgTemplate, 1>> =
    Table((&raw const crate::data::pokenav_main_menu::gPokenavMainMenuBgTemplates).cast());
static sHelpBarTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::pokenav_main_menu::sHelpBarTextColors).cast());
static sHelpBarTexts: Table<CArray<*mut u8, 12>> =
    Table((&raw const crate::data::pokenav_main_menu::sHelpBarTexts).cast());
static sHelpBarWindowTemplate: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::pokenav_main_menu::sHelpBarWindowTemplate).cast());
static sLeftHeaderSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_main_menu::sLeftHeaderSpriteTemplate).cast());
static sMenuLeftHeaderSpriteSheet: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::pokenav_main_menu::sMenuLeftHeaderSpriteSheet).cast());
static sMenuLeftHeaderSpriteSheets: Table<CArray<CompressedSpriteSheet, 6>> =
    Table((&raw const crate::data::pokenav_main_menu::sMenuLeftHeaderSpriteSheets).cast());
static sPokenavSubMenuLeftHeaderSpriteSheets: Table<CArray<CompressedSpriteSheetNoSize, 7>> = Table(
    (&raw const crate::data::pokenav_main_menu::sPokenavSubMenuLeftHeaderSpriteSheets).cast(),
);
static sSpinningNavgearPalettes: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::pokenav_main_menu::sSpinningNavgearPalettes).cast());
static sSpinningPokenavSpriteSheet: Table<CArray<CompressedSpriteSheet, 1>> =
    Table((&raw const crate::data::pokenav_main_menu::sSpinningPokenavSpriteSheet).cast());
static sSpinningPokenavSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_main_menu::sSpinningPokenavSpriteTemplate).cast());
static sSubmenuLeftHeaderSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::pokenav_main_menu::sSubmenuLeftHeaderSpriteTemplate).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

pub unsafe fn InitPokenavMainMenu() -> u32 {
    let menu: *mut Pokenav_MainMenu =
        AllocSubstruct(POKENAV_SUBSTRUCT_MAIN_MENU, 2092) as *mut Pokenav_MainMenu;
    if menu.is_null() {
        return FALSE as u32;
    }
    ResetSpriteData();
    FreeAllSpritePalettes();
    (*menu).currentTaskId = CreateLoopedTask(Some(LoopedTask_InitPokenavMenu), 1);
    TRUE as u32
}
pub unsafe fn PokenavMainMenuLoopedTaskIsActive() -> u32 {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    IsLoopedTaskActive((*menu).currentTaskId)
}
pub unsafe fn ShutdownPokenav() {
    PlaySE(SE_POKENAV_OFF);
    ResetBldCnt_();
    BeginNormalPaletteFade(PALETTES_ALL, -1, 0, 16, 0);
}
pub unsafe fn WaitForPokenavShutdownFade() -> u32 {
    if gPaletteFade.active() == 0 {
        FreeMenuHandlerSubstruct2();
        CleanupPokenavMainMenuResources();
        FreeAllWindowBuffers();
        return FALSE as u32;
    }
    TRUE as u32
}
pub(crate) unsafe fn LoopedTask_InitPokenavMenu(state: i32) -> u32 {
    let mut menu: *mut Pokenav_MainMenu = null_mut();
    match state {
        0 => {
            SetGpuReg(REG_OFFSET_DISPCNT, 4160);
            FreeAllWindowBuffers();
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, gPokenavMainMenuBgTemplates.as_ptr().cast_mut(), 1);
            ResetBgPositions();
            ResetTempTileDataBuffers();
            return LT_INC_AND_CONTINUE;
        }
        1 => {
            menu = GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
            DecompressAndCopyTileDataToVram(
                0,
                (&raw const (*(&raw const crate::data::graphics::gPokenavHeader_Gfx)
                    .cast::<CArray<u32, 0>>()))
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            SetBgTilemapBuffer(0, (*menu).tilemapBuffer.as_mut_ptr() as *mut c_void);
            CopyToBgTilemapBuffer(
                0,
                (&raw const (*(&raw const crate::data::graphics::gPokenavHeader_Tilemap)
                    .cast::<CArray<u32, 0>>()))
                    .cast_mut() as *mut c_void,
                0,
                0,
            );
            CopyPaletteIntoBufferUnfaded(
                (*(&raw const crate::data::graphics::gPokenavHeader_Pal).cast::<CArray<u16, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                32,
            );
            CopyBgTilemapBufferToVram(0);
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if FreeTempTileDataBuffersIfPossible() != 0 {
                return LT_PAUSE;
            }
            InitHelpBar();
            return LT_INC_AND_PAUSE;
        }
        3 => {
            if IsDma3ManagerBusyWithBgCopy() != 0 {
                return LT_PAUSE;
            }
            InitPokenavMainMenuResources();
            CreateLeftHeaderSprites();
            ShowBg(0);
            return LT_FINISH;
        }
        _ => {
            return LT_FINISH;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn SetActiveMenuLoopTasks(createLoopTask: *mut c_void, isLoopTaskActive: *mut c_void) {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*menu).loopTask = core::mem::transmute::<_, Option<unsafe fn(u32)>>(createLoopTask);
    (*menu).isLoopTaskActiveFunc =
        core::mem::transmute::<_, Option<unsafe fn() -> u32>>(isLoopTaskActive);
    (*menu).unused = 0;
}
pub unsafe fn RunMainMenuLoopedTask(state: u32) {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*menu).unused = 0;
    (*menu).loopTask.unwrap_unchecked()(state);
}
pub unsafe fn IsActiveMenuLoopTaskActive() -> u32 {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*menu).isLoopTaskActiveFunc.unwrap_unchecked()()
}
pub unsafe fn SlideMenuHeaderUp() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*menu).currentTaskId = CreateLoopedTask(Some(LoopedTask_SlideMenuHeaderUp), 4);
}
pub unsafe fn SlideMenuHeaderDown() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*menu).currentTaskId = CreateLoopedTask(Some(LoopedTask_SlideMenuHeaderDown), 4);
}
pub unsafe fn MainMenuLoopedTaskIsBusy() -> u32 {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    IsLoopedTaskActive((*menu).currentTaskId)
}
pub(crate) unsafe fn LoopedTask_SlideMenuHeaderUp(state: i32) -> u32 {
    match state {
        1 => {
            return LT_INC_AND_PAUSE;
        }
        0 => {
            return LT_INC_AND_PAUSE;
        }
        2 => {
            if ChangeBgY(0, 384, BG_COORD_ADD) >= 0x2000 {
                ChangeBgY(0, 0x2000, BG_COORD_SET);
                return LT_FINISH;
            }
            return LT_PAUSE;
        }
        _ => {
            return LT_FINISH;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn LoopedTask_SlideMenuHeaderDown(state: i32) -> u32 {
    if ChangeBgY(0, 384, BG_COORD_SUB) <= 0 {
        ChangeBgY(0, 0, BG_COORD_SET);
        return LT_FINISH;
    }
    LT_PAUSE
}
pub unsafe fn CopyPaletteIntoBufferUnfaded(palette: *mut u16, bufferOffset: u32, size: u32) {
    CpuSet(
        palette as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[bufferOffset] as *mut c_void,
        (size / 2) & 0x1FFFFF,
    );
}
pub unsafe fn Pokenav_AllocAndLoadPalettes(palettes: *mut SpritePalette) {
    let mut index: u32 = 0;
    let mut current: *mut SpritePalette = palettes;
    while !(*current).data.is_null() {
        index = AllocSpritePalette((*current).tag) as u32;
        if index == 0xFF {
            break;
        } else {
            index = 0x100 + index * 16;
            CopyPaletteIntoBufferUnfaded((*current).data, index, 32);
        }
        current = current.at(1);
    }
}
pub unsafe fn PokenavFillPalette(palIndex: u32, fillValue: u16) {
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, fillValue);
            CpuSet(
                &raw mut tmp as *mut c_void,
                &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[0x100 + palIndex * 16] as *mut c_void,
                0x1000010,
            );
        }
    }
}
pub unsafe fn PokenavCopyPalette(
    mut src: *mut u16,
    mut dest: *mut u16,
    mut size: i32,
    a3: i32,
    a4: i32,
    mut palette: *mut u16,
) {
    if a4 == 0 {
        CpuSet(
            src as *mut c_void,
            palette as *mut c_void,
            (size * 2 / 2) as u32 & 0x1FFFFF,
        );
    } else if a4 >= a3 {
        CpuSet(
            dest as *mut c_void,
            palette as *mut c_void,
            (size * 2 / 2) as u32 & 0x1FFFFF,
        );
    } else {
        let mut r: i32 = 0;
        let mut g: i32 = 0;
        let mut b: i32 = 0;
        let mut r1: i32 = 0;
        let mut g1: i32 = 0;
        let mut b1: i32 = 0;
        while ({
            let t1 = size;
            size -= 1;
            t1
        }) != 0
        {
            r = *src as i32 & 0x1F;
            g = (*src >> 5) as i32 & 0x1F;
            b = (*src >> 10) as i32 & 0x1F;
            r1 = (div_i32(((*dest as i32 & 0x1F) << 8) - (r << 8), a3) * a4) >> 8;
            g1 = (div_i32((((*dest >> 5) as i32 & 0x1F) << 8) - (g << 8), a3) * a4) >> 8;
            b1 = (div_i32((((*dest >> 10) as i32 & 0x1F) << 8) - (b << 8), a3) * a4) >> 8;
            r = (r + r1) & 0x1F;
            g = (g + g1) & 0x1F;
            b = (b + b1) & 0x1F;
            *palette = (b as u16) << 10 | (g as u16) << 5 | r as u16;
            src = src.at(1);
            dest = dest.at(1);
            palette = palette.at(1);
        }
    }
}
pub unsafe fn PokenavFadeScreen(fadeType: i32) {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    match fadeType {
        POKENAV_FADE_TO_BLACK => {
            BeginNormalPaletteFade((*menu).palettes, -2, 0, 16, 0);
        }
        POKENAV_FADE_FROM_BLACK => {
            BeginNormalPaletteFade((*menu).palettes, -2, 16, 0, 0);
        }
        POKENAV_FADE_TO_BLACK_ALL => {
            BeginNormalPaletteFade(PALETTES_ALL, -2, 0, 16, 0);
        }
        POKENAV_FADE_FROM_BLACK_ALL => {
            BeginNormalPaletteFade(PALETTES_ALL, -2, 16, 0, 0);
        }
        _ => {}
    }
}
pub unsafe fn IsPaletteFadeActive() -> u32 {
    gPaletteFade.active() as u32
}
pub unsafe fn FadeToBlackExceptPrimary() {
    BlendPalettes(0xfffefffe, 16, 0);
}
pub unsafe fn InitBgTemplates(mut templates: *mut BgTemplate, count: i32) {
    for i in 0..count {
        InitBgFromTemplate({
            let t1 = templates;
            templates = templates.at(1);
            t1
        });
    }
}
unsafe fn InitHelpBar() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    InitWindows((&raw const sHelpBarWindowTemplate[0]).cast_mut());
    (*menu).helpBarWindowId = 0;
    DrawHelpBar((*menu).helpBarWindowId);
    PutWindowTilemap((*menu).helpBarWindowId as u8);
    CopyWindowToVram((*menu).helpBarWindowId as u8, COPYWIN_FULL);
}
pub unsafe fn PrintHelpBarText(textId: u32) {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    DrawHelpBar((*menu).helpBarWindowId);
    AddTextPrinterParameterized3(
        (*menu).helpBarWindowId as u8,
        FONT_NORMAL,
        0,
        1,
        sHelpBarTextColors.as_ptr().cast_mut(),
        0,
        sHelpBarTexts[textId],
    );
}
pub unsafe fn WaitForHelpBar() -> u32 {
    IsDma3ManagerBusyWithBgCopy() as u32
}
unsafe fn DrawHelpBar(windowId: u32) {
    FillWindowPixelBuffer(windowId as u8, 68);
    FillWindowPixelRect(windowId as u8, 85, 0, 0, 0x80, 1);
}
unsafe fn InitPokenavMainMenuResources() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    for i in 0..1i32 {
        LoadCompressedSpriteSheet((&raw const sSpinningPokenavSpriteSheet[i]).cast_mut());
    }
    Pokenav_AllocAndLoadPalettes(sSpinningNavgearPalettes.as_ptr().cast_mut());
    (*menu).palettes = 0xfffffffe & !(shl_i32(0x10000, IndexOfSpritePaletteTag(0) as u32) as u32);
    let spriteId: u8 = CreateSprite(
        (&raw const *sSpinningPokenavSpriteTemplate).cast_mut(),
        220,
        12,
        0,
    );
    (*menu).spinningPokenav = &raw mut gSprites[spriteId];
}
unsafe fn CleanupPokenavMainMenuResources() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    DestroySprite((*menu).spinningPokenav);
    FreeSpriteTilesByTag(0);
    FreeSpritePaletteByTag(0);
}
pub(crate) unsafe fn SpriteCB_SpinningPokenav(sprite: *mut Sprite) {
    (*sprite).y2 = -((GetBgY(0) / 256) as i16);
}
pub unsafe fn GetSpinningPokenavSprite() -> *mut Sprite {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*(*menu).spinningPokenav).callback = Some(SpriteCallbackDummy);
    (*menu).spinningPokenav
}
pub unsafe fn HideSpinningPokenavSprite() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    (*(*menu).spinningPokenav).x = 220;
    (*(*menu).spinningPokenav).y = 12;
    (*(*menu).spinningPokenav).callback = Some(SpriteCB_SpinningPokenav);
    (*(*menu).spinningPokenav).set_invisible(FALSE as u16);
    (*(*menu).spinningPokenav).oam.set_priority(0);
    (*(*menu).spinningPokenav).subpriority = 0;
}
unsafe fn CreateLeftHeaderSprites() {
    let mut spriteId: i32 = 0;
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    LoadCompressedSpriteSheet((&raw const *sMenuLeftHeaderSpriteSheet).cast_mut());
    AllocSpritePalette(1);
    AllocSpritePalette(2);
    for i in 0..2i32 {
        spriteId = CreateSprite((&raw const *sLeftHeaderSpriteTemplate).cast_mut(), 0, 0, 1) as i32;
        (*menu).leftHeaderSprites[i] = &raw mut gSprites[spriteId];
        (*(*menu).leftHeaderSprites[i]).set_invisible(TRUE as u16);
        (*(*menu).leftHeaderSprites[i]).x2 = i as i16 * 64;
        spriteId = CreateSprite(
            (&raw const *sSubmenuLeftHeaderSpriteTemplate).cast_mut(),
            0,
            0,
            2,
        ) as i32;
        (*menu).submenuLeftHeaderSprites[i] = &raw mut gSprites[spriteId];
        (*(*menu).submenuLeftHeaderSprites[i]).set_invisible(TRUE as u16);
        (*(*menu).submenuLeftHeaderSprites[i]).x2 = i as i16 * 32;
        (*(*menu).submenuLeftHeaderSprites[i]).y2 = 18;
        (*(*menu).submenuLeftHeaderSprites[i]).oam.set_tileNum(
            (*(*menu).submenuLeftHeaderSprites[i]).oam.tileNum() + (i as u16 * 8 + 64),
        );
    }
}
pub unsafe fn LoadLeftHeaderGfxForIndex(menuGfxId: u32) {
    if menuGfxId < POKENAV_GFX_PARTY_MENU {
        LoadLeftHeaderGfxForMenu(menuGfxId);
    } else {
        LoadLeftHeaderGfxForSubMenu(menuGfxId - POKENAV_GFX_PARTY_MENU);
    }
}
pub unsafe fn UpdateRegionMapRightHeaderTiles(menuGfxId: u32) {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    if menuGfxId == POKENAV_GFX_MAP_MENU_ZOOMED_OUT {
        (*(*menu).leftHeaderSprites[1])
            .oam
            .set_tileNum(GetSpriteTileStartByTag(2) + 32);
    } else {
        (*(*menu).leftHeaderSprites[1])
            .oam
            .set_tileNum(GetSpriteTileStartByTag(2) + 64);
    }
}
unsafe fn LoadLeftHeaderGfxForMenu(menuGfxId: u32) {
    let mut tag: u32 = 0;
    if menuGfxId >= POKENAV_GFX_PARTY_MENU {
        return;
    }
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    tag = sMenuLeftHeaderSpriteSheets[menuGfxId].tag as u32;
    let size: u32 = GetDecompressedDataSize(sMenuLeftHeaderSpriteSheets[menuGfxId].data);
    LoadPalette(
        (&raw const (*(&raw const crate::data::graphics::gPokenavLeftHeader_Pal)
            .cast::<CArray<u16, 0>>())[tag * 16])
            .cast_mut() as *mut c_void,
        0x100 + IndexOfSpritePaletteTag(1) as u16 * 16,
        32,
    );
    LZ77UnCompWram(
        sMenuLeftHeaderSpriteSheets[menuGfxId].data,
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
    );
    RequestDma3Copy(
        (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())
        .as_mut_ptr() as *mut c_void,
        (OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(GetSpriteTileStartByTag(2) as i32 * 32)
            as *mut c_void,
        size as u16,
        1,
    );
    (*(*menu).leftHeaderSprites[1])
        .oam
        .set_tileNum(GetSpriteTileStartByTag(2) + sMenuLeftHeaderSpriteSheets[menuGfxId].size);
    if menuGfxId == POKENAV_GFX_MAP_MENU_ZOOMED_OUT || menuGfxId == POKENAV_GFX_MAP_MENU_ZOOMED_IN {
        (*(*menu).leftHeaderSprites[1]).x2 = 56;
    } else {
        (*(*menu).leftHeaderSprites[1]).x2 = 64;
    }
}
unsafe fn LoadLeftHeaderGfxForSubMenu(menuGfxId: u32) {
    let mut tag: u32 = 0;
    if menuGfxId >= 7 {
        return;
    }
    tag = sPokenavSubMenuLeftHeaderSpriteSheets[menuGfxId].tag;
    let size: u32 = GetDecompressedDataSize(sPokenavSubMenuLeftHeaderSpriteSheets[menuGfxId].data);
    LoadPalette(
        (&raw const (*(&raw const crate::data::graphics::gPokenavLeftHeader_Pal)
            .cast::<CArray<u16, 0>>())[tag * 16])
            .cast_mut() as *mut c_void,
        0x100 + IndexOfSpritePaletteTag(2) as u16 * 16,
        32,
    );
    LZ77UnCompWram(
        sPokenavSubMenuLeftHeaderSpriteSheets[menuGfxId].data,
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[4096] as *mut c_void,
    );
    RequestDma3Copy(
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[4096] as *mut c_void,
        ((OBJ_VRAM0 as usize as *mut c_void as *mut u8).at(2048) as *mut c_void as *mut u8)
            .at(GetSpriteTileStartByTag(2) as i32 * 32) as *mut c_void,
        size as u16,
        1,
    );
}
pub unsafe fn ShowLeftHeaderGfx(menuGfxId: u32, isMain: u32, isOnRightSide: u32) {
    let mut tileTop: u32 = 0;
    if isMain == 0 {
        tileTop = 0x30;
    } else {
        tileTop = 0x10;
    }
    if menuGfxId < POKENAV_GFX_PARTY_MENU {
        ShowLeftHeaderSprites(tileTop, isOnRightSide);
    } else {
        ShowLeftHeaderSubmenuSprites(tileTop, isOnRightSide);
    }
}
pub unsafe fn HideMainOrSubMenuLeftHeader(id: u32, onRightSide: u32) {
    if id < POKENAV_GFX_PARTY_MENU {
        HideLeftHeaderSprites(onRightSide);
    } else {
        HideLeftHeaderSubmenuSprites(onRightSide);
    }
}
pub unsafe fn SetLeftHeaderSpritesInvisibility() {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    for i in 0..2i32 {
        (*(*menu).leftHeaderSprites[i]).set_invisible(TRUE as u16);
        (*(*menu).submenuLeftHeaderSprites[i]).set_invisible(TRUE as u16);
    }
}
pub unsafe fn AreLeftHeaderSpritesMoving() -> u32 {
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    if (*(*menu).leftHeaderSprites[0]).callback
        == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && (*(*menu).submenuLeftHeaderSprites[0]).callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ShowLeftHeaderSprites(startY: u32, isOnRightSide: u32) {
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    if isOnRightSide == 0 {
        start = -96;
        end = 32;
    } else {
        start = 256;
        end = 160;
    }
    for i in 0..2i32 {
        (*(*menu).leftHeaderSprites[i]).y = startY as i16;
        MoveLeftHeader((*menu).leftHeaderSprites[i], start, end, 12);
    }
}
unsafe fn ShowLeftHeaderSubmenuSprites(startY: u32, isOnRightSide: u32) {
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    if isOnRightSide == 0 {
        start = -96;
        end = 16;
    } else {
        start = 256;
        end = 192;
    }
    for i in 0..2i32 {
        (*(*menu).submenuLeftHeaderSprites[i]).y = startY as i16;
        MoveLeftHeader((*menu).submenuLeftHeaderSprites[i], start, end, 12);
    }
}
unsafe fn HideLeftHeaderSprites(isOnRightSide: u32) {
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    if isOnRightSide == 0 {
        start = 32;
        end = -96;
    } else {
        start = 192;
        end = 256;
    }
    for i in 0..2i32 {
        MoveLeftHeader((*menu).leftHeaderSprites[i], start, end, 12);
    }
}
unsafe fn HideLeftHeaderSubmenuSprites(isOnRightSide: u32) {
    let mut start: i32 = 0;
    let mut end: i32 = 0;
    let menu: *mut Pokenav_MainMenu =
        GetSubstructPtr(POKENAV_SUBSTRUCT_MAIN_MENU) as *mut Pokenav_MainMenu;
    if isOnRightSide == 0 {
        start = 16;
        end = -96;
    } else {
        start = 192;
        end = 256;
    }
    for i in 0..2i32 {
        MoveLeftHeader((*menu).submenuLeftHeaderSprites[i], start, end, 12);
    }
}
unsafe fn MoveLeftHeader(sprite: *mut Sprite, startX: i32, endX: i32, duration: i32) {
    (*sprite).x = startX as i16;
    (*sprite).data[0] = startX as i16 * 16;
    (*sprite).data[1] = div_i32((endX - startX) * 16, duration) as i16;
    (*sprite).data[2] = duration as i16;
    (*sprite).data[7] = endX as i16;
    (*sprite).callback = Some(SpriteCB_MoveLeftHeader);
}
pub(crate) unsafe fn SpriteCB_MoveLeftHeader(sprite: *mut Sprite) {
    if (*sprite).data[2] != 0 {
        (*sprite).data[2] -= 1;
        (*sprite).data[0] += (*sprite).data[1];
        (*sprite).x = (*sprite).data[0] >> 4;
        if (*sprite).x < -16 || (*sprite).x > 256 {
            (*sprite).set_invisible(TRUE as u16);
        } else {
            (*sprite).set_invisible(FALSE as u16);
        }
    } else {
        (*sprite).x = (*sprite).data[7];
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
