//! Translated from `src/wallclock.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gLocalTime: Time;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSpecialVar_0x8004: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    static gText_Cancel4: CArray<u8, 0>;
    static gText_Confirm3: CArray<u8, 0>;
    static gText_IsThisTheCorrectTime: CArray<u8, 0>;
    static gWallClockFemale_Pal: CArray<u16, 0>;
    static gWallClockMale_Pal: CArray<u16, 0>;
    static gWallClockStart_Tilemap: CArray<u32, 0>;
    static gWallClockView_Tilemap: CArray<u32, 0>;
    static gWallClock_Gfx: CArray<u32, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
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
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DoScheduledBgTilemapCopiesToVram();
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn EnableInterrupts(a0: u16);
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn GetOverworldTextboxPalettePtr() -> *mut u16;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadCompressedSpriteSheet(a0: *mut CompressedSpriteSheet) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LoadSpritePalettes(a0: *mut SpritePalette);
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
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn LoadWallClockGraphics() {
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x85000000 | _size / 4);
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
                            let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                            volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                            volatile_write(dmaRegs.at(1), _dest as usize as u32);
                            volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                            let _ = (dmaRegs.at(2)).read_volatile();
                        }
                    }
                }
            }
        }
    }
    LZ77UnCompVram(
        gWallClock_Gfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    if gSpecialVar_0x8004 == MALE as u16 {
        LoadPalette(gWallClockMale_Pal.as_ptr().cast_mut() as *mut c_void, 0, 32);
    } else {
        LoadPalette(
            gWallClockFemale_Pal.as_ptr().cast_mut() as *mut c_void,
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
pub(crate) unsafe extern "C" fn WallClockInit() {
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
pub unsafe extern "C" fn CB2_StartWallClock() {
    let mut taskId: u8 = 0;
    let mut spriteId: u8 = 0;
    LoadWallClockGraphics();
    LZ77UnCompVram(
        gWallClockStart_Tilemap.as_ptr().cast_mut(),
        0x6003800 as usize as *mut u16 as *mut c_void,
    );
    taskId = CreateTask(Some(Task_SetClock_WaitFadeIn), 0);
    gTasks[taskId].data[2] = 10;
    gTasks[taskId].data[3] = 0;
    gTasks[taskId].data[4] = 0;
    gTasks[taskId].data[5] = 0;
    gTasks[taskId].data[6] = 0;
    gTasks[taskId].data[0] = 0;
    gTasks[taskId].data[1] = 300;
    spriteId = CreateSprite(
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
        gText_Confirm3.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_BUTTON_LABEL);
    ScheduleBgCopyTilemapToVram(2);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ViewWallClock() {
    let mut taskId: u8 = 0;
    let mut spriteId: u8 = 0;
    let mut angle1: u8 = 0;
    let mut angle2: u8 = 0;
    LoadWallClockGraphics();
    LZ77UnCompVram(
        gWallClockView_Tilemap.as_ptr().cast_mut(),
        0x6003800 as usize as *mut u16 as *mut c_void,
    );
    taskId = CreateTask(Some(Task_ViewClock_WaitFadeIn), 0);
    InitClockWithRtc(taskId);
    if gTasks[taskId].data[5] == PERIOD_AM {
        angle1 = 45;
        angle2 = 90;
    } else {
        angle1 = 90;
        angle2 = 135;
    }
    spriteId = CreateSprite(
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
    gSprites[spriteId].data[1] = angle1 as i16;
    spriteId = CreateSprite((&raw const *sSpriteTemplate_AM).cast_mut(), 120, 80, 2);
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = angle2 as i16;
    WallClockInit();
    AddTextPrinterParameterized(
        WIN_BUTTON_LABEL,
        FONT_NORMAL,
        gText_Cancel4.as_ptr().cast_mut(),
        0,
        1,
        0,
        None,
    );
    PutWindowTilemap(WIN_BUTTON_LABEL);
    ScheduleBgCopyTilemapToVram(2);
}
pub(crate) unsafe extern "C" fn CB2_WallClock() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn Task_SetClock_WaitFadeIn(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gTasks[taskId].func = Some(Task_SetClock_HandleInput);
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_HandleInput(taskId: u8) {
    if gTasks[taskId].data[0] % 6 != 0 {
        gTasks[taskId].data[0] = CalcNewMinHandAngle(
            gTasks[taskId].data[0] as u16,
            gTasks[taskId].data[4] as u8,
            gTasks[taskId].data[6] as u8,
        ) as i16;
    } else {
        gTasks[taskId].data[0] = gTasks[taskId].data[3] * 6;
        gTasks[taskId].data[1] = gTasks[taskId].data[2] % 12 * 30 + gTasks[taskId].data[3] / 10 * 5;
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            gTasks[taskId].func = Some(Task_SetClock_AskConfirm);
        } else {
            gTasks[taskId].data[4] = MOVE_NONE;
            if gMain.heldKeys as i32 & DPAD_LEFT != 0 {
                gTasks[taskId].data[4] = MOVE_BACKWARD as i16;
            }
            if gMain.heldKeys as i32 & DPAD_RIGHT != 0 {
                gTasks[taskId].data[4] = MOVE_FORWARD as i16;
            }
            if gTasks[taskId].data[4] != MOVE_NONE {
                if gTasks[taskId].data[6] < 0xFF {
                    gTasks[taskId].data[6] += 1;
                }
                gTasks[taskId].data[0] = CalcNewMinHandAngle(
                    gTasks[taskId].data[0] as u16,
                    gTasks[taskId].data[4] as u8,
                    gTasks[taskId].data[6] as u8,
                ) as i16;
                AdvanceClock(taskId, gTasks[taskId].data[4] as u8);
            } else {
                gTasks[taskId].data[6] = 0;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_AskConfirm(taskId: u8) {
    DrawStdFrameWithCustomTileAndPalette(WIN_MSG, FALSE, 0x250, 0x0d);
    AddTextPrinterParameterized(
        WIN_MSG,
        FONT_NORMAL,
        gText_IsThisTheCorrectTime.as_ptr().cast_mut(),
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
    gTasks[taskId].func = Some(Task_SetClock_HandleConfirmInput);
}
pub(crate) unsafe extern "C" fn Task_SetClock_HandleConfirmInput(taskId: u8) {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PlaySE(SE_SELECT);
            gTasks[taskId].func = Some(Task_SetClock_Confirmed);
        }
        1 | MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            ClearStdWindowAndFrameToTransparent(WIN_MSG, FALSE);
            ClearWindowTilemap(WIN_MSG);
            gTasks[taskId].func = Some(Task_SetClock_HandleInput);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_SetClock_Confirmed(taskId: u8) {
    RtcInitLocalTimeOffset(gTasks[taskId].data[2] as i32, gTasks[taskId].data[3] as i32);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_SetClock_Exit);
}
pub(crate) unsafe extern "C" fn Task_SetClock_Exit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe extern "C" fn Task_ViewClock_WaitFadeIn(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gTasks[taskId].func = Some(Task_ViewClock_HandleInput);
    }
}
pub(crate) unsafe extern "C" fn Task_ViewClock_HandleInput(taskId: u8) {
    InitClockWithRtc(taskId);
    if gMain.newKeys as i32 & 3 != 0 {
        gTasks[taskId].func = Some(Task_ViewClock_FadeOut);
    }
}
pub(crate) unsafe extern "C" fn Task_ViewClock_FadeOut(taskId: u8) {
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_ViewClock_Exit);
}
pub(crate) unsafe extern "C" fn Task_ViewClock_Exit(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe extern "C" fn CalcMinHandDelta(speed: u16) -> u8 {
    if speed > 60 {
        return 6;
    }
    if speed > 30 {
        return 3;
    }
    if speed > 10 {
        return 2;
    }
    return 1;
}
pub(crate) unsafe extern "C" fn CalcNewMinHandAngle(
    mut angle: u16,
    direction: u8,
    speed: u8,
) -> u16 {
    let mut delta: u8 = CalcMinHandDelta(speed as u16);
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
    return angle;
}
pub(crate) unsafe extern "C" fn AdvanceClock(taskId: u8, direction: u8) -> u32 {
    match direction {
        MOVE_BACKWARD => {
            if gTasks[taskId].data[3] > 0 {
                gTasks[taskId].data[3] -= 1;
            } else {
                gTasks[taskId].data[3] = 59;
                if gTasks[taskId].data[2] > 0 {
                    gTasks[taskId].data[2] -= 1;
                } else {
                    gTasks[taskId].data[2] = 23;
                }
                UpdateClockPeriod(taskId, direction);
            }
        }
        MOVE_FORWARD => {
            if gTasks[taskId].data[3] < 59 {
                gTasks[taskId].data[3] += 1;
            } else {
                gTasks[taskId].data[3] = 0;
                if gTasks[taskId].data[2] < 23 {
                    gTasks[taskId].data[2] += 1;
                } else {
                    gTasks[taskId].data[2] = 0;
                }
                UpdateClockPeriod(taskId, direction);
            }
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn UpdateClockPeriod(taskId: u8, direction: u8) {
    let mut hours: u8 = gTasks[taskId].data[2] as u8;
    match direction {
        MOVE_BACKWARD => match hours {
            11 => {
                gTasks[taskId].data[5] = PERIOD_AM;
            }
            23 => {
                gTasks[taskId].data[5] = PERIOD_PM;
            }
            _ => {}
        },
        MOVE_FORWARD => match hours {
            0 => {
                gTasks[taskId].data[5] = PERIOD_AM;
            }
            12 => {
                gTasks[taskId].data[5] = PERIOD_PM;
            }
            _ => {}
        },
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn InitClockWithRtc(taskId: u8) {
    RtcCalcLocalTime();
    gTasks[taskId].data[2] = gLocalTime.hours as i16;
    gTasks[taskId].data[3] = gLocalTime.minutes as i16;
    gTasks[taskId].data[0] = gTasks[taskId].data[3] * 6;
    gTasks[taskId].data[1] = gTasks[taskId].data[2] % 12 * 30 + gTasks[taskId].data[3] / 10 * 5;
    if gLocalTime.hours < 12 {
        gTasks[taskId].data[5] = PERIOD_AM;
    } else {
        gTasks[taskId].data[5] = PERIOD_PM;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MinuteHand(sprite: *mut Sprite) {
    let mut angle: u16 = gTasks[(*sprite).data[0]].data[0] as u16;
    let mut sin: i16 = Sin2(angle) / 16;
    let mut cos: i16 = Cos2(angle) / 16;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    SetOamMatrix(
        0,
        cos as u16,
        sin as u16,
        (sin as u16).wrapping_neg(),
        cos as u16,
    );
    x = sClockHandCoords[angle][0] as u16;
    y = sClockHandCoords[angle][1] as u16;
    if x > 128 {
        x |= 0xff00;
    }
    if y > 128 {
        y |= 0xff00;
    }
    (*sprite).x2 = x as i16;
    (*sprite).y2 = y as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_HourHand(sprite: *mut Sprite) {
    let mut angle: u16 = gTasks[(*sprite).data[0]].data[1] as u16;
    let mut sin: i16 = Sin2(angle) / 16;
    let mut cos: i16 = Cos2(angle) / 16;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    SetOamMatrix(
        1,
        cos as u16,
        sin as u16,
        (sin as u16).wrapping_neg(),
        cos as u16,
    );
    x = sClockHandCoords[angle][0] as u16;
    y = sClockHandCoords[angle][1] as u16;
    if x > 128 {
        x |= 0xff00;
    }
    if y > 128 {
        y |= 0xff00;
    }
    (*sprite).x2 = x as i16;
    (*sprite).y2 = y as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_PMIndicator(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[0]].data[5] != PERIOD_AM {
        if (*sprite).data[1] >= 60 && (*sprite).data[1] < 90 {
            (*sprite).data[1] += 5;
        }
        if (*sprite).data[1] < 60 {
            (*sprite).data[1] += 1;
        }
    } else {
        if (*sprite).data[1] >= 46 && (*sprite).data[1] < 76 {
            (*sprite).data[1] -= 5;
        }
        if (*sprite).data[1] > 75 {
            (*sprite).data[1] -= 1;
        }
    }
    (*sprite).x2 = (Cos2((*sprite).data[1] as u16) as i32 * 30 / 4096) as i16;
    (*sprite).y2 = (Sin2((*sprite).data[1] as u16) as i32 * 30 / 4096) as i16;
}
pub(crate) unsafe extern "C" fn SpriteCB_AMIndicator(sprite: *mut Sprite) {
    if gTasks[(*sprite).data[0]].data[5] != PERIOD_AM {
        if (*sprite).data[1] >= 105 && (*sprite).data[1] < 135 {
            (*sprite).data[1] += 5;
        }
        if (*sprite).data[1] < 105 {
            (*sprite).data[1] += 1;
        }
    } else {
        if (*sprite).data[1] >= 91 && (*sprite).data[1] < 121 {
            (*sprite).data[1] -= 5;
        }
        if (*sprite).data[1] > 120 {
            (*sprite).data[1] -= 1;
        }
    }
    (*sprite).x2 = (Cos2((*sprite).data[1] as u16) as i32 * 30 / 4096) as i16;
    (*sprite).y2 = (Sin2((*sprite).data[1] as u16) as i32 * 30 / 4096) as i16;
}
