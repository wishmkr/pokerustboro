//! Translated from `src/menu.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): gStandardMenuPalette sTextSpeedFrameDelays sStandardTextBox_WindowTemplates sYesNo_WindowTemplates sHofPC_TopBar_Pal sTextColors sMenuInfoIcons
#[allow(unused_imports)]
use crate::data::menu::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStartMenuWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMapNamePopupWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sMenu: crate::ffi::Align4<[u8; 12]> = crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTileNum: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPaletteNum: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sYesNoWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sHofPCTopBarWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFiller: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScheduledBgCopiesToVram: crate::ffi::Align4<[u8; 4]> =
    crate::ffi::Align4([0; 4]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTempTileDataBufferIdx: u16 = 0u16;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTempTileDataBuffer: crate::ffi::Align4<[u8; 128]> =
    crate::ffi::Align4([0; 128]);

unsafe extern "C" {
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMenuInfoElements1_Pal: u8;
    static mut gMenuInfoElements2_Pal: u8;
    static mut gMenuInfoElements3_Pal: u8;
    static mut gMenuInfoElements_Gfx: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gTextFlags: u8;
    static mut gText_SelectorArrow3: u8;
    static mut gText_YesNo: u8;
    fn AddTextPrinter(a0: *mut u8, a1: u8, a2: Option<unsafe extern "C" fn(*mut u8, u16)>) -> u16;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn Alloc(a0: u32) -> *mut u8;
    fn BlitBitmapRectToWindow(
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
    );
    fn BlitBitmapToWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn CallWindowFunction(a0: u8, a1: Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CheckForSpaceForDma3Request(a0: i16) -> i16;
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DisplayMessageAndContinueTask(
        a0: u8,
        a1: u8,
        a2: u16,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: *mut u8,
    );
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagGet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn FreeAllWindowBuffers();
    fn GetBgAttribute(a0: u8, a1: u8) -> u16;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetHoennPokedexCount(a0: u8) -> u16;
    fn GetLRKeysPressed() -> u8;
    fn GetLRKeysPressedAndHeld() -> u8;
    fn GetMapNameGeneric(a0: *mut u8, a1: u16) -> *mut u8;
    fn GetMenuCursorDimensionByFont(a0: u8, a1: u8) -> u8;
    fn GetMonIconPtr(a0: u16, a1: u32, a2: u32) -> *mut u8;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetValidMonIconPalettePtr(a0: u16) -> *mut u16;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GetWordTaskArg(a0: u8, a1: u8) -> u32;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTilemap(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn PlaySE(a0: u16);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn RequestDma3Fill(a0: i32, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn RunTextPrinters();
    fn SetWordTaskArg(a0: u8, a1: u8, a2: u32);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitStandardTextBoxWindows() {
    unsafe {
        InitWindows(
            ((&raw const sStandardTextBox_WindowTemplates)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).write(255u8);
        ((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).write(255u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeAllOverworldWindowBuffers() {
    unsafe {
        FreeAllWindowBuffers();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitTextBoxGfxAndPrinters() {
    unsafe {
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        DeactivateAllTextPrinters();
        LoadMessageBoxAndBorderGfx();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunTextPrintersAndIsPrinter0Active() -> u16 {
    unsafe {
        RunTextPrinters();
        return IsTextPrinterActive(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterParameterized2(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    speed: u8,
    callback: Option<unsafe extern "C" fn(*mut u8, u16)>,
    fgColor: u8,
    bgColor: u8,
    shadowColor: u8,
) -> u16 {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut speed = speed;
        let mut callback = callback;
        let mut fgColor = fgColor;
        let mut bgColor = bgColor;
        let mut shadowColor = shadowColor;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(1u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(9)).write(1u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(0u8);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (fgColor) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (bgColor) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (shadowColor) as i32,
        );
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            1,
            1,
            (0u8) as i32,
        );
        return AddTextPrinter((&raw mut printer).cast::<u8>(), speed, callback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterForMessage(allowSkippingDelayWithButtonPress: u8) {
    unsafe {
        let mut allowSkippingDelayWithButtonPress = allowSkippingDelayWithButtonPress;
        let mut callback: Option<unsafe extern "C" fn(*mut u8, u16)> = None;
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (allowSkippingDelayWithButtonPress) as i32,
        );
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            GetPlayerTextSpeedDelay(),
            callback,
            2u8,
            1u8,
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterForMessage_2(allowSkippingDelayWithButtonPress: u8) {
    unsafe {
        let mut allowSkippingDelayWithButtonPress = allowSkippingDelayWithButtonPress;
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (allowSkippingDelayWithButtonPress) as i32,
        );
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            GetPlayerTextSpeedDelay(),
            None,
            2u8,
            1u8,
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterWithCustomSpeedForMessage(
    allowSkippingDelayWithButtonPress: u8,
    speed: u8,
) {
    unsafe {
        let mut allowSkippingDelayWithButtonPress = allowSkippingDelayWithButtonPress;
        let mut speed = speed;
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (allowSkippingDelayWithButtonPress) as i32,
        );
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            speed,
            None,
            2u8,
            1u8,
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMessageBoxAndBorderGfx() {
    unsafe {
        LoadMessageBoxGfx(0u8, 512u16, 240u8);
        LoadUserWindowBorderGfx(0u8, 532u16, 224u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawDialogueFrame(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(windowId, Some(WindowFunc_DrawDialogueFrame));
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawStdWindowFrame(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(windowId, Some(WindowFunc_DrawStandardFrame));
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearDialogWindowAndFrame(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(windowId, Some(WindowFunc_ClearDialogWindowAndFrame));
        FillWindowPixelBuffer(windowId, 17u8);
        ClearWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearStdWindowAndFrame(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(windowId, Some(WindowFunc_ClearStdWindowAndFrame));
        FillWindowPixelBuffer(windowId, 17u8);
        ClearWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_DrawStandardFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        let mut i: i32 = 0i32;
        FillBgTilemapBufferRect(
            bg,
            532u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            14u8,
        );
        FillBgTilemapBufferRect(
            bg,
            533u16,
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            width,
            1u8,
            14u8,
        );
        FillBgTilemapBufferRect(
            bg,
            534u16,
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            14u8,
        );
        {
            i = ((tilemapTop) as i32);
            'l1: loop {
                if !(i < ((tilemapTop) as i32).wrapping_add(((height) as i32))) {
                    break 'l1;
                }
                'l2: {
                    FillBgTilemapBufferRect(
                        bg,
                        535u16,
                        ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
                        ((i) as u8),
                        1u8,
                        1u8,
                        14u8,
                    );
                    FillBgTilemapBufferRect(
                        bg,
                        537u16,
                        ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
                        ((i) as u8),
                        1u8,
                        1u8,
                        14u8,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        FillBgTilemapBufferRect(
            bg,
            538u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            14u8,
        );
        FillBgTilemapBufferRect(
            bg,
            539u16,
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            width,
            1u8,
            14u8,
        );
        FillBgTilemapBufferRect(
            bg,
            540u16,
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            14u8,
        );
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_DrawDialogueFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            513u16,
            ((((tilemapLeft) as i32).wrapping_sub(2i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            515u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            516u16,
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            ((((width) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            517u16,
            (((((tilemapLeft) as i32).wrapping_add(((width) as i32))).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            518u16,
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            519u16,
            ((((tilemapLeft) as i32).wrapping_sub(2i32)) as u8),
            tilemapTop,
            1u8,
            5u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            521u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            tilemapTop,
            ((((width) as i32).wrapping_add(1i32)) as u8),
            5u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            522u16,
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            tilemapTop,
            1u8,
            5u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            2561u16,
            ((((tilemapLeft) as i32).wrapping_sub(2i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            2563u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            2564u16,
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            ((((width) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            2565u16,
            (((((tilemapLeft) as i32).wrapping_add(((width) as i32))).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            15u8,
        );
        FillBgTilemapBufferRect(
            bg,
            2566u16,
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            15u8,
        );
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_ClearStdWindowAndFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            0u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            ((((width) as i32).wrapping_add(2i32)) as u8),
            ((((height) as i32).wrapping_add(2i32)) as u8),
            14u8,
        );
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_ClearDialogWindowAndFrame(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            0u16,
            ((((tilemapLeft) as i32).wrapping_sub(3i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            ((((width) as i32).wrapping_add(6i32)) as u8),
            ((((height) as i32).wrapping_add(2i32)) as u8),
            14u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetStandardWindowBorderStyle(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        DrawStdFrameWithCustomTileAndPalette(windowId, copyToVram, 532u16, 14u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadMessageBoxAndFrameGfx(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        LoadMessageBoxGfx(windowId, 512u16, 240u8);
        DrawDialogFrameWithCustomTileAndPalette(windowId, copyToVram, 512u16, 15u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_LoadStdPal() {
    unsafe {
        LoadPalette(
            (((&raw const gStandardMenuPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            224u16,
            20u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_LoadStdPalAt(offset: u16) {
    unsafe {
        let mut offset = offset;
        LoadPalette(
            (((&raw const gStandardMenuPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            offset,
            20u16,
        );
    }
}
pub(crate) unsafe extern "C" fn Menu_GetStdPal() -> *mut u16 {
    unsafe {
        return ((&raw const gStandardMenuPalette)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>();
    }
}
pub(crate) unsafe extern "C" fn Menu_GetStdPalColor(colorNum: u8) -> u16 {
    unsafe {
        let mut colorNum = colorNum;
        if ((colorNum) as i32) > 15i32 {
            colorNum = 0u8;
        }
        return ((((&raw const gStandardMenuPalette)
            .cast::<u8>()
            .cast_mut()
            .cast::<u16>())
        .cast::<u16>())
        .wrapping_offset(((colorNum) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayItemMessageOnField(
    taskId: u8,
    string: *mut u8,
    callback: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut taskId = taskId;
        let mut string = string;
        let mut callback = callback;
        LoadMessageBoxAndBorderGfx();
        DisplayMessageAndContinueTask(
            taskId,
            0u8,
            512u16,
            15u8,
            1u8,
            GetPlayerTextSpeedDelay(),
            string,
            core::mem::transmute::<_, *mut u8>(callback),
        );
        CopyWindowToVram(0u8, 3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayYesNoMenuDefaultYes() {
    unsafe {
        CreateYesNoMenu(
            (&raw const sYesNo_WindowTemplates).cast::<u8>().cast_mut(),
            532u16,
            14u8,
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayYesNoMenuWithDefault(initialCursorPos: u8) {
    unsafe {
        let mut initialCursorPos = initialCursorPos;
        CreateYesNoMenu(
            (&raw const sYesNo_WindowTemplates).cast::<u8>().cast_mut(),
            532u16,
            14u8,
            initialCursorPos,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerTextSpeed() -> u32 {
    unsafe {
        if (crate::c::bf_read(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            3,
            1,
            false,
        ) as u8)
            != 0
        {
            return 1u32;
        }
        return ((crate::c::bf_read(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
            0,
            3,
            false,
        ) as u16) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerTextSpeedDelay() -> u8 {
    unsafe {
        let mut speed: u32 = 0u32;
        if ((crate::c::bf_read(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
            0,
            3,
            false,
        ) as u16) as i32)
            > 2i32
        {
            crate::c::bf_write(
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                0,
                3,
                (1u16) as i32,
            );
        }
        speed = GetPlayerTextSpeed();
        return ((((&raw const sTextSpeedFrameDelays).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((speed) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddStartMenuWindow(numActions: u8) -> u8 {
    unsafe {
        let mut numActions = numActions;
        if ((((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).read()) as i32) == 255i32 {
            ((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).write(
                ((AddWindowParameterized(
                    0u8,
                    22u8,
                    1u8,
                    7u8,
                    (((((numActions) as i32).wrapping_mul(2i32)).wrapping_add(2i32)) as u8),
                    15u8,
                    313u16,
                )) as u8),
            );
        }
        return ((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetStartMenuWindowId() -> u8 {
    unsafe {
        return ((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveStartMenuWindow() {
    unsafe {
        if ((((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            RemoveWindow(((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).read());
            ((&raw mut sStartMenuWindowId).cast::<u8>().cast::<u8>()).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn GetDialogFrameBaseTileNum() -> u16 {
    unsafe {
        return 512u16;
    }
}
pub(crate) unsafe extern "C" fn GetStandardFrameBaseTileNum() -> u16 {
    unsafe {
        return 532u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddMapNamePopUpWindow() -> u8 {
    unsafe {
        if ((((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).read()) as i32) == 255i32
        {
            ((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>())
                .write(((AddWindowParameterized(0u8, 1u8, 1u8, 10u8, 3u8, 14u8, 263u16)) as u8));
        }
        return ((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMapNamePopUpWindowId() -> u8 {
    unsafe {
        return ((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RemoveMapNamePopUpWindow() {
    unsafe {
        if ((((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32
        {
            RemoveWindow(((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).read());
            ((&raw mut sMapNamePopupWindowId).cast::<u8>().cast::<u8>()).write(255u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterWithCallbackForMessage(
    canSpeedUp: u8,
    callback: Option<unsafe extern "C" fn(*mut u8, u16)>,
) {
    unsafe {
        let mut canSpeedUp = canSpeedUp;
        let mut callback = callback;
        crate::c::bf_write(
            ((&raw mut gTextFlags).cast::<u8>()).wrapping_add(0),
            0,
            1,
            (canSpeedUp) as i32,
        );
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            GetPlayerTextSpeedDelay(),
            callback,
            2u8,
            1u8,
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EraseFieldMessageBox(copyToVram: u8) {
    unsafe {
        let mut copyToVram = copyToVram;
        FillBgTilemapBufferRect(0u8, 0u16, 0u8, 0u8, 32u8, 32u8, 17u8);
        if ((copyToVram) as i32) == 1i32 {
            CopyBgTilemapBufferToVram(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawDialogFrameWithCustomTileAndPalette(
    windowId: u8,
    copyToVram: u8,
    tileNum: u16,
    paletteNum: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        let mut tileNum = tileNum;
        let mut paletteNum = paletteNum;
        ((&raw mut sTileNum).cast::<u8>().cast::<u16>()).write(tileNum);
        ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).write(paletteNum);
        CallWindowFunction(
            windowId,
            Some(WindowFunc_DrawDialogFrameWithCustomTileAndPalette),
        );
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn DrawDialogFrameWithCustomTile(
    windowId: u8,
    copyToVram: u8,
    tileNum: u16,
) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        let mut tileNum = tileNum;
        ((&raw mut sTileNum).cast::<u8>().cast::<u16>()).write(tileNum);
        ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>())
            .write(((GetWindowAttribute(windowId, 5u8)) as u8));
        CallWindowFunction(
            windowId,
            Some(WindowFunc_DrawDialogFrameWithCustomTileAndPalette),
        );
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_DrawDialogFrameWithCustomTileAndPalette(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(1i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(2i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(3i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(4i32))
                as u16),
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            ((((width) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(5i32))
                as u16),
            (((((tilemapLeft) as i32).wrapping_add(((width) as i32))).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(6i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(7i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(2i32)) as u8),
            tilemapTop,
            1u8,
            5u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(9i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            tilemapTop,
            ((((width) as i32).wrapping_add(1i32)) as u8),
            5u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32)
                .wrapping_add(10i32)) as u16),
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            tilemapTop,
            1u8,
            5u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            (((2048i32).wrapping_add(
                ((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32)
                    .wrapping_add(1i32),
            )) as u16),
            ((((tilemapLeft) as i32).wrapping_sub(2i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            (((2048i32).wrapping_add(
                ((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32)
                    .wrapping_add(3i32),
            )) as u16),
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            (((2048i32).wrapping_add(
                ((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32)
                    .wrapping_add(4i32),
            )) as u16),
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            ((((width) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            (((2048i32).wrapping_add(
                ((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32)
                    .wrapping_add(5i32),
            )) as u16),
            (((((tilemapLeft) as i32).wrapping_add(((width) as i32))).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            (((2048i32).wrapping_add(
                ((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32)
                    .wrapping_add(6i32),
            )) as u16),
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearDialogWindowAndFrameToTransparent(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(
            windowId,
            Some(WindowFunc_ClearDialogWindowAndFrameNullPalette),
        );
        FillWindowPixelBuffer(windowId, 0u8);
        ClearWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_ClearDialogWindowAndFrameNullPalette(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            0u16,
            ((((tilemapLeft) as i32).wrapping_sub(3i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            ((((width) as i32).wrapping_add(6i32)) as u8),
            ((((height) as i32).wrapping_add(2i32)) as u8),
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawStdFrameWithCustomTileAndPalette(
    windowId: u8,
    copyToVram: u8,
    baseTileNum: u16,
    paletteNum: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        let mut baseTileNum = baseTileNum;
        let mut paletteNum = paletteNum;
        ((&raw mut sTileNum).cast::<u8>().cast::<u16>()).write(baseTileNum);
        ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).write(paletteNum);
        CallWindowFunction(
            windowId,
            Some(WindowFunc_DrawStdFrameWithCustomTileAndPalette),
        );
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrawStdFrameWithCustomTile(
    windowId: u8,
    copyToVram: u8,
    baseTileNum: u16,
) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        let mut baseTileNum = baseTileNum;
        ((&raw mut sTileNum).cast::<u8>().cast::<u16>()).write(baseTileNum);
        ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>())
            .write(((GetWindowAttribute(windowId, 5u8)) as u8));
        CallWindowFunction(
            windowId,
            Some(WindowFunc_DrawStdFrameWithCustomTileAndPalette),
        );
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_DrawStdFrameWithCustomTileAndPalette(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(0i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(1i32))
                as u16),
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            width,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(2i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(3i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            tilemapTop,
            1u8,
            height,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(5i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            tilemapTop,
            1u8,
            height,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(6i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(7i32))
                as u16),
            tilemapLeft,
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            width,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
        FillBgTilemapBufferRect(
            bg,
            ((((((&raw mut sTileNum).cast::<u8>().cast::<u16>()).read()) as i32).wrapping_add(8i32))
                as u16),
            ((((tilemapLeft) as i32).wrapping_add(((width) as i32))) as u8),
            ((((tilemapTop) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            ((&raw mut sPaletteNum).cast::<u8>().cast::<u8>()).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearStdWindowAndFrameToTransparent(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(
            windowId,
            Some(WindowFunc_ClearStdWindowAndFrameToTransparent),
        );
        FillWindowPixelBuffer(windowId, 0u8);
        ClearWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn WindowFunc_ClearStdWindowAndFrameToTransparent(
    bg: u8,
    tilemapLeft: u8,
    tilemapTop: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut tilemapLeft = tilemapLeft;
        let mut tilemapTop = tilemapTop;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        FillBgTilemapBufferRect(
            bg,
            0u16,
            ((((tilemapLeft) as i32).wrapping_sub(1i32)) as u8),
            ((((tilemapTop) as i32).wrapping_sub(1i32)) as u8),
            ((((width) as i32).wrapping_add(2i32)) as u8),
            ((((height) as i32).wrapping_add(2i32)) as u8),
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HofPCTopBar_AddWindow(
    bg: u8,
    xPos: u8,
    yPos: u8,
    palette: u8,
    baseTile: u16,
) -> u8 {
    unsafe {
        let mut bg = bg;
        let mut xPos = xPos;
        let mut yPos = yPos;
        let mut palette = palette;
        let mut baseTile = baseTile;
        let mut window = crate::ffi::Align4([0u8; 8]);
        crate::c::memset((&raw mut window).cast::<u8>(), 0i32, 8u32);
        if ((bg) as i32) > 3i32 {
            ((&raw mut window).cast::<u8>()).write(0u8);
        } else {
            ((&raw mut window).cast::<u8>()).write(bg);
        }
        (((&raw mut window).cast::<u8>()).wrapping_add(2)).write(yPos);
        (((&raw mut window).cast::<u8>()).wrapping_add(4)).write(2u8);
        (((&raw mut window).cast::<u8>()).wrapping_add(1))
            .write((((30i32).wrapping_sub(((xPos) as i32))) as u8));
        (((&raw mut window).cast::<u8>()).wrapping_add(3)).write(xPos);
        (((&raw mut window).cast::<u8>()).wrapping_add(5)).write(palette);
        (((&raw mut window).cast::<u8>())
            .wrapping_add(6)
            .cast::<u16>())
        .write(baseTile);
        ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>())
            .write(((AddWindow((&raw mut window).cast::<u8>())) as u8));
        if ((palette) as i32) > 15i32 {
            palette = 240u8;
        } else {
            palette = (((0i32).wrapping_add(((palette) as i32).wrapping_mul(16i32))) as u8);
        }
        LoadPalette(
            (((&raw const sHofPC_TopBar_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            ((palette) as u16),
            32u16,
        );
        return ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HofPCTopBar_Print(string: *mut u8, left: u8, copyToVram: u8) {
    unsafe {
        let mut string = string;
        let mut left = left;
        let mut copyToVram = copyToVram;
        let mut width: u16 = 0u16;
        if ((((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            PutWindowTilemap(((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read());
            FillWindowPixelBuffer(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                255u8,
            );
            width = ((GetStringWidth(0u8, string, 0i16)) as u16);
            AddTextPrinterParameterized3(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                0u8,
                (((((236u32).wrapping_sub(
                    (GetWindowAttribute(
                        ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                        1u8,
                    ))
                    .wrapping_mul(8u32),
                ))
                .wrapping_sub(((left) as u32)))
                .wrapping_sub(((width) as u32))) as u8),
                1u8,
                ((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
                0i8,
                string,
            );
            if (copyToVram) != 0 {
                CopyWindowToVram(
                    ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                    3u8,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HofPCTopBar_PrintPair(
    string: *mut u8,
    string2: *mut u8,
    noBg: u8,
    left: u8,
    copyToVram: u8,
) {
    unsafe {
        let mut string = string;
        let mut string2 = string2;
        let mut noBg = noBg;
        let mut left = left;
        let mut copyToVram = copyToVram;
        let mut color = crate::ffi::Align4([0u8; 3]);
        let mut width: u16 = 0u16;
        if ((((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            if (noBg) != 0 {
                ((&raw mut color).cast::<u8>()).write(0u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(1u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(2u8);
            } else {
                ((&raw mut color).cast::<u8>()).write(15u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(1)).write(1u8);
                (((&raw mut color).cast::<u8>()).wrapping_offset(2)).write(2u8);
            }
            PutWindowTilemap(((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read());
            FillWindowPixelBuffer(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                255u8,
            );
            if ((string2) as usize) != 0usize {
                width = ((GetStringWidth(0u8, string2, 0i16)) as u16);
                AddTextPrinterParameterized3(
                    ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                    0u8,
                    (((((236u32).wrapping_sub(
                        (GetWindowAttribute(
                            ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                            1u8,
                        ))
                        .wrapping_mul(8u32),
                    ))
                    .wrapping_sub(((left) as u32)))
                    .wrapping_sub(((width) as u32))) as u8),
                    1u8,
                    (&raw mut color).cast::<u8>(),
                    0i8,
                    string2,
                );
            }
            AddTextPrinterParameterized4(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                4u8,
                1u8,
                0u8,
                0u8,
                (&raw mut color).cast::<u8>(),
                0i8,
                string,
            );
            if (copyToVram) != 0 {
                CopyWindowToVram(
                    ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                    3u8,
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HofPCTopBar_CopyToVram() {
    unsafe {
        if ((((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            CopyWindowToVram(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                3u8,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn HofPCTopBar_Clear() {
    unsafe {
        if ((((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            FillWindowPixelBuffer(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                255u8,
            );
            CopyWindowToVram(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                3u8,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HofPCTopBar_RemoveWindow() {
    unsafe {
        if ((((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read()) as i32) != 255i32 {
            FillWindowPixelBuffer(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                0u8,
            );
            ClearWindowTilemap(((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read());
            CopyWindowToVram(
                ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read(),
                3u8,
            );
            RemoveWindow(((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).read());
            ((&raw mut sHofPCTopBarWindowId).cast::<u8>().cast::<u8>()).write(255u8);
        }
    }
}
pub(crate) unsafe extern "C" fn InitMenu(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    cursorHeight: u8,
    numChoices: u8,
    initialCursorPos: u8,
    muteAPress: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut cursorHeight = cursorHeight;
        let mut numChoices = numChoices;
        let mut initialCursorPos = initialCursorPos;
        let mut muteAPress = muteAPress;
        let mut pos: i32 = 0i32;
        ((&raw mut sMenu).cast::<u8>()).write(left);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).write(top);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).write(0i8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>())
            .write(((((numChoices) as i32).wrapping_sub(1i32)) as i8));
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).write(windowId);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).write(fontId);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).write(cursorHeight);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(11)).write(muteAPress);
        pos = ((initialCursorPos) as i32);
        if (pos < 0i32)
            || (pos
                > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32))
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(0i8);
        } else {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(((pos) as i8));
        }
        Menu_MoveCursor(0i8);
        return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMenuNormal(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    cursorHeight: u8,
    numChoices: u8,
    initialCursorPos: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut cursorHeight = cursorHeight;
        let mut numChoices = numChoices;
        let mut initialCursorPos = initialCursorPos;
        return InitMenu(
            windowId,
            fontId,
            left,
            top,
            cursorHeight,
            numChoices,
            initialCursorPos,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn InitMenuDefaultCursorHeight(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    numChoices: u8,
    initialCursorPos: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut numChoices = numChoices;
        let mut initialCursorPos = initialCursorPos;
        let mut cursorHeight: u8 = GetMenuCursorDimensionByFont(fontId, 1u8);
        return InitMenuNormal(
            windowId,
            fontId,
            left,
            top,
            cursorHeight,
            numChoices,
            initialCursorPos,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RedrawMenuCursor(oldPos: u8, newPos: u8) {
    unsafe {
        let mut oldPos = oldPos;
        let mut newPos = newPos;
        let mut width: u8 = 0u8;
        let mut height: u8 = 0u8;
        width = GetMenuCursorDimensionByFont(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).read(),
            0u8,
        );
        height = GetMenuCursorDimensionByFont(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).read(),
            1u8,
        );
        FillWindowPixelRect(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).read(),
            17u8,
            ((((&raw mut sMenu).cast::<u8>()).read()) as u16),
            ((((((((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).read()) as i32)
                .wrapping_mul(((oldPos) as i32)))
            .wrapping_add((((((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).read()) as i32)))
                as u16),
            ((width) as u16),
            ((height) as u16),
        );
        AddTextPrinterParameterized(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).read(),
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).read(),
            (&raw mut gText_SelectorArrow3).cast::<u8>(),
            ((&raw mut sMenu).cast::<u8>()).read(),
            ((((((((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).read()) as i32)
                .wrapping_mul(((newPos) as i32)))
            .wrapping_add((((((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).read()) as i32)))
                as u8),
            0u8,
            None,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_MoveCursor(cursorDelta: i8) -> u8 {
    unsafe {
        let mut cursorDelta = cursorDelta;
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        let mut newPos: i32 = (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>())
            .read()) as i32)
            .wrapping_add(((cursorDelta) as i32));
        if newPos < (((((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).read()) as i32)
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>())
                .write((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read());
        } else {
            if newPos
                > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32)
            {
                (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>())
                    .write((((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).read());
            } else {
                let __p1 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                (__p1)
                    .write((((((__p1).read()) as i32).wrapping_add(((cursorDelta) as i32))) as i8));
            }
        }
        RedrawMenuCursor(
            oldPos,
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8),
        );
        return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_MoveCursorNoWrapAround(cursorDelta: i8) -> u8 {
    unsafe {
        let mut cursorDelta = cursorDelta;
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        let mut newPos: i32 = (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>())
            .read()) as i32)
            .wrapping_add(((cursorDelta) as i32));
        if newPos < (((((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).read()) as i32)
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>())
                .write((((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).read());
        } else {
            if newPos
                > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32)
            {
                (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>())
                    .write((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read());
            } else {
                let __p1 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                (__p1)
                    .write((((((__p1).read()) as i32).wrapping_add(((cursorDelta) as i32))) as i8));
            }
        }
        RedrawMenuCursor(
            oldPos,
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8),
        );
        return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_GetCursorPos() -> u8 {
    unsafe {
        return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_ProcessInput() -> i8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if !(((((&raw mut sMenu).cast::<u8>()).wrapping_add(11)).read()) != 0) {
                PlaySE(5u16);
            }
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    PlaySE(5u16);
                    Menu_MoveCursor((-1i8));
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        PlaySE(5u16);
                        Menu_MoveCursor(1i8);
                        return (-2i8);
                    }
                }
            }
        }
        return (-2i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_ProcessInputNoWrap() -> i8 {
    unsafe {
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if !(((((&raw mut sMenu).cast::<u8>()).wrapping_add(11)).read()) != 0) {
                PlaySE(5u16);
            }
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    if ((oldPos) as i32) != ((Menu_MoveCursorNoWrapAround((-1i8))) as i32) {
                        PlaySE(5u16);
                    }
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        if ((oldPos) as i32) != ((Menu_MoveCursorNoWrapAround(1i8)) as i32) {
                            PlaySE(5u16);
                        }
                        return (-2i8);
                    }
                }
            }
        }
        return (-2i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProcessMenuInput_other() -> i8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if !(((((&raw mut sMenu).cast::<u8>()).wrapping_add(11)).read()) != 0) {
                PlaySE(5u16);
            }
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 240i32)
                    == 64i32
                {
                    PlaySE(5u16);
                    Menu_MoveCursor((-1i8));
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 240i32)
                        == 128i32
                    {
                        PlaySE(5u16);
                        Menu_MoveCursor(1i8);
                        return (-2i8);
                    }
                }
            }
        }
        return (-2i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_ProcessInputNoWrapAround_other() -> i8 {
    unsafe {
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if !(((((&raw mut sMenu).cast::<u8>()).wrapping_add(11)).read()) != 0) {
                PlaySE(5u16);
            }
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 240i32)
                    == 64i32
                {
                    if ((oldPos) as i32) != ((Menu_MoveCursorNoWrapAround((-1i8))) as i32) {
                        PlaySE(5u16);
                    }
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 240i32)
                        == 128i32
                    {
                        if ((oldPos) as i32) != ((Menu_MoveCursorNoWrapAround(1i8)) as i32) {
                            PlaySE(5u16);
                        }
                        return (-2i8);
                    }
                }
            }
        }
        return (-2i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMenuActionTextsAtPos(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut lineHeight = lineHeight;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((itemCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized(
                        windowId,
                        fontId,
                        (((menuActions).wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<*mut u8>())
                        .read(),
                        left,
                        (((((lineHeight) as i32).wrapping_mul(((i) as i32)))
                            .wrapping_add(((top) as i32))) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionTextsWithSpacing(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut u8,
    letterSpacing: u8,
    lineSpacing: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut lineHeight = lineHeight;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        let mut letterSpacing = letterSpacing;
        let mut lineSpacing = lineSpacing;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((itemCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized5(
                        windowId,
                        fontId,
                        (((menuActions).wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<*mut u8>())
                        .read(),
                        left,
                        (((((lineHeight) as i32).wrapping_mul(((i) as i32)))
                            .wrapping_add(((top) as i32))) as u8),
                        255u8,
                        None,
                        letterSpacing,
                        lineSpacing,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionTextsAtTop(
    windowId: u8,
    fontId: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut lineHeight = lineHeight;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        PrintMenuActionTextsAtPos(
            windowId,
            fontId,
            GetFontAttribute(fontId, 0u8),
            1u8,
            lineHeight,
            itemCount,
            menuActions,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMenuActionTexts(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    letterSpacing: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut u8,
    actionIds: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut letterSpacing = letterSpacing;
        let mut lineHeight = lineHeight;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        let mut actionIds = actionIds;
        let mut i: u8 = 0u8;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(fontId, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(fontId, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(fontId, 7u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (GetFontAttribute(fontId, 4u8)) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(letterSpacing);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(GetFontAttribute(fontId, 3u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(left);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(left);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((itemCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(
                        (((menuActions).wrapping_offset(
                            ((((actionIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 8,
                        ))
                        .cast::<*mut u8>())
                        .read(),
                    );
                    (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(
                        (((((lineHeight) as i32).wrapping_mul(((i) as i32)))
                            .wrapping_add(((top) as i32))) as u8),
                    );
                    (((&raw mut printer).cast::<u8>()).wrapping_add(9))
                        .write((((&raw mut printer).cast::<u8>()).wrapping_add(7)).read());
                    AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionTextsAtTopById(
    windowId: u8,
    fontId: u8,
    lineHeight: u8,
    itemCount: u8,
    menuActions: *mut u8,
    actionIds: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut lineHeight = lineHeight;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        let mut actionIds = actionIds;
        PrintMenuActionTexts(
            windowId,
            fontId,
            GetFontAttribute(fontId, 0u8),
            1u8,
            GetFontAttribute(fontId, 2u8),
            lineHeight,
            itemCount,
            menuActions,
            actionIds,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetWindowTemplateFields(
    template: *mut u8,
    bg: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
    baseBlock: u16,
) {
    unsafe {
        let mut template = template;
        let mut bg = bg;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        let mut baseBlock = baseBlock;
        (template).write(bg);
        ((template).wrapping_add(1)).write(left);
        ((template).wrapping_add(2)).write(top);
        ((template).wrapping_add(3)).write(width);
        ((template).wrapping_add(4)).write(height);
        ((template).wrapping_add(5)).write(paletteNum);
        ((template).wrapping_add(6).cast::<u16>()).write(baseBlock);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateWindowTemplate(
    bg: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
    baseBlock: u16,
) -> crate::c::Rec4<8> {
    unsafe {
        let mut bg = bg;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        let mut baseBlock = baseBlock;
        let mut template = crate::ffi::Align4([0u8; 8]);
        SetWindowTemplateFields(
            (&raw mut template).cast::<u8>(),
            bg,
            left,
            top,
            width,
            height,
            paletteNum,
            baseBlock,
        );
        return (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .read_unaligned();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddWindowParameterized(
    bg: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    paletteNum: u8,
    baseBlock: u16,
) -> u16 {
    unsafe {
        let mut bg = bg;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut paletteNum = paletteNum;
        let mut baseBlock = baseBlock;
        let mut template = crate::ffi::Align4([0u8; 8]);
        SetWindowTemplateFields(
            (&raw mut template).cast::<u8>(),
            bg,
            left,
            top,
            width,
            height,
            paletteNum,
            baseBlock,
        );
        return AddWindow((&raw mut template).cast::<u8>());
    }
}
pub(crate) unsafe extern "C" fn CreateYesNoMenuAtPos(
    window: *mut u8,
    fontId: u8,
    left: u8,
    top: u8,
    baseTileNum: u16,
    paletteNum: u8,
    initialCursorPos: u8,
) {
    unsafe {
        let mut window = window;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut baseTileNum = baseTileNum;
        let mut paletteNum = paletteNum;
        let mut initialCursorPos = initialCursorPos;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).write(((AddWindow(window)) as u8));
        DrawStdFrameWithCustomTileAndPalette(
            ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            baseTileNum,
            paletteNum,
        );
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gText_YesNo).cast::<u8>());
        (((&raw mut printer).cast::<u8>()).wrapping_add(4))
            .write(((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(
            ((((GetFontAttribute(fontId, 0u8)) as i32).wrapping_add(((left) as i32))) as u8),
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(top);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(7)).read());
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(fontId, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(fontId, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(fontId, 7u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (GetFontAttribute(fontId, 4u8)) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(GetFontAttribute(fontId, 2u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(GetFontAttribute(fontId, 3u8));
        AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
        InitMenuNormal(
            ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read(),
            fontId,
            left,
            top,
            GetFontAttribute(fontId, 1u8),
            2u8,
            initialCursorPos,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateYesNoMenuInTopLeft(
    window: *mut u8,
    fontId: u8,
    baseTileNum: u16,
    paletteNum: u8,
) {
    unsafe {
        let mut window = window;
        let mut fontId = fontId;
        let mut baseTileNum = baseTileNum;
        let mut paletteNum = paletteNum;
        CreateYesNoMenuAtPos(window, fontId, 0u8, 1u8, baseTileNum, paletteNum, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_ProcessInputNoWrapClearOnChoose() -> i8 {
    unsafe {
        let mut result: i8 = Menu_ProcessInputNoWrap();
        if ((result) as i32) != (-2i32) {
            EraseYesNoWindow();
        }
        return result;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EraseYesNoWindow() {
    unsafe {
        ClearStdWindowAndFrameToTransparent(
            ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
        );
        RemoveWindow(((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionGridText(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut columns = columns;
        let mut rows = rows;
        let mut menuActions = menuActions;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((rows) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((columns) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                AddTextPrinterParameterized(
                                    windowId,
                                    fontId,
                                    (((menuActions).wrapping_offset(
                                        ((((i) as i32).wrapping_mul(((columns) as i32)))
                                            .wrapping_add(((j) as i32)))
                                            as isize
                                            * 8,
                                    ))
                                    .cast::<*mut u8>())
                                    .read(),
                                    (((((width) as i32).wrapping_mul(((j) as i32)))
                                        .wrapping_add(((left) as i32)))
                                        as u8),
                                    (((((height) as i32).wrapping_mul(((i) as i32)))
                                        .wrapping_add(((top) as i32)))
                                        as u8),
                                    255u8,
                                    None,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionGridTextAtTop(
    windowId: u8,
    fontId: u8,
    width: u8,
    height: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut width = width;
        let mut height = height;
        let mut columns = columns;
        let mut rows = rows;
        let mut menuActions = menuActions;
        PrintMenuActionGridText(
            windowId,
            fontId,
            GetFontAttribute(fontId, 0u8),
            0u8,
            width,
            height,
            columns,
            rows,
            menuActions,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMenuActionGrid(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    optionWidth: u8,
    horizontalCount: u8,
    verticalCount: u8,
    menuActions: *mut u8,
    actionIds: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut optionWidth = optionWidth;
        let mut horizontalCount = horizontalCount;
        let mut verticalCount = verticalCount;
        let mut menuActions = menuActions;
        let mut actionIds = actionIds;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(fontId, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(fontId, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(fontId, 7u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (GetFontAttribute(fontId, 4u8)) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(GetFontAttribute(fontId, 2u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(GetFontAttribute(fontId, 3u8));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((verticalCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((horizontalCount) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(
                                    (((menuActions).wrapping_offset(
                                        ((((actionIds).wrapping_offset(
                                            ((((horizontalCount) as i32)
                                                .wrapping_mul(((i) as i32)))
                                            .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .cast::<*mut u8>())
                                    .read(),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(
                                    (((((optionWidth) as i32).wrapping_mul(((j) as i32)))
                                        .wrapping_add(((left) as i32)))
                                        as u8),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(
                                    (((((GetFontAttribute(fontId, 1u8)) as i32)
                                        .wrapping_mul(((i) as i32)))
                                    .wrapping_add(((top) as i32)))
                                        as u8),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(
                                    (((&raw mut printer).cast::<u8>()).wrapping_add(6)).read(),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(9)).write(
                                    (((&raw mut printer).cast::<u8>()).wrapping_add(7)).read(),
                                );
                                AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionGrid_TopLeft(
    windowId: u8,
    fontId: u8,
    optionWidth: u8,
    unused: u8,
    horizontalCount: u8,
    verticalCount: u8,
    menuActions: *mut u8,
    actionIds: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut optionWidth = optionWidth;
        let mut unused = unused;
        let mut horizontalCount = horizontalCount;
        let mut verticalCount = verticalCount;
        let mut menuActions = menuActions;
        let mut actionIds = actionIds;
        PrintMenuActionGrid(
            windowId,
            fontId,
            GetFontAttribute(fontId, 0u8),
            0u8,
            optionWidth,
            horizontalCount,
            verticalCount,
            menuActions,
            actionIds,
        );
    }
}
pub(crate) unsafe extern "C" fn InitMenuGrid(
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
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut optionWidth = optionWidth;
        let mut optionHeight = optionHeight;
        let mut columns = columns;
        let mut rows = rows;
        let mut numChoices = numChoices;
        let mut cursorPos = cursorPos;
        let mut pos: i32 = 0i32;
        ((&raw mut sMenu).cast::<u8>()).write(left);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).write(top);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).write(0i8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>())
            .write(((((numChoices) as i32).wrapping_sub(1i32)) as i8));
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).write(windowId);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).write(fontId);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(7)).write(optionWidth);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).write(optionHeight);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).write(columns);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(10)).write(rows);
        pos = ((cursorPos) as i32);
        if (pos < 0i32)
            || (pos
                > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32))
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(0i8);
        } else {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(((pos) as i8));
        }
        ChangeMenuGridCursorPosition(0i8, 0i8);
        return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
    }
}
pub(crate) unsafe extern "C" fn InitMenuGridDefaultCursorHeight(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    width: u8,
    columns: u8,
    rows: u8,
    cursorPos: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut columns = columns;
        let mut rows = rows;
        let mut cursorPos = cursorPos;
        let mut cursorHeight: u8 = GetMenuCursorDimensionByFont(fontId, 1u8);
        let mut numChoices: u8 = ((((columns) as i32).wrapping_mul(((rows) as i32))) as u8);
        return InitMenuGrid(
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
        );
    }
}
pub(crate) unsafe extern "C" fn MoveMenuGridCursor(oldCursorPos: u8, newCursorPos: u8) {
    unsafe {
        let mut oldCursorPos = oldCursorPos;
        let mut newCursorPos = newCursorPos;
        let mut cursorWidth: u8 = GetMenuCursorDimensionByFont(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).read(),
            0u8,
        );
        let mut cursorHeight: u8 = GetMenuCursorDimensionByFont(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).read(),
            1u8,
        );
        let mut xPos: u8 = ((((crate::c::rem_i32(
            ((oldCursorPos) as i32),
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
        ))
        .wrapping_mul((((((&raw mut sMenu).cast::<u8>()).wrapping_add(7)).read()) as i32)))
        .wrapping_add(((((&raw mut sMenu).cast::<u8>()).read()) as i32)))
            as u8);
        let mut yPos: u8 = ((((crate::c::div_i32(
            ((oldCursorPos) as i32),
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
        ))
        .wrapping_mul((((((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).read()) as i32)))
        .wrapping_add((((((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).read()) as i32)))
            as u8);
        FillWindowPixelRect(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).read(),
            17u8,
            ((xPos) as u16),
            ((yPos) as u16),
            ((cursorWidth) as u16),
            ((cursorHeight) as u16),
        );
        xPos = ((((crate::c::rem_i32(
            ((newCursorPos) as i32),
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
        ))
        .wrapping_mul((((((&raw mut sMenu).cast::<u8>()).wrapping_add(7)).read()) as i32)))
        .wrapping_add(((((&raw mut sMenu).cast::<u8>()).read()) as i32))) as u8);
        yPos = ((((crate::c::div_i32(
            ((newCursorPos) as i32),
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
        ))
        .wrapping_mul((((((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).read()) as i32)))
        .wrapping_add((((((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).read()) as i32)))
            as u8);
        AddTextPrinterParameterized(
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).read(),
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).read(),
            (&raw mut gText_SelectorArrow3).cast::<u8>(),
            xPos,
            yPos,
            0u8,
            None,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeMenuGridCursorPosition(deltaX: i8, deltaY: i8) -> u8 {
    unsafe {
        let mut deltaX = deltaX;
        let mut deltaY = deltaY;
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        if ((deltaX) as i32) != 0i32 {
            if (crate::c::rem_i32(
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as i32),
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
            ))
            .wrapping_add(((deltaX) as i32))
                < 0i32
            {
                let __p1 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_add(
                        (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32)
                            .wrapping_sub(1i32),
                    )) as i8),
                );
            } else {
                if (crate::c::rem_i32(
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read())
                        as i32),
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
                ))
                .wrapping_add(((deltaX) as i32))
                    >= (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32)
                {
                    (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(
                        (((crate::c::div_i32(
                            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read())
                                as i32),
                            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
                        ))
                        .wrapping_mul(
                            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
                        )) as i8),
                    );
                } else {
                    let __p2 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                    (__p2)
                        .write((((((__p2).read()) as i32).wrapping_add(((deltaX) as i32))) as i8));
                }
            }
        }
        if ((deltaY) as i32) != 0i32 {
            if (crate::c::div_i32(
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as i32),
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
            ))
            .wrapping_add(((deltaY) as i32))
                < 0i32
            {
                let __p3 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32)
                            .wrapping_mul(
                                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(10)).read())
                                    as i32)
                                    .wrapping_sub(1i32),
                            ),
                    )) as i8),
                );
            } else {
                if (crate::c::div_i32(
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read())
                        as i32),
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
                ))
                .wrapping_add(((deltaY) as i32))
                    >= (((((&raw mut sMenu).cast::<u8>()).wrapping_add(10)).read()) as i32)
                {
                    let __p4 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_sub(
                            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32)
                                .wrapping_mul(
                                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(10)).read())
                                        as i32)
                                        .wrapping_sub(1i32),
                                ),
                        )) as i8),
                    );
                } else {
                    let __p5 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                    (__p5).write(
                        (((((__p5).read()) as i32).wrapping_add(
                            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32)
                                .wrapping_mul(((deltaY) as i32)),
                        )) as i8),
                    );
                }
            }
        }
        if (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as i32)
            > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32)
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(((oldPos) as i8));
            return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        } else {
            MoveMenuGridCursor(
                oldPos,
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8),
            );
            return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ChangeGridMenuCursorPosition(deltaX: i8, deltaY: i8) -> u8 {
    unsafe {
        let mut deltaX = deltaX;
        let mut deltaY = deltaY;
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        if ((deltaX) as i32) != 0i32 {
            if ((crate::c::rem_i32(
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as i32),
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
            ))
            .wrapping_add(((deltaX) as i32))
                >= 0i32)
                && ((crate::c::rem_i32(
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read())
                        as i32),
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
                ))
                .wrapping_add(((deltaX) as i32))
                    < (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32))
            {
                let __p1 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                (__p1).write((((((__p1).read()) as i32).wrapping_add(((deltaX) as i32))) as i8));
            }
        }
        if ((deltaY) as i32) != 0i32 {
            if ((crate::c::div_i32(
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as i32),
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
            ))
            .wrapping_add(((deltaY) as i32))
                >= 0i32)
                && ((crate::c::div_i32(
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read())
                        as i32),
                    (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32),
                ))
                .wrapping_add(((deltaY) as i32))
                    < (((((&raw mut sMenu).cast::<u8>()).wrapping_add(10)).read()) as i32))
            {
                let __p2 = ((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (((((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).read()) as i32)
                            .wrapping_mul(((deltaY) as i32)),
                    )) as i8),
                );
            }
        }
        if (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as i32)
            > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32)
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(((oldPos) as i8));
            return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        } else {
            MoveMenuGridCursor(
                oldPos,
                (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8),
            );
            return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Menu_ProcessGridInput_NoSoundLimit() -> i8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    PlaySE(5u16);
                    ChangeMenuGridCursorPosition(0i8, (-1i8));
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        PlaySE(5u16);
                        ChangeMenuGridCursorPosition(0i8, 1i8);
                        return (-2i8);
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 32i32)
                            != 0)
                            || (((GetLRKeysPressed()) as i32) == 1i32)
                        {
                            PlaySE(5u16);
                            ChangeMenuGridCursorPosition((-1i8), 0i8);
                            return (-2i8);
                        } else {
                            if (((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0)
                                || (((GetLRKeysPressed()) as i32) == 2i32)
                            {
                                PlaySE(5u16);
                                ChangeMenuGridCursorPosition(1i8, 0i8);
                                return (-2i8);
                            }
                        }
                    }
                }
            }
        }
        return (-2i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Menu_ProcessGridInput() -> i8 {
    unsafe {
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    if ((oldPos) as i32) != ((ChangeGridMenuCursorPosition(0i8, (-1i8))) as i32) {
                        PlaySE(5u16);
                    }
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        if ((oldPos) as i32) != ((ChangeGridMenuCursorPosition(0i8, 1i8)) as i32) {
                            PlaySE(5u16);
                        }
                        return (-2i8);
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(46)
                            .cast::<u16>())
                        .read()) as i32)
                            & 32i32)
                            != 0)
                            || (((GetLRKeysPressed()) as i32) == 1i32)
                        {
                            if ((oldPos) as i32)
                                != ((ChangeGridMenuCursorPosition((-1i8), 0i8)) as i32)
                            {
                                PlaySE(5u16);
                            }
                            return (-2i8);
                        } else {
                            if (((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(46)
                                .cast::<u16>())
                            .read()) as i32)
                                & 16i32)
                                != 0)
                                || (((GetLRKeysPressed()) as i32) == 2i32)
                            {
                                if ((oldPos) as i32)
                                    != ((ChangeGridMenuCursorPosition(1i8, 0i8)) as i32)
                                {
                                    PlaySE(5u16);
                                }
                                return (-2i8);
                            }
                        }
                    }
                }
            }
        }
        return (-2i8);
    }
}
pub(crate) unsafe extern "C" fn Menu_ProcessGridInputRepeat_NoSoundLimit() -> i8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 240i32)
                    == 64i32
                {
                    PlaySE(5u16);
                    ChangeMenuGridCursorPosition(0i8, (-1i8));
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 240i32)
                        == 128i32
                    {
                        PlaySE(5u16);
                        ChangeMenuGridCursorPosition(0i8, 1i8);
                        return (-2i8);
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 240i32)
                            == 32i32)
                            || (((GetLRKeysPressedAndHeld()) as i32) == 1i32)
                        {
                            PlaySE(5u16);
                            ChangeMenuGridCursorPosition((-1i8), 0i8);
                            return (-2i8);
                        } else {
                            if (((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 240i32)
                                == 16i32)
                                || (((GetLRKeysPressedAndHeld()) as i32) == 2i32)
                            {
                                PlaySE(5u16);
                                ChangeMenuGridCursorPosition(1i8, 0i8);
                                return (-2i8);
                            }
                        }
                    }
                }
            }
        }
        return (-2i8);
    }
}
pub(crate) unsafe extern "C" fn Menu_ProcessGridInputRepeat() -> i8 {
    unsafe {
        let mut oldPos: u8 =
            (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            return (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read();
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                return (-1i8);
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 240i32)
                    == 64i32
                {
                    if ((oldPos) as i32) != ((ChangeGridMenuCursorPosition(0i8, (-1i8))) as i32) {
                        PlaySE(5u16);
                    }
                    return (-2i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(48)
                        .cast::<u16>())
                    .read()) as i32)
                        & 240i32)
                        == 128i32
                    {
                        if ((oldPos) as i32) != ((ChangeGridMenuCursorPosition(0i8, 1i8)) as i32) {
                            PlaySE(5u16);
                        }
                        return (-2i8);
                    } else {
                        if (((((((&raw mut gMain).cast::<u8>())
                            .wrapping_add(48)
                            .cast::<u16>())
                        .read()) as i32)
                            & 240i32)
                            == 32i32)
                            || (((GetLRKeysPressedAndHeld()) as i32) == 1i32)
                        {
                            if ((oldPos) as i32)
                                != ((ChangeGridMenuCursorPosition((-1i8), 0i8)) as i32)
                            {
                                PlaySE(5u16);
                            }
                            return (-2i8);
                        } else {
                            if (((((((&raw mut gMain).cast::<u8>())
                                .wrapping_add(48)
                                .cast::<u16>())
                            .read()) as i32)
                                & 240i32)
                                == 16i32)
                                || (((GetLRKeysPressedAndHeld()) as i32) == 2i32)
                            {
                                if ((oldPos) as i32)
                                    != ((ChangeGridMenuCursorPosition(1i8, 0i8)) as i32)
                                {
                                    PlaySE(5u16);
                                }
                                return (-2i8);
                            }
                        }
                    }
                }
            }
        }
        return (-2i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMenuInUpperLeftCorner(
    windowId: u8,
    itemCount: u8,
    initialCursorPos: u8,
    APressMuted: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut itemCount = itemCount;
        let mut initialCursorPos = initialCursorPos;
        let mut APressMuted = APressMuted;
        let mut pos: i32 = 0i32;
        ((&raw mut sMenu).cast::<u8>()).write(0u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).write(1u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).write(0i8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>())
            .write(((((itemCount) as i32).wrapping_sub(1i32)) as i8));
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).write(windowId);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).write(1u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).write(16u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(11)).write(APressMuted);
        pos = ((initialCursorPos) as i32);
        if (pos < 0i32)
            || (pos
                > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32))
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(0i8);
        } else {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(((pos) as i8));
        }
        return Menu_MoveCursor(0i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMenuInUpperLeftCornerNormal(
    windowId: u8,
    itemCount: u8,
    initialCursorPos: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut itemCount = itemCount;
        let mut initialCursorPos = initialCursorPos;
        return InitMenuInUpperLeftCorner(windowId, itemCount, initialCursorPos, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMenuTable(windowId: u8, itemCount: u8, menuActions: *mut u8) {
    unsafe {
        let mut windowId = windowId;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < ((itemCount) as u32)) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized(
                        windowId,
                        1u8,
                        (((menuActions).wrapping_offset(((i) as i32) as isize * 8))
                            .cast::<*mut u8>())
                        .read(),
                        8u8,
                        ((((i).wrapping_mul(16u32)).wrapping_add(1u32)) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMenuActionTextsInUpperLeftCorner(
    windowId: u8,
    itemCount: u8,
    menuActions: *mut u8,
    actionIds: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut itemCount = itemCount;
        let mut menuActions = menuActions;
        let mut actionIds = actionIds;
        let mut i: u8 = 0u8;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(1u8);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(1u8, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(1u8, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(1u8, 7u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (GetFontAttribute(1u8, 4u8)) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(8u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(8u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((itemCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(
                        (((menuActions).wrapping_offset(
                            ((((actionIds).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                                as isize
                                * 8,
                        ))
                        .cast::<*mut u8>())
                        .read(),
                    );
                    (((&raw mut printer).cast::<u8>()).wrapping_add(7))
                        .write((((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8));
                    (((&raw mut printer).cast::<u8>()).wrapping_add(9))
                        .write((((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8));
                    AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateYesNoMenu(
    window: *mut u8,
    baseTileNum: u16,
    paletteNum: u8,
    initialCursorPos: u8,
) {
    unsafe {
        let mut window = window;
        let mut baseTileNum = baseTileNum;
        let mut paletteNum = paletteNum;
        let mut initialCursorPos = initialCursorPos;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).write(((AddWindow(window)) as u8));
        DrawStdFrameWithCustomTileAndPalette(
            ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            baseTileNum,
            paletteNum,
        );
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>())
            .write((&raw mut gText_YesNo).cast::<u8>());
        (((&raw mut printer).cast::<u8>()).wrapping_add(4))
            .write(((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(1u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(8u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(1u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(7)).read());
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(1u8, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(1u8, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(1u8, 7u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (GetFontAttribute(1u8, 4u8)) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(0u8);
        AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
        InitMenuInUpperLeftCornerNormal(
            ((&raw mut sYesNoWindowId).cast::<u8>().cast::<u8>()).read(),
            2u8,
            initialCursorPos,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintMenuGridTable(
    windowId: u8,
    optionWidth: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut optionWidth = optionWidth;
        let mut columns = columns;
        let mut rows = rows;
        let mut menuActions = menuActions;
        let mut i: u32 = 0u32;
        let mut j: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < ((rows) as u32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u32;
                        'l3: loop {
                            if !(j < ((columns) as u32)) {
                                break 'l3;
                            }
                            'l4: {
                                AddTextPrinterParameterized(
                                    windowId,
                                    1u8,
                                    (((menuActions).wrapping_offset(
                                        ((((i).wrapping_mul(((columns) as u32))).wrapping_add(j))
                                            as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .cast::<*mut u8>())
                                    .read(),
                                    (((((optionWidth) as u32).wrapping_mul(j)).wrapping_add(8u32))
                                        as u8),
                                    ((((i).wrapping_mul(16u32)).wrapping_add(1u32)) as u8),
                                    255u8,
                                    None,
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn PrintMenuActionGridTextNoSpacing(
    windowId: u8,
    optionWidth: u8,
    columns: u8,
    rows: u8,
    menuActions: *mut u8,
    actionIds: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut optionWidth = optionWidth;
        let mut columns = columns;
        let mut rows = rows;
        let mut menuActions = menuActions;
        let mut actionIds = actionIds;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(1u8);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(1u8, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(1u8, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(1u8, 7u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (GetFontAttribute(1u8, 4u8)) as i32,
        );
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((rows) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((columns) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(
                                    (((menuActions).wrapping_offset(
                                        ((((actionIds).wrapping_offset(
                                            ((((columns) as i32).wrapping_mul(((i) as i32)))
                                                .wrapping_add(((j) as i32)))
                                                as isize,
                                        ))
                                        .read()) as i32)
                                            as isize
                                            * 8,
                                    ))
                                    .cast::<*mut u8>())
                                    .read(),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(
                                    (((((optionWidth) as i32).wrapping_mul(((j) as i32)))
                                        .wrapping_add(8i32))
                                        as u8),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(
                                    ((((16i32).wrapping_mul(((i) as i32))).wrapping_add(1i32))
                                        as u8),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(
                                    (((&raw mut printer).cast::<u8>()).wrapping_add(6)).read(),
                                );
                                (((&raw mut printer).cast::<u8>()).wrapping_add(9)).write(
                                    (((&raw mut printer).cast::<u8>()).wrapping_add(7)).read(),
                                );
                                AddTextPrinter((&raw mut printer).cast::<u8>(), 255u8, None);
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(windowId, 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitMenuActionGrid(
    windowId: u8,
    optionWidth: u8,
    columns: u8,
    rows: u8,
    initialCursorPos: u8,
) -> u8 {
    unsafe {
        let mut windowId = windowId;
        let mut optionWidth = optionWidth;
        let mut columns = columns;
        let mut rows = rows;
        let mut initialCursorPos = initialCursorPos;
        let mut pos: i32 = 0i32;
        ((&raw mut sMenu).cast::<u8>()).write(0u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(1)).write(1u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(3).cast::<i8>()).write(0i8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>())
            .write((((((columns) as i32).wrapping_mul(((rows) as i32))).wrapping_sub(1i32)) as i8));
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(5)).write(windowId);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(6)).write(1u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(7)).write(optionWidth);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(8)).write(16u8);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(9)).write(columns);
        (((&raw mut sMenu).cast::<u8>()).wrapping_add(10)).write(rows);
        pos = ((initialCursorPos) as i32);
        if (pos < 0i32)
            || (pos
                > (((((&raw mut sMenu).cast::<u8>()).wrapping_add(4).cast::<i8>()).read()) as i32))
        {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(0i8);
        } else {
            (((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).write(((pos) as i8));
        }
        ChangeMenuGridCursorPosition(0i8, 0i8);
        return (((((&raw mut sMenu).cast::<u8>()).wrapping_add(2).cast::<i8>()).read()) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearScheduledBgCopiesToVram() {
    unsafe {
        crate::c::memset(
            ((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>(),
            0i32,
            4u32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScheduleBgCopyTilemapToVram(bgId: u8) {
    unsafe {
        let mut bgId = bgId;
        ((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((bgId) as i32) as isize))
        .write(1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoScheduledBgTilemapCopiesToVram() {
    unsafe {
        if (((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).read()) as i32)
            == 1i32
        {
            CopyBgTilemapBufferToVram(0u8);
            (((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).write(0u8);
        }
        if ((((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
            .read()) as i32)
            == 1i32
        {
            CopyBgTilemapBufferToVram(1u8);
            ((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .write(0u8);
        }
        if ((((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
            .read()) as i32)
            == 1i32
        {
            CopyBgTilemapBufferToVram(2u8);
            ((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).wrapping_offset(2))
                .write(0u8);
        }
        if ((((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
            .read()) as i32)
            == 1i32
        {
            CopyBgTilemapBufferToVram(3u8);
            ((((&raw mut sScheduledBgCopiesToVram).cast::<u8>()).cast::<u8>()).wrapping_offset(3))
                .write(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTempTileDataBuffers() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(128u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sTempTileDataBuffer)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset((i) as isize))
                    .write(core::ptr::null_mut());
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sTempTileDataBufferIdx).cast::<u8>().cast::<u16>()).write(0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeTempTileDataBuffersIfPossible() -> u8 {
    unsafe {
        let mut i: i32 = 0i32;
        if !((IsDma3ManagerBusyWithBgCopy()) != 0) {
            if (((&raw mut sTempTileDataBufferIdx).cast::<u8>().cast::<u16>()).read()) != 0 {
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i
                            < ((((&raw mut sTempTileDataBufferIdx).cast::<u8>().cast::<u16>())
                                .read()) as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            Free(
                                ((((&raw mut sTempTileDataBuffer)
                                    .cast::<u8>()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset((i) as isize))
                                .read(),
                            );
                            ((((&raw mut sTempTileDataBuffer)
                                .cast::<u8>()
                                .cast::<*mut u8>())
                            .cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                            .write(core::ptr::null_mut());
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((&raw mut sTempTileDataBufferIdx).cast::<u8>().cast::<u16>()).write(0u16);
            }
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressAndCopyTileDataToVram(
    bgId: u8,
    src: *mut u8,
    size: u32,
    offset: u16,
    mode: u8,
) -> *mut u8 {
    unsafe {
        let mut bgId = bgId;
        let mut src = src;
        let mut size = size;
        let mut offset = offset;
        let mut mode = mode;
        let mut sizeOut: u32 = 0u32;
        if ((((&raw mut sTempTileDataBufferIdx).cast::<u8>().cast::<u16>()).read()) as u32)
            < crate::c::div_u32(128u32, 4u32)
        {
            let mut ptr: *mut u8 = malloc_and_decompress(src, &raw mut sizeOut);
            if !((size) != 0) {
                size = sizeOut;
            }
            if !(ptr).is_null() {
                copy_decompressed_tile_data_to_vram(bgId, ptr, ((size) as u16), offset, mode);
                ((((&raw mut sTempTileDataBuffer)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(
                    (({
                        let __p1 = (&raw mut sTempTileDataBufferIdx).cast::<u8>().cast::<u16>();
                        let __t2 = (__p1).read();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                        __t2
                    }) as i32) as isize,
                ))
                .write(ptr);
            }
            return ptr;
        }
        return core::ptr::null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DecompressAndLoadBgGfxUsingHeap(
    bgId: u8,
    src: *mut u8,
    size: u32,
    offset: u16,
    mode: u8,
) {
    unsafe {
        let mut bgId = bgId;
        let mut src = src;
        let mut size = size;
        let mut offset = offset;
        let mut mode = mode;
        let mut sizeOut: u32 = 0u32;
        let mut ptr: *mut u8 = malloc_and_decompress(src, &raw mut sizeOut);
        if !((size) != 0) {
            size = sizeOut;
        }
        if !(ptr).is_null() {
            let mut taskId: u8 =
                CreateTask(Some(task_free_buf_after_copying_tile_data_to_vram), 0u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((copy_decompressed_tile_data_to_vram(bgId, ptr, ((size) as u16), offset, mode))
                    as i16),
            );
            SetWordTaskArg(taskId, 1u8, ((ptr) as usize as u32));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn task_free_buf_after_copying_tile_data_to_vram(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((CheckForSpaceForDma3Request(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .read(),
        )) != 0)
        {
            Free(((GetWordTaskArg(taskId, 1u8)) as usize as *mut u8));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn malloc_and_decompress(src: *mut u8, size: *mut u32) -> *mut u8 {
    unsafe {
        let mut src = src;
        let mut size = size;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        let mut sizeAsBytes: *mut u8 = (size).cast::<u8>();
        let mut srcAsBytes: *mut u8 = src;
        (sizeAsBytes).write(((srcAsBytes).wrapping_offset(1)).read());
        ((sizeAsBytes).wrapping_offset(1)).write(((srcAsBytes).wrapping_offset(2)).read());
        ((sizeAsBytes).wrapping_offset(2)).write(((srcAsBytes).wrapping_offset(3)).read());
        ((sizeAsBytes).wrapping_offset(3)).write(0u8);
        ptr = Alloc((size).read());
        if !(ptr).is_null() {
            LZ77UnCompWram((src).cast::<u32>(), ptr);
        }
        return ptr;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn copy_decompressed_tile_data_to_vram(
    bgId: u8,
    src: *mut u8,
    size: u16,
    offset: u16,
    mode: u8,
) -> u16 {
    unsafe {
        let mut bgId = bgId;
        let mut src = src;
        let mut size = size;
        let mut offset = offset;
        let mut mode = mode;
        'l1: {
            let __sw1 = ((mode) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if __sw1 == 0i32 {
                return LoadBgTiles(bgId, src, size, offset);
            }
            if __sw1 == 1i32 {
                return LoadBgTilemap(bgId, src, size, offset);
            }
            if !__matched {
                return 65535u16;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBgTilemapPalette(
    bgId: u8,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
    palette: u8,
) {
    unsafe {
        let mut bgId = bgId;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut palette = palette;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut ptr: *mut u16 = (GetBgTilemapBuffer(bgId)).cast::<u16>();
        {
            i = top;
            'l1: loop {
                if !(((i) as i32) < ((top) as i32).wrapping_add(((height) as i32))) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = left;
                        'l3: loop {
                            if !(((j) as i32) < ((left) as i32).wrapping_add(((width) as i32))) {
                                break 'l3;
                            }
                            'l4: {
                                ((ptr).wrapping_offset(
                                    ((((i) as i32).wrapping_mul(32i32)).wrapping_add(((j) as i32)))
                                        as isize,
                                ))
                                .write(
                                    (((((((ptr).wrapping_offset(
                                        ((((i) as i32).wrapping_mul(32i32))
                                            .wrapping_add(((j) as i32)))
                                            as isize,
                                    ))
                                    .read()) as i32)
                                        & 4095i32)
                                        | (((palette) as i32) << 12))
                                        as u16),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyToBufferFromBgTilemap(
    bgId: u8,
    dest: *mut u16,
    left: u8,
    top: u8,
    width: u8,
    height: u8,
) {
    unsafe {
        let mut bgId = bgId;
        let mut dest = dest;
        let mut left = left;
        let mut top = top;
        let mut width = width;
        let mut height = height;
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut src: *mut u16 = (GetBgTilemapBuffer(bgId)).cast::<u16>();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((height) as i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0u8;
                        'l3: loop {
                            if !(((j) as i32) < ((width) as i32)) {
                                break 'l3;
                            }
                            'l4: {
                                ((dest).wrapping_offset(
                                    ((((i) as i32).wrapping_mul(((width) as i32)))
                                        .wrapping_add(((j) as i32)))
                                        as isize,
                                ))
                                .write(
                                    ((src).wrapping_offset(
                                        ((((((i) as i32).wrapping_add(((top) as i32)))
                                            .wrapping_mul(32i32))
                                        .wrapping_add(((j) as i32)))
                                        .wrapping_add(((left) as i32)))
                                            as isize,
                                    ))
                                    .read(),
                                );
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddValToTilemapBuffer(
    ptr: *mut u8,
    delta: i32,
    width: i32,
    height: i32,
    isAffine: u32,
) {
    unsafe {
        let mut ptr = ptr;
        let mut delta = delta;
        let mut width = width;
        let mut height = height;
        let mut isAffine = isAffine;
        let mut i: i32 = 0i32;
        let mut area: i32 = (width).wrapping_mul(height);
        if isAffine == 1u32 {
            let mut as8BPP: *mut u8 = ptr;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < area) {
                        break 'l1;
                    }
                    'l2: {
                        let __p1 = (as8BPP).wrapping_offset((i) as isize);
                        (__p1).write((((((__p1).read()) as i32).wrapping_add(delta)) as u8));
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            let mut as4BPP: *mut u16 = (ptr).cast::<u16>();
            {
                i = 0i32;
                'l3: loop {
                    if !(i < area) {
                        break 'l3;
                    }
                    'l4: {
                        ((as4BPP).wrapping_offset((i) as isize)).write(
                            (((((((as4BPP).wrapping_offset((i) as isize)).read()) as i32)
                                & 64512i32)
                                | (((((as4BPP).wrapping_offset((i) as isize)).read()) as i32)
                                    .wrapping_add(delta)
                                    & 1023i32)) as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetBgPositions() {
    unsafe {
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BgDmaFill(bg: u32, value: u8, offset: i32, size: i32) {
    unsafe {
        let mut bg = bg;
        let mut value = value;
        let mut offset = offset;
        let mut size = size;
        let mut temp: i32 = (if !((GetBgAttribute(((bg) as u8), 4u8)) != 0) {
            32i32
        } else {
            64i32
        });
        let mut addr: *mut u8 = (((((GetBgAttribute(((bg) as u8), 1u8)) as i32)
            .wrapping_mul(16384i32))
        .wrapping_add(
            (((GetBgAttribute(((bg) as u8), 10u8)) as i32).wrapping_add(offset)).wrapping_mul(temp),
        )) as usize as *mut u8);
        RequestDma3Fill(
            ((((((value) as i32) << 24) | (((value) as i32) << 16)) | (((value) as i32) << 8))
                | ((value) as i32)),
            (addr).wrapping_offset(100663296),
            (((size).wrapping_mul(temp)) as u16),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterParameterized3(
    windowId: u8,
    fontId: u8,
    left: u8,
    top: u8,
    color: *mut u8,
    speed: i8,
    str: *mut u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut color = color;
        let mut speed = speed;
        let mut str = str;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(left);
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(top);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(7)).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(GetFontAttribute(fontId, 2u8));
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(GetFontAttribute(fontId, 3u8));
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (((color).wrapping_offset(1)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            ((color).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (((color).wrapping_offset(2)).read()) as i32,
        );
        AddTextPrinter((&raw mut printer).cast::<u8>(), ((speed) as u8), None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterParameterized4(
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
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut left = left;
        let mut top = top;
        let mut letterSpacing = letterSpacing;
        let mut lineSpacing = lineSpacing;
        let mut color = color;
        let mut speed = speed;
        let mut str = str;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(left);
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(top);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(6)).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(9))
            .write((((&raw mut printer).cast::<u8>()).wrapping_add(7)).read());
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(letterSpacing);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(lineSpacing);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (((color).wrapping_offset(1)).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            ((color).read()) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (((color).wrapping_offset(2)).read()) as i32,
        );
        AddTextPrinter((&raw mut printer).cast::<u8>(), ((speed) as u8), None);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AddTextPrinterParameterized5(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    left: u8,
    top: u8,
    speed: u8,
    callback: Option<unsafe extern "C" fn(*mut u8, u16)>,
    letterSpacing: u8,
    lineSpacing: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut fontId = fontId;
        let mut str = str;
        let mut left = left;
        let mut top = top;
        let mut speed = speed;
        let mut callback = callback;
        let mut letterSpacing = letterSpacing;
        let mut lineSpacing = lineSpacing;
        let mut printer = crate::ffi::Align4([0u8; 16]);
        (((&raw mut printer).cast::<u8>()).cast::<*mut u8>()).write(str);
        (((&raw mut printer).cast::<u8>()).wrapping_add(4)).write(windowId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(5)).write(fontId);
        (((&raw mut printer).cast::<u8>()).wrapping_add(6)).write(left);
        (((&raw mut printer).cast::<u8>()).wrapping_add(7)).write(top);
        (((&raw mut printer).cast::<u8>()).wrapping_add(8)).write(left);
        (((&raw mut printer).cast::<u8>()).wrapping_add(9)).write(top);
        (((&raw mut printer).cast::<u8>()).wrapping_add(10)).write(letterSpacing);
        (((&raw mut printer).cast::<u8>()).wrapping_add(11)).write(lineSpacing);
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(12),
            4,
            4,
            (GetFontAttribute(fontId, 5u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            0,
            4,
            (GetFontAttribute(fontId, 6u8)) as i32,
        );
        crate::c::bf_write(
            ((&raw mut printer).cast::<u8>()).wrapping_add(13),
            4,
            4,
            (GetFontAttribute(fontId, 7u8)) as i32,
        );
        AddTextPrinter((&raw mut printer).cast::<u8>(), speed, callback);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintPlayerNameOnWindow(windowId: u8, src: *mut u8, x: u16, y: u16) {
    unsafe {
        let mut windowId = windowId;
        let mut src = src;
        let mut x = x;
        let mut y = y;
        let mut count: i32 = 0i32;
        'l1: loop {
            if !((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset((count) as isize))
            .read()) as i32)
                != 255i32)
            {
                break 'l1;
            }
            count = (count).wrapping_add(1);
        }
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), src);
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u8),
            ((y) as u8),
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn UnusedBlitBitmapRect(
    src: *mut u8,
    dst: *mut u8,
    srcX: u16,
    srcY: u16,
    dstX: u16,
    dstY: u16,
    width: u16,
    height: u16,
) {
    unsafe {
        let mut src = src;
        let mut dst = dst;
        let mut srcX = srcX;
        let mut srcY = srcY;
        let mut dstX = dstX;
        let mut dstY = dstY;
        let mut width = width;
        let mut height = height;
        let mut loopSrcY: i32 = 0i32;
        let mut loopDstY: i32 = 0i32;
        let mut loopSrcX: i32 = 0i32;
        let mut loopDstX: i32 = 0i32;
        let mut xEnd: i32 = 0i32;
        let mut yEnd: i32 = 0i32;
        let mut multiplierSrcY: i32 = 0i32;
        let mut multiplierDstY: i32 = 0i32;
        let mut pixelsSrc: *mut u8 = core::ptr::null_mut();
        let mut pixelsDst: *mut u8 = core::ptr::null_mut();
        let mut toOrr: u16 = 0u16;
        if (crate::c::bf_read((dst).wrapping_add(4), 0, 16, false) as u32)
            .wrapping_sub(((dstX) as u32))
            < ((width) as u32)
        {
            xEnd = ((((crate::c::bf_read((dst).wrapping_add(4), 0, 16, false) as u32)
                .wrapping_sub(((dstX) as u32)))
            .wrapping_add(((srcX) as u32))) as i32);
        } else {
            xEnd = ((width) as i32).wrapping_add(((srcX) as i32));
        }
        if (crate::c::bf_read((dst).wrapping_add(6), 0, 16, false) as u32)
            .wrapping_sub(((dstY) as u32))
            < ((height) as u32)
        {
            yEnd = (((((srcY) as u32)
                .wrapping_add((crate::c::bf_read((dst).wrapping_add(6), 0, 16, false) as u32)))
            .wrapping_sub(((dstY) as u32))) as i32);
        } else {
            yEnd = ((srcY) as i32).wrapping_add(((height) as i32));
        }
        multiplierSrcY = (((crate::c::bf_read((src).wrapping_add(4), 0, 16, false) as u32)
            .wrapping_add(crate::c::rem_u32(
                (crate::c::bf_read((src).wrapping_add(4), 0, 16, false) as u32),
                8u32,
            ))
            >> 3) as i32);
        multiplierDstY = (((crate::c::bf_read((dst).wrapping_add(4), 0, 16, false) as u32)
            .wrapping_add(crate::c::rem_u32(
                (crate::c::bf_read((dst).wrapping_add(4), 0, 16, false) as u32),
                8u32,
            ))
            >> 3) as i32);
        {
            loopSrcY = ((srcY) as i32);
            loopDstY = ((dstY) as i32);
            'l1: loop {
                if !(loopSrcY < yEnd) {
                    break 'l1;
                }
                'l2: {
                    {
                        loopSrcX = ((srcX) as i32);
                        loopDstX = ((dstX) as i32);
                        'l3: loop {
                            if !(loopSrcX < xEnd) {
                                break 'l3;
                            }
                            'l4: {
                                pixelsSrc = ((((((src).cast::<*mut u8>()).read())
                                    .wrapping_offset(((loopSrcX >> 1) & 3i32) as isize))
                                .wrapping_offset(((loopSrcX >> 3) << 5) as isize))
                                .wrapping_offset(
                                    ((loopSrcY >> 3).wrapping_mul(multiplierSrcY) << 5) as isize,
                                ))
                                .wrapping_offset(
                                    ((((loopSrcY << 29) as u32) >> 27) as i32) as isize,
                                );
                                pixelsDst = ((((((dst).cast::<*mut u8>()).read())
                                    .wrapping_offset(((loopDstX >> 1) & 3i32) as isize * 1))
                                .wrapping_offset(((loopDstX >> 3) << 5) as isize * 1))
                                .wrapping_offset(
                                    ((loopDstY >> 3).wrapping_mul(multiplierDstY) << 5) as isize
                                        * 1,
                                ))
                                .wrapping_offset(
                                    ((((loopDstY << 29) as u32) >> 27) as i32) as isize * 1,
                                );
                                if (((pixelsDst) as usize as u32) & 1u32) != 0 {
                                    pixelsDst = (pixelsDst).wrapping_offset(-1);
                                    if (loopDstX & 1i32) != 0 {
                                        toOrr = ((pixelsDst).cast::<u16>()).read_volatile();
                                        toOrr = ((((toOrr) as i32) & 4095i32) as u16);
                                        if (loopSrcX & 1i32) != 0 {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 240i32) << 8))
                                                as u16);
                                        } else {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 15i32) << 12))
                                                as u16);
                                        }
                                    } else {
                                        toOrr = ((pixelsDst).cast::<u16>()).read_volatile();
                                        toOrr = ((((toOrr) as i32) & 61695i32) as u16);
                                        if (loopSrcX & 1i32) != 0 {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 240i32) << 4))
                                                as u16);
                                        } else {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 15i32) << 8))
                                                as u16);
                                        }
                                    }
                                } else {
                                    if (loopDstX & 1i32) != 0 {
                                        toOrr = ((pixelsDst).cast::<u16>()).read_volatile();
                                        toOrr = ((((toOrr) as i32) & 65295i32) as u16);
                                        if (loopSrcX & 1i32) != 0 {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 240i32) << 0))
                                                as u16);
                                        } else {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 15i32) << 4))
                                                as u16);
                                        }
                                    } else {
                                        toOrr = ((pixelsDst).cast::<u16>()).read_volatile();
                                        toOrr = ((((toOrr) as i32) & 65520i32) as u16);
                                        if (loopSrcX & 1i32) != 0 {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 240i32) >> 4))
                                                as u16);
                                        } else {
                                            toOrr = ((((toOrr) as i32)
                                                | (((((pixelsSrc).read()) as i32) & 15i32) >> 0))
                                                as u16);
                                        }
                                    }
                                }
                                crate::c::volatile_write((pixelsDst).cast::<u16>(), toOrr);
                            }
                            loopSrcX = (loopSrcX).wrapping_add(1);
                            loopDstX = (loopDstX).wrapping_add(1);
                        }
                    }
                }
                loopSrcY = (loopSrcY).wrapping_add(1);
                loopDstY = (loopDstY).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LoadMonIconPalAtOffset(palOffset: u8, speciesId: u16) {
    unsafe {
        let mut palOffset = palOffset;
        let mut speciesId = speciesId;
        LoadPalette(
            (GetValidMonIconPalettePtr(speciesId)).cast::<u8>(),
            ((palOffset) as u16),
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawMonIconAtPos(
    windowId: u8,
    speciesId: u16,
    personality: u32,
    x: u16,
    y: u16,
) {
    unsafe {
        let mut windowId = windowId;
        let mut speciesId = speciesId;
        let mut personality = personality;
        let mut x = x;
        let mut y = y;
        BlitBitmapToWindow(
            windowId,
            GetMonIconPtr(speciesId, personality, 1u32),
            x,
            y,
            32u16,
            32u16,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ListMenuLoadStdPalAt(palOffset: u8, palId: u8) {
    unsafe {
        let mut palOffset = palOffset;
        let mut palId = palId;
        let mut palette: *mut u16 = core::ptr::null_mut();
        'l1: {
            let __sw1 = ((palId) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 || !__matched {
                palette = ((&raw mut gMenuInfoElements1_Pal).cast::<u16>()).cast::<u16>();
                break 'l1;
            }
            if __sw1 == 1i32 {
                palette = ((&raw mut gMenuInfoElements2_Pal).cast::<u16>()).cast::<u16>();
                break 'l1;
            }
            if __sw1 == 2i32 {
                palette = ((&raw mut gMenuInfoElements3_Pal).cast::<u16>()).cast::<u16>();
                break 'l1;
            }
        }
        LoadPalette((palette).cast::<u8>(), ((palOffset) as u16), 32u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BlitMenuInfoIcon(windowId: u8, iconId: u8, x: u16, y: u16) {
    unsafe {
        let mut windowId = windowId;
        let mut iconId = iconId;
        let mut x = x;
        let mut y = y;
        BlitBitmapRectToWindow(
            windowId,
            ((&raw mut gMenuInfoElements_Gfx).cast::<u8>()).wrapping_offset(
                ((((((((&raw const sMenuInfoIcons).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((iconId) as i32) as isize * 4))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    .wrapping_mul(32i32)) as isize,
            ),
            0u16,
            0u16,
            128u16,
            128i32,
            x,
            y,
            ((((((&raw const sMenuInfoIcons).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((iconId) as i32) as isize * 4))
            .read()) as u16),
            (((((((&raw const sMenuInfoIcons).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((iconId) as i32) as isize * 4))
            .wrapping_add(1))
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferSaveMenuText(textId: u8, dest: *mut u8, color: u8) {
    unsafe {
        let mut textId = textId;
        let mut dest = dest;
        let mut color = color;
        let mut curFlag: i32 = 0i32;
        let mut flagCount: i32 = 0i32;
        let mut endOfString: *mut u8 = core::ptr::null_mut();
        let mut string: *mut u8 = dest;
        ({
            let __t1 = string;
            string = (string).wrapping_offset(1);
            __t1
        })
        .write(252u8);
        ({
            let __t2 = string;
            string = (string).wrapping_offset(1);
            __t2
        })
        .write(1u8);
        ({
            let __t3 = string;
            string = (string).wrapping_offset(1);
            __t3
        })
        .write(color);
        ({
            let __t4 = string;
            string = (string).wrapping_offset(1);
            __t4
        })
        .write(252u8);
        ({
            let __t5 = string;
            string = (string).wrapping_offset(1);
            __t5
        })
        .write(3u8);
        ({
            let __t6 = string;
            string = (string).wrapping_offset(1);
            __t6
        })
        .write(((((color) as i32).wrapping_add(1i32)) as u8));
        'l1: {
            let __sw7 = ((textId) as i32);
            if __sw7 == 0i32 {
                StringCopy(
                    string,
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                );
                break 'l1;
            }
            if __sw7 == 1i32 {
                if (IsNationalPokedexEnabled()) != 0 {
                    string = ConvertIntToDecimalStringN(
                        string,
                        ((GetNationalPokedexCount(1u8)) as i32),
                        0i32,
                        3u8,
                    );
                } else {
                    string = ConvertIntToDecimalStringN(
                        string,
                        ((GetHoennPokedexCount(1u8)) as i32),
                        0i32,
                        3u8,
                    );
                }
                (string).write(255u8);
                break 'l1;
            }
            if __sw7 == 2i32 {
                string = ConvertIntToDecimalStringN(
                    string,
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(14)
                        .cast::<u16>())
                    .read()) as i32),
                    0i32,
                    3u8,
                );
                ({
                    let __t8 = string;
                    string = (string).wrapping_offset(1);
                    __t8
                })
                .write(240u8);
                ConvertIntToDecimalStringN(
                    string,
                    ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(16))
                        .read()) as i32),
                    2i32,
                    2u8,
                );
                break 'l1;
            }
            if __sw7 == 3i32 {
                GetMapNameGeneric(
                    string,
                    (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as u16),
                );
                break 'l1;
            }
            if __sw7 == 4i32 {
                {
                    curFlag = 2151i32;
                    flagCount = 0i32;
                    endOfString = (string).wrapping_offset(1);
                    'l2: loop {
                        if !(curFlag < 2159i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (FlagGet(((curFlag) as u16))) != 0 {
                                flagCount = (flagCount).wrapping_add(1);
                            }
                        }
                        curFlag = (curFlag).wrapping_add(1);
                    }
                }
                (string).write((((flagCount).wrapping_add(161i32)) as u8));
                (endOfString).write(255u8);
                break 'l1;
            }
        }
    }
}
