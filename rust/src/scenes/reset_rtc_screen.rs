//! Translated from `src/reset_rtc_screen.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sBgTemplates sWindowTemplates sInputTimeWindow sInputMap sOamData_Arrow sArrowDown_Gfx sArrowRight_Gfx sArrow_Pal sPicTable_Arrow sSpritePalette_Arrow sAnim_Arrow_Down sAnim_Arrow_Up sAnim_Arrow_Right sAnims_Arrow sSpriteTemplate_Arrow

/// `struct ResetRtcInputMap`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct ResetRtcInputMap {
    pub dataIndex: u8,
    pub minVal: u16,
    pub maxVal: u16,
    pub left: u8,
    pub right: u8,
    pub unk: u8,
}

unsafe impl Sync for ResetRtcInputMap {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<ResetRtcInputMap>() == 12);
    assert!(offset_of!(ResetRtcInputMap, dataIndex) == 0);
    assert!(offset_of!(ResetRtcInputMap, minVal) == 2);
    assert!(offset_of!(ResetRtcInputMap, maxVal) == 4);
    assert!(offset_of!(ResetRtcInputMap, left) == 6);
    assert!(offset_of!(ResetRtcInputMap, right) == 7);
    assert!(offset_of!(ResetRtcInputMap, unk) == 8);
};

const ARROW_DOWN: u8 = 0;
const ARROW_RIGHT: u8 = 2;
const ARROW_UP: u8 = 1;
const DATAIDX_DAYS: i32 = 3;
const DATAIDX_HOURS: i32 = 4;
const DATAIDX_MINS: i32 = 5;
const DATAIDX_SECS: i32 = 6;
const MAINSTATE_CHECK_SAVE: i16 = 1;
const MAINSTATE_EXIT: i16 = 6;
const MAINSTATE_FADE_IN: i16 = 0;
const MAINSTATE_SAVE: i16 = 4;
const MAINSTATE_START_SET_TIME: i16 = 2;
const MAINSTATE_WAIT_EXIT: i16 = 5;
const MAINSTATE_WAIT_SET_TIME: i16 = 3;
const SELECTION_CONFIRM: i32 = 5;
const SELECTION_DAYS: i32 = 1;
const SELECTION_HOURS: i32 = 2;
const SELECTION_MINS: i32 = 3;
const SELECTION_NONE: i32 = 6;
const SELECTION_SECS: i32 = 4;
const WIN_MSG: u8 = 1;
const WIN_TIME: u8 = 0;

static sBgTemplates: Table<CArray<BgTemplate, 1>> =
    Table((&raw const crate::data::reset_rtc_screen::sBgTemplates).cast());
static sInputMap: Table<CArray<ResetRtcInputMap, 5>> =
    Table((&raw const crate::data::reset_rtc_screen::sInputMap).cast());
static sInputTimeWindow: Table<WindowTemplate> =
    Table((&raw const crate::data::reset_rtc_screen::sInputTimeWindow).cast());
static sSpritePalette_Arrow: Table<SpritePalette> =
    Table((&raw const crate::data::reset_rtc_screen::sSpritePalette_Arrow).cast());
static sSpriteTemplate_Arrow: Table<SpriteTemplate> =
    Table((&raw const crate::data::reset_rtc_screen::sSpriteTemplate_Arrow).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::reset_rtc_screen::sWindowTemplates).cast());

