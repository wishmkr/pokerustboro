//! Translated from `src/pokenav_main_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sSpinningPokenav_Pal sSpinningPokenav_Gfx sBlueLightCopy gPokenavMainMenuBgTemplates sHelpBarWindowTemplate sHelpBarTexts sHelpBarTextColors sSpinningPokenavSpriteSheet sSpinningNavgearPalettes sMenuLeftHeaderSpriteSheet sMenuLeftHeaderSpriteSheets sPokenavSubMenuLeftHeaderSpriteSheets sSpinningPokenavSpriteOam sSpinningPokenavAnims sSpinningPokenavAnimTable sSpinningPokenavSpriteTemplate sOamData_LeftHeader sOamData_SubmenuLeftHeader sLeftHeaderSpriteTemplate sSubmenuLeftHeaderSpriteTemplate
#[allow(unused_imports)]
use crate::data::pokenav_main_menu::*;

unsafe extern "C" {
    static mut gDecompressionBuffer: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gPokenavHeader_Gfx: u8;
    static mut gPokenavHeader_Pal: u8;
    static mut gPokenavHeader_Tilemap: u8;
    static mut gPokenavLeftHeader_Pal: u8;
    static mut gSprites: u8;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AllocSubstruct(a0: u32, a1: u32) -> *mut u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateLoopedTask(a0: Option<unsafe extern "C" fn(i32) -> u32>, a1: u32) -> u32;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DecompressAndCopyTileDataToVram(a0: u8, a1: *mut u8, a2: u32, a3: u16, a4: u8) -> *mut u8;
    fn DestroySprite(a0: *mut u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeMenuHandlerSubstruct2();
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FreeTempTileDataBuffersIfPossible() -> u8;
    fn GetBgY(a0: u8) -> i32;
    fn GetDecompressedDataSize(a0: *mut u32) -> u32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn GetSubstructPtr(a0: u32) -> *mut u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitBgFromTemplate(a0: *mut u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn IsLoopedTaskActive(a0: u32) -> u32;
    fn LZ77UnCompWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn PlaySE(a0: u16);
    fn PutWindowTilemap(a0: u8);
    fn RequestDma3Copy(a0: *mut u8, a1: *mut u8, a2: u16, a3: u8) -> i16;
    fn ResetBgPositions();
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetBldCnt_();
    fn ResetSpriteData();
    fn ResetTempTileDataBuffers();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPokenavMainMenu() -> u32 {
    unsafe {
        let mut menu: *mut u8 = core::ptr::null_mut();
        menu = AllocSubstruct(0u32, 2092u32);
        if ((menu) as usize) == 0usize {
            return 0u32;
        }
        ResetSpriteData();
        FreeAllSpritePalettes();
        ((menu).wrapping_add(12).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_InitPokenavMenu), 1u32));
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavMainMenuLoopedTaskIsActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        return IsLoopedTaskActive(((menu).wrapping_add(12).cast::<u32>()).read());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShutdownPokenav() {
    unsafe {
        PlaySE(111u16);
        ResetBldCnt_();
        BeginNormalPaletteFade(4294967295u32, (-1i8), 0u8, 16u8, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitForPokenavShutdownFade() -> u32 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            FreeMenuHandlerSubstruct2();
            CleanupPokenavMainMenuResources();
            FreeAllWindowBuffers();
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_InitPokenavMenu(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = core::ptr::null_mut();
        'l1: {
            let __sw1 = state;
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                SetGpuReg(0u8, 4160u16);
                FreeAllWindowBuffers();
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const gPokenavMainMenuBgTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(4u32, 4u32)) as u8),
                );
                ResetBgPositions();
                ResetTempTileDataBuffers();
                return 1u32;
            }
            if __sw1 == 1i32 {
                menu = GetSubstructPtr(0u32);
                DecompressAndCopyTileDataToVram(
                    0u8,
                    (((&raw mut gPokenavHeader_Gfx).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u32,
                    0u16,
                    0u8,
                );
                SetBgTilemapBuffer(0u8, ((menu).wrapping_add(44)).cast::<u8>());
                CopyToBgTilemapBuffer(
                    0u8,
                    (((&raw mut gPokenavHeader_Tilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
                    0u16,
                    0u16,
                );
                CopyPaletteIntoBufferUnfaded(
                    ((&raw mut gPokenavHeader_Pal).cast::<u16>()).cast::<u16>(),
                    0u32,
                    32u32,
                );
                CopyBgTilemapBufferToVram(0u8);
                return 0u32;
            }
            if __sw1 == 2i32 {
                if (FreeTempTileDataBuffersIfPossible()) != 0 {
                    return 2u32;
                }
                InitHelpBar();
                return 0u32;
            }
            if __sw1 == 3i32 {
                if (IsDma3ManagerBusyWithBgCopy()) != 0 {
                    return 2u32;
                }
                InitPokenavMainMenuResources();
                CreateLeftHeaderSprites();
                ShowBg(0u8);
                return 4u32;
            }
            if !__matched {
                return 4u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetActiveMenuLoopTasks(
    createLoopTask: *mut u8,
    isLoopTaskActive: *mut u8,
) {
    unsafe {
        let mut createLoopTask = createLoopTask;
        let mut isLoopTaskActive = isLoopTaskActive;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        ((menu).cast::<Option<unsafe extern "C" fn(u32)>>())
            .write(core::mem::transmute::<_, Option<unsafe extern "C" fn(u32)>>(createLoopTask));
        ((menu)
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .write(core::mem::transmute::<
            _,
            Option<unsafe extern "C" fn() -> u32>,
        >(isLoopTaskActive));
        ((menu).wrapping_add(8).cast::<u32>()).write(0u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RunMainMenuLoopedTask(state: u32) {
    unsafe {
        let mut state = state;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        ((menu).wrapping_add(8).cast::<u32>()).write(0u32);
        (((menu).cast::<Option<unsafe extern "C" fn(u32)>>()).read()).unwrap_unchecked()(state);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsActiveMenuLoopTaskActive() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        return (((menu)
            .wrapping_add(4)
            .cast::<Option<unsafe extern "C" fn() -> u32>>())
        .read())
        .unwrap_unchecked()();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SlideMenuHeaderUp() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        ((menu).wrapping_add(12).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_SlideMenuHeaderUp), 4u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SlideMenuHeaderDown() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        ((menu).wrapping_add(12).cast::<u32>())
            .write(CreateLoopedTask(Some(LoopedTask_SlideMenuHeaderDown), 4u32));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MainMenuLoopedTaskIsBusy() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        return IsLoopedTaskActive(((menu).wrapping_add(12).cast::<u32>()).read());
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_SlideMenuHeaderUp(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = state;
            let __matched = __sw1 == 1i32 || __sw1 == 0i32 || __sw1 == 2i32;
            if !__matched {
                return 4u32;
            }
            if __sw1 == 1i32 {
                return 0u32;
            }
            if __sw1 == 0i32 {
                return 0u32;
            }
            if __sw1 == 2i32 {
                if ((ChangeBgY(0u8, 384i32, 1u8)) as u32) >= 8192u32 {
                    ChangeBgY(0u8, 8192i32, 0u8);
                    return 4u32;
                }
                return 2u32;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn LoopedTask_SlideMenuHeaderDown(state: i32) -> u32 {
    unsafe {
        let mut state = state;
        if ChangeBgY(0u8, 384i32, 2u8) <= 0i32 {
            ChangeBgY(0u8, 0i32, 0u8);
            return 4u32;
        }
        return 2u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyPaletteIntoBufferUnfaded(
    palette: *mut u16,
    bufferOffset: u32,
    size: u32,
) {
    unsafe {
        let mut palette = palette;
        let mut bufferOffset = bufferOffset;
        let mut size = size;
        'l1: loop {
            'l2: {
                'l3: loop {
                    'l4: {
                        CpuSet(
                            (palette).cast::<u8>(),
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(((bufferOffset) as i32) as isize))
                            .cast::<u8>(),
                            (0u32
                                | (crate::c::div_u32(
                                    size,
                                    ((crate::c::div_i32(16i32, 8i32)) as u32),
                                ) & 2097151u32)),
                        );
                    }
                    if !((0i32) != 0) {
                        break 'l3;
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Pokenav_AllocAndLoadPalettes(palettes: *mut u8) {
    unsafe {
        let mut palettes = palettes;
        let mut current: *mut u8 = core::ptr::null_mut();
        let mut index: u32 = 0u32;
        {
            current = palettes;
            'l1: loop {
                if !(((((current).cast::<*mut u16>()).read()) as usize) != 0usize) {
                    break 'l1;
                }
                'l2: {
                    index = ((AllocSpritePalette(((current).wrapping_add(4).cast::<u16>()).read()))
                        as u32);
                    if index == 255u32 {
                        break 'l1;
                    } else {
                        index = (256u32).wrapping_add((index).wrapping_mul(16u32));
                        CopyPaletteIntoBufferUnfaded(
                            ((current).cast::<*mut u16>()).read(),
                            index,
                            32u32,
                        );
                    }
                }
                current = (current).wrapping_offset(8);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavFillPalette(palIndex: u32, fillValue: u16) {
    unsafe {
        let mut palIndex = palIndex;
        let mut fillValue = fillValue;
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(fillValue);
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (&raw mut tmp).cast::<u8>(),
                                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        (((256u32).wrapping_add((palIndex).wrapping_mul(16u32)))
                                            as i32)
                                            as isize,
                                    ))
                                .cast::<u8>(),
                                (16777216u32
                                    | (crate::c::div_u32(
                                        32u32,
                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                    ) & 2097151u32)),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavCopyPalette(
    src: *mut u16,
    dest: *mut u16,
    size: i32,
    a3: i32,
    a4: i32,
    palette: *mut u16,
) {
    unsafe {
        let mut src = src;
        let mut dest = dest;
        let mut size = size;
        let mut a3 = a3;
        let mut a4 = a4;
        let mut palette = palette;
        if a4 == 0i32 {
            'l1: loop {
                'l2: {
                    'l3: loop {
                        'l4: {
                            CpuSet(
                                (src).cast::<u8>(),
                                (palette).cast::<u8>(),
                                ((0i32
                                    | (crate::c::div_i32(
                                        (size).wrapping_mul(2i32),
                                        crate::c::div_i32(16i32, 8i32),
                                    ) & 2097151i32)) as u32),
                            );
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        } else {
            if a4 >= a3 {
                'l5: loop {
                    'l6: {
                        'l7: loop {
                            'l8: {
                                CpuSet(
                                    (dest).cast::<u8>(),
                                    (palette).cast::<u8>(),
                                    ((0i32
                                        | (crate::c::div_i32(
                                            (size).wrapping_mul(2i32),
                                            crate::c::div_i32(16i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l7;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l5;
                    }
                }
            } else {
                let mut r: i32 = 0i32;
                let mut g: i32 = 0i32;
                let mut b: i32 = 0i32;
                let mut r1: i32 = 0i32;
                let mut g1: i32 = 0i32;
                let mut b1: i32 = 0i32;
                'l9: loop {
                    if !(({
                        let __t1 = size;
                        size = (size).wrapping_sub(1);
                        __t1
                    }) != 0)
                    {
                        break 'l9;
                    }
                    r = ((((src).read()) as i32) & 31i32);
                    g = (((((src).read()) as i32) >> 5) & 31i32);
                    b = (((((src).read()) as i32) >> 10) & 31i32);
                    r1 = ((crate::c::div_i32(
                        (((((dest).read()) as i32) & 31i32) << 8).wrapping_sub((r << 8)),
                        a3,
                    ))
                    .wrapping_mul(a4)
                        >> 8);
                    g1 = ((crate::c::div_i32(
                        ((((((dest).read()) as i32) >> 5) & 31i32) << 8).wrapping_sub((g << 8)),
                        a3,
                    ))
                    .wrapping_mul(a4)
                        >> 8);
                    b1 = ((crate::c::div_i32(
                        ((((((dest).read()) as i32) >> 10) & 31i32) << 8).wrapping_sub((b << 8)),
                        a3,
                    ))
                    .wrapping_mul(a4)
                        >> 8);
                    r = ((r).wrapping_add(r1) & 31i32);
                    g = ((g).wrapping_add(g1) & 31i32);
                    b = ((b).wrapping_add(b1) & 31i32);
                    (palette).write(((((b << 10) | (g << 5)) | r) as u16));
                    src = (src).wrapping_offset(1);
                    dest = (dest).wrapping_offset(1);
                    palette = (palette).wrapping_offset(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PokenavFadeScreen(fadeType: i32) {
    unsafe {
        let mut fadeType = fadeType;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        'l1: {
            let __sw1 = fadeType;
            if __sw1 == 0i32 {
                BeginNormalPaletteFade(
                    ((menu).wrapping_add(20).cast::<u32>()).read(),
                    (-2i8),
                    0u8,
                    16u8,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                BeginNormalPaletteFade(
                    ((menu).wrapping_add(20).cast::<u32>()).read(),
                    (-2i8),
                    16u8,
                    0u8,
                    0u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                BeginNormalPaletteFade(4294967295u32, (-2i8), 0u8, 16u8, 0u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                BeginNormalPaletteFade(4294967295u32, (-2i8), 16u8, 0u8, 0u16);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPaletteFadeActive() -> u32 {
    unsafe {
        return ((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FadeToBlackExceptPrimary() {
    unsafe {
        BlendPalettes(4294901758u32, 16u8, 0u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitBgTemplates(templates: *mut u8, count: i32) {
    unsafe {
        let mut templates = templates;
        let mut count = count;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < count) {
                    break 'l1;
                }
                'l2: {
                    InitBgFromTemplate({
                        let __t1 = templates;
                        templates = (templates).wrapping_offset(4);
                        __t1
                    });
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitHelpBar() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        InitWindows(((&raw const sHelpBarWindowTemplate).cast::<u8>().cast_mut()).cast::<u8>());
        ((menu).wrapping_add(16).cast::<u32>()).write(0u32);
        DrawHelpBar(((menu).wrapping_add(16).cast::<u32>()).read());
        PutWindowTilemap(((((menu).wrapping_add(16).cast::<u32>()).read()) as u8));
        CopyWindowToVram(
            ((((menu).wrapping_add(16).cast::<u32>()).read()) as u8),
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrintHelpBarText(textId: u32) {
    unsafe {
        let mut textId = textId;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        DrawHelpBar(((menu).wrapping_add(16).cast::<u32>()).read());
        AddTextPrinterParameterized3(
            ((((menu).wrapping_add(16).cast::<u32>()).read()) as u8),
            1u8,
            0u8,
            1u8,
            ((&raw const sHelpBarTextColors).cast::<u8>().cast_mut()).cast::<u8>(),
            0i8,
            ((((&raw const sHelpBarTexts)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((textId) as i32) as isize))
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WaitForHelpBar() -> u32 {
    unsafe {
        return ((IsDma3ManagerBusyWithBgCopy()) as u32);
    }
}
pub(crate) unsafe extern "C" fn DrawHelpBar(windowId: u32) {
    unsafe {
        let mut windowId = windowId;
        FillWindowPixelBuffer(((windowId) as u8), 68u8);
        FillWindowPixelRect(((windowId) as u8), 85u8, 0u16, 0u16, 128u16, 1u16);
    }
}
pub(crate) unsafe extern "C" fn InitPokenavMainMenuResources() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = 0u8;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 8u32)) {
                    break 'l1;
                }
                'l2: {
                    LoadCompressedSpriteSheet(
                        (((&raw const sSpinningPokenavSpriteSheet)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset((i) as isize * 8),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        Pokenav_AllocAndLoadPalettes(
            ((&raw const sSpinningNavgearPalettes)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((menu).wrapping_add(20).cast::<u32>()).write(
            (((-2i32) & !(crate::c::shl_i32(65536i32, ((IndexOfSpritePaletteTag(0u16)) as u32))))
                as u32),
        );
        spriteId = CreateSprite(
            (&raw const sSpinningPokenavSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            220i16,
            12i16,
            0u8,
        );
        ((menu).wrapping_add(24).cast::<*mut u8>()).write(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
        );
    }
}
pub(crate) unsafe extern "C" fn CleanupPokenavMainMenuResources() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        DestroySprite(((menu).wrapping_add(24).cast::<*mut u8>()).read());
        FreeSpriteTilesByTag(0u16);
        FreeSpritePaletteByTag(0u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_SpinningPokenav(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((crate::c::div_u32(((GetBgY(0u8)) as u32), 256u32)).wrapping_mul(4294967295u32))
                as i16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSpinningPokenavSprite() -> *mut u8 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        ((((menu).wrapping_add(24).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        return ((menu).wrapping_add(24).cast::<*mut u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideSpinningPokenavSprite() {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        ((((menu).wrapping_add(24).cast::<*mut u8>()).read())
            .wrapping_add(32)
            .cast::<i16>())
        .write(220i16);
        ((((menu).wrapping_add(24).cast::<*mut u8>()).read())
            .wrapping_add(34)
            .cast::<i16>())
        .write(12i16);
        ((((menu).wrapping_add(24).cast::<*mut u8>()).read())
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_SpinningPokenav));
        crate::c::bf_write(
            (((menu).wrapping_add(24).cast::<*mut u8>()).read()).wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((menu).wrapping_add(24).cast::<*mut u8>()).read()).wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        ((((menu).wrapping_add(24).cast::<*mut u8>()).read()).wrapping_add(67)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn CreateLeftHeaderSprites() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut spriteId: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        LoadCompressedSpriteSheet(
            (&raw const sMenuLeftHeaderSpriteSheet)
                .cast::<u8>()
                .cast_mut(),
        );
        AllocSpritePalette(1u16);
        AllocSpritePalette(2u16);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    spriteId = ((CreateSprite(
                        (&raw const sLeftHeaderSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        0i16,
                        0i16,
                        1u8,
                    )) as i32);
                    ((((menu).wrapping_add(28)).cast::<*mut u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset((spriteId) as isize * 68),
                        );
                    crate::c::bf_write(
                        (((((menu).wrapping_add(28)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    ((((((menu).wrapping_add(28)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write((((i).wrapping_mul(64i32)) as i16));
                    spriteId = ((CreateSprite(
                        (&raw const sSubmenuLeftHeaderSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        0i16,
                        0i16,
                        2u8,
                    )) as i32);
                    ((((menu).wrapping_add(36)).cast::<*mut u8>()).wrapping_offset((i) as isize))
                        .write(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset((spriteId) as isize * 68),
                        );
                    crate::c::bf_write(
                        (((((menu).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    ((((((menu).wrapping_add(36)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write((((i).wrapping_mul(32i32)) as i16));
                    ((((((menu).wrapping_add(36)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(18i16);
                    crate::c::bf_write(
                        (((((menu).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(4),
                        0,
                        10,
                        ((((crate::c::bf_read(
                            (((((menu).wrapping_add(36)).cast::<*mut u8>())
                                .wrapping_offset((i) as isize))
                            .read())
                            .wrapping_add(4),
                            0,
                            10,
                            false,
                        ) as u16) as i32)
                            .wrapping_add(((i).wrapping_mul(8i32)).wrapping_add(64i32)))
                            as u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadLeftHeaderGfxForIndex(menuGfxId: u32) {
    unsafe {
        let mut menuGfxId = menuGfxId;
        if menuGfxId < 6u32 {
            LoadLeftHeaderGfxForMenu(menuGfxId);
        } else {
            LoadLeftHeaderGfxForSubMenu((menuGfxId).wrapping_sub(6u32));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateRegionMapRightHeaderTiles(menuGfxId: u32) {
    unsafe {
        let mut menuGfxId = menuGfxId;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        if menuGfxId == 4u32 {
            crate::c::bf_write(
                (((((menu).wrapping_add(28)).cast::<*mut u8>()).wrapping_offset(1)).read())
                    .wrapping_add(4),
                0,
                10,
                ((((GetSpriteTileStartByTag(2u16)) as i32).wrapping_add(32i32)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((((menu).wrapping_add(28)).cast::<*mut u8>()).wrapping_offset(1)).read())
                    .wrapping_add(4),
                0,
                10,
                ((((GetSpriteTileStartByTag(2u16)) as i32).wrapping_add(64i32)) as u16) as i32,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn LoadLeftHeaderGfxForMenu(menuGfxId: u32) {
    unsafe {
        let mut menuGfxId = menuGfxId;
        let mut menu: *mut u8 = core::ptr::null_mut();
        let mut size: u32 = 0u32;
        let mut tag: u32 = 0u32;
        if menuGfxId >= 6u32 {
            return;
        }
        menu = GetSubstructPtr(0u32);
        tag = (((((((&raw const sMenuLeftHeaderSpriteSheets)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((menuGfxId) as i32) as isize * 8))
        .wrapping_add(6)
        .cast::<u16>())
        .read()) as u32);
        size = GetDecompressedDataSize(
            (((((&raw const sMenuLeftHeaderSpriteSheets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((menuGfxId) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
        );
        LoadPalette(
            ((((&raw mut gPokenavLeftHeader_Pal).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((tag).wrapping_mul(16u32)) as i32) as isize))
            .cast::<u8>(),
            (((256i32).wrapping_add(((IndexOfSpritePaletteTag(1u16)) as i32).wrapping_mul(16i32)))
                as u16),
            32u16,
        );
        LZ77UnCompWram(
            (((((&raw const sMenuLeftHeaderSpriteSheets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((menuGfxId) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        RequestDma3Copy(
            (&raw mut gDecompressionBuffer).cast::<u8>(),
            ((100728832i32) as usize as *mut u8).wrapping_offset(
                (((GetSpriteTileStartByTag(2u16)) as i32).wrapping_mul(32i32)) as isize * 1,
            ),
            ((size) as u16),
            1u8,
        );
        crate::c::bf_write(
            (((((menu).wrapping_add(28)).cast::<*mut u8>()).wrapping_offset(1)).read())
                .wrapping_add(4),
            0,
            10,
            ((((GetSpriteTileStartByTag(2u16)) as i32).wrapping_add(
                (((((((&raw const sMenuLeftHeaderSpriteSheets)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(((menuGfxId) as i32) as isize * 8))
                .wrapping_add(4)
                .cast::<u16>())
                .read()) as i32),
            )) as u16) as i32,
        );
        if (menuGfxId == 4u32) || (menuGfxId == 5u32) {
            ((((((menu).wrapping_add(28)).cast::<*mut u8>()).wrapping_offset(1)).read())
                .wrapping_add(36)
                .cast::<i16>())
            .write(56i16);
        } else {
            ((((((menu).wrapping_add(28)).cast::<*mut u8>()).wrapping_offset(1)).read())
                .wrapping_add(36)
                .cast::<i16>())
            .write(64i16);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadLeftHeaderGfxForSubMenu(menuGfxId: u32) {
    unsafe {
        let mut menuGfxId = menuGfxId;
        let mut size: u32 = 0u32;
        let mut tag: u32 = 0u32;
        if menuGfxId >= 7u32 {
            return;
        }
        tag = (((((&raw const sPokenavSubMenuLeftHeaderSpriteSheets)
            .cast::<u8>()
            .cast_mut())
        .cast::<u8>())
        .wrapping_offset(((menuGfxId) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<u32>())
        .read();
        size = GetDecompressedDataSize(
            (((((&raw const sPokenavSubMenuLeftHeaderSpriteSheets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((menuGfxId) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
        );
        LoadPalette(
            ((((&raw mut gPokenavLeftHeader_Pal).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((tag).wrapping_mul(16u32)) as i32) as isize))
            .cast::<u8>(),
            (((256i32).wrapping_add(((IndexOfSpritePaletteTag(2u16)) as i32).wrapping_mul(16i32)))
                as u16),
            32u16,
        );
        LZ77UnCompWram(
            (((((&raw const sPokenavSubMenuLeftHeaderSpriteSheets)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(((menuGfxId) as i32) as isize * 8))
            .cast::<*mut u32>())
            .read(),
            ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(4096),
        );
        RequestDma3Copy(
            ((&raw mut gDecompressionBuffer).cast::<u8>()).wrapping_offset(4096),
            (((100728832i32) as usize as *mut u8).wrapping_offset(2048)).wrapping_offset(
                (((GetSpriteTileStartByTag(2u16)) as i32).wrapping_mul(32i32)) as isize * 1,
            ),
            ((size) as u16),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowLeftHeaderGfx(menuGfxId: u32, isMain: u32, isOnRightSide: u32) {
    unsafe {
        let mut menuGfxId = menuGfxId;
        let mut isMain = isMain;
        let mut isOnRightSide = isOnRightSide;
        let mut tileTop: u32 = 0u32;
        if !((isMain) != 0) {
            tileTop = 48u32;
        } else {
            tileTop = 16u32;
        }
        if menuGfxId < 6u32 {
            ShowLeftHeaderSprites(tileTop, isOnRightSide);
        } else {
            ShowLeftHeaderSubmenuSprites(tileTop, isOnRightSide);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideMainOrSubMenuLeftHeader(id: u32, onRightSide: u32) {
    unsafe {
        let mut id = id;
        let mut onRightSide = onRightSide;
        if id < 6u32 {
            HideLeftHeaderSprites(onRightSide);
        } else {
            HideLeftHeaderSubmenuSprites(onRightSide);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetLeftHeaderSpritesInvisibility() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    crate::c::bf_write(
                        (((((menu).wrapping_add(28)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    crate::c::bf_write(
                        (((((menu).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read())
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AreLeftHeaderSpritesMoving() -> u32 {
    unsafe {
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        if (core::mem::transmute::<_, usize>(
            (((((menu).wrapping_add(28)).cast::<*mut u8>()).read())
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize))
            && (core::mem::transmute::<_, usize>(
                (((((menu).wrapping_add(36)).cast::<*mut u8>()).read())
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            ) == (SpriteCallbackDummy as *const () as usize))
        {
            return 0u32;
        } else {
            return 1u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
pub(crate) unsafe extern "C" fn ShowLeftHeaderSprites(startY: u32, isOnRightSide: u32) {
    unsafe {
        let mut startY = startY;
        let mut isOnRightSide = isOnRightSide;
        let mut start: i32 = 0i32;
        let mut end: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        if !((isOnRightSide) != 0) {
            start = (-96i32);
            end = 32i32;
        } else {
            start = 256i32;
            end = 160i32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((menu).wrapping_add(28)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((startY) as i16));
                    MoveLeftHeader(
                        ((((menu).wrapping_add(28)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                        start,
                        end,
                        12i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowLeftHeaderSubmenuSprites(startY: u32, isOnRightSide: u32) {
    unsafe {
        let mut startY = startY;
        let mut isOnRightSide = isOnRightSide;
        let mut start: i32 = 0i32;
        let mut end: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        if !((isOnRightSide) != 0) {
            start = (-96i32);
            end = 16i32;
        } else {
            start = 256i32;
            end = 192i32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    ((((((menu).wrapping_add(36)).cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                    .read())
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((startY) as i16));
                    MoveLeftHeader(
                        ((((menu).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                        start,
                        end,
                        12i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HideLeftHeaderSprites(isOnRightSide: u32) {
    unsafe {
        let mut isOnRightSide = isOnRightSide;
        let mut start: i32 = 0i32;
        let mut end: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        if !((isOnRightSide) != 0) {
            start = 32i32;
            end = (-96i32);
        } else {
            start = 192i32;
            end = 256i32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    MoveLeftHeader(
                        ((((menu).wrapping_add(28)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                        start,
                        end,
                        12i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HideLeftHeaderSubmenuSprites(isOnRightSide: u32) {
    unsafe {
        let mut isOnRightSide = isOnRightSide;
        let mut start: i32 = 0i32;
        let mut end: i32 = 0i32;
        let mut i: i32 = 0i32;
        let mut menu: *mut u8 = GetSubstructPtr(0u32);
        if !((isOnRightSide) != 0) {
            start = 16i32;
            end = (-96i32);
        } else {
            start = 192i32;
            end = 256i32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((crate::c::div_u32(8u32, 4u32)) as i32)) {
                    break 'l1;
                }
                'l2: {
                    MoveLeftHeader(
                        ((((menu).wrapping_add(36)).cast::<*mut u8>())
                            .wrapping_offset((i) as isize))
                        .read(),
                        start,
                        end,
                        12i32,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MoveLeftHeader(
    sprite: *mut u8,
    startX: i32,
    endX: i32,
    duration: i32,
) {
    unsafe {
        let mut sprite = sprite;
        let mut startX = startX;
        let mut endX = endX;
        let mut duration = duration;
        ((sprite).wrapping_add(32).cast::<i16>()).write(((startX) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write((((startX).wrapping_mul(16i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((crate::c::div_i32(((endX).wrapping_sub(startX)).wrapping_mul(16i32), duration))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((duration) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(((endX) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_MoveLeftHeader));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MoveLeftHeader(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            != 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 4) as i16),
            );
            if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-16i32))
                || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 256i32)
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
        } else {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
