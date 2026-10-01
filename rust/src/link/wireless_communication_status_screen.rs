//! Translated from `src/wireless_communication_status_screen.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::CopyToBgTilemapBuffer;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    IsDma3ManagerBusyWithBgCopy, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::dynamic_placeholder_text_util::DynamicPlaceholderTextUtil_Reset;
use crate::gpu_regs::SetGpuReg;
use crate::international_string_util::GetStringCenterAlignXOffset;
use crate::m4a::m4aSoundVSyncOn;
use crate::menu::{
    AddTextPrinterParameterized4, DecompressAndLoadBgGfxUsingHeap, Menu_LoadStdPalAt,
};
use crate::overworld::CB2_ReturnToFieldContinueScriptPlayMapMusic;
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::PlaySE;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, LoadOam, ProcessSpriteCopyRequests, ResetSpriteData,
};
use crate::string_util::ConvertIntToDecimalStringN;
use crate::string_util::gStringVar4;
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_data_ptr, task_get, task_set};
use crate::text::{DeactivateAllTextPrinters, RunTextPrinters};
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::CreateTask_ListenToWireless;
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
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
/// `SetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void) {
    unsafe {
        crate::bg::SetBgTilemapBuffer(a0, a1 as _);
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
// Data tables (translate with cdata.py): sPalettes sBgTiles_Gfx sBgTiles_Tilemap sBgTemplates sWindowTemplates sHeaderTexts sActivityGroupInfo

/// `struct WirelessCommunicationStatusScreen`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WirelessCommunicationStatusScreen {
    pub groupCounts: CArray<u32, 4>,
    pub prevGroupCounts: CArray<u32, 4>,
    pub activities: CArray<u32, 16>,
    pub taskId: u8,
    pub rfuTaskId: u8,
    pub filler: CArray<u8, 10>,
}

unsafe impl Sync for WirelessCommunicationStatusScreen {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<WirelessCommunicationStatusScreen>() == 108);
    assert!(offset_of!(WirelessCommunicationStatusScreen, groupCounts) == 0);
    assert!(offset_of!(WirelessCommunicationStatusScreen, prevGroupCounts) == 16);
    assert!(offset_of!(WirelessCommunicationStatusScreen, activities) == 32);
    assert!(offset_of!(WirelessCommunicationStatusScreen, taskId) == 96);
    assert!(offset_of!(WirelessCommunicationStatusScreen, rfuTaskId) == 97);
    assert!(offset_of!(WirelessCommunicationStatusScreen, filler) == 98);
};

const COLORMODE_GREEN: u8 = 3;
const COLORMODE_NORMAL: u8 = 0;
const COLORMODE_RED: u8 = 2;
const COLORMODE_WHITE_DGRAY: u8 = 4;
const COLORMODE_WHITE_LGRAY: u8 = 1;
const GROUPTYPE_BATTLE: i32 = 1;
const GROUPTYPE_NONE: u8 = 255;
const GROUPTYPE_TOTAL: i32 = 3;
const GROUPTYPE_TRADE: i32 = 0;
const GROUPTYPE_UNION: i32 = 2;
const NUM_GROUPTYPES: i32 = 4;
const WIN_GROUP_COUNTS: u8 = 2;
const WIN_GROUP_NAMES: u8 = 1;
const WIN_TITLE: u8 = 0;

static sActivityGroupInfo: Table<CArray<CArray<u8, 3>, 31>> = Table(
    (&raw const crate::data::wireless_communication_status_screen::sActivityGroupInfo).cast(),
);
static sBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::wireless_communication_status_screen::sBgTemplates).cast());
static sBgTiles_Gfx: Table<CArray<u32, 132>> =
    Table((&raw const crate::data::wireless_communication_status_screen::sBgTiles_Gfx).cast());
static sBgTiles_Tilemap: Table<CArray<u32, 101>> =
    Table((&raw const crate::data::wireless_communication_status_screen::sBgTiles_Tilemap).cast());
