//! Translated from `src/menu.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect, GetBgAttribute,
    IsDma3ManagerBusyWithBgCopy,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::dma3_manager::CheckForSpaceForDma3Request;
use crate::event_data::{FlagGet, IsNationalPokedexEnabled};
use crate::fieldmap::gMapHeader;
use crate::load_save::gSaveBlock2Ptr;
use crate::menu_helpers::{
    DisplayMessageAndContinueTask, GetLRKeysPressed, GetLRKeysPressedAndHeld,
};
use crate::palette::LoadPalette;
use crate::pokedex::{GetHoennPokedexCount, GetNationalPokedexCount};
use crate::pokemon_icon::{GetMonIconPtr, GetValidMonIconPalettePtr};
use crate::region_map::GetMapNameGeneric;
use crate::sound::PlaySE;
use crate::string_util::gStringVar4;
use crate::task::{DestroyTask, GetWordTaskArg, SetWordTaskArg};
use crate::task::{task_get, task_set};
use crate::text::{
    DeactivateAllTextPrinters, GetFontAttribute, GetMenuCursorDimensionByFont, IsTextPrinterActive,
    RunTextPrinters,
};
use crate::text_window::{LoadMessageBoxGfx, LoadUserWindowBorderGfx};
#[allow(unused_imports)]
use crate::types::*;
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
/// `BlitBitmapRectToWindow` with this module's view of its types.
#[inline]
unsafe fn BlitBitmapRectToWindow(
    a0: u8,
    a1: *mut u8,
    a2: u16,
    a3: u16,
    a4: u16,
    a5: i32,
    a6: u16,
    a7: u16,
    a8: u16,
    a9: u16,
) {
    unsafe {
        crate::window::BlitBitmapRectToWindow(a0, a1 as _, a2, a3, a4, a5, a6, a7, a8, a9);
    }
}
/// `BlitBitmapToWindow` with this module's view of its types.
#[inline]
unsafe fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16) {
    unsafe {
        crate::window::BlitBitmapToWindow(a0, a1 as _, a2, a3, a4, a5);
    }
}
/// `CallWindowFunction` with this module's view of its types.
#[inline]
unsafe fn CallWindowFunction(a0: u8, a1: Option<unsafe fn(u8, u8, u8, u8, u8, u8)>) {
    unsafe {
        crate::window::CallWindowFunction(a0, core::mem::transmute(a1));
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
/// `GetStringWidth` with this module's view of its types.
#[inline]
unsafe fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32 {
    unsafe { crate::text::GetStringWidth(a0, a1 as _, a2) }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `LoadBgTilemap` with this module's view of its types.
#[inline]
unsafe fn LoadBgTilemap(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTilemap(a0, a1 as _, a2, a3) }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `RequestDma3Fill` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Fill(a0, a1 as _, a2, a3) }
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
// Data tables (translate with cdata.py): gStandardMenuPalette sTextSpeedFrameDelays sStandardTextBox_WindowTemplates sYesNo_WindowTemplates sHofPC_TopBar_Pal sTextColors sMenuInfoIcons

/// `struct Menu`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct Menu {
    pub left: u8,
    pub top: u8,
    pub cursorPos: i8,
    pub minCursorPos: i8,
    pub maxCursorPos: i8,
    pub windowId: u8,
    pub fontId: u8,
    pub optionWidth: u8,
    pub optionHeight: u8,
    pub columns: u8,
    pub rows: u8,
    pub APressMuted: u8,
}

unsafe impl Sync for Menu {}

/// `struct MenuInfoIcon`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct MenuInfoIcon {
    pub width: u8,
    pub height: u8,
    pub offset: u16,
}

unsafe impl Sync for MenuInfoIcon {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Menu>() == 12);
    assert!(offset_of!(Menu, left) == 0);
    assert!(offset_of!(Menu, top) == 1);
    assert!(offset_of!(Menu, cursorPos) == 2);
    assert!(offset_of!(Menu, minCursorPos) == 3);
    assert!(offset_of!(Menu, maxCursorPos) == 4);
    assert!(offset_of!(Menu, windowId) == 5);
    assert!(offset_of!(Menu, fontId) == 6);
    assert!(offset_of!(Menu, optionWidth) == 7);
    assert!(offset_of!(Menu, optionHeight) == 8);
    assert!(offset_of!(Menu, columns) == 9);
    assert!(offset_of!(Menu, rows) == 10);
    assert!(offset_of!(Menu, APressMuted) == 11);
    assert!(size_of::<MenuInfoIcon>() == 4);
    assert!(offset_of!(MenuInfoIcon, width) == 0);
    assert!(offset_of!(MenuInfoIcon, height) == 1);
    assert!(offset_of!(MenuInfoIcon, offset) == 2);
};

const DLG_WINDOW_BASE_TILE_NUM: u16 = 512;
const DLG_WINDOW_PALETTE_NUM: u8 = 15;
const STD_WINDOW_BASE_TILE_NUM: u16 = 532;
const STD_WINDOW_PALETTE_NUM: u8 = 14;

static gStandardMenuPalette: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::menu::gStandardMenuPalette).cast());
static sHofPC_TopBar_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::menu::sHofPC_TopBar_Pal).cast());
static sMenuInfoIcons: Table<CArray<MenuInfoIcon, 26>> =
    Table((&raw const crate::data::menu::sMenuInfoIcons).cast());
