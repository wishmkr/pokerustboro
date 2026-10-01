//! Translated from `src/option_menu.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::LoadBgTiles;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::GetStringRightAlignXOffset;
use crate::load_save::gSaveBlock2Ptr;
use crate::m4a::SetPokemonCryStereo;
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_get, task_set, task_set_func};
use crate::text::DeactivateAllTextPrinters;
use crate::text::GetStringWidth;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    CopyWindowToVram, FillWindowPixelBuffer, FreeAllWindowBuffers, PutWindowTilemap,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
// The C's names for task and sprite data slots.
const tMenuSelection: usize = 0;
const tTextSpeed: usize = 1;
const tBattleSceneOff: usize = 2;
const tBattleStyle: usize = 3;
const tSound: usize = 4;
const tButtonMode: usize = 5;
const tWindowFrameType: usize = 6;
// Data tables (translate with cdata.py): sOptionMenuText_Pal sEqualSignGfx sOptionMenuItemsNames sOptionMenuWinTemplates sOptionMenuBgTemplates sOptionMenuBg_Pal

const MENUITEM_BATTLESCENE: i16 = 1;
const MENUITEM_BATTLESTYLE: i16 = 2;
const MENUITEM_BUTTONMODE: i16 = 4;
const MENUITEM_CANCEL: i16 = 6;
const MENUITEM_COUNT: u8 = 7;
const MENUITEM_FRAMETYPE: i16 = 5;
const MENUITEM_SOUND: i16 = 3;
const MENUITEM_TEXTSPEED: i16 = 0;
const TILE_BOT_CORNER_L: u16 = 424;
const TILE_BOT_CORNER_R: u16 = 426;
const TILE_BOT_EDGE: u16 = 425;
const TILE_LEFT_EDGE: u16 = 421;
const TILE_RIGHT_EDGE: u16 = 423;
const TILE_TOP_CORNER_L: u16 = 418;
const TILE_TOP_CORNER_R: u16 = 420;
const TILE_TOP_EDGE: u16 = 419;
const WIN_HEADER: u8 = 0;
const WIN_OPTIONS: u8 = 1;

static sOptionMenuBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::option_menu::sOptionMenuBgTemplates).cast());
static sOptionMenuBg_Pal: Table<CArray<u16, 1>> =
    Table((&raw const crate::data::option_menu::sOptionMenuBg_Pal).cast());
