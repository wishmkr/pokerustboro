//! Translated from `src/option_menu.c` by tools/rustport/c2rs.py.
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
pub(crate) static mut sArrowPressed: u8 = 0;

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gTasks: CArray<Task, 0>;
    static gText_BattleSceneOff: CArray<u8, 0>;
    static gText_BattleSceneOn: CArray<u8, 0>;
    static gText_BattleStyleSet: CArray<u8, 0>;
    static gText_BattleStyleShift: CArray<u8, 0>;
    static gText_ButtonTypeLEqualsA: CArray<u8, 0>;
    static gText_ButtonTypeLR: CArray<u8, 0>;
    static gText_ButtonTypeNormal: CArray<u8, 0>;
    static gText_FrameType: CArray<u8, 0>;
    static gText_FrameTypeNumber: CArray<u8, 0>;
    static gText_Option: CArray<u8, 0>;
    static gText_SoundMono: CArray<u8, 0>;
    static gText_SoundStereo: CArray<u8, 0>;
    static gText_TextSpeedFast: CArray<u8, 0>;
    static gText_TextSpeedMid: CArray<u8, 0>;
    static gText_TextSpeedSlow: CArray<u8, 0>;
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
    fn GetWindowFrameTilesPal(a0: u8) -> *mut TilesPal;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
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
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitOptionMenu() {
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
            fall = true;
            SetVBlankCallback(None);
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
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
                    let mut _dest: *mut u32 = OAM as i32 as usize as *mut u32;
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
            fall = true;
            ResetPaletteFade();
            ScanlineEffect_Stop();
            ResetTasks();
            ResetSpriteData();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
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
            fall = true;
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
            fall = true;
            LoadPalette(
                sOptionMenuText_Pal.as_ptr().cast_mut() as *mut c_void,
                16,
                32,
            );
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            PutWindowTilemap(WIN_HEADER);
            DrawHeaderText();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 7 {
            fall = true;
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
            fall = true;
            DrawBgWindowFrames();
            gMain.state += 1;
            break 'l1;
        }
        if sw1 == 10 {
            fall = true;
            {
                let mut taskId: u8 = CreateTask(Some(Task_OptionMenuFadeIn), 0);
                gTasks[taskId].data[0] = 0;
                gTasks[taskId].data[1] = (*gSaveBlock2Ptr).optionsTextSpeed() as i16;
                gTasks[taskId].data[2] = (*gSaveBlock2Ptr).optionsBattleSceneOff() as i16;
                gTasks[taskId].data[3] = (*gSaveBlock2Ptr).optionsBattleStyle() as i16;
                gTasks[taskId].data[4] = (*gSaveBlock2Ptr).optionsSound() as i16;
                gTasks[taskId].data[5] = (*gSaveBlock2Ptr).optionsButtonMode as i16;
                gTasks[taskId].data[6] = (*gSaveBlock2Ptr).optionsWindowFrameType() as i16;
                TextSpeed_DrawChoices(gTasks[taskId].data[1] as u8);
                BattleScene_DrawChoices(gTasks[taskId].data[2] as u8);
                BattleStyle_DrawChoices(gTasks[taskId].data[3] as u8);
                Sound_DrawChoices(gTasks[taskId].data[4] as u8);
                ButtonMode_DrawChoices(gTasks[taskId].data[5] as u8);
                FrameType_DrawChoices(gTasks[taskId].data[6] as u8);
                HighlightOptionMenuItem(gTasks[taskId].data[0] as u8);
                CopyWindowToVram(WIN_OPTIONS, COPYWIN_FULL);
                gMain.state += 1;
                break 'l1;
            }
        }
        if sw1 == 11 {
            fall = true;
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            SetVBlankCallback(Some(VBlankCB));
            SetMainCallback2(Some(MainCB2));
            return;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuFadeIn(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gTasks[taskId].func = Some(Task_OptionMenuProcessInput);
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuProcessInput(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        if gTasks[taskId].data[0] == MENUITEM_CANCEL {
            gTasks[taskId].func = Some(Task_OptionMenuSave);
        }
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        gTasks[taskId].func = Some(Task_OptionMenuSave);
    } else if gMain.newKeys as i32 & DPAD_UP != 0 {
        if gTasks[taskId].data[0] > 0 {
            gTasks[taskId].data[0] -= 1;
        } else {
            gTasks[taskId].data[0] = MENUITEM_CANCEL;
        }
        HighlightOptionMenuItem(gTasks[taskId].data[0] as u8);
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        if gTasks[taskId].data[0] < MENUITEM_CANCEL {
            gTasks[taskId].data[0] += 1;
        } else {
            gTasks[taskId].data[0] = 0;
        }
        HighlightOptionMenuItem(gTasks[taskId].data[0] as u8);
    } else {
        let mut previousOption: u8 = 0;
        match gTasks[taskId].data[0] {
            MENUITEM_TEXTSPEED => {
                previousOption = gTasks[taskId].data[1] as u8;
                gTasks[taskId].data[1] =
                    TextSpeed_ProcessInput(gTasks[taskId].data[1] as u8) as i16;
                if previousOption as i16 != gTasks[taskId].data[1] {
                    TextSpeed_DrawChoices(gTasks[taskId].data[1] as u8);
                }
            }
            MENUITEM_BATTLESCENE => {
                previousOption = gTasks[taskId].data[2] as u8;
                gTasks[taskId].data[2] =
                    BattleScene_ProcessInput(gTasks[taskId].data[2] as u8) as i16;
                if previousOption as i16 != gTasks[taskId].data[2] {
                    BattleScene_DrawChoices(gTasks[taskId].data[2] as u8);
                }
            }
            MENUITEM_BATTLESTYLE => {
                previousOption = gTasks[taskId].data[3] as u8;
                gTasks[taskId].data[3] =
                    BattleStyle_ProcessInput(gTasks[taskId].data[3] as u8) as i16;
                if previousOption as i16 != gTasks[taskId].data[3] {
                    BattleStyle_DrawChoices(gTasks[taskId].data[3] as u8);
                }
            }
            MENUITEM_SOUND => {
                previousOption = gTasks[taskId].data[4] as u8;
                gTasks[taskId].data[4] = Sound_ProcessInput(gTasks[taskId].data[4] as u8) as i16;
                if previousOption as i16 != gTasks[taskId].data[4] {
                    Sound_DrawChoices(gTasks[taskId].data[4] as u8);
                }
            }
            MENUITEM_BUTTONMODE => {
                previousOption = gTasks[taskId].data[5] as u8;
                gTasks[taskId].data[5] =
                    ButtonMode_ProcessInput(gTasks[taskId].data[5] as u8) as i16;
                if previousOption as i16 != gTasks[taskId].data[5] {
                    ButtonMode_DrawChoices(gTasks[taskId].data[5] as u8);
                }
            }
            MENUITEM_FRAMETYPE => {
                previousOption = gTasks[taskId].data[6] as u8;
                gTasks[taskId].data[6] =
                    FrameType_ProcessInput(gTasks[taskId].data[6] as u8) as i16;
                if previousOption as i16 != gTasks[taskId].data[6] {
                    FrameType_DrawChoices(gTasks[taskId].data[6] as u8);
                }
            }
            _ => {
                return;
            }
        }
        if sArrowPressed != 0 {
            sArrowPressed = FALSE;
            CopyWindowToVram(WIN_OPTIONS, COPYWIN_GFX);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_OptionMenuSave(taskId: u8) {
    (*gSaveBlock2Ptr).set_optionsTextSpeed(gTasks[taskId].data[1] as u16);
    (*gSaveBlock2Ptr).set_optionsBattleSceneOff(gTasks[taskId].data[2] as u16);
    (*gSaveBlock2Ptr).set_optionsBattleStyle(gTasks[taskId].data[3] as u16);
    (*gSaveBlock2Ptr).set_optionsSound(gTasks[taskId].data[4] as u16);
    (*gSaveBlock2Ptr).optionsButtonMode = gTasks[taskId].data[5] as u8;
    (*gSaveBlock2Ptr).set_optionsWindowFrameType(gTasks[taskId].data[6] as u16);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
    gTasks[taskId].func = Some(Task_OptionMenuFadeOut);
}
pub(crate) unsafe extern "C" fn Task_OptionMenuFadeOut(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
        FreeAllWindowBuffers();
        SetMainCallback2(gMain.savedCallback);
    }
}
pub(crate) unsafe extern "C" fn HighlightOptionMenuItem(index: u8) {
    SetGpuReg(REG_OFFSET_WIN0H, 4320);
    SetGpuReg(
        REG_OFFSET_WIN0V,
        index as u16 * 16 + 40 << 8 | index as u16 * 16 + 56,
    );
}
pub(crate) unsafe extern "C" fn DrawOptionMenuChoice(mut text: *mut u8, x: u8, y: u8, style: u8) {
    let mut dst: CArray<u8, 16> = zeroed();
    let mut i: u16 = 0;
    i = 0;
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
pub(crate) unsafe extern "C" fn TextSpeed_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if selection <= 1 {
            selection += 1;
        } else {
            selection = 0;
        }
        sArrowPressed = TRUE;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if selection != 0 {
            selection -= 1;
        } else {
            selection = 2;
        }
        sArrowPressed = TRUE;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn TextSpeed_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 3> = zeroed();
    let mut widthSlow: i32 = 0;
    let mut widthMid: i32 = 0;
    let mut widthFast: i32 = 0;
    let mut xMid: i32 = 0;
    styles[0] = 0;
    styles[1] = 0;
    styles[2] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(gText_TextSpeedSlow.as_ptr().cast_mut(), 104, 0, styles[0]);
    widthSlow = GetStringWidth(FONT_NORMAL, gText_TextSpeedSlow.as_ptr().cast_mut(), 0);
    widthMid = GetStringWidth(FONT_NORMAL, gText_TextSpeedMid.as_ptr().cast_mut(), 0);
    widthFast = GetStringWidth(FONT_NORMAL, gText_TextSpeedFast.as_ptr().cast_mut(), 0);
    widthMid -= 94;
    xMid = (widthSlow - widthMid - widthFast) / 2 + 104;
    DrawOptionMenuChoice(
        gText_TextSpeedMid.as_ptr().cast_mut(),
        xMid as u8,
        0,
        styles[1],
    );
    DrawOptionMenuChoice(
        gText_TextSpeedFast.as_ptr().cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            gText_TextSpeedFast.as_ptr().cast_mut(),
            198,
        ) as u8,
        0,
        styles[2],
    );
}
pub(crate) unsafe extern "C" fn BattleScene_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & 48 != 0 {
        selection ^= 1;
        sArrowPressed = TRUE;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn BattleScene_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 2> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(gText_BattleSceneOn.as_ptr().cast_mut(), 104, 16, styles[0]);
    DrawOptionMenuChoice(
        gText_BattleSceneOff.as_ptr().cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            gText_BattleSceneOff.as_ptr().cast_mut(),
            198,
        ) as u8,
        16,
        styles[1],
    );
}
pub(crate) unsafe extern "C" fn BattleStyle_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & 48 != 0 {
        selection ^= 1;
        sArrowPressed = TRUE;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn BattleStyle_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 2> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        gText_BattleStyleShift.as_ptr().cast_mut(),
        104,
        32,
        styles[0],
    );
    DrawOptionMenuChoice(
        gText_BattleStyleSet.as_ptr().cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            gText_BattleStyleSet.as_ptr().cast_mut(),
            198,
        ) as u8,
        32,
        styles[1],
    );
}
pub(crate) unsafe extern "C" fn Sound_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & 48 != 0 {
        selection ^= 1;
        SetPokemonCryStereo(selection as u32);
        sArrowPressed = TRUE;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn Sound_DrawChoices(selection: u8) {
    let mut styles: CArray<u8, 2> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(gText_SoundMono.as_ptr().cast_mut(), 104, 48, styles[0]);
    DrawOptionMenuChoice(
        gText_SoundStereo.as_ptr().cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            gText_SoundStereo.as_ptr().cast_mut(),
            198,
        ) as u8,
        48,
        styles[1],
    );
}
pub(crate) unsafe extern "C" fn FrameType_ProcessInput(mut selection: u8) -> u8 {
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
        sArrowPressed = TRUE;
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
        sArrowPressed = TRUE;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn FrameType_DrawChoices(selection: u8) {
    let mut text: CArray<u8, 16> = zeroed();
    let mut n: u8 = selection + 1;
    let mut i: u16 = 0;
    i = 0;
    while gText_FrameTypeNumber[i] != EOS && i <= 5 {
        text[i] = gText_FrameTypeNumber[i];
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
    DrawOptionMenuChoice(gText_FrameType.as_ptr().cast_mut(), 104, 80, 0);
    DrawOptionMenuChoice(text.as_mut_ptr(), 128, 80, 1);
}
pub(crate) unsafe extern "C" fn ButtonMode_ProcessInput(mut selection: u8) -> u8 {
    if gMain.newKeys as i32 & DPAD_RIGHT != 0 {
        if selection <= 1 {
            selection += 1;
        } else {
            selection = 0;
        }
        sArrowPressed = TRUE;
    }
    if gMain.newKeys as i32 & DPAD_LEFT != 0 {
        if selection != 0 {
            selection -= 1;
        } else {
            selection = 2;
        }
        sArrowPressed = TRUE;
    }
    return selection;
}
pub(crate) unsafe extern "C" fn ButtonMode_DrawChoices(selection: u8) {
    let mut widthNormal: i32 = 0;
    let mut widthLR: i32 = 0;
    let mut widthLA: i32 = 0;
    let mut xLR: i32 = 0;
    let mut styles: CArray<u8, 3> = zeroed();
    styles[0] = 0;
    styles[1] = 0;
    styles[2] = 0;
    styles[selection] = 1;
    DrawOptionMenuChoice(
        gText_ButtonTypeNormal.as_ptr().cast_mut(),
        104,
        64,
        styles[0],
    );
    widthNormal = GetStringWidth(FONT_NORMAL, gText_ButtonTypeNormal.as_ptr().cast_mut(), 0);
    widthLR = GetStringWidth(FONT_NORMAL, gText_ButtonTypeLR.as_ptr().cast_mut(), 0);
    widthLA = GetStringWidth(FONT_NORMAL, gText_ButtonTypeLEqualsA.as_ptr().cast_mut(), 0);
    widthLR -= 94;
    xLR = (widthNormal - widthLR - widthLA) / 2 + 104;
    DrawOptionMenuChoice(
        gText_ButtonTypeLR.as_ptr().cast_mut(),
        xLR as u8,
        64,
        styles[1],
    );
    DrawOptionMenuChoice(
        gText_ButtonTypeLEqualsA.as_ptr().cast_mut(),
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            gText_ButtonTypeLEqualsA.as_ptr().cast_mut(),
            198,
        ) as u8,
        64,
        styles[2],
    );
}
pub(crate) unsafe extern "C" fn DrawHeaderText() {
    FillWindowPixelBuffer(WIN_HEADER, 17);
    AddTextPrinterParameterized(
        WIN_HEADER,
        FONT_NORMAL,
        gText_Option.as_ptr().cast_mut(),
        8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(WIN_HEADER, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn DrawOptionMenuTexts() {
    let mut i: u8 = 0;
    FillWindowPixelBuffer(WIN_OPTIONS, 17);
    i = 0;
    while i < MENUITEM_COUNT {
        AddTextPrinterParameterized(
            WIN_OPTIONS,
            FONT_NORMAL,
            sOptionMenuItemsNames[i],
            8,
            i * 16 + 1,
            TEXT_SKIP_DRAW,
            None,
        );
        i += 1;
    }
    CopyWindowToVram(WIN_OPTIONS, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn DrawBgWindowFrames() {
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
