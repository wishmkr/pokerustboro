//! Translated from `src/script_menu.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): MultichoiceList_BrineyOnDewford MultichoiceList_EnterInfo MultichoiceList_ContestInfo MultichoiceList_ContestType MultichoiceList_BasePCWithRegistry MultichoiceList_BasePCNoRegistry MultichoiceList_RegisterMenu MultichoiceList_Bike MultichoiceList_StatusInfo MultichoiceList_BrineyOffDewford MultichoiceList_ViewedPaintings MultichoiceList_YesNoInfo2 MultichoiceList_ChallengeInfo MultichoiceList_LevelMode MultichoiceList_Mechadoll1_Q1 MultichoiceList_Mechadoll1_Q2 MultichoiceList_Mechadoll1_Q3 MultichoiceList_Mechadoll2_Q1 MultichoiceList_Mechadoll2_Q2 MultichoiceList_Mechadoll2_Q3 MultichoiceList_Mechadoll3_Q1 MultichoiceList_Mechadoll3_Q2 MultichoiceList_Mechadoll3_Q3 MultichoiceList_Mechadoll4_Q1 MultichoiceList_Mechadoll4_Q2 MultichoiceList_Mechadoll4_Q3 MultichoiceList_Mechadoll5_Q1 MultichoiceList_Mechadoll5_Q2 MultichoiceList_Mechadoll5_Q3 MultichoiceList_VendingMachine MultichoiceList_MachBikeInfo MultichoiceList_AcroBikeInfo MultichoiceList_Satisfaction MultichoiceList_SternDeepSea MultichoiceList_UnusedAshVendor MultichoiceList_GameCornerDolls MultichoiceList_GameCornerTMs MultichoiceList_GameCornerCoins MultichoiceList_HowsFishing MultichoiceList_SSTidalSlateportWithBF MultichoiceList_SSTidalBattleFrontier MultichoiceList_RightLeft MultichoiceList_SSTidalSlateportNoBF MultichoiceList_Floors MultichoiceList_ShardsR MultichoiceList_ShardsY MultichoiceList_ShardsRY MultichoiceList_ShardsB MultichoiceList_ShardsRB MultichoiceList_ShardsYB MultichoiceList_ShardsRYB MultichoiceList_ShardsG MultichoiceList_ShardsRG MultichoiceList_ShardsYG MultichoiceList_ShardsRYG MultichoiceList_ShardsBG MultichoiceList_ShardsRBG MultichoiceList_ShardsYBG MultichoiceList_ShardsRYBG MultichoiceList_TourneyWithRecord MultichoiceList_TourneyNoRecord MultichoiceList_Tent MultichoiceList_LinkServicesNoBerry MultichoiceList_YesNoInfo MultichoiceList_BattleMode MultichoiceList_LinkServicesNoRecord MultichoiceList_LinkServicesAll MultichoiceList_LinkServicesNoRecordBerry MultichoiceList_WirelessMinigame MultichoiceList_LinkLeader MultichoiceList_ContestRank MultichoiceList_FrontierItemChoose MultichoiceList_LinkContestInfo MultichoiceList_LinkContestMode MultichoiceList_ForcedStartMenu MultichoiceList_FrontierGamblerBet MultichoiceList_UnusedSSTidal1 MultichoiceList_UnusedSSTidal2 MultichoiceList_UnusedSSTidal3 MultichoiceList_UnusedSSTidal4 MultichoiceList_Fossil MultichoiceList_YesNo MultichoiceList_FrontierRules MultichoiceList_FrontierPassInfo MultichoiceList_BattleArenaRules MultichoiceList_BattleTowerRules MultichoiceList_BattleDomeRules MultichoiceList_BattleFactoryRules MultichoiceList_BattlePalaceRules MultichoiceList_BattlePyramidRules MultichoiceList_BattlePikeRules MultichoiceList_GoOnRecordRestRetire MultichoiceList_GoOnRestRetire MultichoiceList_GoOnRecordRetire MultichoiceList_GoOnRetire MultichoiceList_TVLati MultichoiceList_BattleTowerFeelings MultichoiceList_WheresRayquaza MultichoiceList_SlateportTentRules MultichoiceList_FallarborTentRules MultichoiceList_TagMatchType MultichoiceList_Exit sMultichoiceLists gStdStrings sLinkServicesMultichoiceIds sPCNameStrings sLilycoveSSTidalDestinations sCableClubOptions_WithRecordMix sWirelessOptionsNoBerryCrush sWirelessOptions_NoRecordMix sWirelessOptions_AllServices sCableClubOptions_NoRecordMix sWirelessOptions_NoRecordMixBerryCrush
#[allow(unused_imports)]
use crate::data::script_menu::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sProcessInputDelay: u8 = 0u8;
pub(crate) static mut sLilycoveSSTidalSelections: crate::ffi::Align4<[u8; 7]> =
    crate::ffi::Align4([0; 7]);