unsafe extern "C" {
    static mut gLocalTime: Time;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSaveFileStatus: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_ClockHasBeenReset: CArray<u8, 0>;
    static gText_Colon3: CArray<u8, 0>;
    static gText_Confirm2: CArray<u8, 0>;
    static gText_Day: CArray<u8, 0>;
    static gText_NoSaveFileCantSetTime: CArray<u8, 0>;
    static gText_PleaseResetTime: CArray<u8, 0>;
    static gText_PresentTime: CArray<u8, 0>;
    static gText_PreviousTime: CArray<u8, 0>;
    static gText_ResetRTCConfirmCancel: CArray<u8, 0>;
    static gText_SaveCompleted: CArray<u8, 0>;
    static gText_SaveFailed: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn ClearScheduledBgCopiesToVram();
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DeactivateAllTextPrinters();
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DisableResetRTC();
    fn DoScheduledBgTilemapCopiesToVram();
    fn DoSoftReset();
    fn DrawDialogFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn DrawStdFrameWithCustomTileAndPalette(a0: u8, a1: u8, a2: u16, a3: u8);
    fn FreeAllWindowBuffers();
    fn FreeSpritePaletteByTag(a0: u16);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadOam();
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
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

pub(crate) unsafe extern "C" fn SpriteCB_Cursor_UpOrRight(sprite: *mut Sprite) {
    let mut state: i32 = gTasks[(*sprite).data[0]].data[2] as i32;
    if state != (*sprite).data[1] as i32 {
        (*sprite).data[1] = state as i16;
        match state {
            SELECTION_DAYS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_UP;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 53;
                (*sprite).y = 68;
            }
            SELECTION_HOURS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_UP;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 86;
                (*sprite).y = 68;
            }
            SELECTION_MINS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_UP;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 101;
                (*sprite).y = 68;
            }
            SELECTION_SECS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_UP;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 116;
                (*sprite).y = 68;
            }
            SELECTION_CONFIRM => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_RIGHT;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 153;
                (*sprite).y = 80;
            }
            SELECTION_NONE => {
                DestroySprite(sprite);
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Cursor_Down(sprite: *mut Sprite) {
    let mut state: i32 = gTasks[(*sprite).data[0]].data[2] as i32;
    if state != (*sprite).data[1] as i32 {
        (*sprite).data[1] = state as i16;
        match state {
            SELECTION_DAYS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_DOWN;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 53;
                (*sprite).y = 92;
            }
            SELECTION_HOURS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_DOWN;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 86;
                (*sprite).y = 92;
            }
            SELECTION_MINS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_DOWN;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 101;
                (*sprite).y = 92;
            }
            SELECTION_SECS => {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).animNum = ARROW_DOWN;
                (*sprite).set_animDelayCounter(0);
                (*sprite).x = 116;
                (*sprite).y = 92;
            }
            SELECTION_CONFIRM => {
                (*sprite).set_invisible(TRUE as u16);
            }
            SELECTION_NONE => {
                DestroySprite(sprite);
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn CreateCursor(taskId: u8) {
    let mut spriteId: u32 = 0;
    LoadSpritePalette((&raw const *sSpritePalette_Arrow).cast_mut());
    spriteId = CreateSpriteAtEnd((&raw const *sSpriteTemplate_Arrow).cast_mut(), 53, 68, 0) as u32;
    gSprites[spriteId].callback = Some(SpriteCB_Cursor_UpOrRight);
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = -1;
    spriteId = CreateSpriteAtEnd((&raw const *sSpriteTemplate_Arrow).cast_mut(), 53, 68, 0) as u32;
    gSprites[spriteId].callback = Some(SpriteCB_Cursor_Down);
    gSprites[spriteId].data[0] = taskId as i16;
    gSprites[spriteId].data[1] = -1;
}
pub(crate) unsafe extern "C" fn FreeCursorPalette() {
    FreeSpritePaletteByTag(sSpritePalette_Arrow.tag);
}
pub(crate) unsafe extern "C" fn HideChooseTimeWindow(windowId: u8) {
    ClearStdWindowAndFrameToTransparent(windowId, FALSE);
    RemoveWindow(windowId);
    ScheduleBgCopyTilemapToVram(0);
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
    let mut dest: *mut u8 = gStringVar4.as_mut_ptr();
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        days as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        4,
    );
    dest = StringCopy(dest, gStringVar1.as_mut_ptr());
    dest = StringCopy(dest, gText_Day.as_ptr().cast_mut());
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        hours as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        3,
    );
    dest = StringCopy(dest, gStringVar1.as_mut_ptr());
    dest = StringCopy(dest, gText_Colon3.as_ptr().cast_mut());
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        minutes as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    dest = StringCopy(dest, gStringVar1.as_mut_ptr());
    dest = StringCopy(dest, gText_Colon3.as_ptr().cast_mut());
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        seconds as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    dest = StringCopy(dest, gStringVar1.as_mut_ptr());
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        x,
        y,
        TEXT_SKIP_DRAW,
        None,
    );
}
pub(crate) unsafe extern "C" fn ShowChooseTimeWindow(
    windowId: u8,
    days: u16,
    hours: u8,
    minutes: u8,
    seconds: u8,
) {
    DrawStdFrameWithCustomTileAndPalette(windowId, FALSE, 0x214, 0xE);
    PrintTime(windowId, 0, 1, days, hours, minutes, seconds);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_Confirm2.as_ptr().cast_mut(),
        126,
        1,
        0,
        None,
    );
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn MoveTimeUpDown(
    val: *mut i16,
    minVal: i32,
    maxVal: i32,
    keys: u16,
) -> u32 {
    if keys as i32 & DPAD_DOWN != 0 {
        *val -= 1;
        if (*val as i32) < minVal {
            *val = maxVal as i16;
        }
    } else if keys as i32 & DPAD_UP != 0 {
        *val += 1;
        if *val as i32 > maxVal {
            *val = minVal as i16;
        }
    } else if keys as i32 & DPAD_LEFT != 0 {
        *val -= 10;
        if (*val as i32) < minVal {
            *val = maxVal as i16;
        }
    } else if keys as i32 & DPAD_RIGHT != 0 {
        *val += 10;
        if *val as i32 > maxVal {
            *val = minVal as i16;
        }
    } else {
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_SetFinished(taskId: u8) {
    gTasks[taskId].data[0] = TRUE as i16;
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_Exit(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    HideChooseTimeWindow(*data.at(8) as u8);
    FreeCursorPalette();
    gTasks[taskId].func = Some(Task_ResetRtc_SetFinished);
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_HandleInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut selection: u8 = *data.at(2) as u8;
    let mut selectionInfo: *mut ResetRtcInputMap =
        (&raw const sInputMap[selection as i32 - 1]).cast_mut();
    if gMain.newKeys as i32 & B_BUTTON != 0 {
        gTasks[taskId].func = Some(Task_ResetRtc_Exit);
        *data.at(1) = FALSE as i16;
        *data.at(2) = SELECTION_NONE as i16;
        PlaySE(SE_SELECT);
        return;
    }
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if (*selectionInfo).right != 0 {
            *data.at(2) = (*selectionInfo).right as i16;
            PlaySE(SE_SELECT);
            return;
        }
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if (*selectionInfo).left != 0 {
            *data.at(2) = (*selectionInfo).left as i16;
            PlaySE(SE_SELECT);
            return;
        }
    }
    if selection == SELECTION_CONFIRM as u8 {
        if gMain.newKeys as i32 & A_BUTTON != 0 {
            gLocalTime.days = *data.at(3);
            gLocalTime.hours = *data.at(4) as i8;
            gLocalTime.minutes = *data.at(5) as i8;
            gLocalTime.seconds = *data.at(6) as i8;
            PlaySE(SE_SELECT);
            gTasks[taskId].func = Some(Task_ResetRtc_Exit);
            *data.at(1) = TRUE as i16;
            *data.at(2) = SELECTION_NONE as i16;
        }
    } else if MoveTimeUpDown(
        data.at((*selectionInfo).dataIndex),
        (*selectionInfo).minVal as i32,
        (*selectionInfo).maxVal as i32,
        gMain.newAndRepeatedKeys & 192,
    ) != 0
    {
        PlaySE(SE_SELECT);
        PrintTime(
            *data.at(8) as u8,
            0,
            1,
            *data.at(3) as u16,
            *data.at(4) as u8,
            *data.at(5) as u8,
            *data.at(6) as u8,
        );
        CopyWindowToVram(*data.at(8) as u8, COPYWIN_GFX);
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtc_Init(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data = FALSE as i16;
    *data.at(3) = gLocalTime.days;
    *data.at(4) = gLocalTime.hours as i16;
    *data.at(5) = gLocalTime.minutes as i16;
    *data.at(6) = gLocalTime.seconds as i16;
    *data.at(8) = AddWindow((&raw const *sInputTimeWindow).cast_mut()) as i16;
    ShowChooseTimeWindow(
        *data.at(8) as u8,
        *data.at(3) as u16,
        *data.at(4) as u8,
        *data.at(5) as u8,
        *data.at(6) as u8,
    );
    CreateCursor(taskId);
    *data.at(2) = SELECTION_HOURS as i16;
    gTasks[taskId].func = Some(Task_ResetRtc_HandleInput);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitResetRtcScreen() {
    SetGpuReg(0x0, 0);
    SetVBlankCallback(None);
    {
        {
            let mut _dest: *mut u16 = PLTT as i32 as usize as *mut u16;
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
    ResetOamRange(0, 128);
    LoadOam();
    ScanlineEffect_Stop();
    ScanlineEffect_Clear();
    ResetSpriteData();
    ResetTasks();
    ResetPaletteFade();
    InitResetRtcScreenBgAndWindows();
    SetVBlankCallback(Some(VBlankCB));
    SetMainCallback2(Some(CB2_ResetRtcScreen));
    CreateTask(Some(Task_ResetRtcScreen), 80);
}
pub(crate) unsafe extern "C" fn InitResetRtcScreenBgAndWindows() {
    ClearScheduledBgCopiesToVram();
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 1);
    ScheduleBgCopyTilemapToVram(0);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    ShowBg(0);
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadMessageBoxAndBorderGfx();
}
pub(crate) unsafe extern "C" fn CB2_ResetRtcScreen() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    DoScheduledBgTilemapCopiesToVram();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    ProcessSpriteCopyRequests();
    LoadOam();
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn ShowMessage(str: *mut u8) {
    DrawDialogFrameWithCustomTileAndPalette(WIN_MSG, FALSE, 0x200, 0xF);
    AddTextPrinterParameterized(WIN_MSG, FONT_NORMAL, str, 0, 1, 0, None);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn Task_ShowResetRtcPrompt(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            DrawStdFrameWithCustomTileAndPalette(WIN_TIME, FALSE, 0x214, 0xE);
            AddTextPrinterParameterized(
                WIN_TIME,
                FONT_NORMAL,
                gText_PresentTime.as_ptr().cast_mut(),
                0,
                1,
                TEXT_SKIP_DRAW,
                None,
            );
            PrintTime(
                WIN_TIME,
                0,
                17,
                gLocalTime.days as u16,
                gLocalTime.hours as u8,
                gLocalTime.minutes as u8,
                gLocalTime.seconds as u8,
            );
            AddTextPrinterParameterized(
                WIN_TIME,
                FONT_NORMAL,
                gText_PreviousTime.as_ptr().cast_mut(),
                0,
                33,
                TEXT_SKIP_DRAW,
                None,
            );
            PrintTime(
                WIN_TIME,
                0,
                49,
                (*gSaveBlock2Ptr).lastBerryTreeUpdate.days as u16,
                (*gSaveBlock2Ptr).lastBerryTreeUpdate.hours as u8,
                (*gSaveBlock2Ptr).lastBerryTreeUpdate.minutes as u8,
                (*gSaveBlock2Ptr).lastBerryTreeUpdate.seconds as u8,
            );
            ShowMessage(gText_ResetRTCConfirmCancel.as_ptr().cast_mut());
            CopyWindowToVram(WIN_TIME, COPYWIN_GFX);
            ScheduleBgCopyTilemapToVram(0);
            *data += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if gMain.newKeys as i32 & B_BUTTON != 0 {
                DestroyTask(taskId);
                DoSoftReset();
            } else if gMain.newKeys as i32 & A_BUTTON != 0 {
                PlaySE(SE_SELECT);
                DestroyTask(taskId);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ResetRtcScreen(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == MAINSTATE_FADE_IN {
            fall = true;
            BeginNormalPaletteFade(PALETTES_ALL, 1, 0x10, 0, 65535);
            *data = MAINSTATE_CHECK_SAVE;
            break 'l1;
        }
        if sw1 == MAINSTATE_CHECK_SAVE {
            fall = true;
            if gPaletteFade.active() == 0 {
                if gSaveFileStatus == SAVE_STATUS_EMPTY as u16
                    || gSaveFileStatus == SAVE_STATUS_CORRUPT
                {
                    ShowMessage(gText_NoSaveFileCantSetTime.as_ptr().cast_mut());
                    *data = MAINSTATE_WAIT_EXIT;
                } else {
                    RtcCalcLocalTime();
                    *data.at(1) = CreateTask(Some(Task_ShowResetRtcPrompt), 80) as i16;
                    *data = MAINSTATE_START_SET_TIME;
                }
            }
            break 'l1;
        }
        if sw1 == MAINSTATE_START_SET_TIME {
            fall = true;
            if gTasks[*data.at(1)].isActive != TRUE {
                ClearStdWindowAndFrameToTransparent(WIN_TIME, FALSE);
                ShowMessage(gText_PleaseResetTime.as_ptr().cast_mut());
                gLocalTime = (*gSaveBlock2Ptr).lastBerryTreeUpdate;
                *data.at(1) = CreateTask(Some(Task_ResetRtc_Init), 80) as i16;
                *data = MAINSTATE_WAIT_SET_TIME;
            }
            break 'l1;
        }
        if sw1 == MAINSTATE_WAIT_SET_TIME {
            fall = true;
            if gTasks[*data.at(1)].data[0] != 0 {
                if gTasks[*data.at(1)].data[1] == 0 {
                    DestroyTask(*data.at(1) as u8);
                    *data = MAINSTATE_START_SET_TIME;
                } else {
                    DestroyTask(*data.at(1) as u8);
                    RtcReset();
                    RtcCalcLocalTimeOffset(
                        gLocalTime.days as i32,
                        gLocalTime.hours as i32,
                        gLocalTime.minutes as i32,
                        gLocalTime.seconds as i32,
                    );
                    (*gSaveBlock2Ptr).lastBerryTreeUpdate = gLocalTime;
                    VarSet(VAR_DAYS, gLocalTime.days as u16);
                    DisableResetRTC();
                    ShowMessage(gText_ClockHasBeenReset.as_ptr().cast_mut());
                    *data = MAINSTATE_SAVE;
                }
            }
            break 'l1;
        }
        if sw1 == MAINSTATE_SAVE {
            fall = true;
            if TrySavingData(SAVE_NORMAL) == SAVE_STATUS_OK {
                ShowMessage(gText_SaveCompleted.as_ptr().cast_mut());
                PlaySE(SE_DING_DONG);
            } else {
                ShowMessage(gText_SaveFailed.as_ptr().cast_mut());
                PlaySE(SE_BOO);
            }
            *data = MAINSTATE_WAIT_EXIT;
        }
        if fall || sw1 == MAINSTATE_WAIT_EXIT {
            fall = true;
            if gMain.newKeys as i32 & A_BUTTON != 0 {
                BeginNormalPaletteFade(PALETTES_ALL, 1, 0, 0x10, 65535);
                *data = MAINSTATE_EXIT;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == MAINSTATE_EXIT {
            fall = true;
            if gPaletteFade.active() == 0 {
                DestroyTask(taskId);
                FreeAllWindowBuffers();
                DoSoftReset();
            }
        }
    }
}
