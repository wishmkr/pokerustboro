//! Translated from `src/script_menu.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): MultichoiceList_BrineyOnDewford MultichoiceList_EnterInfo MultichoiceList_ContestInfo MultichoiceList_ContestType MultichoiceList_BasePCWithRegistry MultichoiceList_BasePCNoRegistry MultichoiceList_RegisterMenu MultichoiceList_Bike MultichoiceList_StatusInfo MultichoiceList_BrineyOffDewford MultichoiceList_ViewedPaintings MultichoiceList_YesNoInfo2 MultichoiceList_ChallengeInfo MultichoiceList_LevelMode MultichoiceList_Mechadoll1_Q1 MultichoiceList_Mechadoll1_Q2 MultichoiceList_Mechadoll1_Q3 MultichoiceList_Mechadoll2_Q1 MultichoiceList_Mechadoll2_Q2 MultichoiceList_Mechadoll2_Q3 MultichoiceList_Mechadoll3_Q1 MultichoiceList_Mechadoll3_Q2 MultichoiceList_Mechadoll3_Q3 MultichoiceList_Mechadoll4_Q1 MultichoiceList_Mechadoll4_Q2 MultichoiceList_Mechadoll4_Q3 MultichoiceList_Mechadoll5_Q1 MultichoiceList_Mechadoll5_Q2 MultichoiceList_Mechadoll5_Q3 MultichoiceList_VendingMachine MultichoiceList_MachBikeInfo MultichoiceList_AcroBikeInfo MultichoiceList_Satisfaction MultichoiceList_SternDeepSea MultichoiceList_UnusedAshVendor MultichoiceList_GameCornerDolls MultichoiceList_GameCornerTMs MultichoiceList_GameCornerCoins MultichoiceList_HowsFishing MultichoiceList_SSTidalSlateportWithBF MultichoiceList_SSTidalBattleFrontier MultichoiceList_RightLeft MultichoiceList_SSTidalSlateportNoBF MultichoiceList_Floors MultichoiceList_ShardsR MultichoiceList_ShardsY MultichoiceList_ShardsRY MultichoiceList_ShardsB MultichoiceList_ShardsRB MultichoiceList_ShardsYB MultichoiceList_ShardsRYB MultichoiceList_ShardsG MultichoiceList_ShardsRG MultichoiceList_ShardsYG MultichoiceList_ShardsRYG MultichoiceList_ShardsBG MultichoiceList_ShardsRBG MultichoiceList_ShardsYBG MultichoiceList_ShardsRYBG MultichoiceList_TourneyWithRecord MultichoiceList_TourneyNoRecord MultichoiceList_Tent MultichoiceList_LinkServicesNoBerry MultichoiceList_YesNoInfo MultichoiceList_BattleMode MultichoiceList_LinkServicesNoRecord MultichoiceList_LinkServicesAll MultichoiceList_LinkServicesNoRecordBerry MultichoiceList_WirelessMinigame MultichoiceList_LinkLeader MultichoiceList_ContestRank MultichoiceList_FrontierItemChoose MultichoiceList_LinkContestInfo MultichoiceList_LinkContestMode MultichoiceList_ForcedStartMenu MultichoiceList_FrontierGamblerBet MultichoiceList_UnusedSSTidal1 MultichoiceList_UnusedSSTidal2 MultichoiceList_UnusedSSTidal3 MultichoiceList_UnusedSSTidal4 MultichoiceList_Fossil MultichoiceList_YesNo MultichoiceList_FrontierRules MultichoiceList_FrontierPassInfo MultichoiceList_BattleArenaRules MultichoiceList_BattleTowerRules MultichoiceList_BattleDomeRules MultichoiceList_BattleFactoryRules MultichoiceList_BattlePalaceRules MultichoiceList_BattlePyramidRules MultichoiceList_BattlePikeRules MultichoiceList_GoOnRecordRestRetire MultichoiceList_GoOnRestRetire MultichoiceList_GoOnRecordRetire MultichoiceList_GoOnRetire MultichoiceList_TVLati MultichoiceList_BattleTowerFeelings MultichoiceList_WheresRayquaza MultichoiceList_SlateportTentRules MultichoiceList_FallarborTentRules MultichoiceList_TagMatchType MultichoiceList_Exit sMultichoiceLists gStdStrings sLinkServicesMultichoiceIds sPCNameStrings sLilycoveSSTidalDestinations sCableClubOptions_WithRecordMix sWirelessOptionsNoBerryCrush sWirelessOptions_NoRecordMix sWirelessOptions_AllServices sCableClubOptions_NoRecordMix sWirelessOptions_NoRecordMixBerryCrush

