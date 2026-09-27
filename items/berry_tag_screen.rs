//! Translated from `src/berry_tag_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBackgroundTemplates sFontPalette sTextColors sWindowTemplates sBerryFirmnessStrings
#[allow(unused_imports)]
use crate::data::berry_tag_screen::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBerryTag: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut gBagPosition: u8;
    static mut gBerryCheckCirclePaletteTable: u8;
    static mut gBerryCheckCircleSpriteSheet: u8;
    static mut gBerryCheck_Gfx: u8;
    static mut gBerryCheck_Pal: u8;
    static mut gBerryTag_Gfx: u8;
    static mut gBerryTag_Tilemap: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BerryTag: u8;
    static mut gText_FirmSlash: u8;
    static mut gText_NumberVar1Var2: u8;
    static mut gText_SizeSlash: u8;
    static mut gText_ThreeMarks: u8;
    static mut gText_Var1DotVar2: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
    fn AllocZeroed(a0: u32) -> *mut u8;
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
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoScheduledBgTilemapCopiesToVram();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut u8);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeBerryTagSpritePalette();
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBerryInfo(a0: u8) -> *mut u8;
    fn GetBgTilemapBuffer(a0: u8) -> *mut u8;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn ItemIdToBerryType(a0: u16) -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadCompressedSpritePalette(a0: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
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
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
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
    unsafe {
        ((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(6156u32));
        ((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6144)
            .cast::<u16>())
        .write(((ItemIdToBerryType(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as u16));
        SetMainCallback2(Some(CB2_InitBerryTagScreen));
    }
}
pub(crate) unsafe extern "C" fn CB2_BerryTagScreen() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VblankCB() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn CB2_InitBerryTagScreen() {
    unsafe {
        'l1: loop {
            if !((1i32) != 0) {
                break 'l1;
            }
            if ((MenuHelpers_ShouldWaitForLinkRecv()) as i32) == 1i32 {
                break 'l1;
            }
            if ((InitBerryTagScreen()) as i32) == 1i32 {
                break 'l1;
            }
            if ((MenuHelpers_IsLinkActive()) as i32) == 1i32 {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitBerryTagScreen() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = (((((&raw mut gMain).cast::<u8>()).wrapping_add(1080)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 10i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32;
            if __sw1 == 0i32 {
                SetVBlankHBlankCallbacksToNull();
                ResetVramOamAndBgCntRegs();
                ClearScheduledBgCopiesToVram();
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ScanlineEffect_Stop();
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetPaletteFade();
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (1u16) as i32,
                );
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetSpriteData();
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                FreeAllSpritePalettes();
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if !((MenuHelpers_IsLinkActive()) != 0) {
                    ResetTasks();
                }
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                HandleInitBackgrounds();
                ((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6152)
                    .cast::<u16>())
                .write(0u16);
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                if (LoadBerryTagGfx()) != 0 {
                    let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                HandleInitWindows();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                AddBerryTagTextToBg0();
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                PrintAllBerryData();
                let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 11i32 {
                CreateBerrySprite();
                let __p13 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 12i32 {
                CreateFlavorCircleSprites();
                SetFlavorCirclesVisiblity();
                let __p14 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 13i32 {
                CreateTask(Some(Task_HandleInput), 0u8);
                let __p15 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 14i32 {
                BlendPalettes(4294967295u32, 16u8, 0u16);
                let __p16 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 15i32 {
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                crate::c::bf_write(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(8),
                    7,
                    1,
                    (0u16) as i32,
                );
                let __p17 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p17).write(((__p17).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                SetVBlankCallback(Some(VblankCB));
                SetMainCallback2(Some(CB2_BerryTagScreen));
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HandleInitBackgrounds() {
    unsafe {
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBackgroundTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(16u32, 4u32)) as u8),
        );
        SetBgTilemapBuffer(
            2u8,
            (((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .cast::<u16>())
            .cast::<u8>(),
        );
        SetBgTilemapBuffer(
            3u8,
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(2048))
            .cast::<u16>())
            .cast::<u8>(),
        );
        ResetAllBgsCoordinates();
        ScheduleBgCopyTilemapToVram(2u8);
        ScheduleBgCopyTilemapToVram(3u8);
        SetGpuReg(0u8, 4160u16);
        SetGpuReg(80u8, 0u16);
        ShowBg(0u8);
        ShowBg(1u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
pub(crate) unsafe extern "C" fn LoadBerryTagGfx() -> u8 {
    unsafe {
        let mut i: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6152)
                .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32;
            if __sw1 == 0i32 {
                ResetTempTileDataBuffers();
                DecompressAndCopyTileDataToVram(
                    2u8,
                    (((&raw mut gBerryCheck_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                let __p2 = (((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6152)
                    .cast::<u16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((FreeTempTileDataBuffersIfPossible()) as i32) != 1i32 {
                    LZDecompressWram(
                        ((&raw mut gBerryTag_Gfx).cast::<u32>()).cast::<u32>(),
                        (((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .cast::<u16>())
                        .cast::<u8>(),
                    );
                    let __p3 = (((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6152)
                        .cast::<u16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                LZDecompressWram(
                    ((&raw mut gBerryTag_Tilemap).cast::<u32>()).cast::<u32>(),
                    ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .cast::<u8>())
                    .wrapping_offset(4096))
                    .cast::<u16>())
                    .cast::<u8>(),
                );
                let __p4 = (((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6152)
                    .cast::<u16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32)
                    == 0i32
                {
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as u32) < crate::c::div_u32(2048u32, 2u32)) {
                                break 'l2;
                            }
                            'l3: {
                                (((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(2048))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(16450u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                } else {
                    {
                        i = 0u16;
                        'l4: loop {
                            if !(((i) as u32) < crate::c::div_u32(2048u32, 2u32)) {
                                break 'l4;
                            }
                            'l5: {
                                (((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>())
                                    .read())
                                .cast::<u8>())
                                .wrapping_offset(2048))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(20546u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                let __p5 = (((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6152)
                    .cast::<u16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                LoadCompressedPalette(
                    ((&raw mut gBerryCheck_Pal).cast::<u32>()).cast::<u32>(),
                    0u16,
                    192u16,
                );
                let __p6 = (((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6152)
                    .cast::<u16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                LoadCompressedSpriteSheet((&raw mut gBerryCheckCircleSpriteSheet).cast::<u8>());
                let __p7 = (((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6152)
                    .cast::<u16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                LoadCompressedSpritePalette((&raw mut gBerryCheckCirclePaletteTable).cast::<u8>());
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn HandleInitWindows() {
    unsafe {
        let mut i: u16 = 0u16;
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadPalette(
            (((&raw const sFontPalette)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < (crate::c::div_u32(40u32, 8u32)).wrapping_sub(1u32)) {
                    break 'l1;
                }
                'l2: {
                    PutWindowTilemap(((i) as u8));
                }
                i = (i).wrapping_add(1);
            }
        }
        ScheduleBgCopyTilemapToVram(0u8);
        ScheduleBgCopyTilemapToVram(1u8);
    }
}
pub(crate) unsafe extern "C" fn PrintTextInBerryTagScreen(
    windowId: u8,
    text: *mut u8,
    x: u8,
    y: u8,
    speed: i32,
    colorStructId: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut text = text;
        let mut x = x;
        let mut y = y;
        let mut speed = speed;
        let mut colorStructId = colorStructId;
        AddTextPrinterParameterized4(
            windowId,
            1u8,
            x,
            y,
            0u8,
            0u8,
            ((((&raw const sTextColors).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((colorStructId) as i32) as isize * 3))
            .cast::<u8>(),
            ((speed) as i8),
            text,
        );
    }
}
pub(crate) unsafe extern "C" fn AddBerryTagTextToBg0() {
    unsafe {
        crate::c::memcpy(
            GetBgTilemapBuffer(0u8),
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
                .wrapping_offset(4096))
            .cast::<u16>())
            .cast::<u8>(),
            2048u32,
        );
        FillWindowPixelBuffer(3u8, 255u8);
        PrintTextInBerryTagScreen(
            3u8,
            (&raw mut gText_BerryTag).cast::<u8>(),
            ((GetStringCenterAlignXOffset(1i32, (&raw mut gText_BerryTag).cast::<u8>(), 64i32))
                as u8),
            1u8,
            0i32,
            1u8,
        );
        PutWindowTilemap(3u8);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintAllBerryData() {
    unsafe {
        PrintBerryNumberAndName();
        PrintBerrySize();
        PrintBerryFirmness();
        PrintBerryDescription1();
        PrintBerryDescription2();
    }
}
pub(crate) unsafe extern "C" fn PrintBerryNumberAndName() {
    unsafe {
        let mut berry: *mut u8 = GetBerryInfo(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as u8),
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as i32),
            2i32,
            2u8,
        );
        StringCopy((&raw mut gStringVar2).cast::<u8>(), (berry).cast::<u8>());
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_NumberVar1Var2).cast::<u8>(),
        );
        PrintTextInBerryTagScreen(
            0u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            0i32,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintBerrySize() {
    unsafe {
        let mut berry: *mut u8 = GetBerryInfo(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as u8),
        );
        AddTextPrinterParameterized(
            1u8,
            1u8,
            (&raw mut gText_SizeSlash).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        if ((((berry).wrapping_add(8).cast::<u16>()).read()) as i32) != 0i32 {
            let mut inches: u32 = 0u32;
            let mut fraction: u32 = 0u32;
            inches = ((crate::c::div_i32(
                (1000i32).wrapping_mul(((((berry).wrapping_add(8).cast::<u16>()).read()) as i32)),
                254i32,
            )) as u32);
            if crate::c::rem_u32(inches, 10u32) > 4u32 {
                inches = (inches).wrapping_add(10u32);
            }
            fraction = crate::c::div_u32(crate::c::rem_u32(inches, 100u32), 10u32);
            inches = crate::c::div_u32(inches, 100u32);
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar1).cast::<u8>(),
                ((inches) as i32),
                0i32,
                2u8,
            );
            ConvertIntToDecimalStringN(
                (&raw mut gStringVar2).cast::<u8>(),
                ((fraction) as i32),
                0i32,
                2u8,
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Var1DotVar2).cast::<u8>(),
            );
            AddTextPrinterParameterized(
                1u8,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                40u8,
                1u8,
                0u8,
                None,
            );
        } else {
            AddTextPrinterParameterized(
                1u8,
                1u8,
                (&raw mut gText_ThreeMarks).cast::<u8>(),
                40u8,
                1u8,
                0u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintBerryFirmness() {
    unsafe {
        let mut berry: *mut u8 = GetBerryInfo(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as u8),
        );
        AddTextPrinterParameterized(
            1u8,
            1u8,
            (&raw mut gText_FirmSlash).cast::<u8>(),
            0u8,
            17u8,
            255u8,
            None,
        );
        if ((((berry).wrapping_add(7)).read()) as i32) != 0i32 {
            AddTextPrinterParameterized(
                1u8,
                1u8,
                ((((&raw const sBerryFirmnessStrings)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<*mut u8>())
                .cast::<*mut u8>())
                .wrapping_offset(
                    (((((berry).wrapping_add(7)).read()) as i32).wrapping_sub(1i32)) as isize,
                ))
                .read(),
                40u8,
                17u8,
                0u8,
                None,
            );
        } else {
            AddTextPrinterParameterized(
                1u8,
                1u8,
                (&raw mut gText_ThreeMarks).cast::<u8>(),
                40u8,
                17u8,
                0u8,
                None,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintBerryDescription1() {
    unsafe {
        let mut berry: *mut u8 = GetBerryInfo(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as u8),
        );
        AddTextPrinterParameterized(
            2u8,
            1u8,
            ((berry).wrapping_add(12).cast::<*mut u8>()).read(),
            0u8,
            1u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn PrintBerryDescription2() {
    unsafe {
        let mut berry: *mut u8 = GetBerryInfo(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as u8),
        );
        AddTextPrinterParameterized(
            2u8,
            1u8,
            ((berry).wrapping_add(16).cast::<*mut u8>()).read(),
            0u8,
            17u8,
            0u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn CreateBerrySprite() {
    unsafe {
        ((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6146)).write(
            CreateBerryTagSprite(
                ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6144)
                    .cast::<u16>())
                .read()) as i32)
                    .wrapping_sub(1i32)) as u8),
                56i16,
                64i16,
            ),
        );
    }
}
pub(crate) unsafe extern "C" fn DestroyBerrySprite() {
    unsafe {
        DestroySprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                    .wrapping_add(6146))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        FreeBerryTagSpritePalette();
    }
}
pub(crate) unsafe extern "C" fn CreateFlavorCircleSprites() {
    unsafe {
        (((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6147))
            .cast::<u8>())
        .write(CreateBerryFlavorCircleSprite(64i16));
        ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6147))
            .cast::<u8>())
        .wrapping_offset(1))
        .write(CreateBerryFlavorCircleSprite(104i16));
        ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6147))
            .cast::<u8>())
        .wrapping_offset(2))
        .write(CreateBerryFlavorCircleSprite(144i16));
        ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6147))
            .cast::<u8>())
        .wrapping_offset(3))
        .write(CreateBerryFlavorCircleSprite(184i16));
        ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6147))
            .cast::<u8>())
        .wrapping_offset(4))
        .write(CreateBerryFlavorCircleSprite(224i16));
    }
}
pub(crate) unsafe extern "C" fn SetFlavorCirclesVisiblity() {
    unsafe {
        let mut berry: *mut u8 = GetBerryInfo(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                .wrapping_add(6144)
                .cast::<u16>())
            .read()) as u8),
        );
        if (((berry).wrapping_add(21)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        if (((berry).wrapping_add(22)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        if (((berry).wrapping_add(23)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        if (((berry).wrapping_add(24)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(3))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        if (((berry).wrapping_add(25)).read()) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                        .wrapping_add(6147))
                    .cast::<u8>())
                    .wrapping_offset(4))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyFlavorCircleSprites() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                                .wrapping_add(6147))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PrepareToCloseBerryTagScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(5u16);
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_CloseBerryTagScreen));
    }
}
pub(crate) unsafe extern "C" fn Task_CloseBerryTagScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyBerrySprite();
            DestroyFlavorCircleSprites();
            Free(((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read());
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_ReturnToBagMenuPocket));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            let mut arrowKeys: u16 = (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(48)
                .cast::<u16>())
            .read()) as i32)
                & 240i32) as u16);
            if ((arrowKeys) as i32) == 64i32 {
                TryChangeDisplayedBerry(taskId, (-1i8));
            } else {
                if ((arrowKeys) as i32) == 128i32 {
                    TryChangeDisplayedBerry(taskId, 1i8);
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 3i32)
                        != 0
                    {
                        PrepareToCloseBerryTagScreen(taskId);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryChangeDisplayedBerry(taskId: u8, toMove: i8) {
    unsafe {
        let mut taskId = taskId;
        let mut toMove = toMove;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut currPocketPosition: i16 =
            (((((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18)).cast::<u16>())
                .wrapping_offset(3))
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8)).cast::<u16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16);
        let mut newPocketPosition: u32 =
            ((((currPocketPosition) as i32).wrapping_add(((toMove) as i32))) as u32);
        if (newPocketPosition < 46u32)
            && (((BagGetItemIdByPocketPosition(4u8, ((newPocketPosition) as u16))) as i32) != 0i32)
        {
            if ((toMove) as i32) < 0i32 {
                ((data).wrapping_offset(1)).write(2i16);
            } else {
                ((data).wrapping_offset(1)).write(1i16);
            }
            (data).write(0i16);
            PlaySE(5u16);
            HandleBagCursorPositionChange(toMove);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DisplayAnotherBerry));
        }
    }
}
pub(crate) unsafe extern "C" fn HandleBagCursorPositionChange(toMove: i8) {
    unsafe {
        let mut toMove = toMove;
        let mut scrollPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(18))
            .cast::<u16>())
        .wrapping_offset(3);
        let mut cursorPos: *mut u16 = ((((&raw mut gBagPosition).cast::<u8>()).wrapping_add(8))
            .cast::<u16>())
        .wrapping_offset(3);
        if ((toMove) as i32) > 0i32 {
            if ((((cursorPos).read()) as i32) < 4i32)
                || (((BagGetItemIdByPocketPosition(
                    4u8,
                    (((((scrollPos).read()) as i32).wrapping_add(8i32)) as u16),
                )) as i32)
                    == 0i32)
            {
                (cursorPos).write(
                    (((((cursorPos).read()) as i32).wrapping_add(((toMove) as i32))) as u16),
                );
            } else {
                (scrollPos).write(
                    (((((scrollPos).read()) as i32).wrapping_add(((toMove) as i32))) as u16),
                );
            }
        } else {
            if ((((cursorPos).read()) as i32) > 3i32) || ((((scrollPos).read()) as i32) == 0i32) {
                (cursorPos).write(
                    (((((cursorPos).read()) as i32).wrapping_add(((toMove) as i32))) as u16),
                );
            } else {
                (scrollPos).write(
                    (((((scrollPos).read()) as i32).wrapping_add(((toMove) as i32))) as u16),
                );
            }
        }
        ((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
            .wrapping_add(6144)
            .cast::<u16>())
        .write(
            ((ItemIdToBerryType(BagGetItemIdByPocketPosition(
                4u8,
                (((((scrollPos).read()) as i32).wrapping_add((((cursorPos).read()) as i32)))
                    as u16),
            ))) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayAnotherBerry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut y: i16 = 0i16;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (data).write((((((data).read()) as i32).wrapping_add(16i32)) as i16));
        (data).write((((((data).read()) as i32) & 255i32) as i16));
        if ((((data).wrapping_offset(1)).read()) as i32) == 1i32 {
            'l1: {
                let __sw1 = (((data).read()) as i32);
                if __sw1 == 48i32 {
                    FillWindowPixelBuffer(0u8, 0u8);
                    break 'l1;
                }
                if __sw1 == 64i32 {
                    PrintBerryNumberAndName();
                    break 'l1;
                }
                if __sw1 == 80i32 {
                    DestroyBerrySprite();
                    CreateBerrySprite();
                    break 'l1;
                }
                if __sw1 == 96i32 {
                    FillWindowPixelBuffer(1u8, 0u8);
                    break 'l1;
                }
                if __sw1 == 112i32 {
                    PrintBerrySize();
                    break 'l1;
                }
                if __sw1 == 128i32 {
                    PrintBerryFirmness();
                    break 'l1;
                }
                if __sw1 == 144i32 {
                    SetFlavorCirclesVisiblity();
                    break 'l1;
                }
                if __sw1 == 160i32 {
                    FillWindowPixelBuffer(2u8, 0u8);
                    break 'l1;
                }
                if __sw1 == 176i32 {
                    PrintBerryDescription1();
                    break 'l1;
                }
                if __sw1 == 192i32 {
                    PrintBerryDescription2();
                    break 'l1;
                }
            }
        } else {
            'l2: {
                let __sw2 = (((data).read()) as i32);
                if __sw2 == 48i32 {
                    FillWindowPixelBuffer(2u8, 0u8);
                    break 'l2;
                }
                if __sw2 == 64i32 {
                    PrintBerryDescription2();
                    break 'l2;
                }
                if __sw2 == 80i32 {
                    PrintBerryDescription1();
                    break 'l2;
                }
                if __sw2 == 96i32 {
                    SetFlavorCirclesVisiblity();
                    break 'l2;
                }
                if __sw2 == 112i32 {
                    FillWindowPixelBuffer(1u8, 0u8);
                    break 'l2;
                }
                if __sw2 == 128i32 {
                    PrintBerryFirmness();
                    break 'l2;
                }
                if __sw2 == 144i32 {
                    PrintBerrySize();
                    break 'l2;
                }
                if __sw2 == 160i32 {
                    DestroyBerrySprite();
                    CreateBerrySprite();
                    break 'l2;
                }
                if __sw2 == 176i32 {
                    FillWindowPixelBuffer(0u8, 0u8);
                    break 'l2;
                }
                if __sw2 == 192i32 {
                    PrintBerryNumberAndName();
                    break 'l2;
                }
            }
        }
        if ((((data).wrapping_offset(1)).read()) as i32) == 1i32 {
            y = (((((data).read()) as i32).wrapping_neg()) as i16);
        } else {
            y = (data).read();
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read()).wrapping_add(6146))
                .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(38)
        .cast::<i16>())
        .write(y);
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l3;
                }
                'l4: {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut sBerryTag).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_add(6147))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(y);
                }
                i = (i).wrapping_add(1);
            }
        }
        ChangeBgY(1u8, 4096i32, ((((data).wrapping_offset(1)).read()) as u8));
        ChangeBgY(2u8, 4096i32, ((((data).wrapping_offset(1)).read()) as u8));
        if (((data).read()) as i32) == 0i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleInput));
        }
    }
}
