//! Translated from `src/main_menu.c` by tools/rustport/c2rs.py.
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
pub(crate) static mut sStartedPokeBallTask: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrItemAndOptionMenuCheck: u16 = 0;
pub(crate) static mut sBirchSpeechMainTaskId: u8 = 0;

unsafe extern "C" {
    static mut gDecompressionBuffer: CArray<u8, 16384>;
    static gJPText_No1MSubCircuit: CArray<u8, 0>;
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSaveFileStatus: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BatteryRunDry: CArray<u8, 0>;
    static gText_Birch_AndYouAre: CArray<u8, 0>;
    static gText_Birch_AreYouReady: CArray<u8, 0>;
    static gText_Birch_BoyOrGirl: CArray<u8, 0>;
    static gText_Birch_MainSpeech: CArray<u8, 0>;
    static gText_Birch_SoItsPlayer: CArray<u8, 0>;
    static gText_Birch_Welcome: CArray<u8, 0>;
    static gText_Birch_WhatsYourName: CArray<u8, 0>;
    static gText_Birch_YourePlayer: CArray<u8, 0>;
    static gText_ContinueMenuBadges: CArray<u8, 0>;
    static gText_ContinueMenuPlayer: CArray<u8, 0>;
    static gText_ContinueMenuPokedex: CArray<u8, 0>;
    static gText_ContinueMenuTime: CArray<u8, 0>;
    static gText_MainMenuContinue: CArray<u8, 0>;
    static gText_MainMenuMysteryEvents: CArray<u8, 0>;
    static gText_MainMenuMysteryGift: CArray<u8, 0>;
    static gText_MainMenuMysteryGift2: CArray<u8, 0>;
    static gText_MainMenuNewGame: CArray<u8, 0>;
    static gText_MainMenuOption: CArray<u8, 0>;
    static gText_MysteryEventsCantUse: CArray<u8, 0>;
    static gText_MysteryGiftCantUse: CArray<u8, 0>;
    static gText_SaveFileCorrupted: CArray<u8, 0>;
    static gText_SaveFileErased: CArray<u8, 0>;
    static gText_ThisIsAPokemon: CArray<u8, 0>;
    static gText_WirelessNotConnected: CArray<u8, 0>;
    fn AddNewGameBirchObject(a0: i16, a1: i16, a2: u8) -> u8;
    fn AddScrollIndicatorArrowPair(a0: *mut ScrollArrowsTemplate, a1: *mut u16) -> u8;
    fn AddTextPrinterForMessage(a0: u8);
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized3(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: *mut u8,
        a5: i8,
        a6: *mut u8,
    );
    fn AddTextPrinterWithCallbackForMessage(
        a0: u8,
        a1: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    );
    fn AnimateSprites();
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BuildOamBuffer();
    fn CB2_ContinueSavedGame();
    fn CB2_InitEReader();
    fn CB2_InitMysteryEventMenu();
    fn CB2_InitMysteryGift();
    fn CB2_InitOptionMenu();
    fn CB2_InitTitleScreen();
    fn CB2_NewGame();
    fn CallWindowFunction(a0: u8, a1: Option<unsafe extern "C" fn(u8, u8, u8, u8, u8, u8)>);
    fn ChangeBgX(a0: u8, a1: i32, a2: u8) -> i32;
    fn ChangeBgY(a0: u8, a1: i32, a2: u8) -> i32;
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyBgTilemapBufferToVram(a0: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMonPicSprite_Affine(
        a0: u16,
        a1: u32,
        a2: u32,
        a3: u8,
        a4: i16,
        a5: i16,
        a6: u8,
        a7: u16,
    ) -> u16;
    fn CreatePokeballSpriteToReleaseMon(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u32,
        a8: u16,
    );
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateTrainerSprite(a0: u8, a1: i16, a2: i16, a3: u8, a4: *mut u8) -> u8;
    fn CreateWindowTemplate(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u16,
    ) -> WindowTemplate;
    fn CreateYesNoMenu(a0: *mut WindowTemplate, a1: u16, a2: u8, a3: u8);
    fn DeactivateAllTextPrinters();
    fn DestroyTask(a0: u8);
    fn DoNamingScreen(
        a0: u8,
        a1: *mut u8,
        a2: u16,
        a3: u16,
        a4: u32,
        a5: Option<unsafe extern "C" fn()>,
    );
    fn EnableInterrupts(a0: u16);
    fn FacilityClassToPicIndex(a0: u16) -> u16;
    fn FadeOutBGM(a0: u8);
    fn FillBgTilemapBufferRect(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8);
    fn FillBgTilemapBufferRect_Palette0(a0: u8, a1: u16, a2: u8, a3: u8, a4: u8, a5: u8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FlagGet(a0: u16) -> u8;
    fn FreeAllSpritePalettes();
    fn FreeAllWindowBuffers();
    fn FreeAndDestroyMonPicSprite(a0: u16) -> u16;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetHoennPokedexCount(a0: u8) -> u16;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetWindowAttribute(a0: u8, a1: u8) -> u32;
    fn GetWindowFrameTilesPal(a0: u8) -> *mut TilesPal;
    fn HideBg(a0: u8);
    fn InitBgFromTemplate(a0: *mut BgTemplate);
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitSpriteAffineAnim(a0: *mut Sprite);
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
    fn IsMysteryGiftEnabled() -> u32;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn IsWirelessAdapterConnected() -> u8;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut c_void);
    fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayBGM(a0: u16);
    fn PlaySE(a0: u16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
    fn ProcessSpriteCopyRequests();
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn ResetAllPicSprites() -> u16;
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn RtcGetErrorStatus() -> u16;
    fn RunTasks();
    fn RunTextPrinters();
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn ScanlineEffect_Stop();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_ScrollIndicatorArrowPairOnMainMenu(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn CB2_MainMenu() {
    RunTasks();
    AnimateSprites();
    BuildOamBuffer();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn VBlankCB_MainMenu() {
    LoadOam();
    ProcessSpriteCopyRequests();
    TransferPlttBuffer();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitMainMenu() {
    InitMainMenu(FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReinitMainMenu() {
    InitMainMenu(TRUE);
}
pub(crate) unsafe extern "C" fn InitMainMenu(returningFromOptionsMenu: u8) -> u32 {
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                    volatile_write(
                        dmaRegs.at(1),
                        83886082 as usize as *mut c_void as usize as u32,
                    );
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
    return 0;
}
pub(crate) unsafe extern "C" fn Task_MainMenuCheckSaveFile(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
                gTasks[taskId].func = Some(Task_MainMenuCheckBattery);
            }
            SAVE_STATUS_CORRUPT => {
                CreateMainMenuErrorWindow(gText_SaveFileErased.as_ptr().cast_mut());
                *data = HAS_NO_SAVED_GAME;
                gTasks[taskId].func = Some(Task_WaitForSaveFileErrorWindow);
            }
            255 => {
                CreateMainMenuErrorWindow(gText_SaveFileCorrupted.as_ptr().cast_mut());
                gTasks[taskId].func = Some(Task_WaitForSaveFileErrorWindow);
                *data = HAS_SAVED_GAME;
                if IsMysteryGiftEnabled() == TRUE as u32 {
                    *data += 1;
                }
            }
            SAVE_STATUS_NO_FLASH => {
                CreateMainMenuErrorWindow(gJPText_No1MSubCircuit.as_ptr().cast_mut());
                gTasks[taskId].data[0] = HAS_NO_SAVED_GAME;
                gTasks[taskId].func = Some(Task_WaitForSaveFileErrorWindow);
            }
            _ => {
                *data = HAS_NO_SAVED_GAME;
                gTasks[taskId].func = Some(Task_MainMenuCheckBattery);
            }
        }
        if sCurrItemAndOptionMenuCheck as i32 & OPTION_MENU_FLAG != 0 {
            match *data {
                HAS_NO_SAVED_GAME | HAS_SAVED_GAME => {
                    sCurrItemAndOptionMenuCheck = *data as u16 + 1;
                }
                HAS_MYSTERY_GIFT => {
                    sCurrItemAndOptionMenuCheck = 3;
                }
                HAS_MYSTERY_EVENTS => {
                    sCurrItemAndOptionMenuCheck = 4;
                }
                _ => {}
            }
        }
        sCurrItemAndOptionMenuCheck &= 32767;
        *data.at(1) = sCurrItemAndOptionMenuCheck as i16;
        *data.at(12) = *data + 2;
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForSaveFileErrorWindow(taskId: u8) {
    RunTextPrinters();
    if IsTextPrinterActive(7) == 0 && gMain.newKeys as i32 & A_BUTTON != 0 {
        ClearWindowTilemap(7);
        ClearMainMenuWindowTilemap((&raw const sWindowTemplates_MainMenu[7]).cast_mut());
        gTasks[taskId].func = Some(Task_MainMenuCheckBattery);
    }
}
pub(crate) unsafe extern "C" fn Task_MainMenuCheckBattery(taskId: u8) {
    if gPaletteFade.active() == 0 {
        SetGpuReg(REG_OFFSET_WIN0H, 0);
        SetGpuReg(REG_OFFSET_WIN0V, 0);
        SetGpuReg(REG_OFFSET_WININ, 17);
        SetGpuReg(REG_OFFSET_WINOUT, 49);
        SetGpuReg(REG_OFFSET_BLDCNT, 193);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        SetGpuReg(REG_OFFSET_BLDY, 7);
        if RtcGetErrorStatus() as i32 & RTC_ERR_FLAG_MASK == 0 {
            gTasks[taskId].func = Some(Task_DisplayMainMenu);
        } else {
            CreateMainMenuErrorWindow(gText_BatteryRunDry.as_ptr().cast_mut());
            gTasks[taskId].func = Some(Task_WaitForBatteryDryErrorWindow);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForBatteryDryErrorWindow(taskId: u8) {
    RunTextPrinters();
    if IsTextPrinterActive(7) == 0 && gMain.newKeys as i32 & A_BUTTON != 0 {
        ClearWindowTilemap(7);
        ClearMainMenuWindowTilemap((&raw const sWindowTemplates_MainMenu[7]).cast_mut());
        gTasks[taskId].func = Some(Task_DisplayMainMenu);
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayMainMenu(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
        match gTasks[taskId].data[0] {
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
                    gText_MainMenuContinue.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    3,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuNewGame.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    4,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuOption.as_ptr().cast_mut(),
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
                    gText_MainMenuContinue.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    3,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuNewGame.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    4,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuMysteryGift.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    5,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuOption.as_ptr().cast_mut(),
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
                    gText_MainMenuContinue.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    3,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuNewGame.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    4,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuMysteryGift2.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    5,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuMysteryEvents.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    6,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuOption.as_ptr().cast_mut(),
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
                    &raw mut sCurrItemAndOptionMenuCheck,
                ) as i16;
                gTasks[*data.at(13)].func = Some(Task_ScrollIndicatorArrowPairOnMainMenu);
                if sCurrItemAndOptionMenuCheck == 4 {
                    ChangeBgY(0, 0x2000, BG_COORD_ADD);
                    ChangeBgY(1, 0x2000, BG_COORD_ADD);
                    *data.at(14) = TRUE as i16;
                    gTasks[*data.at(13)].data[15] = TRUE as i16;
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
                    gText_MainMenuNewGame.as_ptr().cast_mut(),
                );
                AddTextPrinterParameterized3(
                    1,
                    FONT_NORMAL,
                    0,
                    1,
                    sTextColor_Headers.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW as i8,
                    gText_MainMenuOption.as_ptr().cast_mut(),
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
        gTasks[taskId].func = Some(Task_HighlightSelectedMainMenuItem);
    }
}
pub(crate) unsafe extern "C" fn Task_HighlightSelectedMainMenuItem(taskId: u8) {
    HighlightSelectedMainMenuItem(
        gTasks[taskId].data[0] as u8,
        gTasks[taskId].data[1] as u8,
        gTasks[taskId].data[14],
    );
    gTasks[taskId].func = Some(Task_HandleMainMenuInput);
}
pub(crate) unsafe extern "C" fn HandleMainMenuInput(taskId: u8) -> u8 {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        IsWirelessAdapterConnected();
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 0);
        gTasks[taskId].func = Some(Task_HandleMainMenuAPressed);
    } else if gMain.newKeys as i32 & B_BUTTON != 0 {
        PlaySE(SE_SELECT);
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 0x10, 65535);
        SetGpuReg(REG_OFFSET_WIN0H, DISPLAY_WIDTH);
        SetGpuReg(REG_OFFSET_WIN0V, DISPLAY_HEIGHT);
        gTasks[taskId].func = Some(Task_HandleMainMenuBPressed);
    } else if gMain.newKeys as i32 & DPAD_UP != 0 && *data.at(1) > 0 {
        if *data == HAS_MYSTERY_EVENTS && *data.at(14) == 1 && *data.at(1) == 1 {
            ChangeBgY(0, 0x2000, BG_COORD_SUB);
            ChangeBgY(1, 0x2000, BG_COORD_SUB);
            gTasks[*data.at(13)].data[15] = {
                *data.at(14) = FALSE as i16;
                *data.at(14)
            };
        }
        *data.at(1) -= 1;
        sCurrItemAndOptionMenuCheck = *data.at(1) as u16;
        return TRUE;
    } else if gMain.newKeys as i32 & DPAD_DOWN != 0
        && (*data.at(1) as i32) < *data.at(12) as i32 - 1
    {
        if *data == HAS_MYSTERY_EVENTS && *data.at(1) == 3 && *data.at(14) == FALSE as i16 {
            ChangeBgY(0, 0x2000, BG_COORD_ADD);
            ChangeBgY(1, 0x2000, BG_COORD_ADD);
            gTasks[*data.at(13)].data[15] = {
                *data.at(14) = TRUE as i16;
                *data.at(14)
            };
        }
        *data.at(1) += 1;
        sCurrItemAndOptionMenuCheck = *data.at(1) as u16;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_HandleMainMenuInput(taskId: u8) {
    if HandleMainMenuInput(taskId) != 0 {
        gTasks[taskId].func = Some(Task_HighlightSelectedMainMenuItem);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMainMenuAPressed(taskId: u8) {
    let mut wirelessAdapterConnected: u8 = 0;
    let mut action: u8 = 0;
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[0] == HAS_MYSTERY_EVENTS {
            RemoveScrollIndicatorArrowPair(gTasks[taskId].data[13] as u8);
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
        match gTasks[taskId].data[0] {
            HAS_SAVED_GAME => match gTasks[taskId].data[1] {
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
            HAS_MYSTERY_GIFT => match gTasks[taskId].data[1] {
                1 => {
                    action = ACTION_NEW_GAME;
                }
                2 => {
                    action = ACTION_MYSTERY_GIFT;
                    if wirelessAdapterConnected == 0 {
                        action = ACTION_INVALID;
                        gTasks[taskId].data[0] = HAS_NO_SAVED_GAME;
                    }
                }
                3 => {
                    action = ACTION_OPTION;
                }
                _ => {
                    action = ACTION_CONTINUE;
                }
            },
            HAS_MYSTERY_EVENTS => match gTasks[taskId].data[1] {
                1 => {
                    action = ACTION_NEW_GAME;
                }
                2 => {
                    if gTasks[taskId].data[15] != 0 {
                        action = ACTION_MYSTERY_GIFT;
                        if wirelessAdapterConnected == 0 {
                            action = ACTION_INVALID;
                            gTasks[taskId].data[0] = HAS_NO_SAVED_GAME;
                        }
                    } else if wirelessAdapterConnected != 0 {
                        action = ACTION_INVALID;
                        gTasks[taskId].data[0] = HAS_SAVED_GAME;
                    } else {
                        action = ACTION_EREADER;
                    }
                }
                3 => {
                    if wirelessAdapterConnected != 0 {
                        action = ACTION_INVALID;
                        gTasks[taskId].data[0] = HAS_MYSTERY_GIFT;
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
            _ => match gTasks[taskId].data[1] {
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
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].func = Some(Task_DisplayMainMenuInvalidActionError);
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
                gTasks[taskId].func = Some(Task_NewGameBirchSpeech_Init);
            }
        }
        FreeAllWindowBuffers();
        if action != ACTION_OPTION {
            sCurrItemAndOptionMenuCheck = 0;
        } else {
            sCurrItemAndOptionMenuCheck |= OPTION_MENU_FLAG as u16;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMainMenuBPressed(taskId: u8) {
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[0] == HAS_MYSTERY_EVENTS {
            RemoveScrollIndicatorArrowPair(gTasks[taskId].data[13] as u8);
        }
        sCurrItemAndOptionMenuCheck = 0;
        FreeAllWindowBuffers();
        SetMainCallback2(Some(CB2_InitTitleScreen));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayMainMenuInvalidActionError(taskId: u8) {
    match gTasks[taskId].data[1] {
        0 => {
            FillBgTilemapBufferRect_Palette0(0, 0, 0, 0, DISPLAY_TILE_WIDTH, DISPLAY_TILE_HEIGHT);
            match gTasks[taskId].data[0] {
                0 => {
                    CreateMainMenuErrorWindow(gText_WirelessNotConnected.as_ptr().cast_mut());
                }
                1 => {
                    CreateMainMenuErrorWindow(gText_MysteryGiftCantUse.as_ptr().cast_mut());
                }
                2 => {
                    CreateMainMenuErrorWindow(gText_MysteryEventsCantUse.as_ptr().cast_mut());
                }
                _ => {}
            }
            gTasks[taskId].data[1] += 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[1] += 1;
            }
        }
        2 => {
            RunTextPrinters();
            if IsTextPrinterActive(7) == 0 {
                gTasks[taskId].data[1] += 1;
            }
        }
        3 => {
            if gMain.newKeys as i32 & 3 != 0 {
                PlaySE(SE_SELECT);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                gTasks[taskId].func = Some(Task_HandleMainMenuBPressed);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn HighlightSelectedMainMenuItem(
    menuType: u8,
    selectedMenuItem: u8,
    isScrolled: i16,
) {
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
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_Init(taskId: u8) {
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
        0x6003800 as usize as *mut c_void,
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
    gTasks[taskId].data[4] = 0;
    gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitToShowBirch);
    gTasks[taskId].data[2] = SPRITE_NONE as i16;
    gTasks[taskId].data[3] = 0xFF;
    gTasks[taskId].data[7] = 0xD8;
    PlayBGM(MUS_ROUTE122);
    ShowBg(0);
    ShowBg(1);
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitToShowBirch(taskId: u8) {
    let mut spriteId: u8 = 0;
    if gTasks[taskId].data[7] != 0 {
        gTasks[taskId].data[7] -= 1;
    } else {
        spriteId = gTasks[taskId].data[8] as u8;
        gSprites[spriteId].x = 136;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 10);
        NewGameBirchSpeech_StartFadePlatformOut(taskId, 20);
        gTasks[taskId].data[7] = 80;
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitForSpriteFadeInWelcome);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForSpriteFadeInWelcome(taskId: u8) {
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[8]]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        if gTasks[taskId].data[7] != 0 {
            gTasks[taskId].data[7] -= 1;
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
                gText_Birch_Welcome.as_ptr().cast_mut(),
            );
            AddTextPrinterForMessage(TRUE);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_ThisIsAPokemon);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ThisIsAPokemon(taskId: u8) {
    if gPaletteFade.active() == 0 && RunTextPrintersAndIsPrinter0Active() == 0 {
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_MainSpeech);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_ThisIsAPokemon.as_ptr().cast_mut(),
        );
        AddTextPrinterWithCallbackForMessage(
            TRUE,
            Some(NewGameBirchSpeech_WaitForThisIsPokemonText),
        );
        sBirchSpeechMainTaskId = taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_MainSpeech(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_Birch_MainSpeech.as_ptr().cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_AndYouAre);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeechSub_InitPokeBall(taskId: u8) {
    let mut spriteId: u8 = gTasks[sBirchSpeechMainTaskId].data[9] as u8;
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
    gTasks[taskId].func = Some(Task_NewGameBirchSpeechSub_WaitForLotad);
    gTasks[sBirchSpeechMainTaskId].data[7] = 0;
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeechSub_WaitForLotad(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut sprite: *mut Sprite = &raw mut gSprites[gTasks[sBirchSpeechMainTaskId].data[9]];
    match *data {
        0 => {
            if (*sprite).callback != Some(SpriteCallbackDummy as unsafe extern "C" fn(*mut Sprite))
            {
                return;
            }
            (*sprite).oam.set_affineMode(ST_OAM_AFFINE_OFF);
        }
        1 => {
            if gTasks[sBirchSpeechMainTaskId].data[7] >= 96 {
                DestroyTask(taskId);
                if gTasks[sBirchSpeechMainTaskId].data[7] < 0x4000 {
                    gTasks[sBirchSpeechMainTaskId].data[7] += 1;
                }
            }
            return;
        }
        _ => {}
    }
    *data += 1;
    if gTasks[sBirchSpeechMainTaskId].data[7] < 0x4000 {
        gTasks[sBirchSpeechMainTaskId].data[7] += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_AndYouAre(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        sStartedPokeBallTask = FALSE;
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_Birch_AndYouAre.as_ptr().cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_StartBirchLotadPlatformFade);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_StartBirchLotadPlatformFade(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        gSprites[gTasks[taskId].data[8]]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        gSprites[gTasks[taskId].data[9]]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2);
        NewGameBirchSpeech_StartFadePlatformIn(taskId, 1);
        gTasks[taskId].data[7] = 64;
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_SlidePlatformAway);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlidePlatformAway(taskId: u8) {
    if gTasks[taskId].data[4] != -60 {
        gTasks[taskId].data[4] -= 2;
        SetGpuReg(REG_OFFSET_BG1HOFS, gTasks[taskId].data[4] as u16);
    } else {
        gTasks[taskId].data[4] = -60;
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_StartPlayerFadeIn);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_StartPlayerFadeIn(taskId: u8) {
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[8]].set_invisible(TRUE as u16);
        gSprites[gTasks[taskId].data[9]].set_invisible(TRUE as u16);
        if gTasks[taskId].data[7] != 0 {
            gTasks[taskId].data[7] -= 1;
        } else {
            let mut spriteId: u8 = gTasks[taskId].data[10] as u8;
            gSprites[spriteId].x = 180;
            gSprites[spriteId].y = 60;
            gSprites[spriteId].set_invisible(FALSE as u16);
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
            gTasks[taskId].data[2] = spriteId as i16;
            gTasks[taskId].data[6] = MALE as i16;
            NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2);
            NewGameBirchSpeech_StartFadePlatformOut(taskId, 1);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitForPlayerFadeIn);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForPlayerFadeIn(taskId: u8) {
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[2]]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_BoyOrGirl);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_BoyOrGirl(taskId: u8) {
    NewGameBirchSpeech_ClearWindow(0);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_Birch_BoyOrGirl.as_ptr().cast_mut(),
    );
    AddTextPrinterForMessage(TRUE);
    gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitToShowGenderMenu);
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitToShowGenderMenu(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        NewGameBirchSpeech_ShowGenderMenu();
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_ChooseGender);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ChooseGender(taskId: u8) {
    let mut gender: i32 = NewGameBirchSpeech_ProcessGenderMenuInput() as i32;
    let mut gender2: i32 = 0;
    match gender {
        0 => {
            PlaySE(SE_SELECT);
            (*gSaveBlock2Ptr).playerGender = gender as u8;
            NewGameBirchSpeech_ClearGenderWindow(1, 1);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WhatsYourName);
        }
        1 => {
            PlaySE(SE_SELECT);
            (*gSaveBlock2Ptr).playerGender = gender as u8;
            NewGameBirchSpeech_ClearGenderWindow(1, 1);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WhatsYourName);
        }
        _ => {}
    }
    gender2 = Menu_GetCursorPos() as i32;
    if gender2 != gTasks[taskId].data[6] as i32 {
        gTasks[taskId].data[6] = gender2 as i16;
        gSprites[gTasks[taskId].data[2]]
            .oam
            .set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 0);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_SlideOutOldGenderSprite);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlideOutOldGenderSprite(taskId: u8) {
    let mut spriteId: u8 = gTasks[taskId].data[2] as u8;
    if gTasks[taskId].data[5] == 0 {
        gSprites[spriteId].x += 4;
    } else {
        gSprites[spriteId].set_invisible(TRUE as u16);
        if gTasks[taskId].data[6] != MALE as i16 {
            spriteId = gTasks[taskId].data[11] as u8;
        } else {
            spriteId = gTasks[taskId].data[10] as u8;
        }
        gSprites[spriteId].x = DISPLAY_WIDTH as i16;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gTasks[taskId].data[2] = spriteId as i16;
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 0);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_SlideInNewGenderSprite);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlideInNewGenderSprite(taskId: u8) {
    let mut spriteId: u8 = gTasks[taskId].data[2] as u8;
    if gSprites[spriteId].x > 180 {
        gSprites[spriteId].x -= 4;
    } else {
        gSprites[spriteId].x = 180;
        if gTasks[taskId].data[5] != 0 {
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_NORMAL as u32);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_ChooseGender);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WhatsYourName(taskId: u8) {
    NewGameBirchSpeech_ClearWindow(0);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_Birch_WhatsYourName.as_ptr().cast_mut(),
    );
    AddTextPrinterForMessage(TRUE);
    gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitForWhatsYourNameToPrint);
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForWhatsYourNameToPrint(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitPressBeforeNameChoice);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitPressBeforeNameChoice(taskId: u8) {
    if gMain.newKeys as i32 & A_BUTTON != 0 || gMain.newKeys as i32 & B_BUTTON != 0 {
        BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_StartNamingScreen);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_StartNamingScreen(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[9] as u16);
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
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SoItsPlayerName(taskId: u8) {
    NewGameBirchSpeech_ClearWindow(0);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_Birch_SoItsPlayer.as_ptr().cast_mut(),
    );
    AddTextPrinterForMessage(TRUE);
    gTasks[taskId].func = Some(Task_NewGameBirchSpeech_CreateNameYesNo);
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_CreateNameYesNo(taskId: u8) {
    if RunTextPrintersAndIsPrinter0Active() == 0 {
        CreateYesNoMenuParameterized(2, 1, 0xF3, 0xDF, 2, 15);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_ProcessNameYesNoMenu);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ProcessNameYesNoMenu(taskId: u8) {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            PlaySE(SE_SELECT);
            gSprites[gTasks[taskId].data[2]]
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
            NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2);
            NewGameBirchSpeech_StartFadePlatformIn(taskId, 1);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_SlidePlatformAway2);
        }
        MENU_B_PRESSED | 1 => {
            PlaySE(SE_SELECT);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_BoyOrGirl);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlidePlatformAway2(taskId: u8) {
    if gTasks[taskId].data[4] != 0 {
        gTasks[taskId].data[4] += 2;
        SetGpuReg(REG_OFFSET_BG1HOFS, gTasks[taskId].data[4] as u16);
    } else {
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_ReshowBirchLotad);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ReshowBirchLotad(taskId: u8) {
    let mut spriteId: u8 = 0;
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[10]].set_invisible(TRUE as u16);
        gSprites[gTasks[taskId].data[11]].set_invisible(TRUE as u16);
        spriteId = gTasks[taskId].data[8] as u8;
        gSprites[spriteId].x = 136;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        spriteId = gTasks[taskId].data[9] as u8;
        gSprites[spriteId].x = 100;
        gSprites[spriteId].y = 75;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2);
        NewGameBirchSpeech_StartFadePlatformOut(taskId, 1);
        NewGameBirchSpeech_ClearWindow(0);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_Birch_YourePlayer.as_ptr().cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitForSpriteFadeInAndTextPrinter);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForSpriteFadeInAndTextPrinter(
    taskId: u8,
) {
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[8]]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        gSprites[gTasks[taskId].data[9]]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        if RunTextPrintersAndIsPrinter0Active() == 0 {
            gSprites[gTasks[taskId].data[8]]
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
            gSprites[gTasks[taskId].data[9]]
                .oam
                .set_objMode(ST_OAM_OBJ_BLEND);
            NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2);
            NewGameBirchSpeech_StartFadePlatformIn(taskId, 1);
            gTasks[taskId].data[7] = 64;
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_AreYouReady);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_AreYouReady(taskId: u8) {
    let mut spriteId: u8 = 0;
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[8]].set_invisible(TRUE as u16);
        gSprites[gTasks[taskId].data[9]].set_invisible(TRUE as u16);
        if gTasks[taskId].data[7] != 0 {
            gTasks[taskId].data[7] -= 1;
            return;
        }
        if (*gSaveBlock2Ptr).playerGender != MALE {
            spriteId = gTasks[taskId].data[11] as u8;
        } else {
            spriteId = gTasks[taskId].data[10] as u8;
        }
        gSprites[spriteId].x = 120;
        gSprites[spriteId].y = 60;
        gSprites[spriteId].set_invisible(FALSE as u16);
        gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
        gTasks[taskId].data[2] = spriteId as i16;
        NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2);
        NewGameBirchSpeech_StartFadePlatformOut(taskId, 1);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_Birch_AreYouReady.as_ptr().cast_mut(),
        );
        AddTextPrinterForMessage(TRUE);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_ShrinkPlayer);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ShrinkPlayer(taskId: u8) {
    let mut spriteId: u8 = 0;
    if gTasks[taskId].data[5] != 0 {
        gSprites[gTasks[taskId].data[2]]
            .oam
            .set_objMode(ST_OAM_OBJ_NORMAL as u32);
        if RunTextPrintersAndIsPrinter0Active() == 0 {
            spriteId = gTasks[taskId].data[2] as u8;
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            gSprites[spriteId].affineAnims =
                sSpriteAffineAnimTable_PlayerShrink.as_ptr().cast_mut();
            InitSpriteAffineAnim(&raw mut gSprites[spriteId]);
            StartSpriteAffineAnim(&raw mut gSprites[spriteId], 0);
            gSprites[spriteId].callback = Some(SpriteCB_MovePlayerDownWhileShrinking);
            BeginNormalPaletteFade(PALETTES_BG, 0, 0, 16, 0);
            FadeOutBGM(4);
            gTasks[taskId].func = Some(Task_NewGameBirchSpeech_WaitForPlayerShrink);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForPlayerShrink(taskId: u8) {
    let mut spriteId: u8 = gTasks[taskId].data[2] as u8;
    if gSprites[spriteId].affineAnimEnded() != 0 {
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_FadePlayerToWhite);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadePlayerToWhite(taskId: u8) {
    let mut spriteId: u8 = 0;
    if gPaletteFade.active() == 0 {
        spriteId = gTasks[taskId].data[2] as u8;
        gSprites[spriteId].callback = Some(SpriteCB_Null);
        SetGpuReg(REG_OFFSET_DISPCNT, 4160);
        BeginNormalPaletteFade(PALETTES_OBJECTS, 0, 0, 16, 65535);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_Cleanup);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_Cleanup(taskId: u8) {
    if gPaletteFade.active() == 0 {
        FreeAllWindowBuffers();
        FreeAndDestroyMonPicSprite(gTasks[taskId].data[9] as u16);
        ResetAllPicSprites();
        SetMainCallback2(Some(CB2_NewGame));
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CB2_NewGameBirchSpeech_ReturnFromNamingScreen() {
    let mut taskId: u8 = 0;
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
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
            let mut tmp: u16 = 0;
            volatile_write(&raw mut tmp, 0);
            {
                {
                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
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
        0x6003800 as usize as *mut u8 as *mut c_void,
    );
    LoadPalette(sBirchSpeechBgPals.as_ptr().cast_mut() as *mut c_void, 0, 64);
    LoadPalette(
        (&raw const sBirchSpeechBgGradientPal[1]).cast_mut() as *mut c_void,
        1,
        16,
    );
    ResetTasks();
    taskId = CreateTask(
        Some(Task_NewGameBirchSpeech_ReturnFromNamingScreenShowTextbox),
        0,
    );
    gTasks[taskId].data[7] = 5;
    gTasks[taskId].data[4] = -60;
    ScanlineEffect_Stop();
    ResetSpriteData();
    FreeAllSpritePalettes();
    ResetAllPicSprites();
    AddBirchSpeechObjects(taskId);
    if (*gSaveBlock2Ptr).playerGender != MALE {
        gTasks[taskId].data[6] = FEMALE as i16;
        spriteId = gTasks[taskId].data[11] as u8;
    } else {
        gTasks[taskId].data[6] = MALE as i16;
        spriteId = gTasks[taskId].data[10] as u8;
    }
    gSprites[spriteId].x = 180;
    gSprites[spriteId].y = 60;
    gSprites[spriteId].set_invisible(FALSE as u16);
    gTasks[taskId].data[2] = spriteId as i16;
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
        let mut imeTemp: u16 = 0;
        imeTemp = (67109384 as usize as *mut u16).read_volatile();
        volatile_write(67109384 as usize as *mut u16, 0);
        volatile_write(
            0x4000200 as usize as *mut u16,
            (0x4000200 as usize as *mut u16).read_volatile() | INTR_FLAG_VBLANK,
        );
        volatile_write(67109384 as usize as *mut u16, imeTemp);
    }
    SetVBlankCallback(Some(VBlankCB_MainMenu));
    SetMainCallback2(Some(CB2_MainMenu));
    InitWindows(sNewGameBirchSpeechTextWindows.as_ptr().cast_mut());
    LoadMainMenuWindowFrameTiles(0, 0xF3);
    LoadMessageBoxGfx(0, BIRCH_DLG_BASE_TILE_NUM, 240);
    PutWindowTilemap(0);
    CopyWindowToVram(0, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn SpriteCB_Null(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn SpriteCB_MovePlayerDownWhileShrinking(sprite: *mut Sprite) {
    let mut y: u32 = 0;
    y = (((*sprite).y as u32) << 16) + (*sprite).data[0] as u32 + 0xC000;
    (*sprite).y = (y >> 16) as i16;
    (*sprite).data[0] = y as i16;
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_CreateLotadSprite(x: u8, y: u8) -> u8 {
    return CreateMonPicSprite_Affine(
        SPECIES_LOTAD,
        SHINY_ODDS,
        0,
        MON_PIC_AFFINE_FRONT,
        x as i16,
        y as i16,
        14,
        TAG_NONE,
    ) as u8;
}
pub(crate) unsafe extern "C" fn AddBirchSpeechObjects(taskId: u8) {
    let mut birchSpriteId: u8 = 0;
    let mut lotadSpriteId: u8 = 0;
    let mut brendanSpriteId: u8 = 0;
    let mut maySpriteId: u8 = 0;
    birchSpriteId = AddNewGameBirchObject(0x88, 0x3C, 1);
    gSprites[birchSpriteId].callback = Some(SpriteCB_Null);
    gSprites[birchSpriteId].oam.set_priority(0);
    gSprites[birchSpriteId].set_invisible(TRUE as u16);
    gTasks[taskId].data[8] = birchSpriteId as i16;
    lotadSpriteId = NewGameBirchSpeech_CreateLotadSprite(100, 0x4B);
    gSprites[lotadSpriteId].callback = Some(SpriteCB_Null);
    gSprites[lotadSpriteId].oam.set_priority(0);
    gSprites[lotadSpriteId].set_invisible(TRUE as u16);
    gTasks[taskId].data[9] = lotadSpriteId as i16;
    brendanSpriteId = CreateTrainerSprite(
        FacilityClassToPicIndex(0x3c) as u8,
        120,
        60,
        0,
        &raw mut gDecompressionBuffer[0],
    );
    gSprites[brendanSpriteId].callback = Some(SpriteCB_Null);
    gSprites[brendanSpriteId].set_invisible(TRUE as u16);
    gSprites[brendanSpriteId].oam.set_priority(0);
    gTasks[taskId].data[10] = brendanSpriteId as i16;
    maySpriteId = CreateTrainerSprite(
        FacilityClassToPicIndex(FACILITY_CLASS_MAY) as u8,
        120,
        60,
        0,
        &raw mut gDecompressionBuffer[2048],
    );
    gSprites[maySpriteId].callback = Some(SpriteCB_Null);
    gSprites[maySpriteId].set_invisible(TRUE as u16);
    gSprites[maySpriteId].oam.set_priority(0);
    gTasks[taskId].data[11] = maySpriteId as i16;
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadeOutTarget1InTarget2(taskId: u8) {
    let mut alphaCoeff2: i32 = 0;
    if gTasks[taskId].data[1] == 0 {
        gTasks[gTasks[taskId].data[0]].data[5] = TRUE as i16;
        DestroyTask(taskId);
    } else if gTasks[taskId].data[4] != 0 {
        gTasks[taskId].data[4] -= 1;
    } else {
        gTasks[taskId].data[4] = gTasks[taskId].data[3];
        gTasks[taskId].data[1] -= 1;
        gTasks[taskId].data[2] += 1;
        alphaCoeff2 = (gTasks[taskId].data[2] as i32) << 8;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            gTasks[taskId].data[1] as u16 + alphaCoeff2 as u16,
        );
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadeOutTarget1InTarget2(
    taskId: u8,
    delay: u8,
) {
    let mut taskId2: u8 = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 592);
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    gTasks[taskId].data[5] = 0;
    taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadeOutTarget1InTarget2), 0);
    gTasks[taskId2].data[0] = taskId as i16;
    gTasks[taskId2].data[1] = 16;
    gTasks[taskId2].data[2] = 0;
    gTasks[taskId2].data[3] = delay as i16;
    gTasks[taskId2].data[4] = delay as i16;
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadeInTarget1OutTarget2(taskId: u8) {
    let mut alphaCoeff2: i32 = 0;
    if gTasks[taskId].data[1] == 16 {
        gTasks[gTasks[taskId].data[0]].data[5] = TRUE as i16;
        DestroyTask(taskId);
    } else if gTasks[taskId].data[4] != 0 {
        gTasks[taskId].data[4] -= 1;
    } else {
        gTasks[taskId].data[4] = gTasks[taskId].data[3];
        gTasks[taskId].data[1] += 1;
        gTasks[taskId].data[2] -= 1;
        alphaCoeff2 = (gTasks[taskId].data[2] as i32) << 8;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            gTasks[taskId].data[1] as u16 + alphaCoeff2 as u16,
        );
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadeInTarget1OutTarget2(
    taskId: u8,
    delay: u8,
) {
    let mut taskId2: u8 = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 592);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetGpuReg(REG_OFFSET_BLDY, 0);
    gTasks[taskId].data[5] = 0;
    taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadeInTarget1OutTarget2), 0);
    gTasks[taskId2].data[0] = taskId as i16;
    gTasks[taskId2].data[1] = 0;
    gTasks[taskId2].data[2] = 16;
    gTasks[taskId2].data[3] = delay as i16;
    gTasks[taskId2].data[4] = delay as i16;
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadePlatformIn(taskId: u8) {
    if gTasks[taskId].data[2] != 0 {
        gTasks[taskId].data[2] -= 1;
    } else if gTasks[taskId].data[1] == 8 {
        DestroyTask(taskId);
    } else if gTasks[taskId].data[4] != 0 {
        gTasks[taskId].data[4] -= 1;
    } else {
        gTasks[taskId].data[4] = gTasks[taskId].data[3];
        gTasks[taskId].data[1] += 1;
        LoadPalette(
            (&raw const sBirchSpeechBgGradientPal[gTasks[taskId].data[1]]).cast_mut()
                as *mut c_void,
            1,
            16,
        );
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadePlatformIn(taskId: u8, delay: u8) {
    let mut taskId2: u8 = 0;
    taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadePlatformIn), 0);
    gTasks[taskId2].data[0] = taskId as i16;
    gTasks[taskId2].data[1] = 0;
    gTasks[taskId2].data[2] = 8;
    gTasks[taskId2].data[3] = delay as i16;
    gTasks[taskId2].data[4] = delay as i16;
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadePlatformOut(taskId: u8) {
    if gTasks[taskId].data[2] != 0 {
        gTasks[taskId].data[2] -= 1;
    } else if gTasks[taskId].data[1] == 0 {
        DestroyTask(taskId);
    } else if gTasks[taskId].data[4] != 0 {
        gTasks[taskId].data[4] -= 1;
    } else {
        gTasks[taskId].data[4] = gTasks[taskId].data[3];
        gTasks[taskId].data[1] -= 1;
        LoadPalette(
            (&raw const sBirchSpeechBgGradientPal[gTasks[taskId].data[1]]).cast_mut()
                as *mut c_void,
            1,
            16,
        );
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadePlatformOut(taskId: u8, delay: u8) {
    let mut taskId2: u8 = 0;
    taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadePlatformOut), 0);
    gTasks[taskId2].data[0] = taskId as i16;
    gTasks[taskId2].data[1] = 8;
    gTasks[taskId2].data[2] = 8;
    gTasks[taskId2].data[3] = delay as i16;
    gTasks[taskId2].data[4] = delay as i16;
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ShowGenderMenu() {
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
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ProcessGenderMenuInput() -> i8 {
    return Menu_ProcessInputNoWrap();
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_SetDefaultPlayerName(nameId: u8) {
    let mut name: *mut u8 = null_mut();
    let mut i: u8 = 0;
    if (*gSaveBlock2Ptr).playerGender == MALE {
        name = sMalePresetNames[nameId];
    } else {
        name = sFemalePresetNames[nameId];
    }
    i = 0;
    while i < PLAYER_NAME_LENGTH as u8 {
        (*gSaveBlock2Ptr).playerName[i] = *name.at(i);
        i += 1;
    }
    (*gSaveBlock2Ptr).playerName[7] = EOS;
}
pub(crate) unsafe extern "C" fn CreateMainMenuErrorWindow(str: *mut u8) {
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
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegameText() {
    MainMenu_FormatSavegamePlayer();
    MainMenu_FormatSavegamePokedex();
    MainMenu_FormatSavegameTime();
    MainMenu_FormatSavegameBadges();
}
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegamePlayer() {
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_ContinueMenuPlayer.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegameTime() {
    let mut str: CArray<u8, 32> = zeroed();
    let mut ptr: *mut u8 = null_mut();
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_ContinueMenuTime.as_ptr().cast_mut(),
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
    ptr = ConvertIntToDecimalStringN(
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
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegamePokedex() {
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
            gText_ContinueMenuPokedex.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegameBadges() {
    let mut str: CArray<u8, 32> = zeroed();
    let mut badgeCount: u8 = 0;
    let mut i: u32 = 0;
    i = FLAG_BADGE01_GET;
    while i < 2159 {
        if FlagGet(i as u16) != 0 {
            badgeCount += 1;
        }
        i += 1;
    }
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_ContinueMenuBadges.as_ptr().cast_mut(),
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
pub(crate) unsafe extern "C" fn LoadMainMenuWindowFrameTiles(bgId: u8, tileOffset: u16) {
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
pub(crate) unsafe extern "C" fn DrawMainMenuWindowBorder(
    template: *mut WindowTemplate,
    baseTileNum: u16,
) {
    let mut r9: u16 = 1 + baseTileNum;
    let mut r10: u16 = 2 + baseTileNum;
    let mut sp18: u16 = 3 + baseTileNum;
    let mut spC: u16 = 5 + baseTileNum;
    let mut sp10: u16 = 6 + baseTileNum;
    let mut sp14: u16 = 7 + baseTileNum;
    let mut r6: u16 = 8 + baseTileNum;
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
pub(crate) unsafe extern "C" fn ClearMainMenuWindowTilemap(template: *mut WindowTemplate) {
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
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ClearGenderWindowTilemap(
    bg: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    unused: u8,
) {
    FillBgTilemapBufferRect(bg, 0, x + 255, y + 255, width + 2, height + 2, 2);
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ClearGenderWindow(windowId: u8, copyToVram: u8) {
    CallWindowFunction(windowId, Some(NewGameBirchSpeech_ClearGenderWindowTilemap));
    FillWindowPixelBuffer(windowId, 17);
    ClearWindowTilemap(windowId);
    if copyToVram == TRUE {
        CopyWindowToVram(windowId, COPYWIN_FULL);
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ClearWindow(windowId: u8) {
    let mut bgColor: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_COLOR_BACKGROUND);
    let mut maxCharWidth: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_MAX_LETTER_WIDTH);
    let mut maxCharHeight: u8 = GetFontAttribute(FONT_NORMAL, FONTATTR_MAX_LETTER_HEIGHT);
    let mut winWidth: u8 = GetWindowAttribute(windowId, WINDOW_WIDTH) as u8;
    let mut winHeight: u8 = GetWindowAttribute(windowId, WINDOW_HEIGHT) as u8;
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
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_WaitForThisIsPokemonText(
    printer: *mut TextPrinterTemplate,
    renderCmd: u16,
) {
    if *(*printer).currentChar.at(-2) == EXT_CTRL_CODE_PAUSE && sStartedPokeBallTask == 0 {
        sStartedPokeBallTask = TRUE;
        CreateTask(Some(Task_NewGameBirchSpeechSub_InitPokeBall), 0);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateYesNoMenuParameterized(
    x: u8,
    y: u8,
    baseTileNum: u16,
    baseBlock: u16,
    yesNoPalNum: u8,
    winPalNum: u8,
) {
    let mut template: WindowTemplate = zeroed();
    template = CreateWindowTemplate(0, x + 1, y + 1, 5, 4, winPalNum, baseBlock);
    CreateYesNoMenu(&raw mut template, baseTileNum, yesNoPalNum, 0);
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ShowDialogueWindow(
    windowId: u8,
    copyToVram: u8,
) {
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
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_CreateDialogueWindowBorder(
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
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ReturnFromNamingScreenShowTextbox(
    taskId: u8,
) {
    if ({
        let t1 = gTasks[taskId].data[7];
        gTasks[taskId].data[7] -= 1;
        t1
    }) <= 0
    {
        NewGameBirchSpeech_ShowDialogueWindow(0, 1);
        gTasks[taskId].func = Some(Task_NewGameBirchSpeech_SoItsPlayerName);
    }
}