unsafe extern "C" {
    static mut gMain: u8;
    static mut gPaletteFade: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_HallOfFame: u8;
    static mut gText_LanettesPC: u8;
    static mut gText_LogOff: u8;
    static mut gText_MenuOptionBag: u8;
    static mut gText_MenuOptionExit: u8;
    static mut gText_MenuOptionOption: u8;
    static mut gText_MenuOptionPokedex: u8;
    static mut gText_MenuOptionPokemon: u8;
    static mut gText_MenuOptionPokenav: u8;
    static mut gText_MenuOptionSave: u8;
    static mut gText_PlayersPC: u8;
    static mut gText_SomeonesPC: u8;
    static mut gText_WhichPCShouldBeAccessed: u8;
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
    fn CheckBagHasItem(a0: u16, a1: u16) -> u8;
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CreateMonSprite_PicBox(a0: u16, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWindowTemplate(
        a0: u8,
        a1: u8,
        a2: u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u16,
    ) -> crate::c::Rec4<8>;
    fn DestroyTask(a0: u8);
    fn DisplayYesNoMenuDefaultYes();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FreeResourcesAndDestroySprite(a0: *mut u8, a1: u8);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetFontAttribute(a0: u8, a1: u8) -> u8;
    fn GetStringWidth(a0: u8, a1: *mut u8, a2: i16) -> i32;
    fn InitMenuActionGrid(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn InitMenuNormal(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn LoadMessageBoxAndFrameGfx(a0: u8, a1: u8);
    fn Menu_GetCursorPos() -> u8;
    fn Menu_ProcessGridInput() -> i8;
    fn Menu_ProcessInput() -> i8;
    fn Menu_ProcessInputNoWrap() -> i8;
    fn Menu_ProcessInputNoWrapClearOnChoose() -> i8;
    fn PlaySE(a0: u16);
    fn PrintMenuGridTable(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut u8);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
    fn PrintPlayerNameOnWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn ShowScrollableMultichoice();
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringLength(a0: *mut u8) -> u16;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_Multichoice(
    left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
) -> u8 {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut multichoiceId = multichoiceId;
        let mut ignoreBPress = ignoreBPress;
        if ((FuncIsActiveTask(Some(Task_HandleMultichoiceInput))) as i32) == 1i32 {
            return 0u8;
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            DrawMultichoiceMenu(left, top, multichoiceId, ignoreBPress, 0u8);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_MultichoiceWithDefault(
    left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    defaultChoice: u8,
) -> u8 {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut multichoiceId = multichoiceId;
        let mut ignoreBPress = ignoreBPress;
        let mut defaultChoice = defaultChoice;
        if ((FuncIsActiveTask(Some(Task_HandleMultichoiceInput))) as i32) == 1i32 {
            return 0u8;
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            DrawMultichoiceMenu(left, top, multichoiceId, ignoreBPress, defaultChoice);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetLengthWithExpandedPlayerName(str: *mut u8) -> u16 {
    unsafe {
        let mut str = str;
        let mut length: u16 = 0u16;
        'l1: loop {
            if !((((str).read()) as i32) != 255i32) {
                break 'l1;
            }
            if (((str).read()) as i32) == 253i32 {
                str = (str).wrapping_offset(1);
                if (((str).read()) as i32) == 1i32 {
                    length = ((((length) as i32).wrapping_add(
                        ((StringLength(
                            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
                        )) as i32),
                    )) as u16);
                    str = (str).wrapping_offset(1);
                }
            } else {
                str = (str).wrapping_offset(1);
                length = (length).wrapping_add(1);
            }
        }
        return length;
    }
}
pub(crate) unsafe extern "C" fn DrawMultichoiceMenu(
    left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    cursorPos: u8,
) {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut multichoiceId = multichoiceId;
        let mut ignoreBPress = ignoreBPress;
        let mut cursorPos = cursorPos;
        let mut i: i32 = 0i32;
        let mut windowId: u8 = 0u8;
        let mut count: u8 = (((((&raw const sMultichoiceLists).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((multichoiceId) as i32) as isize * 8))
        .wrapping_add(4))
        .read();
        let mut actions: *mut u8 = (((((&raw const sMultichoiceLists).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset(((multichoiceId) as i32) as isize * 8))
        .cast::<*mut u8>())
        .read();
        let mut width: i32 = 0i32;
        let mut newWidth: u8 = 0u8;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((count) as i32)) {
                    break 'l1;
                }
                'l2: {
                    width = DisplayTextAndGetWidth(
                        (((actions).wrapping_offset((i) as isize * 8)).cast::<*mut u8>()).read(),
                        width,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        newWidth = ((ConvertPixelWidthToTileWidth(width)) as u8);
        left = ((ScriptMenu_AdjustLeftCoordFromWidth(((left) as i32), ((newWidth) as i32))) as u8);
        windowId = CreateWindowFromRect(
            left,
            top,
            newWidth,
            ((((count) as i32).wrapping_mul(2i32)) as u8),
        );
        SetStandardWindowBorderStyle(windowId, 0u8);
        PrintMenuTable(windowId, count, actions);
        InitMenuInUpperLeftCornerNormal(windowId, count, cursorPos);
        ScheduleBgCopyTilemapToVram(0u8);
        InitMultichoiceCheckWrap(ignoreBPress, count, windowId, multichoiceId);
    }
}
pub(crate) unsafe extern "C" fn InitMultichoiceCheckWrap(
    ignoreBPress: u8,
    count: u8,
    windowId: u8,
    multichoiceId: u8,
) {
    unsafe {
        let mut ignoreBPress = ignoreBPress;
        let mut count = count;
        let mut windowId = windowId;
        let mut multichoiceId = multichoiceId;
        let mut i: u8 = 0u8;
        let mut taskId: u8 = 0u8;
        ((&raw mut sProcessInputDelay).cast::<u8>().cast::<u8>()).write(2u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(6u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sLinkServicesMultichoiceIds)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        == ((multichoiceId) as i32)
                    {
                        ((&raw mut sProcessInputDelay).cast::<u8>().cast::<u8>()).write(12u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        taskId = CreateTask(Some(Task_HandleMultichoiceInput), 80u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((ignoreBPress) as i16));
        if ((count) as i32) > 3i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(1i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((windowId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((multichoiceId) as i16));
        DrawLinkServicesMultichoiceMenu(multichoiceId);
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMultichoiceInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selection: i8 = 0i8;
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
            if (((&raw mut sProcessInputDelay).cast::<u8>().cast::<u8>()).read()) != 0 {
                let __p1 = (&raw mut sProcessInputDelay).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                if !((((data).wrapping_offset(5)).read()) != 0) {
                    selection = Menu_ProcessInputNoWrap();
                } else {
                    selection = Menu_ProcessInput();
                }
                if ((((((&raw mut gMain).cast::<u8>())
                    .wrapping_add(46)
                    .cast::<u16>())
                .read()) as i32)
                    & 192i32)
                    != 0
                {
                    DrawLinkServicesMultichoiceMenu(((((data).wrapping_offset(7)).read()) as u8));
                }
                if ((selection) as i32) != (-2i32) {
                    if ((selection) as i32) == (-1i32) {
                        if (((data).wrapping_offset(4)).read()) != 0 {
                            return;
                        }
                        PlaySE(5u16);
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(127u16);
                    } else {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((selection) as u16));
                    }
                    ClearToTransparentAndRemoveWindow(((((data).wrapping_offset(6)).read()) as u8));
                    DestroyTask(taskId);
                    ScriptContext_Enable();
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_YesNo(left: u8, top: u8) -> u8 {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut taskId: u8 = 0u8;
        if ((FuncIsActiveTask(Some(Task_HandleYesNoInput))) as i32) == 1i32 {
            return 0u8;
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            DisplayYesNoMenuDefaultYes();
            taskId = CreateTask(Some(Task_HandleYesNoInput), 80u8);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsScriptActive() -> u8 {
    unsafe {
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 255i32 {
            return 0u8;
        } else {
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleYesNoInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            < 5i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
            return;
        }
        'l1: {
            let __sw2 = ((Menu_ProcessInputNoWrapClearOnChoose()) as i32);
            if __sw2 == (-2i32) {
                return;
            }
            if __sw2 == (-1i32) || __sw2 == 1i32 {
                PlaySE(5u16);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
                break 'l1;
            }
            if __sw2 == 0i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                break 'l1;
            }
        }
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_MultichoiceGrid(
    left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    columnCount: u8,
) -> u8 {
    unsafe {
        let mut left = left;
        let mut top = top;
        let mut multichoiceId = multichoiceId;
        let mut ignoreBPress = ignoreBPress;
        let mut columnCount = columnCount;
        if ((FuncIsActiveTask(Some(Task_HandleMultichoiceGridInput))) as i32) == 1i32 {
            return 0u8;
        } else {
            let mut taskId: u8 = 0u8;
            let mut rowCount: u8 = 0u8;
            let mut newWidth: u8 = 0u8;
            let mut i: i32 = 0i32;
            let mut width: i32 = 0i32;
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            width = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i
                        < (((((((&raw const sMultichoiceLists).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(((multichoiceId) as i32) as isize * 8))
                        .wrapping_add(4))
                        .read()) as i32))
                    {
                        break 'l1;
                    }
                    'l2: {
                        width = DisplayTextAndGetWidth(
                            ((((((((&raw const sMultichoiceLists).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(((multichoiceId) as i32) as isize * 8))
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 8))
                            .cast::<*mut u8>())
                            .read(),
                            width,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            newWidth = ((ConvertPixelWidthToTileWidth(width)) as u8);
            left = ((ScriptMenu_AdjustLeftCoordFromWidth(
                ((left) as i32),
                ((columnCount) as i32).wrapping_mul(((newWidth) as i32)),
            )) as u8);
            rowCount = ((crate::c::div_i32(
                (((((((&raw const sMultichoiceLists).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((multichoiceId) as i32) as isize * 8))
                .wrapping_add(4))
                .read()) as i32),
                ((columnCount) as i32),
            )) as u8);
            taskId = CreateTask(Some(Task_HandleMultichoiceGridInput), 80u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((ignoreBPress) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(
                ((CreateWindowFromRect(
                    left,
                    top,
                    ((((columnCount) as i32).wrapping_mul(((newWidth) as i32))) as u8),
                    ((((rowCount) as i32).wrapping_mul(2i32)) as u8),
                )) as i16),
            );
            SetStandardWindowBorderStyle(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u8),
                0u8,
            );
            PrintMenuGridTable(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u8),
                ((((newWidth) as i32).wrapping_mul(8i32)) as u8),
                columnCount,
                rowCount,
                (((((&raw const sMultichoiceLists).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((multichoiceId) as i32) as isize * 8))
                .cast::<*mut u8>())
                .read(),
            );
            InitMenuActionGrid(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u8),
                ((((newWidth) as i32).wrapping_mul(8i32)) as u8),
                columnCount,
                rowCount,
                0u8,
            );
            CopyWindowToVram(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u8),
                3u8,
            );
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMultichoiceGridInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut selection: i8 = Menu_ProcessGridInput();
        'l1: {
            let __sw1 = ((selection) as i32);
            let __matched = __sw1 == (-2i32) || __sw1 == (-1i32);
            if __sw1 == (-2i32) {
                return;
            }
            if __sw1 == (-1i32) {
                if (((data).wrapping_offset(4)).read()) != 0 {
                    return;
                }
                PlaySE(5u16);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(127u16);
                break 'l1;
            }
            if !__matched {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(((selection) as u16));
                break 'l1;
            }
        }
        ClearToTransparentAndRemoveWindow(((((data).wrapping_offset(6)).read()) as u8));
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_CreatePCMultichoice() -> u16 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_HandleMultichoiceInput))) as i32) == 1i32 {
            return 0u16;
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            CreatePCMultichoice();
            return 1u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn CreatePCMultichoice() {
    unsafe {
        let mut x: u8 = 8u8;
        let mut pixelWidth: u32 = 0u32;
        let mut width: u8 = 0u8;
        let mut numChoices: u8 = 0u8;
        let mut windowId: u8 = 0u8;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(16u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    pixelWidth = ((DisplayTextAndGetWidth(
                        ((((&raw const sPCNameStrings)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<*mut u8>())
                        .cast::<*mut u8>())
                        .wrapping_offset((i) as isize))
                        .read(),
                        ((pixelWidth) as i32),
                    )) as u32);
                }
                i = (i).wrapping_add(1);
            }
        }
        if (FlagGet(2148u16)) != 0 {
            pixelWidth = ((DisplayTextAndGetWidth(
                (&raw mut gText_HallOfFame).cast::<u8>(),
                ((pixelWidth) as i32),
            )) as u32);
        }
        width = ((ConvertPixelWidthToTileWidth(((pixelWidth) as i32))) as u8);
        if (FlagGet(2148u16)) != 0 {
            numChoices = 4u8;
            windowId = CreateWindowFromRect(0u8, 0u8, width, 8u8);
            SetStandardWindowBorderStyle(windowId, 0u8);
            AddTextPrinterParameterized(
                windowId,
                1u8,
                (&raw mut gText_HallOfFame).cast::<u8>(),
                x,
                33u8,
                255u8,
                None,
            );
            AddTextPrinterParameterized(
                windowId,
                1u8,
                (&raw mut gText_LogOff).cast::<u8>(),
                x,
                49u8,
                255u8,
                None,
            );
        } else {
            numChoices = 3u8;
            windowId = CreateWindowFromRect(0u8, 0u8, width, 6u8);
            SetStandardWindowBorderStyle(windowId, 0u8);
            AddTextPrinterParameterized(
                windowId,
                1u8,
                (&raw mut gText_LogOff).cast::<u8>(),
                x,
                33u8,
                255u8,
                None,
            );
        }
        if (FlagGet(2219u16)) != 0 {
            AddTextPrinterParameterized(
                windowId,
                1u8,
                (&raw mut gText_LanettesPC).cast::<u8>(),
                x,
                1u8,
                255u8,
                None,
            );
        } else {
            AddTextPrinterParameterized(
                windowId,
                1u8,
                (&raw mut gText_SomeonesPC).cast::<u8>(),
                x,
                1u8,
                255u8,
                None,
            );
        }
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PlayersPC).cast::<u8>(),
        );
        PrintPlayerNameOnWindow(
            windowId,
            (&raw mut gStringVar4).cast::<u8>(),
            ((x) as u16),
            17u16,
        );
        InitMenuInUpperLeftCornerNormal(windowId, numChoices, 0u8);
        CopyWindowToVram(windowId, 3u8);
        InitMultichoiceCheckWrap(0u8, numChoices, windowId, 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_DisplayPCStartupPrompt() {
    unsafe {
        LoadMessageBoxAndFrameGfx(0u8, 1u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gText_WhichPCShouldBeAccessed).cast::<u8>(),
            0u8,
            None,
            2u8,
            1u8,
            3u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_CreateLilycoveSSTidalMultichoice() -> u8 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_HandleMultichoiceInput))) as i32) == 1i32 {
            return 0u8;
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            CreateLilycoveSSTidalMultichoice();
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateLilycoveSSTidalMultichoice() {
    unsafe {
        let mut selectionCount: u8 = 0u8;
        let mut count: u8 = 0u8;
        let mut pixelWidth: u32 = 0u32;
        let mut width: u8 = 0u8;
        let mut windowId: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut j: u32 = 0u32;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        GetFontAttribute(1u8, 0u8);
        if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
            ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                .wrapping_offset(((selectionCount) as i32) as isize))
            .write(0u8);
            selectionCount = (selectionCount).wrapping_add(1);
            if ((FlagGet(464u16)) as i32) == 1i32 {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(1u8);
                selectionCount = (selectionCount).wrapping_add(1);
            }
        }
        if (((CheckBagHasItem(275u16, 1u16)) as i32) == 1i32)
            && (((FlagGet(2227u16)) as i32) == 1i32)
        {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(2u8);
                selectionCount = (selectionCount).wrapping_add(1);
            }
            if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
                && (((FlagGet(430u16)) as i32) == 0i32)
            {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(2u8);
                selectionCount = (selectionCount).wrapping_add(1);
                FlagSet(430u16);
            }
        }
        if (((CheckBagHasItem(370u16, 1u16)) as i32) == 1i32)
            && (((FlagGet(2272u16)) as i32) == 1i32)
        {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(3u8);
                selectionCount = (selectionCount).wrapping_add(1);
            }
            if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
                && (((FlagGet(475u16)) as i32) == 0i32)
            {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(3u8);
                selectionCount = (selectionCount).wrapping_add(1);
                FlagSet(475u16);
            }
        }
        if (((CheckBagHasItem(371u16, 1u16)) as i32) == 1i32)
            && (((FlagGet(2261u16)) as i32) == 1i32)
        {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(4u8);
                selectionCount = (selectionCount).wrapping_add(1);
            }
            if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
                && (((FlagGet(431u16)) as i32) == 0i32)
            {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(4u8);
                selectionCount = (selectionCount).wrapping_add(1);
                FlagSet(431u16);
            }
        }
        if (((CheckBagHasItem(376u16, 1u16)) as i32) == 1i32)
            && (((FlagGet(2262u16)) as i32) == 1i32)
        {
            if ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32 {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(5u8);
                selectionCount = (selectionCount).wrapping_add(1);
            }
            if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 1i32)
                && (((FlagGet(432u16)) as i32) == 0i32)
            {
                ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((selectionCount) as i32) as isize))
                .write(5u8);
                selectionCount = (selectionCount).wrapping_add(1);
                FlagSet(432u16);
            }
        }
        ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((selectionCount) as i32) as isize))
        .write(6u8);
        selectionCount = (selectionCount).wrapping_add(1);
        if (((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32) == 0i32)
            && (((FlagGet(464u16)) as i32) == 1i32)
        {
            count = selectionCount;
        }
        count = selectionCount;
        if ((count) as i32) == 7i32 {
            ((&raw mut gSpecialVar_0x8004).cast::<u16>()).write(11u16);
            ShowScrollableMultichoice();
        } else {
            pixelWidth = 0u32;
            {
                j = 0u32;
                'l3: loop {
                    if !(j < 7u32) {
                        break 'l3;
                    }
                    'l4: {
                        let mut selection: u8 =
                            ((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                                .wrapping_offset(((j) as i32) as isize))
                            .read();
                        if ((selection) as i32) != 255i32 {
                            pixelWidth = ((DisplayTextAndGetWidth(
                                ((((&raw const sLilycoveSSTidalDestinations)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(((selection) as i32) as isize))
                                .read(),
                                ((pixelWidth) as i32),
                            )) as u32);
                        }
                    }
                    j = (j).wrapping_add(1);
                }
            }
            width = ((ConvertPixelWidthToTileWidth(((pixelWidth) as i32))) as u8);
            windowId = CreateWindowFromRect(
                (((28i32).wrapping_sub(((width) as i32))) as u8),
                ((((6i32).wrapping_sub(((count) as i32))).wrapping_mul(2i32)) as u8),
                width,
                ((((count) as i32).wrapping_mul(2i32)) as u8),
            );
            SetStandardWindowBorderStyle(windowId, 0u8);
            {
                selectionCount = 0u8;
                i = 0u8;
                'l5: loop {
                    if !(((i) as i32) < 7i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 255i32
                        {
                            AddTextPrinterParameterized(
                                windowId,
                                1u8,
                                ((((&raw const sLilycoveSSTidalDestinations)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<*mut u8>())
                                .cast::<*mut u8>())
                                .wrapping_offset(
                                    ((((((&raw mut sLilycoveSSTidalSelections).cast::<u8>())
                                        .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize,
                                ))
                                .read(),
                                8u8,
                                (((((selectionCount) as i32).wrapping_mul(16i32))
                                    .wrapping_add(1i32)) as u8),
                                255u8,
                                None,
                            );
                            selectionCount = (selectionCount).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            InitMenuInUpperLeftCornerNormal(
                windowId,
                count,
                ((((count) as i32).wrapping_sub(1i32)) as u8),
            );
            CopyWindowToVram(windowId, 3u8);
            InitMultichoiceCheckWrap(0u8, count, windowId, 8u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLilycoveSSTidalSelection() {
    unsafe {
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) != 127i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                ((((((&raw mut sLilycoveSSTidalSelections).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(
                        ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) as isize,
                    ))
                .read()) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonPicWindow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                break 'l1;
            }
            if __sw1 == 2i32 {
                FreeResourcesAndDestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 68,
                    ),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
                );
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                ClearToTransparentAndRemoveWindow(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u8),
                );
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_ShowPokemonPic(species: u16, x: u8, y: u8) -> u8 {
    unsafe {
        let mut species = species;
        let mut x = x;
        let mut y = y;
        let mut taskId: u8 = 0u8;
        let mut spriteId: u8 = 0u8;
        if ((FindTaskIdByFunc(Some(Task_PokemonPicWindow))) as i32) != 255i32 {
            return 0u8;
        } else {
            spriteId = CreateMonSprite_PicBox(
                species,
                (((((x) as i32).wrapping_mul(8i32)).wrapping_add(40i32)) as i16),
                (((((y) as i32).wrapping_mul(8i32)).wrapping_add(40i32)) as i16),
                0u8,
            );
            taskId = CreateTask(Some(Task_PokemonPicWindow), 80u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((CreateWindowFromRect(x, y, 8u8, 8u8)) as i16));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((species) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((spriteId) as i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                (0u16) as i32,
            );
            SetStandardWindowBorderStyle(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u8),
                1u8,
            );
            ScheduleBgCopyTilemapToVram(0u8);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_HidePokemonPic() -> Option<unsafe extern "C" fn() -> u8> {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_PokemonPicWindow));
        if ((taskId) as i32) == 255i32 {
            return None;
        }
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return Some(IsPicboxClosed);
    }
}
pub(crate) unsafe extern "C" fn IsPicboxClosed() -> u8 {
    unsafe {
        if ((FindTaskIdByFunc(Some(Task_PokemonPicWindow))) as i32) == 255i32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateWindowFromRect(x: u8, y: u8, width: u8, height: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut width = width;
        let mut height = height;
        let mut template = crate::ffi::Align4([0u8; 8]);
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(CreateWindowTemplate(
                0u8,
                ((((x) as i32).wrapping_add(1i32)) as u8),
                ((((y) as i32).wrapping_add(1i32)) as u8),
                width,
                height,
                15u8,
                100u16,
            ));
        let mut windowId: u8 = ((AddWindow((&raw mut template).cast::<u8>())) as u8);
        PutWindowTilemap(windowId);
        return windowId;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearToTransparentAndRemoveWindow(windowId: u8) {
    unsafe {
        let mut windowId = windowId;
        ClearStdWindowAndFrameToTransparent(windowId, 1u8);
        RemoveWindow(windowId);
    }
}
pub(crate) unsafe extern "C" fn DrawLinkServicesMultichoiceMenu(multichoiceId: u8) {
    unsafe {
        let mut multichoiceId = multichoiceId;
        'l1: {
            let __sw1 = ((multichoiceId) as i32);
            if __sw1 == 77i32 {
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    ((((&raw const sWirelessOptionsNoBerryCrush)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
                    .read(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 76i32 {
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    ((((&raw const sCableClubOptions_WithRecordMix)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
                    .read(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 78i32 {
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    ((((&raw const sWirelessOptions_NoRecordMix)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
                    .read(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 79i32 {
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    ((((&raw const sWirelessOptions_AllServices)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
                    .read(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 75i32 {
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    ((((&raw const sWirelessOptions_NoRecordMixBerryCrush)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
                    .read(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                break 'l1;
            }
            if __sw1 == 74i32 {
                FillWindowPixelBuffer(0u8, 17u8);
                AddTextPrinterParameterized2(
                    0u8,
                    1u8,
                    ((((&raw const sCableClubOptions_NoRecordMix)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<*mut u8>())
                    .cast::<*mut u8>())
                    .wrapping_offset(((Menu_GetCursorPos()) as i32) as isize))
                    .read(),
                    0u8,
                    None,
                    2u8,
                    1u8,
                    3u8,
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_CreateStartMenuForPokenavTutorial() -> u16 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_HandleMultichoiceInput))) as i32) == 1i32 {
            return 0u16;
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(255u16);
            CreateStartMenuForPokenavTutorial();
            return 1u16;
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateStartMenuForPokenavTutorial() {
    unsafe {
        let mut windowId: u8 = CreateWindowFromRect(21u8, 0u8, 7u8, 18u8);
        SetStandardWindowBorderStyle(windowId, 0u8);
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionPokedex).cast::<u8>(),
            8u8,
            9u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionPokemon).cast::<u8>(),
            8u8,
            25u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionBag).cast::<u8>(),
            8u8,
            41u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionPokenav).cast::<u8>(),
            8u8,
            57u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            8u8,
            73u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionSave).cast::<u8>(),
            8u8,
            89u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionOption).cast::<u8>(),
            8u8,
            105u8,
            255u8,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            1u8,
            (&raw mut gText_MenuOptionExit).cast::<u8>(),
            8u8,
            121u8,
            255u8,
            None,
        );
        InitMenuNormal(
            windowId,
            1u8,
            0u8,
            9u8,
            16u8,
            ((crate::c::div_u32(64u32, 8u32)) as u8),
            0u8,
        );
        InitMultichoiceNoWrap(
            0u8,
            ((crate::c::div_u32(64u32, 8u32)) as u8),
            windowId,
            86u8,
        );
        CopyWindowToVram(windowId, 3u8);
    }
}
pub(crate) unsafe extern "C" fn InitMultichoiceNoWrap(
    ignoreBPress: u8,
    unusedCount: u8,
    windowId: u8,
    multichoiceId: u8,
) {
    unsafe {
        let mut ignoreBPress = ignoreBPress;
        let mut unusedCount = unusedCount;
        let mut windowId = windowId;
        let mut multichoiceId = multichoiceId;
        let mut taskId: u8 = 0u8;
        ((&raw mut sProcessInputDelay).cast::<u8>().cast::<u8>()).write(2u8);
        taskId = CreateTask(Some(Task_HandleMultichoiceInput), 80u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((ignoreBPress) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((windowId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((multichoiceId) as i16));
    }
}
pub(crate) unsafe extern "C" fn DisplayTextAndGetWidthInternal(str: *mut u8) -> i32 {
    unsafe {
        let mut str = str;
        let mut temp = crate::ffi::Align4([0u8; 64]);
        StringExpandPlaceholders((&raw mut temp).cast::<u8>(), str);
        return GetStringWidth(1u8, (&raw mut temp).cast::<u8>(), 0i16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayTextAndGetWidth(str: *mut u8, prevWidth: i32) -> i32 {
    unsafe {
        let mut str = str;
        let mut prevWidth = prevWidth;
        let mut width: i32 = DisplayTextAndGetWidthInternal(str);
        if width < prevWidth {
            width = prevWidth;
        }
        return width;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertPixelWidthToTileWidth(width: i32) -> i32 {
    unsafe {
        let mut width = width;
        return (if (crate::c::div_i32((width).wrapping_add(9i32), 8i32)).wrapping_add(1i32) > 28i32
        {
            28i32
        } else {
            (crate::c::div_i32((width).wrapping_add(9i32), 8i32)).wrapping_add(1i32)
        });
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_AdjustLeftCoordFromWidth(left: i32, width: i32) -> i32 {
    unsafe {
        let mut left = left;
        let mut width = width;
        let mut adjustedLeft: i32 = left;
        if (left).wrapping_add(width) > 28i32 {
            if (28i32).wrapping_sub(width) < 0i32 {
                adjustedLeft = 0i32;
            } else {
                adjustedLeft = (28i32).wrapping_sub(width);
            }
        }
        return adjustedLeft;
    }
}