static sHeaderTexts: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::wireless_communication_status_screen::sHeaderTexts).cast());
static sPalettes: Table<CArray<CArray<u16, 16>, 16>> =
    Table((&raw const crate::data::wireless_communication_status_screen::sPalettes).cast());
static sWindowTemplates: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::wireless_communication_status_screen::sWindowTemplates).cast());

pub(crate) static mut sStatusScreen: *mut WirelessCommunicationStatusScreen = null_mut();

/// `Alloc` with this module's view of its types.
#[inline]
unsafe fn Alloc(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::Alloc(a0) as *mut c_void }
}
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `GetBgTilemapBuffer` with this module's view of its types.
#[inline]
unsafe fn GetBgTilemapBuffer(a0: u8) -> *mut c_void {
    unsafe { crate::bg::GetBgTilemapBuffer(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

pub(crate) unsafe fn CB2_RunWirelessCommunicationScreen() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe fn VBlankCB_WirelessCommunicationScreen() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe fn ShowWirelessCommunicationScreen() {
    SetMainCallback2(Some(CB2_InitWirelessCommunicationScreen));
}
pub(crate) unsafe fn CB2_InitWirelessCommunicationScreen() {
    SetGpuReg(0x0, 0);
    sStatusScreen = AllocZeroed(108) as *mut WirelessCommunicationStatusScreen;
    SetVBlankCallback(None);
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sBgTemplates.as_ptr().cast_mut(), 2);
    SetBgTilemapBuffer(1, Alloc(BG_SCREEN_SIZE));
    SetBgTilemapBuffer(0, Alloc(BG_SCREEN_SIZE));
    DecompressAndLoadBgGfxUsingHeap(1, sBgTiles_Gfx.as_ptr().cast_mut() as *mut c_void, 0, 0, 0);
    CopyToBgTilemapBuffer(1, sBgTiles_Tilemap.as_ptr().cast_mut() as *mut c_void, 0, 0);
    InitWindows(sWindowTemplates.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    ResetPaletteFade();
    ResetSpriteData();
    ResetTasks();
    ScanlineEffect_Stop();
    m4aSoundVSyncOn();
    SetVBlankCallback(Some(VBlankCB_WirelessCommunicationScreen));
    (*sStatusScreen).taskId = CreateTask(Some(Task_WirelessCommunicationScreen), 0);
    (*sStatusScreen).rfuTaskId = CreateTask_ListenToWireless();
    (*sStatusScreen).prevGroupCounts[3] = 1;
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    LoadPalette(sPalettes.as_ptr().cast_mut() as *mut c_void, 0, 32);
    Menu_LoadStdPalAt(240);
    DynamicPlaceholderTextUtil_Reset();
    FillBgTilemapBufferRect(0, 0, 0, 0, 32, 32, 15);
    CopyBgTilemapBufferToVram(1);
    SetMainCallback2(Some(CB2_RunWirelessCommunicationScreen));
    RunTasks();
    RunTextPrinters();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn CB2_ExitWirelessCommunicationStatusScreen() {
    FreeAllWindowBuffers();
    for i in 0..2i32 {
        Free(GetBgTilemapBuffer(i as u8));
    }
    Free(sStatusScreen as *mut c_void);
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
unsafe fn CyclePalette(counter: *mut i16, palIdx: *mut i16) {
    if ({
        *counter += 1;
        *counter
    }) > 5
    {
        if ({
            *palIdx += 1;
            *palIdx
        }) == 14
        {
            *palIdx = 0;
        }
        *counter = 0;
    }
    let idx: i32 = *palIdx as i32 + 2;
    LoadPalette(sPalettes[idx].as_ptr().cast_mut() as *mut c_void, 0, 16);
}
unsafe fn PrintHeaderTexts() {
    FillWindowPixelBuffer(WIN_TITLE, 0);
    FillWindowPixelBuffer(WIN_GROUP_NAMES, 0);
    FillWindowPixelBuffer(WIN_GROUP_COUNTS, 0);
    WCSS_AddTextPrinterParameterized(
        WIN_TITLE,
        FONT_NORMAL,
        sHeaderTexts[0],
        GetStringCenterAlignXOffset(FONT_NORMAL as i32, sHeaderTexts[0], 0xC0) as u8,
        6,
        COLORMODE_GREEN,
    );
    let mut i: i32 = 0;
    while i < 3 {
        WCSS_AddTextPrinterParameterized(
            WIN_GROUP_NAMES,
            FONT_NORMAL,
            sHeaderTexts[i + 1],
            0,
            30 * i as u8 + 8,
            COLORMODE_WHITE_LGRAY,
        );
        i += 1;
    }
    WCSS_AddTextPrinterParameterized(
        WIN_GROUP_NAMES,
        FONT_NORMAL,
        sHeaderTexts[i + 1],
        0,
        30 * i as u8 + 8,
        COLORMODE_RED,
    );
    PutWindowTilemap(WIN_TITLE);
    CopyWindowToVram(WIN_TITLE, COPYWIN_GFX);
    PutWindowTilemap(WIN_GROUP_NAMES);
    CopyWindowToVram(WIN_GROUP_NAMES, COPYWIN_GFX);
}
pub(crate) unsafe fn Task_WirelessCommunicationScreen(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            PrintHeaderTexts();
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        1 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            ShowBg(1);
            CopyBgTilemapBufferToVram(0);
            ShowBg(0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        2 => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            if UpdateCommunicationCounts(
                (*sStatusScreen).groupCounts.as_mut_ptr(),
                (*sStatusScreen).prevGroupCounts.as_mut_ptr(),
                (*sStatusScreen).activities.as_mut_ptr(),
                (*sStatusScreen).rfuTaskId,
            ) != 0
            {
                FillWindowPixelBuffer(WIN_GROUP_COUNTS, 0);
                for i in 0..NUM_GROUPTYPES {
                    ConvertIntToDecimalStringN(
                        gStringVar4.as_mut_ptr(),
                        (*sStatusScreen).groupCounts[i] as i32,
                        STR_CONV_MODE_RIGHT_ALIGN,
                        2,
                    );
                    if i != GROUPTYPE_TOTAL {
                        WCSS_AddTextPrinterParameterized(
                            WIN_GROUP_COUNTS,
                            FONT_NORMAL,
                            gStringVar4.as_mut_ptr(),
                            12,
                            30 * i as u8 + 8,
                            COLORMODE_WHITE_LGRAY,
                        );
                    } else {
                        WCSS_AddTextPrinterParameterized(
                            WIN_GROUP_COUNTS,
                            FONT_NORMAL,
                            gStringVar4.as_mut_ptr(),
                            12,
                            98,
                            COLORMODE_RED,
                        );
                    }
                }
                PutWindowTilemap(WIN_GROUP_COUNTS);
                CopyWindowToVram(WIN_GROUP_COUNTS, COPYWIN_FULL);
            }
            if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                task_set((*sStatusScreen).rfuTaskId, 15, 0xFF);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
            CyclePalette(task_data_ptr(taskId, 7), task_data_ptr(taskId, 8));
        }
        4 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        5 if gPaletteFade.active() == 0 => {
            SetMainCallback2(Some(CB2_ExitWirelessCommunicationStatusScreen));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
unsafe fn WCSS_AddTextPrinterParameterized(
    windowId: u8,
    fontId: u8,
    str: *mut u8,
    x: u8,
    y: u8,
    mode: u8,
) {
    let mut color: CArray<u8, 3> = zeroed();
    match mode {
        COLORMODE_NORMAL => {
            color[0] = 0x0;
            color[1] = TEXT_COLOR_DARK_GRAY;
            color[2] = TEXT_COLOR_LIGHT_GRAY;
        }
        COLORMODE_WHITE_LGRAY => {
            color[0] = 0x0;
            color[1] = 0x1;
            color[2] = TEXT_COLOR_LIGHT_GRAY;
        }
        COLORMODE_RED => {
            color[0] = 0x0;
            color[1] = TEXT_COLOR_RED;
            color[2] = TEXT_COLOR_LIGHT_RED;
        }
        COLORMODE_GREEN => {
            color[0] = 0x0;
            color[1] = TEXT_COLOR_LIGHT_GREEN;
            color[2] = TEXT_COLOR_GREEN;
        }
        COLORMODE_WHITE_DGRAY => {
            color[0] = 0x0;
            color[1] = 0x1;
            color[2] = 0x2;
        }
        _ => {}
    }
    AddTextPrinterParameterized4(
        windowId,
        fontId,
        x,
        y,
        0,
        0,
        color.as_mut_ptr(),
        TEXT_SKIP_DRAW as i8,
        str,
    );
}
unsafe fn CountPlayersInGroupAndGetActivity(player: *mut RfuPlayer, groupCounts: *mut u32) -> u32 {
    let mut k: i32 = 0;
    let activity: u32 = (*player).rfu.data.activity() as u32;
    for i in 0..31i32 {
        'l1: {
            if sActivityGroupInfo[i][1] == GROUPTYPE_NONE {
                break 'l1;
            }
            if activity == sActivityGroupInfo[i][0] as u32
                && (*player).groupScheduledAnim() == UNION_ROOM_SPAWN_IN
            {
                if sActivityGroupInfo[i][2] == 0 {
                    k = 0;
                    for j in 0..(RFU_CHILD_MAX as i32) {
                        if (*player).rfu.data.partnerInfo[j] != 0 {
                            k += 1;
                        }
                    }
                    k += 1;
                    *groupCounts.at(sActivityGroupInfo[i][1]) += k as u32;
                } else {
                    *groupCounts.at(sActivityGroupInfo[i][1]) += sActivityGroupInfo[i][2] as u32;
                }
            }
        }
    }
    activity
}
unsafe fn HaveCountsChanged(currCounts: *mut u32, prevCounts: *mut u32) -> u32 {
    for i in 0..NUM_GROUPTYPES {
        if *currCounts.at(i) != *prevCounts.at(i) {
            return TRUE as u32;
        }
    }
    FALSE as u32
}
unsafe fn UpdateCommunicationCounts(
    groupCounts: *mut u32,
    prevGroupCounts: *mut u32,
    activities: *mut u32,
    taskId: u8,
) -> u32 {
    let mut activitiesChanged: u32 = FALSE as u32;
    let mut groupCountBuffer: CArray<u32, 4> = CArray([0, 0, 0, 0]);
    let players: *mut *mut RfuPlayer =
        (*gTasks.as_ptr())[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuPlayer;
    for i in 0..(NUM_TASK_DATA as i32) {
        let activity: u32 =
            CountPlayersInGroupAndGetActivity((*players).at(i), groupCountBuffer.as_mut_ptr());
        if activity != *activities.at(i) {
            *activities.at(i) = activity;
            activitiesChanged = TRUE as u32;
        }
    }
    if HaveCountsChanged(groupCountBuffer.as_mut_ptr(), prevGroupCounts) == 0 {
        if activitiesChanged == TRUE as u32 {
            return TRUE as u32;
        } else {
            return FALSE as u32;
        }
    } else {
        memcpy(
            groupCounts as *mut u8,
            groupCountBuffer.as_mut_ptr() as *mut u8,
            16,
        );
        memcpy(
            prevGroupCounts as *mut u8,
            groupCountBuffer.as_mut_ptr() as *mut u8,
            16,
        );
        *groupCounts.at(3) =
            *groupCounts + *groupCounts.at(1) + *groupCounts.at(2) + *groupCounts.at(3);
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