/// `struct MultichoiceListStruct`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MultichoiceListStruct {
    pub list: *mut MenuAction,
    pub count: u8,
}

unsafe impl Sync for MultichoiceListStruct {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<MultichoiceListStruct>() == 8);
    assert!(offset_of!(MultichoiceListStruct, list) == 0);
    assert!(offset_of!(MultichoiceListStruct, count) == 4);
};

static MultichoiceList_ForcedStartMenu: Table<CArray<MenuAction, 8>> =
    Table((&raw const crate::data::script_menu::MultichoiceList_ForcedStartMenu).cast());
static sCableClubOptions_NoRecordMix: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::script_menu::sCableClubOptions_NoRecordMix).cast());
static sCableClubOptions_WithRecordMix: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::script_menu::sCableClubOptions_WithRecordMix).cast());
static sLilycoveSSTidalDestinations: Table<CArray<*mut u8, 7>> =
    Table((&raw const crate::data::script_menu::sLilycoveSSTidalDestinations).cast());
static sLinkServicesMultichoiceIds: Table<CArray<u8, 6>> =
    Table((&raw const crate::data::script_menu::sLinkServicesMultichoiceIds).cast());
static sMultichoiceLists: Table<CArray<MultichoiceListStruct, 114>> =
    Table((&raw const crate::data::script_menu::sMultichoiceLists).cast());
static sPCNameStrings: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::script_menu::sPCNameStrings).cast());
static sWirelessOptionsNoBerryCrush: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::script_menu::sWirelessOptionsNoBerryCrush).cast());
static sWirelessOptions_AllServices: Table<CArray<*mut u8, 5>> =
    Table((&raw const crate::data::script_menu::sWirelessOptions_AllServices).cast());
static sWirelessOptions_NoRecordMix: Table<CArray<*mut u8, 4>> =
    Table((&raw const crate::data::script_menu::sWirelessOptions_NoRecordMix).cast());
