//! Translated from `src/wireless_communication_status_screen.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    fn AddTextPrinterParameterized4(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: *mut u8,
        a7: i8,
        a8: *mut u8,
    );
    fn Alloc(a0: u32) -> *mut c_void;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyToBgTilemapBuffer(a0: u8, a1: *mut c_void, a2: u16, a3: u16);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTask_ListenToWireless() -> u8;
    fn DeactivateAllTextPrinters();
    fn DecompressAndLoadBgGfxUsingHeap(a0: u8, a1: *mut c_void, a2: u32, a3: u16, a4: u8);
    fn DestroyTask(a0: u8);
    fn DynamicPlaceholderTextUtil_Reset();
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn Free(a0: *mut c_void);
    fn FreeAllWindowBuffers();
    fn GetBgTilemapBuffer(a0: u8) -> *mut c_void;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsDma3ManagerBusyWithBgCopy() -> u8;
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn Menu_LoadStdPalAt(a0: u16);
    fn PlaySE(a0: u16);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RunTasks();
    fn RunTextPrinters();
    fn ScanlineEffect_Stop();
    fn SetBgTilemapBuffer(a0: u8, a1: *mut c_void);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
    fn m4aSoundVSyncOn();
}

pub(crate) unsafe extern "C" fn CB2_RunWirelessCommunicationScreen() {
    if IsDma3ManagerBusyWithBgCopy() == 0 {
        RunTasks();
        RunTextPrinters();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_WirelessCommunicationScreen() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowWirelessCommunicationScreen() {
    SetMainCallback2(Some(CB2_InitWirelessCommunicationScreen));
}
pub(crate) unsafe extern "C" fn CB2_InitWirelessCommunicationScreen() {
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
pub(crate) unsafe extern "C" fn CB2_ExitWirelessCommunicationStatusScreen() {
    let mut i: i32 = 0;
    FreeAllWindowBuffers();
    i = 0;
    while i < 2 {
        Free(GetBgTilemapBuffer(i as u8));
        i += 1;
    }
    Free(sStatusScreen as *mut c_void);
    SetMainCallback2(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
pub(crate) unsafe extern "C" fn CyclePalette(counter: *mut i16, palIdx: *mut i16) {
    let mut idx: i32 = 0;
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
    idx = *palIdx as i32 + 2;
    LoadPalette(sPalettes[idx].as_ptr().cast_mut() as *mut c_void, 0, 16);
}
pub(crate) unsafe extern "C" fn PrintHeaderTexts() {
    let mut i: i32 = 0;
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
    i = 0;
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
pub(crate) unsafe extern "C" fn Task_WirelessCommunicationScreen(taskId: u8) {
    let mut i: i32 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            PrintHeaderTexts();
            gTasks[taskId].data[0] += 1;
        }
        1 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
            ShowBg(1);
            CopyBgTilemapBufferToVram(0);
            ShowBg(0);
            gTasks[taskId].data[0] += 1;
        }
        2 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] += 1;
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
                i = 0;
                while i < NUM_GROUPTYPES {
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
                    i += 1;
                }
                PutWindowTilemap(WIN_GROUP_COUNTS);
                CopyWindowToVram(WIN_GROUP_COUNTS, COPYWIN_FULL);
            }
            if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
                PlaySE(SE_SELECT);
                gTasks[(*sStatusScreen).rfuTaskId].data[15] = 0xFF;
                gTasks[taskId].data[0] += 1;
            }
            CyclePalette(
                &raw mut gTasks[taskId].data[7],
                &raw mut gTasks[taskId].data[8],
            );
        }
        4 => {
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            gTasks[taskId].data[0] += 1;
        }
        5 => {
            if gPaletteFade.active() == 0 {
                SetMainCallback2(Some(CB2_ExitWirelessCommunicationStatusScreen));
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn WCSS_AddTextPrinterParameterized(
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
pub(crate) unsafe extern "C" fn CountPlayersInGroupAndGetActivity(
    player: *mut RfuPlayer,
    mut groupCounts: *mut u32,
) -> u32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut activity: u32 = (*player).rfu.data.activity() as u32;
    i = 0;
    while i < 31 {
        'l1: {
            if sActivityGroupInfo[i][1] == GROUPTYPE_NONE {
                break 'l1;
            }
            if activity == sActivityGroupInfo[i][0] as u32
                && (*player).groupScheduledAnim() == UNION_ROOM_SPAWN_IN
            {
                if sActivityGroupInfo[i][2] == 0 {
                    k = 0;
                    j = 0;
                    while j < RFU_CHILD_MAX as i32 {
                        if (*player).rfu.data.partnerInfo[j] != 0 {
                            k += 1;
                        }
                        j += 1;
                    }
                    k += 1;
                    *groupCounts.at(sActivityGroupInfo[i][1]) += k as u32;
                } else {
                    *groupCounts.at(sActivityGroupInfo[i][1]) += sActivityGroupInfo[i][2] as u32;
                }
            }
        }
        i += 1;
    }
    return activity;
}
pub(crate) unsafe extern "C" fn HaveCountsChanged(
    currCounts: *mut u32,
    prevCounts: *mut u32,
) -> u32 {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_GROUPTYPES {
        if *currCounts.at(i) != *prevCounts.at(i) {
            return TRUE as u32;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn UpdateCommunicationCounts(
    mut groupCounts: *mut u32,
    prevGroupCounts: *mut u32,
    mut activities: *mut u32,
    taskId: u8,
) -> u32 {
    let mut activitiesChanged: u32 = FALSE as u32;
    let mut groupCountBuffer: CArray<u32, 4> = CArray([0, 0, 0, 0]);
    let mut players: *mut *mut RfuPlayer =
        gTasks[taskId].data.as_mut_ptr() as *mut c_void as *mut *mut RfuPlayer;
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_TASK_DATA as i32 {
        let mut activity: u32 =
            CountPlayersInGroupAndGetActivity((*players).at(i), groupCountBuffer.as_mut_ptr());
        if activity != *activities.at(i) {
            *activities.at(i) = activity;
            activitiesChanged = TRUE as u32;
        }
        i += 1;
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
        return 0;
    }
}
