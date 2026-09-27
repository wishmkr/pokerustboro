//! Translated from `src/wallclock.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sHand_Gfx sTextPrompt_Pal sWindowTemplates sWindowTemplate_ConfirmYesNo sBgTemplates sSpriteSheet_ClockHand sUnused sSpritePalettes_Clock sOam_ClockHand sAnim_MinuteHand sAnim_HourHand sAnims_MinuteHand sAnims_HourHand sSpriteTemplate_MinuteHand sSpriteTemplate_HourHand sOam_PeriodIndicator sAnim_PM sAnim_AM sAnims_PM sAnims_AM sSpriteTemplate_PM sSpriteTemplate_AM sClockHandCoords
#[allow(unused_imports)]
use crate::data::wallclock::*;

unsafe extern "C" {
    static mut gLocalTime: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_Cancel4: u8;
    static mut gText_Confirm3: u8;
    static mut gText_IsThisTheCorrectTime: u8;
    static mut gWallClockFemale_Pal: u8;
    static mut gWallClockMale_Pal: u8;
    static mut gWallClockStart_Tilemap: u8;
    static mut gWallClockView_Tilemap: u8;
    static mut gWallClock_Gfx: u8;
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
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn Cos2(a0: u16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn EnableInterrupts(a0: u16);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedSpriteSheet(a0: *mut u8) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut u8);
    fn LoadUserWindowBorderGfx(a0: u8, a1: u16, a2: u8);
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RtcCalcLocalTime();
    fn RtcInitLocalTimeOffset(a0: i32, a1: i32);
    fn RunTasks();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn Sin2(a0: u16) -> i16;
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn VBlankCB_WallClock() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn LoadWallClockGraphics() {
    unsafe {
        SetVBlankCallback(None);
        SetGpuReg(0u8, 0u16);
        SetGpuReg(14u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        ChangeBgX(2u8, 0i32, 0u8);
        ChangeBgY(2u8, 0i32, 0u8);
        ChangeBgX(3u8, 0i32, 0u8);
        ChangeBgY(3u8, 0i32, 0u8);
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l1: loop {
                if !((1i32) != 0) {
                    break 'l1;
                }
                'l2: loop {
                    'l3: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l4: loop {
                                'l5: {
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
                                        let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l6: loop {
                        'l7: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l8: loop {
                                    'l9: {
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
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l8;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l6;
                        }
                    }
                    break 'l1;
                }
            }
        }
        'l10: loop {
            'l11: {
                {
                    let mut _dest: *mut u32 = ((117440512i32) as usize as *mut u8).cast::<u32>();
                    let mut _size: u32 = 1024u32;
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u32 = 0u32;
                                (&raw mut tmp).write_volatile(0u32);
                                'l14: loop {
                                    'l15: {
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
                                                        ((crate::c::div_i32(32i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l14;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l12;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l10;
            }
        }
        'l16: loop {
            'l17: {
                {
                    let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u8).cast::<u16>();
                    let mut _size: u32 = 1024u32;
                    'l18: loop {
                        'l19: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
                                'l20: loop {
                                    'l21: {
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
                                                        ((crate::c::div_i32(16i32, 8i32)) as u32),
                                                    )),
                                            );
                                            let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                                        }
                                    }
                                    if !((0i32) != 0) {
                                        break 'l20;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l18;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l16;
            }
        }
        LZ77UnCompVram(
            ((&raw mut gWallClock_Gfx).cast::<u32>()).cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
            LoadPalette(
                (((&raw mut gWallClockMale_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                0u16,
                32u16,
            );
        } else {
            LoadPalette(
                (((&raw mut gWallClockFemale_Pal).cast::<u16>()).cast::<u16>()).cast::<u8>(),
                0u16,
                32u16,
            );
        }
        LoadPalette(
            (GetOverworldTextboxPalettePtr()).cast::<u8>(),
            224u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sTextPrompt_Pal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            192u16,
            8u16,
        );
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(12u32, 4u32)) as u8),
        );
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadUserWindowBorderGfx(0u8, 592u16, 208u8);
        ClearScheduledBgCopiesToVram();
        ScanlineEffect_Stop();
        ResetTasks();
        ResetSpriteData();
        ResetPaletteFade();
        FreeAllSpritePalettes();
        LoadCompressedSpriteSheet((&raw const sSpriteSheet_ClockHand).cast::<u8>().cast_mut());
        LoadSpritePalettes(
            ((&raw const sSpritePalettes_Clock).cast::<u8>().cast_mut()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn WallClockInit() {
    unsafe {
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        EnableInterrupts(1u16);
        SetVBlankCallback(Some(VBlankCB_WallClock));
        SetMainCallback2(Some(CB2_WallClock));
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        ShowBg(2u8);
        ShowBg(3u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_StartWallClock() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        LoadWallClockGraphics();
        LZ77UnCompVram(
            ((&raw mut gWallClockStart_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100677632i32) as usize as *mut u16).cast::<u8>(),
        );
        taskId = CreateTask(Some(Task_SetClock_WaitFadeIn), 0u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(10i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(0i16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(300i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_MinuteHand)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            80i16,
            1u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_HourHand)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            80i16,
            0u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            (1u32) as i32,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_PM).cast::<u8>().cast_mut(),
            120i16,
            80i16,
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(45i16);
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_AM).cast::<u8>().cast_mut(),
            120i16,
            80i16,
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(90i16);
        WallClockInit();
        AddTextPrinterParameterized(
            1u8,
            1u8,
            (&raw mut gText_Confirm3).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ViewWallClock() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        let mut angle1: u8 = 0u8;
        let mut angle2: u8 = 0u8;
        LoadWallClockGraphics();
        LZ77UnCompVram(
            ((&raw mut gWallClockView_Tilemap).cast::<u32>()).cast::<u32>(),
            ((100677632i32) as usize as *mut u16).cast::<u8>(),
        );
        taskId = CreateTask(Some(Task_ViewClock_WaitFadeIn), 0u8);
        InitClockWithRtc(taskId);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            == 0i32
        {
            angle1 = 45u8;
            angle2 = 90u8;
        } else {
            angle1 = 90u8;
            angle2 = 135u8;
        }
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_MinuteHand)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            80i16,
            1u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            (0u32) as i32,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_HourHand)
                .cast::<u8>()
                .cast_mut(),
            120i16,
            80i16,
            0u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (1u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            (1u32) as i32,
        );
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_PM).cast::<u8>().cast_mut(),
            120i16,
            80i16,
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((angle1) as i16));
        spriteId = CreateSprite(
            (&raw const sSpriteTemplate_AM).cast::<u8>().cast_mut(),
            120i16,
            80i16,
            2u8,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((angle2) as i16));
        WallClockInit();
        AddTextPrinterParameterized(
            1u8,
            1u8,
            (&raw mut gText_Cancel4).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(1u8);
        ScheduleBgCopyTilemapToVram(2u8);
    }
}
pub(crate) unsafe extern "C" fn CB2_WallClock() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_WaitFadeIn(taskId: u8) {
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
            .write(Some(Task_SetClock_HandleInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (crate::c::rem_i32(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32),
            6i32,
        )) != 0
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((CalcNewMinHandAngle(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u16),
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8),
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as u8),
                )) as i16),
            );
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    .wrapping_mul(6i32)) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((crate::c::rem_i32(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32),
                    12i32,
                ))
                .wrapping_mul(30i32))
                .wrapping_add(
                    (crate::c::div_i32(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read()) as i32),
                        10i32,
                    ))
                    .wrapping_mul(5i32),
                )) as i16),
            );
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SetClock_AskConfirm));
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(0i16);
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 32i32)
                    != 0
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(1i16);
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(44)
                    .cast::<u16>())
                .read()) as i32)
                    & 16i32)
                    != 0
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(2i16);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    != 0i32
                {
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .read()) as i32)
                        < 255i32
                    {
                        let __p1 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(
                        ((CalcNewMinHandAngle(
                            (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .read()) as u16),
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as u8),
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .read()) as u8),
                        )) as i16),
                    );
                    AdvanceClock(
                        taskId,
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as u8),
                    );
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(0i16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_AskConfirm(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DrawStdFrameWithCustomTileAndPalette(0u8, 0u8, 592u16, 13u8);
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_IsThisTheCorrectTime).cast::<u8>(),
            0u8,
            1u8,
            0u8,
            None,
        );
        PutWindowTilemap(0u8);
        ScheduleBgCopyTilemapToVram(0u8);
        CreateYesNoMenu(
            (&raw const sWindowTemplate_ConfirmYesNo)
                .cast::<u8>()
                .cast_mut(),
            592u16,
            13u8,
            1u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SetClock_HandleConfirmInput));
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_HandleConfirmInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                PlaySE(5u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SetClock_Confirmed));
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == (-1i32) {
                PlaySE(5u16);
                ClearStdWindowAndFrameToTransparent(0u8, 0u8);
                ClearWindowTilemap(0u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_SetClock_HandleInput));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_Confirmed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RtcInitLocalTimeOffset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32),
        );
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_SetClock_Exit));
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_Exit(taskId: u8) {
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
pub(crate) unsafe extern "C" fn Task_ViewClock_WaitFadeIn(taskId: u8) {
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
            .write(Some(Task_ViewClock_HandleInput));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ViewClock_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        InitClockWithRtc(taskId);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ViewClock_FadeOut));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ViewClock_FadeOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ViewClock_Exit));
    }
}
pub(crate) unsafe extern "C" fn Task_ViewClock_Exit(taskId: u8) {
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
            SetMainCallback2(
                (((&raw mut gMain).cast::<u8>())
                    .wrapping_add(8)
                    .cast::<Option<unsafe extern "C" fn()>>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn CalcMinHandDelta(speed: u16) -> u8 {
    unsafe {
        let mut speed = speed;
        if ((speed) as i32) > 60i32 {
            return 6u8;
        }
        if ((speed) as i32) > 30i32 {
            return 3u8;
        }
        if ((speed) as i32) > 10i32 {
            return 2u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn CalcNewMinHandAngle(angle: u16, direction: u8, speed: u8) -> u16 {
    unsafe {
        let mut angle = angle;
        let mut direction = direction;
        let mut speed = speed;
        let mut delta: u8 = CalcMinHandDelta(((speed) as u16));
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 1i32 {
                if (angle) != 0 {
                    angle = ((((angle) as i32).wrapping_sub(((delta) as i32))) as u16);
                } else {
                    angle = (((360i32).wrapping_sub(((delta) as i32))) as u16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((angle) as i32) < (360i32).wrapping_sub(((delta) as i32)) {
                    angle = ((((angle) as i32).wrapping_add(((delta) as i32))) as u16);
                } else {
                    angle = 0u16;
                }
                break 'l1;
            }
        }
        return angle;
    }
}
pub(crate) unsafe extern "C" fn AdvanceClock(taskId: u8, direction: u8) -> u32 {
    unsafe {
        let mut taskId = taskId;
        let mut direction = direction;
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 1i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    > 0i32
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(59i16);
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        > 0i32
                    {
                        let __p3 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        (__p3).write(((__p3).read()).wrapping_sub(1));
                    } else {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(23i16);
                    }
                    UpdateClockPeriod(taskId, direction);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    < 59i32
                {
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)
                        < 23i32
                    {
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    } else {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(0i16);
                    }
                    UpdateClockPeriod(taskId, direction);
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn UpdateClockPeriod(taskId: u8, direction: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut direction = direction;
        let mut hours: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        'l1: {
            let __sw1 = ((direction) as i32);
            if __sw1 == 1i32 {
                'l2: {
                    let __sw2 = ((hours) as i32);
                    if __sw2 == 11i32 {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l2;
                    }
                    if __sw2 == 23i32 {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(1i16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l3: {
                    let __sw3 = ((hours) as i32);
                    if __sw3 == 0i32 {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(0i16);
                        break 'l3;
                    }
                    if __sw3 == 12i32 {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(1i16);
                        break 'l3;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitClockWithRtc(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RtcCalcLocalTime();
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(2)
                .cast::<i8>())
            .read()) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(3)
                .cast::<i8>())
            .read()) as i16),
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                .wrapping_mul(6i32)) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((crate::c::rem_i32(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32),
                12i32,
            ))
            .wrapping_mul(30i32))
            .wrapping_add(
                (crate::c::div_i32(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32),
                    10i32,
                ))
                .wrapping_mul(5i32),
            )) as i16),
        );
        if (((((&raw mut gLocalTime).cast::<u8>())
            .wrapping_add(2)
            .cast::<i8>())
        .read()) as i32)
            < 12i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MinuteHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut angle: u16 = (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u16);
        let mut sin: i16 = ((crate::c::div_i32(((Sin2(angle)) as i32), 16i32)) as i16);
        let mut cos: i16 = ((crate::c::div_i32(((Cos2(angle)) as i32), 16i32)) as i16);
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        SetOamMatrix(
            0u8,
            ((cos) as u16),
            ((sin) as u16),
            ((((sin) as i32).wrapping_neg()) as u16),
            ((cos) as u16),
        );
        x = (((((((&raw const sClockHandCoords).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((angle) as i32) as isize * 2))
        .cast::<i8>())
        .read()) as u16);
        y = ((((((((&raw const sClockHandCoords).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((angle) as i32) as isize * 2))
        .cast::<i8>())
        .wrapping_offset(1))
        .read()) as u16);
        if ((x) as i32) > 128i32 {
            x = ((((x) as i32) | 65280i32) as u16);
        }
        if ((y) as i32) > 128i32 {
            y = ((((y) as i32) | 65280i32) as u16);
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(((x) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(((y) as i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HourHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut angle: u16 = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u16);
        let mut sin: i16 = ((crate::c::div_i32(((Sin2(angle)) as i32), 16i32)) as i16);
        let mut cos: i16 = ((crate::c::div_i32(((Cos2(angle)) as i32), 16i32)) as i16);
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        SetOamMatrix(
            1u8,
            ((cos) as u16),
            ((sin) as u16),
            ((((sin) as i32).wrapping_neg()) as u16),
            ((cos) as u16),
        );
        x = (((((((&raw const sClockHandCoords).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((angle) as i32) as isize * 2))
        .cast::<i8>())
        .read()) as u16);
        y = ((((((((&raw const sClockHandCoords).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((angle) as i32) as isize * 2))
        .cast::<i8>())
        .wrapping_offset(1))
        .read()) as u16);
        if ((x) as i32) > 128i32 {
            x = ((((x) as i32) | 65280i32) as u16);
        }
        if ((y) as i32) > 128i32 {
            y = ((((y) as i32) | 65280i32) as u16);
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(((x) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(((y) as i16));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PMIndicator(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            != 0i32
        {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >= 60i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    < 90i32)
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(5i32)) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 60i32
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        } else {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >= 46i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    < 76i32)
            {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(5i32)) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 75i32
            {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_sub(1));
            }
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((Cos2(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u16),
                )) as i32)
                    .wrapping_mul(30i32),
                4096i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((Sin2(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u16),
                )) as i32)
                    .wrapping_mul(30i32),
                4096i32,
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_AMIndicator(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            != 0i32
        {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >= 105i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    < 135i32)
            {
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p1).write((((((__p1).read()) as i32).wrapping_add(5i32)) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 105i32
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        } else {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >= 91i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    < 121i32)
            {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(5i32)) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 120i32
            {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_sub(1));
            }
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((Cos2(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u16),
                )) as i32)
                    .wrapping_mul(30i32),
                4096i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((Sin2(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u16),
                )) as i32)
                    .wrapping_mul(30i32),
                4096i32,
            )) as i16),
        );
    }
}
