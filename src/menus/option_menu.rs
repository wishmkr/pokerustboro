//! Translated from `src/option_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sOptionMenuText_Pal sEqualSignGfx sOptionMenuItemsNames sOptionMenuWinTemplates sOptionMenuBgTemplates sOptionMenuBg_Pal
#[allow(unused_imports)]
use crate::data::option_menu::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sArrowPressed: u8 = 0u8;

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gTasks: u8;
    static mut gText_BattleSceneOff: u8;
    static mut gText_BattleSceneOn: u8;
    static mut gText_BattleStyleSet: u8;
    static mut gText_BattleStyleShift: u8;
    static mut gText_ButtonTypeLEqualsA: u8;
    static mut gText_ButtonTypeLR: u8;
    static mut gText_ButtonTypeNormal: u8;
    static mut gText_FrameType: u8;
    static mut gText_FrameTypeNumber: u8;
    static mut gText_Option: u8;
    static mut gText_SoundMono: u8;
    static mut gText_SoundStereo: u8;
    static mut gText_TextSpeedFast: u8;
    static mut gText_TextSpeedMid: u8;
    static mut gText_TextSpeedSlow: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FreeAllWindowBuffers();
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn GetWindowFrameTilesPal(a0: u8) -> *mut u8;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetPokemonCryStereo(a0: u32);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn MainCB2() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitOptionMenu() {
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
                || __sw1 == 11i32;
            let mut __fall = false;
            if __sw1 == 0i32 || !__matched {
                __fall = true;
                SetVBlankCallback(None);
                let __p2 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                {
                    let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
                    let mut _size: u32 = 98304u32;
                    'l2: loop {
                        if !((1i32) != 0) {
                            break 'l2;
                        }
                        'l3: loop {
                            'l4: {
                                {
                                    let mut tmp: u16 = 0u16;
                                    (&raw mut tmp).write_volatile(0u16);
                                    'l5: loop {
                                        'l6: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((&raw mut tmp) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (((-2130706432i32)
                                                        | crate::c::div_i32(
                                                            4096i32,
                                                            crate::c::div_i32(16i32, 8i32),
                                                        ))
                                                        as u32),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
                                            }
                                        }
                                        if !((0i32) != 0) {
                                            break 'l5;
                                        }
                                    }
                                }
                            }
                            if !((0i32) != 0) {
                                break 'l3;
                            }
                        }
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                        if _size <= 4096u32 {
                            'l7: loop {
                                'l8: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l9: loop {
                                            'l10: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2164260864u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l9;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l7;
                                }
                            }
                            break 'l2;
                        }
                    }
                }
                'l11: loop {
                    'l12: {
                        {
                            let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u32);
                            let mut _size: u32 = 1024u32;
                            'l13: loop {
                                'l14: {
                                    {
                                        let mut tmp: u32 = 0u32;
                                        (&raw mut tmp).write_volatile(0u32);
                                        'l15: loop {
                                            'l16: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2231369728u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(32i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l15;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l11;
                    }
                }
                'l17: loop {
                    'l18: {
                        {
                            let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u16);
                            let mut _size: u32 = 1024u32;
                            'l19: loop {
                                'l20: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l21: loop {
                                            'l22: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2164260864u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l21;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l19;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l17;
                    }
                }
                SetGpuReg(0u8, 0u16);
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sOptionMenuBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                    ((crate::c::div_u32(8u32, 4u32)) as u8),
                );
                ChangeBgX(0u8, 0i32, 0u8);
                ChangeBgY(0u8, 0i32, 0u8);
                ChangeBgX(1u8, 0i32, 0u8);
                ChangeBgY(1u8, 0i32, 0u8);
                ChangeBgX(2u8, 0i32, 0u8);
                ChangeBgY(2u8, 0i32, 0u8);
                ChangeBgX(3u8, 0i32, 0u8);
                ChangeBgY(3u8, 0i32, 0u8);
                InitWindows(
                    ((&raw const sOptionMenuWinTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                DeactivateAllTextPrinters();
                SetGpuReg(64u8, 0u16);
                SetGpuReg(68u8, 0u16);
                SetGpuReg(72u8, 1u16);
                SetGpuReg(74u8, 35u16);
                SetGpuReg(80u8, 193u16);
                SetGpuReg(82u8, 0u16);
                SetGpuReg(84u8, 4u16);
                SetGpuReg(0u8, 12352u16);
                ShowBg(0u8);
                ShowBg(1u8);
                let __p3 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                ResetPaletteFade();
                ScanlineEffect_Stop();
                ResetTasks();
                ResetSpriteData();
                let __p4 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                LoadBgTiles(
                    1u8,
                    ((GetWindowFrameTilesPal(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                            3,
                            5,
                            false,
                        ) as u16) as u8),
                    ))
                    .cast::<*mut u8>())
                    .read(),
                    288u16,
                    418u16,
                );
                let __p5 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                LoadPalette(
                    (((&raw const sOptionMenuBg_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    0u16,
                    2u16,
                );
                LoadPalette(
                    (((GetWindowFrameTilesPal(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                            3,
                            5,
                            false,
                        ) as u16) as u8),
                    ))
                    .wrapping_add(4)
                    .cast::<*mut u16>())
                    .read())
                    .cast::<u8>(),
                    112u16,
                    32u16,
                );
                let __p6 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                LoadPalette(
                    (((&raw const sOptionMenuText_Pal)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<u16>())
                    .cast::<u16>())
                    .cast::<u8>(),
                    16u16,
                    32u16,
                );
                let __p7 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                PutWindowTilemap(0u8);
                DrawHeaderText();
                let __p8 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                let __p9 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p9).write(((__p9).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 8i32 {
                __fall = true;
                PutWindowTilemap(1u8);
                DrawOptionMenuTexts();
                let __p10 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p10).write(((__p10).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 9i32 {
                __fall = true;
                DrawBgWindowFrames();
                let __p11 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                __fall = true;
                {
                    let mut taskId: u8 = CreateTask(Some(Task_OptionMenuFadeIn), 0u8);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                            0,
                            3,
                            false,
                        ) as u16) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                            2,
                            1,
                            false,
                        ) as u16) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                            1,
                            1,
                            false,
                        ) as u16) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
                            0,
                            1,
                            false,
                        ) as u16) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19))
                            .read()) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(
                        ((crate::c::bf_read(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                            3,
                            5,
                            false,
                        ) as u16) as i16),
                    );
                    TextSpeed_DrawChoices(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as u8),
                    );
                    BattleScene_DrawChoices(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as u8),
                    );
                    BattleStyle_DrawChoices(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as u8),
                    );
                    Sound_DrawChoices(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as u8),
                    );
                    ButtonMode_DrawChoices(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as u8),
                    );
                    FrameType_DrawChoices(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as u8),
                    );
                    HighlightOptionMenuItem(
                        (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as u8),
                    );
                    CopyWindowToVram(1u8, 3u8);
                    let __p12 = ((&raw mut gMain).cast::<u8>()).wrapping_add(1080);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                    break 'l1;
                }
            }
            if __sw1 == 11i32 {
                __fall = true;
                BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                SetVBlankCallback(Some(VBlankCB));
                SetMainCallback2(Some(MainCB2));
                return;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuFadeIn(taskId: u8) {
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
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_OptionMenuProcessInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuProcessInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 6i32
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OptionMenuSave));
            }
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OptionMenuSave));
            } else {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0
                {
                    if (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        > 0i32
                    {
                        let __p1 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p1).write(((__p1).read()).wrapping_sub(1));
                    } else {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(6i16);
                    }
                    HighlightOptionMenuItem(
                        (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as u8),
                    );
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0
                    {
                        if (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32)
                            < 6i32
                        {
                            let __p2 = ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>();
                            (__p2).write(((__p2).read()).wrapping_add(1));
                        } else {
                            (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .write(0i16);
                        }
                        HighlightOptionMenuItem(
                            (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as u8),
                        );
                    } else {
                        let mut previousOption: u8 = 0u8;
                        'l1: {
                            let __sw3 = (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as i32);
                            let __matched = __sw3 == 0i32
                                || __sw3 == 1i32
                                || __sw3 == 2i32
                                || __sw3 == 3i32
                                || __sw3 == 4i32
                                || __sw3 == 5i32;
                            if __sw3 == 0i32 {
                                previousOption = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as u8);
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(
                                    ((TextSpeed_ProcessInput(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .read()) as u8),
                                    )) as i16),
                                );
                                if ((previousOption) as i32)
                                    != ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                {
                                    TextSpeed_DrawChoices(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(1))
                                        .read()) as u8),
                                    );
                                }
                                break 'l1;
                            }
                            if __sw3 == 1i32 {
                                previousOption = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .read()) as u8);
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(
                                    ((BattleScene_ProcessInput(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(2))
                                        .read()) as u8),
                                    )) as i16),
                                );
                                if ((previousOption) as i32)
                                    != ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32)
                                {
                                    BattleScene_DrawChoices(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(2))
                                        .read()) as u8),
                                    );
                                }
                                break 'l1;
                            }
                            if __sw3 == 2i32 {
                                previousOption = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .read()) as u8);
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .write(
                                    ((BattleStyle_ProcessInput(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(3))
                                        .read()) as u8),
                                    )) as i16),
                                );
                                if ((previousOption) as i32)
                                    != ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(3))
                                    .read()) as i32)
                                {
                                    BattleStyle_DrawChoices(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(3))
                                        .read()) as u8),
                                    );
                                }
                                break 'l1;
                            }
                            if __sw3 == 3i32 {
                                previousOption = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(4))
                                .read()) as u8);
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(4))
                                .write(
                                    ((Sound_ProcessInput(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(4))
                                        .read()) as u8),
                                    )) as i16),
                                );
                                if ((previousOption) as i32)
                                    != ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(4))
                                    .read()) as i32)
                                {
                                    Sound_DrawChoices(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(4))
                                        .read()) as u8),
                                    );
                                }
                                break 'l1;
                            }
                            if __sw3 == 4i32 {
                                previousOption = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(5))
                                .read()) as u8);
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(5))
                                .write(
                                    ((ButtonMode_ProcessInput(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(5))
                                        .read()) as u8),
                                    )) as i16),
                                );
                                if ((previousOption) as i32)
                                    != ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(5))
                                    .read()) as i32)
                                {
                                    ButtonMode_DrawChoices(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(5))
                                        .read()) as u8),
                                    );
                                }
                                break 'l1;
                            }
                            if __sw3 == 5i32 {
                                previousOption = ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .read()) as u8);
                                ((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(
                                    ((FrameType_ProcessInput(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(6))
                                        .read()) as u8),
                                    )) as i16),
                                );
                                if ((previousOption) as i32)
                                    != ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(6))
                                    .read()) as i32)
                                {
                                    FrameType_DrawChoices(
                                        ((((((((&raw mut gTasks).cast::<u8>())
                                            .wrapping_offset(((taskId) as i32) as isize * 40))
                                        .wrapping_add(8))
                                        .cast::<i16>())
                                        .wrapping_offset(6))
                                        .read()) as u8),
                                    );
                                }
                                break 'l1;
                            }
                            if !__matched {
                                return;
                            }
                        }
                        if (((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).read()) != 0 {
                            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(0u8);
                            CopyWindowToVram(1u8, 2u8);
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuSave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        crate::c::bf_write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
            0,
            3,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
            2,
            1,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
            1,
            1,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(21),
            0,
            1,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u16) as i32,
        );
        ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(19)).write(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u8),
        );
        crate::c::bf_write(
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
            3,
            5,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u16) as i32,
        );
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_OptionMenuFadeOut));
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuFadeOut(taskId: u8) {
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
            DestroyTask(taskId);
            FreeAllWindowBuffers();
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn HighlightOptionMenuItem(index: u8) {
    unsafe {
        let mut index = index;
        SetGpuReg(64u8, 4320u16);
        SetGpuReg(
            68u8,
            ((((((index) as i32).wrapping_mul(16i32)).wrapping_add(40i32) << 8)
                | (((index) as i32).wrapping_mul(16i32)).wrapping_add(56i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn DrawOptionMenuChoice(text: *mut u8, x: u8, y: u8, style: u8) {
    unsafe {
        let mut text = text;
        let mut x = x;
        let mut y = y;
        let mut style = style;
        let mut dst = crate::ffi::Align4([0u8; 16]);
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((((text).read()) as i32) != 255i32)
                    && (((i) as u32) < (crate::c::div_u32(16u32, 1u32)).wrapping_sub(1u32)))
                {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut dst).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(
                        ({
                            let __t2 = text;
                            text = (text).wrapping_offset(1);
                            __t2
                        })
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((style) as i32) != 0i32 {
            (((&raw mut dst).cast::<u8>()).wrapping_offset(2)).write(4u8);
            (((&raw mut dst).cast::<u8>()).wrapping_offset(5)).write(5u8);
        }
        (((&raw mut dst).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(255u8);
        AddTextPrinterParameterized(
            1u8,
            1u8,
            (&raw mut dst).cast::<u8>(),
            x,
            ((((y) as i32).wrapping_add(1i32)) as u8),
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn TextSpeed_ProcessInput(selection: u8) -> u8 {
    unsafe {
        let mut selection = selection;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            if ((selection) as i32) <= 1i32 {
                selection = (selection).wrapping_add(1);
            } else {
                selection = 0u8;
            }
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            if ((selection) as i32) != 0i32 {
                selection = (selection).wrapping_sub(1);
            } else {
                selection = 2u8;
            }
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn TextSpeed_DrawChoices(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut styles = crate::ffi::Align4([0u8; 3]);
        let mut widthSlow: i32 = 0i32;
        let mut widthMid: i32 = 0i32;
        let mut widthFast: i32 = 0i32;
        let mut xMid: i32 = 0i32;
        ((&raw mut styles).cast::<u8>()).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(2)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(((selection) as i32) as isize))
            .write(1u8);
        DrawOptionMenuChoice(
            (&raw mut gText_TextSpeedSlow).cast::<u8>(),
            104u8,
            0u8,
            ((&raw mut styles).cast::<u8>()).read(),
        );
        widthSlow = GetStringWidth(1u8, (&raw mut gText_TextSpeedSlow).cast::<u8>(), 0i16);
        widthMid = GetStringWidth(1u8, (&raw mut gText_TextSpeedMid).cast::<u8>(), 0i16);
        widthFast = GetStringWidth(1u8, (&raw mut gText_TextSpeedFast).cast::<u8>(), 0i16);
        widthMid = (widthMid).wrapping_sub(94i32);
        xMid = (crate::c::div_i32(
            ((widthSlow).wrapping_sub(widthMid)).wrapping_sub(widthFast),
            2i32,
        ))
        .wrapping_add(104i32);
        DrawOptionMenuChoice(
            (&raw mut gText_TextSpeedMid).cast::<u8>(),
            ((xMid) as u8),
            0u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).read(),
        );
        DrawOptionMenuChoice(
            (&raw mut gText_TextSpeedFast).cast::<u8>(),
            ((GetStringRightAlignXOffset(1i32, (&raw mut gText_TextSpeedFast).cast::<u8>(), 198i32))
                as u8),
            0u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(2)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn BattleScene_ProcessInput(selection: u8) -> u8 {
    unsafe {
        let mut selection = selection;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 48i32)
            != 0
        {
            selection = ((((selection) as i32) ^ 1i32) as u8);
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn BattleScene_DrawChoices(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut styles = crate::ffi::Align4([0u8; 2]);
        ((&raw mut styles).cast::<u8>()).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(((selection) as i32) as isize))
            .write(1u8);
        DrawOptionMenuChoice(
            (&raw mut gText_BattleSceneOn).cast::<u8>(),
            104u8,
            16u8,
            ((&raw mut styles).cast::<u8>()).read(),
        );
        DrawOptionMenuChoice(
            (&raw mut gText_BattleSceneOff).cast::<u8>(),
            ((GetStringRightAlignXOffset(
                1i32,
                (&raw mut gText_BattleSceneOff).cast::<u8>(),
                198i32,
            )) as u8),
            16u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn BattleStyle_ProcessInput(selection: u8) -> u8 {
    unsafe {
        let mut selection = selection;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 48i32)
            != 0
        {
            selection = ((((selection) as i32) ^ 1i32) as u8);
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn BattleStyle_DrawChoices(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut styles = crate::ffi::Align4([0u8; 2]);
        ((&raw mut styles).cast::<u8>()).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(((selection) as i32) as isize))
            .write(1u8);
        DrawOptionMenuChoice(
            (&raw mut gText_BattleStyleShift).cast::<u8>(),
            104u8,
            32u8,
            ((&raw mut styles).cast::<u8>()).read(),
        );
        DrawOptionMenuChoice(
            (&raw mut gText_BattleStyleSet).cast::<u8>(),
            ((GetStringRightAlignXOffset(
                1i32,
                (&raw mut gText_BattleStyleSet).cast::<u8>(),
                198i32,
            )) as u8),
            32u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn Sound_ProcessInput(selection: u8) -> u8 {
    unsafe {
        let mut selection = selection;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 48i32)
            != 0
        {
            selection = ((((selection) as i32) ^ 1i32) as u8);
            SetPokemonCryStereo(((selection) as u32));
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn Sound_DrawChoices(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut styles = crate::ffi::Align4([0u8; 2]);
        ((&raw mut styles).cast::<u8>()).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(((selection) as i32) as isize))
            .write(1u8);
        DrawOptionMenuChoice(
            (&raw mut gText_SoundMono).cast::<u8>(),
            104u8,
            48u8,
            ((&raw mut styles).cast::<u8>()).read(),
        );
        DrawOptionMenuChoice(
            (&raw mut gText_SoundStereo).cast::<u8>(),
            ((GetStringRightAlignXOffset(1i32, (&raw mut gText_SoundStereo).cast::<u8>(), 198i32))
                as u8),
            48u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn FrameType_ProcessInput(selection: u8) -> u8 {
    unsafe {
        let mut selection = selection;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            if ((selection) as i32) < 19i32 {
                selection = (selection).wrapping_add(1);
            } else {
                selection = 0u8;
            }
            LoadBgTiles(
                1u8,
                ((GetWindowFrameTilesPal(selection)).cast::<*mut u8>()).read(),
                288u16,
                418u16,
            );
            LoadPalette(
                (((GetWindowFrameTilesPal(selection))
                    .wrapping_add(4)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
                112u16,
                32u16,
            );
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            if ((selection) as i32) != 0i32 {
                selection = (selection).wrapping_sub(1);
            } else {
                selection = 19u8;
            }
            LoadBgTiles(
                1u8,
                ((GetWindowFrameTilesPal(selection)).cast::<*mut u8>()).read(),
                288u16,
                418u16,
            );
            LoadPalette(
                (((GetWindowFrameTilesPal(selection))
                    .wrapping_add(4)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
                112u16,
                32u16,
            );
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn FrameType_DrawChoices(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut text = crate::ffi::Align4([0u8; 16]);
        let mut n: u8 = ((((selection) as i32).wrapping_add(1i32)) as u8);
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((((((&raw mut gText_FrameTypeNumber).cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                .read()) as i32)
                    != 255i32)
                    && (((i) as i32) <= 5i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut text).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(
                        (((&raw mut gText_FrameTypeNumber).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        if crate::c::div_i32(((n) as i32), 10i32) != 0i32 {
            (((&raw mut text).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                .write((((crate::c::div_i32(((n) as i32), 10i32)).wrapping_add(161i32)) as u8));
            i = (i).wrapping_add(1);
            (((&raw mut text).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                .write((((crate::c::rem_i32(((n) as i32), 10i32)).wrapping_add(161i32)) as u8));
            i = (i).wrapping_add(1);
        } else {
            (((&raw mut text).cast::<u8>()).wrapping_offset(((i) as i32) as isize))
                .write((((crate::c::rem_i32(((n) as i32), 10i32)).wrapping_add(161i32)) as u8));
            i = (i).wrapping_add(1);
            (((&raw mut text).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(119u8);
            i = (i).wrapping_add(1);
        }
        (((&raw mut text).cast::<u8>()).wrapping_offset(((i) as i32) as isize)).write(255u8);
        DrawOptionMenuChoice((&raw mut gText_FrameType).cast::<u8>(), 104u8, 80u8, 0u8);
        DrawOptionMenuChoice((&raw mut text).cast::<u8>(), 128u8, 80u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn ButtonMode_ProcessInput(selection: u8) -> u8 {
    unsafe {
        let mut selection = selection;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            if ((selection) as i32) <= 1i32 {
                selection = (selection).wrapping_add(1);
            } else {
                selection = 0u8;
            }
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            if ((selection) as i32) != 0i32 {
                selection = (selection).wrapping_sub(1);
            } else {
                selection = 2u8;
            }
            ((&raw mut sArrowPressed).cast::<u8>().cast::<u8>()).write(1u8);
        }
        return selection;
    }
}
pub(crate) unsafe extern "C" fn ButtonMode_DrawChoices(selection: u8) {
    unsafe {
        let mut selection = selection;
        let mut widthNormal: i32 = 0i32;
        let mut widthLR: i32 = 0i32;
        let mut widthLA: i32 = 0i32;
        let mut xLR: i32 = 0i32;
        let mut styles = crate::ffi::Align4([0u8; 3]);
        ((&raw mut styles).cast::<u8>()).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(2)).write(0u8);
        (((&raw mut styles).cast::<u8>()).wrapping_offset(((selection) as i32) as isize))
            .write(1u8);
        DrawOptionMenuChoice(
            (&raw mut gText_ButtonTypeNormal).cast::<u8>(),
            104u8,
            64u8,
            ((&raw mut styles).cast::<u8>()).read(),
        );
        widthNormal = GetStringWidth(1u8, (&raw mut gText_ButtonTypeNormal).cast::<u8>(), 0i16);
        widthLR = GetStringWidth(1u8, (&raw mut gText_ButtonTypeLR).cast::<u8>(), 0i16);
        widthLA = GetStringWidth(1u8, (&raw mut gText_ButtonTypeLEqualsA).cast::<u8>(), 0i16);
        widthLR = (widthLR).wrapping_sub(94i32);
        xLR = (crate::c::div_i32(
            ((widthNormal).wrapping_sub(widthLR)).wrapping_sub(widthLA),
            2i32,
        ))
        .wrapping_add(104i32);
        DrawOptionMenuChoice(
            (&raw mut gText_ButtonTypeLR).cast::<u8>(),
            ((xLR) as u8),
            64u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(1)).read(),
        );
        DrawOptionMenuChoice(
            (&raw mut gText_ButtonTypeLEqualsA).cast::<u8>(),
            ((GetStringRightAlignXOffset(
                1i32,
                (&raw mut gText_ButtonTypeLEqualsA).cast::<u8>(),
                198i32,
            )) as u8),
            64u8,
            (((&raw mut styles).cast::<u8>()).wrapping_offset(2)).read(),
        );
    }
}
pub(crate) unsafe extern "C" fn DrawHeaderText() {
    unsafe {
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_Option).cast::<u8>(),
            8u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn DrawOptionMenuTexts() {
    unsafe {
        let mut i: u8 = 0u8;
        FillWindowPixelBuffer(1u8, 17u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    AddTextPrinterParameterized(
                        1u8,
                        1u8,
                        ((((&raw const sOptionMenuItemsNames)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                        8u8,
                        (((((i) as i32).wrapping_mul(16i32)).wrapping_add(1i32)) as u8),
                        255u8,
                        None,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn DrawBgWindowFrames() {
    unsafe {
        FillBgTilemapBufferRect(1u8, 418u16, 1u8, 0u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 419u16, 2u8, 0u8, 27u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 420u16, 28u8, 0u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 421u16, 1u8, 1u8, 1u8, 2u8, 7u8);
        FillBgTilemapBufferRect(1u8, 423u16, 28u8, 1u8, 1u8, 2u8, 7u8);
        FillBgTilemapBufferRect(1u8, 424u16, 1u8, 3u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 425u16, 2u8, 3u8, 27u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 426u16, 28u8, 3u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 418u16, 1u8, 4u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 419u16, 2u8, 4u8, 26u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 420u16, 28u8, 4u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 421u16, 1u8, 5u8, 1u8, 18u8, 7u8);
        FillBgTilemapBufferRect(1u8, 423u16, 28u8, 5u8, 1u8, 18u8, 7u8);
        FillBgTilemapBufferRect(1u8, 424u16, 1u8, 19u8, 1u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 425u16, 2u8, 19u8, 26u8, 1u8, 7u8);
        FillBgTilemapBufferRect(1u8, 426u16, 28u8, 19u8, 1u8, 1u8, 7u8);
        CopyBgTilemapBufferToVram(1u8);
    }
}
