//! Translated from `src/main_menu.c` by tools/rustport/c2rs.py.
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
    clippy::eq_op,
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    clippy::useless_transmute,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::SetVBlankCallback;
use crate::agb_main::gMain;
use crate::bg::LoadBgTiles;
use crate::bg::{
    ChangeBgX, ChangeBgY, CopyBgTilemapBufferToVram, FillBgTilemapBufferRect,
    FillBgTilemapBufferRect_Palette0, HideBg, ResetBgsAndClearDma3BusyFlags, ShowBg,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, IsMysteryGiftEnabled, IsNationalPokedexEnabled};
use crate::field_effect::{AddNewGameBirchObject, CreateTrainerSprite};
use crate::gpu_regs::{EnableInterrupts, SetGpuReg};
use crate::international_string_util::GetStringRightAlignXOffset;
use crate::link::IsWirelessAdapterConnected;
use crate::list_menu::{
    AddScrollIndicatorArrowPair, RemoveScrollIndicatorArrowPair,
    Task_ScrollIndicatorArrowPairOnMainMenu,
};
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    AddTextPrinterForMessage, AddTextPrinterParameterized3, AddTextPrinterWithCallbackForMessage,
    ClearStdWindowAndFrame, CreateWindowTemplate, CreateYesNoMenu, InitMenuInUpperLeftCornerNormal,
    Menu_GetCursorPos, Menu_ProcessInputNoWrap, Menu_ProcessInputNoWrapClearOnChoose,
    PrintMenuTable, RunTextPrintersAndIsPrinter0Active,
};
use crate::mystery_event_menu::CB2_InitMysteryEventMenu;
use crate::mystery_gift_menu::{CB2_InitEReader, CB2_InitMysteryGift};
use crate::naming_screen::DoNamingScreen;
use crate::option_menu::CB2_InitOptionMenu;
use crate::overworld::{CB2_ContinueSavedGame, CB2_NewGame};
use crate::palette::{
    BeginNormalPaletteFade, LoadPalette, ResetPaletteFade, TransferPlttBuffer, UpdatePaletteFade,
    gPaletteFade,
};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokeball::CreatePokeballSpriteToReleaseMon;
use crate::pokedex::{GetHoennPokedexCount, GetNationalPokedexCount};
use crate::pokemon::FacilityClassToPicIndex;
use crate::random::Random;
use crate::rtc::RtcGetErrorStatus;
use crate::save::gSaveFileStatus;
use crate::scanline_effect::ScanlineEffect_Stop;
use crate::sound::{FadeOutBGM, PlayBGM, PlaySE};
use crate::sprite::gSprites;
use crate::sprite::{
    AnimateSprites, BuildOamBuffer, FreeAllSpritePalettes, LoadOam, ProcessSpriteCopyRequests,
    ResetSpriteData,
};
use crate::string_util::gStringVar4;
use crate::string_util::{ConvertIntToDecimalStringN, StringExpandPlaceholders};
use crate::task::gTasks;
use crate::task::{DestroyTask, ResetTasks, RunTasks};
use crate::task::{task_get, task_set, task_set_func};
use crate::text::{
    DeactivateAllTextPrinters, GetFontAttribute, IsTextPrinterActive, RunTextPrinters,
};
use crate::text_window::LoadMessageBoxGfx;
use crate::title_screen::CB2_InitTitleScreen;
use crate::trainer_pokemon_sprites::{
    CreateMonPicSprite_Affine, FreeAndDestroyMonPicSprite, ResetAllPicSprites,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    FreeAllWindowBuffers, GetWindowAttribute, PutWindowTilemap,
};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CallWindowFunction` with this module's view of its types.
#[inline]
unsafe fn CallWindowFunction(a0: u8, a1: Option<unsafe fn(u8, u8, u8, u8, u8, u8)>) {
    unsafe {
        crate::window::CallWindowFunction(a0, core::mem::transmute(a1));
    }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `InitBgFromTemplate` with this module's view of its types.
#[inline]
unsafe fn InitBgFromTemplate(a0: *mut BgTemplate) {
    unsafe {
        crate::bg::InitBgFromTemplate(a0 as _);
    }
}
/// `InitBgsFromTemplates` with this module's view of its types.
#[inline]
unsafe fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8) {
    unsafe {
        crate::bg::InitBgsFromTemplates(a0, a1 as _, a2);
    }
}
/// `InitSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn InitSpriteAffineAnim(a0: *mut Sprite) {
    unsafe {
        crate::sprite::InitSpriteAffineAnim(a0 as _);
    }
}
/// `InitWindows` with this module's view of its types.
#[inline]
unsafe fn InitWindows(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::InitWindows(a0 as _) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const tMainTask: usize = 0;
const tMenuType: usize = 0;
const tAlphaCoeff1: usize = 1;
const tCurrItem: usize = 1;
const tPalIndex: usize = 1;
const tAlphaCoeff2: usize = 2;
const tDelayBefore: usize = 2;
const tPlayerSpriteId: usize = 2;
const tDelay: usize = 3;
const tBG1HOFS: usize = 4;
const tDelayTimer: usize = 4;
const tIsDoneFadingSprites: usize = 5;
const tPlayerGender: usize = 6;
const tTimer: usize = 7;
const tBirchSpriteId: usize = 8;
const tLotadSpriteId: usize = 9;
const tBrendanSpriteId: usize = 10;
const tMaySpriteId: usize = 11;
const tScrollArrowTaskId: usize = 13;
const tIsScrolled: usize = 14;
const tArrowTaskIsScrolled: usize = 15;
const tWirelessAdapterConnected: usize = 15;
// Data tables (translate with cdata.py): sBirchSpeechBgPals sBirchSpeechShadowGfx sBirchSpeechBgMap sBirchSpeechBgGradientPal sWindowTemplates_MainMenu sNewGameBirchSpeechTextWindows sMainMenuBgPal sMainMenuTextPal sTextColor_Headers sTextColor_MenuInfo sMainMenuBgTemplates sBirchBgTemplate sScrollArrowsTemplate_MainMenu sSpriteAffineAnim_PlayerShrink sSpriteAffineAnimTable_PlayerShrink sMenuActions_Gender sMalePresetNames sFemalePresetNames

const ACTION_CONTINUE: u8 = 1;
const ACTION_EREADER: u8 = 5;
const ACTION_INVALID: u8 = 6;
const ACTION_MYSTERY_EVENTS: u8 = 4;
const ACTION_MYSTERY_GIFT: u8 = 3;
const ACTION_NEW_GAME: u8 = 0;
const ACTION_OPTION: u8 = 2;
const BIRCH_DLG_BASE_TILE_NUM: u16 = 252;
const HAS_MYSTERY_EVENTS: i16 = 3;
const HAS_MYSTERY_GIFT: i16 = 2;
const HAS_NO_SAVED_GAME: i16 = 0;
const HAS_SAVED_GAME: i16 = 1;
const MAIN_MENU_BORDER_TILE: u16 = 469;
const OPTION_MENU_FLAG: i32 = 32768;

static sBirchBgTemplate: Table<BgTemplate> =
    Table((&raw const crate::data::main_menu::sBirchBgTemplate).cast());
static sBirchSpeechBgGradientPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::main_menu::sBirchSpeechBgGradientPal).cast());
static sBirchSpeechBgMap: Table<CArray<u32, 74>> =
    Table((&raw const crate::data::main_menu::sBirchSpeechBgMap).cast());
static sBirchSpeechBgPals: Table<CArray<CArray<u16, 16>, 2>> =
    Table((&raw const crate::data::main_menu::sBirchSpeechBgPals).cast());
static sBirchSpeechShadowGfx: Table<CArray<u32, 109>> =
    Table((&raw const crate::data::main_menu::sBirchSpeechShadowGfx).cast());
static sFemalePresetNames: Table<CArray<*mut u8, 20>> =
    Table((&raw const crate::data::main_menu::sFemalePresetNames).cast());
static sMainMenuBgPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::main_menu::sMainMenuBgPal).cast());
static sMainMenuBgTemplates: Table<CArray<BgTemplate, 2>> =
    Table((&raw const crate::data::main_menu::sMainMenuBgTemplates).cast());
static sMainMenuTextPal: Table<CArray<u16, 16>> =
    Table((&raw const crate::data::main_menu::sMainMenuTextPal).cast());
static sMalePresetNames: Table<CArray<*mut u8, 20>> =
    Table((&raw const crate::data::main_menu::sMalePresetNames).cast());
static sMenuActions_Gender: Table<CArray<MenuAction, 2>> =
    Table((&raw const crate::data::main_menu::sMenuActions_Gender).cast());
static sNewGameBirchSpeechTextWindows: Table<CArray<WindowTemplate, 4>> =
    Table((&raw const crate::data::main_menu::sNewGameBirchSpeechTextWindows).cast());
static sScrollArrowsTemplate_MainMenu: Table<ScrollArrowsTemplate> =
    Table((&raw const crate::data::main_menu::sScrollArrowsTemplate_MainMenu).cast());
static sSpriteAffineAnimTable_PlayerShrink: Table<CArray<*mut AffineAnimCmd, 1>> =
    Table((&raw const crate::data::main_menu::sSpriteAffineAnimTable_PlayerShrink).cast());
static sTextColor_Headers: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::main_menu::sTextColor_Headers).cast());
static sTextColor_MenuInfo: Table<CArray<u8, 3>> =
    Table((&raw const crate::data::main_menu::sTextColor_MenuInfo).cast());