static sWirelessOptions_NoRecordMixBerryCrush: Table<CArray<*mut u8, 3>> =
    Table((&raw const crate::data::script_menu::sWirelessOptions_NoRecordMixBerryCrush).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sProcessInputDelay: u8 = 0;
pub(crate) static mut sLilycoveSSTidalSelections: Aligned<CArray<u8, 7>> =
    Aligned(unsafe { zeroed() });

unsafe extern "C" {
    static mut gMain: Main;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_Result: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_HallOfFame: CArray<u8, 0>;
    static gText_LanettesPC: CArray<u8, 0>;
    static gText_LogOff: CArray<u8, 0>;
    static gText_MenuOptionBag: CArray<u8, 0>;
    static gText_MenuOptionExit: CArray<u8, 0>;
    static gText_MenuOptionOption: CArray<u8, 0>;
    static gText_MenuOptionPokedex: CArray<u8, 0>;
    static gText_MenuOptionPokemon: CArray<u8, 0>;
    static gText_MenuOptionPokenav: CArray<u8, 0>;
    static gText_MenuOptionSave: CArray<u8, 0>;
    static gText_PlayersPC: CArray<u8, 0>;
    static gText_SomeonesPC: CArray<u8, 0>;
    static gText_WhichPCShouldBeAccessed: CArray<u8, 0>;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
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
    ) -> WindowTemplate;
    fn DestroyTask(a0: u8);
    fn DisplayYesNoMenuDefaultYes();
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FreeResourcesAndDestroySprite(a0: *mut Sprite, a1: u8);
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
    fn PrintMenuGridTable(a0: u8, a1: u8, a2: u8, a3: u8, a4: *mut MenuAction);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
    fn PrintPlayerNameOnWindow(a0: u8, a1: *mut u8, a2: u16, a3: u16);
    fn PutWindowTilemap(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn ShowScrollableMultichoice();
    fn SpriteCallbackDummy(a0: *mut Sprite);
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
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        DrawMultichoiceMenu(left, top, multichoiceId, ignoreBPress, 0);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
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
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        DrawMultichoiceMenu(left, top, multichoiceId, ignoreBPress, defaultChoice);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetLengthWithExpandedPlayerName(mut str: *mut u8) -> u16 {
    let mut length: u16 = 0;
    while *str != EOS {
        if *str == PLACEHOLDER_BEGIN {
            str = str.at(1);
            if *str == PLACEHOLDER_ID_PLAYER {
                length += StringLength((*gSaveBlock2Ptr).playerName.as_mut_ptr());
                str = str.at(1);
            }
        } else {
            str = str.at(1);
            length += 1;
        }
    }
    return length;
}
pub(crate) unsafe extern "C" fn DrawMultichoiceMenu(
    mut left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    cursorPos: u8,
) {
    let mut i: i32 = 0;
    let mut windowId: u8 = 0;
    let mut count: u8 = sMultichoiceLists[multichoiceId].count;
    let mut actions: *mut MenuAction = sMultichoiceLists[multichoiceId].list;
    let mut width: i32 = 0;
    let mut newWidth: u8 = 0;
    i = 0;
    while i < count as i32 {
        width = DisplayTextAndGetWidth((*actions.at(i)).text, width);
        i += 1;
    }
    newWidth = ConvertPixelWidthToTileWidth(width) as u8;
    left = ScriptMenu_AdjustLeftCoordFromWidth(left as i32, newWidth as i32) as u8;
    windowId = CreateWindowFromRect(left, top, newWidth, count * 2);
    SetStandardWindowBorderStyle(windowId, FALSE);
    PrintMenuTable(windowId, count, actions);
    InitMenuInUpperLeftCornerNormal(windowId, count, cursorPos);
    ScheduleBgCopyTilemapToVram(0);
    InitMultichoiceCheckWrap(ignoreBPress, count, windowId, multichoiceId);
}
pub(crate) unsafe extern "C" fn InitMultichoiceCheckWrap(
    ignoreBPress: u8,
    count: u8,
    windowId: u8,
    multichoiceId: u8,
) {
    let mut i: u8 = 0;
    let mut taskId: u8 = 0;
    sProcessInputDelay = 2;
    i = 0;
    while i < 6 {
        if sLinkServicesMultichoiceIds[i] == multichoiceId {
            sProcessInputDelay = 12;
        }
        i += 1;
    }
    taskId = CreateTask(Some(Task_HandleMultichoiceInput), 80);
    gTasks[taskId].data[4] = ignoreBPress as i16;
    if count > 3 {
        gTasks[taskId].data[5] = TRUE as i16;
    } else {
        gTasks[taskId].data[5] = FALSE as i16;
    }
    gTasks[taskId].data[6] = windowId as i16;
    gTasks[taskId].data[7] = multichoiceId as i16;
    DrawLinkServicesMultichoiceMenu(multichoiceId);
}
pub(crate) unsafe extern "C" fn Task_HandleMultichoiceInput(taskId: u8) {
    let mut selection: i8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        if sProcessInputDelay != 0 {
            sProcessInputDelay -= 1;
        } else {
            if *data.at(5) == 0 {
                selection = Menu_ProcessInputNoWrap();
            } else {
                selection = Menu_ProcessInput();
            }
            if gMain.newKeys as i32 & 192 != 0 {
                DrawLinkServicesMultichoiceMenu(*data.at(7) as u8);
            }
            if selection != MENU_NOTHING_CHOSEN {
                if selection == MENU_B_PRESSED {
                    if *data.at(4) != 0 {
                        return;
                    }
                    PlaySE(SE_SELECT);
                    gSpecialVar_Result = MULTI_B_PRESSED;
                } else {
                    gSpecialVar_Result = selection as u16;
                }
                ClearToTransparentAndRemoveWindow(*data.at(6) as u8);
                DestroyTask(taskId);
                ScriptContext_Enable();
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_YesNo(left: u8, top: u8) -> u8 {
    let mut taskId: u8 = 0;
    if FuncIsActiveTask(Some(Task_HandleYesNoInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        DisplayYesNoMenuDefaultYes();
        taskId = CreateTask(Some(Task_HandleYesNoInput), 0x50);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsScriptActive() -> u8 {
    if gSpecialVar_Result == 0xFF {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleYesNoInput(taskId: u8) {
    if gTasks[taskId].data[2] < 5 {
        gTasks[taskId].data[2] += 1;
        return;
    }
    match Menu_ProcessInputNoWrapClearOnChoose() {
        MENU_NOTHING_CHOSEN => {
            return;
        }
        MENU_B_PRESSED | 1 => {
            PlaySE(SE_SELECT);
            gSpecialVar_Result = 0;
        }
        0 => {
            gSpecialVar_Result = 1;
        }
        _ => {}
    }
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_MultichoiceGrid(
    mut left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    columnCount: u8,
) -> u8 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceGridInput)) == TRUE {
        return FALSE;
    } else {
        let mut taskId: u8 = 0;
        let mut rowCount: u8 = 0;
        let mut newWidth: u8 = 0;
        let mut i: i32 = 0;
        let mut width: i32 = 0;
        gSpecialVar_Result = 0xFF;
        width = 0;
        i = 0;
        while i < sMultichoiceLists[multichoiceId].count as i32 {
            width =
                DisplayTextAndGetWidth((*sMultichoiceLists[multichoiceId].list.at(i)).text, width);
            i += 1;
        }
        newWidth = ConvertPixelWidthToTileWidth(width) as u8;
        left =
            ScriptMenu_AdjustLeftCoordFromWidth(left as i32, columnCount as i32 * newWidth as i32)
                as u8;
        rowCount = div_i32(
            sMultichoiceLists[multichoiceId].count as i32,
            columnCount as i32,
        ) as u8;
        taskId = CreateTask(Some(Task_HandleMultichoiceGridInput), 80);
        gTasks[taskId].data[4] = ignoreBPress as i16;
        gTasks[taskId].data[6] =
            CreateWindowFromRect(left, top, columnCount * newWidth, rowCount * 2) as i16;
        SetStandardWindowBorderStyle(gTasks[taskId].data[6] as u8, FALSE);
        PrintMenuGridTable(
            gTasks[taskId].data[6] as u8,
            newWidth * 8,
            columnCount,
            rowCount,
            sMultichoiceLists[multichoiceId].list,
        );
        InitMenuActionGrid(
            gTasks[taskId].data[6] as u8,
            newWidth * 8,
            columnCount,
            rowCount,
            0,
        );
        CopyWindowToVram(gTasks[taskId].data[6] as u8, COPYWIN_FULL);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn Task_HandleMultichoiceGridInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut selection: i8 = Menu_ProcessGridInput();
    match selection {
        MENU_NOTHING_CHOSEN => {
            return;
        }
        MENU_B_PRESSED => {
            if *data.at(4) != 0 {
                return;
            }
            PlaySE(SE_SELECT);
            gSpecialVar_Result = MULTI_B_PRESSED;
        }
        _ => {
            gSpecialVar_Result = selection as u16;
        }
    }
    ClearToTransparentAndRemoveWindow(*data.at(6) as u8);
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_CreatePCMultichoice() -> u16 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE as u16;
    } else {
        gSpecialVar_Result = 0xFF;
        CreatePCMultichoice();
        return TRUE as u16;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreatePCMultichoice() {
    let mut x: u8 = 8;
    let mut pixelWidth: u32 = 0;
    let mut width: u8 = 0;
    let mut numChoices: u8 = 0;
    let mut windowId: u8 = 0;
    let mut i: i32 = 0;
    i = 0;
    while i < 4 {
        pixelWidth = DisplayTextAndGetWidth(sPCNameStrings[i], pixelWidth as i32) as u32;
        i += 1;
    }
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        pixelWidth =
            DisplayTextAndGetWidth(gText_HallOfFame.as_ptr().cast_mut(), pixelWidth as i32) as u32;
    }
    width = ConvertPixelWidthToTileWidth(pixelWidth as i32) as u8;
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        numChoices = 4;
        windowId = CreateWindowFromRect(0, 0, width, 8);
        SetStandardWindowBorderStyle(windowId, FALSE);
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            gText_HallOfFame.as_ptr().cast_mut(),
            x,
            33,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            gText_LogOff.as_ptr().cast_mut(),
            x,
            49,
            TEXT_SKIP_DRAW,
            None,
        );
    } else {
        numChoices = 3;
        windowId = CreateWindowFromRect(0, 0, width, 6);
        SetStandardWindowBorderStyle(windowId, FALSE);
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            gText_LogOff.as_ptr().cast_mut(),
            x,
            33,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    if FlagGet(FLAG_SYS_PC_LANETTE) != 0 {
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            gText_LanettesPC.as_ptr().cast_mut(),
            x,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
    } else {
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            gText_SomeonesPC.as_ptr().cast_mut(),
            x,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PlayersPC.as_ptr().cast_mut(),
    );
    PrintPlayerNameOnWindow(windowId, gStringVar4.as_mut_ptr(), x as u16, 17);
    InitMenuInUpperLeftCornerNormal(windowId, numChoices, 0);
    CopyWindowToVram(windowId, COPYWIN_FULL);
    InitMultichoiceCheckWrap(FALSE, numChoices, windowId, MULTI_PC);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_DisplayPCStartupPrompt() {
    LoadMessageBoxAndFrameGfx(0, TRUE);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        gText_WhichPCShouldBeAccessed.as_ptr().cast_mut(),
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_CreateLilycoveSSTidalMultichoice() -> u8 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        CreateLilycoveSSTidalMultichoice();
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreateLilycoveSSTidalMultichoice() {
    let mut selectionCount: u8 = 0;
    let mut count: u8 = 0;
    let mut pixelWidth: u32 = 0;
    let mut width: u8 = 0;
    let mut windowId: u8 = 0;
    let mut i: u8 = 0;
    let mut j: u32 = 0;
    i = 0;
    while i < SSTIDAL_SELECTION_COUNT {
        sLilycoveSSTidalSelections[i] = 0xFF;
        i += 1;
    }
    GetFontAttribute(FONT_NORMAL, FONTATTR_MAX_LETTER_WIDTH);
    if gSpecialVar_0x8004 == 0 {
        sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_SLATEPORT;
        selectionCount += 1;
        if FlagGet(FLAG_MET_SCOTT_ON_SS_TIDAL) == TRUE {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_BATTLE_FRONTIER;
            selectionCount += 1;
        }
    }
    if CheckBagHasItem(ITEM_EON_TICKET, 1) == 1 && FlagGet(FLAG_ENABLE_SHIP_SOUTHERN_ISLAND) == 1 {
        if gSpecialVar_0x8004 == 0 {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_SOUTHERN_ISLAND;
            selectionCount += 1;
        }
        if gSpecialVar_0x8004 == 1 && FlagGet(FLAG_SHOWN_EON_TICKET) == FALSE {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_SOUTHERN_ISLAND;
            selectionCount += 1;
            FlagSet(FLAG_SHOWN_EON_TICKET);
        }
    }
    if CheckBagHasItem(ITEM_MYSTIC_TICKET, 1) == 1 && FlagGet(FLAG_ENABLE_SHIP_NAVEL_ROCK) == 1 {
        if gSpecialVar_0x8004 == 0 {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_NAVEL_ROCK;
            selectionCount += 1;
        }
        if gSpecialVar_0x8004 == 1 && FlagGet(FLAG_SHOWN_MYSTIC_TICKET) == FALSE {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_NAVEL_ROCK;
            selectionCount += 1;
            FlagSet(FLAG_SHOWN_MYSTIC_TICKET);
        }
    }
    if CheckBagHasItem(ITEM_AURORA_TICKET, 1) == 1 && FlagGet(FLAG_ENABLE_SHIP_BIRTH_ISLAND) == 1 {
        if gSpecialVar_0x8004 == 0 {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_BIRTH_ISLAND;
            selectionCount += 1;
        }
        if gSpecialVar_0x8004 == 1 && FlagGet(FLAG_SHOWN_AURORA_TICKET) == FALSE {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_BIRTH_ISLAND;
            selectionCount += 1;
            FlagSet(FLAG_SHOWN_AURORA_TICKET);
        }
    }
    if CheckBagHasItem(ITEM_OLD_SEA_MAP, 1) == 1 && FlagGet(FLAG_ENABLE_SHIP_FARAWAY_ISLAND) == 1 {
        if gSpecialVar_0x8004 == 0 {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_FARAWAY_ISLAND;
            selectionCount += 1;
        }
        if gSpecialVar_0x8004 == 1 && FlagGet(FLAG_SHOWN_OLD_SEA_MAP) == FALSE {
            sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_FARAWAY_ISLAND;
            selectionCount += 1;
            FlagSet(FLAG_SHOWN_OLD_SEA_MAP);
        }
    }
    sLilycoveSSTidalSelections[selectionCount] = SSTIDAL_SELECTION_EXIT;
    selectionCount += 1;
    if gSpecialVar_0x8004 == 0 && FlagGet(FLAG_MET_SCOTT_ON_SS_TIDAL) == TRUE {
        count = selectionCount;
    }
    count = selectionCount;
    if count == SSTIDAL_SELECTION_COUNT {
        gSpecialVar_0x8004 = SCROLL_MULTI_SS_TIDAL_DESTINATION;
        ShowScrollableMultichoice();
    } else {
        pixelWidth = 0;
        j = 0;
        while j < SSTIDAL_SELECTION_COUNT as u32 {
            let mut selection: u8 = sLilycoveSSTidalSelections[j];
            if selection != 0xFF {
                pixelWidth = DisplayTextAndGetWidth(
                    sLilycoveSSTidalDestinations[selection],
                    pixelWidth as i32,
                ) as u32;
            }
            j += 1;
        }
        width = ConvertPixelWidthToTileWidth(pixelWidth as i32) as u8;
        windowId = CreateWindowFromRect(
            MAX_MULTICHOICE_WIDTH as u8 - width,
            (6 - count) * 2,
            width,
            count * 2,
        );
        SetStandardWindowBorderStyle(windowId, FALSE);
        selectionCount = 0;
        i = 0;
        while i < SSTIDAL_SELECTION_COUNT {
            if sLilycoveSSTidalSelections[i] != 0xFF {
                AddTextPrinterParameterized(
                    windowId,
                    FONT_NORMAL,
                    sLilycoveSSTidalDestinations[sLilycoveSSTidalSelections[i]],
                    8,
                    selectionCount * 16 + 1,
                    TEXT_SKIP_DRAW,
                    None,
                );
                selectionCount += 1;
            }
            i += 1;
        }
        InitMenuInUpperLeftCornerNormal(windowId, count, count - 1);
        CopyWindowToVram(windowId, COPYWIN_FULL);
        InitMultichoiceCheckWrap(FALSE, count, windowId, MULTI_SSTIDAL_LILYCOVE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLilycoveSSTidalSelection() {
    if gSpecialVar_Result != MULTI_B_PRESSED {
        gSpecialVar_Result = sLilycoveSSTidalSelections[gSpecialVar_Result] as u16;
    }
}
pub(crate) unsafe extern "C" fn Task_PokemonPicWindow(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[0] += 1;
        }
        1 => {}
        2 => {
            FreeResourcesAndDestroySprite(
                &raw mut gSprites[(*task).data[2]],
                (*task).data[2] as u8,
            );
            (*task).data[0] += 1;
        }
        3 => {
            ClearToTransparentAndRemoveWindow((*task).data[5] as u8);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_ShowPokemonPic(species: u16, x: u8, y: u8) -> u8 {
    let mut taskId: u8 = 0;
    let mut spriteId: u8 = 0;
    if FindTaskIdByFunc(Some(Task_PokemonPicWindow)) != TASK_NONE {
        return FALSE;
    } else {
        spriteId = CreateMonSprite_PicBox(species, x as i16 * 8 + 40, y as i16 * 8 + 40, 0);
        taskId = CreateTask(Some(Task_PokemonPicWindow), 0x50);
        gTasks[taskId].data[5] = CreateWindowFromRect(x, y, 8, 8) as i16;
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[1] = species as i16;
        gTasks[taskId].data[2] = spriteId as i16;
        gSprites[spriteId].callback = Some(SpriteCallbackDummy);
        gSprites[spriteId].oam.set_priority(0);
        SetStandardWindowBorderStyle(gTasks[taskId].data[5] as u8, TRUE);
        ScheduleBgCopyTilemapToVram(0);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_HidePokemonPic() -> Option<unsafe extern "C" fn() -> u8> {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_PokemonPicWindow));
    if taskId == TASK_NONE {
        return None;
    }
    gTasks[taskId].data[0] += 1;
    return Some(IsPicboxClosed);
}
pub(crate) unsafe extern "C" fn IsPicboxClosed() -> u8 {
    if FindTaskIdByFunc(Some(Task_PokemonPicWindow)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateWindowFromRect(x: u8, y: u8, width: u8, height: u8) -> u8 {
    let mut template: WindowTemplate = zeroed();
    template = CreateWindowTemplate(0, x + 1, y + 1, width, height, 15, 100);
    let mut windowId: u8 = AddWindow(&raw mut template) as u8;
    PutWindowTilemap(windowId);
    return windowId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearToTransparentAndRemoveWindow(windowId: u8) {
    ClearStdWindowAndFrameToTransparent(windowId, TRUE);
    RemoveWindow(windowId);
}
pub(crate) unsafe extern "C" fn DrawLinkServicesMultichoiceMenu(multichoiceId: u8) {
    match multichoiceId {
        MULTI_WIRELESS_NO_BERRY => {
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sWirelessOptionsNoBerryCrush[Menu_GetCursorPos()],
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
        MULTI_CABLE_CLUB_WITH_RECORD_MIX => {
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sCableClubOptions_WithRecordMix[Menu_GetCursorPos()],
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
        MULTI_WIRELESS_NO_RECORD => {
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sWirelessOptions_NoRecordMix[Menu_GetCursorPos()],
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
        MULTI_WIRELESS_ALL_SERVICES => {
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sWirelessOptions_AllServices[Menu_GetCursorPos()],
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
        MULTI_WIRELESS_NO_RECORD_BERRY => {
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sWirelessOptions_NoRecordMixBerryCrush[Menu_GetCursorPos()],
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
        MULTI_CABLE_CLUB_NO_RECORD_MIX => {
            FillWindowPixelBuffer(0, 17);
            AddTextPrinterParameterized2(
                0,
                FONT_NORMAL,
                sCableClubOptions_NoRecordMix[Menu_GetCursorPos()],
                0,
                None,
                TEXT_COLOR_DARK_GRAY,
                TEXT_COLOR_WHITE,
                TEXT_COLOR_LIGHT_GRAY,
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_CreateStartMenuForPokenavTutorial() -> u16 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE as u16;
    } else {
        gSpecialVar_Result = 0xFF;
        CreateStartMenuForPokenavTutorial();
        return TRUE as u16;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CreateStartMenuForPokenavTutorial() {
    let mut windowId: u8 = CreateWindowFromRect(21, 0, 7, 18);
    SetStandardWindowBorderStyle(windowId, FALSE);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionPokedex.as_ptr().cast_mut(),
        8,
        9,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionPokemon.as_ptr().cast_mut(),
        8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionBag.as_ptr().cast_mut(),
        8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionPokenav.as_ptr().cast_mut(),
        8,
        57,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        8,
        73,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionSave.as_ptr().cast_mut(),
        8,
        89,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionOption.as_ptr().cast_mut(),
        8,
        105,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        gText_MenuOptionExit.as_ptr().cast_mut(),
        8,
        121,
        TEXT_SKIP_DRAW,
        None,
    );
    InitMenuNormal(windowId, FONT_NORMAL, 0, 9, 16, 8, 0);
    InitMultichoiceNoWrap(FALSE, 8, windowId, MULTI_FORCED_START_MENU);
    CopyWindowToVram(windowId, COPYWIN_FULL);
}
pub(crate) unsafe extern "C" fn InitMultichoiceNoWrap(
    ignoreBPress: u8,
    unusedCount: u8,
    windowId: u8,
    multichoiceId: u8,
) {
    let mut taskId: u8 = 0;
    sProcessInputDelay = 2;
    taskId = CreateTask(Some(Task_HandleMultichoiceInput), 80);
    gTasks[taskId].data[4] = ignoreBPress as i16;
    gTasks[taskId].data[5] = 0;
    gTasks[taskId].data[6] = windowId as i16;
    gTasks[taskId].data[7] = multichoiceId as i16;
}
pub(crate) unsafe extern "C" fn DisplayTextAndGetWidthInternal(str: *mut u8) -> i32 {
    let mut temp: CArray<u8, 64> = zeroed();
    StringExpandPlaceholders(temp.as_mut_ptr(), str);
    return GetStringWidth(FONT_NORMAL, temp.as_mut_ptr(), 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DisplayTextAndGetWidth(str: *mut u8, prevWidth: i32) -> i32 {
    let mut width: i32 = DisplayTextAndGetWidthInternal(str);
    if width < prevWidth {
        width = prevWidth;
    }
    return width;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ConvertPixelWidthToTileWidth(width: i32) -> i32 {
    return if (width + 9) / 8 + 1 > MAX_MULTICHOICE_WIDTH {
        MAX_MULTICHOICE_WIDTH
    } else {
        (width + 9) / 8 + 1
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptMenu_AdjustLeftCoordFromWidth(left: i32, width: i32) -> i32 {
    let mut adjustedLeft: i32 = left;
    if left + width > MAX_MULTICHOICE_WIDTH {
        if MAX_MULTICHOICE_WIDTH - width < 0 {
            adjustedLeft = 0;
        } else {
            adjustedLeft = MAX_MULTICHOICE_WIDTH - width;
        }
    }
    return adjustedLeft;
}