static sStandardTextBox_WindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::menu::sStandardTextBox_WindowTemplates).cast());
static sTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::menu::sTextColors).cast());
static sTextSpeedFrameDelays: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::menu::sTextSpeedFrameDelays).cast());
static sYesNo_WindowTemplates: Table<WindowTemplate> =
    Table((&raw const crate::data::menu::sYesNo_WindowTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sStartMenuWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sMapNamePopupWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenu: Menu = unsafe { zeroed() };
#[unsafe(link_section = "ewram_data")]
pub(crate) static sTileNum: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sPaletteNum: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sYesNoWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sHofPCTopBarWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScheduledBgCopiesToVram: Aligned<CArray<u8, 4>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static sTempTileDataBufferIdx: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTempTileDataBuffer: CArray<*mut c_void, 32> = unsafe { zeroed() };

/// `AddTextPrinter` with this module's view of its types.
#[inline]
unsafe fn AddTextPrinter(
    a0: *mut TextPrinterTemplate,
    a1: u8,
    a2: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) -> u16 {
    unsafe { crate::text::AddTextPrinter(a0 as _, a1, core::mem::transmute(a2)) }
}
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
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `LZ77UnCompWram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompWram(a0 as _, a1 as _);
    }
}

pub unsafe fn InitStandardTextBoxWindows() {
    InitWindows(sStandardTextBox_WindowTemplates.as_ptr().cast_mut());
    sStartMenuWindowId.set(WINDOW_NONE);
    sMapNamePopupWindowId.set(WINDOW_NONE);
}
pub unsafe fn FreeAllOverworldWindowBuffers() {
    FreeAllWindowBuffers();
}
pub unsafe fn InitTextBoxGfxAndPrinters() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    DeactivateAllTextPrinters();
    LoadMessageBoxAndBorderGfx();
}
#[unsafe(no_mangle)]
pub unsafe fn RunTextPrintersAndIsPrinter0Active() -> u16 {
    RunTextPrinters();
    IsTextPrinterActive(0)
}
pub unsafe fn AddTextPrinterParameterized2(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    speed: u8,
    callback: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
    fgColor: u8,
    bgColor: u8,
    shadowColor: u8,
) -> u16 {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.currentChar = str;
    printer.windowId = windowId;
    printer.fontId = fontId;
    printer.x = 0;
    printer.y = 1;
    printer.currentX = 0;
    printer.currentY = 1;
    printer.letterSpacing = 0;
    printer.lineSpacing = 0;
    printer.set_unk(0);
    printer.set_fgColor(fgColor);
    printer.set_bgColor(bgColor);
    printer.set_shadowColor(shadowColor);
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_useAlternateDownArrow(0);
    AddTextPrinter(&raw mut printer, speed, callback)
}
#[unsafe(no_mangle)]
pub unsafe fn AddTextPrinterForMessage(allowSkippingDelayWithButtonPress: u8) {
    let callback: Option<unsafe fn(*mut TextPrinterTemplate, u16)> = None;
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(allowSkippingDelayWithButtonPress);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        GetPlayerTextSpeedDelay(),
        callback,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
pub unsafe fn AddTextPrinterForMessage_2(allowSkippingDelayWithButtonPress: u8) {
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(allowSkippingDelayWithButtonPress);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        GetPlayerTextSpeedDelay(),
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
pub unsafe fn AddTextPrinterWithCustomSpeedForMessage(
    allowSkippingDelayWithButtonPress: u8,
    speed: u8,
) {
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(allowSkippingDelayWithButtonPress);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        speed,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn LoadMessageBoxAndBorderGfx() {
    LoadMessageBoxGfx(0, DLG_WINDOW_BASE_TILE_NUM, 240);
    LoadUserWindowBorderGfx(0, STD_WINDOW_BASE_TILE_NUM, 224);
}
#[unsafe(no_mangle)]
pub unsafe fn DrawDialogueFrame(windowId: u8, copyToVram: u8) {
    CallWindowFunction(windowId, Some(WindowFunc_DrawDialogueFrame));
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub unsafe fn DrawStdWindowFrame(windowId: u8, copyToVram: u8) {
    CallWindowFunction(windowId, Some(WindowFunc_DrawStandardFrame));
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ClearDialogWindowAndFrame(windowId: u8, copyToVram: u8) {
    CallWindowFunction(windowId, Some(WindowFunc_ClearDialogWindowAndFrame));
    FillWindowPixelBuffer(windowId, 17);
    ClearWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ClearStdWindowAndFrame(windowId: u8, copyToVram: u8) {
    CallWindowFunction(windowId, Some(WindowFunc_ClearStdWindowAndFrame));
    FillWindowPixelBuffer(windowId, 17);
    ClearWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe fn WindowFunc_DrawStandardFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        STD_WINDOW_BASE_TILE_NUM,
        tilemapLeft - 1,
        tilemapTop - 1,
        1,
        1,
        STD_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        533,
        tilemapLeft,
        tilemapTop - 1,
        width,
        1,
        STD_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        534,
        tilemapLeft + width,
        tilemapTop - 1,
        1,
        1,
        STD_WINDOW_PALETTE_NUM,
    );
    for i in (tilemapTop as i32)..(tilemapTop as i32 + height as i32) {
        FillBgTilemapBufferRect(
            bg,
            535,
            tilemapLeft - 1,
            i as u8,
            1,
            1,
            STD_WINDOW_PALETTE_NUM,
        );
        FillBgTilemapBufferRect(
            bg,
            537,
            tilemapLeft + width,
            i as u8,
            1,
            1,
            STD_WINDOW_PALETTE_NUM,
        );
    }
    FillBgTilemapBufferRect(
        bg,
        538,
        tilemapLeft - 1,
        tilemapTop + height,
        1,
        1,
        STD_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        539,
        tilemapLeft,
        tilemapTop + height,
        width,
        1,
        STD_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        540,
        tilemapLeft + width,
        tilemapTop + height,
        1,
        1,
        STD_WINDOW_PALETTE_NUM,
    );
}
pub(crate) unsafe fn WindowFunc_DrawDialogueFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        513,
        tilemapLeft - 2,
        tilemapTop - 1,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        515,
        tilemapLeft - 1,
        tilemapTop - 1,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        516,
        tilemapLeft,
        tilemapTop - 1,
        width - 1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        517,
        tilemapLeft + width - 1,
        tilemapTop - 1,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        518,
        tilemapLeft + width,
        tilemapTop - 1,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        519,
        tilemapLeft - 2,
        tilemapTop,
        1,
        5,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        521,
        tilemapLeft - 1,
        tilemapTop,
        width + 1,
        5,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        522,
        tilemapLeft + width,
        tilemapTop,
        1,
        5,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        2561,
        tilemapLeft - 2,
        tilemapTop + height,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        2563,
        tilemapLeft - 1,
        tilemapTop + height,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        2564,
        tilemapLeft,
        tilemapTop + height,
        width - 1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        2565,
        tilemapLeft + width - 1,
        tilemapTop + height,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
    FillBgTilemapBufferRect(
        bg,
        2566,
        tilemapLeft + width,
        tilemapTop + height,
        1,
        1,
        DLG_WINDOW_PALETTE_NUM,
    );
}
pub(crate) unsafe fn WindowFunc_ClearStdWindowAndFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        0,
        tilemapLeft - 1,
        tilemapTop - 1,
        width + 2,
        height + 2,
        STD_WINDOW_PALETTE_NUM,
    );
}
pub(crate) unsafe fn WindowFunc_ClearDialogWindowAndFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        0,
        tilemapLeft - 3,
        tilemapTop - 1,
        width + 6,
        height + 2,
        STD_WINDOW_PALETTE_NUM,
    );
}
pub unsafe fn SetStandardWindowBorderStyle(windowId: u8, copyToVram: u8) {
    DrawStdFrameWithCustomTileAndPalette(
        windowId,
        copyToVram,
        STD_WINDOW_BASE_TILE_NUM,
        STD_WINDOW_PALETTE_NUM,
    );
}
pub unsafe fn LoadMessageBoxAndFrameGfx(windowId: u8, copyToVram: u8) {
    LoadMessageBoxGfx(windowId, DLG_WINDOW_BASE_TILE_NUM, 240);
    DrawDialogFrameWithCustomTileAndPalette(
        windowId,
        copyToVram,
        DLG_WINDOW_BASE_TILE_NUM,
        DLG_WINDOW_PALETTE_NUM,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn Menu_LoadStdPal() {
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        224,
        20,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn Menu_LoadStdPalAt(offset: u16) {
    LoadPalette(
        gStandardMenuPalette.as_ptr().cast_mut() as *mut c_void,
        offset,
        20,
    );
}
fn Menu_GetStdPal() -> *mut u16 {
    gStandardMenuPalette.as_ptr().cast_mut()
}
fn Menu_GetStdPalColor(mut colorNum: u8) -> u16 {
    if colorNum > 15 {
        colorNum = 0;
    }
    gStandardMenuPalette[colorNum]
}
pub unsafe fn DisplayItemMessageOnField(
    taskId: u8,
    string: *mut u8,
    callback: Option<unsafe fn(u8)>,
) {
    LoadMessageBoxAndBorderGfx();
    DisplayMessageAndContinueTask(
        taskId,
        0,
        DLG_WINDOW_BASE_TILE_NUM,
        DLG_WINDOW_PALETTE_NUM,
        FONT_NORMAL,
        GetPlayerTextSpeedDelay(),
        string,
        core::mem::transmute::<Option<unsafe fn(u8)>, *mut c_void>(callback),
    );
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub unsafe fn DisplayYesNoMenuDefaultYes() {
    CreateYesNoMenu(
        (&raw const *sYesNo_WindowTemplates).cast_mut(),
        STD_WINDOW_BASE_TILE_NUM,
        STD_WINDOW_PALETTE_NUM,
        0,
    );
}
pub unsafe fn DisplayYesNoMenuWithDefault(initialCursorPos: u8) {
    CreateYesNoMenu(
        (&raw const *sYesNo_WindowTemplates).cast_mut(),
        STD_WINDOW_BASE_TILE_NUM,
        STD_WINDOW_PALETTE_NUM,
        initialCursorPos,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerTextSpeed() -> u32 {
    if (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .forceMidTextSpeed()
        != 0
    {
        return OPTIONS_TEXT_SPEED_MID as u32;
    }
    (*gSaveBlock2Ptr).optionsTextSpeed() as u32
}
pub unsafe fn GetPlayerTextSpeedDelay() -> u8 {
    if (*gSaveBlock2Ptr).optionsTextSpeed() > OPTIONS_TEXT_SPEED_FAST {
        (*gSaveBlock2Ptr).set_optionsTextSpeed(OPTIONS_TEXT_SPEED_MID);
    }
    let speed: u32 = GetPlayerTextSpeed();
    sTextSpeedFrameDelays[speed]
}
pub unsafe fn AddStartMenuWindow(numActions: u8) -> u8 {
    if sStartMenuWindowId.get() == WINDOW_NONE {
        sStartMenuWindowId
            .set(AddWindowParameterized(0, 22, 1, 7, numActions * 2 + 2, 15, 0x139) as u8);
    }
    sStartMenuWindowId.get()
}
pub unsafe fn GetStartMenuWindowId() -> u8 {
    sStartMenuWindowId.get()
}
pub unsafe fn RemoveStartMenuWindow() {
    if sStartMenuWindowId.get() != WINDOW_NONE {
        RemoveWindow(sStartMenuWindowId.get());
        sStartMenuWindowId.set(WINDOW_NONE);
    }
}
fn GetDialogFrameBaseTileNum() -> u16 {
    DLG_WINDOW_BASE_TILE_NUM
}
fn GetStandardFrameBaseTileNum() -> u16 {
    STD_WINDOW_BASE_TILE_NUM
}
#[unsafe(no_mangle)]
pub unsafe fn AddMapNamePopUpWindow() -> u8 {
    if sMapNamePopupWindowId.get() == WINDOW_NONE {
        sMapNamePopupWindowId.set(AddWindowParameterized(0, 1, 1, 10, 3, 14, 0x107) as u8);
    }
    sMapNamePopupWindowId.get()
}
#[unsafe(no_mangle)]
pub fn GetMapNamePopUpWindowId() -> u8 {
    sMapNamePopupWindowId.get()
}
#[unsafe(no_mangle)]
pub unsafe fn RemoveMapNamePopUpWindow() {
    if sMapNamePopupWindowId.get() != WINDOW_NONE {
        RemoveWindow(sMapNamePopupWindowId.get());
        sMapNamePopupWindowId.set(WINDOW_NONE);
    }
}
pub unsafe fn AddTextPrinterWithCallbackForMessage(
    canSpeedUp: u8,
    callback: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
) {
    (*(&raw const crate::text::gTextFlags)
        .cast::<TextFlags>()
        .cast_mut())
    .set_canABSpeedUpPrint(canSpeedUp);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        GetPlayerTextSpeedDelay(),
        callback,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
pub unsafe fn EraseFieldMessageBox(copyToVram: u8) {
    FillBgTilemapBufferRect(0, 0, 0, 0, 32, 32, 17);
    if copyToVram == TRUE {
        CopyBgTilemapBufferToVram(0);
    }
}
pub unsafe fn DrawDialogFrameWithCustomTileAndPalette(
    windowId: u8,
    copyToVram: u8,
    tileNum: u16,
    paletteNum: u8,
) {
    sTileNum.set(tileNum);
    sPaletteNum.set(paletteNum);
    CallWindowFunction(
        windowId,
        Some(WindowFunc_DrawDialogFrameWithCustomTileAndPalette),
    );
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
unsafe fn DrawDialogFrameWithCustomTile(windowId: u8, copyToVram: u8, tileNum: u16) {
    sTileNum.set(tileNum);
    sPaletteNum.set(GetWindowAttribute(windowId, WINDOW_PALETTE_NUM) as u8);
    CallWindowFunction(
        windowId,
        Some(WindowFunc_DrawDialogFrameWithCustomTileAndPalette),
    );
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe fn WindowFunc_DrawDialogFrameWithCustomTileAndPalette(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 1,
        tilemapLeft - 2,
        tilemapTop - 1,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 3,
        tilemapLeft - 1,
        tilemapTop - 1,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 4,
        tilemapLeft,
        tilemapTop - 1,
        width - 1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 5,
        tilemapLeft + width - 1,
        tilemapTop - 1,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 6,
        tilemapLeft + width,
        tilemapTop - 1,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 7,
        tilemapLeft - 2,
        tilemapTop,
        1,
        5,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 9,
        tilemapLeft - 1,
        tilemapTop,
        width + 1,
        5,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 10,
        tilemapLeft + width,
        tilemapTop,
        1,
        5,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        0x800 + (sTileNum.get() + 1),
        tilemapLeft - 2,
        tilemapTop + height,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        0x800 + (sTileNum.get() + 3),
        tilemapLeft - 1,
        tilemapTop + height,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        0x800 + (sTileNum.get() + 4),
        tilemapLeft,
        tilemapTop + height,
        width - 1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        0x800 + (sTileNum.get() + 5),
        tilemapLeft + width - 1,
        tilemapTop + height,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        0x800 + (sTileNum.get() + 6),
        tilemapLeft + width,
        tilemapTop + height,
        1,
        1,
        sPaletteNum.get(),
    );
}
pub unsafe fn ClearDialogWindowAndFrameToTransparent(windowId: u8, copyToVram: u8) {
    CallWindowFunction(
        windowId,
        Some(WindowFunc_ClearDialogWindowAndFrameNullPalette),
    );
    FillWindowPixelBuffer(windowId, 0);
    ClearWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe fn WindowFunc_ClearDialogWindowAndFrameNullPalette(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        0,
        tilemapLeft - 3,
        tilemapTop - 1,
        width + 6,
        height + 2,
        0,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn DrawStdFrameWithCustomTileAndPalette(
    windowId: u8,
    copyToVram: u8,
    baseTileNum: u16,
    paletteNum: u8,
) {
    sTileNum.set(baseTileNum);
    sPaletteNum.set(paletteNum);
    CallWindowFunction(
        windowId,
        Some(WindowFunc_DrawStdFrameWithCustomTileAndPalette),
    );
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub unsafe fn DrawStdFrameWithCustomTile(windowId: u8, copyToVram: u8, baseTileNum: u16) {
    sTileNum.set(baseTileNum);
    sPaletteNum.set(GetWindowAttribute(windowId, WINDOW_PALETTE_NUM) as u8);
    CallWindowFunction(
        windowId,
        Some(WindowFunc_DrawStdFrameWithCustomTileAndPalette),
    );
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe fn WindowFunc_DrawStdFrameWithCustomTileAndPalette(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get(),
        tilemapLeft - 1,
        tilemapTop - 1,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 1,
        tilemapLeft,
        tilemapTop - 1,
        width,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 2,
        tilemapLeft + width,
        tilemapTop - 1,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 3,
        tilemapLeft - 1,
        tilemapTop,
        1,
        height,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 5,
        tilemapLeft + width,
        tilemapTop,
        1,
        height,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 6,
        tilemapLeft - 1,
        tilemapTop + height,
        1,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 7,
        tilemapLeft,
        tilemapTop + height,
        width,
        1,
        sPaletteNum.get(),
    );
    FillBgTilemapBufferRect(
        bg,
        sTileNum.get() + 8,
        tilemapLeft + width,
        tilemapTop + height,
        1,
        1,
        sPaletteNum.get(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ClearStdWindowAndFrameToTransparent(windowId: u8, copyToVram: u8) {
    CallWindowFunction(
        windowId,
        Some(WindowFunc_ClearStdWindowAndFrameToTransparent),
    );
    FillWindowPixelBuffer(windowId, 0);
    ClearWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe fn WindowFunc_ClearStdWindowAndFrameToTransparent(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    FillBgTilemapBufferRect(
        bg,
        0,
        tilemapLeft - 1,
        tilemapTop - 1,
        width + 2,
        height + 2,
        0,
    );
}
pub unsafe fn HofPCTopBar_AddWindow(
    bg: u8,
    xPos: u8,
    yPos: u8,
    mut palette: u8,
    baseTile: u16,
) -> u8 {
    let mut window: WindowTemplate = zeroed();
    memset(&raw mut window as *mut u8, 0, 8);
    if bg > 3 {
        window.bg = 0;
    } else {
        window.bg = bg;
    }
    window.tilemapTop = yPos;
    window.height = 2;
    window.tilemapLeft = 30 - xPos;
    window.width = xPos;
    window.paletteNum = palette;
    window.baseBlock = baseTile;
    sHofPCTopBarWindowId.set(AddWindow(&raw mut window) as u8);
    if palette > 15 {
        palette = 240;
    } else {
        palette *= 16;
    }
    LoadPalette(
        sHofPC_TopBar_Pal.as_ptr().cast_mut() as *mut c_void,
        palette as u16,
        32,
    );
    sHofPCTopBarWindowId.get()
}
pub unsafe fn HofPCTopBar_Print(string: *mut u8, left: u8, copyToVram: u8) {
    let mut width: u16 = 0;
    if sHofPCTopBarWindowId.get() != WINDOW_NONE {
        PutWindowTilemap(sHofPCTopBarWindowId.get());
        FillWindowPixelBuffer(sHofPCTopBarWindowId.get(), 255);
        width = GetStringWidth(FONT_SMALL, string, 0) as u16;
        AddTextPrinterParameterized3(
            sHofPCTopBarWindowId.get(),
            FONT_SMALL,
            236 - GetWindowAttribute(sHofPCTopBarWindowId.get(), WINDOW_TILEMAP_LEFT) as u8 * 8
                - left
                - width as u8,
            1,
            sTextColors.as_ptr().cast_mut(),
            0,
            string,
        );
        if copyToVram != 0 {
            CopyWindowToVram(sHofPCTopBarWindowId.get(), COPYWIN_FULL);
        }
    }
}
pub unsafe fn HofPCTopBar_PrintPair(
    string: *mut u8,
    string2: *mut u8,
    noBg: u8,
    left: u8,
    copyToVram: u8,
) {
    let mut color: CArray<u8, 3> = zeroed();
    let mut width: u16 = 0;
    if sHofPCTopBarWindowId.get() != WINDOW_NONE {
        if noBg != 0 {
            color[0] = 0x0;
            color[1] = 0x1;
            color[2] = 0x2;
        } else {
            color[0] = TEXT_DYNAMIC_COLOR_6;
            color[1] = 0x1;
            color[2] = 0x2;
        }
        PutWindowTilemap(sHofPCTopBarWindowId.get());
        FillWindowPixelBuffer(sHofPCTopBarWindowId.get(), 255);
        if !string2.is_null() {
            width = GetStringWidth(FONT_SMALL, string2, 0) as u16;
            AddTextPrinterParameterized3(
                sHofPCTopBarWindowId.get(),
                FONT_SMALL,
                236 - GetWindowAttribute(sHofPCTopBarWindowId.get(), WINDOW_TILEMAP_LEFT) as u8 * 8
                    - left
                    - width as u8,
                1,
                color.as_mut_ptr(),
                0,
                string2,
            );
        }
        AddTextPrinterParameterized4(
            sHofPCTopBarWindowId.get(),
            FONT_NORMAL,
            4,
            1,
            0,
            0,
            color.as_mut_ptr(),
            0,
            string,
        );
        if copyToVram != 0 {
            CopyWindowToVram(sHofPCTopBarWindowId.get(), COPYWIN_FULL);
        }
    }
}
unsafe fn HofPCTopBar_CopyToVram() {
    if sHofPCTopBarWindowId.get() != WINDOW_NONE {
        CopyWindowToVram(sHofPCTopBarWindowId.get(), COPYWIN_FULL);
    }
}
unsafe fn HofPCTopBar_Clear() {
    if sHofPCTopBarWindowId.get() != WINDOW_NONE {
        FillWindowPixelBuffer(sHofPCTopBarWindowId.get(), 255);
        CopyWindowToVram(sHofPCTopBarWindowId.get(), COPYWIN_FULL);
    }
}
pub unsafe fn HofPCTopBar_RemoveWindow() {
    if sHofPCTopBarWindowId.get() != WINDOW_NONE {
        FillWindowPixelBuffer(sHofPCTopBarWindowId.get(), 0);
        ClearWindowTilemap(sHofPCTopBarWindowId.get());
        CopyWindowToVram(sHofPCTopBarWindowId.get(), COPYWIN_FULL);
        RemoveWindow(sHofPCTopBarWindowId.get());
        sHofPCTopBarWindowId.set(WINDOW_NONE);
    }
}
pub(crate) unsafe fn InitMenu(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    cursorHeight: u8,
    numChoices: u8,
    initialCursorPos: u8,
    muteAPress: u8,
) -> u8 {
    sMenu.left = left;
    sMenu.top = top;
    sMenu.minCursorPos = 0;
    sMenu.maxCursorPos = numChoices as i8 - 1;
    sMenu.windowId = windowId;
    sMenu.fontId = fontId;
    sMenu.optionHeight = cursorHeight;
    sMenu.APressMuted = muteAPress;
    let pos: i32 = initialCursorPos as i32;
    if pos < 0 || pos > sMenu.maxCursorPos as i32 {
        sMenu.cursorPos = 0;
    } else {
        sMenu.cursorPos = pos as i8;
    }
    Menu_MoveCursor(0);
    sMenu.cursorPos as u8
}
pub unsafe fn InitMenuNormal(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    cursorHeight: u8,
    numChoices: u8,
    initialCursorPos: u8,
) -> u8 {
    InitMenu(
        windowId,
        fontId,
        left,
        top,
        cursorHeight,
        numChoices,
        initialCursorPos,
        FALSE,
    )
}
unsafe fn InitMenuDefaultCursorHeight(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    numChoices: u8,
    initialCursorPos: u8,
) -> u8 {
    let cursorHeight: u8 = GetMenuCursorDimensionByFont(fontId, 1);
    InitMenuNormal(
        windowId,
        fontId,
        left,
        top,
        cursorHeight,
        numChoices,
        initialCursorPos,
    )
}
pub unsafe fn RedrawMenuCursor(oldPos: u8, newPos: u8) {
    let width: u8 = GetMenuCursorDimensionByFont(sMenu.fontId, 0);
    let height: u8 = GetMenuCursorDimensionByFont(sMenu.fontId, 1);
    FillWindowPixelRect(
        sMenu.windowId,
        17,
        sMenu.left as u16,
        sMenu.optionHeight as u16 * oldPos as u16 + sMenu.top as u16,
        width as u16,
        height as u16,
    );
    AddTextPrinterParameterized(
        sMenu.windowId,
        sMenu.fontId,
        (*(&raw const crate::data::strings::gText_SelectorArrow3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        sMenu.left,
        sMenu.optionHeight * newPos + sMenu.top,
        0,
        None,
    );
}
pub unsafe fn Menu_MoveCursor(cursorDelta: i8) -> u8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    let newPos: i32 = sMenu.cursorPos as i32 + cursorDelta as i32;
    if newPos < sMenu.minCursorPos as i32 {
        sMenu.cursorPos = sMenu.maxCursorPos;
    } else if newPos > sMenu.maxCursorPos as i32 {
        sMenu.cursorPos = sMenu.minCursorPos;
    } else {
        sMenu.cursorPos += cursorDelta;
    }
    RedrawMenuCursor(oldPos, sMenu.cursorPos as u8);
    sMenu.cursorPos as u8
}
pub unsafe fn Menu_MoveCursorNoWrapAround(cursorDelta: i8) -> u8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    let newPos: i32 = sMenu.cursorPos as i32 + cursorDelta as i32;
    if newPos < sMenu.minCursorPos as i32 {
        sMenu.cursorPos = sMenu.minCursorPos;
    } else if newPos > sMenu.maxCursorPos as i32 {
        sMenu.cursorPos = sMenu.maxCursorPos;
    } else {
        sMenu.cursorPos += cursorDelta;
    }
    RedrawMenuCursor(oldPos, sMenu.cursorPos as u8);
    sMenu.cursorPos as u8
}
pub unsafe fn Menu_GetCursorPos() -> u8 {
    sMenu.cursorPos as u8
}
#[unsafe(no_mangle)]
pub unsafe fn Menu_ProcessInput() -> i8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if sMenu.APressMuted == 0 {
            PlaySE(SE_SELECT);
        }
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        PlaySE(SE_SELECT);
        Menu_MoveCursor(-1);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        PlaySE(SE_SELECT);
        Menu_MoveCursor(1);
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
pub unsafe fn Menu_ProcessInputNoWrap() -> i8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if sMenu.APressMuted == 0 {
            PlaySE(SE_SELECT);
        }
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if oldPos != Menu_MoveCursorNoWrapAround(-1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if oldPos != Menu_MoveCursorNoWrapAround(1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
pub unsafe fn ProcessMenuInput_other() -> i8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if sMenu.APressMuted == 0 {
            PlaySE(SE_SELECT);
        }
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_UP {
        PlaySE(SE_SELECT);
        Menu_MoveCursor(-1);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_DOWN {
        PlaySE(SE_SELECT);
        Menu_MoveCursor(1);
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
pub unsafe fn Menu_ProcessInputNoWrapAround_other() -> i8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if sMenu.APressMuted == 0 {
            PlaySE(SE_SELECT);
        }
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_UP {
        if oldPos != Menu_MoveCursorNoWrapAround(-1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_DOWN {
        if oldPos != Menu_MoveCursorNoWrapAround(1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
pub unsafe fn PrintMenuActionTextsAtPos(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut MenuAction,
) {
    for i in 0..itemCount {
        AddTextPrinterParameterized(
            windowId,
            fontId,
            (*menuActions.at(i)).text,
            left,
            lineHeight * i + top,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn PrintMenuActionTextsWithSpacing(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut MenuAction,
    letterSpacing: u8,
    lineSpacing: u8,
) {
    for i in 0..itemCount {
        AddTextPrinterParameterized5(
            windowId,
            fontId,
            (*menuActions.at(i)).text,
            left,
            lineHeight * i + top,
            TEXT_SKIP_DRAW,
            None,
            letterSpacing,
            lineSpacing,
        );
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn PrintMenuActionTextsAtTop(
    windowId: u8,
    fontId: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut MenuAction,
) {
    PrintMenuActionTextsAtPos(
        windowId,
        fontId,
        GetFontAttribute(fontId, FONTATTR_MAX_LETTER_WIDTH),
        1,
        lineHeight,
        itemCount,
        menuActions,
    );
}
pub unsafe fn PrintMenuActionTexts(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    letterSpacing: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut MenuAction,
    actionIds: *mut u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.windowId = windowId;
    printer.fontId = fontId;
    printer.set_fgColor(GetFontAttribute(fontId, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(fontId, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(fontId, FONTATTR_COLOR_SHADOW));
    printer.set_unk(GetFontAttribute(fontId, FONTATTR_UNKNOWN));
    printer.letterSpacing = letterSpacing;
    printer.lineSpacing = GetFontAttribute(fontId, FONTATTR_LINE_SPACING);
    printer.x = left;
    printer.currentX = left;
    for i in 0..itemCount {
        printer.currentChar = (*menuActions.at(*actionIds.at(i))).text;
        printer.y = lineHeight * i + top;
        printer.currentY = printer.y;
        AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn PrintMenuActionTextsAtTopById(
    windowId: u8,
    fontId: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut MenuAction,
    actionIds: *mut u8,
) {
    PrintMenuActionTexts(
        windowId,
        fontId,
        GetFontAttribute(fontId, FONTATTR_MAX_LETTER_WIDTH),
        1,
        GetFontAttribute(fontId, FONTATTR_LETTER_SPACING),
        lineHeight,
        itemCount,
        menuActions,
        actionIds,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn SetWindowTemplateFields(
    template: *mut WindowTemplate,
    bg: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
    baseBlock: u16,
) {
    (*template).bg = bg;
    (*template).tilemapLeft = left;
    (*template).tilemapTop = top;
    (*template).width = width;
    (*template).height = height;
    (*template).paletteNum = paletteNum;
    (*template).baseBlock = baseBlock;
}
pub unsafe fn CreateWindowTemplate(
    bg: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
    baseBlock: u16,
) -> WindowTemplate {
    let mut template: WindowTemplate = zeroed();
    SetWindowTemplateFields(
        &raw mut template,
        bg,
        left,
        top,
        width,
        height,
        paletteNum,
        baseBlock,
    );
    template
}
pub unsafe fn AddWindowParameterized(
    bg: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
    baseBlock: u16,
) -> u16 {
    let mut template: WindowTemplate = zeroed();
    SetWindowTemplateFields(
        &raw mut template,
        bg,
        left,
        top,
        width,
        height,
        paletteNum,
        baseBlock,
    );
    AddWindow(&raw mut template)
}
unsafe fn CreateYesNoMenuAtPos(
    window: *mut WindowTemplate,
    fontId: u8,
    left: u8,
    top: u8,
    baseTileNum: u16,
    paletteNum: u8,
    initialCursorPos: u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    sYesNoWindowId.set(AddWindow(window) as u8);
    DrawStdFrameWithCustomTileAndPalette(sYesNoWindowId.get(), TRUE, baseTileNum, paletteNum);
    printer.currentChar = (*(&raw const crate::data::strings::gText_YesNo).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    printer.windowId = sYesNoWindowId.get();
    printer.fontId = fontId;
    printer.x = GetFontAttribute(fontId, FONTATTR_MAX_LETTER_WIDTH) + left;
    printer.y = top;
    printer.currentX = printer.x;
    printer.currentY = printer.y;
    printer.set_fgColor(GetFontAttribute(fontId, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(fontId, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(fontId, FONTATTR_COLOR_SHADOW));
    printer.set_unk(GetFontAttribute(fontId, FONTATTR_UNKNOWN));
    printer.letterSpacing = GetFontAttribute(fontId, FONTATTR_LETTER_SPACING);
    printer.lineSpacing = GetFontAttribute(fontId, FONTATTR_LINE_SPACING);
    AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
    InitMenuNormal(
        sYesNoWindowId.get(),
        fontId,
        left,
        top,
        GetFontAttribute(fontId, FONTATTR_MAX_LETTER_HEIGHT),
        2,
        initialCursorPos,
    );
}
unsafe fn CreateYesNoMenuInTopLeft(
    window: *mut WindowTemplate,
    fontId: u8,
    baseTileNum: u16,
    paletteNum: u8,
) {
    CreateYesNoMenuAtPos(window, fontId, 0, 1, baseTileNum, paletteNum, 0);
}
#[unsafe(no_mangle)]
pub unsafe fn Menu_ProcessInputNoWrapClearOnChoose() -> i8 {
    let result: i8 = Menu_ProcessInputNoWrap();
    if result != MENU_NOTHING_CHOSEN {
        EraseYesNoWindow();
    }
    result
}
pub unsafe fn EraseYesNoWindow() {
    ClearStdWindowAndFrameToTransparent(sYesNoWindowId.get(), TRUE);
    RemoveWindow(sYesNoWindowId.get());
}
unsafe fn PrintMenuActionGridText(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut MenuAction,
) {
    for i in 0..rows {
        for j in 0..columns {
            AddTextPrinterParameterized(
                windowId,
                fontId,
                (*menuActions.at(i as i32 * columns as i32 + j as i32)).text,
                width * j + left,
                height * i + top,
                TEXT_SKIP_DRAW,
                None,
            );
        }
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn PrintMenuActionGridTextAtTop(
    windowId: u8,
    fontId: u8,
    width: u8,
    height: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut MenuAction,
) {
    PrintMenuActionGridText(
        windowId,
        fontId,
        GetFontAttribute(fontId, FONTATTR_MAX_LETTER_WIDTH),
        0,
        width,
        height,
        columns,
        rows,
        menuActions,
    );
}
pub unsafe fn PrintMenuActionGrid(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    optionWidth: u8,
    horizontalCount: u8,
    verticalCount: u8,
    menuActions: *mut MenuAction,
    actionIds: *mut u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.windowId = windowId;
    printer.fontId = fontId;
    printer.set_fgColor(GetFontAttribute(fontId, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(fontId, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(fontId, FONTATTR_COLOR_SHADOW));
    printer.set_unk(GetFontAttribute(fontId, FONTATTR_UNKNOWN));
    printer.letterSpacing = GetFontAttribute(fontId, FONTATTR_LETTER_SPACING);
    printer.lineSpacing = GetFontAttribute(fontId, FONTATTR_LINE_SPACING);
    for i in 0..verticalCount {
        for j in 0..horizontalCount {
            printer.currentChar =
                (*menuActions.at(*actionIds.at(horizontalCount as i32 * i as i32 + j as i32))).text;
            printer.x = optionWidth * j + left;
            printer.y = GetFontAttribute(fontId, FONTATTR_MAX_LETTER_HEIGHT) * i + top;
            printer.currentX = printer.x;
            printer.currentY = printer.y;
            AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
        }
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn PrintMenuActionGrid_TopLeft(
    windowId: u8,
    fontId: u8,
    optionWidth: u8,
    unused: u8,
    horizontalCount: u8,
    verticalCount: u8,
    menuActions: *mut MenuAction,
    actionIds: *mut u8,
) {
    PrintMenuActionGrid(
        windowId,
        fontId,
        GetFontAttribute(fontId, FONTATTR_MAX_LETTER_WIDTH),
        0,
        optionWidth,
        horizontalCount,
        verticalCount,
        menuActions,
        actionIds,
    );
}
unsafe fn InitMenuGrid(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    optionWidth: u8,
    optionHeight: u8,
    columns: u8,
    rows: u8,
    numChoices: u8,
    cursorPos: u8,
) -> u8 {
    sMenu.left = left;
    sMenu.top = top;
    sMenu.minCursorPos = 0;
    sMenu.maxCursorPos = numChoices as i8 - 1;
    sMenu.windowId = windowId;
    sMenu.fontId = fontId;
    sMenu.optionWidth = optionWidth;
    sMenu.optionHeight = optionHeight;
    sMenu.columns = columns;
    sMenu.rows = rows;
    let pos: i32 = cursorPos as i32;
    if pos < 0 || pos > sMenu.maxCursorPos as i32 {
        sMenu.cursorPos = 0;
    } else {
        sMenu.cursorPos = pos as i8;
    }
    ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_NONE);
    sMenu.cursorPos as u8
}
unsafe fn InitMenuGridDefaultCursorHeight(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    width: u8,
    columns: u8,
    rows: u8,
    cursorPos: u8,
) -> u8 {
    let cursorHeight: u8 = GetMenuCursorDimensionByFont(fontId, 1);
    let numChoices: u8 = columns * rows;
    InitMenuGrid(
        windowId,
        fontId,
        left,
        top,
        width,
        cursorHeight,
        columns,
        rows,
        numChoices,
        cursorPos,
    )
}
unsafe fn MoveMenuGridCursor(oldCursorPos: u8, newCursorPos: u8) {
    let cursorWidth: u8 = GetMenuCursorDimensionByFont(sMenu.fontId, 0);
    let cursorHeight: u8 = GetMenuCursorDimensionByFont(sMenu.fontId, 1);
    let mut xPos: u8 =
        rem_i32(oldCursorPos as i32, sMenu.columns as i32) as u8 * sMenu.optionWidth + sMenu.left;
    let mut yPos: u8 =
        div_i32(oldCursorPos as i32, sMenu.columns as i32) as u8 * sMenu.optionHeight + sMenu.top;
    FillWindowPixelRect(
        sMenu.windowId,
        17,
        xPos as u16,
        yPos as u16,
        cursorWidth as u16,
        cursorHeight as u16,
    );
    xPos =
        rem_i32(newCursorPos as i32, sMenu.columns as i32) as u8 * sMenu.optionWidth + sMenu.left;
    yPos =
        div_i32(newCursorPos as i32, sMenu.columns as i32) as u8 * sMenu.optionHeight + sMenu.top;
    AddTextPrinterParameterized(
        sMenu.windowId,
        sMenu.fontId,
        (*(&raw const crate::data::strings::gText_SelectorArrow3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        xPos,
        yPos,
        0,
        None,
    );
}
pub unsafe fn ChangeMenuGridCursorPosition(deltaX: i8, deltaY: i8) -> u8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    if deltaX != 0 {
        if (rem_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaX as i32) < 0 {
            sMenu.cursorPos += sMenu.columns as i8 - 1;
        } else if rem_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaX as i32
            >= sMenu.columns as i32
        {
            sMenu.cursorPos =
                div_i32(sMenu.cursorPos as i32, sMenu.columns as i32) as i8 * sMenu.columns as i8;
        } else {
            sMenu.cursorPos += deltaX;
        }
    }
    if deltaY != 0 {
        if (div_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaY as i32) < 0 {
            sMenu.cursorPos += sMenu.columns as i8 * (sMenu.rows as i8 - 1);
        } else if div_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaY as i32
            >= sMenu.rows as i32
        {
            sMenu.cursorPos -= sMenu.columns as i8 * (sMenu.rows as i8 - 1);
        } else {
            sMenu.cursorPos += sMenu.columns as i8 * deltaY;
        }
    }
    if sMenu.cursorPos > sMenu.maxCursorPos {
        sMenu.cursorPos = oldPos as i8;
        return sMenu.cursorPos as u8;
    } else {
        MoveMenuGridCursor(oldPos, sMenu.cursorPos as u8);
        return sMenu.cursorPos as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ChangeGridMenuCursorPosition(deltaX: i8, deltaY: i8) -> u8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    if deltaX != 0
        && rem_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaX as i32 >= 0
        && (rem_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaX as i32)
            < sMenu.columns as i32
    {
        sMenu.cursorPos += deltaX;
    }
    if deltaY != 0
        && div_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaY as i32 >= 0
        && (div_i32(sMenu.cursorPos as i32, sMenu.columns as i32) + deltaY as i32)
            < sMenu.rows as i32
    {
        sMenu.cursorPos += sMenu.columns as i8 * deltaY;
    }
    if sMenu.cursorPos > sMenu.maxCursorPos {
        sMenu.cursorPos = oldPos as i8;
        return sMenu.cursorPos as u8;
    } else {
        MoveMenuGridCursor(oldPos, sMenu.cursorPos as u8);
        return sMenu.cursorPos as u8;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn Menu_ProcessGridInput_NoSoundLimit() -> i8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_UP);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_DOWN);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_LEFT, MENU_CURSOR_DELTA_NONE);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_RIGHT, MENU_CURSOR_DELTA_NONE);
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
pub unsafe fn Menu_ProcessGridInput() -> i8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if oldPos != ChangeGridMenuCursorPosition(0, -1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if oldPos != ChangeGridMenuCursorPosition(0, 1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_LEFT != 0 || GetLRKeysPressed() == MENU_L_PRESSED {
        if oldPos != ChangeGridMenuCursorPosition(-1, 0) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newKeys as i32 & DPAD_RIGHT != 0 || GetLRKeysPressed() == MENU_R_PRESSED {
        if oldPos != ChangeGridMenuCursorPosition(1, 0) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
unsafe fn Menu_ProcessGridInputRepeat_NoSoundLimit() -> i8 {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_UP {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_UP);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_DOWN {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_DOWN);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_LEFT
        || GetLRKeysPressedAndHeld() == MENU_L_PRESSED
    {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_LEFT, MENU_CURSOR_DELTA_NONE);
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_RIGHT
        || GetLRKeysPressedAndHeld() == MENU_R_PRESSED
    {
        PlaySE(SE_SELECT);
        ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_RIGHT, MENU_CURSOR_DELTA_NONE);
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
unsafe fn Menu_ProcessGridInputRepeat() -> i8 {
    let oldPos: u8 = sMenu.cursorPos as u8;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return sMenu.cursorPos;
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        return MENU_B_PRESSED;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_UP {
        if oldPos != ChangeGridMenuCursorPosition(0, -1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_DOWN {
        if oldPos != ChangeGridMenuCursorPosition(0, 1) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_LEFT
        || GetLRKeysPressedAndHeld() == MENU_L_PRESSED
    {
        if oldPos != ChangeGridMenuCursorPosition(-1, 0) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    } else if gMain.newAndRepeatedKeys as i32 & DPAD_ANY == DPAD_RIGHT
        || GetLRKeysPressedAndHeld() == MENU_R_PRESSED
    {
        if oldPos != ChangeGridMenuCursorPosition(1, 0) {
            PlaySE(SE_SELECT);
        }
        return MENU_NOTHING_CHOSEN;
    }
    MENU_NOTHING_CHOSEN
}
pub unsafe fn InitMenuInUpperLeftCorner(
    windowId: u8,
    itemCount: u8,
    initialCursorPos: u8,
    APressMuted: u8,
) -> u8 {
    sMenu.left = 0;
    sMenu.top = 1;
    sMenu.minCursorPos = 0;
    sMenu.maxCursorPos = itemCount as i8 - 1;
    sMenu.windowId = windowId;
    sMenu.fontId = FONT_NORMAL;
    sMenu.optionHeight = 16;
    sMenu.APressMuted = APressMuted;
    let pos: i32 = initialCursorPos as i32;
    if pos < 0 || pos > sMenu.maxCursorPos as i32 {
        sMenu.cursorPos = 0;
    } else {
        sMenu.cursorPos = pos as i8;
    }
    Menu_MoveCursor(0)
}
#[unsafe(no_mangle)]
pub unsafe fn InitMenuInUpperLeftCornerNormal(
    windowId: u8,
    itemCount: u8,
    initialCursorPos: u8,
) -> u8 {
    InitMenuInUpperLeftCorner(windowId, itemCount, initialCursorPos, FALSE)
}
pub unsafe fn PrintMenuTable(windowId: u8, itemCount: u8, menuActions: *mut MenuAction) {
    for i in 0..(itemCount as u32) {
        AddTextPrinterParameterized(
            windowId,
            1,
            (*menuActions.at(i)).text,
            8,
            i as u8 * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
pub unsafe fn PrintMenuActionTextsInUpperLeftCorner(
    windowId: u8,
    itemCount: u8,
    menuActions: *mut MenuAction,
    actionIds: *mut u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.windowId = windowId;
    printer.fontId = FONT_NORMAL;
    printer.set_fgColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_SHADOW));
    printer.set_unk(GetFontAttribute(FONT_NORMAL, FONTATTR_UNKNOWN));
    printer.letterSpacing = 0;
    printer.lineSpacing = 0;
    printer.x = 8;
    printer.currentX = 8;
    for i in 0..itemCount {
        printer.currentChar = (*menuActions.at(*actionIds.at(i))).text;
        printer.y = i * 16 + 1;
        printer.currentY = i * 16 + 1;
        AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe fn CreateYesNoMenu(
    window: *mut WindowTemplate,
    baseTileNum: u16,
    paletteNum: u8,
    initialCursorPos: u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    sYesNoWindowId.set(AddWindow(window) as u8);
    DrawStdFrameWithCustomTileAndPalette(sYesNoWindowId.get(), TRUE, baseTileNum, paletteNum);
    printer.currentChar = (*(&raw const crate::data::strings::gText_YesNo).cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    printer.windowId = sYesNoWindowId.get();
    printer.fontId = FONT_NORMAL;
    printer.x = 8;
    printer.y = 1;
    printer.currentX = printer.x;
    printer.currentY = printer.y;
    printer.set_fgColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_SHADOW));
    printer.set_unk(GetFontAttribute(FONT_NORMAL, FONTATTR_UNKNOWN));
    printer.letterSpacing = 0;
    printer.lineSpacing = 0;
    AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
    InitMenuInUpperLeftCornerNormal(sYesNoWindowId.get(), 2, initialCursorPos);
}
pub unsafe fn PrintMenuGridTable(
    windowId: u8,
    optionWidth: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut MenuAction,
) {
    for i in 0..(rows as u32) {
        for j in 0..(columns as u32) {
            AddTextPrinterParameterized(
                windowId,
                1,
                (*menuActions.at(i * columns as u32 + j)).text,
                optionWidth * j as u8 + 8,
                i as u8 * 16 + 1,
                TEXT_SKIP_DRAW,
                None,
            );
        }
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
unsafe fn PrintMenuActionGridTextNoSpacing(
    windowId: u8,
    optionWidth: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut MenuAction,
    actionIds: *mut u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.windowId = windowId;
    printer.fontId = FONT_NORMAL;
    printer.set_fgColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_SHADOW));
    printer.set_unk(GetFontAttribute(FONT_NORMAL, FONTATTR_UNKNOWN));
    printer.letterSpacing = 0;
    printer.lineSpacing = 0;
    for i in 0..rows {
        for j in 0..columns {
            printer.currentChar =
                (*menuActions.at(*actionIds.at(columns as i32 * i as i32 + j as i32))).text;
            printer.x = optionWidth * j + 8;
            printer.y = 16 * i + 1;
            printer.currentX = printer.x;
            printer.currentY = printer.y;
            AddTextPrinter(&raw mut printer, TEXT_SKIP_DRAW, None);
        }
    }
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
pub unsafe fn InitMenuActionGrid(
    windowId: u8,
    optionWidth: u8,
    columns: u8,
    rows: u8,
    initialCursorPos: u8,
) -> u8 {
    sMenu.left = 0;
    sMenu.top = 1;
    sMenu.minCursorPos = 0;
    sMenu.maxCursorPos = columns as i8 * rows as i8 - 1;
    sMenu.windowId = windowId;
    sMenu.fontId = FONT_NORMAL;
    sMenu.optionWidth = optionWidth;
    sMenu.optionHeight = 16;
    sMenu.columns = columns;
    sMenu.rows = rows;
    let pos: i32 = initialCursorPos as i32;
    if pos < 0 || pos > sMenu.maxCursorPos as i32 {
        sMenu.cursorPos = 0;
    } else {
        sMenu.cursorPos = pos as i8;
    }
    ChangeMenuGridCursorPosition(MENU_CURSOR_DELTA_NONE, MENU_CURSOR_DELTA_NONE);
    sMenu.cursorPos as u8
}
#[unsafe(no_mangle)]
pub unsafe fn ClearScheduledBgCopiesToVram() {
    memset(sScheduledBgCopiesToVram.as_mut_ptr(), 0, 4);
}
#[unsafe(no_mangle)]
pub unsafe fn ScheduleBgCopyTilemapToVram(bgId: u8) {
    sScheduledBgCopiesToVram[bgId] = TRUE;
}
#[unsafe(no_mangle)]
pub unsafe fn DoScheduledBgTilemapCopiesToVram() {
    if sScheduledBgCopiesToVram[0] == TRUE {
        CopyBgTilemapBufferToVram(0);
        sScheduledBgCopiesToVram[0] = 0;
    }
    if sScheduledBgCopiesToVram[1] == 1 {
        CopyBgTilemapBufferToVram(1);
        sScheduledBgCopiesToVram[1] = FALSE;
    }
    if sScheduledBgCopiesToVram[2] == TRUE {
        CopyBgTilemapBufferToVram(2);
        sScheduledBgCopiesToVram[2] = FALSE;
    }
    if sScheduledBgCopiesToVram[3] == TRUE {
        CopyBgTilemapBufferToVram(3);
        sScheduledBgCopiesToVram[3] = FALSE;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ResetTempTileDataBuffers() {
    for i in 0..32i32 {
        sTempTileDataBuffer[i] = null_mut();
    }
    sTempTileDataBufferIdx.set(0);
}
#[unsafe(no_mangle)]
pub unsafe fn FreeTempTileDataBuffersIfPossible() -> u8 {
    let mut i: i32 = 0;
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        if sTempTileDataBufferIdx.get() != 0 {
            i = 0;
            while i < sTempTileDataBufferIdx.get() as i32 {
                Free(sTempTileDataBuffer[i]);
                sTempTileDataBuffer[i] = null_mut();
                i += 1;
            }
            sTempTileDataBufferIdx.set(0);
        }
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DecompressAndCopyTileDataToVram(
    bgId: u8,
    src: *mut c_void,
    mut size: u32,
    offset: u16,
    mode: u8,
) -> *mut c_void {
    let mut sizeOut: u32 = 0;
    if sTempTileDataBufferIdx.get() < 32 {
        let ptr: *mut c_void = malloc_and_decompress(src, &raw mut sizeOut);
        if size == 0 {
            size = sizeOut;
        }
        if !ptr.is_null() {
            copy_decompressed_tile_data_to_vram(bgId, ptr, size as u16, offset, mode);
            sTempTileDataBuffer[{
                let t1 = sTempTileDataBufferIdx.get();
                sTempTileDataBufferIdx.set(sTempTileDataBufferIdx.get() + 1);
                t1
            }] = ptr;
        }
        return ptr;
    }
    null_mut()
}
pub unsafe fn DecompressAndLoadBgGfxUsingHeap(
    bgId: u8,
    src: *mut c_void,
    mut size: u32,
    offset: u16,
    mode: u8,
) {
    let mut sizeOut: u32 = 0;
    let ptr: *mut c_void = malloc_and_decompress(src, &raw mut sizeOut);
    if size == 0 {
        size = sizeOut;
    }
    if !ptr.is_null() {
        let taskId: u8 = CreateTask(Some(task_free_buf_after_copying_tile_data_to_vram), 0);
        task_set(
            taskId,
            0,
            copy_decompressed_tile_data_to_vram(bgId, ptr, size as u16, offset, mode) as i16,
        );
        SetWordTaskArg(taskId, 1, ptr as usize as u32);
    }
}
pub unsafe fn task_free_buf_after_copying_tile_data_to_vram(taskId: u8) {
    if CheckForSpaceForDma3Request(task_get(taskId, 0)) == 0 {
        Free(GetWordTaskArg(taskId, 1) as usize as *mut c_void);
        DestroyTask(taskId);
    }
}
pub unsafe fn malloc_and_decompress(src: *mut c_void, size: *mut u32) -> *mut c_void {
    let sizeAsBytes: *mut u8 = size as *mut u8;
    let srcAsBytes: *mut u8 = src as *mut u8;
    *sizeAsBytes = *srcAsBytes.at(1);
    *sizeAsBytes.at(1) = *srcAsBytes.at(2);
    *sizeAsBytes.at(2) = *srcAsBytes.at(3);
    *sizeAsBytes.at(3) = 0;
    let ptr: *mut c_void = Alloc(*size);
    if !ptr.is_null() {
        LZ77UnCompWram(src as *mut u32, ptr);
    }
    ptr
}
pub unsafe fn copy_decompressed_tile_data_to_vram(
    bgId: u8,
    src: *mut c_void,
    size: u16,
    offset: u16,
    mode: u8,
) -> u16 {
    match mode {
        0 => {
            return LoadBgTiles(bgId, src, size, offset);
        }
        1 => {
            return LoadBgTilemap(bgId, src, size, offset);
        }
        _ => {
            return 65535;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn SetBgTilemapPalette(bgId: u8, left: u8, top: u8, width: u8, height: u8, palette: u8) {
    let mut j: u8 = 0;
    let ptr: *mut u16 = GetBgTilemapBuffer(bgId) as *mut u16;
    let mut i: u8 = top;
    while (i as i32) < top as i32 + height as i32 {
        j = left;
        while (j as i32) < left as i32 + width as i32 {
            *ptr.at(i as i32 * 32 + j as i32) =
                *ptr.at(i as i32 * 32 + j as i32) & 0xFFF | (palette as u16) << 12;
            j += 1;
        }
        i += 1;
    }
}
pub unsafe fn CopyToBufferFromBgTilemap(
    bgId: u8,
    dest: *mut u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    let src: *mut u16 = GetBgTilemapBuffer(bgId) as *mut u16;
    for i in 0..height {
        for j in 0..width {
            *dest.at(i as i32 * width as i32 + j as i32) =
                *src.at((i as i32 + top as i32) * 32 + j as i32 + left as i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AddValToTilemapBuffer(
    ptr: *mut c_void,
    delta: i32,
    width: i32,
    height: i32,
    isAffine: u32,
) {
    let area: i32 = width * height;
    if isAffine == TRUE as u32 {
        let as8BPP: *mut u8 = ptr as *mut u8;
        for i in 0..area {
            *as8BPP.at(i) += delta as u8;
        }
    } else {
        let as4BPP: *mut u16 = ptr as *mut u16;
        for i in 0..area {
            *as4BPP.at(i) = *as4BPP.at(i) & 0xFC00 | (*as4BPP.at(i) + delta as u16) & 0x3FF;
        }
    }
}
pub unsafe fn ResetBgPositions() {
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
}
pub unsafe fn BgDmaFill(bg: u32, value: u8, offset: i32, size: i32) {
    let temp: i32 = if GetBgAttribute(bg as u8, BG_ATTR_PALETTEMODE) == 0 {
        32
    } else {
        64
    };
    let addr: *mut c_void = (GetBgAttribute(bg as u8, BG_ATTR_CHARBASEINDEX) as i32 * 0x4000
        + (GetBgAttribute(bg as u8, BG_ATTR_BASETILE) as i32 + offset) * temp)
        as usize as *mut c_void;
    RequestDma3Fill(
        (value as i32) << 24 | (value as i32) << 16 | (value as i32) << 8 | value as i32,
        (addr as *mut u8).at(0x6000000) as *mut c_void,
        size as u16 * temp as u16,
        1,
    );
}
pub unsafe fn AddTextPrinterParameterized3(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    color: *mut u8,
    speed: i8,
    str: *mut u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.currentChar = str;
    printer.windowId = windowId;
    printer.fontId = fontId;
    printer.x = left;
    printer.y = top;
    printer.currentX = printer.x;
    printer.currentY = printer.y;
    printer.letterSpacing = GetFontAttribute(fontId, FONTATTR_LETTER_SPACING);
    printer.lineSpacing = GetFontAttribute(fontId, FONTATTR_LINE_SPACING);
    printer.set_unk(0);
    printer.set_fgColor(*color.at(1));
    printer.set_bgColor(*color);
    printer.set_shadowColor(*color.at(2));
    AddTextPrinter(&raw mut printer, speed as u8, None);
}
#[unsafe(no_mangle)]
pub unsafe fn AddTextPrinterParameterized4(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    letterSpacing: u8,
    lineSpacing: u8,
    color: *mut u8,
    speed: i8,
    str: *mut u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.currentChar = str;
    printer.windowId = windowId;
    printer.fontId = fontId;
    printer.x = left;
    printer.y = top;
    printer.currentX = printer.x;
    printer.currentY = printer.y;
    printer.letterSpacing = letterSpacing;
    printer.lineSpacing = lineSpacing;
    printer.set_unk(0);
    printer.set_fgColor(*color.at(1));
    printer.set_bgColor(*color);
    printer.set_shadowColor(*color.at(2));
    AddTextPrinter(&raw mut printer, speed as u8, None);
}
pub unsafe fn AddTextPrinterParameterized5(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
    speed: u8,
    callback: Option<unsafe fn(*mut TextPrinterTemplate, u16)>,
    letterSpacing: u8,
    lineSpacing: u8,
) {
    let mut printer: TextPrinterTemplate = zeroed();
    printer.currentChar = str;
    printer.windowId = windowId;
    printer.fontId = fontId;
    printer.x = left;
    printer.y = top;
    printer.currentX = left;
    printer.currentY = top;
    printer.letterSpacing = letterSpacing;
    printer.lineSpacing = lineSpacing;
    printer.set_unk(0);
    printer.set_fgColor(GetFontAttribute(fontId, FONTATTR_COLOR_FOREGROUND));
    printer.set_bgColor(GetFontAttribute(fontId, FONTATTR_COLOR_BACKGROUND));
    printer.set_shadowColor(GetFontAttribute(fontId, FONTATTR_COLOR_SHADOW));
    AddTextPrinter(&raw mut printer, speed, callback);
}
pub unsafe fn PrintPlayerNameOnWindow(windowId: u8, src: *mut u8, x: u16, y: u16) {
    let mut count: i32 = 0;
    while (*gSaveBlock2Ptr).playerName[count] != EOS {
        count += 1;
    }
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), src);
    AddTextPrinterParameterized(
        windowId,
        1,
        gStringVar4.as_mut_ptr(),
        x as u8,
        y as u8,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn UnusedBlitBitmapRect(
    src: *mut Bitmap,
    dst: *mut Bitmap,
    srcX: u16,
    srcY: u16,
    dstX: u16,
    dstY: u16,
    width: u16,
    height: u16,
) {
    let mut loopSrcX: i32 = 0;
    let mut loopDstX: i32 = 0;
    let mut xEnd: i32 = 0;
    let mut yEnd: i32 = 0;
    let mut pixelsSrc: *mut u8 = null_mut();
    let mut pixelsDst: *mut u8 = null_mut();
    let mut toOrr: u16 = 0;
    if ((*dst).width() - dstX as u32) < width as u32 {
        xEnd = (*dst).width() as i32 - dstX as i32 + srcX as i32;
    } else {
        xEnd = width as i32 + srcX as i32;
    }
    if ((*dst).height() - dstY as u32) < height as u32 {
        yEnd = srcY as i32 + (*dst).height() as i32 - dstY as i32;
    } else {
        yEnd = srcY as i32 + height as i32;
    }
    let multiplierSrcY: i32 = (((*src).width() + (*src).width() % 8) >> 3) as i32;
    let multiplierDstY: i32 = (((*dst).width() + (*dst).width() % 8) >> 3) as i32;
    let mut loopSrcY: i32 = srcY as i32;
    let mut loopDstY: i32 = dstY as i32;
    while loopSrcY < yEnd {
        loopSrcX = srcX as i32;
        loopDstX = dstX as i32;
        while loopSrcX < xEnd {
            pixelsSrc = (*src)
                .pixels
                .at(loopSrcX >> 1 & 3)
                .at(loopSrcX >> 3 << 5)
                .at(((loopSrcY >> 3) * multiplierSrcY) << 5)
                .at((loopSrcY as u32) << 29 >> 27);
            pixelsDst = (((((*dst).pixels as *mut c_void as *mut u8).at(loopDstX >> 1 & 3)
                as *mut c_void as *mut u8)
                .at(loopDstX >> 3 << 5) as *mut c_void as *mut u8)
                .at(((loopDstY >> 3) * multiplierDstY) << 5) as *mut c_void
                as *mut u8)
                .at((loopDstY as u32) << 29 >> 27) as *mut c_void
                as *mut u8;
            if pixelsDst as usize as u32 & 1 != 0 {
                pixelsDst = pixelsDst.at(-1);
                if loopDstX & 1 != 0 {
                    toOrr = (pixelsDst as *mut u16).read_volatile();
                    toOrr &= 0x0fff;
                    if loopSrcX & 1 != 0 {
                        toOrr |= (*pixelsSrc as u16 & 0xf0) << 8;
                    } else {
                        toOrr |= (*pixelsSrc as u16 & 0x0f) << 12;
                    }
                } else {
                    toOrr = (pixelsDst as *mut u16).read_volatile();
                    toOrr &= 0xf0ff;
                    if loopSrcX & 1 != 0 {
                        toOrr |= (*pixelsSrc as u16 & 0xf0) << 4;
                    } else {
                        toOrr |= (*pixelsSrc as u16 & 0x0f) << 8;
                    }
                }
            } else {
                if loopDstX & 1 != 0 {
                    toOrr = (pixelsDst as *mut u16).read_volatile();
                    toOrr &= 0xff0f;
                    if loopSrcX & 1 != 0 {
                        toOrr |= *pixelsSrc as u16 & 0xf0;
                    } else {
                        toOrr |= (*pixelsSrc as u16 & 0x0f) << 4;
                    }
                } else {
                    toOrr = (pixelsDst as *mut u16).read_volatile();
                    toOrr &= 0xfff0;
                    if loopSrcX & 1 != 0 {
                        toOrr |= ((*pixelsSrc as i32 & 0xf0) >> 4) as u16;
                    } else {
                        toOrr |= (*pixelsSrc as i32 & 0x0f) as u16;
                    }
                }
            }
            volatile_write(pixelsDst as *mut u16, toOrr);
            loopSrcX += 1;
            loopDstX += 1;
        }
        loopSrcY += 1;
        loopDstY += 1;
    }
}
unsafe fn LoadMonIconPalAtOffset(palOffset: u8, speciesId: u16) {
    LoadPalette(
        GetValidMonIconPalettePtr(speciesId) as *mut c_void,
        palOffset as u16,
        32,
    );
}
unsafe fn DrawMonIconAtPos(windowId: u8, speciesId: u16, personality: u32, x: u16, y: u16) {
    BlitBitmapToWindow(
        windowId,
        GetMonIconPtr(speciesId, personality, 1),
        x,
        y,
        32,
        32,
    );
}
pub unsafe fn ListMenuLoadStdPalAt(palOffset: u8, palId: u8) {
    let mut palette: *mut u16 = null_mut();
    match palId {
        1 => {
            palette = (*(&raw const crate::data::graphics::gMenuInfoElements2_Pal)
                .cast::<CArray<u16, 16>>())
            .as_ptr()
            .cast_mut();
        }
        2 => {
            palette = (*(&raw const crate::data::graphics::gMenuInfoElements3_Pal)
                .cast::<CArray<u16, 16>>())
            .as_ptr()
            .cast_mut();
        }
        _ => {
            palette = (*(&raw const crate::data::graphics::gMenuInfoElements1_Pal)
                .cast::<CArray<u16, 16>>())
            .as_ptr()
            .cast_mut();
        }
    }
    LoadPalette(palette as *mut c_void, palOffset as u16, 32);
}
pub unsafe fn BlitMenuInfoIcon(windowId: u8, iconId: u8, x: u16, y: u16) {
    BlitBitmapRectToWindow(
        windowId,
        (&raw const (*(&raw const crate::data::graphics::gMenuInfoElements_Gfx)
            .cast::<CArray<u8, 0>>())[sMenuInfoIcons[iconId].offset as i32 * 32])
            .cast_mut(),
        0,
        0,
        128,
        128,
        x,
        y,
        sMenuInfoIcons[iconId].width as u16,
        sMenuInfoIcons[iconId].height as u16,
    );
}
pub unsafe fn BufferSaveMenuText(textId: u8, dest: *mut u8, color: u8) {
    let mut flagCount: i32 = 0;
    let mut endOfString: *mut u8 = null_mut();
    let mut string: *mut u8 = dest;
    *({
        let t1 = string;
        string = string.at(1);
        t1
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t2 = string;
        string = string.at(1);
        t2
    }) = EXT_CTRL_CODE_COLOR;
    *({
        let t3 = string;
        string = string.at(1);
        t3
    }) = color;
    *({
        let t4 = string;
        string = string.at(1);
        t4
    }) = EXT_CTRL_CODE_BEGIN;
    *({
        let t5 = string;
        string = string.at(1);
        t5
    }) = EXT_CTRL_CODE_SHADOW;
    *({
        let t6 = string;
        string = string.at(1);
        t6
    }) = color + 1;
    match textId {
        SAVE_MENU_NAME => {
            StringCopy(string, (*gSaveBlock2Ptr).playerName.as_mut_ptr());
        }
        SAVE_MENU_CAUGHT => {
            if IsNationalPokedexEnabled() != 0 {
                string = ConvertIntToDecimalStringN(
                    string,
                    GetNationalPokedexCount(FLAG_GET_CAUGHT) as i32,
                    STR_CONV_MODE_LEFT_ALIGN,
                    3,
                );
            } else {
                string = ConvertIntToDecimalStringN(
                    string,
                    GetHoennPokedexCount(FLAG_GET_CAUGHT) as i32,
                    STR_CONV_MODE_LEFT_ALIGN,
                    3,
                );
            }
            *string = EOS;
        }
        SAVE_MENU_PLAY_TIME => {
            string = ConvertIntToDecimalStringN(
                string,
                (*gSaveBlock2Ptr).playTimeHours as i32,
                STR_CONV_MODE_LEFT_ALIGN,
                3,
            );
            *({
                let t7 = string;
                string = string.at(1);
                t7
            }) = CHAR_COLON;
            ConvertIntToDecimalStringN(
                string,
                (*gSaveBlock2Ptr).playTimeMinutes as i32,
                STR_CONV_MODE_LEADING_ZEROS,
                2,
            );
        }
        SAVE_MENU_LOCATION => {
            GetMapNameGeneric(string, gMapHeader.regionMapSectionId as u16);
        }
        SAVE_MENU_BADGES => {
            flagCount = 0;
            endOfString = string.at(1);
            for curFlag in (FLAG_BADGE01_GET as i32)..2159 {
                if FlagGet(curFlag as u16) != 0 {
                    flagCount += 1;
                }
            }
            *string = flagCount as u8 + CHAR_0;
            *endOfString = EOS;
        }
        _ => {}
    }
}
