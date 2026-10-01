//! Translated from `src/berry_tag_screen.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gBagPosition: BagPosition;
    static gBerryCheckCirclePaletteTable: CompressedSpritePalette;
    static gBerryCheckCircleSpriteSheet: CompressedSpriteSheet;
    static gBerryCheck_Gfx: CArray<u32, 0>;
    static gBerryCheck_Pal: CArray<u32, 0>;
    static gBerryTag_Gfx: CArray<u32, 0>;
    static gBerryTag_Tilemap: CArray<u32, 0>;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_ItemId: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BerryTag: CArray<u8, 0>;
    static gText_FirmSlash: CArray<u8, 0>;
    static gText_NumberVar1Var2: CArray<u8, 0>;
    static gText_SizeSlash: CArray<u8, 0>;
    static gText_ThreeMarks: CArray<u8, 0>;
    static gText_Var1DotVar2: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BagGetItemIdByPocketPosition(a0: u8, a1: u16) -> u16;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BuildOamBuffer();
    fn CB2_ReturnToBagMenuPocket();
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearScheduledBgCopiesToVram();
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CreateBerryFlavorCircleSprite(a0: i16) -> u8;
    fn CreateBerryTagSprite(a0: u8, a1: i16, a2: i16) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndCopyTileDataToVram(
        a0: u8,
        a1: *mut c_void,
        a2: u32,
        a3: u16,
        a4: u8,
    ) -> *mut c_void;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeBerryTagSpritePalette();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBerryInfo(a0: u8) -> *mut Berry;
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn ItemIdToBerryType(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut CompressedSpritePalette);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MenuHelpers_ShouldWaitForLinkRecv() -> u8;
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetAllBgsCoordinates();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ResetTempTileDataBuffers();
    fn ResetVramOamAndBgCntRegs();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankHBlankCallbacksToNull();
    fn ShowBg(a0: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoBerryTagScreen() {
    sBerryTag = AllocZeroed(6156) as *mut BerryTagScreenStruct;
    (*sBerryTag).berryId = ItemIdToBerryType(gSpecialVar_ItemId) as u16;
    SetMainCallback2(Some(CB2_InitBerryTagScreen));
}
pub(crate) unsafe extern "C" fn CB2_BerryTagScreen() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VblankCB() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn CB2_InitBerryTagScreen() {
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
pub(crate) unsafe extern "C" fn InitBerryTagScreen() -> u8 {
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
    return FALSE;
}
pub(crate) unsafe extern "C" fn HandleInitBackgrounds() {
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
pub(crate) unsafe extern "C" fn LoadBerryTagGfx() -> u8 {
    let mut i: u16 = 0;
    match (*sBerryTag).gfxState {
        0 => {
            ResetTempTileDataBuffers();
            DecompressAndCopyTileDataToVram(
                2,
                gBerryCheck_Gfx.as_ptr().cast_mut() as *mut c_void,
                0,
                0,
                0,
            );
            (*sBerryTag).gfxState += 1;
        }
        1 => {
            if FreeTempTileDataBuffersIfPossible() != TRUE {
                LZDecompressWram(
                    gBerryTag_Gfx.as_ptr().cast_mut(),
                    (*sBerryTag).tilemapBuffers[0].as_mut_ptr() as *mut c_void,
                );
                (*sBerryTag).gfxState += 1;
            }
        }
        2 => {
            LZDecompressWram(
                gBerryTag_Tilemap.as_ptr().cast_mut(),
                (*sBerryTag).tilemapBuffers[2].as_mut_ptr() as *mut c_void,
            );
            (*sBerryTag).gfxState += 1;
        }
        3 => {
            if (*gSaveBlock2Ptr).playerGender == MALE {
                i = 0;
                while i < 1024 {
                    (*sBerryTag).tilemapBuffers[1][i] = 16450;
                    i += 1;
                }
            } else {
                i = 0;
                while i < 1024 {
                    (*sBerryTag).tilemapBuffers[1][i] = 20546;
                    i += 1;
                }
            }
            (*sBerryTag).gfxState += 1;
        }
        4 => {
            LoadCompressedPalette(gBerryCheck_Pal.as_ptr().cast_mut(), 0, 192);
            (*sBerryTag).gfxState += 1;
        }
        5 => {
            LoadCompressedSpriteSheet((&raw const gBerryCheckCircleSpriteSheet).cast_mut());
            (*sBerryTag).gfxState += 1;
        }
        _ => {
            LoadCompressedSpritePalette((&raw const gBerryCheckCirclePaletteTable).cast_mut());
            return TRUE;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn HandleInitWindows() {
    let mut i: u16 = 0;
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadPalette(sFontPalette.as_ptr().cast_mut() as *mut c_void, 240, 32);
    i = 0;
    while i < 4 {
        PutWindowTilemap(i as u8);
        i += 1;
    }
    ScheduleBgCopyTilemapToVram(0);
    ScheduleBgCopyTilemapToVram(1);
}
pub(crate) unsafe extern "C" fn PrintTextInBerryTagScreen(
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
pub(crate) unsafe extern "C" fn AddBerryTagTextToBg0() {
    memcpy(
        GetBgTilemapBuffer(0) as *mut u8,
        (*sBerryTag).tilemapBuffers[2].as_mut_ptr() as *mut u8,
        2048,
    );
    FillWindowPixelBuffer(WIN_BERRY_TAG, 255);
    PrintTextInBerryTagScreen(
        WIN_BERRY_TAG,
        gText_BerryTag.as_ptr().cast_mut(),
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, gText_BerryTag.as_ptr().cast_mut(), 0x40)
            as u8,
        1,
        0,
        1,
    );
    PutWindowTilemap(WIN_BERRY_TAG);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn PrintAllBerryData() {
    PrintBerryNumberAndName();
    PrintBerrySize();
    PrintBerryFirmness();
    PrintBerryDescription1();
    PrintBerryDescription2();
}
pub(crate) unsafe extern "C" fn PrintBerryNumberAndName() {
    let mut berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        (*sBerryTag).berryId as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    StringCopy(gStringVar2.as_mut_ptr(), (*berry).name.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_NumberVar1Var2.as_ptr().cast_mut(),
    );
    PrintTextInBerryTagScreen(WIN_BERRY_NAME, gStringVar4.as_mut_ptr(), 0, 1, 0, 0);
}
pub(crate) unsafe extern "C" fn PrintBerrySize() {
    let mut berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(
        WIN_SIZE_FIRM,
        FONT_NORMAL,
        gText_SizeSlash.as_ptr().cast_mut(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    if (*berry).size != 0 {
        let mut inches: u32 = 0;
        let mut fraction: u32 = 0;
        inches = (1000 * (*berry).size as i32 / 254) as u32;
        if inches % 10 > 4 {
            inches += 10;
        }
        fraction = inches % 100 / 10;
        inches = inches / 100;
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
            gText_Var1DotVar2.as_ptr().cast_mut(),
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
            gText_ThreeMarks.as_ptr().cast_mut(),
            0x28,
            1,
            0,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintBerryFirmness() {
    let mut berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(
        WIN_SIZE_FIRM,
        FONT_NORMAL,
        gText_FirmSlash.as_ptr().cast_mut(),
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
            gText_ThreeMarks.as_ptr().cast_mut(),
            0x28,
            0x11,
            0,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintBerryDescription1() {
    let mut berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
    AddTextPrinterParameterized(WIN_DESC, FONT_NORMAL, (*berry).description1, 0, 1, 0, None);
}
pub(crate) unsafe extern "C" fn PrintBerryDescription2() {
    let mut berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
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
pub(crate) unsafe extern "C" fn CreateBerrySprite() {
    (*sBerryTag).berrySpriteId = CreateBerryTagSprite((*sBerryTag).berryId as u8 - 1, 56, 64);
}
pub(crate) unsafe extern "C" fn DestroyBerrySprite() {
    DestroySprite(&raw mut gSprites[(*sBerryTag).berrySpriteId]);
    FreeBerryTagSpritePalette();
}
pub(crate) unsafe extern "C" fn CreateFlavorCircleSprites() {
    (*sBerryTag).flavorCircleIds[0] = CreateBerryFlavorCircleSprite(64);
    (*sBerryTag).flavorCircleIds[1] = CreateBerryFlavorCircleSprite(104);
    (*sBerryTag).flavorCircleIds[2] = CreateBerryFlavorCircleSprite(144);
    (*sBerryTag).flavorCircleIds[3] = CreateBerryFlavorCircleSprite(184);
    (*sBerryTag).flavorCircleIds[4] = CreateBerryFlavorCircleSprite(224);
}
pub(crate) unsafe extern "C" fn SetFlavorCirclesVisiblity() {
    let mut berry: *mut Berry = GetBerryInfo((*sBerryTag).berryId as u8);
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
pub(crate) unsafe extern "C" fn DestroyFlavorCircleSprites() {
    let mut i: u16 = 0;
    i = 0;
    while i < FLAVOR_COUNT as u16 {
        DestroySprite(&raw mut gSprites[(*sBerryTag).flavorCircleIds[i]]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn PrepareToCloseBerryTagScreen(taskId: u8) {
    PlaySE(SE_SELECT);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
    gTasks[taskId].func = Some(Task_CloseBerryTagScreen);
}
pub(crate) unsafe extern "C" fn Task_CloseBerryTagScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyBerrySprite();
        DestroyFlavorCircleSprites();
        Free(sBerryTag as *mut c_void);
        FreeAllWindowBuffers();
        SetMainCallback2(Some(CB2_ReturnToBagMenuPocket));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let mut arrowKeys: u16 = gMain.newAndRepeatedKeys & DPAD_ANY as u16;
        if arrowKeys == DPAD_UP as u16 {
            TryChangeDisplayedBerry(taskId, -1);
        } else if arrowKeys == DPAD_DOWN as u16 {
            TryChangeDisplayedBerry(taskId, 1);
        } else if gMain.newKeys as i32 & 3 != 0 {
            PrepareToCloseBerryTagScreen(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn TryChangeDisplayedBerry(taskId: u8, toMove: i8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut currPocketPosition: i16 =
        gBagPosition.scrollPosition[3] as i16 + gBagPosition.cursorPosition[3] as i16;
    let mut newPocketPosition: u32 = currPocketPosition as u32 + toMove as u32;
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
        gTasks[taskId].func = Some(Task_DisplayAnotherBerry);
    }
}
pub(crate) unsafe extern "C" fn HandleBagCursorPositionChange(toMove: i8) {
    let mut scrollPos: *mut u16 = &raw mut gBagPosition.scrollPosition[3];
    let mut cursorPos: *mut u16 = &raw mut gBagPosition.cursorPosition[3];
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
pub(crate) unsafe extern "C" fn Task_DisplayAnotherBerry(taskId: u8) {
    let mut i: u16 = 0;
    let mut y: i16 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
    i = 0;
    while i < FLAVOR_COUNT as u16 {
        gSprites[(*sBerryTag).flavorCircleIds[i]].y2 = y;
        i += 1;
    }
    ChangeBgY(1, 0x1000, *data.at(1) as u8);
    ChangeBgY(2, 0x1000, *data.at(1) as u8);
    if *data == 0 {
        gTasks[taskId].func = Some(Task_HandleInput);
    }
}
