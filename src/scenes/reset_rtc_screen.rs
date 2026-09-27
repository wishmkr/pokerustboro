//! Translated from `src/reset_rtc_screen.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sBgTemplates sWindowTemplates sInputTimeWindow sInputMap sOamData_Arrow sArrowDown_Gfx sArrowRight_Gfx sArrow_Pal sPicTable_Arrow sSpritePalette_Arrow sAnim_Arrow_Down sAnim_Arrow_Up sAnim_Arrow_Right sAnims_Arrow sSpriteTemplate_Arrow
#[allow(unused_imports)]
use crate::data::reset_rtc_screen::*;

unsafe extern "C" {
    static mut gLocalTime: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSaveFileStatus: u8;
    static mut gSprites: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_ClockHasBeenReset: u8;
    static mut gText_Colon3: u8;
    static mut gText_Confirm2: u8;
    static mut gText_Day: u8;
    static mut gText_NoSaveFileCantSetTime: u8;
    static mut gText_PleaseResetTime: u8;
    static mut gText_PresentTime: u8;
    static mut gText_PreviousTime: u8;
    static mut gText_ResetRTCConfirmCancel: u8;
    static mut gText_SaveCompleted: u8;
    static mut gText_SaveFailed: u8;
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
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DisableResetRTC();
    fn DoScheduledBgTilemapCopiesToVram();
    fn DoSoftReset();
    fn DrawDialogFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FreeAllWindowBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadOam();
    fn LoadSpritePalette(a0: *mut u8) -> u8;
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetOamRange(a0: u8, a1: u8);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RtcCalcLocalTime();
    fn RtcCalcLocalTimeOffset(a0: i32, a1: i32, a2: i32, a3: i32);
    fn RtcReset();
    fn RunTasks();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_Stop();
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UpdatePaletteFade() -> u8;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn SpriteCB_Cursor_UpOrRight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut state: i32 = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32);
        if state
            != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((state) as i16));
            'l1: {
                let __sw1 = state;
                if __sw1 == 1i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(1u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(53i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(68i16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(1u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(86i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(68i16);
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(1u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(101i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(68i16);
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(1u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(116i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(68i16);
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(2u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(153i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(80i16);
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    DestroySprite(sprite);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor_Down(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut state: i32 = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32);
        if state
            != ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((state) as i16));
            'l1: {
                let __sw1 = state;
                if __sw1 == 1i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(0u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(53i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(92i16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(0u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(86i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(92i16);
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(0u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(101i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(92i16);
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    ((sprite).wrapping_add(42)).write(0u8);
                    crate::c::bf_write((sprite).wrapping_add(44), 0, 6, (0u8) as i32);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(116i16);
                    ((sprite).wrapping_add(34).cast::<i16>()).write(92i16);
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    DestroySprite(sprite);
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCursor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u32 = 0u32;
        LoadSpritePalette((&raw const sSpritePalette_Arrow).cast::<u8>().cast_mut());
        spriteId = ((CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_Arrow).cast::<u8>().cast_mut(),
            53i16,
            68i16,
            0u8,
        )) as u32);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Cursor_UpOrRight));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((-1i16));
        spriteId = ((CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_Arrow).cast::<u8>().cast_mut(),
            53i16,
            68i16,
            0u8,
        )) as u32);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Cursor_Down));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((-1i16));
    }
}
pub(crate) unsafe extern "C" fn FreeCursorPalette() {
    unsafe {
        FreeSpritePaletteByTag(
            (((&raw const sSpritePalette_Arrow).cast::<u8>().cast_mut())
                .wrapping_add(4)
                .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn HideChooseTimeWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        ClearStdWindowAndFrameToTransparent(windowId, 0u8);
        RemoveWindow(windowId);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn PrintTime(
    windowId: u8,
    x: u8,
    y: u8,
    days: u16,
    hours: u8,
    minutes: u8,
    seconds: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut x = x;
        let mut y = y;
        let mut days = days;
        let mut hours = hours;
        let mut minutes = minutes;
        let mut seconds = seconds;
        let mut dest: *mut u8 = (&raw mut gStringVar4).cast::<u8>();
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((days) as i32),
            1i32,
            4u8,
        );
        dest = StringCopy(dest, (&raw mut gStringVar1).cast::<u8>());
        dest = StringCopy(dest, (&raw mut gText_Day).cast::<u8>());
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((hours) as i32),
            1i32,
            3u8,
        );
        dest = StringCopy(dest, (&raw mut gStringVar1).cast::<u8>());
        dest = StringCopy(dest, (&raw mut gText_Colon3).cast::<u8>());
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((minutes) as i32),
            2i32,
            2u8,
        );
        dest = StringCopy(dest, (&raw mut gStringVar1).cast::<u8>());
        dest = StringCopy(dest, (&raw mut gText_Colon3).cast::<u8>());
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((seconds) as i32),
            2i32,
            2u8,
        );
        dest = StringCopy(dest, (&raw mut gStringVar1).cast::<u8>());
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            x,
            y,
            255u8,
            None,
        );
    }
}
pub(crate) unsafe extern "C" fn ShowChooseTimeWindow(
    windowId: u8,
    days: u16,
    hours: u8,
    minutes: u8,
    seconds: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut days = days;
        let mut hours = hours;
        let mut minutes = minutes;
        let mut seconds = seconds;
        DrawStdFrameWithCustomTileAndPalette(windowId, 0u8, 532u16, 14u8);
        PrintTime(windowId, 0u8, 1u8, days, hours, minutes, seconds);
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_Confirm2).cast::<u8>(),
            126u8,
            1u8,
            0u8,
            None,
        );
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn MoveTimeUpDown(
    val: *mut i16,
    minVal: i32,
    maxVal: i32,
    keys: u16,
) -> u32 {
    unsafe {
        let mut val = val;
        let mut minVal = minVal;
        let mut maxVal = maxVal;
        let mut keys = keys;
        if (((keys) as i32) & 128i32) != 0 {
            (val).write((((((val).read()) as i32).wrapping_sub(1i32)) as i16));
            if (((val).read()) as i32) < minVal {
                (val).write(((maxVal) as i16));
            }
        } else {
            if (((keys) as i32) & 64i32) != 0 {
                (val).write((((((val).read()) as i32).wrapping_add(1i32)) as i16));
                if (((val).read()) as i32) > maxVal {
                    (val).write(((minVal) as i16));
                }
            } else {
                if (((keys) as i32) & 32i32) != 0 {
                    (val).write((((((val).read()) as i32).wrapping_sub(10i32)) as i16));
                    if (((val).read()) as i32) < minVal {
                        (val).write(((maxVal) as i16));
                    }
                } else {
                    if (((keys) as i32) & 16i32) != 0 {
                        (val).write((((((val).read()) as i32).wrapping_add(10i32)) as i16));
                        if (((val).read()) as i32) > maxVal {
                            (val).write(((minVal) as i16));
                        }
                    } else {
                        return 0u32;
                    }
                }
            }
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_SetFinished(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(1i16);
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_Exit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        HideChooseTimeWindow(((((data).wrapping_offset(8)).read()) as u8));
        FreeCursorPalette();
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ResetRtc_SetFinished));
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_HandleInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut selection: u8 = ((((data).wrapping_offset(2)).read()) as u8);
        let mut selectionInfo: *mut u8 = (((&raw const sInputMap).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((((selection) as i32).wrapping_sub(1i32)) as isize * 12);
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 2i32)
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_ResetRtc_Exit));
            ((data).wrapping_offset(1)).write(0i16);
            ((data).wrapping_offset(2)).write(6i16);
            PlaySE(5u16);
            return;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 16i32)
            != 0
        {
            if (((selectionInfo).wrapping_add(7)).read()) != 0 {
                ((data).wrapping_offset(2))
                    .write(((((selectionInfo).wrapping_add(7)).read()) as i16));
                PlaySE(5u16);
                return;
            }
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 32i32)
            != 0
        {
            if (((selectionInfo).wrapping_add(6)).read()) != 0 {
                ((data).wrapping_offset(2))
                    .write(((((selectionInfo).wrapping_add(6)).read()) as i16));
                PlaySE(5u16);
                return;
            }
        }
        if ((selection) as i32) == 5i32 {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                (((&raw mut gLocalTime).cast::<u8>()).cast::<i16>())
                    .write(((data).wrapping_offset(3)).read());
                (((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(2)
                    .cast::<i8>())
                .write(((((data).wrapping_offset(4)).read()) as i8));
                (((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(3)
                    .cast::<i8>())
                .write(((((data).wrapping_offset(5)).read()) as i8));
                (((&raw mut gLocalTime).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<i8>())
                .write(((((data).wrapping_offset(6)).read()) as i8));
                PlaySE(5u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_ResetRtc_Exit));
                ((data).wrapping_offset(1)).write(1i16);
                ((data).wrapping_offset(2)).write(6i16);
            }
        } else {
            if (MoveTimeUpDown(
                (data).wrapping_offset((((selectionInfo).read()) as i32) as isize),
                ((((selectionInfo).wrapping_add(2).cast::<u16>()).read()) as i32),
                ((((selectionInfo).wrapping_add(4).cast::<u16>()).read()) as i32),
                (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(48)
                    .cast::<u16>())
                .read()) as i32)
                    & 192i32) as u16),
            )) != 0
            {
                PlaySE(5u16);
                PrintTime(
                    ((((data).wrapping_offset(8)).read()) as u8),
                    0u8,
                    1u8,
                    ((((data).wrapping_offset(3)).read()) as u16),
                    ((((data).wrapping_offset(4)).read()) as u8),
                    ((((data).wrapping_offset(5)).read()) as u8),
                    ((((data).wrapping_offset(6)).read()) as u8),
                );
                CopyWindowToVram(((((data).wrapping_offset(8)).read()) as u8), 2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (data).write(0i16);
        ((data).wrapping_offset(3))
            .write((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read());
        ((data).wrapping_offset(4)).write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(2)
                .cast::<i8>())
            .read()) as i16),
        );
        ((data).wrapping_offset(5)).write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(3)
                .cast::<i8>())
            .read()) as i16),
        );
        ((data).wrapping_offset(6)).write(
            (((((&raw mut gLocalTime).cast::<u8>())
                .wrapping_add(4)
                .cast::<i8>())
            .read()) as i16),
        );
        ((data).wrapping_offset(8))
            .write(((AddWindow((&raw const sInputTimeWindow).cast::<u8>().cast_mut())) as i16));
        ShowChooseTimeWindow(
            ((((data).wrapping_offset(8)).read()) as u8),
            ((((data).wrapping_offset(3)).read()) as u16),
            ((((data).wrapping_offset(4)).read()) as u8),
            ((((data).wrapping_offset(5)).read()) as u8),
            ((((data).wrapping_offset(6)).read()) as u8),
        );
        CreateCursor(taskId);
        ((data).wrapping_offset(2)).write(2i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ResetRtc_HandleInput));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitResetRtcScreen() {
    unsafe {
        SetGpuReg(0u8, 0u16);
        SetVBlankCallback(None);
        'l1: loop {
            'l2: {
                {
                    let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u16);
                    let mut _size: u32 = 1024u32;
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
                                        break 'l5;
                                    }
                                }
                            }
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
        {
            let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
            let mut _size: u32 = 98304u32;
            'l7: loop {
                if !((1i32) != 0) {
                    break 'l7;
                }
                'l8: loop {
                    'l9: {
                        {
                            let mut tmp: u16 = 0u16;
                            (&raw mut tmp).write_volatile(0u16);
                            'l10: loop {
                                'l11: {
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
                                    break 'l10;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l8;
                    }
                }
                _dest = (_dest).wrapping_offset(4096);
                _size = (_size).wrapping_sub(4096u32);
                if _size <= 4096u32 {
                    'l12: loop {
                        'l13: {
                            {
                                let mut tmp: u16 = 0u16;
                                (&raw mut tmp).write_volatile(0u16);
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
                                        break 'l14;
                                    }
                                }
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l12;
                        }
                    }
                    break 'l7;
                }
            }
        }
        ResetOamRange(0u8, 128u8);
        LoadOam();
        ScanlineEffect_Stop();
        ScanlineEffect_Clear();
        ResetSpriteData();
        ResetTasks();
        ResetPaletteFade();
        InitResetRtcScreenBgAndWindows();
        SetVBlankCallback(Some(VBlankCB));
        SetMainCallback2(Some(CB2_ResetRtcScreen));
        CreateTask(Some(Task_ResetRtcScreen), 80u8);
    }
}
pub(crate) unsafe extern "C" fn InitResetRtcScreenBgAndWindows() {
    unsafe {
        ClearScheduledBgCopiesToVram();
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(4u32, 4u32)) as u8),
        );
        ScheduleBgCopyTilemapToVram(0u8);
        SetGpuReg(0u8, 4160u16);
        ShowBg(0u8);
        InitWindows(((&raw const sWindowTemplates).cast::<u8>().cast_mut()).cast::<u8>());
        DeactivateAllTextPrinters();
        LoadMessageBoxAndBorderGfx();
    }
}
pub(crate) unsafe extern "C" fn CB2_ResetRtcScreen() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        DoScheduledBgTilemapCopiesToVram();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    unsafe {
        ProcessSpriteCopyRequests();
        LoadOam();
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn ShowMessage(str: *mut u8) {
    unsafe {
        let mut str = str;
        DrawDialogFrameWithCustomTileAndPalette(1u8, 0u8, 512u16, 15u8);
        AddTextPrinterParameterized(1u8, 1u8, str, 0u8, 1u8, 0u8, None);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowResetRtcPrompt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                DrawStdFrameWithCustomTileAndPalette(0u8, 0u8, 532u16, 14u8);
                AddTextPrinterParameterized(
                    0u8,
                    1u8,
                    (&raw mut gText_PresentTime).cast::<u8>(),
                    0u8,
                    1u8,
                    255u8,
                    None,
                );
                PrintTime(
                    0u8,
                    0u8,
                    17u8,
                    (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as u16),
                    (((((&raw mut gLocalTime).cast::<u8>())
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as u8),
                    (((((&raw mut gLocalTime).cast::<u8>())
                        .wrapping_add(3)
                        .cast::<i8>())
                    .read()) as u8),
                    (((((&raw mut gLocalTime).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<i8>())
                    .read()) as u8),
                );
                AddTextPrinterParameterized(
                    0u8,
                    1u8,
                    (&raw mut gText_PreviousTime).cast::<u8>(),
                    0u8,
                    33u8,
                    255u8,
                    None,
                );
                PrintTime(
                    0u8,
                    0u8,
                    49u8,
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(160))
                        .cast::<i16>())
                    .read()) as u16),
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(160))
                        .wrapping_add(2)
                        .cast::<i8>())
                    .read()) as u8),
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(160))
                        .wrapping_add(3)
                        .cast::<i8>())
                    .read()) as u8),
                    (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(160))
                        .wrapping_add(4)
                        .cast::<i8>())
                    .read()) as u8),
                );
                ShowMessage((&raw mut gText_ResetRTCConfirmCancel).cast::<u8>());
                CopyWindowToVram(0u8, 2u8);
                ScheduleBgCopyTilemapToVram(0u8);
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 2i32)
                    != 0
                {
                    DestroyTask(taskId);
                    DoSoftReset();
                } else {
                    if ((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 1i32)
                        != 0
                    {
                        PlaySE(5u16);
                        DestroyTask(taskId);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtcScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                BeginNormalPaletteFade(4294967295u32, 1i8, 16u8, 0u8, 65535u16);
                (data).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    if (((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32) == 0i32)
                        || (((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32) == 2i32)
                    {
                        ShowMessage((&raw mut gText_NoSaveFileCantSetTime).cast::<u8>());
                        (data).write(5i16);
                    } else {
                        RtcCalcLocalTime();
                        ((data).wrapping_offset(1))
                            .write(((CreateTask(Some(Task_ShowResetRtcPrompt), 80u8)) as i16));
                        (data).write(2i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(1)).read()) as i32) as isize * 40))
                .wrapping_add(4))
                .read()) as i32)
                    != 1i32
                {
                    ClearStdWindowAndFrameToTransparent(0u8, 0u8);
                    ShowMessage((&raw mut gText_PleaseResetTime).cast::<u8>());
                    (&raw mut gLocalTime)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<8>>()
                        .write_unaligned(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(160)
                                .cast::<crate::c::Rec4<8>>()
                                .read_unaligned(),
                        );
                    ((data).wrapping_offset(1))
                        .write(((CreateTask(Some(Task_ResetRtc_Init), 80u8)) as i16));
                    (data).write(3i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((data).wrapping_offset(1)).read()) as i32) as isize * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .read())
                    != 0
                {
                    if !((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(1)).read()) as i32) as isize * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read())
                        != 0)
                    {
                        DestroyTask(((((data).wrapping_offset(1)).read()) as u8));
                        (data).write(2i16);
                    } else {
                        DestroyTask(((((data).wrapping_offset(1)).read()) as u8));
                        RtcReset();
                        RtcCalcLocalTimeOffset(
                            (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as i32),
                            (((((&raw mut gLocalTime).cast::<u8>())
                                .wrapping_add(2)
                                .cast::<i8>())
                            .read()) as i32),
                            (((((&raw mut gLocalTime).cast::<u8>())
                                .wrapping_add(3)
                                .cast::<i8>())
                            .read()) as i32),
                            (((((&raw mut gLocalTime).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<i8>())
                            .read()) as i32),
                        );
                        (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(160)
                            .cast::<crate::c::Rec4<8>>()
                            .write_unaligned(
                                (&raw mut gLocalTime)
                                    .cast::<u8>()
                                    .cast::<crate::c::Rec4<8>>()
                                    .read_unaligned(),
                            );
                        VarSet(
                            16448u16,
                            (((((&raw mut gLocalTime).cast::<u8>()).cast::<i16>()).read()) as u16),
                        );
                        DisableResetRTC();
                        ShowMessage((&raw mut gText_ClockHasBeenReset).cast::<u8>());
                        (data).write(4i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if ((TrySavingData(0u8)) as i32) == 1i32 {
                    ShowMessage((&raw mut gText_SaveCompleted).cast::<u8>());
                    PlaySE(73u16);
                } else {
                    ShowMessage((&raw mut gText_SaveFailed).cast::<u8>());
                    PlaySE(22u16);
                }
                (data).write(5i16);
            }
            if __fall || __sw1 == 5i32 {
                __fall = true;
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    BeginNormalPaletteFade(4294967295u32, 1i8, 0u8, 16u8, 65535u16);
                    (data).write(6i16);
                } else {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 6i32 {
                __fall = true;
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
                    DoSoftReset();
                }
            }
        }
    }
}
