//! Translated from `src/start_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sWindowTemplate_SafariBalls sPyramidFloorNames sWindowTemplate_PyramidFloor sWindowTemplate_PyramidPeak sStartMenuItems sBgTemplates_LinkBattleSave sWindowTemplates_LinkBattleSave sSaveInfoWindowTemplate
#[allow(unused_imports)]
use crate::data::start_menu::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMenuCallback: Option<unsafe extern "C" fn() -> u8> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSafariBallsWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlePyramidFloorWindowId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStartMenuCursorPos: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNumStartMenuActions: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrentStartMenuActions: crate::ffi::Align4<[u8; 9]> =
    crate::ffi::Align4([0; 9]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInitStartMenuData: crate::ffi::Align4<[u8; 2]> = crate::ffi::Align4([0; 2]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSaveDialogCallback: Option<unsafe extern "C" fn() -> u8> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSaveDialogTimer: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavingComplete: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSaveInfoWindowId: u8 = 0u8;

unsafe extern "C" {
    static mut BattlePyramid_Retire: u8;
    static mut gDifferentSaveFile: u8;
    static mut gFieldCallback2: u8;
    static mut gLocalLinkPlayerId: u8;
    static mut gMain: u8;
    static mut gNumSafariBalls: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSaveFileStatus: u8;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_AlreadySavedFile: u8;
    static mut gText_BattlePyramidConfirmRest: u8;
    static mut gText_BattlePyramidConfirmRetire: u8;
    static mut gText_BattlePyramidFloor: u8;
    static mut gText_ConfirmSave: u8;
    static mut gText_DifferentSaveFile: u8;
    static mut gText_PlayerSavedGame: u8;
    static mut gText_SafariBallStock: u8;
    static mut gText_SaveError: u8;
    static mut gText_SavingBadges: u8;
    static mut gText_SavingDontTurnOff: u8;
    static mut gText_SavingDontTurnOffPower: u8;
    static mut gText_SavingPlayer: u8;
    static mut gText_SavingPokedex: u8;
    static mut gText_SavingTime: u8;
    static mut gWirelessCommType: u8;
    fn AddStartMenuWindow(a0: u8) -> u8;
    fn AddTextPrinterForMessage_2(a0: u8);
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn AddWindow(a0: *mut u8) -> u16;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn BufferSaveMenuText(a0: u8, a1: *mut u8, a2: u8);
    fn CB2_BagMenuFromStartMenu();
    fn CB2_InitOptionMenu();
    fn CB2_InitPokeNav();
    fn CB2_OpenPokedex();
    fn CB2_PartyMenuFromStartMenu();
    fn CB2_PyramidBagMenuFromStartMenu();
    fn CB2_ReturnToFieldWithOpenMenu();
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearContinueGameWarpStatus2();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearDialogWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn DisplayYesNoMenuDefaultYes();
    fn DisplayYesNoMenuWithDefault(a0: u8);
    fn DrawStdWindowFrame(a0: u8, a1: u8);
    fn DrawTextBorderOuter(a0: u8, a1: u16, a2: u8);
    fn EnableInterrupts(a0: u16);
    fn FadeScreen(a0: u8, a1: i8);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FreeAllWindowBuffers();
    fn FreezeObjectEvents();
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetNationalPokedexCount(a0: u8) -> u16;
    fn GetSafariZoneFlag() -> u32;
    fn GetStartMenuWindowId() -> u8;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn InBattlePike() -> u8;
    fn InMultiPartnerRoom() -> u8;
    fn InUnionRoom() -> u32;
    fn IncrementGameStat(a0: u8);
    fn InitBgsFromTemplates(a0: u8, a1: *mut u8, a2: u8);
    fn InitMenuNormal(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn InitWindows(a0: *mut u8) -> u16;
    fn IsOverworldLinkActive() -> u32;
    fn IsSEPlaying() -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn Link_AnyPartnersPlayingFRLG_JP() -> u32;
    fn LoadMessageBoxAndBorderGfx();
    fn LoadMessageBoxAndFrameGfx(a0: u8, a1: u8);
    fn LoadUserWindowBorderGfx_(a0: u8, a1: u16, a2: u8);
    fn LockPlayerFieldControls();
    fn Menu_LoadStdPalAt(a0: u16);
    fn Menu_MoveCursor(a0: i8) -> u8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PausePyramidChallenge();
    fn PlayRainStoppingSoundEffect();
    fn PlaySE(a0: u16);
    fn PlayerFreeze();
    fn PrintPlayerNameOnWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn PutWindowTilemap(a0: u8);
    fn RemoveStartMenuWindow();
    fn RemoveWindow(a0: u8);
    fn ResetBgsAndClearDma3BusyFlags(a0: u32);
    fn ResetPaletteFade();
    fn ResetSpriteData();
    fn ResetTasks();
    fn ReturnToFieldOpenStartMenu();
    fn RunTasks();
    fn RunTextPrintersAndIsPrinter0Active() -> u16;
    fn SafariZoneRetirePrompt();
    fn SaveMapView();
    fn ScanlineEffect_Clear();
    fn ScanlineEffect_Stop();
    fn ScriptContext_Enable();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn ScriptUnfreezeObjectEvents();
    fn SetContinueGameWarpStatusToDynamicWarp();
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetUsingUnionRoomStartMenu();
    fn SetVBlankCallback(a0: Option<unsafe extern "C" fn()>);
    fn ShowBg(a0: u8);
    fn ShowFrontierPass(a0: Option<unsafe extern "C" fn()>);
    fn ShowPlayerTrainerCard(a0: Option<unsafe extern "C" fn()>);
    fn ShowTrainerCardInLink(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn SoftResetInBattlePyramid();
    fn StopPlayerAvatar();
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn Task_LinkFullSave(a0: u8);
    fn TransferPlttBuffer();
    fn TrySavingData(a0: u8) -> u8;
    fn UnlockPlayerFieldControls();
    fn UpdatePaletteFade() -> u8;
    fn WriteSaveBlock1Sector() -> u8;
    fn WriteSaveBlock2() -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDexPokemonPokenavFlags() {
    unsafe {
        FlagSet(2145u16);
        FlagSet(2144u16);
        FlagSet(2146u16);
    }
}
pub(crate) unsafe extern "C" fn BuildStartMenuActions() {
    unsafe {
        ((&raw mut sNumStartMenuActions).cast::<u8>().cast::<u8>()).write(0u8);
        if IsOverworldLinkActive() == 1u32 {
            BuildLinkModeStartMenu();
        } else {
            if InUnionRoom() == 1u32 {
                BuildUnionRoomStartMenu();
            } else {
                if GetSafariZoneFlag() == 1u32 {
                    BuildSafariZoneStartMenu();
                } else {
                    if (InBattlePike()) != 0 {
                        BuildBattlePikeStartMenu();
                    } else {
                        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                            BuildBattlePyramidStartMenu();
                        } else {
                            if (InMultiPartnerRoom()) != 0 {
                                BuildMultiPartnerRoomStartMenu();
                            } else {
                                BuildNormalStartMenu();
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AddStartMenuAction(action: u8) {
    unsafe {
        let mut action = action;
        AppendToList(
            ((&raw mut sCurrentStartMenuActions).cast::<u8>()).cast::<u8>(),
            (&raw mut sNumStartMenuActions).cast::<u8>().cast::<u8>(),
            action,
        );
    }
}
pub(crate) unsafe extern "C" fn BuildNormalStartMenu() {
    unsafe {
        if ((FlagGet(2145u16)) as i32) == 1i32 {
            AddStartMenuAction(0u8);
        }
        if ((FlagGet(2144u16)) as i32) == 1i32 {
            AddStartMenuAction(1u8);
        }
        AddStartMenuAction(2u8);
        if ((FlagGet(2146u16)) as i32) == 1i32 {
            AddStartMenuAction(3u8);
        }
        AddStartMenuAction(4u8);
        AddStartMenuAction(5u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn BuildSafariZoneStartMenu() {
    unsafe {
        AddStartMenuAction(8u8);
        AddStartMenuAction(0u8);
        AddStartMenuAction(1u8);
        AddStartMenuAction(2u8);
        AddStartMenuAction(4u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn BuildLinkModeStartMenu() {
    unsafe {
        AddStartMenuAction(1u8);
        AddStartMenuAction(2u8);
        if ((FlagGet(2146u16)) as i32) == 1i32 {
            AddStartMenuAction(3u8);
        }
        AddStartMenuAction(9u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn BuildUnionRoomStartMenu() {
    unsafe {
        AddStartMenuAction(1u8);
        AddStartMenuAction(2u8);
        if ((FlagGet(2146u16)) as i32) == 1i32 {
            AddStartMenuAction(3u8);
        }
        AddStartMenuAction(4u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn BuildBattlePikeStartMenu() {
    unsafe {
        AddStartMenuAction(0u8);
        AddStartMenuAction(1u8);
        AddStartMenuAction(4u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn BuildBattlePyramidStartMenu() {
    unsafe {
        AddStartMenuAction(1u8);
        AddStartMenuAction(12u8);
        AddStartMenuAction(4u8);
        AddStartMenuAction(10u8);
        AddStartMenuAction(11u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn BuildMultiPartnerRoomStartMenu() {
    unsafe {
        AddStartMenuAction(1u8);
        AddStartMenuAction(4u8);
        AddStartMenuAction(6u8);
        AddStartMenuAction(7u8);
    }
}
pub(crate) unsafe extern "C" fn ShowSafariBallsWindow() {
    unsafe {
        ((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).write(
            ((AddWindow(
                (&raw const sWindowTemplate_SafariBalls)
                    .cast::<u8>()
                    .cast_mut(),
            )) as u8),
        );
        PutWindowTilemap(((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read());
        DrawStdWindowFrame(
            ((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw mut gNumSafariBalls).cast::<u8>()).read()) as i32),
            1i32,
            2u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_SafariBallStock).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read(),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn ShowPyramidFloorWindow() {
    unsafe {
        if (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
            .wrapping_add(1638)
            .cast::<u16>())
        .read()) as i32)
            == 7i32
        {
            ((&raw mut sBattlePyramidFloorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .write(
                ((AddWindow(
                    (&raw const sWindowTemplate_PyramidPeak)
                        .cast::<u8>()
                        .cast_mut(),
                )) as u8),
            );
        } else {
            ((&raw mut sBattlePyramidFloorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .write(
                ((AddWindow(
                    (&raw const sWindowTemplate_PyramidFloor)
                        .cast::<u8>()
                        .cast_mut(),
                )) as u8),
            );
        }
        PutWindowTilemap(
            ((&raw mut sBattlePyramidFloorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
        );
        DrawStdWindowFrame(
            ((&raw mut sBattlePyramidFloorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            0u8,
        );
        StringCopy(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((&raw const sPyramidFloorNames)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>())
            .wrapping_offset(
                (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(1612))
                    .wrapping_add(1638)
                    .cast::<u16>())
                .read()) as i32) as isize,
            ))
            .read(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_BattlePyramidFloor).cast::<u8>(),
        );
        AddTextPrinterParameterized(
            ((&raw mut sBattlePyramidFloorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            1u8,
            255u8,
            None,
        );
        CopyWindowToVram(
            ((&raw mut sBattlePyramidFloorWindowId)
                .cast::<u8>()
                .cast::<u8>())
            .read(),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn RemoveExtraStartMenuWindows() {
    unsafe {
        if (GetSafariZoneFlag()) != 0 {
            ClearStdWindowAndFrameToTransparent(
                ((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read(),
                0u8,
            );
            CopyWindowToVram(
                ((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read(),
                2u8,
            );
            RemoveWindow(((&raw mut sSafariBallsWindowId).cast::<u8>().cast::<u8>()).read());
        }
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            ClearStdWindowAndFrameToTransparent(
                ((&raw mut sBattlePyramidFloorWindowId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read(),
                0u8,
            );
            RemoveWindow(
                ((&raw mut sBattlePyramidFloorWindowId)
                    .cast::<u8>()
                    .cast::<u8>())
                .read(),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn PrintStartMenuActions(pIndex: *mut i8, count: u32) -> u32 {
    unsafe {
        let mut pIndex = pIndex;
        let mut count = count;
        let mut index: i8 = (pIndex).read();
        'l1: loop {
            'l2: {
                if core::mem::transmute::<_, usize>(
                    ((((((&raw const sStartMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((&raw mut sCurrentStartMenuActions).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((index) as i32) as isize))
                            .read()) as i32) as isize
                                * 8,
                        ))
                    .wrapping_add(4))
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                    .read(),
                ) == (StartMenuPlayerNameCallback as *const () as usize)
                {
                    PrintPlayerNameOnWindow(
                        GetStartMenuWindowId(),
                        (((((&raw const sStartMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sCurrentStartMenuActions).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(((index) as i32) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .cast::<*mut u8>())
                        .read(),
                        8u16,
                        (((((index) as i32) << 4).wrapping_add(9i32)) as u16),
                    );
                } else {
                    StringExpandPlaceholders(
                        (&raw mut gStringVar4).cast::<u8>(),
                        (((((&raw const sStartMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sCurrentStartMenuActions).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset(((index) as i32) as isize))
                                .read()) as i32) as isize
                                    * 8,
                            ))
                        .cast::<*mut u8>())
                        .read(),
                    );
                    AddTextPrinterParameterized(
                        GetStartMenuWindowId(),
                        1u8,
                        (&raw mut gStringVar4).cast::<u8>(),
                        8u8,
                        (((((index) as i32) << 4).wrapping_add(9i32)) as u8),
                        255u8,
                        None,
                    );
                }
                index = (index).wrapping_add(1);
                if ((index) as i32)
                    >= ((((&raw mut sNumStartMenuActions).cast::<u8>().cast::<u8>()).read()) as i32)
                {
                    (pIndex).write(index);
                    return 1u32;
                }
                count = (count).wrapping_sub(1);
            }
            if !(count != 0u32) {
                break 'l1;
            }
        }
        (pIndex).write(index);
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn InitStartMenuStep() -> u32 {
    unsafe {
        let mut state: i8 =
            (((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>()).read();
        'l1: {
            let __sw1 = ((state) as i32);
            if __sw1 == 0i32 {
                let __p2 = ((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                BuildStartMenuActions();
                let __p3 = ((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                LoadMessageBoxAndBorderGfx();
                DrawStdWindowFrame(
                    AddStartMenuWindow(
                        ((&raw mut sNumStartMenuActions).cast::<u8>().cast::<u8>()).read(),
                    ),
                    0u8,
                );
                ((((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>())
                    .wrapping_offset(1))
                .write(0i8);
                let __p4 = ((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (GetSafariZoneFlag()) != 0 {
                    ShowSafariBallsWindow();
                }
                if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
                    ShowPyramidFloorWindow();
                }
                let __p5 = ((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (PrintStartMenuActions(
                    (((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>())
                        .wrapping_offset(1),
                    2u32,
                )) != 0
                {
                    let __p6 =
                        ((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ((&raw mut sStartMenuCursorPos).cast::<u8>().cast::<u8>()).write(InitMenuNormal(
                    GetStartMenuWindowId(),
                    1u8,
                    0u8,
                    9u8,
                    16u8,
                    ((&raw mut sNumStartMenuActions).cast::<u8>().cast::<u8>()).read(),
                    ((&raw mut sStartMenuCursorPos).cast::<u8>().cast::<u8>()).read(),
                ));
                CopyWindowToVram(GetStartMenuWindowId(), 1u8);
                return 1u32;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn InitStartMenu() {
    unsafe {
        (((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>()).write(0i8);
        ((((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>())
            .wrapping_offset(1))
        .write(0i8);
        'l1: loop {
            if !(!((InitStartMenuStep()) != 0)) {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn StartMenuTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if InitStartMenuStep() == 1u32 {
            SwitchTaskToFollowupFunc(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn CreateStartMenuTask(
    followupFunc: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut followupFunc = followupFunc;
        let mut taskId: u8 = 0u8;
        (((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>()).write(0i8);
        ((((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>())
            .wrapping_offset(1))
        .write(0i8);
        taskId = CreateTask(Some(StartMenuTask), 80u8);
        SetTaskFuncWithFollowupFunc(taskId, Some(StartMenuTask), followupFunc);
    }
}
pub(crate) unsafe extern "C" fn FieldCB_ReturnToFieldStartMenu() -> u8 {
    unsafe {
        if InitStartMenuStep() == 0u32 {
            return 0u8;
        }
        ReturnToFieldOpenStartMenu();
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowReturnToFieldStartMenu() {
    unsafe {
        (((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>()).write(0i8);
        ((((&raw mut sInitStartMenuData).cast::<u8>().cast::<i8>()).cast::<i8>())
            .wrapping_offset(1))
        .write(0i8);
        ((&raw mut gFieldCallback2).cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(FieldCB_ReturnToFieldStartMenu));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ShowStartMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if InUnionRoom() == 1u32 {
                    SetUsingUnionRoomStartMenu();
                }
                ((&raw mut gMenuCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(HandleStartMenuInput));
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((&raw mut gMenuCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read())
                .unwrap_unchecked()()) as i32)
                    == 1i32
                {
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowStartMenu() {
    unsafe {
        if !((IsOverworldLinkActive()) != 0) {
            FreezeObjectEvents();
            PlayerFreeze();
            StopPlayerAvatar();
        }
        CreateStartMenuTask(Some(Task_ShowStartMenu));
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn HandleStartMenuInput() -> u8 {
    unsafe {
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 64i32)
            != 0
        {
            PlaySE(5u16);
            ((&raw mut sStartMenuCursorPos).cast::<u8>().cast::<u8>())
                .write(Menu_MoveCursor((-1i8)));
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 128i32)
            != 0
        {
            PlaySE(5u16);
            ((&raw mut sStartMenuCursorPos).cast::<u8>().cast::<u8>()).write(Menu_MoveCursor(1i8));
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            if core::mem::transmute::<_, usize>(
                ((((((&raw const sStartMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sCurrentStartMenuActions).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut sStartMenuCursorPos).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 8,
                    ))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read(),
            ) == (StartMenuPokedexCallback as *const () as usize)
            {
                if ((GetNationalPokedexCount(0u8)) as i32) == 0i32 {
                    return 0u8;
                }
            }
            ((&raw mut gMenuCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(
                ((((((&raw const sStartMenuItems).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((((((&raw mut sCurrentStartMenuActions).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(
                                ((((&raw mut sStartMenuCursorPos).cast::<u8>().cast::<u8>()).read())
                                    as i32) as isize,
                            ))
                        .read()) as i32) as isize
                            * 8,
                    ))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read(),
            );
            if (((core::mem::transmute::<_, usize>(
                ((&raw mut gMenuCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .read(),
            ) != (StartMenuSaveCallback as *const () as usize))
                && (core::mem::transmute::<_, usize>(
                    ((&raw mut gMenuCallback)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn() -> u8>>())
                    .read(),
                ) != (StartMenuExitCallback as *const () as usize)))
                && (core::mem::transmute::<_, usize>(
                    ((&raw mut gMenuCallback)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn() -> u8>>())
                    .read(),
                ) != (StartMenuSafariZoneRetireCallback as *const () as usize)))
                && (core::mem::transmute::<_, usize>(
                    ((&raw mut gMenuCallback)
                        .cast::<u8>()
                        .cast::<Option<unsafe extern "C" fn() -> u8>>())
                    .read(),
                ) != (StartMenuBattlePyramidRetireCallback as *const () as usize))
            {
                FadeScreen(1u8, 0i8);
            }
            return 0u8;
        }
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 10i32)
            != 0
        {
            RemoveExtraStartMenuWindows();
            HideStartMenu();
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuPokedexCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            IncrementGameStat(41u8);
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_OpenPokedex));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuPokemonCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_PartyMenuFromStartMenu));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuBagCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_BagMenuFromStartMenu));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuPokeNavCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_InitPokeNav));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuPlayerNameCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            if ((IsOverworldLinkActive()) != 0) || ((InUnionRoom()) != 0) {
                ShowPlayerTrainerCard(Some(CB2_ReturnToFieldWithOpenMenu));
            } else {
                if (FlagGet(2258u16)) != 0 {
                    ShowFrontierPass(Some(CB2_ReturnToFieldWithOpenMenu));
                } else {
                    ShowPlayerTrainerCard(Some(CB2_ReturnToFieldWithOpenMenu));
                }
            }
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuSaveCallback() -> u8 {
    unsafe {
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            RemoveExtraStartMenuWindows();
        }
        ((&raw mut gMenuCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(SaveStartCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuOptionCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_InitOptionMenu));
            (((&raw mut gMain).cast::<u8>())
                .wrapping_add(8)
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ReturnToFieldWithOpenMenu));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuExitCallback() -> u8 {
    unsafe {
        RemoveExtraStartMenuWindows();
        HideStartMenu();
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuSafariZoneRetireCallback() -> u8 {
    unsafe {
        RemoveExtraStartMenuWindows();
        HideStartMenu();
        SafariZoneRetirePrompt();
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuLinkModePlayerNameCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            CleanupOverworldWindowsAndTilemaps();
            ShowTrainerCardInLink(
                ((&raw mut gLocalLinkPlayerId).cast::<u8>()).read(),
                Some(CB2_ReturnToFieldWithOpenMenu),
            );
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn StartMenuBattlePyramidRetireCallback() -> u8 {
    unsafe {
        ((&raw mut gMenuCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(BattlePyramidRetireStartCallback));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBattlePyramidStartMenu() {
    unsafe {
        ClearDialogWindowAndFrameToTransparent(0u8, 0u8);
        ScriptUnfreezeObjectEvents();
        CreateStartMenuTask(Some(Task_ShowStartMenu));
        LockPlayerFieldControls();
    }
}
pub(crate) unsafe extern "C" fn StartMenuBattlePyramidBagCallback() -> u8 {
    unsafe {
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            PlayRainStoppingSoundEffect();
            RemoveExtraStartMenuWindows();
            CleanupOverworldWindowsAndTilemaps();
            SetMainCallback2(Some(CB2_PyramidBagMenuFromStartMenu));
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveStartCallback() -> u8 {
    unsafe {
        InitSave();
        ((&raw mut gMenuCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(SaveCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveCallback() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((RunSaveCallback()) as i32);
            if __sw1 == 0i32 {
                return 0u8;
            }
            if __sw1 == 2i32 {
                ClearDialogWindowAndFrameToTransparent(0u8, 0u8);
                InitStartMenu();
                ((&raw mut gMenuCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(HandleStartMenuInput));
                return 0u8;
            }
            if __sw1 == 1i32 || __sw1 == 3i32 {
                ClearDialogWindowAndFrameToTransparent(0u8, 1u8);
                ScriptUnfreezeObjectEvents();
                UnlockPlayerFieldControls();
                SoftResetInBattlePyramid();
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireStartCallback() -> u8 {
    unsafe {
        InitBattlePyramidRetire();
        ((&raw mut gMenuCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(BattlePyramidRetireCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireReturnCallback() -> u8 {
    unsafe {
        InitStartMenu();
        ((&raw mut gMenuCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(HandleStartMenuInput));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireCallback() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((RunSaveCallback()) as i32);
            if __sw1 == 1i32 {
                RemoveExtraStartMenuWindows();
                ((&raw mut gMenuCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(BattlePyramidRetireReturnCallback));
                return 0u8;
            }
            if __sw1 == 0i32 {
                return 0u8;
            }
            if __sw1 == 2i32 {
                ClearDialogWindowAndFrameToTransparent(0u8, 1u8);
                ScriptUnfreezeObjectEvents();
                UnlockPlayerFieldControls();
                ScriptContext_SetupScript((&raw mut BattlePyramid_Retire).cast::<u8>());
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitSave() {
    unsafe {
        SaveMapView();
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(SaveConfirmSaveCallback));
        ((&raw mut sSavingComplete).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn RunSaveCallback() -> u8 {
    unsafe {
        if ((RunTextPrintersAndIsPrinter0Active()) as i32) == 1i32 {
            return 0u8;
        }
        ((&raw mut sSavingComplete).cast::<u8>().cast::<u8>()).write(0u8);
        return (((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .read())
        .unwrap_unchecked()();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveGame() {
    unsafe {
        InitSave();
        CreateTask(Some(SaveGameTask), 80u8);
    }
}
pub(crate) unsafe extern "C" fn ShowSaveMessage(
    message: *mut u8,
    saveCallback: Option<unsafe extern "C" fn() -> u8>,
) {
    unsafe {
        let mut message = message;
        let mut saveCallback = saveCallback;
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), message);
        LoadMessageBoxAndFrameGfx(0u8, 1u8);
        AddTextPrinterForMessage_2(1u8);
        ((&raw mut sSavingComplete).cast::<u8>().cast::<u8>()).write(1u8);
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(saveCallback);
    }
}
pub(crate) unsafe extern "C" fn SaveGameTask(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut status: u8 = RunSaveCallback();
        'l1: {
            let __sw1 = ((status) as i32);
            if __sw1 == 2i32 || __sw1 == 3i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((status) as u16));
                break 'l1;
            }
            if __sw1 == 0i32 {
                return;
            }
        }
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
pub(crate) unsafe extern "C" fn HideSaveMessageWindow() {
    unsafe {
        ClearDialogWindowAndFrame(0u8, 1u8);
    }
}
pub(crate) unsafe extern "C" fn HideSaveInfoWindow() {
    unsafe {
        RemoveSaveInfoWindow();
    }
}
pub(crate) unsafe extern "C" fn SaveStartTimer() {
    unsafe {
        ((&raw mut sSaveDialogTimer).cast::<u8>().cast::<u8>()).write(60u8);
    }
}
pub(crate) unsafe extern "C" fn SaveSuccesTimer() -> u8 {
    unsafe {
        let __p1 = (&raw mut sSaveDialogTimer).cast::<u8>().cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(44)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            PlaySE(5u16);
            return 1u8;
        }
        if ((((&raw mut sSaveDialogTimer).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveErrorTimer() -> u8 {
    unsafe {
        if ((((&raw mut sSaveDialogTimer).cast::<u8>().cast::<u8>()).read()) as i32) != 0i32 {
            let __p1 = (&raw mut sSaveDialogTimer).cast::<u8>().cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(44)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveConfirmSaveCallback() -> u8 {
    unsafe {
        ClearStdWindowAndFrame(GetStartMenuWindowId(), 0u8);
        RemoveStartMenuWindow();
        ShowSaveInfoWindow();
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            ShowSaveMessage(
                (&raw mut gText_BattlePyramidConfirmRest).cast::<u8>(),
                Some(SaveYesNoCallback),
            );
        } else {
            ShowSaveMessage(
                (&raw mut gText_ConfirmSave).cast::<u8>(),
                Some(SaveYesNoCallback),
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveYesNoCallback() -> u8 {
    unsafe {
        DisplayYesNoMenuDefaultYes();
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(SaveConfirmInputCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveConfirmInputCallback() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                'l2: {
                    let __sw2 = ((((&raw mut gSaveFileStatus).cast::<u16>()).read()) as i32);
                    let __matched = __sw2 == 0i32 || __sw2 == 2i32;
                    if __sw2 == 0i32 || __sw2 == 2i32 {
                        if ((((&raw mut gDifferentSaveFile).cast::<u8>()).read()) as i32) == 0i32 {
                            ((&raw mut sSaveDialogCallback)
                                .cast::<u8>()
                                .cast::<Option<unsafe extern "C" fn() -> u8>>())
                            .write(Some(SaveFileExistsCallback));
                            return 0u8;
                        }
                        ((&raw mut sSaveDialogCallback)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn() -> u8>>())
                        .write(Some(SaveSavingMessageCallback));
                        return 0u8;
                    }
                    if !__matched {
                        ((&raw mut sSaveDialogCallback)
                            .cast::<u8>()
                            .cast::<Option<unsafe extern "C" fn() -> u8>>())
                        .write(Some(SaveFileExistsCallback));
                        return 0u8;
                    }
                }
            }
            if __fall || __sw1 == (-1i32) || __sw1 == 1i32 {
                __fall = true;
                HideSaveInfoWindow();
                HideSaveMessageWindow();
                return 2u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveFileExistsCallback() -> u8 {
    unsafe {
        if ((((&raw mut gDifferentSaveFile).cast::<u8>()).read()) as i32) == 1i32 {
            ShowSaveMessage(
                (&raw mut gText_DifferentSaveFile).cast::<u8>(),
                Some(SaveConfirmOverwriteDefaultNoCallback),
            );
        } else {
            ShowSaveMessage(
                (&raw mut gText_AlreadySavedFile).cast::<u8>(),
                Some(SaveConfirmOverwriteCallback),
            );
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveConfirmOverwriteDefaultNoCallback() -> u8 {
    unsafe {
        DisplayYesNoMenuWithDefault(1u8);
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(SaveOverwriteInputCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveConfirmOverwriteCallback() -> u8 {
    unsafe {
        DisplayYesNoMenuDefaultYes();
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(SaveOverwriteInputCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveOverwriteInputCallback() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                ((&raw mut sSaveDialogCallback)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn() -> u8>>())
                .write(Some(SaveSavingMessageCallback));
                return 0u8;
            }
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                HideSaveInfoWindow();
                HideSaveMessageWindow();
                return 2u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveSavingMessageCallback() -> u8 {
    unsafe {
        ShowSaveMessage(
            (&raw mut gText_SavingDontTurnOff).cast::<u8>(),
            Some(SaveDoSaveCallback),
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveDoSaveCallback() -> u8 {
    unsafe {
        let mut saveStatus: u8 = 0u8;
        IncrementGameStat(0u8);
        PausePyramidChallenge();
        if ((((&raw mut gDifferentSaveFile).cast::<u8>()).read()) as i32) == 1i32 {
            saveStatus = TrySavingData(4u8);
            ((&raw mut gDifferentSaveFile).cast::<u8>()).write(0u8);
        } else {
            saveStatus = TrySavingData(0u8);
        }
        if ((saveStatus) as i32) == 1i32 {
            ShowSaveMessage(
                (&raw mut gText_PlayerSavedGame).cast::<u8>(),
                Some(SaveSuccessCallback),
            );
        } else {
            ShowSaveMessage(
                (&raw mut gText_SaveError).cast::<u8>(),
                Some(SaveErrorCallback),
            );
        }
        SaveStartTimer();
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveSuccessCallback() -> u8 {
    unsafe {
        if !((IsTextPrinterActive(0u8)) != 0) {
            PlaySE(55u16);
            ((&raw mut sSaveDialogCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(SaveReturnSuccessCallback));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveReturnSuccessCallback() -> u8 {
    unsafe {
        if (!((IsSEPlaying()) != 0)) && ((SaveSuccesTimer()) != 0) {
            HideSaveInfoWindow();
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn SaveErrorCallback() -> u8 {
    unsafe {
        if !((IsTextPrinterActive(0u8)) != 0) {
            PlaySE(22u16);
            ((&raw mut sSaveDialogCallback)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn() -> u8>>())
            .write(Some(SaveReturnErrorCallback));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SaveReturnErrorCallback() -> u8 {
    unsafe {
        if !((SaveErrorTimer()) != 0) {
            return 0u8;
        } else {
            HideSaveInfoWindow();
            return 3u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn InitBattlePyramidRetire() {
    unsafe {
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(BattlePyramidConfirmRetireCallback));
        ((&raw mut sSavingComplete).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidConfirmRetireCallback() -> u8 {
    unsafe {
        ClearStdWindowAndFrame(GetStartMenuWindowId(), 0u8);
        RemoveStartMenuWindow();
        ShowSaveMessage(
            (&raw mut gText_BattlePyramidConfirmRetire).cast::<u8>(),
            Some(BattlePyramidRetireYesNoCallback),
        );
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireYesNoCallback() -> u8 {
    unsafe {
        DisplayYesNoMenuWithDefault(1u8);
        ((&raw mut sSaveDialogCallback)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .write(Some(BattlePyramidRetireInputCallback));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireInputCallback() -> u8 {
    unsafe {
        'l1: {
            let __sw1 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw1 == 0i32 {
                return 2u8;
            }
            if __sw1 == (-1i32) || __sw1 == 1i32 {
                HideSaveMessageWindow();
                return 1u8;
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn VBlankCB_LinkBattleSave() {
    unsafe {
        TransferPlttBuffer();
    }
}
pub(crate) unsafe extern "C" fn InitSaveWindowAfterLinkBattle(state: *mut u8) -> u32 {
    unsafe {
        let mut state = state;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(0u8, 0u16);
                SetVBlankCallback(None);
                ScanlineEffect_Stop();
                'l2: loop {
                    'l3: {
                        {
                            let mut _dest: *mut u16 = ((83886080i32) as usize as *mut u16);
                            let mut _size: u32 = 1024u32;
                            'l4: loop {
                                'l5: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l6: loop {
                                            'l7: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2164260864u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l6;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l4;
                                }
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                {
                    let mut _dest: *mut u8 = ((100663296i32) as usize as *mut u8);
                    let mut _size: u32 = 98304u32;
                    'l8: loop {
                        if !((1i32) != 0) {
                            break 'l8;
                        }
                        'l9: loop {
                            'l10: {
                                {
                                    let mut tmp: u16 = 0u16;
                                    (&raw mut tmp).write_volatile(0u16);
                                    'l11: loop {
                                        'l12: {
                                            {
                                                let mut dmaRegs: *mut u32 =
                                                    ((67109076i32) as usize as *mut u32);
                                                crate::c::volatile_write(
                                                    dmaRegs,
                                                    ((&raw mut tmp) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(1),
                                                    ((_dest) as usize as u32),
                                                );
                                                crate::c::volatile_write(
                                                    (dmaRegs).wrapping_offset(2),
                                                    (((-2130706432i32)
                                                        | crate::c::div_i32(
                                                            4096i32,
                                                            crate::c::div_i32(16i32, 8i32),
                                                        ))
                                                        as u32),
                                                );
                                                let _ =
                                                    ((dmaRegs).wrapping_offset(2)).read_volatile();
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
                        _dest = (_dest).wrapping_offset(4096);
                        _size = (_size).wrapping_sub(4096u32);
                        if _size <= 4096u32 {
                            'l13: loop {
                                'l14: {
                                    {
                                        let mut tmp: u16 = 0u16;
                                        (&raw mut tmp).write_volatile(0u16);
                                        'l15: loop {
                                            'l16: {
                                                {
                                                    let mut dmaRegs: *mut u32 =
                                                        ((67109076i32) as usize as *mut u32);
                                                    crate::c::volatile_write(
                                                        dmaRegs,
                                                        ((&raw mut tmp) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(1),
                                                        ((_dest) as usize as u32),
                                                    );
                                                    crate::c::volatile_write(
                                                        (dmaRegs).wrapping_offset(2),
                                                        (2164260864u32
                                                            | crate::c::div_u32(
                                                                _size,
                                                                ((crate::c::div_i32(16i32, 8i32))
                                                                    as u32),
                                                            )),
                                                    );
                                                    let _ = ((dmaRegs).wrapping_offset(2))
                                                        .read_volatile();
                                                }
                                            }
                                            if !((0i32) != 0) {
                                                break 'l15;
                                            }
                                        }
                                    }
                                }
                                if !((0i32) != 0) {
                                    break 'l13;
                                }
                            }
                            break 'l8;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ResetSpriteData();
                ResetTasks();
                ResetPaletteFade();
                ScanlineEffect_Clear();
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetBgsAndClearDma3BusyFlags(0u32);
                InitBgsFromTemplates(
                    0u8,
                    ((&raw const sBgTemplates_LinkBattleSave)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                    ((crate::c::div_u32(4u32, 4u32)) as u8),
                );
                InitWindows(
                    ((&raw const sWindowTemplates_LinkBattleSave)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                LoadUserWindowBorderGfx_(0u8, 8u16, 224u8);
                Menu_LoadStdPalAt(240u16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ShowBg(0u8);
                BlendPalettes(4294967295u32, 16u8, 0u16);
                SetVBlankCallback(Some(VBlankCB_LinkBattleSave));
                EnableInterrupts(1u16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                return 1u32;
            }
        }
        (state).write(((state).read()).wrapping_add(1));
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetUpSaveAfterLinkBattle() {
    unsafe {
        if (InitSaveWindowAfterLinkBattle(((&raw mut gMain).cast::<u8>()).wrapping_add(1080))) != 0
        {
            CreateTask(Some(Task_SaveAfterLinkBattle), 80u8);
            SetMainCallback2(Some(CB2_SaveAfterLinkBattle));
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_SaveAfterLinkBattle() {
    unsafe {
        RunTasks();
        UpdatePaletteFade();
    }
}
pub(crate) unsafe extern "C" fn Task_SaveAfterLinkBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut state: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
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
            'l1: {
                let __sw1 = (((state).read()) as i32);
                if __sw1 == 0i32 {
                    FillWindowPixelBuffer(0u8, 17u8);
                    AddTextPrinterParameterized2(
                        0u8,
                        1u8,
                        (&raw mut gText_SavingDontTurnOffPower).cast::<u8>(),
                        255u8,
                        None,
                        2u8,
                        1u8,
                        3u8,
                    );
                    DrawTextBorderOuter(0u8, 8u16, 14u8);
                    PutWindowTilemap(0u8);
                    CopyWindowToVram(0u8, 3u8);
                    BeginNormalPaletteFade(4294967295u32, 0i8, 16u8, 0u8, 0u16);
                    if (((((&raw mut gWirelessCommType).cast::<u8>()).read()) as i32) != 0i32)
                        && ((InUnionRoom()) != 0)
                    {
                        if (Link_AnyPartnersPlayingFRLG_JP()) != 0 {
                            (state).write(1i16);
                        } else {
                            (state).write(5i16);
                        }
                    } else {
                        ((&raw mut gSoftResetDisabled).cast::<u8>()).write(1u8);
                        (state).write(1i16);
                    }
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    SetContinueGameWarpStatusToDynamicWarp();
                    WriteSaveBlock2();
                    (state).write(2i16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    if (WriteSaveBlock1Sector()) != 0 {
                        ClearContinueGameWarpStatus2();
                        (state).write(3i16);
                        ((&raw mut gSoftResetDisabled).cast::<u8>()).write(0u8);
                    }
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    BeginNormalPaletteFade(4294967295u32, 0i8, 0u8, 16u8, 0u16);
                    (state).write(4i16);
                    break 'l1;
                }
                if __sw1 == 4i32 {
                    FreeAllWindowBuffers();
                    SetMainCallback2(
                        (((&raw mut gMain).cast::<u8>())
                            .wrapping_add(8)
                            .cast::<Option<unsafe extern "C" fn()>>())
                        .read(),
                    );
                    DestroyTask(taskId);
                    break 'l1;
                }
                if __sw1 == 5i32 {
                    CreateTask(Some(Task_LinkFullSave), 5u8);
                    (state).write(6i16);
                    break 'l1;
                }
                if __sw1 == 6i32 {
                    if !((FuncIsActiveTask(Some(Task_LinkFullSave))) != 0) {
                        (state).write(3i16);
                    }
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowSaveInfoWindow() {
    unsafe {
        let mut saveInfoWindow = crate::ffi::Align4([0u8; 8]);
        (&raw mut saveInfoWindow)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (&raw const sSaveInfoWindowTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<8>>()
                    .read_unaligned(),
            );
        let mut gender: u8 = 0u8;
        let mut color: u8 = 0u8;
        let mut xOffset: u32 = 0u32;
        let mut yOffset: u32 = 0u32;
        if !((FlagGet(2145u16)) != 0) {
            let __p1 = ((&raw mut saveInfoWindow).cast::<u8>()).wrapping_add(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as u8));
        }
        ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>())
            .write(((AddWindow((&raw mut saveInfoWindow).cast::<u8>())) as u8));
        DrawStdWindowFrame(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        gender = ((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read();
        color = 4u8;
        if ((gender) as i32) == 0i32 {
            color = 8u8;
        }
        yOffset = 1u32;
        BufferSaveMenuText(3u8, (&raw mut gStringVar4).cast::<u8>(), 6u8);
        AddTextPrinterParameterized(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            0u8,
            ((yOffset) as u8),
            255u8,
            None,
        );
        yOffset = (yOffset).wrapping_add(16u32);
        AddTextPrinterParameterized(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gText_SavingPlayer).cast::<u8>(),
            0u8,
            ((yOffset) as u8),
            255u8,
            None,
        );
        BufferSaveMenuText(0u8, (&raw mut gStringVar4).cast::<u8>(), color);
        xOffset = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 112i32))
            as u32);
        PrintPlayerNameOnWindow(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            (&raw mut gStringVar4).cast::<u8>(),
            ((xOffset) as u16),
            ((yOffset) as u16),
        );
        yOffset = (yOffset).wrapping_add(16u32);
        AddTextPrinterParameterized(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gText_SavingBadges).cast::<u8>(),
            0u8,
            ((yOffset) as u8),
            255u8,
            None,
        );
        BufferSaveMenuText(4u8, (&raw mut gStringVar4).cast::<u8>(), color);
        xOffset = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 112i32))
            as u32);
        AddTextPrinterParameterized(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((xOffset) as u8),
            ((yOffset) as u8),
            255u8,
            None,
        );
        if ((FlagGet(2145u16)) as i32) == 1i32 {
            yOffset = (yOffset).wrapping_add(16u32);
            AddTextPrinterParameterized(
                ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut gText_SavingPokedex).cast::<u8>(),
                0u8,
                ((yOffset) as u8),
                255u8,
                None,
            );
            BufferSaveMenuText(1u8, (&raw mut gStringVar4).cast::<u8>(), color);
            xOffset =
                ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 112i32))
                    as u32);
            AddTextPrinterParameterized(
                ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                ((xOffset) as u8),
                ((yOffset) as u8),
                255u8,
                None,
            );
        }
        yOffset = (yOffset).wrapping_add(16u32);
        AddTextPrinterParameterized(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gText_SavingTime).cast::<u8>(),
            0u8,
            ((yOffset) as u8),
            255u8,
            None,
        );
        BufferSaveMenuText(2u8, (&raw mut gStringVar4).cast::<u8>(), color);
        xOffset = ((GetStringRightAlignXOffset(1i32, (&raw mut gStringVar4).cast::<u8>(), 112i32))
            as u32);
        AddTextPrinterParameterized(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            1u8,
            (&raw mut gStringVar4).cast::<u8>(),
            ((xOffset) as u8),
            ((yOffset) as u8),
            255u8,
            None,
        );
        CopyWindowToVram(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn RemoveSaveInfoWindow() {
    unsafe {
        ClearStdWindowAndFrame(
            ((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read(),
            0u8,
        );
        RemoveWindow(((&raw mut sSaveInfoWindowId).cast::<u8>().cast::<u8>()).read());
    }
}
pub(crate) unsafe extern "C" fn Task_WaitForBattleTowerLinkSave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((FuncIsActiveTask(Some(Task_LinkFullSave))) != 0) {
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveForBattleTowerLink() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(Task_LinkFullSave), 5u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(1i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((CreateTask(Some(Task_WaitForBattleTowerLinkSave), 6u8)) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((taskId) as i16));
    }
}
pub(crate) unsafe extern "C" fn HideStartMenuWindow() {
    unsafe {
        ClearStdWindowAndFrame(GetStartMenuWindowId(), 1u8);
        RemoveStartMenuWindow();
        ScriptUnfreezeObjectEvents();
        UnlockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideStartMenu() {
    unsafe {
        PlaySE(5u16);
        HideStartMenuWindow();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AppendToList(list: *mut u8, pos: *mut u8, newEntry: u8) {
    unsafe {
        let mut list = list;
        let mut pos = pos;
        let mut newEntry = newEntry;
        ((list).wrapping_offset((((pos).read()) as i32) as isize)).write(newEntry);
        (pos).write(((pos).read()).wrapping_add(1));
    }
}
