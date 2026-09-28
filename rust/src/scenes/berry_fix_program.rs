//! Translated from `src/berry_fix_program.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sText_BerryProgramUpdate sText_RubySapphire sText_Emerald sText_BerryProgramWillBeUpdatedPressA sText_EnsureGBAConnectionMatches sText_TurnOffPowerHoldingStartSelect sText_TransmittingPleaseWait sText_PleaseFollowInstructionsOnScreen sText_TransmissionFailureTryAgain sBerryFixBgTemplates sBerryFixWindowTemplates sText_Pal sBerryProgramTextColors sGameTitleTextColors sBerryProgramTexts sBerryFixGraphics

/// `__typeof__(*((__typeof__(sBerryFix))0))`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct typeof___sBerryFix_0_t {
    pub state: u8,
    pub curScene: u8,
    pub timer: u16,
    pub mb: MultiBootParam,
}

unsafe impl Sync for typeof___sBerryFix_0_t {}

/// `__typeof__(sBerryFixGraphics[0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sBerryFixGraphics_0_t {
    pub gfx: *mut u32,
    pub tilemap: *mut u32,
    pub palette: *mut u16,
}

unsafe impl Sync for sBerryFixGraphics_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<typeof___sBerryFix_0_t>() == 80);
    assert!(offset_of!(typeof___sBerryFix_0_t, state) == 0);
    assert!(offset_of!(typeof___sBerryFix_0_t, curScene) == 1);
    assert!(offset_of!(typeof___sBerryFix_0_t, timer) == 2);
    assert!(offset_of!(typeof___sBerryFix_0_t, mb) == 4);
    assert!(size_of::<sBerryFixGraphics_0_t>() == 12);
    assert!(offset_of!(sBerryFixGraphics_0_t, gfx) == 0);
    assert!(offset_of!(sBerryFixGraphics_0_t, tilemap) == 4);
    assert!(offset_of!(sBerryFixGraphics_0_t, palette) == 8);
};

const MAINSTATE_BEGIN: u8 = 1;
const MAINSTATE_CONNECT: u8 = 2;
const MAINSTATE_EXIT: u8 = 6;
const MAINSTATE_FAILED: u8 = 7;
const MAINSTATE_INIT: u8 = 0;
const MAINSTATE_INIT_MULTIBOOT: u8 = 3;
const MAINSTATE_MULTIBOOT: u8 = 4;
const MAINSTATE_TRANSMIT: u8 = 5;
const SCENE_BEGIN: i32 = 5;
const SCENE_ENSURE_CONNECT: i32 = 0;
const SCENE_FOLLOW_INSTRUCT: i32 = 3;
const SCENE_NONE: u8 = 6;
const SCENE_TRANSMITTING: i32 = 2;
const SCENE_TRANSMIT_FAILED: i32 = 4;
const SCENE_TURN_OFF_POWER: i32 = 1;
const WIN_GAME_NAMES: u8 = 2;
const WIN_MSG_BODY: u8 = 1;
const WIN_TITLE: u8 = 0;
const WIN_TURN_OFF_TITLE: u8 = 3;

static sBerryFixBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::berry_fix_program::sBerryFixBgTemplates).cast());
static sBerryFixGraphics: Table<CArray<sBerryFixGraphics_0_t, 6>> =
    Table((&raw const crate::data::berry_fix_program::sBerryFixGraphics).cast());
static sBerryFixWindowTemplates: Table<CArray<WindowTemplate, 5>> =
    Table((&raw const crate::data::berry_fix_program::sBerryFixWindowTemplates).cast());
static sBerryProgramTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::berry_fix_program::sBerryProgramTextColors).cast());
static sBerryProgramTexts: Table<CArray<*mut u8, 6>> =
    Table((&raw const crate::data::berry_fix_program::sBerryProgramTexts).cast());
static sGameTitleTextColors: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::berry_fix_program::sGameTitleTextColors).cast());
static sText_BerryProgramUpdate: Table<CArray<u8, 21>> =
    Table((&raw const crate::data::berry_fix_program::sText_BerryProgramUpdate).cast());
static sText_Emerald: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::berry_fix_program::sText_Emerald).cast());
static sText_Pal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::berry_fix_program::sText_Pal).cast());
static sText_RubySapphire: Table<CArray<u8, 14>> =
    Table((&raw const crate::data::berry_fix_program::sText_RubySapphire).cast());

pub(crate) static mut sBerryFix: *mut typeof___sBerryFix_0_t = null_mut();