static sWindowTemplates_MainMenu: Table<CArray<WindowTemplate, 9>> =
    Table((&raw const crate::data::main_menu::sWindowTemplates_MainMenu).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sStartedPokeBallTask: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sCurrItemAndOptionMenuCheck: crate::global::Global<u16> =
    crate::global::Global::new(0);
pub(crate) static sBirchSpeechMainTaskId: crate::global::Global<u8> = crate::global::Global::new(0);

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

pub(crate) unsafe fn CB2_MainMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe fn VBlankCB_MainMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
pub unsafe fn CB2_InitMainMenu() {
    InitMainMenu(FALSE);
}
pub unsafe fn CB2_ReinitMainMenu() {
    InitMainMenu(TRUE);
}
unsafe fn InitMainMenu(returningFromOptionsMenu: u8) -> u32 {
    SetVBlankCallback(None);
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), VRAM as usize as *mut c_void as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x8100c000);
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        OAM as i32 as usize as *mut c_void as usize as u32,
                    );
                    volatile_write(dmaRegs.at(2), 0x85000100);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), 83886082_usize as *mut c_void as usize as u32);
                    volatile_write(dmaRegs.at(2), 0x810001ff);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    ResetPaletteFade();
    LoadPalette(sMainMenuBgPal.as_ptr().cast_mut() as *mut c_void, 0, 32);
    LoadPalette(sMainMenuTextPal.as_ptr().cast_mut() as *mut c_void, 240, 32);
    ScanlineEffect_Stop();
    ResetTasks();
    ResetSpriteData();
    FreeAllSpritePalettes();
    if returningFromOptionsMenu != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 0);
    } else {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0x10, 0, 65535);
    }
    ResetBgsAndClearDma3BusyFlags(0);
    InitBgsFromTemplates(0, sMainMenuBgTemplates.as_ptr().cast_mut(), 2);
    ChangeBgX(0, 0, BG_COORD_SET);
    ChangeBgY(0, 0, BG_COORD_SET);
    ChangeBgX(1, 0, BG_COORD_SET);
    ChangeBgY(1, 0, BG_COORD_SET);
    InitWindows(sWindowTemplates_MainMenu.as_ptr().cast_mut());
    DeactivateAllTextPrinters();
    LoadMainMenuWindowFrameTiles(0, MAIN_MENU_BORDER_TILE);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    EnableInterrupts(1);
    SetVBlankCallback(Some(VBlankCB_MainMenu));
    SetMainCallback2(Some(CB2_MainMenu));
    SetGpuReg(REG_OFFSET_DISPCNT, 12352);
    ShowBg(0);
    HideBg(1);
    CreateTask(Some(Task_MainMenuCheckSaveFile), 0);
    0
}
pub(crate) unsafe fn Task_MainMenuCheckSaveFile(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        SetGpuReg(REG_OFFSET_WIN0H, 0);
        SetGpuReg(REG_OFFSET_WIN0V, 0);
        SetGpuReg(REG_OFFSET_WININ, 17);
        SetGpuReg(REG_OFFSET_WINOUT, 49);
        SetGpuReg(REG_OFFSET_BLDCNT, 193);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        SetGpuReg(REG_OFFSET_BLDY, 7);
        if IsWirelessAdapterConnected() != 0 {
            *data.at(15) = TRUE as i16;
        }
        match gSaveFileStatus {
            1 => {
                *data = HAS_SAVED_GAME;
                if IsMysteryGiftEnabled() != 0 {
                    *data += 1;
                }
                task_set_func(taskId, Some(Task_MainMenuCheckBattery));
            }
            SAVE_STATUS_CORRUPT => {
                CreateMainMenuErrorWindow(
                    (*(&raw const crate::data::strings::gText_SaveFileErased)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                *data = HAS_NO_SAVED_GAME;
                task_set_func(taskId, Some(Task_WaitForSaveFileErrorWindow));
            }
            255 => {
                CreateMainMenuErrorWindow(
                    (*(&raw const crate::data::strings::gText_SaveFileCorrupted)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                task_set_func(taskId, Some(Task_WaitForSaveFileErrorWindow));
                *data = HAS_SAVED_GAME;
                if IsMysteryGiftEnabled() == TRUE as u32 {
                    *data += 1;
                }
            }
            SAVE_STATUS_NO_FLASH => {
                CreateMainMenuErrorWindow(
                    (*(&raw const crate::data::strings::gJPText_No1MSubCircuit)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                task_set(taskId, tMenuType, HAS_NO_SAVED_GAME);
                task_set_func(taskId, Some(Task_WaitForSaveFileErrorWindow));
            }
            _ => {
                *data = HAS_NO_SAVED_GAME;
                task_set_func(taskId, Some(Task_MainMenuCheckBattery));
            }
        }
        if sCurrItemAndOptionMenuCheck.get() as i32 & OPTION_MENU_FLAG != 0 {
            match *data {
                HAS_NO_SAVED_GAME | HAS_SAVED_GAME => {
                    sCurrItemAndOptionMenuCheck.set(*data as u16 + 1);
                }
                HAS_MYSTERY_GIFT => {
                    sCurrItemAndOptionMenuCheck.set(3);
                }
                HAS_MYSTERY_EVENTS => {
                    sCurrItemAndOptionMenuCheck.set(4);
                }
                _ => {}
            }
        }
        sCurrItemAndOptionMenuCheck.set(sCurrItemAndOptionMenuCheck.get() & 32767);
        *data.at(1) = sCurrItemAndOptionMenuCheck.get() as i16;
        *data.at(12) = *data + 2;
    }
}
pub(crate) unsafe fn Task_WaitForSaveFileErrorWindow(taskId: u8) {
    RunTextPrinters();
    if IsTextPrinterActive(7) == 0 && gMain.newKeys as i32 & A_BUTTON != 0 {
        ClearWindowTilemap(7);
        ClearMainMenuWindowTilemap((&raw const sWindowTemplates_MainMenu[7]).cast_mut());
        task_set_func(taskId, Some(Task_MainMenuCheckBattery));
    }
}
pub(crate) unsafe fn Task_MainMenuCheckBattery(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetGpuReg(REG_OFFSET_WIN0H, 0);
        SetGpuReg(REG_OFFSET_WIN0V, 0);
        SetGpuReg(REG_OFFSET_WININ, 17);
        SetGpuReg(REG_OFFSET_WINOUT, 49);
        SetGpuReg(REG_OFFSET_BLDCNT, 193);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        SetGpuReg(REG_OFFSET_BLDY, 7);
        if RtcGetErrorStatus() as i32 & RTC_ERR_FLAG_MASK == 0 {
            task_set_func(taskId, Some(Task_DisplayMainMenu));
        } else {
            CreateMainMenuErrorWindow(
                (*(&raw const crate::data::strings::gText_BatteryRunDry).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            task_set_func(taskId, Some(Task_WaitForBatteryDryErrorWindow));
        }
    }
}
pub(crate) unsafe fn Task_WaitForBatteryDryErrorWindow(taskId: u8) {
    RunTextPrinters();
    if IsTextPrinterActive(7) == 0 && gMain.newKeys as i32 & A_BUTTON != 0 {
        ClearWindowTilemap(7);
        ClearMainMenuWindowTilemap((&raw const sWindowTemplates_MainMenu[7]).cast_mut());
        task_set_func(taskId, Some(Task_DisplayMainMenu));
    }
}
pub(crate) unsafe fn Task_DisplayMainMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut palette: u16 = 0;
    if gPaletteFade.active() == 0 {
        SetGpuReg(REG_OFFSET_WIN0H, 0);
        SetGpuReg(REG_OFFSET_WIN0V, 0);
        SetGpuReg(REG_OFFSET_WININ, 17);
        SetGpuReg(REG_OFFSET_WINOUT, 49);
        SetGpuReg(REG_OFFSET_BLDCNT, 193);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        SetGpuReg(REG_OFFSET_BLDY, 7);
        palette = 0;
        LoadPalette(&raw mut palette as *mut c_void, 254, 2);
        palette = 32767;
        LoadPalette(&raw mut palette as *mut c_void, 250, 2);
        palette = 12684;
        LoadPalette(&raw mut palette as *mut c_void, 251, 2);
        palette = 26458;
        LoadPalette(&raw mut palette as *mut c_void, 252, 2);
        if (*gSaveBlock2Ptr).playerGender == MALE {
            palette = 32260;
            LoadPalette(&raw mut palette as *mut c_void, 241, 2);
        } else {
            palette = 21631;
            LoadPalette(&raw mut palette as *mut c_void, 241, 2);
        }
        match task_get(taskId, tMenuType) {
            HAS_SAVED_GAME => {
                FillWindowPixelBuffer(2, 170);
                FillWindowPixelBuffer(3, 170);
                FillWindowPixelBuffer(4, 170);
                AddTextPrinterParameterized3(
                    2,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuContinue)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    3,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuNewGame)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    4,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuOption)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                MainMenu_FormatSavegameText();
                PutWindowTilemap(2);
                PutWindowTilemap(3);
                PutWindowTilemap(4);
                CopyWindowToVram(2, COPYWIN_GFX);
                CopyWindowToVram(3, COPYWIN_GFX);
                CopyWindowToVram(4, COPYWIN_GFX);
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[2]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[3]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[4]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
            }
            HAS_MYSTERY_GIFT => {
                FillWindowPixelBuffer(2, 170);
                FillWindowPixelBuffer(3, 170);
                FillWindowPixelBuffer(4, 170);
                FillWindowPixelBuffer(5, 170);
                AddTextPrinterParameterized3(
                    2,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuContinue)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    3,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuNewGame)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    4,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuMysteryGift)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    5,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuOption)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                MainMenu_FormatSavegameText();
                PutWindowTilemap(2);
                PutWindowTilemap(3);
                PutWindowTilemap(4);
                PutWindowTilemap(5);
                CopyWindowToVram(2, COPYWIN_GFX);
                CopyWindowToVram(3, COPYWIN_GFX);
                CopyWindowToVram(4, COPYWIN_GFX);
                CopyWindowToVram(5, COPYWIN_GFX);
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[2]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[3]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[4]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[5]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
            }
            HAS_MYSTERY_EVENTS => {
                FillWindowPixelBuffer(2, 170);
                FillWindowPixelBuffer(3, 170);
                FillWindowPixelBuffer(4, 170);
                FillWindowPixelBuffer(5, 170);
                FillWindowPixelBuffer(6, 170);
                AddTextPrinterParameterized3(
                    2,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuContinue)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    3,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuNewGame)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    4,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuMysteryGift2)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    5,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuMysteryEvents)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    6,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuOption)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                MainMenu_FormatSavegameText();
                PutWindowTilemap(2);
                PutWindowTilemap(3);
                PutWindowTilemap(4);
                PutWindowTilemap(5);
                PutWindowTilemap(6);
                CopyWindowToVram(2, COPYWIN_GFX);
                CopyWindowToVram(3, COPYWIN_GFX);
                CopyWindowToVram(4, COPYWIN_GFX);
                CopyWindowToVram(5, COPYWIN_GFX);
                CopyWindowToVram(6, COPYWIN_GFX);
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[2]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[3]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[4]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[5]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[6]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                *data.at(13) = AddScrollIndicatorArrowPair(
                    (&raw const *sScrollArrowsTemplate_MainMenu).cast_mut(),
                    sCurrItemAndOptionMenuCheck.as_ptr(),
                ) as i16;
                task_set_func(*data.at(13), Some(Task_ScrollIndicatorArrowPairOnMainMenu));
                if sCurrItemAndOptionMenuCheck.get() == 4 {
                    ChangeBgY(0, 0x2000, BG_COORD_ADD);
                    ChangeBgY(1, 0x2000, BG_COORD_ADD);
                    *data.at(14) = TRUE as i16;
                    task_set(*data.at(13), tArrowTaskIsScrolled, TRUE as i16);
                }
            }
            _ => {
                FillWindowPixelBuffer(0, 170);
                FillWindowPixelBuffer(1, 170);
                AddTextPrinterParameterized3(
                    0,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuNewGame)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                AddTextPrinterParameterized3(
                    1,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    (*(&raw const crate::data::strings::gText_MainMenuOption)
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                );
                PutWindowTilemap(0);
                PutWindowTilemap(1);
                CopyWindowToVram(0, COPYWIN_GFX);
                CopyWindowToVram(1, COPYWIN_GFX);
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[0]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
                DrawMainMenuWindowBorder(
                    (&raw const sWindowTemplates_MainMenu[1]).cast_mut(),
                    MAIN_MENU_BORDER_TILE,
                );
            }
        }
        task_set_func(taskId, Some(Task_HighlightSelectedMainMenuItem));
    }
}
pub(crate) unsafe fn Task_HighlightSelectedMainMenuItem(taskId: u8) {
    HighlightSelectedMainMenuItem(
        task_get(taskId, tMenuType) as u8,
        task_get(taskId, tCurrItem) as u8,
        task_get(taskId, tIsScrolled),
    );
    task_set_func(taskId, Some(Task_HandleMainMenuInput));
}
pub(crate) unsafe fn HandleMainMenuInput(taskId: u8) -> u8 {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        IsWirelessAdapterConnected();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
        task_set_func(taskId, Some(Task_HandleMainMenuAPressed));
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 65535);
        SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
        SetGpuReg(REG_OFFSET_WIN0V, DISPLAY_HEIGHT);
        task_set_func(taskId, Some(Task_HandleMainMenuBPressed));
    } else if gMain.newKeys as i32 & DPAD_UP != 0 && *data.at(1) > 0 {
        if *data == HAS_MYSTERY_EVENTS && *data.at(14) == 1 && *data.at(1) == 1 {
            ChangeBgY(0, 0x2000, BG_COORD_SUB);
            ChangeBgY(1, 0x2000, BG_COORD_SUB);
            task_set(*data.at(13), tArrowTaskIsScrolled, {
                *data.at(14) = FALSE as i16;
                *data.at(14)
            });
        }
        *data.at(1) -= 1;
        sCurrItemAndOptionMenuCheck.set(*data.at(1) as u16);
        return TRUE;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0
        && (*data.at(1) as i32) < *data.at(12) as i32 - 1
    {
        if *data == HAS_MYSTERY_EVENTS && *data.at(1) == 3 && *data.at(14) == FALSE as i16 {
            ChangeBgY(0, 0x2000, BG_COORD_ADD);
            ChangeBgY(1, 0x2000, BG_COORD_ADD);
            task_set(*data.at(13), tArrowTaskIsScrolled, {
                *data.at(14) = TRUE as i16;
                *data.at(14)
            });
        }
        *data.at(1) += 1;
        sCurrItemAndOptionMenuCheck.set(*data.at(1) as u16);
        return TRUE;
    }
    FALSE
}
pub(crate) unsafe fn Task_HandleMainMenuInput(taskId: u8) {
    if HandleMainMenuInput(taskId) != 0 {
        task_set_func(taskId, Some(Task_HighlightSelectedMainMenuItem));
    }
}
pub(crate) unsafe fn Task_HandleMainMenuAPressed(taskId: u8) {
    let mut wirelessAdapterConnected: u8 = 0;
    let mut action: u8 = 0;
    if gPaletteFade.active() == 0 {
        if task_get(taskId, tMenuType) == HAS_MYSTERY_EVENTS {
            RemoveScrollIndicatorArrowPair(task_get(taskId, tScrollArrowTaskId) as u8);
        }
        ClearStdWindowAndFrame(0, TRUE);
        ClearStdWindowAndFrame(1, 1);
        ClearStdWindowAndFrame(2, TRUE);
        ClearStdWindowAndFrame(3, TRUE);
        ClearStdWindowAndFrame(4, TRUE);
        ClearStdWindowAndFrame(5, TRUE);
        ClearStdWindowAndFrame(6, TRUE);
        ClearStdWindowAndFrame(7, TRUE);
        wirelessAdapterConnected = IsWirelessAdapterConnected();
        match task_get(taskId, tMenuType) {
            HAS_SAVED_GAME => match task_get(taskId, tCurrItem) {
                1 => {
                    action = ACTION_NEW_GAME;
                }
                2 => {
                    action = ACTION_OPTION;
                }
                _ => {
                    action = ACTION_CONTINUE;
                }
            },
            HAS_MYSTERY_GIFT => match task_get(taskId, tCurrItem) {
                1 => {
                    action = ACTION_NEW_GAME;
                }
                2 => {
                    action = ACTION_MYSTERY_GIFT;
                    if wirelessAdapterConnected == 0 {
                        action = ACTION_INVALID;
                        task_set(taskId, tMenuType, HAS_NO_SAVED_GAME);
                    }
                }
                3 => {
                    action = ACTION_OPTION;
                }
                _ => {
                    action = ACTION_CONTINUE;
                }
            },
            HAS_MYSTERY_EVENTS => match task_get(taskId, tCurrItem) {
                1 => {
                    action = ACTION_NEW_GAME;
                }
                2 => {
                    if task_get(taskId, tWirelessAdapterConnected) != 0 {
                        action = ACTION_MYSTERY_GIFT;
                        if wirelessAdapterConnected == 0 {
                            action = ACTION_INVALID;
                            task_set(taskId, tMenuType, HAS_NO_SAVED_GAME);
                        }
                    } else if wirelessAdapterConnected != 0 {
                        action = ACTION_INVALID;
                        task_set(taskId, tMenuType, HAS_SAVED_GAME);
                    } else {
                        action = ACTION_EREADER;
                    }
                }
                3 => {
                    if wirelessAdapterConnected != 0 {
                        action = ACTION_INVALID;
                        task_set(taskId, tMenuType, HAS_MYSTERY_GIFT);
                    } else {
                        action = ACTION_MYSTERY_EVENTS;
                    }
                }
                4 => {
                    action = ACTION_OPTION;
                }
                _ => {
                    action = ACTION_CONTINUE;
                }
            },
            _ => match task_get(taskId, tCurrItem) {
                1 => {
                    action = ACTION_OPTION;
                }
                _ => {
                    action = ACTION_NEW_GAME;
                }
            },
        }
        ChangeBgY(0, 0, BG_COORD_SET);
        ChangeBgY(1, 0, BG_COORD_SET);
        match action {
            ACTION_CONTINUE => {
                gPlttBufferUnfaded[0] = 0;
                gPlttBufferFaded[0] = 0;
                SetMainCallback2(Some(CB2_ContinueSavedGame));
                DestroyTask(taskId);
            }
            ACTION_OPTION => {
                gMain.savedCallback = Some(CB2_ReinitMainMenu);
                SetMainCallback2(Some(CB2_InitOptionMenu));
                DestroyTask(taskId);
            }
            ACTION_MYSTERY_GIFT => {
                SetMainCallback2(Some(CB2_InitMysteryGift));
                DestroyTask(taskId);
            }
            ACTION_MYSTERY_EVENTS => {
                SetMainCallback2(Some(CB2_InitMysteryEventMenu));
                DestroyTask(taskId);
            }
            ACTION_EREADER => {
                SetMainCallback2(Some(CB2_InitEReader));
                DestroyTask(taskId);
            }
            ACTION_INVALID => {
                task_set(taskId, tCurrItem, 0);
                task_set_func(taskId, Some(Task_DisplayMainMenuInvalidActionError));
                gPlttBufferUnfaded[241] = 32767;
                gPlttBufferFaded[241] = 32767;
                SetGpuReg(REG_OFFSET_BG2HOFS, 0);
                SetGpuReg(REG_OFFSET_BG2VOFS, 0);
                SetGpuReg(REG_OFFSET_BG1HOFS, 0);
                SetGpuReg(REG_OFFSET_BG1VOFS, 0);
                SetGpuReg(REG_OFFSET_BG0HOFS, 0);
                SetGpuReg(REG_OFFSET_BG0VOFS, 0);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
                return;
            }
            _ => {
                gPlttBufferUnfaded[0] = 0;
                gPlttBufferFaded[0] = 0;
                task_set_func(taskId, Some(Task_NewGameBirchSpeech_Init));
            }
        }
        FreeAllWindowBuffers();
        if action != ACTION_OPTION {
            sCurrItemAndOptionMenuCheck.set(0);
        } else {
            sCurrItemAndOptionMenuCheck
                .set(sCurrItemAndOptionMenuCheck.get() | (OPTION_MENU_FLAG as u16));
        }
    }
}
pub(crate) unsafe fn Task_HandleMainMenuBPressed(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if task_get(taskId, tMenuType) == HAS_MYSTERY_EVENTS {
            RemoveScrollIndicatorArrowPair(task_get(taskId, tScrollArrowTaskId) as u8);
        }
        sCurrItemAndOptionMenuCheck.set(0);
        FreeAllWindowBuffers();
        SetMainCallback2(Some(CB2_InitTitleScreen));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_DisplayMainMenuInvalidActionError(taskId: u8) {
    match task_get(taskId, tCurrItem) {
        0 => {
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            match task_get(taskId, tMenuType) {
                0 => {
                    CreateMainMenuErrorWindow(
                        (*(&raw const crate::data::strings::gText_WirelessNotConnected)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                1 => {
                    CreateMainMenuErrorWindow(
                        (*(&raw const crate::data::strings::gText_MysteryGiftCantUse)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                2 => {
                    CreateMainMenuErrorWindow(
                        (*(&raw const crate::data::strings::gText_MysteryEventsCantUse)
                            .cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                    );
                }
                _ => {}
            }
            task_set(taskId, tCurrItem, task_get(taskId, tCurrItem) + 1);
        }
        1 => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, tCurrItem, task_get(taskId, tCurrItem) + 1);
            }
        }
        2 => {
            RunTextPrinters();
            if IsTextPrinterActive(7) == 0 {
                task_set(taskId, tCurrItem, task_get(taskId, tCurrItem) + 1);
            }
        }
        3 if gMain.newKeys as i32 & 3 != 0 => {
            PlaySE(SE_SELECT);
            BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
            task_set_func(taskId, Some(Task_HandleMainMenuBPressed));
        }
        _ => {}
    }
}
unsafe fn HighlightSelectedMainMenuItem(menuType: u8, selectedMenuItem: u8, isScrolled: i16) {
    SetGpuReg(REG_OFFSET_WIN0H, 2535);
    match menuType {
        1 => match selectedMenuItem {
            1 => {
                SetGpuReg(REG_OFFSET_WIN0V, 16735);
            }
            2 => {
                SetGpuReg(REG_OFFSET_WIN0V, 24959);
            }
            _ => {
                SetGpuReg(REG_OFFSET_WIN0V, 319);
            }
        },
        2 => match selectedMenuItem {
            1 => {
                SetGpuReg(REG_OFFSET_WIN0V, 16735);
            }
            2 => {
                SetGpuReg(REG_OFFSET_WIN0V, 24959);
            }
            3 => {
                SetGpuReg(REG_OFFSET_WIN0V, 33183);
            }
            _ => {
                SetGpuReg(REG_OFFSET_WIN0V, 319);
            }
        },
        3 => match selectedMenuItem {
            1 => {
                if isScrolled != 0 {
                    SetGpuReg(REG_OFFSET_WIN0V, 8511);
                } else {
                    SetGpuReg(REG_OFFSET_WIN0V, 16735);
                }
            }
            2 => {
                if isScrolled != 0 {
                    SetGpuReg(REG_OFFSET_WIN0V, 16735);
                } else {
                    SetGpuReg(REG_OFFSET_WIN0V, 24959);
                }
            }
            3 => {
                if isScrolled != 0 {
                    SetGpuReg(REG_OFFSET_WIN0V, 24959);
                } else {
                    SetGpuReg(REG_OFFSET_WIN0V, 33183);
                }
            }
            4 => {
                SetGpuReg(REG_OFFSET_WIN0V, 33183);
            }
            _ => {
                SetGpuReg(REG_OFFSET_WIN0V, 319);
            }
        },
        _ => match selectedMenuItem {
            1 => {
                SetGpuReg(REG_OFFSET_WIN0V, 8511);
            }
            _ => {
                SetGpuReg(REG_OFFSET_WIN0V, 287);
            }
        },
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_Init(taskId: u8) {
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    InitBgFromTemplate((&raw const *sBirchBgTemplate).cast_mut());
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    LZ77UnCompVram(
        sBirchSpeechShadowGfx.as_ptr().cast_mut(),
        VRAM as usize as *mut c_void,
    );
    LZ77UnCompVram(
        sBirchSpeechBgMap.as_ptr().cast_mut(),
        0x6003800_usize as *mut c_void,
    );
    LoadPalette(sBirchSpeechBgPals.as_ptr().cast_mut() as *mut c_void, 0, 64);
    LoadPalette(
        (&raw const sBirchSpeechBgGradientPal[8]).cast_mut() as *mut c_void,
        1,
        16,
    );
    ScanlineEffect_Stop();
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetAllPicSprites();
    AddBirchSpeechObjects(taskId);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    task_set(taskId, tBG1HOFS, 0);
    task_set_func(taskId, Some(Task_NewGameBirchSpeech_WaitToShowBirch));
    task_set(taskId, tPlayerSpriteId, SPRITE_NONE as i16);
    task_set(taskId, 3, 0xFF);
    task_set(taskId, tTimer, 0xD8);
    PlayBGM(MUS_ROUTE122);
    ShowBg(0);
    ShowBg(1);
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitToShowBirch(taskId: u8) {
    let mut spriteId: u8 = 0;
    if task_get(taskId, tTimer) != 0 {
        task_set(taskId, tTimer, task_get(taskId, tTimer) - 1);
    } else {
        spriteId = task_get(taskId, tBirchSpriteId) as u8;
        gSprites[spriteId].x = 136;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 10);
        NewGameBirchSpeech_StartFadePlatformOut(taskId, 20);
        task_set(taskId, tTimer, 80);
        task_set_func(
            taskId,
            Some(Task_NewGameBirchSpeech_WaitForSpriteFadeInWelcome),
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitForSpriteFadeInWelcome(taskId: u8) {
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tBirchSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        if task_get(taskId, tTimer) != 0 {
            task_set(taskId, tTimer, task_get(taskId, tTimer) - 1);
        } else {
            InitWindows(sNewGameBirchSpeechTextWindows.as_ptr().cast_mut());
            LoadMainMenuWindowFrameTiles(0, 0xF3);
            LoadMessageBoxGfx(0, BIRCH_DLG_BASE_TILE_NUM, 240);
            NewGameBirchSpeech_ShowDialogueWindow(0, 1);
            PutWindowTilemap(0);
            CopyWindowToVram(0, COPYWIN_GFX);
            NewGameBirchSpeech_ClearWindow(0);
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                (*crate::asmdata::gText_Birch_Welcome.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            AddTextPrinterForMessage(TRUE);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_ThisIsAPokemon));
        }
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_ThisIsAPokemon(taskId: u8) {
    if gPaletteFade.active() == 0 && RunTextPrintersAndIsPrinter0Active() == 0 {
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_MainSpeech));
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_ThisIsAPokemon).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterWithCallbackForMessage(
            TRUE,
            Some(NewGameBirchSpeech_WaitForThisIsPokemonText),
        );
        sBirchSpeechMainTaskId.set(taskId);
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_MainSpeech(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*crate::asmdata::gText_Birch_MainSpeech.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_AndYouAre));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeechSub_InitPokeBall(taskId: u8) {
    let spriteId: u8 = task_get(sBirchSpeechMainTaskId.get(), tLotadSpriteId) as u8;
    gSprites[spriteId].x = 100;
    gSprites[spriteId].y = 75;
    gSprites[spriteId].set_invisible(FALSE as u16);
    gSprites[spriteId].data[0] = 0;
    CreatePokeballSpriteToReleaseMon(
        spriteId,
        gSprites[spriteId].oam.paletteNum() as u8,
        112,
        58,
        0,
        0,
        32,
        PALETTES_BG,
        SPECIES_LOTAD,
    );
    task_set_func(taskId, Some(Task_NewGameBirchSpeechSub_WaitForLotad));
    task_set(sBirchSpeechMainTaskId.get(), tTimer, 0);
}
pub(crate) unsafe fn Task_NewGameBirchSpeechSub_WaitForLotad(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let sprite: *mut Sprite =
        &raw mut gSprites[task_get(sBirchSpeechMainTaskId.get(), tLotadSpriteId)];
    match *data {
        0 => {
            if (*sprite).callback != Some(SpriteCallbackDummy as unsafe fn(*mut Sprite)) {
                return;
            }
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
        }
        1 => {
            if task_get(sBirchSpeechMainTaskId.get(), tTimer) >= 96 {
                DestroyTask(taskId);
                if task_get(sBirchSpeechMainTaskId.get(), tTimer) < 0x4000 {
                    task_set(
                        sBirchSpeechMainTaskId.get(),
                        tTimer,
                        task_get(sBirchSpeechMainTaskId.get(), tTimer) + 1,
                    );
                }
            }
            return;
        }
        _ => {}
    }
    *data += 1;
    if task_get(sBirchSpeechMainTaskId.get(), tTimer) < 0x4000 {
        task_set(
            sBirchSpeechMainTaskId.get(),
            tTimer,
            task_get(sBirchSpeechMainTaskId.get(), tTimer) + 1,
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_AndYouAre(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        sStartedPokeBallTask.set(FALSE);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*crate::asmdata::gText_Birch_AndYouAre.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        task_set_func(
            taskId,
            Some(Task_NewGameBirchSpeech_StartBirchLotadPlatformFade),
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_StartBirchLotadPlatformFade(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        gSprites[task_get(taskId, tBirchSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        gSprites[task_get(taskId, tLotadSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2);
        NewGameBirchSpeech_StartFadePlatformIn(taskId, 1);
        task_set(taskId, tTimer, 64);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_SlidePlatformAway));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_SlidePlatformAway(taskId: u8) {
    if task_get(taskId, tBG1HOFS) != -60 {
        task_set(taskId, tBG1HOFS, task_get(taskId, tBG1HOFS) - 2);
        SetGpuReg(REG_OFFSET_BG1HOFS, task_get(taskId, tBG1HOFS) as u16);
    } else {
        task_set(taskId, tBG1HOFS, -60);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_StartPlayerFadeIn));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_StartPlayerFadeIn(taskId: u8) {
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tBirchSpriteId)].set_invisible(TRUE as u16);
        gSprites[task_get(taskId, tLotadSpriteId)].set_invisible(TRUE as u16);
        if task_get(taskId, tTimer) != 0 {
            task_set(taskId, tTimer, task_get(taskId, tTimer) - 1);
        } else {
            let spriteId: u8 = task_get(taskId, tBrendanSpriteId) as u8;
            gSprites[spriteId].x = 180;
            gSprites[spriteId].y = 60;
            gSprites[spriteId].set_invisible(FALSE as u16);
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
            task_set(taskId, tPlayerSpriteId, spriteId as i16);
            task_set(taskId, tPlayerGender, MALE as i16);
            NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2);
            NewGameBirchSpeech_StartFadePlatformOut(taskId, 1);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_WaitForPlayerFadeIn));
        }
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitForPlayerFadeIn(taskId: u8) {
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tPlayerSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_BoyOrGirl));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_BoyOrGirl(taskId: u8) {
    NewGameBirchSpeech_ClearWindow(0);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*crate::asmdata::gText_Birch_BoyOrGirl.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterForMessage(TRUE);
    task_set_func(taskId, Some(Task_NewGameBirchSpeech_WaitToShowGenderMenu));
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitToShowGenderMenu(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        NewGameBirchSpeech_ShowGenderMenu();
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_ChooseGender));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_ChooseGender(taskId: u8) {
    let gender: i32 = NewGameBirchSpeech_ProcessGenderMenuInput() as i32;
    match gender {
        0 => {
            PlaySE(SE_SELECT);
            (*gSaveBlock2Ptr).playerGender = gender as u8;
            NewGameBirchSpeech_ClearGenderWindow(1, 1);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_WhatsYourName));
        }
        1 => {
            PlaySE(SE_SELECT);
            (*gSaveBlock2Ptr).playerGender = gender as u8;
            NewGameBirchSpeech_ClearGenderWindow(1, 1);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_WhatsYourName));
        }
        _ => {}
    }
    let gender2: i32 = Menu_GetCursorPos() as i32;
    if gender2 != task_get(taskId, tPlayerGender) as i32 {
        task_set(taskId, tPlayerGender, gender2 as i16);
        gSprites[task_get(taskId, tPlayerSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 0);
        task_set_func(
            taskId,
            Some(Task_NewGameBirchSpeech_SlideOutOldGenderSprite),
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_SlideOutOldGenderSprite(taskId: u8) {
    let mut spriteId: u8 = task_get(taskId, tPlayerSpriteId) as u8;
    if task_get(taskId, tIsDoneFadingSprites) == 0 {
        gSprites[spriteId].x += 4;
    } else {
        gSprites[spriteId].set_invisible(TRUE as u16);
        if task_get(taskId, tPlayerGender) != MALE as i16 {
            spriteId = task_get(taskId, tMaySpriteId) as u8;
        } else {
            spriteId = task_get(taskId, tBrendanSpriteId) as u8;
        }
        gSprites[spriteId].x = DISPLAY_WIDTH as i16;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        task_set(taskId, tPlayerSpriteId, spriteId as i16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 0);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_SlideInNewGenderSprite));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_SlideInNewGenderSprite(taskId: u8) {
    let spriteId: u8 = task_get(taskId, tPlayerSpriteId) as u8;
    if gSprites[spriteId].x > 180 {
        gSprites[spriteId].x -= 4;
    } else {
        gSprites[spriteId].x = 180;
        if task_get(taskId, tIsDoneFadingSprites) != 0 {
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_ChooseGender));
        }
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WhatsYourName(taskId: u8) {
    NewGameBirchSpeech_ClearWindow(0);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*crate::asmdata::gText_Birch_WhatsYourName.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterForMessage(TRUE);
    task_set_func(
        taskId,
        Some(Task_NewGameBirchSpeech_WaitForWhatsYourNameToPrint),
    );
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitForWhatsYourNameToPrint(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        task_set_func(
            taskId,
            Some(Task_NewGameBirchSpeech_WaitPressBeforeNameChoice),
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitPressBeforeNameChoice(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_StartNamingScreen));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_StartNamingScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        FreeAndDestroyMonPicSprite(task_get(taskId, tLotadSpriteId) as u16);
        NewGameBirchSpeech_SetDefaultPlayerName(rem_u32(
            Random() as u32,
            if 20 < 20 { 20 } else { 20 },
        ) as u8);
        DestroyTask(taskId);
        DoNamingScreen(
            NAMING_SCREEN_PLAYER,
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            (*gSaveBlock2Ptr).playerGender as u16,
            0,
            0,
            Some(CB2_NewGameBirchSpeech_ReturnFromNamingScreen),
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_SoItsPlayerName(taskId: u8) {
    NewGameBirchSpeech_ClearWindow(0);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*crate::asmdata::gText_Birch_SoItsPlayer.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterForMessage(TRUE);
    task_set_func(taskId, Some(Task_NewGameBirchSpeech_CreateNameYesNo));
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_CreateNameYesNo(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        CreateYesNoMenuParameterized(2, 1, 0xF3, 0xDF, 2, 15);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_ProcessNameYesNoMenu));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_ProcessNameYesNoMenu(taskId: u8) {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PlaySE(SE_SELECT);
            gSprites[task_get(taskId, tPlayerSpriteId)]
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
            NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2);
            NewGameBirchSpeech_StartFadePlatformIn(taskId, 1);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_SlidePlatformAway2));
        }
        MENU_B_PRESSED | 1 => {
            PlaySE(SE_SELECT);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_BoyOrGirl));
        }
        _ => {}
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_SlidePlatformAway2(taskId: u8) {
    if task_get(taskId, tBG1HOFS) != 0 {
        task_set(taskId, tBG1HOFS, task_get(taskId, tBG1HOFS) + 2);
        SetGpuReg(REG_OFFSET_BG1HOFS, task_get(taskId, tBG1HOFS) as u16);
    } else {
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_ReshowBirchLotad));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_ReshowBirchLotad(taskId: u8) {
    let mut spriteId: u8 = 0;
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tBrendanSpriteId)].set_invisible(TRUE as u16);
        gSprites[task_get(taskId, tMaySpriteId)].set_invisible(TRUE as u16);
        spriteId = task_get(taskId, tBirchSpriteId) as u8;
        gSprites[spriteId].x = 136;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        spriteId = task_get(taskId, tLotadSpriteId) as u8;
        gSprites[spriteId].x = 100;
        gSprites[spriteId].y = 75;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2);
        NewGameBirchSpeech_StartFadePlatformOut(taskId, 1);
        NewGameBirchSpeech_ClearWindow(0);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*crate::asmdata::gText_Birch_YourePlayer.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        task_set_func(
            taskId,
            Some(Task_NewGameBirchSpeech_WaitForSpriteFadeInAndTextPrinter),
        );
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitForSpriteFadeInAndTextPrinter(taskId: u8) {
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tBirchSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        gSprites[task_get(taskId, tLotadSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        if RunTextPrintersAndIsPrinter0Active() == 0 {
            gSprites[task_get(taskId, tBirchSpriteId)]
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
            gSprites[task_get(taskId, tLotadSpriteId)]
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
            NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2);
            NewGameBirchSpeech_StartFadePlatformIn(taskId, 1);
            task_set(taskId, tTimer, 64);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_AreYouReady));
        }
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_AreYouReady(taskId: u8) {
    let mut spriteId: u8 = 0;
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tBirchSpriteId)].set_invisible(TRUE as u16);
        gSprites[task_get(taskId, tLotadSpriteId)].set_invisible(TRUE as u16);
        if task_get(taskId, tTimer) != 0 {
            task_set(taskId, tTimer, task_get(taskId, tTimer) - 1);
            return;
        }
        if (*gSaveBlock2Ptr).playerGender != MALE {
            spriteId = task_get(taskId, tMaySpriteId) as u8;
        } else {
            spriteId = task_get(taskId, tBrendanSpriteId) as u8;
        }
        gSprites[spriteId].x = 120;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        task_set(taskId, tPlayerSpriteId, spriteId as i16);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2);
        NewGameBirchSpeech_StartFadePlatformOut(taskId, 1);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*crate::asmdata::gText_Birch_AreYouReady.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_ShrinkPlayer));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_ShrinkPlayer(taskId: u8) {
    let mut spriteId: u8 = 0;
    if task_get(taskId, tIsDoneFadingSprites) != 0 {
        gSprites[task_get(taskId, tPlayerSpriteId)]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        if RunTextPrintersAndIsPrinter0Active() == 0 {
            spriteId = task_get(taskId, tPlayerSpriteId) as u8;
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            gSprites[spriteId].affineAnims =
                sSpriteAffineAnimTable_PlayerShrink.as_ptr().cast_mut();
            InitSpriteAffineAnim(&raw mut gSprites[spriteId]);
            StartSpriteAffineAnim(&raw mut gSprites[spriteId], 0);
            gSprites[spriteId].callback = Some(SpriteCB_MovePlayerDownWhileShrinking);
            BeginNormalPaletteFade(PALETTES_BG, 0, 0, 16, 0);
            FadeOutBGM(4);
            task_set_func(taskId, Some(Task_NewGameBirchSpeech_WaitForPlayerShrink));
        }
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_WaitForPlayerShrink(taskId: u8) {
    let spriteId: u8 = task_get(taskId, tPlayerSpriteId) as u8;
    if gSprites[spriteId].affineAnimEnded() != 0 {
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_FadePlayerToWhite));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_FadePlayerToWhite(taskId: u8) {
    let mut spriteId: u8 = 0;
    if gPaletteFade.active() == 0 {
        spriteId = task_get(taskId, tPlayerSpriteId) as u8;
        gSprites[spriteId].callback = Some(SpriteCB_Null);
        SetGpuReg(REG_OFFSET_DISPCNT, 4160);
        BeginNormalPaletteFade(PALETTES_OBJECTS, 0, 0, 16, 65535);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_Cleanup));
    }
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_Cleanup(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        FreeAndDestroyMonPicSprite(task_get(taskId, tLotadSpriteId) as u16);
        ResetAllPicSprites();
        SetMainCallback2(Some(CB2_NewGame));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn CB2_NewGameBirchSpeech_ReturnFromNamingScreen() {
    let mut spriteId: u8 = 0;
    ResetBgsAndClearDma3BusyFlags(0);
    SetGpuReg(0x0, 0);
    SetGpuReg(REG_OFFSET_DISPCNT, 4160);
    InitBgsFromTemplates(0, sMainMenuBgTemplates.as_ptr().cast_mut(), 2);
    InitBgFromTemplate((&raw const *sBirchBgTemplate).cast_mut());
    SetVBlankCallback(None);
    SetGpuReg(REG_OFFSET_BG2CNT, 0);
    SetGpuReg(REG_OFFSET_BG1CNT, 0);
    SetGpuReg(REG_OFFSET_BG0CNT, 0);
    SetGpuReg(REG_OFFSET_BG2HOFS, 0);
    SetGpuReg(REG_OFFSET_BG2VOFS, 0);
    SetGpuReg(REG_OFFSET_BG1HOFS, 0);
    SetGpuReg(REG_OFFSET_BG1VOFS, 0);
    SetGpuReg(REG_OFFSET_BG0HOFS, 0);
    SetGpuReg(REG_OFFSET_BG0VOFS, 0);
    {
        {
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), VRAM as u32);
                    volatile_write(dmaRegs.at(2), 0x8100c000);
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
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
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
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let dmaRegs: *mut u32 = 67109076_usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(dmaRegs.at(1), PLTT);
                    volatile_write(dmaRegs.at(2), 0x81000200);
                    let _ = (dmaRegs.at(2)).read_volatile();
                }
            }
        }
    }
    ResetPaletteFade();
    LZ77UnCompVram(
        sBirchSpeechShadowGfx.as_ptr().cast_mut(),
        VRAM as usize as *mut u8 as *mut c_void,
    );
    LZ77UnCompVram(
        sBirchSpeechBgMap.as_ptr().cast_mut(),
        0x6003800_usize as *mut u8 as *mut c_void,
    );
    LoadPalette(sBirchSpeechBgPals.as_ptr().cast_mut() as *mut c_void, 0, 64);
    LoadPalette(
        (&raw const sBirchSpeechBgGradientPal[1]).cast_mut() as *mut c_void,
        1,
        16,
    );
    ResetTasks();
    let taskId: u8 = CreateTask(
        Some(Task_NewGameBirchSpeech_ReturnFromNamingScreenShowTextbox),
        0,
    );
    task_set(taskId, tTimer, 5);
    task_set(taskId, tBG1HOFS, -60);
    ScanlineEffect_Stop();
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetAllPicSprites();
    AddBirchSpeechObjects(taskId);
    if (*gSaveBlock2Ptr).playerGender != MALE {
        task_set(taskId, tPlayerGender, FEMALE as i16);
        spriteId = task_get(taskId, tMaySpriteId) as u8;
    } else {
        task_set(taskId, tPlayerGender, MALE as i16);
        spriteId = task_get(taskId, tBrendanSpriteId) as u8;
    }
    gSprites[spriteId].x = 180;
    gSprites[spriteId].y = 60;
    gSprites[spriteId].set_invisible(FALSE as u16);
    task_set(taskId, tPlayerSpriteId, spriteId as i16);
    SetGpuReg(REG_OFFSET_BG1HOFS, 65476);
    BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
    SetGpuReg(REG_OFFSET_WIN0H, 0);
    SetGpuReg(REG_OFFSET_WIN0V, 0);
    SetGpuReg(REG_OFFSET_WININ, 0);
    SetGpuReg(REG_OFFSET_WINOUT, 0);
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    ShowBg(0);
    ShowBg(1);
    {
        let imeTemp: u16 = (67109384_usize as *mut u16).read_volatile();
        volatile_write(67109384_usize as *mut u16, 0);
        volatile_write(
            0x4000200_usize as *mut u16,
            (0x4000200_usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
        );
        volatile_write(67109384_usize as *mut u16, imeTemp);
    }
    SetVBlankCallback(Some(VBlankCB_MainMenu));
    SetMainCallback2(Some(CB2_MainMenu));
    InitWindows(sNewGameBirchSpeechTextWindows.as_ptr().cast_mut());
    LoadMainMenuWindowFrameTiles(0, 0xF3);
    LoadMessageBoxGfx(0, BIRCH_DLG_BASE_TILE_NUM, 240);
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub(crate) unsafe fn SpriteCB_Null(sprite: *mut Sprite) {}
pub(crate) unsafe fn SpriteCB_MovePlayerDownWhileShrinking(sprite: *mut Sprite) {
    let mut y: u32 = 0;
    y = (((*sprite).y as u32) << 16) + (*sprite).data[0] as u32 + 0xC000;
    (*sprite).y = (y >> 16) as i16;
    (*sprite).data[0] = y as i16;
}
unsafe fn NewGameBirchSpeech_CreateLotadSprite(x: u8, y: u8) -> u8 {
    CreateMonPicSprite_Affine(
        SPECIES_LOTAD,
        SHINY_ODDS,
        0,
        MON_PIC_AFFINE_FRONT,
        x as i16,
        y as i16,
        14,
        TAG_NONE,
    ) as u8
}
unsafe fn AddBirchSpeechObjects(taskId: u8) {
    let birchSpriteId: u8 = AddNewGameBirchObject(0x88, 0x3C, 1);
    gSprites[birchSpriteId].callback = Some(SpriteCB_Null);
    gSprites[birchSpriteId].oam.set_priority(0);
    gSprites[birchSpriteId].set_invisible(TRUE as u16);
    task_set(taskId, tBirchSpriteId, birchSpriteId as i16);
    let lotadSpriteId: u8 = NewGameBirchSpeech_CreateLotadSprite(100, 0x4B);
    gSprites[lotadSpriteId].callback = Some(SpriteCB_Null);
    gSprites[lotadSpriteId].oam.set_priority(0);
    gSprites[lotadSpriteId].set_invisible(TRUE as u16);
    task_set(taskId, tLotadSpriteId, lotadSpriteId as i16);
    let brendanSpriteId: u8 = CreateTrainerSprite(
        FacilityClassToPicIndex(0x3c) as u8,
        120,
        60,
        0,
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[0],
    );
    gSprites[brendanSpriteId].callback = Some(SpriteCB_Null);
    gSprites[brendanSpriteId].set_invisible(TRUE as u16);
    gSprites[brendanSpriteId].oam.set_priority(0);
    task_set(taskId, tBrendanSpriteId, brendanSpriteId as i16);
    let maySpriteId: u8 = CreateTrainerSprite(
        FacilityClassToPicIndex(FACILITY_CLASS_MAY) as u8,
        120,
        60,
        0,
        &raw mut (*(&raw const crate::decompress::gDecompressionBuffer)
            .cast::<CArray<u8, 16384>>()
            .cast_mut())[2048],
    );
    gSprites[maySpriteId].callback = Some(SpriteCB_Null);
    gSprites[maySpriteId].set_invisible(TRUE as u16);
    gSprites[maySpriteId].oam.set_priority(0);
    task_set(taskId, tMaySpriteId, maySpriteId as i16);
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_FadeOutTarget1InTarget2(taskId: u8) {
    let mut alphaCoeff2: i32 = 0;
    if task_get(taskId, tAlphaCoeff1) == 0 {
        (*gTasks.as_ptr())[task_get(taskId, tMainTask)].data[tIsDoneFadingSprites] = TRUE as i16;
        DestroyTask(taskId);
    } else if task_get(taskId, tDelayTimer) != 0 {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelayTimer) - 1);
    } else {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelay));
        task_set(taskId, tAlphaCoeff1, task_get(taskId, tAlphaCoeff1) - 1);
        task_set(taskId, tAlphaCoeff2, task_get(taskId, tAlphaCoeff2) + 1);
        alphaCoeff2 = (task_get(taskId, tAlphaCoeff2) as i32) << 8;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            task_get(taskId, tAlphaCoeff1) as u16 + alphaCoeff2 as u16,
        );
    }
}
unsafe fn NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId: u8, delay: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 592);
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    task_set(taskId, tIsDoneFadingSprites, 0);
    let taskId2: u8 = CreateTask(Some(Task_NewGameBirchSpeech_FadeOutTarget1InTarget2), 0);
    task_set(taskId2, tMainTask, taskId as i16);
    task_set(taskId2, tAlphaCoeff1, 16);
    task_set(taskId2, tAlphaCoeff2, 0);
    task_set(taskId2, tDelay, delay as i16);
    task_set(taskId2, tDelayTimer, delay as i16);
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_FadeInTarget1OutTarget2(taskId: u8) {
    let mut alphaCoeff2: i32 = 0;
    if task_get(taskId, tAlphaCoeff1) == 16 {
        (*gTasks.as_ptr())[task_get(taskId, tMainTask)].data[tIsDoneFadingSprites] = TRUE as i16;
        DestroyTask(taskId);
    } else if task_get(taskId, tDelayTimer) != 0 {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelayTimer) - 1);
    } else {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelay));
        task_set(taskId, tAlphaCoeff1, task_get(taskId, tAlphaCoeff1) + 1);
        task_set(taskId, tAlphaCoeff2, task_get(taskId, tAlphaCoeff2) - 1);
        alphaCoeff2 = (task_get(taskId, tAlphaCoeff2) as i32) << 8;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            task_get(taskId, tAlphaCoeff1) as u16 + alphaCoeff2 as u16,
        );
    }
}
unsafe fn NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId: u8, delay: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 592);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    task_set(taskId, tIsDoneFadingSprites, 0);
    let taskId2: u8 = CreateTask(Some(Task_NewGameBirchSpeech_FadeInTarget1OutTarget2), 0);
    task_set(taskId2, tMainTask, taskId as i16);
    task_set(taskId2, tAlphaCoeff1, 0);
    task_set(taskId2, tAlphaCoeff2, 16);
    task_set(taskId2, tDelay, delay as i16);
    task_set(taskId2, tDelayTimer, delay as i16);
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_FadePlatformIn(taskId: u8) {
    if task_get(taskId, tDelayBefore) != 0 {
        task_set(taskId, tDelayBefore, task_get(taskId, tDelayBefore) - 1);
    } else if task_get(taskId, tPalIndex) == 8 {
        DestroyTask(taskId);
    } else if task_get(taskId, tDelayTimer) != 0 {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelayTimer) - 1);
    } else {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelay));
        task_set(taskId, tPalIndex, task_get(taskId, tPalIndex) + 1);
        LoadPalette(
            (&raw const sBirchSpeechBgGradientPal[task_get(taskId, tPalIndex)]).cast_mut()
                as *mut c_void,
            1,
            16,
        );
    }
}
unsafe fn NewGameBirchSpeech_StartFadePlatformIn(taskId: u8, delay: u8) {
    let taskId2: u8 = CreateTask(Some(Task_NewGameBirchSpeech_FadePlatformIn), 0);
    task_set(taskId2, tMainTask, taskId as i16);
    task_set(taskId2, tPalIndex, 0);
    task_set(taskId2, tDelayBefore, 8);
    task_set(taskId2, tDelay, delay as i16);
    task_set(taskId2, tDelayTimer, delay as i16);
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_FadePlatformOut(taskId: u8) {
    if task_get(taskId, tDelayBefore) != 0 {
        task_set(taskId, tDelayBefore, task_get(taskId, tDelayBefore) - 1);
    } else if task_get(taskId, tPalIndex) == 0 {
        DestroyTask(taskId);
    } else if task_get(taskId, tDelayTimer) != 0 {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelayTimer) - 1);
    } else {
        task_set(taskId, tDelayTimer, task_get(taskId, tDelay));
        task_set(taskId, tPalIndex, task_get(taskId, tPalIndex) - 1);
        LoadPalette(
            (&raw const sBirchSpeechBgGradientPal[task_get(taskId, tPalIndex)]).cast_mut()
                as *mut c_void,
            1,
            16,
        );
    }
}
unsafe fn NewGameBirchSpeech_StartFadePlatformOut(taskId: u8, delay: u8) {
    let taskId2: u8 = CreateTask(Some(Task_NewGameBirchSpeech_FadePlatformOut), 0);
    task_set(taskId2, tMainTask, taskId as i16);
    task_set(taskId2, tPalIndex, 8);
    task_set(taskId2, tDelayBefore, 8);
    task_set(taskId2, tDelay, delay as i16);
    task_set(taskId2, tDelayTimer, delay as i16);
}
unsafe fn NewGameBirchSpeech_ShowGenderMenu() {
    DrawMainMenuWindowBorder(
        (&raw const sNewGameBirchSpeechTextWindows[1]).cast_mut(),
        0xF3,
    );
    FillWindowPixelBuffer(1, 17);
    PrintMenuTable(1, 2, sMenuActions_Gender.as_ptr().cast_mut());
    InitMenuInUpperLeftCornerNormal(1, 2, 0);
    PutWindowTilemap(1);
    CopyWindowToVram(1, COPYWIN_FULL);
}
unsafe fn NewGameBirchSpeech_ProcessGenderMenuInput() -> i8 {
    Menu_ProcessInputNoWrap()
}
unsafe fn NewGameBirchSpeech_SetDefaultPlayerName(nameId: u8) {
    let mut name: *mut u8 = null_mut();
    if (*gSaveBlock2Ptr).playerGender == MALE {
        name = sMalePresetNames[nameId];
    } else {
        name = sFemalePresetNames[nameId];
    }
    for i in 0..(PLAYER_NAME_LENGTH as u8) {
        (*gSaveBlock2Ptr).playerName[i] = *name.at(i);
    }
    (*gSaveBlock2Ptr).playerName[7] = EOS;
}
unsafe fn CreateMainMenuErrorWindow(str: *mut u8) {
    FillWindowPixelBuffer(7, 17);
    AddTextPrinterParameterized(7, FONT_NORMAL, str, 0, 1, 2, None);
    PutWindowTilemap(7);
    CopyWindowToVram(7, COPYWIN_GFX);
    DrawMainMenuWindowBorder(
        (&raw const sWindowTemplates_MainMenu[7]).cast_mut(),
        MAIN_MENU_BORDER_TILE,
    );
    SetGpuReg(REG_OFFSET_WIN0H, 2535);
    SetGpuReg(REG_OFFSET_WIN0V, 29087);
}
unsafe fn MainMenu_FormatSavegameText() {
    MainMenu_FormatSavegamePlayer();
    MainMenu_FormatSavegamePokedex();
    MainMenu_FormatSavegameTime();
    MainMenu_FormatSavegameBadges();
}
unsafe fn MainMenu_FormatSavegamePlayer() {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ContinueMenuPlayer).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        2,
        FONT_NORMAL,
        0,
        17,
        sTextColor_MenuInfo.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
    AddTextPrinterParameterized3(
        2,
        FONT_NORMAL,
        GetStringRightAlignXOffset(
            FONT_NORMAL as i32,
            (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
            100,
        ) as u8,
        17,
        sTextColor_MenuInfo.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
    );
}
unsafe fn MainMenu_FormatSavegameTime() {
    let mut str: CArray<u8, 32> = zeroed();
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ContinueMenuTime).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        2,
        FONT_NORMAL,
        0x6C,
        17,
        sTextColor_MenuInfo.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
    let mut ptr: *mut u8 = ConvertIntToDecimalStringN(
        str.as_mut_ptr(),
        (*gSaveBlock2Ptr).playTimeHours as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        3,
    );
    *({
        let t1 = ptr;
        ptr = ptr.at(1);
        t1
    }) = CHAR_COLON;
    ConvertIntToDecimalStringN(
        ptr,
        (*gSaveBlock2Ptr).playTimeMinutes as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        2,
    );
    AddTextPrinterParameterized3(
        2,
        FONT_NORMAL,
        GetStringRightAlignXOffset(FONT_NORMAL as i32, str.as_mut_ptr(), 0xD0) as u8,
        17,
        sTextColor_MenuInfo.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        str.as_mut_ptr(),
    );
}
unsafe fn MainMenu_FormatSavegamePokedex() {
    let mut str: CArray<u8, 32> = zeroed();
    let mut dexCount: u16 = 0;
    if FlagGet(FLAG_SYS_POKEDEX_GET) == TRUE {
        if IsNationalPokedexEnabled() != 0 {
            dexCount = GetNationalPokedexCount(FLAG_GET_CAUGHT);
        } else {
            dexCount = GetHoennPokedexCount(FLAG_GET_CAUGHT);
        }
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_ContinueMenuPokedex).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        AddTextPrinterParameterized3(
            2,
            FONT_NORMAL,
            0,
            33,
            sTextColor_MenuInfo.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            gStringVar4.as_mut_ptr(),
        );
        ConvertIntToDecimalStringN(
            str.as_mut_ptr(),
            dexCount as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            3,
        );
        AddTextPrinterParameterized3(
            2,
            FONT_NORMAL,
            GetStringRightAlignXOffset(FONT_NORMAL as i32, str.as_mut_ptr(), 100) as u8,
            33,
            sTextColor_MenuInfo.as_ptr().cast_mut(),
            TEXT_SKIP_DRAW as i8,
            str.as_mut_ptr(),
        );
    }
}
unsafe fn MainMenu_FormatSavegameBadges() {
    let mut str: CArray<u8, 32> = zeroed();
    let mut badgeCount: u8 = 0;
    for i in FLAG_BADGE01_GET..2159 {
        if FlagGet(i as u16) != 0 {
            badgeCount += 1;
        }
    }
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_ContinueMenuBadges).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    AddTextPrinterParameterized3(
        2,
        FONT_NORMAL,
        0x6C,
        33,
        sTextColor_MenuInfo.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        gStringVar4.as_mut_ptr(),
    );
    ConvertIntToDecimalStringN(
        str.as_mut_ptr(),
        badgeCount as i32,
        STR_CONV_MODE_LEADING_ZEROS,
        1,
    );
    AddTextPrinterParameterized3(
        2,
        FONT_NORMAL,
        GetStringRightAlignXOffset(FONT_NORMAL as i32, str.as_mut_ptr(), 0xD0) as u8,
        33,
        sTextColor_MenuInfo.as_ptr().cast_mut(),
        TEXT_SKIP_DRAW as i8,
        str.as_mut_ptr(),
    );
}
unsafe fn LoadMainMenuWindowFrameTiles(bgId: u8, tileOffset: u16) {
    LoadBgTiles(
        bgId,
        (*GetWindowFrameTilesPal((*gSaveBlock2Ptr).optionsWindowFrameType() as u8)).tiles
            as *mut c_void,
        0x120,
        tileOffset,
    );
    LoadPalette(
        (*GetWindowFrameTilesPal((*gSaveBlock2Ptr).optionsWindowFrameType() as u8)).pal
            as *mut c_void,
        32,
        32,
    );
}
unsafe fn DrawMainMenuWindowBorder(template: *mut WindowTemplate, baseTileNum: u16) {
    let r9: u16 = 1 + baseTileNum;
    let r10: u16 = 2 + baseTileNum;
    let sp18: u16 = 3 + baseTileNum;
    let spC: u16 = 5 + baseTileNum;
    let sp10: u16 = 6 + baseTileNum;
    let sp14: u16 = 7 + baseTileNum;
    let r6: u16 = 8 + baseTileNum;
    FillBgTilemapBufferRect(
        (*template).bg,
        baseTileNum,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop - 1,
        1,
        1,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        r9,
        (*template).tilemapLeft,
        (*template).tilemapTop - 1,
        (*template).width,
        1,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        r10,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop - 1,
        1,
        1,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        sp18,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop,
        1,
        (*template).height,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        spC,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop,
        1,
        (*template).height,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        sp10,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop + (*template).height,
        1,
        1,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        sp14,
        (*template).tilemapLeft,
        (*template).tilemapTop + (*template).height,
        (*template).width,
        1,
        2,
    );
    FillBgTilemapBufferRect(
        (*template).bg,
        r6,
        (*template).tilemapLeft + (*template).width,
        (*template).tilemapTop + (*template).height,
        1,
        1,
        2,
    );
    CopyBgTilemapBufferToVram((*template).bg);
}
unsafe fn ClearMainMenuWindowTilemap(template: *mut WindowTemplate) {
    FillBgTilemapBufferRect(
        (*template).bg,
        0,
        (*template).tilemapLeft - 1,
        (*template).tilemapTop - 1,
        (*template).tilemapLeft + (*template).width + 1,
        (*template).tilemapTop + (*template).height + 1,
        2,
    );
    CopyBgTilemapBufferToVram((*template).bg);
}
pub(crate) unsafe fn NewGameBirchSpeech_ClearGenderWindowTilemap(
    bg: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    unused: u8,
) {
    FillBgTilemapBufferRect(bg, 0, x + 255, y + 255, width + 2, height + 2, 2);
}
unsafe fn NewGameBirchSpeech_ClearGenderWindow(windowId: u8, copyToVram: u8) {
    CallWindowFunction(windowId, Some(NewGameBirchSpeech_ClearGenderWindowTilemap));
    FillWindowPixelBuffer(windowId, 17);
    ClearWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
unsafe fn NewGameBirchSpeech_ClearWindow(windowId: u8) {
    let bgColor: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_BACKGROUND);
    let maxCharWidth: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_MAX_LETTER_WIDTH);
    let maxCharHeight: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_MAX_LETTER_HEIGHT);
    let winWidth: u8 = GetWindowAttribute(windowId, WINDOW_WIDTH) as u8;
    let winHeight: u8 = GetWindowAttribute(windowId, WINDOW_HEIGHT) as u8;
    FillWindowPixelRect(
        windowId,
        bgColor,
        0,
        0,
        maxCharWidth as u16 * winWidth as u16,
        maxCharHeight as u16 * winHeight as u16,
    );
    CopyWindowToVram(windowId, COPYWIN_GFX);
}
pub(crate) unsafe fn NewGameBirchSpeech_WaitForThisIsPokemonText(
    printer: *mut TextPrinterTemplate,
    renderCmd: u16,
) {
    if *(*printer).currentChar.at(-2) == EXT_CTRL_CODE_PAUSE && sStartedPokeBallTask.get() == 0 {
        sStartedPokeBallTask.set(TRUE);
        CreateTask(Some(Task_NewGameBirchSpeechSub_InitPokeBall), 0);
    }
}
pub unsafe fn CreateYesNoMenuParameterized(
    x: u8,
    y: u8,
    baseTileNum: u16,
    baseBlock: u16,
    yesNoPalNum: u8,
    winPalNum: u8,
) {
    let mut template: WindowTemplate =
        CreateWindowTemplate(0, x + 1, y + 1, 5, 4, winPalNum, baseBlock);
    CreateYesNoMenu(&raw mut template, baseTileNum, yesNoPalNum, 0);
}
unsafe fn NewGameBirchSpeech_ShowDialogueWindow(windowId: u8, copyToVram: u8) {
    CallWindowFunction(
        windowId,
        Some(NewGameBirchSpeech_CreateDialogueWindowBorder),
    );
    FillWindowPixelBuffer(windowId, 17);
    PutWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe fn NewGameBirchSpeech_CreateDialogueWindowBorder(
    bg: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    palNum: u8,
) {
    FillBgTilemapBufferRect(bg, 253, x - 2, y - 1, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 255, x - 1, y - 1, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 256, x, y - 1, width, 1, palNum);
    FillBgTilemapBufferRect(bg, 257, x + width - 1, y - 1, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 258, x + width, y - 1, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 259, x - 2, y, 1, 5, palNum);
    FillBgTilemapBufferRect(bg, 261, x - 1, y, width + 1, 5, palNum);
    FillBgTilemapBufferRect(bg, 262, x + width, y, 1, 5, palNum);
    FillBgTilemapBufferRect(bg, 2301, x - 2, y + height, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 2303, x - 1, y + height, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 2304, x, y + height, width - 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 2305, x + width - 1, y + height, 1, 1, palNum);
    FillBgTilemapBufferRect(bg, 2306, x + width, y + height, 1, 1, palNum);
}
pub(crate) unsafe fn Task_NewGameBirchSpeech_ReturnFromNamingScreenShowTextbox(taskId: u8) {
    if ({
        let t1 = task_get(taskId, tTimer);
        task_set(taskId, tTimer, task_get(taskId, tTimer) - 1);
        t1
    }) <= 0
    {
        NewGameBirchSpeech_ShowDialogueWindow(0, 1);
        task_set_func(taskId, Some(Task_NewGameBirchSpeech_SoItsPlayerName));
    }
}