static sOptionMenuItemsNames: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::option_menu::sOptionMenuItemsNames).cast());
static sOptionMenuText_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::option_menu::sOptionMenuText_Pal).cast());
static sOptionMenuWinTemplates: Table<CArray<WindowTemplate, 3>> =
    Table((&raw const crate::data::option_menu::sOptionMenuWinTemplates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sArrowPressed: crate::global::Global<u8> = crate::global::Global::new(0);

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
/// `GetWindowFrameTilesPal` with this module's view of its types.
#[inline]
unsafe fn GetWindowFrameTilesPal(a0: u8) -> *mut TilesPal {
    unsafe { crate::text_window::GetWindowFrameTilesPal(a0) as *mut TilesPal }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn MainCB2() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub unsafe fn CB2_InitOptionMenu() {
    'l1: {
        let sw1: u8 = gMain.state;
        let matched = sw1 == 0
            || sw1 == 1
            || sw1 == 2
            || sw1 == 3
            || sw1 == 4
            || sw1 == 5
            || sw1 == 6
            || sw1 == 7
            || sw1 == 8
            || sw1 == 9
            || sw1 == 10
            || sw1 == 11;
        let mut fall = false;
        if sw1 == 0 || !matched {
            SetVBlankCallback(None);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 1 {
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
                    let mut _dest: *mut u32 = OAM as i32 as usize as *mut u32;
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
                    let mut _dest: *mut u16 = PLTT as i32 as usize as *mut u16;
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
            SetGpuReg(0x0, 0);
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sOptionMenuBgTemplates.as_ptr().cast_mut(), 2);
            ChangeBgX(0, 0, BG_COORD_SET);
            ChangeBgY(0, 0, BG_COORD_SET);
            ChangeBgX(1, 0, BG_COORD_SET);
            ChangeBgY(1, 0, BG_COORD_SET);
            ChangeBgX(2, 0, BG_COORD_SET);
            ChangeBgY(2, 0, BG_COORD_SET);
            ChangeBgX(3, 0, BG_COORD_SET);
            ChangeBgY(3, 0, BG_COORD_SET);
            InitWindows(sOptionMenuWinTemplates.as_ptr().cast_mut());
            DeactivateAllTextPrinters();
            SetGpuReg(REG_OFFSET_WIN0H, 0);
            SetGpuReg(REG_OFFSET_WIN0V, 0);
            SetGpuReg(REG_OFFSET_WININ, WININ_WIN0_BG0);
            SetGpuReg(REG_OFFSET_WINOUT, 35);
            SetGpuReg(REG_OFFSET_BLDCNT, 193);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetGpuReg(REG_OFFSET_BLDY, 4);
            SetGpuReg(REG_OFFSET_DISPCNT, 12352);
            ShowBg(0);
            ShowBg(1);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 2 {
            ResetPaletteFade();
            ScanlineEffect_Stop();
            ResetTasks();
            ResetSpriteData();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 3 {
            LoadBgTiles(
                1,
                (*GetWindowFrameTilesPal((*gSaveBlock2Ptr).optionsWindowFrameType() as u8)).tiles
                    as *mut c_void,
                0x120,
                0x1A2,
            );
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 4 {
            LoadPalette(sOptionMenuBg_Pal.as_ptr().cast_mut() as *mut c_void, 0, 2);
            LoadPalette(
                (*GetWindowFrameTilesPal((*gSaveBlock2Ptr).optionsWindowFrameType() as u8)).pal
                    as *mut c_void,
                112,
                32,
            );
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 5 {
            LoadPalette(
                sOptionMenuText_Pal.as_ptr().cast_mut() as *mut c_void,
                16,
                32,
            );
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 6 {
            PutWindowTilemap(WIN_HEADER);
            DrawHeaderText();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 7 {
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 8 {
            fall = true;
            PutWindowTilemap(WIN_OPTIONS);
            DrawOptionMenuTexts();
            gMain.state += 1;
        }
        if fall || sw1 == 9 {
            DrawBgWindowFrames();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 10 {
            {
                let taskId: u8 = CreateTask(Some(Task_OptionMenuFadeIn), 0);
                task_set(taskId, tMenuSelection, 0);
                task_set(
                    taskId,
                    tTextSpeed,
                    (*gSaveBlock2Ptr).optionsTextSpeed() as i16,
                );
                task_set(
                    taskId,
                    tBattleSceneOff,
                    (*gSaveBlock2Ptr).optionsBattleSceneOff() as i16,
                );
                task_set(
                    taskId,
                    tBattleStyle,
                    (*gSaveBlock2Ptr).optionsBattleStyle() as i16,
                );
                task_set(taskId, tSound, (*gSaveBlock2Ptr).optionsSound() as i16);
                task_set(
                    taskId,
                    tButtonMode,
                    (*gSaveBlock2Ptr).optionsButtonMode as i16,
                );
                task_set(
                    taskId,
                    tWindowFrameType,
                    (*gSaveBlock2Ptr).optionsWindowFrameType() as i16,
                );
                TextSpeed_DrawChoices(task_get(taskId, tTextSpeed) as u8);
                BattleScene_DrawChoices(task_get(taskId, tBattleSceneOff) as u8);
                BattleStyle_DrawChoices(task_get(taskId, tBattleStyle) as u8);
                Sound_DrawChoices(task_get(taskId, tSound) as u8);
                ButtonMode_DrawChoices(task_get(taskId, tButtonMode) as u8);
                FrameType_DrawChoices(task_get(taskId, tWindowFrameType) as u8);
                HighlightOptionMenuItem(task_get(taskId, tMenuSelection) as u8);
                CopyWindowToVram(WIN_OPTIONS, COPYWIN_FULL);
                gMain.state += 1;
                break 'l1;
            }
        }
        if sw1 == 11 {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetVBlankCallback(Some(VBlankCB));
            SetMainCallback2(Some(MainCB2));
        }
    }
}
pub(crate) unsafe fn Task_OptionMenuFadeIn(taskId: u8) {
    if gPaletteFade.active() == 0 {
        task_set_func(taskId, Some(Task_OptionMenuProcessInput));
    }
}
pub(crate) unsafe fn Task_OptionMenuProcessInput(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if task_get(taskId, tMenuSelection) == MENUITEM_CANCEL {
            task_set_func(taskId, Some(Task_OptionMenuSave));
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        task_set_func(taskId, Some(Task_OptionMenuSave));
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if task_get(taskId, tMenuSelection) > 0 {
            task_set(taskId, tMenuSelection, task_get(taskId, tMenuSelection) - 1);
        } else {
            task_set(taskId, tMenuSelection, MENUITEM_CANCEL);
        }
        HighlightOptionMenuItem(task_get(taskId, tMenuSelection) as u8);
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if task_get(taskId, tMenuSelection) < MENUITEM_CANCEL {
            task_set(taskId, tMenuSelection, task_get(taskId, tMenuSelection) + 1);
        } else {
            task_set(taskId, tMenuSelection, 0);
        }
        HighlightOptionMenuItem(task_get(taskId, tMenuSelection) as u8);
    } else {
        let mut previousOption: u8 = 0;
        match task_get(taskId, tMenuSelection) {
            MENUITEM_TEXTSPEED => {
                previousOption = task_get(taskId, tTextSpeed) as u8;
                task_set(
                    taskId,
                    tTextSpeed,
                    TextSpeed_ProcessInput(task_get(taskId, tTextSpeed) as u8) as i16,
                );
                if previousOption as i16 != task_get(taskId, tTextSpeed) {
                    TextSpeed_DrawChoices(task_get(taskId, tTextSpeed) as u8);
                }
            }
            MENUITEM_BATTLESCENE => {
                previousOption = task_get(taskId, tBattleSceneOff) as u8;
                task_set(
                    taskId,
                    tBattleSceneOff,
                    BattleScene_ProcessInput(task_get(taskId, tBattleSceneOff) as u8) as i16,
                );
                if previousOption as i16 != task_get(taskId, tBattleSceneOff) {
                    BattleScene_DrawChoices(task_get(taskId, tBattleSceneOff) as u8);
                }
            }
            MENUITEM_BATTLESTYLE => {
                previousOption = task_get(taskId, tBattleStyle) as u8;
                task_set(
                    taskId,
                    tBattleStyle,
                    BattleStyle_ProcessInput(task_get(taskId, tBattleStyle) as u8) as i16,
                );
                if previousOption as i16 != task_get(taskId, tBattleStyle) {
                    BattleStyle_DrawChoices(task_get(taskId, tBattleStyle) as u8);
                }
            }
            MENUITEM_SOUND => {
                previousOption = task_get(taskId, tSound) as u8;
                task_set(
                    taskId,
                    tSound,
                    Sound_ProcessInput(task_get(taskId, tSound) as u8) as i16,
                );
                if previousOption as i16 != task_get(taskId, tSound) {
                    Sound_DrawChoices(task_get(taskId, tSound) as u8);
                }
            }
            MENUITEM_BUTTONMODE => {
                previousOption = task_get(taskId, tButtonMode) as u8;
                task_set(
                    taskId,
                    tButtonMode,
                    ButtonMode_ProcessInput(task_get(taskId, tButtonMode) as u8) as i16,
                );
                if previousOption as i16 != task_get(taskId, tButtonMode) {
                    ButtonMode_DrawChoices(task_get(taskId, tButtonMode) as u8);
                }
            }
            MENUITEM_FRAMETYPE => {
                previousOption = task_get(taskId, tWindowFrameType) as u8;
                task_set(
                    taskId,
                    tWindowFrameType,
                    FrameType_ProcessInput(task_get(taskId, tWindowFrameType) as u8) as i16,
                );
                if previousOption as i16 != task_get(taskId, tWindowFrameType) {
                    FrameType_DrawChoices(task_get(taskId, tWindowFrameType) as u8);
                }
            }
            _ => {
                return;
            }
        }
        if sArrowPressed.get() != 0 {
            sArrowPressed.set(FALSE);
            CopyWindowToVram(WIN_OPTIONS, COPYWIN_GFX);
        }
    }
}
pub(crate) unsafe fn Task_OptionMenuSave(taskId: u8) {
    (*gSaveBlock2Ptr).set_optionsTextSpeed(task_get(taskId, tTextSpeed) as u16);
    (*gSaveBlock2Ptr).set_optionsBattleSceneOff(task_get(taskId, tBattleSceneOff) as u16);
    (*gSaveBlock2Ptr).set_optionsBattleStyle(task_get(taskId, tBattleStyle) as u16);
    (*gSaveBlock2Ptr).set_optionsSound(task_get(taskId, tSound) as u16);
    (*gSaveBlock2Ptr).optionsButtonMode = task_get(taskId, tButtonMode) as u8;
    (*gSaveBlock2Ptr).set_optionsWindowFrameType(task_get(taskId, tWindowFrameType) as u16);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    task_set_func(taskId, Some(Task_OptionMenuFadeOut));
}
pub(crate) unsafe fn Task_OptionMenuFadeOut(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        FreeAllWindowBuffers();
        SetMainCallback2(gMain.savedCallback);
    }
}
unsafe fn HighlightOptionMenuItem(index: u8) {
    SetGpuReg(REG_OFFSET_WIN0H, 4320);
    SetGpuReg(
        REG_OFFSET_WIN0V,
        ((index as u16 * 16 + 40) << 8) | (index as u16 * 16 + 56),
    );
}
unsafe fn DrawOptionMenuChoice(mut text: *mut u8, x: u8, y: u8, style: u8) {
    let mut dst: CArray<u8, 16> = zeroed();
    let mut i: u16 = 0;
    while *text != EOS && i < 15 {
        dst[i] = *({
            let t2 = text;
            text = text.at(1);
            t2
        });
        i += 1;
    }
    if style != 0 {
        dst[2] = TEXT_COLOR_RED;
        dst[5] = 0x5;
    }
    dst[i] = EOS;
    AddTextPrinterParameterized(
        WIN_OPTIONS,
        FONT_NORMAL,
        dst.as_mut_ptr(),
        x,
        y + 1,
        TEXT_SKIP_DRAW,
        None,
    );
}
unsafe fn TextSpeed_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if selection <= 1 {
            selection += 1;
        } else {
            selection = 0;
        }
        sArrowPressed.set(TRUE);
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if selection != 0 {
            selection -= 1;
        } else {
            selection = 2;
        }
        sArrowPressed.set(TRUE);
    }
    selection
}
unsafe fn TextSpeed_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 3> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[2] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_TextSpeedSlow).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        104,
        0,
        styles[0],
    );
    let widthSlow: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_TextSpeedSlow).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    let mut widthMid: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_TextSpeedMid).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    let widthFast: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_TextSpeedFast).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    widthMid -= 94;
    let xMid: i32 = (widthSlow - widthMid - widthFast) / 2 + 104;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_TextSpeedMid).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        xMid as u8,
        0,
        styles[1],
    );
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_TextSpeedFast).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_TextSpeedFast).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            198,
        ) as u8,
        0,
        styles[2],
    );
}
unsafe fn BattleScene_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & 48 != 0 {
        selection ^= 1;
        sArrowPressed.set(TRUE);
    }
    selection
}
unsafe fn BattleScene_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 2> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_BattleSceneOn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        104,
        16,
        styles[0],
    );
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_BattleSceneOff).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_BattleSceneOff).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            198,
        ) as u8,
        16,
        styles[1],
    );
}
unsafe fn BattleStyle_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & 48 != 0 {
        selection ^= 1;
        sArrowPressed.set(TRUE);
    }
    selection
}
unsafe fn BattleStyle_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 2> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_BattleStyleShift).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        104,
        32,
        styles[0],
    );
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_BattleStyleSet).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_BattleStyleSet).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            198,
        ) as u8,
        32,
        styles[1],
    );
}
unsafe fn Sound_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & 48 != 0 {
        selection ^= 1;
        SetPokemonCryStereo(selection as u32);
        sArrowPressed.set(TRUE);
    }
    selection
}
unsafe fn Sound_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 2> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_SoundMono).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        104,
        48,
        styles[0],
    );
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_SoundStereo).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_SoundStereo).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            198,
        ) as u8,
        48,
        styles[1],
    );
}
unsafe fn FrameType_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if selection < 19 {
            selection += 1;
        } else {
            selection = 0;
        }
        LoadBgTiles(
            1,
            (*GetWindowFrameTilesPal(selection)).tiles as *mut c_void,
            0x120,
            0x1A2,
        );
        LoadPalette(
            (*GetWindowFrameTilesPal(selection)).pal as *mut c_void,
            112,
            32,
        );
        sArrowPressed.set(TRUE);
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if selection != 0 {
            selection -= 1;
        } else {
            selection = 19;
        }
        LoadBgTiles(
            1,
            (*GetWindowFrameTilesPal(selection)).tiles as *mut c_void,
            0x120,
            0x1A2,
        );
        LoadPalette(
            (*GetWindowFrameTilesPal(selection)).pal as *mut c_void,
            112,
            32,
        );
        sArrowPressed.set(TRUE);
    }
    selection
}
unsafe fn FrameType_DrawChoices(selection: u8) {
    let mut text: CArray<u8, 16> = zeroed();
    let n: u8 = selection + 1;
    let mut i: u16 = 0;
    while (*(&raw const crate::data::strings::gText_FrameTypeNumber).cast::<CArray<u8, 0>>())[i]
        != EOS
        && i <= 5
    {
        text[i] =
            (*(&raw const crate::data::strings::gText_FrameTypeNumber).cast::<CArray<u8, 0>>())[i];
        i += 1;
    }
    if n as i32 / 10 != 0 {
        text[i] = (n as i32 / 10) as u8 + CHAR_0;
        i += 1;
        text[i] = (n as i32 % 10) as u8 + CHAR_0;
        i += 1;
    } else {
        text[i] = (n as i32 % 10) as u8 + CHAR_0;
        i += 1;
        text[i] = CHAR_SPACER;
        i += 1;
    }
    text[i] = EOS;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_FrameType).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        104,
        80,
        0,
    );
    DrawOptionMenuChoice(text.as_mut_ptr(), 128, 80, 1);
}
unsafe fn ButtonMode_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if selection <= 1 {
            selection += 1;
        } else {
            selection = 0;
        }
        sArrowPressed.set(TRUE);
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if selection != 0 {
            selection -= 1;
        } else {
            selection = 2;
        }
        sArrowPressed.set(TRUE);
    }
    selection
}
unsafe fn ButtonMode_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 3> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[2] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_ButtonTypeNormal).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        104,
        64,
        styles[0],
    );
    let widthNormal: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_ButtonTypeNormal).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    let mut widthLR: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_ButtonTypeLR).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    let widthLA: i32 = GetStringWidth(
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_ButtonTypeLEqualsA).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
    );
    widthLR -= 94;
    let xLR: i32 = (widthNormal - widthLR - widthLA) / 2 + 104;
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_ButtonTypeLR).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        xLR as u8,
        64,
        styles[1],
    );
    DrawOptionMenuChoice(
        (*(&raw const crate::data::strings::gText_ButtonTypeLEqualsA).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            (*(&raw const crate::data::strings::gText_ButtonTypeLEqualsA).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            198,
        ) as u8,
        64,
        styles[2],
    );
}
unsafe fn DrawHeaderText() {
    FillWindowPixelBuffer(WIN_HEADER, 17);
    AddTextPrinterParameterized(
        WIN_HEADER,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_Option).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(WIN_HEADER, COPYWIN_FULL);
}
unsafe fn DrawOptionMenuTexts() {
    FillWindowPixelBuffer(WIN_OPTIONS, 17);
    for i in 0..MENUITEM_COUNT {
        AddTextPrinterParameterized(
            WIN_OPTIONS,
            FONT_NORMAL,
            sOptionMenuItemsNames[i],
            8,
            i * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    CopyWindowToVram(WIN_OPTIONS, COPYWIN_FULL);
}
unsafe fn DrawBgWindowFrames() {
    FillBgTilemapBufferRect(1, TILE_TOP_CORNER_L, 1, 0, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_TOP_EDGE, 2, 0, 27, 1, 7);
    FillBgTilemapBufferRect(1, TILE_TOP_CORNER_R, 28, 0, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_LEFT_EDGE, 1, 1, 1, 2, 7);
    FillBgTilemapBufferRect(1, TILE_RIGHT_EDGE, 28, 1, 1, 2, 7);
    FillBgTilemapBufferRect(1, TILE_BOT_CORNER_L, 1, 3, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_BOT_EDGE, 2, 3, 27, 1, 7);
    FillBgTilemapBufferRect(1, TILE_BOT_CORNER_R, 28, 3, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_TOP_CORNER_L, 1, 4, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_TOP_EDGE, 2, 4, 26, 1, 7);
    FillBgTilemapBufferRect(1, TILE_TOP_CORNER_R, 28, 4, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_LEFT_EDGE, 1, 5, 1, 18, 7);
    FillBgTilemapBufferRect(1, TILE_RIGHT_EDGE, 28, 5, 1, 18, 7);
    FillBgTilemapBufferRect(1, TILE_BOT_CORNER_L, 1, 19, 1, 1, 7);
    FillBgTilemapBufferRect(1, TILE_BOT_EDGE, 2, 19, 26, 1, 7);
    FillBgTilemapBufferRect(1, TILE_BOT_CORNER_R, 28, 19, 1, 1, 7);
    CopyBgTilemapBufferToVram(1);
}
