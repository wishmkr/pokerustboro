//! Translated from `src/main_menu.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): sBirchSpeechBgPals sBirchSpeechShadowGfx sBirchSpeechBgMap sBirchSpeechBgGradientPal sWindowTemplates_MainMenu sNewGameBirchSpeechTextWindows sMainMenuBgPal sMainMenuTextPal sTextColor_Headers sTextColor_MenuInfo sMainMenuBgTemplates sBirchBgTemplate sScrollArrowsTemplate_MainMenu sSpriteAffineAnim_PlayerShrink sSpriteAffineAnimTable_PlayerShrink sMenuActions_Gender sMalePresetNames sFemalePresetNames
#[allow(unused_imports)]
use crate::data::main_menu::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStartedPokeBallTask: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrItemAndOptionMenuCheck: u16 = 0u16;
pub(crate) static mut sBirchSpeechMainTaskId: u8 = 0u8;

unsafe extern "C" {
    static mut gDecompressionBuffer: u8;
    static mut gJPText_No1MSubCircuit: u8;
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSaveFileStatus: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BatteryRunDry: u8;
    static mut gText_Birch_AndYouAre: u8;
    static mut gText_Birch_AreYouReady: u8;
    static mut gText_Birch_BoyOrGirl: u8;
    static mut gText_Birch_MainSpeech: u8;
    static mut gText_Birch_SoItsPlayer: u8;
    static mut gText_Birch_Welcome: u8;
    static mut gText_Birch_WhatsYourName: u8;
    static mut gText_Birch_YourePlayer: u8;
    static mut gText_ContinueMenuBadges: u8;
    static mut gText_ContinueMenuPlayer: u8;
    static mut gText_ContinueMenuPokedex: u8;
    static mut gText_ContinueMenuTime: u8;
    static mut gText_MainMenuContinue: u8;
    static mut gText_MainMenuMysteryEvents: u8;
    static mut gText_MainMenuMysteryGift: u8;
    static mut gText_MainMenuMysteryGift2: u8;
    static mut gText_MainMenuNewGame: u8;
    static mut gText_MainMenuOption: u8;
    static mut gText_MysteryEventsCantUse: u8;
    static mut gText_MysteryGiftCantUse: u8;
    static mut gText_SaveFileCorrupted: u8;
    static mut gText_SaveFileErased: u8;
    static mut gText_ThisIsAPokemon: u8;
    static mut gText_WirelessNotConnected: u8;
    fn AddNewGameBirchObject(a0: i16, a1: i16, a2: u8) -> u8;
    fn AddScrollIndicatorArrowPair(a0: *mut u8, a1: *mut u16) -> u8;
    fn AddTextPrinterForMessage(a0: u8);
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
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
    fn AddTextPrinterWithCallbackForMessage(a0: u8, a1: Option<unsafe extern "C" fn(*mut u8, u16)>);
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
    ) -> crate::c::Rec4<8>;
    fn CreateYesNoMenu(a0: *mut u8, a1: u16, a2: u8, a3: u8);
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
    fn GetWindowFrameTilesPal(a0: u8) -> *mut u8;
    fn HideBg(a0: u8);
    fn InitBgFromTemplate(a0: *mut u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsMysteryGiftEnabled() -> u32;
    fn IsNationalPokedexEnabled() -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn IsWirelessAdapterConnected() -> u8;
    fn LZ77UnCompVram(a0: *mut u32, a1: *mut u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadMessageBoxGfx(a0: u8, a1: u16, a2: u8);
    fn LoadOam();
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlayBGM(a0: u16);
    fn PlaySE(a0: u16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
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
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_ScrollIndicatorArrowPairOnMainMenu(a0: u8);
    fn TransferPlttBuffer();
    fn UpdatePaletteFade() -> u8;
}

pub(crate) unsafe extern "C" fn CB2_MainMenu() {
    unsafe {
        RunTasks();
        AnimateSprites();
        BuildOamBuffer();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_MainMenu() {
    unsafe {
        LoadOam();
        ProcessSpriteCopyRequests();
        TransferPlttBuffer();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_InitMainMenu() {
    unsafe {
        InitMainMenu(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_ReinitMainMenu() {
    unsafe {
        InitMainMenu(1u8);
    }
}
pub(crate) unsafe extern "C" fn InitMainMenu(returningFromOptionsMenu: u8) -> u32 {
    unsafe {
        let mut returningFromOptionsMenu = returningFromOptionsMenu;
        SetVBlankCallback(None);
        SetGpuReg(0u8, 0u16);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((100663296i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            98304i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((117440512i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2063597568i32)
                                        | crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l11: loop {
                        'l12: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    (((83886082i32) as usize as *mut u8) as usize as u32),
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            1022i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        ResetPaletteFade();
        LoadPalette(
            (((&raw const sMainMenuBgPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            0u16,
            32u16,
        );
        LoadPalette(
            (((&raw const sMainMenuTextPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            240u16,
            32u16,
        );
        ScanlineEffect_Stop();
        ResetTasks();
        ResetSpriteData();
        FreeAllSpritePalettes();
        if (returningFromOptionsMenu) != 0 {
            BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        } else {
            BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 65535u16);
        }
        ResetBgsAndClearDma3BusyFlags(0u32);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sMainMenuBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        ChangeBgX(0u8, 0i32, 0u8);
        ChangeBgY(0u8, 0i32, 0u8);
        ChangeBgX(1u8, 0i32, 0u8);
        ChangeBgY(1u8, 0i32, 0u8);
        InitWindows(
            ((&raw const sWindowTemplates_MainMenu)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        DeactivateAllTextPrinters();
        LoadMainMenuWindowFrameTiles(0u8, 469u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        EnableInterrupts(1u16);
        SetVBlankCallback(Some(VBlankCB_MainMenu));
        SetMainCallback2(Some(CB2_MainMenu));
        SetGpuReg(0u8, 12352u16);
        ShowBg(0u8);
        HideBg(1u8);
        CreateTask(Some(Task_MainMenuCheckSaveFile), 0u8);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn Task_MainMenuCheckSaveFile(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetGpuReg(64u8, 0u16);
            SetGpuReg(68u8, 0u16);
            SetGpuReg(72u8, 17u16);
            SetGpuReg(74u8, 49u16);
            SetGpuReg(80u8, 193u16);
            SetGpuReg(82u8, 0u16);
            SetGpuReg(84u8, 7u16);
            if (IsWirelessAdapterConnected()) != 0 {
                ((data).wrapping_offset(15)).write(1i16);
            }
            'l1: {
                let __sw1 = ((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32);
                let __matched = __sw1 == 1i32
                    || __sw1 == 2i32
                    || __sw1 == 255i32
                    || __sw1 == 0i32
                    || __sw1 == 4i32;
                if __sw1 == 1i32 {
                    (data).write(1i16);
                    if (IsMysteryGiftEnabled()) != 0 {
                        (data).write(((data).read()).wrapping_add(1));
                    }
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_MainMenuCheckBattery));
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    CreateMainMenuErrorWindow((&raw mut gText_SaveFileErased).cast::<u8>());
                    (data).write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_WaitForSaveFileErrorWindow));
                    break 'l1;
                }
                if __sw1 == 255i32 {
                    CreateMainMenuErrorWindow((&raw mut gText_SaveFileCorrupted).cast::<u8>());
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_WaitForSaveFileErrorWindow));
                    (data).write(1i16);
                    if IsMysteryGiftEnabled() == 1u32 {
                        (data).write(((data).read()).wrapping_add(1));
                    }
                    break 'l1;
                }
                if __sw1 == 0i32 || !__matched {
                    (data).write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_MainMenuCheckBattery));
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    CreateMainMenuErrorWindow((&raw mut gJPText_No1MSubCircuit).cast::<u8>());
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_WaitForSaveFileErrorWindow));
                    break 'l1;
                }
            }
            if (((((&raw mut sCurrItemAndOptionMenuCheck)
                .cast::<u8>()
                .cast::<u16>())
            .read()) as i32)
                & 32768i32)
                != 0
            {
                'l2: {
                    let __sw2 = (((data).read()) as i32);
                    if __sw2 == 0i32 || __sw2 == 1i32 {
                        ((&raw mut sCurrItemAndOptionMenuCheck)
                            .cast::<u8>()
                            .cast::<u16>())
                        .write((((((data).read()) as i32).wrapping_add(1i32)) as u16));
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        ((&raw mut sCurrItemAndOptionMenuCheck)
                            .cast::<u8>()
                            .cast::<u16>())
                        .write(3u16);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        ((&raw mut sCurrItemAndOptionMenuCheck)
                            .cast::<u8>()
                            .cast::<u16>())
                        .write(4u16);
                        break 'l2;
                    }
                }
            }
            let __p3 = (&raw mut sCurrItemAndOptionMenuCheck)
                .cast::<u8>()
                .cast::<u16>();
            (__p3).write((((((__p3).read()) as i32) & (-32769i32)) as u16));
            ((data).wrapping_offset(1)).write(
                ((((&raw mut sCurrItemAndOptionMenuCheck)
                    .cast::<u8>()
                    .cast::<u16>())
                .read()) as i16),
            );
            ((data).wrapping_offset(12))
                .write((((((data).read()) as i32).wrapping_add(2i32)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForSaveFileErrorWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RunTextPrinters();
        if (!((IsTextPrinterActive(7u8)) != 0))
            && (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0)
        {
            ClearWindowTilemap(7u8);
            ClearMainMenuWindowTilemap(
                (((&raw const sWindowTemplates_MainMenu)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(56),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_MainMenuCheckBattery));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_MainMenuCheckBattery(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetGpuReg(64u8, 0u16);
            SetGpuReg(68u8, 0u16);
            SetGpuReg(72u8, 17u16);
            SetGpuReg(74u8, 49u16);
            SetGpuReg(80u8, 193u16);
            SetGpuReg(82u8, 0u16);
            SetGpuReg(84u8, 7u16);
            if !((((RtcGetErrorStatus()) as i32) & 4080i32) != 0) {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_DisplayMainMenu));
            } else {
                CreateMainMenuErrorWindow((&raw mut gText_BatteryRunDry).cast::<u8>());
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_WaitForBatteryDryErrorWindow));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForBatteryDryErrorWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RunTextPrinters();
        if (!((IsTextPrinterActive(7u8)) != 0))
            && (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0)
        {
            ClearWindowTilemap(7u8);
            ClearMainMenuWindowTilemap(
                (((&raw const sWindowTemplates_MainMenu)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(56),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_DisplayMainMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayMainMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut palette: u16 = 0u16;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            SetGpuReg(64u8, 0u16);
            SetGpuReg(68u8, 0u16);
            SetGpuReg(72u8, 17u16);
            SetGpuReg(74u8, 49u16);
            SetGpuReg(80u8, 193u16);
            SetGpuReg(82u8, 0u16);
            SetGpuReg(84u8, 7u16);
            palette = 0u16;
            LoadPalette((&raw mut palette).cast::<u8>(), 254u16, 2u16);
            palette = 32767u16;
            LoadPalette((&raw mut palette).cast::<u8>(), 250u16, 2u16);
            palette = 12684u16;
            LoadPalette((&raw mut palette).cast::<u8>(), 251u16, 2u16);
            palette = 26458u16;
            LoadPalette((&raw mut palette).cast::<u8>(), 252u16, 2u16);
            if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
                == 0i32
            {
                palette = 32260u16;
                LoadPalette((&raw mut palette).cast::<u8>(), 241u16, 2u16);
            } else {
                palette = 21631u16;
                LoadPalette((&raw mut palette).cast::<u8>(), 241u16, 2u16);
            }
            'l1: {
                let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                if __sw1 == 0i32 || !__matched {
                    FillWindowPixelBuffer(0u8, 170u8);
                    FillWindowPixelBuffer(1u8, 170u8);
                    AddTextPrinterParameterized3(
                        0u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuNewGame).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        1u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuOption).cast::<u8>(),
                    );
                    PutWindowTilemap(0u8);
                    PutWindowTilemap(1u8);
                    CopyWindowToVram(0u8, 2u8);
                    CopyWindowToVram(1u8, 2u8);
                    DrawMainMenuWindowBorder(
                        ((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>(),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(8),
                        469u16,
                    );
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    FillWindowPixelBuffer(2u8, 170u8);
                    FillWindowPixelBuffer(3u8, 170u8);
                    FillWindowPixelBuffer(4u8, 170u8);
                    AddTextPrinterParameterized3(
                        2u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuContinue).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        3u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuNewGame).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        4u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuOption).cast::<u8>(),
                    );
                    MainMenu_FormatSavegameText();
                    PutWindowTilemap(2u8);
                    PutWindowTilemap(3u8);
                    PutWindowTilemap(4u8);
                    CopyWindowToVram(2u8, 2u8);
                    CopyWindowToVram(3u8, 2u8);
                    CopyWindowToVram(4u8, 2u8);
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(16),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(24),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(32),
                        469u16,
                    );
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    FillWindowPixelBuffer(2u8, 170u8);
                    FillWindowPixelBuffer(3u8, 170u8);
                    FillWindowPixelBuffer(4u8, 170u8);
                    FillWindowPixelBuffer(5u8, 170u8);
                    AddTextPrinterParameterized3(
                        2u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuContinue).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        3u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuNewGame).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        4u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuMysteryGift).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        5u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuOption).cast::<u8>(),
                    );
                    MainMenu_FormatSavegameText();
                    PutWindowTilemap(2u8);
                    PutWindowTilemap(3u8);
                    PutWindowTilemap(4u8);
                    PutWindowTilemap(5u8);
                    CopyWindowToVram(2u8, 2u8);
                    CopyWindowToVram(3u8, 2u8);
                    CopyWindowToVram(4u8, 2u8);
                    CopyWindowToVram(5u8, 2u8);
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(16),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(24),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(32),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(40),
                        469u16,
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    FillWindowPixelBuffer(2u8, 170u8);
                    FillWindowPixelBuffer(3u8, 170u8);
                    FillWindowPixelBuffer(4u8, 170u8);
                    FillWindowPixelBuffer(5u8, 170u8);
                    FillWindowPixelBuffer(6u8, 170u8);
                    AddTextPrinterParameterized3(
                        2u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuContinue).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        3u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuNewGame).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        4u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuMysteryGift2).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        5u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuMysteryEvents).cast::<u8>(),
                    );
                    AddTextPrinterParameterized3(
                        6u8,
                        1u8,
                        0u8,
                        1u8,
                        ((&raw const sTextColor_Headers).cast::<u8>().cast_mut()).cast::<u8>(),
                        (-1i8),
                        (&raw mut gText_MainMenuOption).cast::<u8>(),
                    );
                    MainMenu_FormatSavegameText();
                    PutWindowTilemap(2u8);
                    PutWindowTilemap(3u8);
                    PutWindowTilemap(4u8);
                    PutWindowTilemap(5u8);
                    PutWindowTilemap(6u8);
                    CopyWindowToVram(2u8, 2u8);
                    CopyWindowToVram(3u8, 2u8);
                    CopyWindowToVram(4u8, 2u8);
                    CopyWindowToVram(5u8, 2u8);
                    CopyWindowToVram(6u8, 2u8);
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(16),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(24),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(32),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(40),
                        469u16,
                    );
                    DrawMainMenuWindowBorder(
                        (((&raw const sWindowTemplates_MainMenu)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(48),
                        469u16,
                    );
                    ((data).wrapping_offset(13)).write(
                        ((AddScrollIndicatorArrowPair(
                            (&raw const sScrollArrowsTemplate_MainMenu)
                                .cast::<u8>()
                                .cast_mut(),
                            (&raw mut sCurrItemAndOptionMenuCheck)
                                .cast::<u8>()
                                .cast::<u16>(),
                        )) as i16),
                    );
                    ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((data).wrapping_offset(13)).read()) as i32) as isize * 40,
                    ))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_ScrollIndicatorArrowPairOnMainMenu));
                    if ((((&raw mut sCurrItemAndOptionMenuCheck)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32)
                        == 4i32
                    {
                        ChangeBgY(0u8, 8192i32, 1u8);
                        ChangeBgY(1u8, 8192i32, 1u8);
                        ((data).wrapping_offset(14)).write(1i16);
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_offset(13)).read()) as i32) as isize * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .write(1i16);
                    }
                    break 'l1;
                }
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HighlightSelectedMainMenuItem));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HighlightSelectedMainMenuItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        HighlightSelectedMainMenuItem(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8),
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_HandleMainMenuInput));
    }
}
pub(crate) unsafe extern "C" fn HandleMainMenuInput(taskId: u8) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            IsWirelessAdapterConnected();
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HandleMainMenuAPressed));
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0
            {
                PlaySE(5u16);
                BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 65535u16);
                SetGpuReg(64u8, 240u16);
                SetGpuReg(68u8, 160u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_HandleMainMenuBPressed));
            } else {
                if (((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 64i32)
                    != 0)
                    && (((((data).wrapping_offset(1)).read()) as i32) > 0i32)
                {
                    if (((((data).read()) as i32) == 3i32)
                        && (((((data).wrapping_offset(14)).read()) as i32) == 1i32))
                        && (((((data).wrapping_offset(1)).read()) as i32) == 1i32)
                    {
                        ChangeBgY(0u8, 8192i32, 2u8);
                        ChangeBgY(1u8, 8192i32, 2u8);
                        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((data).wrapping_offset(13)).read()) as i32) as isize * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15))
                        .write({
                            let __v1 = 0i16;
                            ((data).wrapping_offset(14)).write(__v1);
                            __v1
                        });
                    }
                    let __p2 = (data).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    ((&raw mut sCurrItemAndOptionMenuCheck)
                        .cast::<u8>()
                        .cast::<u16>())
                    .write(((((data).wrapping_offset(1)).read()) as u16));
                    return 1u8;
                } else {
                    if (((((((&raw mut gMain).cast::<u8>())
                        .wrapping_add(46)
                        .cast::<u16>())
                    .read()) as i32)
                        & 128i32)
                        != 0)
                        && (((((data).wrapping_offset(1)).read()) as i32)
                            < ((((data).wrapping_offset(12)).read()) as i32).wrapping_sub(1i32))
                    {
                        if (((((data).read()) as i32) == 3i32)
                            && (((((data).wrapping_offset(1)).read()) as i32) == 3i32))
                            && (((((data).wrapping_offset(14)).read()) as i32) == 0i32)
                        {
                            ChangeBgY(0u8, 8192i32, 1u8);
                            ChangeBgY(1u8, 8192i32, 1u8);
                            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                                ((((data).wrapping_offset(13)).read()) as i32) as isize * 40,
                            ))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .write({
                                let __v3 = 1i16;
                                ((data).wrapping_offset(14)).write(__v3);
                                __v3
                            });
                        }
                        let __p4 = (data).wrapping_offset(1);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        ((&raw mut sCurrItemAndOptionMenuCheck)
                            .cast::<u8>()
                            .cast::<u16>())
                        .write(((((data).wrapping_offset(1)).read()) as u16));
                        return 1u8;
                    }
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMainMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (HandleMainMenuInput(taskId)) != 0 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_HighlightSelectedMainMenuItem));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMainMenuAPressed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut wirelessAdapterConnected: u8 = 0u8;
        let mut action: u8 = 0u8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 3i32
            {
                RemoveScrollIndicatorArrowPair(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .read()) as u8),
                );
            }
            ClearStdWindowAndFrame(0u8, 1u8);
            ClearStdWindowAndFrame(1u8, 1u8);
            ClearStdWindowAndFrame(2u8, 1u8);
            ClearStdWindowAndFrame(3u8, 1u8);
            ClearStdWindowAndFrame(4u8, 1u8);
            ClearStdWindowAndFrame(5u8, 1u8);
            ClearStdWindowAndFrame(6u8, 1u8);
            ClearStdWindowAndFrame(7u8, 1u8);
            wirelessAdapterConnected = IsWirelessAdapterConnected();
            'l1: {
                let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                if __sw1 == 0i32 || !__matched {
                    'l2: {
                        let __sw2 = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32);
                        let __matched = __sw2 == 0i32 || __sw2 == 1i32;
                        if __sw2 == 0i32 || !__matched {
                            action = 0u8;
                            break 'l2;
                        }
                        if __sw2 == 1i32 {
                            action = 2u8;
                            break 'l2;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    'l3: {
                        let __sw3 = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32);
                        let __matched = __sw3 == 0i32 || __sw3 == 1i32 || __sw3 == 2i32;
                        if __sw3 == 0i32 || !__matched {
                            action = 1u8;
                            break 'l3;
                        }
                        if __sw3 == 1i32 {
                            action = 0u8;
                            break 'l3;
                        }
                        if __sw3 == 2i32 {
                            action = 2u8;
                            break 'l3;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    'l4: {
                        let __sw4 = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32);
                        let __matched =
                            __sw4 == 0i32 || __sw4 == 1i32 || __sw4 == 2i32 || __sw4 == 3i32;
                        if __sw4 == 0i32 || !__matched {
                            action = 1u8;
                            break 'l4;
                        }
                        if __sw4 == 1i32 {
                            action = 0u8;
                            break 'l4;
                        }
                        if __sw4 == 2i32 {
                            action = 3u8;
                            if !((wirelessAdapterConnected) != 0) {
                                action = 6u8;
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(0i16);
                            }
                            break 'l4;
                        }
                        if __sw4 == 3i32 {
                            action = 2u8;
                            break 'l4;
                        }
                    }
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    'l5: {
                        let __sw5 = ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32);
                        let __matched = __sw5 == 0i32
                            || __sw5 == 1i32
                            || __sw5 == 2i32
                            || __sw5 == 3i32
                            || __sw5 == 4i32;
                        if __sw5 == 0i32 || !__matched {
                            action = 1u8;
                            break 'l5;
                        }
                        if __sw5 == 1i32 {
                            action = 0u8;
                            break 'l5;
                        }
                        if __sw5 == 2i32 {
                            if (((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(15))
                            .read())
                                != 0
                            {
                                action = 3u8;
                                if !((wirelessAdapterConnected) != 0) {
                                    action = 6u8;
                                    (((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .write(0i16);
                                }
                            } else {
                                if (wirelessAdapterConnected) != 0 {
                                    action = 6u8;
                                    (((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .write(1i16);
                                } else {
                                    action = 5u8;
                                }
                            }
                            break 'l5;
                        }
                        if __sw5 == 3i32 {
                            if (wirelessAdapterConnected) != 0 {
                                action = 6u8;
                                (((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .write(2i16);
                            } else {
                                action = 4u8;
                            }
                            break 'l5;
                        }
                        if __sw5 == 4i32 {
                            action = 2u8;
                            break 'l5;
                        }
                    }
                    break 'l1;
                }
            }
            ChangeBgY(0u8, 0i32, 0u8);
            ChangeBgY(1u8, 0i32, 0u8);
            'l6: {
                let __sw6 = ((action) as i32);
                let __matched = __sw6 == 0i32
                    || __sw6 == 1i32
                    || __sw6 == 2i32
                    || __sw6 == 3i32
                    || __sw6 == 4i32
                    || __sw6 == 5i32
                    || __sw6 == 6i32;
                if __sw6 == 0i32 || !__matched {
                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).write(0u16);
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_NewGameBirchSpeech_Init));
                    break 'l6;
                }
                if __sw6 == 1i32 {
                    (((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).write(0u16);
                    (((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).write(0u16);
                    SetMainCallback2(Some(CB2_ContinueSavedGame));
                    DestroyTask(taskId);
                    break 'l6;
                }
                if __sw6 == 2i32 {
                    (((&raw mut gMain).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(CB2_ReinitMainMenu));
                    SetMainCallback2(Some(CB2_InitOptionMenu));
                    DestroyTask(taskId);
                    break 'l6;
                }
                if __sw6 == 3i32 {
                    SetMainCallback2(Some(CB2_InitMysteryGift));
                    DestroyTask(taskId);
                    break 'l6;
                }
                if __sw6 == 4i32 {
                    SetMainCallback2(Some(CB2_InitMysteryEventMenu));
                    DestroyTask(taskId);
                    break 'l6;
                }
                if __sw6 == 5i32 {
                    SetMainCallback2(Some(CB2_InitEReader));
                    DestroyTask(taskId);
                    break 'l6;
                }
                if __sw6 == 6i32 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_DisplayMainMenuInvalidActionError));
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(241))
                    .write(32767u16);
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(241))
                    .write(32767u16);
                    SetGpuReg(24u8, 0u16);
                    SetGpuReg(26u8, 0u16);
                    SetGpuReg(20u8, 0u16);
                    SetGpuReg(22u8, 0u16);
                    SetGpuReg(16u8, 0u16);
                    SetGpuReg(18u8, 0u16);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    return;
                }
            }
            FreeAllWindowBuffers();
            if ((action) as i32) != 2i32 {
                ((&raw mut sCurrItemAndOptionMenuCheck)
                    .cast::<u8>()
                    .cast::<u16>())
                .write(0u16);
            } else {
                let __p7 = (&raw mut sCurrItemAndOptionMenuCheck)
                    .cast::<u8>()
                    .cast::<u16>();
                (__p7).write((((((__p7).read()) as i32) | 32768i32) as u16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMainMenuBPressed(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 3i32
            {
                RemoveScrollIndicatorArrowPair(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(13))
                    .read()) as u8),
                );
            }
            ((&raw mut sCurrItemAndOptionMenuCheck)
                .cast::<u8>()
                .cast::<u16>())
            .write(0u16);
            FreeAllWindowBuffers();
            SetMainCallback2(Some(CB2_InitTitleScreen));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_DisplayMainMenuInvalidActionError(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32);
            if __sw1 == 0i32 {
                FillBgTilemapBufferRect_Palette0(
                    0u8,
                    0u16,
                    0u8,
                    0u8,
                    ((crate::c::div_i32(240i32, 8i32)) as u8),
                    ((crate::c::div_i32(160i32, 8i32)) as u8),
                );
                'l2: {
                    let __sw2 = (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32);
                    if __sw2 == 0i32 {
                        CreateMainMenuErrorWindow(
                            (&raw mut gText_WirelessNotConnected).cast::<u8>(),
                        );
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        CreateMainMenuErrorWindow((&raw mut gText_MysteryGiftCantUse).cast::<u8>());
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        CreateMainMenuErrorWindow(
                            (&raw mut gText_MysteryEventsCantUse).cast::<u8>(),
                        );
                        break 'l2;
                    }
                }
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                RunTextPrinters();
                if !((IsTextPrinterActive(7u8)) != 0) {
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 3i32)
                    != 0
                {
                    PlaySE(5u16);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HandleMainMenuBPressed));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn HighlightSelectedMainMenuItem(
    menuType: u8,
    selectedMenuItem: u8,
    isScrolled: i16,
) {
    unsafe {
        let mut menuType = menuType;
        let mut selectedMenuItem = selectedMenuItem;
        let mut isScrolled = isScrolled;
        SetGpuReg(64u8, 2535u16);
        'l1: {
            let __sw1 = ((menuType) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 || !__matched {
                'l2: {
                    let __sw2 = ((selectedMenuItem) as i32);
                    let __matched = __sw2 == 0i32 || __sw2 == 1i32;
                    if __sw2 == 0i32 || !__matched {
                        SetGpuReg(68u8, 287u16);
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        SetGpuReg(68u8, 8511u16);
                        break 'l2;
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                'l3: {
                    let __sw3 = ((selectedMenuItem) as i32);
                    let __matched = __sw3 == 0i32 || __sw3 == 1i32 || __sw3 == 2i32;
                    if __sw3 == 0i32 || !__matched {
                        SetGpuReg(68u8, 319u16);
                        break 'l3;
                    }
                    if __sw3 == 1i32 {
                        SetGpuReg(68u8, 16735u16);
                        break 'l3;
                    }
                    if __sw3 == 2i32 {
                        SetGpuReg(68u8, 24959u16);
                        break 'l3;
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                'l4: {
                    let __sw4 = ((selectedMenuItem) as i32);
                    let __matched =
                        __sw4 == 0i32 || __sw4 == 1i32 || __sw4 == 2i32 || __sw4 == 3i32;
                    if __sw4 == 0i32 || !__matched {
                        SetGpuReg(68u8, 319u16);
                        break 'l4;
                    }
                    if __sw4 == 1i32 {
                        SetGpuReg(68u8, 16735u16);
                        break 'l4;
                    }
                    if __sw4 == 2i32 {
                        SetGpuReg(68u8, 24959u16);
                        break 'l4;
                    }
                    if __sw4 == 3i32 {
                        SetGpuReg(68u8, 33183u16);
                        break 'l4;
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                'l5: {
                    let __sw5 = ((selectedMenuItem) as i32);
                    let __matched = __sw5 == 0i32
                        || __sw5 == 1i32
                        || __sw5 == 2i32
                        || __sw5 == 3i32
                        || __sw5 == 4i32;
                    if __sw5 == 0i32 || !__matched {
                        SetGpuReg(68u8, 319u16);
                        break 'l5;
                    }
                    if __sw5 == 1i32 {
                        if (isScrolled) != 0 {
                            SetGpuReg(68u8, 8511u16);
                        } else {
                            SetGpuReg(68u8, 16735u16);
                        }
                        break 'l5;
                    }
                    if __sw5 == 2i32 {
                        if (isScrolled) != 0 {
                            SetGpuReg(68u8, 16735u16);
                        } else {
                            SetGpuReg(68u8, 24959u16);
                        }
                        break 'l5;
                    }
                    if __sw5 == 3i32 {
                        if (isScrolled) != 0 {
                            SetGpuReg(68u8, 24959u16);
                        } else {
                            SetGpuReg(68u8, 33183u16);
                        }
                        break 'l5;
                    }
                    if __sw5 == 4i32 {
                        SetGpuReg(68u8, 33183u16);
                        break 'l5;
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_Init(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(0u8, 0u16);
        SetGpuReg(0u8, 4160u16);
        InitBgFromTemplate((&raw const sBirchBgTemplate).cast::<u8>().cast_mut());
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        LZ77UnCompVram(
            ((&raw const sBirchSpeechShadowGfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sBirchSpeechBgMap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100677632i32) as usize as *mut u8),
        );
        LoadPalette(
            ((&raw const sBirchSpeechBgPals).cast::<u8>().cast_mut()).cast::<u8>(),
            0u16,
            64u16,
        );
        LoadPalette(
            ((((&raw const sBirchSpeechBgGradientPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(8))
            .cast::<u8>(),
            1u16,
            16u16,
        );
        ScanlineEffect_Stop();
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetAllPicSprites();
        AddBirchSpeechObjects(taskId);
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_NewGameBirchSpeech_WaitToShowBirch));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(255i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(255i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(216i16);
        PlayBGM(374u16);
        ShowBg(0u8);
        ShowBg(1u8);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitToShowBirch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .read()) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(136i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(60i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 10u8);
            NewGameBirchSpeech_StartFadePlatformOut(taskId, 20u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(80i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_WaitForSpriteFadeInWelcome));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForSpriteFadeInWelcome(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(8))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (0u32) as i32,
            );
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .read())
                != 0
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                InitWindows(
                    ((&raw const sNewGameBirchSpeechTextWindows)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadMainMenuWindowFrameTiles(0u8, 243u16);
                LoadMessageBoxGfx(0u8, 252u16, 240u8);
                NewGameBirchSpeech_ShowDialogueWindow(0u8, 1u8);
                PutWindowTilemap(0u8);
                CopyWindowToVram(0u8, 2u8);
                NewGameBirchSpeech_ClearWindow(0u8);
                StringExpandPlaceholders(
                    (&raw mut gStringVar4).cast::<u8>(),
                    (&raw mut gText_Birch_Welcome).cast::<u8>(),
                );
                AddTextPrinterForMessage(1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_ThisIsAPokemon));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ThisIsAPokemon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (!((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0))
            && (!((RunTextPrintersAndIsPrinter0Active()) != 0))
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_MainSpeech));
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_ThisIsAPokemon).cast::<u8>(),
            );
            AddTextPrinterWithCallbackForMessage(
                1u8,
                Some(NewGameBirchSpeech_WaitForThisIsPokemonText),
            );
            ((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).write(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_MainSpeech(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Birch_MainSpeech).cast::<u8>(),
            );
            AddTextPrinterForMessage(1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_AndYouAre));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeechSub_InitPokeBall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .read()) as u8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
        .write(100i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
        .write(75i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        CreatePokeballSpriteToReleaseMon(
            spriteId,
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as u8),
            112u8,
            58u8,
            0u8,
            0u8,
            32u8,
            65535u32,
            295u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_NewGameBirchSpeechSub_WaitForLotad));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeechSub_WaitForLotad(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .read()) as i32) as isize
                * 68,
        );
        'l1: {
            let __sw1 = (((data).read()) as i32);
            if __sw1 == 0i32 {
                if core::mem::transmute::<_, usize>(
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .read(),
                ) != (SpriteCallbackDummy as *const () as usize)
                {
                    return;
                }
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                        as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .read()) as i32)
                    >= 96i32
                {
                    DestroyTask(taskId);
                    if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read())
                            as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read()) as i32)
                        < 16384i32
                    {
                        let __p2 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read())
                                as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(7);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                return;
            }
        }
        (data).write(((data).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .read()) as i32)
            < 16384i32
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((&raw mut sBirchSpeechMainTaskId).cast::<u8>().cast::<u8>()).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_AndYouAre(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            ((&raw mut sStartedPokeBallTask).cast::<u8>().cast::<u8>()).write(0u8);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Birch_AndYouAre).cast::<u8>(),
            );
            AddTextPrinterForMessage(1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_StartBirchLotadPlatformFade));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_StartBirchLotadPlatformFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(8))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2u8);
            NewGameBirchSpeech_StartFadePlatformIn(taskId, 1u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(64i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_SlidePlatformAway));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlidePlatformAway(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            != (-60i32)
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
            SetGpuReg(
                20u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write((-60i16));
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_StartPlayerFadeIn));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_StartPlayerFadeIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(8))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .read())
                != 0
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as u8);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .write(180i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .write(60i16);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (1u32) as i32,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((spriteId) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(0i16);
                NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2u8);
                NewGameBirchSpeech_StartFadePlatformOut(taskId, 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_WaitForPlayerFadeIn));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForPlayerFadeIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (0u32) as i32,
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_BoyOrGirl));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_BoyOrGirl(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        NewGameBirchSpeech_ClearWindow(0u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Birch_BoyOrGirl).cast::<u8>(),
        );
        AddTextPrinterForMessage(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_NewGameBirchSpeech_WaitToShowGenderMenu));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitToShowGenderMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            NewGameBirchSpeech_ShowGenderMenu();
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_ChooseGender));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ChooseGender(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut gender: i32 = ((NewGameBirchSpeech_ProcessGenderMenuInput()) as i32);
        let mut gender2: i32 = 0i32;
        'l1: {
            let __sw1 = gender;
            if __sw1 == 0i32 {
                PlaySE(5u16);
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                    .write(((gender) as u8));
                NewGameBirchSpeech_ClearGenderWindow(1u8, 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_WhatsYourName));
                break 'l1;
            }
            if __sw1 == 1i32 {
                PlaySE(5u16);
                ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                    .write(((gender) as u8));
                NewGameBirchSpeech_ClearGenderWindow(1u8, 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_WhatsYourName));
                break 'l1;
            }
        }
        gender2 = ((Menu_GetCursorPos()) as i32);
        if gender2
            != ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((gender2) as i16));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 0u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_SlideOutOldGenderSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlideOutOldGenderSprite(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            == 0i32
        {
            let __p1 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i32)
                != 0i32
            {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as u8);
            } else {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as u8);
            }
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(240i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(60i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((spriteId) as i16));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 0u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_SlideInNewGenderSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlideInNewGenderSprite(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        if ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .read()) as i32)
            > 180i32
        {
            let __p1 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(4i32)) as i16));
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(180i16);
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read())
                != 0
            {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (0u32) as i32,
                );
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_ChooseGender));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WhatsYourName(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        NewGameBirchSpeech_ClearWindow(0u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Birch_WhatsYourName).cast::<u8>(),
        );
        AddTextPrinterForMessage(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_NewGameBirchSpeech_WaitForWhatsYourNameToPrint));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForWhatsYourNameToPrint(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_WaitPressBeforeNameChoice));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitPressBeforeNameChoice(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0)
            || (((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 2i32)
                != 0)
        {
            BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_StartNamingScreen));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_StartNamingScreen(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            FreeAllWindowBuffers();
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as u16),
            );
            NewGameBirchSpeech_SetDefaultPlayerName(
                ((crate::c::rem_u32(
                    ((Random()) as u32),
                    (if crate::c::div_u32(80u32, 4u32) < crate::c::div_u32(80u32, 4u32) {
                        crate::c::div_u32(80u32, 4u32)
                    } else {
                        crate::c::div_u32(80u32, 4u32)
                    }),
                )) as u8),
            );
            DestroyTask(taskId);
            DoNamingScreen(
                0u8,
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                    as u16),
                0u16,
                0u32,
                Some(CB2_NewGameBirchSpeech_ReturnFromNamingScreen),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SoItsPlayerName(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        NewGameBirchSpeech_ClearWindow(0u8);
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_Birch_SoItsPlayer).cast::<u8>(),
        );
        AddTextPrinterForMessage(1u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_NewGameBirchSpeech_CreateNameYesNo));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_CreateNameYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
            CreateYesNoMenuParameterized(2u8, 1u8, 243u16, 223u16, 2u8, 15u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_ProcessNameYesNoMenu));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ProcessNameYesNoMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                PlaySE(5u16);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(1),
                    2,
                    2,
                    (1u32) as i32,
                );
                NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2u8);
                NewGameBirchSpeech_StartFadePlatformIn(taskId, 1u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_SlidePlatformAway2));
                break 'l1;
            }
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                PlaySE(5u16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_BoyOrGirl));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_SlidePlatformAway2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
            SetGpuReg(
                20u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16),
            );
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_ReshowBirchLotad));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ReshowBirchLotad(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .read()) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(136i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(60i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .read()) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(100i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(75i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2u8);
            NewGameBirchSpeech_StartFadePlatformOut(taskId, 1u8);
            NewGameBirchSpeech_ClearWindow(0u8);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Birch_YourePlayer).cast::<u8>(),
            );
            AddTextPrinterForMessage(1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(
                Task_NewGameBirchSpeech_WaitForSpriteFadeInAndTextPrinter,
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForSpriteFadeInAndTextPrinter(
    taskId: u8,
) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(8))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (0u32) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (0u32) as i32,
            );
            if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(8))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(1),
                    2,
                    2,
                    (1u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(9))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(1),
                    2,
                    2,
                    (1u32) as i32,
                );
                NewGameBirchSpeech_StartFadeOutTarget1InTarget2(taskId, 2u8);
                NewGameBirchSpeech_StartFadePlatformIn(taskId, 1u8);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(64i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_AreYouReady));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_AreYouReady(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(8))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .read())
                != 0
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7);
                (__p1).write(((__p1).read()).wrapping_sub(1));
                return;
            }
            if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32)
                != 0i32
            {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as u8);
            } else {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as u8);
            }
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(120i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(60i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                2,
                2,
                (1u32) as i32,
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((spriteId) as i16));
            NewGameBirchSpeech_StartFadeInTarget1OutTarget2(taskId, 2u8);
            NewGameBirchSpeech_StartFadePlatformOut(taskId, 1u8);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_Birch_AreYouReady).cast::<u8>(),
            );
            AddTextPrinterForMessage(1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_ShrinkPlayer));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ShrinkPlayer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read())
            != 0
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(1),
                2,
                2,
                (0u32) as i32,
            );
            if !((RunTextPrintersAndIsPrinter0Active()) != 0) {
                spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    0,
                    2,
                    (1u32) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(16)
                .cast::<*mut *mut u8>())
                .write(
                    ((&raw const sSpriteAffineAnimTable_PlayerShrink)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>(),
                );
                InitSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                );
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    0u8,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_MovePlayerDownWhileShrinking));
                BeginNormalPaletteFade(65535u32, 0i8, 0u8, 16u8, 0u16);
                FadeOutBGM(4u8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_NewGameBirchSpeech_WaitForPlayerShrink));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_WaitForPlayerShrink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_FadePlayerToWhite));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadePlayerToWhite(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_Null));
            SetGpuReg(0u8, 4160u16);
            BeginNormalPaletteFade(4294901760u32, 0i8, 0u8, 16u8, 65535u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_Cleanup));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_Cleanup(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            FreeAllWindowBuffers();
            FreeAndDestroyMonPicSprite(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as u16),
            );
            ResetAllPicSprites();
            SetMainCallback2(Some(CB2_NewGame));
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_NewGameBirchSpeech_ReturnFromNamingScreen() {
    unsafe {
        let mut taskId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        ResetBgsAndClearDma3BusyFlags(0u32);
        SetGpuReg(0u8, 0u16);
        SetGpuReg(0u8, 4160u16);
        InitBgsFromTemplates(
            0u8,
            ((&raw const sMainMenuBgTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((crate::c::div_u32(8u32, 4u32)) as u8),
        );
        InitBgFromTemplate((&raw const sBirchBgTemplate).cast::<u8>().cast_mut());
        SetVBlankCallback(None);
        SetGpuReg(12u8, 0u16);
        SetGpuReg(10u8, 0u16);
        SetGpuReg(8u8, 0u16);
        SetGpuReg(24u8, 0u16);
        SetGpuReg(26u8, 0u16);
        SetGpuReg(20u8, 0u16);
        SetGpuReg(22u8, 0u16);
        SetGpuReg(16u8, 0u16);
        SetGpuReg(18u8, 0u16);
        'l1: loop {
            'l2: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l3: loop {
                        'l4: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    100663296u32,
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            98304i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l3;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l1;
            }
        }
        'l5: loop {
            'l6: {
                {
                    let mut tmp: u32 = 0u32;
                    (&raw mut tmp).write_volatile(0u32);
                    'l7: loop {
                        'l8: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(1),
                                    117440512u32,
                                );
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2063597568i32)
                                        | crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(32i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l7;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l5;
            }
        }
        'l9: loop {
            'l10: {
                {
                    let mut tmp: u16 = 0u16;
                    (&raw mut tmp).write_volatile(0u16);
                    'l11: loop {
                        'l12: {
                            {
                                let mut dmaRegs: *mut u32 = ((67109076i32) as usize as *mut u32);
                                crate::c::volatile_write(dmaRegs, ((&raw mut tmp) as usize as u32));
                                crate::c::volatile_write((dmaRegs).wrapping_offset(1), 83886080u32);
                                crate::c::volatile_write(
                                    (dmaRegs).wrapping_offset(2),
                                    (((-2130706432i32)
                                        | crate::c::div_i32(
                                            1024i32,
                                            crate::c::div_i32(16i32, 8i32),
                                        )) as u32),
                                );
                                let _ = ((dmaRegs).wrapping_offset(2)).read_volatile();
                            }
                        }
                        if !((0i32) != 0) {
                            break 'l11;
                        }
                    }
                }
            }
            if !((0i32) != 0) {
                break 'l9;
            }
        }
        ResetPaletteFade();
        LZ77UnCompVram(
            ((&raw const sBirchSpeechShadowGfx)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100663296i32) as usize as *mut u8),
        );
        LZ77UnCompVram(
            ((&raw const sBirchSpeechBgMap)
                .cast::<u8>()
                .cast_mut()
                .cast::<u32>())
            .cast::<u32>(),
            ((100677632i32) as usize as *mut u8),
        );
        LoadPalette(
            ((&raw const sBirchSpeechBgPals).cast::<u8>().cast_mut()).cast::<u8>(),
            0u16,
            64u16,
        );
        LoadPalette(
            ((((&raw const sBirchSpeechBgGradientPal)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .wrapping_offset(1))
            .cast::<u8>(),
            1u16,
            16u16,
        );
        ResetTasks();
        taskId = CreateTask(
            Some(Task_NewGameBirchSpeech_ReturnFromNamingScreenShowTextbox),
            0u8,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(5i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write((-60i16));
        ScanlineEffect_Stop();
        ResetSpriteData();
        FreeAllSpritePalettes();
        ResetAllPicSprites();
        AddBirchSpeechObjects(taskId);
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            != 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(1i16);
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as u8);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(0i16);
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as u8);
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
        .write(180i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
        .write(60i16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((spriteId) as i16));
        SetGpuReg(20u8, 65476u16);
        BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
        SetGpuReg(64u8, 0u16);
        SetGpuReg(68u8, 0u16);
        SetGpuReg(72u8, 0u16);
        SetGpuReg(74u8, 0u16);
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        SetGpuReg(84u8, 0u16);
        ShowBg(0u8);
        ShowBg(1u8);
        {
            let mut imeTemp: u16 = 0u16;
            imeTemp = ((67109384i32) as usize as *mut u16).read_volatile();
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), 0u16);
            let __p1 = ((67109376i32) as usize as *mut u16);
            crate::c::volatile_write(__p1, (((((__p1).read_volatile()) as i32) | 1i32) as u16));
            crate::c::volatile_write(((67109384i32) as usize as *mut u16), imeTemp);
        }
        SetVBlankCallback(Some(VBlankCB_MainMenu));
        SetMainCallback2(Some(CB2_MainMenu));
        InitWindows(
            ((&raw const sNewGameBirchSpeechTextWindows)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        LoadMainMenuWindowFrameTiles(0u8, 243u16);
        LoadMessageBoxGfx(0u8, 252u16, 240u8);
        PutWindowTilemap(0u8);
        CopyWindowToVram(0u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Null(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_MovePlayerDownWhileShrinking(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y: u32 = 0u32;
        y = ((((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 16)
            .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
        .wrapping_add(49152i32)) as u32);
        ((sprite).wrapping_add(34).cast::<i16>()).write(((y >> 16) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((y) as i16));
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_CreateLotadSprite(x: u8, y: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        return ((CreateMonPicSprite_Affine(
            295u16,
            8u32,
            0u32,
            1u8,
            ((x) as i16),
            ((y) as i16),
            14u8,
            65535u16,
        )) as u8);
    }
}
pub(crate) unsafe extern "C" fn AddBirchSpeechObjects(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut birchSpriteId: u8 = 0u8;
        let mut lotadSpriteId: u8 = 0u8;
        let mut brendanSpriteId: u8 = 0u8;
        let mut maySpriteId: u8 = 0u8;
        birchSpriteId = AddNewGameBirchObject(136i16, 60i16, 1u8);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((birchSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Null));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((birchSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((birchSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(((birchSpriteId) as i16));
        lotadSpriteId = NewGameBirchSpeech_CreateLotadSprite(100u8, 75u8);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((lotadSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Null));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((lotadSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((lotadSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((lotadSpriteId) as i16));
        brendanSpriteId = CreateTrainerSprite(
            ((FacilityClassToPicIndex(60u16)) as u8),
            120i16,
            60i16,
            0u8,
            (&raw mut gDecompressionBuffer).cast::<u8>(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((brendanSpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Null));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((brendanSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((brendanSpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((brendanSpriteId) as i16));
        maySpriteId = CreateTrainerSprite(
            ((FacilityClassToPicIndex(63u16)) as u8),
            120i16,
            60i16,
            0u8,
            ((&raw mut gDecompressionBuffer).cast::<u8>())
                .wrapping_offset((crate::c::div_i32(4096i32, 2i32)) as isize),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((maySpriteId) as i32) as isize * 68))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_Null));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((maySpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((maySpriteId) as i32) as isize * 68))
            .wrapping_add(5),
            2,
            2,
            (0u16) as i32,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11))
        .write(((maySpriteId) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadeOutTarget1InTarget2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut alphaCoeff2: i32 = 0i32;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
            DestroyTask(taskId);
        } else {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read())
                != 0
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read(),
                );
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_sub(1));
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                alphaCoeff2 = (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    << 8);
                SetGpuReg(
                    82u8,
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(alphaCoeff2)) as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadeOutTarget1InTarget2(
    taskId: u8,
    delay: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut delay = delay;
        let mut taskId2: u8 = 0u8;
        SetGpuReg(80u8, 592u16);
        SetGpuReg(82u8, 16u16);
        SetGpuReg(84u8, 0u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadeOutTarget1InTarget2), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(16i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((delay) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((delay) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadeInTarget1OutTarget2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut alphaCoeff2: i32 = 0i32;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 16i32
        {
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
            DestroyTask(taskId);
        } else {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read())
                != 0
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read(),
                );
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_sub(1));
                alphaCoeff2 = (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    << 8);
                SetGpuReg(
                    82u8,
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_add(alphaCoeff2)) as u16),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadeInTarget1OutTarget2(
    taskId: u8,
    delay: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut delay = delay;
        let mut taskId2: u8 = 0u8;
        SetGpuReg(80u8, 592u16);
        SetGpuReg(82u8, 4096u16);
        SetGpuReg(84u8, 0u16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadeInTarget1OutTarget2), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(16i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((delay) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((delay) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadePlatformIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 8i32
            {
                DestroyTask(taskId);
            } else {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read())
                    != 0
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read(),
                    );
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    LoadPalette(
                        ((((&raw const sBirchSpeechBgGradientPal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize,
                        ))
                        .cast::<u8>(),
                        1u16,
                        16u16,
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadePlatformIn(taskId: u8, delay: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut delay = delay;
        let mut taskId2: u8 = 0u8;
        taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadePlatformIn), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(8i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((delay) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((delay) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_FadePlatformOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read())
            != 0
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 0i32
            {
                DestroyTask(taskId);
            } else {
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read())
                    != 0
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                } else {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .read(),
                    );
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    LoadPalette(
                        ((((&raw const sBirchSpeechBgGradientPal)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32) as isize,
                        ))
                        .cast::<u8>(),
                        1u16,
                        16u16,
                    );
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_StartFadePlatformOut(taskId: u8, delay: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut delay = delay;
        let mut taskId2: u8 = 0u8;
        taskId2 = CreateTask(Some(Task_NewGameBirchSpeech_FadePlatformOut), 0u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((taskId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(8i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(8i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((delay) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((delay) as i16));
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ShowGenderMenu() {
    unsafe {
        DrawMainMenuWindowBorder(
            (((&raw const sNewGameBirchSpeechTextWindows)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(8),
            243u16,
        );
        FillWindowPixelBuffer(1u8, 17u8);
        PrintMenuTable(
            1u8,
            ((crate::c::div_u32(16u32, 8u32)) as u8),
            ((&raw const sMenuActions_Gender).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(1u8, ((crate::c::div_u32(16u32, 8u32)) as u8), 0u8);
        PutWindowTilemap(1u8);
        CopyWindowToVram(1u8, 3u8);
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ProcessGenderMenuInput() -> i8 {
    unsafe {
        return Menu_ProcessInputNoWrap();
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_SetDefaultPlayerName(nameId: u8) {
    unsafe {
        let mut nameId = nameId;
        let mut name: *mut u8 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        if ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
            as i32)
            == 0i32
        {
            name = ((((&raw const sMalePresetNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((nameId) as i32) as isize))
            .read();
        } else {
            name = ((((&raw const sFemalePresetNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(((nameId) as i32) as isize))
            .read();
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(((name).wrapping_offset(((i) as i32) as isize)).read());
                }
                i = (i).wrapping_add(1);
            }
        }
        (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>()).wrapping_offset(7))
            .write(255u8);
    }
}
pub(crate) unsafe extern "C" fn CreateMainMenuErrorWindow(str: *mut u8) {
    unsafe {
        let mut str = str;
        FillWindowPixelBuffer(7u8, 17u8);
        AddTextPrinterParameterized(7u8, 1u8, str, 0u8, 1u8, 2u8, None);
        PutWindowTilemap(7u8);
        CopyWindowToVram(7u8, 2u8);
        DrawMainMenuWindowBorder(
            (((&raw const sWindowTemplates_MainMenu)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(56),
            469u16,
        );
        SetGpuReg(64u8, 2535u16);
        SetGpuReg(68u8, 29087u16);
    }
}
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegameText() {
    unsafe {
        MainMenu_FormatSavegamePlayer();
        MainMenu_FormatSavegamePokedex();
        MainMenu_FormatSavegameTime();
        MainMenu_FormatSavegameBadges();
    }
}
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegamePlayer() {
    unsafe {
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ContinueMenuPlayer).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            0u8,
            17u8,
            ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            ((GetStringRightAlignXOffset(
                1i32,
                (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                100i32,
            )) as u8),
            17u8,
            ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegameTime() {
    unsafe {
        let mut str = crate::ffi::Align4([0u8; 32]);
        let mut ptr: *mut u8 = core::ptr::null_mut();
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ContinueMenuTime).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            108u8,
            17u8,
            ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
        ptr = ConvertIntToDecimalStringN(
            (&raw mut str).cast::<u8>(),
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                .wrapping_add(14)
                .cast::<u16>())
            .read()) as i32),
            0i32,
            3u8,
        );
        ({
            let __t1 = ptr;
            ptr = (ptr).wrapping_offset(1);
            __t1
        })
        .write(240u8);
        ConvertIntToDecimalStringN(
            ptr,
            ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(16)).read())
                as i32),
            2i32,
            2u8,
        );
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            ((GetStringRightAlignXOffset(1i32, (&raw mut str).cast::<u8>(), 208i32)) as u8),
            17u8,
            ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut str).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegamePokedex() {
    unsafe {
        let mut str = crate::ffi::Align4([0u8; 32]);
        let mut dexCount: u16 = 0u16;
        if ((FlagGet(2145u16)) as i32) == 1i32 {
            if (IsNationalPokedexEnabled()) != 0 {
                dexCount = GetNationalPokedexCount(1u8);
            } else {
                dexCount = GetHoennPokedexCount(1u8);
            }
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_ContinueMenuPokedex).cast::<u8>(),
            );
            AddTextPrinterParameterized3(
                2u8,
                1u8,
                0u8,
                33u8,
                ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut gStringVar4).cast::<u8>(),
            );
            ConvertIntToDecimalStringN((&raw mut str).cast::<u8>(), ((dexCount) as i32), 0i32, 3u8);
            AddTextPrinterParameterized3(
                2u8,
                1u8,
                ((GetStringRightAlignXOffset(1i32, (&raw mut str).cast::<u8>(), 100i32)) as u8),
                33u8,
                ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
                (-1i8),
                (&raw mut str).cast::<u8>(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MainMenu_FormatSavegameBadges() {
    unsafe {
        let mut str = crate::ffi::Align4([0u8; 32]);
        let mut badgeCount: u8 = 0u8;
        let mut i: u32 = 0u32;
        {
            i = 2151u32;
            'l1: loop {
                if !(i < 2159u32) {
                    break 'l1;
                }
                'l2: {
                    if (FlagGet(((i) as u16))) != 0 {
                        badgeCount = (badgeCount).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_ContinueMenuBadges).cast::<u8>(),
        );
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            108u8,
            33u8,
            ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut gStringVar4).cast::<u8>(),
        );
        ConvertIntToDecimalStringN(
            (&raw mut str).cast::<u8>(),
            ((badgeCount) as i32),
            2i32,
            1u8,
        );
        AddTextPrinterParameterized3(
            2u8,
            1u8,
            ((GetStringRightAlignXOffset(1i32, (&raw mut str).cast::<u8>(), 208i32)) as u8),
            33u8,
            ((&raw const sTextColor_MenuInfo).cast::<u8>().cast_mut()).cast::<u8>(),
            (-1i8),
            (&raw mut str).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn LoadMainMenuWindowFrameTiles(bgId: u8, tileOffset: u16) {
    unsafe {
        let mut bgId = bgId;
        let mut tileOffset = tileOffset;
        LoadBgTiles(
            bgId,
            ((GetWindowFrameTilesPal(
                ((crate::c::bf_read(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                    3,
                    5,
                    false,
                ) as u16) as u8),
            ))
            .cast::<*mut u8>())
            .read(),
            288u16,
            tileOffset,
        );
        LoadPalette(
            (((GetWindowFrameTilesPal(
                ((crate::c::bf_read(
                    (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(20),
                    3,
                    5,
                    false,
                ) as u16) as u8),
            ))
            .wrapping_add(4)
            .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
            32u16,
            32u16,
        );
    }
}
pub(crate) unsafe extern "C" fn DrawMainMenuWindowBorder(template: *mut u8, baseTileNum: u16) {
    unsafe {
        let mut template = template;
        let mut baseTileNum = baseTileNum;
        let mut r9: u16 = (((1i32).wrapping_add(((baseTileNum) as i32))) as u16);
        let mut r10: u16 = (((2i32).wrapping_add(((baseTileNum) as i32))) as u16);
        let mut sp18: u16 = (((3i32).wrapping_add(((baseTileNum) as i32))) as u16);
        let mut spC: u16 = (((5i32).wrapping_add(((baseTileNum) as i32))) as u16);
        let mut sp10: u16 = (((6i32).wrapping_add(((baseTileNum) as i32))) as u16);
        let mut sp14: u16 = (((7i32).wrapping_add(((baseTileNum) as i32))) as u16);
        let mut r6: u16 = (((8i32).wrapping_add(((baseTileNum) as i32))) as u16);
        FillBgTilemapBufferRect(
            (template).read(),
            baseTileNum,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            r9,
            ((template).wrapping_add(1)).read(),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((template).wrapping_add(3)).read(),
            1u8,
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            r10,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            sp18,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((template).wrapping_add(2)).read(),
            1u8,
            ((template).wrapping_add(4)).read(),
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            spC,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((template).wrapping_add(2)).read(),
            1u8,
            ((template).wrapping_add(4)).read(),
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            sp10,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            1u8,
            1u8,
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            sp14,
            ((template).wrapping_add(1)).read(),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            ((template).wrapping_add(3)).read(),
            1u8,
            2u8,
        );
        FillBgTilemapBufferRect(
            (template).read(),
            r6,
            ((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32))) as u8),
            ((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32))) as u8),
            1u8,
            1u8,
            2u8,
        );
        CopyBgTilemapBufferToVram((template).read());
    }
}
pub(crate) unsafe extern "C" fn ClearMainMenuWindowTilemap(template: *mut u8) {
    unsafe {
        let mut template = template;
        FillBgTilemapBufferRect(
            (template).read(),
            0u16,
            ((((((template).wrapping_add(1)).read()) as i32).wrapping_sub(1i32)) as u8),
            ((((((template).wrapping_add(2)).read()) as i32).wrapping_sub(1i32)) as u8),
            (((((((template).wrapping_add(1)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(3)).read()) as i32)))
            .wrapping_add(1i32)) as u8),
            (((((((template).wrapping_add(2)).read()) as i32)
                .wrapping_add(((((template).wrapping_add(4)).read()) as i32)))
            .wrapping_add(1i32)) as u8),
            2u8,
        );
        CopyBgTilemapBufferToVram((template).read());
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ClearGenderWindowTilemap(
    bg: u8,
    x: u8,
    y: u8,
    width: u8,
    height: u8,
    unused: u8,
) {
    unsafe {
        let mut bg = bg;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut unused = unused;
        FillBgTilemapBufferRect(
            bg,
            0u16,
            ((((x) as i32).wrapping_add(255i32)) as u8),
            ((((y) as i32).wrapping_add(255i32)) as u8),
            ((((width) as i32).wrapping_add(2i32)) as u8),
            ((((height) as i32).wrapping_add(2i32)) as u8),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ClearGenderWindow(windowId: u8, copyToVram: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(windowId, Some(NewGameBirchSpeech_ClearGenderWindowTilemap));
        FillWindowPixelBuffer(windowId, 17u8);
        ClearWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ClearWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        let mut bgColor: u8 = GetFontAttribute(1u8, 6u8);
        let mut maxCharWidth: u8 = GetFontAttribute(1u8, 0u8);
        let mut maxCharHeight: u8 = GetFontAttribute(1u8, 1u8);
        let mut winWidth: u8 = ((GetWindowAttribute(windowId, 3u8)) as u8);
        let mut winHeight: u8 = ((GetWindowAttribute(windowId, 4u8)) as u8);
        FillWindowPixelRect(
            windowId,
            bgColor,
            0u16,
            0u16,
            ((((maxCharWidth) as i32).wrapping_mul(((winWidth) as i32))) as u16),
            ((((maxCharHeight) as i32).wrapping_mul(((winHeight) as i32))) as u16),
        );
        CopyWindowToVram(windowId, 2u8);
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_WaitForThisIsPokemonText(
    printer: *mut u8,
    renderCmd: u16,
) {
    unsafe {
        let mut printer = printer;
        let mut renderCmd = renderCmd;
        if (((((((printer).cast::<*mut u8>()).read()).wrapping_offset(-2)).read()) as i32) == 8i32)
            && (!((((&raw mut sStartedPokeBallTask).cast::<u8>().cast::<u8>()).read()) != 0))
        {
            ((&raw mut sStartedPokeBallTask).cast::<u8>().cast::<u8>()).write(1u8);
            CreateTask(Some(Task_NewGameBirchSpeechSub_InitPokeBall), 0u8);
        }
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
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut baseTileNum = baseTileNum;
        let mut baseBlock = baseBlock;
        let mut yesNoPalNum = yesNoPalNum;
        let mut winPalNum = winPalNum;
        let mut template = crate::ffi::Align4([0u8; 8]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(CreateWindowTemplate(
                0u8,
                ((((x) as i32).wrapping_add(1i32)) as u8),
                ((((y) as i32).wrapping_add(1i32)) as u8),
                5u8,
                4u8,
                winPalNum,
                baseBlock,
            ));
        CreateYesNoMenu(
            (&raw mut template).cast::<u8>(),
            baseTileNum,
            yesNoPalNum,
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn NewGameBirchSpeech_ShowDialogueWindow(
    windowId: u8,
    copyToVram: u8,
) {
    unsafe {
        let mut windowId = windowId;
        let mut copyToVram = copyToVram;
        CallWindowFunction(
            windowId,
            Some(NewGameBirchSpeech_CreateDialogueWindowBorder),
        );
        FillWindowPixelBuffer(windowId, 17u8);
        PutWindowTilemap(windowId);
        if ((copyToVram) as i32) == 1i32 {
            CopyWindowToVram(windowId, 3u8);
        }
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
    unsafe {
        let mut bg = bg;
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut palNum = palNum;
        FillBgTilemapBufferRect(
            bg,
            253u16,
            ((((x) as i32).wrapping_sub(2i32)) as u8),
            ((((y) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            255u16,
            ((((x) as i32).wrapping_sub(1i32)) as u8),
            ((((y) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            256u16,
            x,
            ((((y) as i32).wrapping_sub(1i32)) as u8),
            width,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            257u16,
            (((((x) as i32).wrapping_add(((width) as i32))).wrapping_sub(1i32)) as u8),
            ((((y) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            258u16,
            ((((x) as i32).wrapping_add(((width) as i32))) as u8),
            ((((y) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            259u16,
            ((((x) as i32).wrapping_sub(2i32)) as u8),
            y,
            1u8,
            5u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            261u16,
            ((((x) as i32).wrapping_sub(1i32)) as u8),
            y,
            ((((width) as i32).wrapping_add(1i32)) as u8),
            5u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            262u16,
            ((((x) as i32).wrapping_add(((width) as i32))) as u8),
            y,
            1u8,
            5u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            2301u16,
            ((((x) as i32).wrapping_sub(2i32)) as u8),
            ((((y) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            2303u16,
            ((((x) as i32).wrapping_sub(1i32)) as u8),
            ((((y) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            2304u16,
            x,
            ((((y) as i32).wrapping_add(((height) as i32))) as u8),
            ((((width) as i32).wrapping_sub(1i32)) as u8),
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            2305u16,
            (((((x) as i32).wrapping_add(((width) as i32))).wrapping_sub(1i32)) as u8),
            ((((y) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            palNum,
        );
        FillBgTilemapBufferRect(
            bg,
            2306u16,
            ((((x) as i32).wrapping_add(((width) as i32))) as u8),
            ((((y) as i32).wrapping_add(((height) as i32))) as u8),
            1u8,
            1u8,
            palNum,
        );
    }
}
pub(crate) unsafe extern "C" fn Task_NewGameBirchSpeech_ReturnFromNamingScreenShowTextbox(
    taskId: u8,
) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            <= 0i32
        {
            NewGameBirchSpeech_ShowDialogueWindow(0u8, 1u8);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_NewGameBirchSpeech_SoItsPlayerName));
        }
    }
}
