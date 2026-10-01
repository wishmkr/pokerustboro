//! Translated from `src/berry_tag_screen.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::berry::{GetBerryInfo, ItemIdToBerryType};
use crate::bg::{ChangeBgY, ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::item::BagGetItemIdByPocketPosition;
use crate::item_menu::{CB2_ReturnToBagMenuPocket, gBagPosition, gSpecialVar_ItemId};
use crate::item_menu_icons::{
    CreateBerryFlavorCircleSprite, CreateBerryTagSprite, FreeBerryTagSpritePalette,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    AddTextPrinterParameterized4, ClearScheduledBgCopiesToVram, DecompressAndCopyTileDataToVram,
    DoScheduledBgTilemapCopiesToVram, FreeTempTileDataBuffersIfPossible, ResetTempTileDataBuffers,
    ScheduleBgCopyTilemapToVram,
};
use crate::menu_helpers::{
    MenuHelpers_IsLinkActive, MenuHelpers_ShouldWaitForLinkRecv, ResetAllBgsCoordinates,
    ResetVramOamAndBgCntRegs, SetVBlankHBlankCallbacksToNull,
};
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, ResetPaletteFade,
    TransferPlttBuffer, UpdatePaletteFade, gPaletteFade,
};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{gTasks, task_set_func};
use crate::text::DeactivateAllTextPrinters;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap};
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
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
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
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
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
// Data tables (translate with cdata.py): sBackgroundTemplates sFontPalette sTextColors sWindowTemplates sBerryFirmnessStrings

/// `struct BerryTagScreenStruct`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct BerryTagScreenStruct {
    pub tilemapBuffers: CArray<CArray<u16, 1024>, 3>,
    pub berryId: u16,
    pub berrySpriteId: u8,
    pub flavorCircleIds: CArray<u8, 5>,
    pub gfxState: u16,
}

unsafe impl Sync for BerryTagScreenStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<BerryTagScreenStruct>() == 6156);
    assert!(offset_of!(BerryTagScreenStruct, tilemapBuffers) == 0);
    assert!(offset_of!(BerryTagScreenStruct, berryId) == 6144);
    assert!(offset_of!(BerryTagScreenStruct, berrySpriteId) == 6146);
    assert!(offset_of!(BerryTagScreenStruct, flavorCircleIds) == 6147);
    assert!(offset_of!(BerryTagScreenStruct, gfxState) == 6152);
};

const BG_TILE: i32 = 66;
const DISPLAY_SPEED: i16 = 16;
const WIN_BERRY_NAME: u8 = 0;
const WIN_BERRY_TAG: u8 = 3;
const WIN_DESC: u8 = 2;
const WIN_SIZE_FIRM: u8 = 1;

static sBackgroundTemplates: Table<CArray<BgTemplate, 4>> =
    Table((&raw const crate::data::berry_tag_screen::sBackgroundTemplates).cast());
