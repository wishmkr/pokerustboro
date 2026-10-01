//! Translated from `src/wallclock.c` by tools/rustport/c2rs.py.
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
    clippy::useless_transmute,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::{ChangeBgX, ChangeBgY, ResetBgsAndClearDma3BusyFlags, ShowBg};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::ffi::gSpecialVar_0x8004;
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::menu::{
    ClearScheduledBgCopiesToVram, ClearStdWindowAndFrameToTransparent, CreateYesNoMenu,
    DoScheduledBgTilemapCopiesToVram, DrawStdFrameWithCustomTileAndPalette,
    Menu_ProcessInputNoWrapClearOnChoose, ScheduleBgCopyTilemapToVram,
};
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::rtc::{RtcCalcLocalTime, RtcInitLocalTimeOffset, gLocalTime};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData, SetOamMatrix,
};
use crate::task::{ResetTasks, RunTasks};
use crate::task::{task_get, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::text_window::LoadUserWindowBorderGfx;
use crate::trig::{Cos2, Sin2};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{ClearWindowTilemap, FreeAllWindowBuffers, PutWindowTilemap};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
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
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const sTaskId: usize = 0;
const tMinuteHandAngle: usize = 0;
const sAngle: usize = 1;
const tHourHandAngle: usize = 1;
const tHours: usize = 2;
const tMinutes: usize = 3;
const tMoveDir: usize = 4;
const tPeriod: usize = 5;
const tMoveSpeed: usize = 6;
// Data tables (translate with cdata.py): sHand_Gfx sTextPrompt_Pal sWindowTemplates sWindowTemplate_ConfirmYesNo sBgTemplates sSpriteSheet_ClockHand sUnused sSpritePalettes_Clock sOam_ClockHand sAnim_MinuteHand sAnim_HourHand sAnims_MinuteHand sAnims_HourHand sSpriteTemplate_MinuteHand sSpriteTemplate_HourHand sOam_PeriodIndicator sAnim_PM sAnim_AM sAnims_PM sAnims_AM sSpriteTemplate_PM sSpriteTemplate_AM sClockHandCoords

const MOVE_BACKWARD: u8 = 1;
const MOVE_FORWARD: u8 = 2;
const MOVE_NONE: i16 = 0;
const PERIOD_AM: i16 = 0;
const PERIOD_PM: i16 = 1;
const WIN_BUTTON_LABEL: u8 = 1;
const WIN_MSG: u8 = 0;

static sBgTemplates: Table<CArray<BgTemplate, 3>> =
    Table((&raw const crate::data::wallclock::sBgTemplates).cast());
static sClockHandCoords: Table<CArray<CArray<i8, 2>, 360>> =
    Table((&raw const crate::data::wallclock::sClockHandCoords).cast());
static sSpritePalettes_Clock: Table<CArray<SpritePalette, 3>> =
    Table((&raw const crate::data::wallclock::sSpritePalettes_Clock).cast());
static sSpriteSheet_ClockHand: Table<CompressedSpriteSheet> =
    Table((&raw const crate::data::wallclock::sSpriteSheet_ClockHand).cast());
static sSpriteTemplate_AM: Table<SpriteTemplate> =
    Table((&raw const crate::data::wallclock::sSpriteTemplate_AM).cast());
static sSpriteTemplate_HourHand: Table<SpriteTemplate> =
    Table((&raw const crate::data::wallclock::sSpriteTemplate_HourHand).cast());
static sSpriteTemplate_MinuteHand: Table<SpriteTemplate> =
    Table((&raw const crate::data::wallclock::sSpriteTemplate_MinuteHand).cast());
static sSpriteTemplate_PM: Table<SpriteTemplate> =
    Table((&raw const crate::data::wallclock::sSpriteTemplate_PM).cast());
static sTextPrompt_Pal: Table<CArray<u16, 4>> =
    Table((&raw const crate::data::wallclock::sTextPrompt_Pal).cast());
static sWindowTemplate_ConfirmYesNo: Table<WindowTemplate> =
    Table((&raw const crate::data::wallclock::sWindowTemplate_ConfirmYesNo).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::wallclock::sWindowTemplates).cast());

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
/// `GetOverworldTextboxPalettePtr` with this module's view of its types.
#[inline]
unsafe fn GetOverworldTextboxPalettePtr() -> *mut u16 {
    unsafe { crate::text_window::GetOverworldTextboxPalettePtr() as *mut u16 }
}
/// `LZ77UnCompVram` with this module's view of its types.
#[inline]
unsafe fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::syscall::LZ77UnCompVram(a0 as _, a1 as _);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn VBlankCB_WallClock() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
unsafe fn LoadWallClockGraphics() {
    SetVBlankCallback(None);
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG3CNT, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    ChangeBgX(2, 0, BG_COORD_SET);
    ChangeBgY(2, 0, BG_COORD_SET);
    ChangeBgX(3, 0, BG_COORD_SET);
    ChangeBgY(3, 0, BG_COORD_SET);
    {
        let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
        let mut _size: u32 = VRAM_SIZE;
        loop {
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000800);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
            _dest = (_dest as *mut u8).at(4096) as *mut c_void;
            _size -= 0x1000;
            if _size <= 0x1000 {
                {
                    {
                        let mut tmp: u16 = 0;
                        volatile_write(&raw mut tmp, 0);
                        {
                            {
                                let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                                let _ = (dmaRegs.at(2)).read_volatile();
                            }
                        }
                    }
                }
                break;
            }
        }
    }
    {
        {
            let mut _dest: *mut u32 = OAM as i32 as usize as *mut c_void as *mut u32;
            let mut _size: u32 = OAM_SIZE;
            {
                {
                    let mut tmp: u32 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | (_size / 4));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    {
        {
            let mut _dest: *mut u16 = PLTT as i32 as usize as *mut c_void as *mut u16;
            let mut _size: u32 = PLTT_SIZE;
            {
                {
                    let mut tmp: u16 = 0;
                    volatile_write(&raw mut tmp, 0);
                    {
                        {
                            let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | (_size / 2));
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        (*(&raw const crate::data::graphics::gWallClock_Gfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        VRAM as usize as *mut c_void,
    );
    if gSpecialVar_0x8004 == MALE as u16 {
        LoadPalette(
            (*(&raw const crate::data::graphics::gWallClockMale_Pal).cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
            0,
            32,
        );
    } else {
        LoadPalette(
            (*(&raw const crate::data::graphics::gWallClockFemale_Pal).cast::<CArray<u16, 0>>())
                .as_ptr()
                .cast_mut() as *mut c_void,
            0,
            32,
        );
    }
    LoadPalette(GetOverworldTextboxPalettePtr() as *mut c_void, 224, 32);
    LoadPalette(sTextPrompt_Pal.as_ptr().cast_mut() as *mut c_void, 192, 8);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 3);
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadUserWindowBorderGfx(0, 0x250, 208);
    ClearScheduledBgCopiesToVram();
    ScanlineEffect_Stop();
    ResetTasks();
    ResetSpriteData();
    ResetPaletteFade();
    FreeAllSpritePalettes();
    LoadCompressedSpriteSheet((&raw const *sSpriteSheet_ClockHand).cast_mut());
    LoadSpritePalettes(sSpritePalettes_Clock.as_ptr().cast_mut());
}
unsafe fn WallClockInit() {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    EnableInterrupts(INTR_FLAG_VBLANK);
    SetVBlankCallback(Some(VBlankCB_WallClock));
    SetMainCallback2(Some(CB2_WallClock));
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    ShowBg(2);
    ShowBg(3);
}
#[unsafe(no_mangle)]
pub unsafe fn CB2_StartWallClock() {
    LoadWallClockGraphics();
    LZ77UnCompVram(
        (*(&raw const crate::data::graphics::gWallClockStart_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x6003800_usize as *mut u16 as *mut c_void,
    );
    let taskId: u8 = CreateTask(Some(Task_SetClock_WaitFadeIn), 0);
    task_set(taskId, tHours, 10);
    task_set(taskId, tMinutes, 0);
    task_set(taskId, tMoveDir, 0);
    task_set(taskId, tPeriod, 0);
    task_set(taskId, tMoveSpeed, 0);
    task_set(taskId, 0, 0);
    task_set(taskId, 1, 300);
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_MinuteHand).cast_mut(),
        120,
        80,
        1,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_matrixNum(0);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_HourHand).cast_mut(),
        120,
        80,
        0,
    );
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_matrixNum(1);
    spriteId = CreateSprite((&raw const *sSpriteTemplate_PM).cast_mut(), 120, 80, 2);
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = 45;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_AM).cast_mut(), 120, 80, 2);
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = 90;
    WallClockInit();
    AddTextPrinterParameterized(
        WIN_BUTTON_LABEL,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Confirm3).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_BUTTON_LABEL);
    ScheduleBgCopyTilemapToVram(2);
}
pub unsafe fn CB2_ViewWallClock() {
    let mut angle1: u8 = 0;
    let mut angle2: u8 = 0;
    LoadWallClockGraphics();
    LZ77UnCompVram(
        (*(&raw const crate::data::graphics::gWallClockView_Tilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        0x6003800_usize as *mut u16 as *mut c_void,
    );
    let taskId: u8 = CreateTask(Some(Task_ViewClock_WaitFadeIn), 0);
    InitClockWithRtc(taskId);
    if task_get(taskId, tPeriod) == PERIOD_AM {
        angle1 = 45;
        angle2 = 90;
    } else {
        angle1 = 90;
        angle2 = 135;
    }
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sSpriteTemplate_MinuteHand).cast_mut(),
        120,
        80,
        1,
    );
    gSprites[spriteId].data[sTaskId] = taskId as i16;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_matrixNum(0);
    spriteId = CreateSprite(
        (&raw const *sSpriteTemplate_HourHand).cast_mut(),
        120,
        80,
        0,
    );
    gSprites[spriteId].data[sTaskId] = taskId as i16;
    gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[spriteId].oam.set_matrixNum(1);
    spriteId = CreateSprite((&raw const *sSpriteTemplate_PM).cast_mut(), 120, 80, 2);
    gSprites[spriteId].data[sTaskId] = taskId as i16;
    gSprites[spriteId].data[1] = angle1 as i16;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_AM).cast_mut(), 120, 80, 2);
    gSprites[spriteId].data[sTaskId] = taskId as i16;
    gSprites[spriteId].data[1] = angle2 as i16;
    WallClockInit();
    AddTextPrinterParameterized(
        WIN_BUTTON_LABEL,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Cancel4).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_BUTTON_LABEL);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe fn CB2_WallClock() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe fn Task_SetClock_WaitFadeIn(taskId: u8) {
    if gPaletteFade.active() == 0 {
        task_set_func(taskId, Some(Task_SetClock_HandleInput));
    }
}
pub(crate) unsafe fn Task_SetClock_HandleInput(taskId: u8) {
    if task_get(taskId, tMinuteHandAngle) % 6 != 0 {
        task_set(
            taskId,
            tMinuteHandAngle,
            CalcNewMinHandAngle(
                task_get(taskId, tMinuteHandAngle) as u16,
                task_get(taskId, tMoveDir) as u8,
                task_get(taskId, tMoveSpeed) as u8,
            ) as i16,
        );
    } else {
        task_set(taskId, tMinuteHandAngle, task_get(taskId, tMinutes) * 6);
        task_set(
            taskId,
            tHourHandAngle,
            task_get(taskId, tHours) % 12 * 30 + task_get(taskId, tMinutes) / 10 * 5,
        );
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            task_set_func(taskId, Some(Task_SetClock_AskConfirm));
        } else {
            task_set(taskId, tMoveDir, MOVE_NONE);
            if gMain.heldKeys as i32 & DPAD_LEFT != 0 {
                task_set(taskId, tMoveDir, MOVE_BACKWARD as i16);
            }
            if gMain.heldKeys as i32 & DPAD_RIGHT != 0 {
                task_set(taskId, tMoveDir, MOVE_FORWARD as i16);
            }
            if task_get(taskId, tMoveDir) != MOVE_NONE {
                if task_get(taskId, tMoveSpeed) < 0xFF {
                    task_set(taskId, tMoveSpeed, task_get(taskId, tMoveSpeed) + 1);
                }
                task_set(
                    taskId,
                    tMinuteHandAngle,
                    CalcNewMinHandAngle(
                        task_get(taskId, tMinuteHandAngle) as u16,
                        task_get(taskId, tMoveDir) as u8,
                        task_get(taskId, tMoveSpeed) as u8,
                    ) as i16,
                );
                AdvanceClock(taskId, task_get(taskId, tMoveDir) as u8);
            } else {
                task_set(taskId, tMoveSpeed, 0);
            }
        }
    }
}
pub(crate) unsafe fn Task_SetClock_AskConfirm(taskId: u8) {
    DrawStdFrameWithCustomTileAndPalette(WIN_MSG, FALSE, 0x250, 0x0d);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_IsThisTheCorrectTime).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_MSG);
    ScheduleBgCopyTilemapToVram(0);
    CreateYesNoMenu(
        (&raw const *sWindowTemplate_ConfirmYesNo).cast_mut(),
        0x250,
        0x0d,
        1,
    );
    task_set_func(taskId, Some(Task_SetClock_HandleConfirmInput));
}
pub(crate) unsafe fn Task_SetClock_HandleConfirmInput(taskId: u8) {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PlaySE(SE_SELECT);
            task_set_func(taskId, Some(Task_SetClock_Confirmed));
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            ClearStdWindowAndFrameToTransparent(WIN_MSG, FALSE);
            ClearWindowTilemap(WIN_MSG);
            task_set_func(taskId, Some(Task_SetClock_HandleInput));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_SetClock_Confirmed(taskId: u8) {
    RtcInitLocalTimeOffset(
        task_get(taskId, tHours) as i32,
        task_get(taskId, tMinutes) as i32,
    );
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_SetClock_Exit));
}
pub(crate) unsafe fn Task_SetClock_Exit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe fn Task_ViewClock_WaitFadeIn(taskId: u8) {
    if gPaletteFade.active() == 0 {
        task_set_func(taskId, Some(Task_ViewClock_HandleInput));
    }
}
pub(crate) unsafe fn Task_ViewClock_HandleInput(taskId: u8) {
    InitClockWithRtc(taskId);
    if gMain.newKeys as i32 & 3 != 0 {
        task_set_func(taskId, Some(Task_ViewClock_FadeOut));
    }
}
pub(crate) unsafe fn Task_ViewClock_FadeOut(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_ViewClock_Exit));
}
pub(crate) unsafe fn Task_ViewClock_Exit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(gMain.savedCallback);
    }
}
fn CalcMinHandDelta(speed: u16) -> u8 {
    if speed > 60 {
        return 6;
    }
    if speed > 30 {
        return 3;
    }
    if speed > 10 {
        return 2;
    }
    1
}
fn CalcNewMinHandAngle(mut angle: u16, direction: u8, speed: u8) -> u16 {
    let delta: u8 = CalcMinHandDelta(speed as u16);
    match direction {
        MOVE_BACKWARD => {
            if angle != 0 {
                angle -= delta as u16;
            } else {
                angle = 360 - delta as u16;
            }
        }
        MOVE_FORWARD => {
            if (angle as i32) < 360 - delta as i32 {
                angle += delta as u16;
            } else {
                angle = 0;
            }
        }
        _ => {}
    }
    angle
}
fn AdvanceClock(taskId: u8, direction: u8) -> u32 {
    match direction {
        MOVE_BACKWARD => {
            if task_get(taskId, tMinutes) > 0 {
                task_set(taskId, tMinutes, task_get(taskId, tMinutes) - 1);
            } else {
                task_set(taskId, tMinutes, 59);
                if task_get(taskId, tHours) > 0 {
                    task_set(taskId, tHours, task_get(taskId, tHours) - 1);
                } else {
                    task_set(taskId, tHours, 23);
                }
                UpdateClockPeriod(taskId, direction);
            }
        }
        MOVE_FORWARD => {
            if task_get(taskId, tMinutes) < 59 {
                task_set(taskId, tMinutes, task_get(taskId, tMinutes) + 1);
            } else {
                task_set(taskId, tMinutes, 0);
                if task_get(taskId, tHours) < 23 {
                    task_set(taskId, tHours, task_get(taskId, tHours) + 1);
                } else {
                    task_set(taskId, tHours, 0);
                }
                UpdateClockPeriod(taskId, direction);
            }
        }
        _ => {}
    }
    FALSE as u32
}
fn UpdateClockPeriod(taskId: u8, direction: u8) {
    let hours: u8 = task_get(taskId, tHours) as u8;
    match direction {
        MOVE_BACKWARD => match hours {
            11 => {
                task_set(taskId, tPeriod, PERIOD_AM);
            }
            23 => {
                task_set(taskId, tPeriod, PERIOD_PM);
            }
            _ => {}
        },
        MOVE_FORWARD => match hours {
            0 => {
                task_set(taskId, tPeriod, PERIOD_AM);
            }
            12 => {
                task_set(taskId, tPeriod, PERIOD_PM);
            }
            _ => {}
        },
        _ => {}
    }
}
unsafe fn InitClockWithRtc(taskId: u8) {
    RtcCalcLocalTime();
    task_set(taskId, tHours, gLocalTime.hours as i16);
    task_set(taskId, tMinutes, gLocalTime.minutes as i16);
    task_set(taskId, tMinuteHandAngle, task_get(taskId, tMinutes) * 6);
    task_set(
        taskId,
        tHourHandAngle,
        task_get(taskId, tHours) % 12 * 30 + task_get(taskId, tMinutes) / 10 * 5,
    );
    if gLocalTime.hours < 12 {
        task_set(taskId, tPeriod, PERIOD_AM);
    } else {
        task_set(taskId, tPeriod, PERIOD_PM);
    }
}
pub(crate) unsafe fn SpriteCB_MinuteHand(sprite: *mut Sprite) {
    let angle: u16 = task_get((*sprite).data[0], 0) as u16;
    let sin: i16 = Sin2(angle) / 16;
    let cos: i16 = Cos2(angle) / 16;
    SetOamMatrix(
        0,
        cos as u16,
        sin as u16,
        (sin as u16).wrapping_neg(),
        cos as u16,
    );
    let mut x: u16 = sClockHandCoords[angle][0] as u16;
    let mut y: u16 = sClockHandCoords[angle][1] as u16;
    if x > 128 {
        x |= 0xff00;
    }
    if y > 128 {
        y |= 0xff00;
    }
    (*sprite).x2 = x as i16;
    (*sprite).y2 = y as i16;
}
pub(crate) unsafe fn SpriteCB_HourHand(sprite: *mut Sprite) {
    let angle: u16 = task_get((*sprite).data[sTaskId], tHourHandAngle) as u16;
    let sin: i16 = Sin2(angle) / 16;
    let cos: i16 = Cos2(angle) / 16;
    SetOamMatrix(
        1,
        cos as u16,
        sin as u16,
        (sin as u16).wrapping_neg(),
        cos as u16,
    );
    let mut x: u16 = sClockHandCoords[angle][0] as u16;
    let mut y: u16 = sClockHandCoords[angle][1] as u16;
    if x > 128 {
        x |= 0xff00;
    }
    if y > 128 {
        y |= 0xff00;
    }
    (*sprite).x2 = x as i16;
    (*sprite).y2 = y as i16;
}
pub(crate) unsafe fn SpriteCB_PMIndicator(sprite: *mut Sprite) {
    if task_get((*sprite).data[sTaskId], tPeriod) != PERIOD_AM {
        if (*sprite).data[sAngle] >= 60 && (*sprite).data[sAngle] < 90 {
            (*sprite).data[sAngle] += 5;
        }
        if (*sprite).data[sAngle] < 60 {
            (*sprite).data[sAngle] += 1;
        }
    } else {
        if (*sprite).data[sAngle] >= 46 && (*sprite).data[sAngle] < 76 {
            (*sprite).data[sAngle] -= 5;
        }
        if (*sprite).data[sAngle] > 75 {
            (*sprite).data[sAngle] -= 1;
        }
    }
    (*sprite).x2 = (Cos2((*sprite).data[sAngle] as u16) as i32 * 30 / 4096) as i16;
    (*sprite).y2 = (Sin2((*sprite).data[sAngle] as u16) as i32 * 30 / 4096) as i16;
}
pub(crate) unsafe fn SpriteCB_AMIndicator(sprite: *mut Sprite) {
    if task_get((*sprite).data[sTaskId], tPeriod) != PERIOD_AM {
        if (*sprite).data[sAngle] >= 105 && (*sprite).data[sAngle] < 135 {
            (*sprite).data[sAngle] += 5;
        }
        if (*sprite).data[sAngle] < 105 {
            (*sprite).data[sAngle] += 1;
        }
    } else {
        if (*sprite).data[sAngle] >= 91 && (*sprite).data[sAngle] < 121 {
            (*sprite).data[sAngle] -= 5;
        }
        if (*sprite).data[sAngle] > 120 {
            (*sprite).data[sAngle] -= 1;
        }
    }
    (*sprite).x2 = (Cos2((*sprite).data[sAngle] as u16) as i32 * 30 / 4096) as i16;
    (*sprite).y2 = (Sin2((*sprite).data[sAngle] as u16) as i32 * 30 / 4096) as i16;
}