unsafe extern "C" {
    static mut gMain: Main;
    static gMultiBootProgram_BerryGlitchFix_End: CArray<u8, 0>;
    static gMultiBootProgram_BerryGlitchFix_Start: CArray<u8, 15348>;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn DeactivateAllTextPrinters();
    fn DisableInterrupts(a0: u16);
    fn DoSoftReset();
    fn EnableInterrupts(a0: u16);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn HideBg(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn MultiBootCheckComplete(a0: *mut MultiBootParam) -> i32;
    fn MultiBootInit(a0: *mut MultiBootParam);
    fn MultiBootMain(a0: *mut MultiBootParam) -> i32;
    fn MultiBootStartMaster(a0: *mut MultiBootParam, a1: *mut u8, a2: i32, a3: u8, a4: i8);
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetSpriteData();
    fn ResetTasks();
    fn ScanlineEffect_Stop();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn m4aSoundVSyncOff();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitBerryFixProgram() {
    DisableInterrupts(0xFFFF);
    EnableInterrupts(INTR_FLAG_VBLANK);
    m4aSoundVSyncOff();
    SetVBlankCallback(None);
    ResetSpriteData();
    ResetTasks();
    ScanlineEffect_Stop();
    SetGpuReg(0x0, 0);
    sBerryFix = AllocZeroed(80) as *mut typeof___sBerryFix_0_t;
    (*sBerryFix).state = MAINSTATE_INIT;
    (*sBerryFix).curScene = SCENE_NONE;
    SetMainCallback2(Some(BerryFix_Main));
}
pub(crate) unsafe extern "C" fn BerryFix_Main() {
    match (*sBerryFix).state {
        MAINSTATE_INIT => {
            BerryFix_GpuSet();
            (*sBerryFix).state = MAINSTATE_BEGIN;
        }
        MAINSTATE_BEGIN => {
            if BerryFix_TrySetScene(SCENE_BEGIN) == SCENE_BEGIN
                && gMain.newKeys as i32 & A_BUTTON != 0
            {
                (*sBerryFix).state = MAINSTATE_CONNECT;
            }
        }
        MAINSTATE_CONNECT => {
            if BerryFix_TrySetScene(SCENE_ENSURE_CONNECT) == SCENE_ENSURE_CONNECT
                && gMain.newKeys as i32 & A_BUTTON != 0
            {
                (*sBerryFix).state = MAINSTATE_INIT_MULTIBOOT;
            }
        }
        MAINSTATE_INIT_MULTIBOOT => {
            if BerryFix_TrySetScene(SCENE_TURN_OFF_POWER) == SCENE_TURN_OFF_POWER {
                (*sBerryFix).mb.masterp =
                    gMultiBootProgram_BerryGlitchFix_Start.as_ptr().cast_mut();
                (*sBerryFix).mb.server_type = 0;
                MultiBootInit(&raw mut (*sBerryFix).mb);
                (*sBerryFix).timer = 0;
                (*sBerryFix).state = MAINSTATE_MULTIBOOT;
            }
        }
        MAINSTATE_MULTIBOOT => {
            MultiBootMain(&raw mut (*sBerryFix).mb);
            if (*sBerryFix).mb.probe_count != 0
                || ((*sBerryFix).mb.response_bit as i32 & 2 == 0
                    || (*sBerryFix).mb.client_bit as i32 & 2 == 0)
            {
                (*sBerryFix).timer = 0;
            } else if ({
                (*sBerryFix).timer += 1;
                (*sBerryFix).timer
            }) > 180
            {
                MultiBootStartMaster(
                    &raw mut (*sBerryFix).mb,
                    gMultiBootProgram_BerryGlitchFix_Start
                        .as_ptr()
                        .cast_mut()
                        .at(192),
                    (gMultiBootProgram_BerryGlitchFix_End.as_ptr().cast_mut() as usize)
                        .wrapping_sub(
                            gMultiBootProgram_BerryGlitchFix_Start
                                .as_ptr()
                                .cast_mut()
                                .at(192) as usize,
                        ) as i32 as u32 as i32,
                    4,
                    1,
                );
                (*sBerryFix).state = MAINSTATE_TRANSMIT;
            }
        }
        MAINSTATE_TRANSMIT => {
            if BerryFix_TrySetScene(SCENE_TRANSMITTING) == SCENE_TRANSMITTING {
                MultiBootMain(&raw mut (*sBerryFix).mb);
                if MultiBootCheckComplete(&raw mut (*sBerryFix).mb) != 0 {
                    (*sBerryFix).state = MAINSTATE_EXIT;
                } else if (*sBerryFix).mb.client_bit as i32 & 2 == 0 {
                    (*sBerryFix).state = MAINSTATE_FAILED;
                }
            }
        }
        MAINSTATE_EXIT => {
            if BerryFix_TrySetScene(SCENE_FOLLOW_INSTRUCT) == SCENE_FOLLOW_INSTRUCT
                && gMain.newKeys as i32 & A_BUTTON != 0
            {
                DoSoftReset();
            }
        }
        MAINSTATE_FAILED => {
            if BerryFix_TrySetScene(SCENE_TRANSMIT_FAILED) == SCENE_TRANSMIT_FAILED
                && gMain.newKeys as i32 & A_BUTTON != 0
            {
                (*sBerryFix).state = MAINSTATE_BEGIN;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn BerryFix_GpuSet() {
    let mut width: i32 = 0;
    let mut left: i32 = 0;
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), VRAM as u32);
                    volatile_write(dmaRegs.at(2), 0x85006000);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), OAM);
                    volatile_write(dmaRegs.at(2), 0x85000100);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u32 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), PLTT);
                    volatile_write(dmaRegs.at(2), 0x85000100);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBerryFixBgTemplates.as_ptr().cast_mut(), 2);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    InitWindows(sBerryFixWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    {
        {
            {
                let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                volatile_write(dmaRegs, sText_Pal.as_ptr().cast_mut() as usize as u32);
                volatile_write(dmaRegs.at(1), 0x50001e0);
                volatile_write(dmaRegs.at(2), 0x84000008);
                let _ = (dmaRegs.at(2)).read_volatile();
            }
        }
    }
    SetGpuReg(REG_OFFSET_DISPCNT, DISPCNT_OBJ_1D_MAP);
    FillWindowPixelBuffer(WIN_GAME_NAMES, 0);
    FillWindowPixelBuffer(WIN_TURN_OFF_TITLE, 0);
    FillWindowPixelBuffer(WIN_TITLE, 170);
    width = GetStringWidth(FONT_SMALL, sText_Emerald.as_ptr().cast_mut(), 0);
    left = (120 - width) / 2;
    AddTextPrinterParameterized3(
        WIN_GAME_NAMES,
        FONT_SMALL,
        left as u8,
        3,
        sGameTitleTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        sText_Emerald.as_ptr().cast_mut(),
    );
    width = GetStringWidth(FONT_SMALL, sText_RubySapphire.as_ptr().cast_mut(), 0);
    left = (120 - width) / 2 + 120;
    AddTextPrinterParameterized3(
        WIN_GAME_NAMES,
        FONT_SMALL,
        left as u8,
        3,
        sGameTitleTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        sText_RubySapphire.as_ptr().cast_mut(),
    );
    width = GetStringWidth(FONT_SMALL, sText_RubySapphire.as_ptr().cast_mut(), 0);
    left = (112 - width) / 2;
    AddTextPrinterParameterized3(
        WIN_TURN_OFF_TITLE,
        FONT_SMALL,
        left as u8,
        0,
        sGameTitleTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        sText_RubySapphire.as_ptr().cast_mut(),
    );
    width = GetStringWidth(FONT_NORMAL, sText_BerryProgramUpdate.as_ptr().cast_mut(), 0);
    left = (208 - width) / 2;
    AddTextPrinterParameterized3(
        WIN_TITLE,
        FONT_NORMAL,
        left as u8,
        2,
        sBerryProgramTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        sText_BerryProgramUpdate.as_ptr().cast_mut(),
    );
    CopyWindowToVram(WIN_GAME_NAMES, COPYWIN_GFX);
    CopyWindowToVram(WIN_TURN_OFF_TITLE, COPYWIN_GFX);
    CopyWindowToVram(WIN_TITLE, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn BerryFix_TrySetScene(scene: i32) -> i32 {
    if (*sBerryFix).curScene as i32 == scene {
        return scene;
    }
    if (*sBerryFix).curScene == SCENE_NONE {
        BerryFix_SetScene(scene);
        (*sBerryFix).curScene = scene as u8;
    } else {
        BerryFix_HideScene();
        (*sBerryFix).curScene = SCENE_NONE;
    }
    return (*sBerryFix).curScene as i32;
}
pub(crate) unsafe extern "C" fn BerryFix_SetScene(scene: i32) {
    FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, 32, 32);
    FillWindowPixelBuffer(WIN_MSG_BODY, 170);
    AddTextPrinterParameterized3(
        WIN_MSG_BODY,
        FONT_NORMAL,
        0,
        0,
        sBerryProgramTextColors.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        sBerryProgramTexts[scene],
    );
    PutWindowTilemap(WIN_MSG_BODY);
    CopyWindowToVram(WIN_MSG_BODY, COPYWIN_GFX);
    match scene {
        SCENE_ENSURE_CONNECT
        | SCENE_TRANSMITTING
        | SCENE_FOLLOW_INSTRUCT
        | SCENE_TRANSMIT_FAILED => {
            PutWindowTilemap(WIN_GAME_NAMES);
        }
        SCENE_TURN_OFF_POWER => {
            PutWindowTilemap(WIN_TURN_OFF_TITLE);
        }
        SCENE_BEGIN => {
            PutWindowTilemap(WIN_TITLE);
        }
        _ => {}
    }
    CopyBgTilemapBufferToVram(0);
    LZ77UnCompVram(
        sBerryFixGraphics[scene].gfx,
        0x6004000 as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sBerryFixGraphics[scene].tilemap,
        0x600f800 as usize as *mut c_void,
    );
    CpuSet(
        sBerryFixGraphics[scene].palette as *mut c_void,
        BG_PLTT as i32 as usize as *mut c_void,
        0x4000040,
    );
    ShowBg(0);
    ShowBg(1);
}
pub(crate) unsafe extern "C" fn BerryFix_HideScene() {
    HideBg(0);
    HideBg(1);
}