static sBerryFirmnessStrings: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::berry_tag_screen::sBerryFirmnessStrings).cast());
static sFontPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::berry_tag_screen::sFontPalette).cast());
static sTextColors: Table<CArray<CArray<u8, 3>, 2>> =
    Table((&raw const crate::data::berry_tag_screen::sTextColors).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::berry_tag_screen::sWindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBerryTag: *mut BerryTagScreenStruct = null_mut();

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
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub unsafe fn DoBerryTagScreen() {
    sBerryTag = AllocZeroed(6156) as *mut BerryTagScreenStruct;
    (*sBerryTag).berryId = ItemIdToBerryType(gSpecialVar_ItemId) as u16;
    SetMainCallback2(Some(CB2_InitBerryTagScreen));
}
pub(crate) unsafe fn CB2_BerryTagScreen() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VblankCB() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe fn CB2_InitBerryTagScreen() {
    loop {
        if MenuHelpers_ShouldWaitForLinkRecv() == TRUE {
            break;
        }
        if InitBerryTagScreen() == TRUE {
            break;
        }
        if MenuHelpers_IsLinkActive() == TRUE {
            break;
        }
    }
}
unsafe fn InitBerryTagScreen() -> u8 {
    match gMain.state {
        0 => {
            SetVBlankHBlankCallbacksToNull();
            ResetVramOamAndBgCntRegs();
            ClearScheduledBgCopiesToVram();
            gMain.state += 1;
        }
        1 => {
            ScanlineEffect_Stop();
            gMain.state += 1;
        }
        2 => {
            ResetPaletteFade();
            gPaletteFade.set_bufferTransferDisabled(1);
            gMain.state += 1;
        }
        3 => {
            ResetSpriteData();
            gMain.state += 1;
        }
        4 => {
            FreeAllSpritePalettes();
            gMain.state += 1;
        }
        5 => {
            if MenuHelpers_IsLinkActive() == 0 {
                ResetTasks();
            }
            gMain.state += 1;
        }
        6 => {
            HandleInitBackgrounds();
            (*sBerryTag).gfxState = 0;
            gMain.state += 1;
        }
        7 => {
            if LoadBerryTagGfx() != 0 {
                gMain.state += 1;
            }
        }
        8 => {
            HandleInitWindows();
            gMain.state += 1;
        }
        9 => {
            AddBerryTagTextToBg0();
            gMain.state += 1;
        }
        10 => {
            PrintAllBerryData();
            gMain.state += 1;
        }
        11 => {
            CreateBerrySprite();
            gMain.state += 1;
        }
        12 => {
            CreateFlavorCircleSprites();
            SetFlavorCirclesVisiblity();
            gMain.state += 1;
        }
        13 => {
            CreateTask(Some(Task_HandleInput), 0);
            gMain.state += 1;
        }
        14 => {
            BlendPalettes(PALETTES_ALL, 0x10, 0);
            gMain.state += 1;
        }
        15 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
            gPaletteFade.set_bufferTransferDisabled(0);
            gMain.state += 1;
        }
        _ => {
            SetVBlankCallback(Some(VblankCB));
            SetMainCallback2(Some(CB2_BerryTagScreen));
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn HandleInitBackgrounds() {
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBackgroundTemplates.as_ptr().cast_mut(), 4);
    SetBgTilemapBuffer(
        2,
        (*sBerryTag).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
    );
    SetBgTilemapBuffer(
        3,
        (*sBerryTag).tilemapBuffers[1].as_mut_ptr() as *mut c_void,
    );
    ResetAllBgsCoordinates();
    ScheduleBgCopyTilemapToVram(2);
    ScheduleBgCopyTilemapToVram(3);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    ShowBg(0);
    ShowBg(1);
    ShowBg(2);
    ShowBg(3);
}
unsafe fn LoadBerryTagGfx() -> u8 {
    match (*sBerryTag).gfxState {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                2,
                (*(&raw const crate::data::graphics::gBerryCheck_Gfx).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*sBerryTag).gfxState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    (*(&raw const crate::data::graphics::gBerryTag_Gfx).cast::<CArray<u32, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    (*sBerryTag).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
                );
                (*sBerryTag).gfxState += 1;
            }
        }
        2 => {
            LZDecompressWram(
                (*(&raw const crate::data::graphics::gBerryTag_Tilemap).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                (*sBerryTag).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
            );
            (*sBerryTag).gfxState += 1;
        }
        3 => {
            if (*gSaveBlock2Ptr).playerGender == MALE {
                for i in 0..1024u16 {
                    (*sBerryTag).tilemapBuffers[1][i] = 16450;
                }
            } else {
                for i in 0..1024u16 {
                    (*sBerryTag).tilemapBuffers[1][i] = 20546;
                }
            }
            (*sBerryTag).gfxState += 1;
        }
        4 => {
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBerryCheck_Pal).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                0,
                192,
            );
            (*sBerryTag).gfxState += 1;
        }
        5 => {
            LoadCompressedSpriteSheet((&raw const (*(&raw const crate::data::item_menu_icons::gBerryCheckCircleSpriteSheet).cast::<CompressedSpriteSheet>())).cast_mut());
            (*sBerryTag).gfxState += 1;
        }
        _ => {
            LoadCompressedSpritePalette((&raw const (*(&raw const crate::data::item_menu_icons::gBerryCheckCirclePaletteTable).cast::<CompressedSpritePalette>())).cast_mut());
            return TRUE;
        }
    }
    FALSE
}
pub(crate) unsafe fn HandleInitWindows() {
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadPalette(sFontPalette.as_ptr().cast_mut() as *mut c_void, 240, 32);
    for i in 0..4u16 {
        PutWindowTilemap(i as u8);
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
}
unsafe fn PrintTextInBerryTagScreen(
    windowId: u8,
    text: *mut u8,
    x: u8,
    y: u8,
    speed: i32,
    colorStructId: u8,
) {
    AddTextPrinterParameterized4(
        windowId,
        FONT_NORMAL,
        x,
        y,
        0,
        0,
        sTextColors[colorStructId].as_ptr().cast_mut(),
        speed as i8,
        text,
    );
}
unsafe fn AddBerryTagTextToBg0() {
    memcpy(
        GetBgTilemapBuffer(0) as *mut u8,
        (*sBerryTag).tilemapBuffers[2].as_mut_ptr() as *mut u8,
        2048,
    );
    FillWindowPixelBuffer(WIN_BERRY_TAG, 255);
    PrintTextInBerryTagScreen(
        WIN_BERRY_TAG,
        (*(&raw const crate::data::strings::gText_BerryTag).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringCenterAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_BerryTag).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0x40,
        ) as u8,
        1,
        0,
        1,
    );
    PutWindowTilemap(WIN_BERRY_TAG);
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn PrintAllBerryData() {
    PrintBerryNumberAndName();
    PrintBerrySize();
    PrintBerryFirmness();
    PrintBerryDescription1();
    PrintBerryDescription2();
}
unsafe fn PrintBerryNumberAndName() {
    let berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sBerryTag).berryId as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    StringCopy(gStringVar2.as_mut_ptr(), (*berry).name.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_NumberVar1Var2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PrintTextInBerryTagScreen(WIN_BERRY_NAME, gStringVar4.as_mut_ptr(), 0, 1, 0, 0);
}
unsafe fn PrintBerrySize() {
    let berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(
        WIN_SIZE_FIRM,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_SizeSlash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    if (*berry).size != 0 {
        let mut inches: u32 = (1000 * (*berry).size as i32 / 254) as u32;
        if inches % 10 > 4 {
            inches += 10;
        }
        let fraction: u32 = inches % 100 / 10;
        inches /= 100;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            inches as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            2,
        );
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            fraction as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            2,
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Var1DotVar2).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterParameterized(
            WIN_SIZE_FIRM,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            0x28,
            1,
            0,
            None,
        );
    } else {
        AddTextPrinterParameterized(
            WIN_SIZE_FIRM,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_ThreeMarks).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0x28,
            1,
            0,
            None,
        );
    }
}
unsafe fn PrintBerryFirmness() {
    let berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(
        WIN_SIZE_FIRM,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_FirmSlash).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        0x11,
        TEXT_SKIP_DRAW,
        None,
    );
    if (*berry).firmness != 0 {
        AddTextPrinterParameterized(
            WIN_SIZE_FIRM,
            FONT_NORMAL,
            sBerryFirmnessStrings[(*berry).firmness as i32 - 1],
            0x28,
            0x11,
            0,
            None,
        );
    } else {
        AddTextPrinterParameterized(
            WIN_SIZE_FIRM,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_ThreeMarks).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0x28,
            0x11,
            0,
            None,
        );
    }
}
unsafe fn PrintBerryDescription1() {
    let berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(WIN_DESC, FONT_NORMAL, (*berry).description1, 0, 1, 0, None);
}
unsafe fn PrintBerryDescription2() {
    let berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(
        WIN_DESC,
        FONT_NORMAL,
        (*berry).description2,
        0,
        0x11,
        0,
        None,
    );
}
pub(crate) unsafe fn CreateBerrySprite() {
    (*sBerryTag).berrySpriteId = CreateBerryTagSprite((*sBerryTag).berryId as u8 - 1, 56, 64);
}
unsafe fn DestroyBerrySprite() {
    DestroySprite(&raw mut gSprites[(*sBerryTag).berrySpriteId]);
    FreeBerryTagSpritePalette();
}
unsafe fn CreateFlavorCircleSprites() {
    (*sBerryTag).flavorCircleIds[0] = CreateBerryFlavorCircleSprite(64);
    (*sBerryTag).flavorCircleIds[1] = CreateBerryFlavorCircleSprite(104);
    (*sBerryTag).flavorCircleIds[2] = CreateBerryFlavorCircleSprite(144);
    (*sBerryTag).flavorCircleIds[3] = CreateBerryFlavorCircleSprite(184);
    (*sBerryTag).flavorCircleIds[4] = CreateBerryFlavorCircleSprite(224);
}
unsafe fn SetFlavorCirclesVisiblity() {
    let berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    if (*berry).spicy != 0 {
        gSprites[(*sBerryTag).flavorCircleIds[0]].set_invisible(0);
    } else {
        gSprites[(*sBerryTag).flavorCircleIds[0]].set_invisible(TRUE as u16);
    }
    if (*berry).dry != 0 {
        gSprites[(*sBerryTag).flavorCircleIds[1]].set_invisible(FALSE as u16);
    } else {
        gSprites[(*sBerryTag).flavorCircleIds[1]].set_invisible(1);
    }
    if (*berry).sweet != 0 {
        gSprites[(*sBerryTag).flavorCircleIds[2]].set_invisible(FALSE as u16);
    } else {
        gSprites[(*sBerryTag).flavorCircleIds[2]].set_invisible(TRUE as u16);
    }
    if (*berry).bitter != 0 {
        gSprites[(*sBerryTag).flavorCircleIds[3]].set_invisible(FALSE as u16);
    } else {
        gSprites[(*sBerryTag).flavorCircleIds[3]].set_invisible(TRUE as u16);
    }
    if (*berry).sour != 0 {
        gSprites[(*sBerryTag).flavorCircleIds[4]].set_invisible(FALSE as u16);
    } else {
        gSprites[(*sBerryTag).flavorCircleIds[4]].set_invisible(TRUE as u16);
    }
}
unsafe fn DestroyFlavorCircleSprites() {
    for i in 0..(FLAVOR_COUNT as u16) {
        DestroySprite(&raw mut gSprites[(*sBerryTag).flavorCircleIds[i]]);
    }
}
unsafe fn PrepareToCloseBerryTagScreen(taskId: u8) {
    PlaySE(SE_SELECT);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    task_set_func(taskId, Some(Task_CloseBerryTagScreen));
}
pub(crate) unsafe fn Task_CloseBerryTagScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyBerrySprite();
        DestroyFlavorCircleSprites();
        Free(sBerryTag as *mut c_void);
        FreeAllWindowBuffers();
        SetMainCallback2(Some(CB2_ReturnToBagMenuPocket));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_HandleInput(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let arrowKeys: u16 = gMain.newAndRepeatedKeys & DPAD_ANY as u16;
        if arrowKeys == DPAD_UP as u16 {
            TryChangeDisplayedBerry(taskId, -1);
        } else if arrowKeys == DPAD_DOWN as u16 {
            TryChangeDisplayedBerry(taskId, 1);
        } else if gMain.newKeys as i32 & 3 != 0 {
            PrepareToCloseBerryTagScreen(taskId);
        }
    }
}
unsafe fn TryChangeDisplayedBerry(taskId: u8, toMove: i8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let currPocketPosition: i16 =
        gBagPosition.scrollPosition[3] as i16 + gBagPosition.cursorPosition[3] as i16;
    let newPocketPosition: u32 = currPocketPosition as u32 + toMove as u32;
    if newPocketPosition < 46
        && BagGetItemIdByPocketPosition(POCKET_BERRIES, newPocketPosition as u16) != ITEM_NONE
    {
        if toMove < 0 {
            *data.at(1) = BG_COORD_SUB as i16;
        } else {
            *data.at(1) = BG_COORD_ADD as i16;
        }
        *data = 0;
        PlaySE(SE_SELECT);
        HandleBagCursorPositionChange(toMove);
        task_set_func(taskId, Some(Task_DisplayAnotherBerry));
    }
}
unsafe fn HandleBagCursorPositionChange(toMove: i8) {
    let scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[3];
    let cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[3];
    if toMove > 0 {
        if *cursorPos < 4 || BagGetItemIdByPocketPosition(4, *scrollPos + 8) == 0 {
            *cursorPos += toMove as u16;
        } else {
            *scrollPos += toMove as u16;
        }
    } else {
        if *cursorPos > 3 || *scrollPos == 0 {
            *cursorPos += toMove as u16;
        } else {
            *scrollPos += toMove as u16;
        }
    }
    (*sBerryTag).berryId = ItemIdToBerryType(BagGetItemIdByPocketPosition(
        POCKET_BERRIES,
        *scrollPos + *cursorPos,
    )) as u16;
}
pub(crate) unsafe fn Task_DisplayAnotherBerry(taskId: u8) {
    let mut y: i16 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data += DISPLAY_SPEED;
    *data &= 0xFF;
    if *data.at(1) == BG_COORD_ADD as i16 {
        match *data {
            48 => {
                FillWindowPixelBuffer(WIN_BERRY_NAME, 0);
            }
            64 => {
                PrintBerryNumberAndName();
            }
            80 => {
                DestroyBerrySprite();
                CreateBerrySprite();
            }
            96 => {
                FillWindowPixelBuffer(WIN_SIZE_FIRM, 0);
            }
            112 => {
                PrintBerrySize();
            }
            128 => {
                PrintBerryFirmness();
            }
            144 => {
                SetFlavorCirclesVisiblity();
            }
            160 => {
                FillWindowPixelBuffer(WIN_DESC, 0);
            }
            176 => {
                PrintBerryDescription1();
            }
            192 => {
                PrintBerryDescription2();
            }
            _ => {}
        }
    } else {
        match *data {
            48 => {
                FillWindowPixelBuffer(WIN_DESC, 0);
            }
            64 => {
                PrintBerryDescription2();
            }
            80 => {
                PrintBerryDescription1();
            }
            96 => {
                SetFlavorCirclesVisiblity();
            }
            112 => {
                FillWindowPixelBuffer(WIN_SIZE_FIRM, 0);
            }
            128 => {
                PrintBerryFirmness();
            }
            144 => {
                PrintBerrySize();
            }
            160 => {
                DestroyBerrySprite();
                CreateBerrySprite();
            }
            176 => {
                FillWindowPixelBuffer(WIN_BERRY_NAME, 0);
            }
            192 => {
                PrintBerryNumberAndName();
            }
            _ => {}
        }
    }
    if *data.at(1) == BG_COORD_ADD as i16 {
        y = -*data;
    } else {
        y = *data;
    }
    gSprites[(*sBerryTag).berrySpriteId].y2 = y;
    for i in 0..(FLAVOR_COUNT as u16) {
        gSprites[(*sBerryTag).flavorCircleIds[i]].y2 = y;
    }
    ChangeBgY(1, 0x1000, *data.at(1) as u8);
    ChangeBgY(2, 0x1000, *data.at(1) as u8);
    if *data == 0 {
        task_set_func(taskId, Some(Task_HandleInput));
    }
}
