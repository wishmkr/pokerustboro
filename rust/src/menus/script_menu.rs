//! Translated from `src/script_menu.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, FlagSet};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_Result};
use crate::field_effect::{CreateMonSprite_PicBox, FreeResourcesAndDestroySprite};
use crate::field_specials::ShowScrollableMultichoice;
use crate::item::CheckBagHasItem;
use crate::load_save::gSaveBlock2Ptr;
use crate::menu::{
    AddTextPrinterParameterized2, ClearStdWindowAndFrameToTransparent, CreateWindowTemplate,
    DisplayYesNoMenuDefaultYes, InitMenuActionGrid, InitMenuInUpperLeftCornerNormal,
    InitMenuNormal, LoadMessageBoxAndFrameGfx, Menu_GetCursorPos, Menu_ProcessGridInput,
    Menu_ProcessInput, Menu_ProcessInputNoWrap, Menu_ProcessInputNoWrapClearOnChoose,
    PrintMenuGridTable, PrintMenuTable, PrintPlayerNameOnWindow, ScheduleBgCopyTilemapToVram,
    SetStandardWindowBorderStyle,
};
use crate::palette::gPaletteFade;
use crate::script::ScriptContext_Enable;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::string_util::gStringVar4;
use crate::string_util::{StringExpandPlaceholders, StringLength};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set};
use crate::text::GetFontAttribute;
use crate::text::GetStringWidth;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{CopyWindowToVram, FillWindowPixelBuffer, PutWindowTilemap, RemoveWindow};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AddWindow` with this module's view of its types.
#[inline]
unsafe fn AddWindow(a0: *mut WindowTemplate) -> u16 {
    unsafe { crate::window::AddWindow(a0 as _) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tMonSpecies: usize = 1;
const tMonSpriteId: usize = 2;
const tRight: usize = 2;
const tIgnoreBPress: usize = 4;
const tDoWrap: usize = 5;
const tMultichoiceId: usize = 7;
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
pub(crate) static sProcessInputDelay: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static mut sLilycoveSSTidalSelections: Aligned<CArray<u8, 7>> =
    Aligned(unsafe { zeroed() });

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

pub unsafe fn ScriptMenu_Multichoice(left: u8, top: u8, multichoiceId: u8, ignoreBPress: u8) -> u8 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        DrawMultichoiceMenu(left, top, multichoiceId, ignoreBPress, 0);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ScriptMenu_MultichoiceWithDefault(
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
        0
    }
}
unsafe fn GetLengthWithExpandedPlayerName(mut str: *mut u8) -> u16 {
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
    length
}
unsafe fn DrawMultichoiceMenu(
    mut left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    cursorPos: u8,
) {
    let count: u8 = sMultichoiceLists[multichoiceId].count;
    let actions: *mut MenuAction = sMultichoiceLists[multichoiceId].list;
    let mut width: i32 = 0;
    for i in 0..(count as i32) {
        width = DisplayTextAndGetWidth((*actions.at(i)).text, width);
    }
    let newWidth: u8 = ConvertPixelWidthToTileWidth(width) as u8;
    left = ScriptMenu_AdjustLeftCoordFromWidth(left as i32, newWidth as i32) as u8;
    let windowId: u8 = CreateWindowFromRect(left, top, newWidth, count * 2);
    SetStandardWindowBorderStyle(windowId, FALSE);
    PrintMenuTable(windowId, count, actions);
    InitMenuInUpperLeftCornerNormal(windowId, count, cursorPos);
    ScheduleBgCopyTilemapToVram(0);
    InitMultichoiceCheckWrap(ignoreBPress, count, windowId, multichoiceId);
}
unsafe fn InitMultichoiceCheckWrap(ignoreBPress: u8, count: u8, windowId: u8, multichoiceId: u8) {
    sProcessInputDelay.set(2);
    for i in 0..6u8 {
        if sLinkServicesMultichoiceIds[i] == multichoiceId {
            sProcessInputDelay.set(12);
        }
    }
    let taskId: u8 = CreateTask(Some(Task_HandleMultichoiceInput), 80);
    task_set(taskId, tIgnoreBPress, ignoreBPress as i16);
    if count > 3 {
        task_set(taskId, tDoWrap, TRUE as i16);
    } else {
        task_set(taskId, tDoWrap, FALSE as i16);
    }
    task_set(taskId, 6, windowId as i16);
    task_set(taskId, tMultichoiceId, multichoiceId as i16);
    DrawLinkServicesMultichoiceMenu(multichoiceId);
}
pub(crate) unsafe fn Task_HandleMultichoiceInput(taskId: u8) {
    let mut selection: i8 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        if sProcessInputDelay.get() != 0 {
            sProcessInputDelay.set(sProcessInputDelay.get() - 1);
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
pub unsafe fn ScriptMenu_YesNo(left: u8, top: u8) -> u8 {
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
        0
    }
}
pub unsafe fn IsScriptActive() -> u8 {
    if gSpecialVar_Result == 0xFF {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_HandleYesNoInput(taskId: u8) {
    if task_get(taskId, tRight) < 5 {
        task_set(taskId, tRight, task_get(taskId, tRight) + 1);
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
pub unsafe fn ScriptMenu_MultichoiceGrid(
    mut left: u8,
    top: u8,
    multichoiceId: u8,
    ignoreBPress: u8,
    columnCount: u8,
) -> u8 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceGridInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        let mut width: i32 = 0;
        let mut i: i32 = 0;
        while i < sMultichoiceLists[multichoiceId].count as i32 {
            width =
                DisplayTextAndGetWidth((*sMultichoiceLists[multichoiceId].list.at(i)).text, width);
            i += 1;
        }
        let newWidth: u8 = ConvertPixelWidthToTileWidth(width) as u8;
        left =
            ScriptMenu_AdjustLeftCoordFromWidth(left as i32, columnCount as i32 * newWidth as i32)
                as u8;
        let rowCount: u8 = div_i32(
            sMultichoiceLists[multichoiceId].count as i32,
            columnCount as i32,
        ) as u8;
        let taskId: u8 = CreateTask(Some(Task_HandleMultichoiceGridInput), 80);
        task_set(taskId, tIgnoreBPress, ignoreBPress as i16);
        task_set(
            taskId,
            6,
            CreateWindowFromRect(left, top, columnCount * newWidth, rowCount * 2) as i16,
        );
        SetStandardWindowBorderStyle(task_get(taskId, 6) as u8, FALSE);
        PrintMenuGridTable(
            task_get(taskId, 6) as u8,
            newWidth * 8,
            columnCount,
            rowCount,
            sMultichoiceLists[multichoiceId].list,
        );
        InitMenuActionGrid(
            task_get(taskId, 6) as u8,
            newWidth * 8,
            columnCount,
            rowCount,
            0,
        );
        CopyWindowToVram(task_get(taskId, 6) as u8, COPYWIN_FULL);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Task_HandleMultichoiceGridInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let selection: i8 = Menu_ProcessGridInput();
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
pub unsafe fn ScriptMenu_CreatePCMultichoice() -> u16 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE as u16;
    } else {
        gSpecialVar_Result = 0xFF;
        CreatePCMultichoice();
        return TRUE as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CreatePCMultichoice() {
    let x: u8 = 8;
    let mut pixelWidth: u32 = 0;
    let mut numChoices: u8 = 0;
    let mut windowId: u8 = 0;
    for i in 0..4i32 {
        pixelWidth = DisplayTextAndGetWidth(sPCNameStrings[i], pixelWidth as i32) as u32;
    }
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        pixelWidth = DisplayTextAndGetWidth(
            (*(&raw const crate::data::strings::gText_HallOfFame).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            pixelWidth as i32,
        ) as u32;
    }
    let width: u8 = ConvertPixelWidthToTileWidth(pixelWidth as i32) as u8;
    if FlagGet(FLAG_SYS_GAME_CLEAR) != 0 {
        numChoices = 4;
        windowId = CreateWindowFromRect(0, 0, width, 8);
        SetStandardWindowBorderStyle(windowId, FALSE);
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_HallOfFame).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            x,
            33,
            TEXT_SKIP_DRAW,
            None,
        );
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_LogOff).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
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
            (*(&raw const crate::data::strings::gText_LogOff).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
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
            (*(&raw const crate::data::strings::gText_LanettesPC).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            x,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
    } else {
        AddTextPrinterParameterized(
            windowId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_SomeonesPC).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            x,
            1,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PlayersPC).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    PrintPlayerNameOnWindow(windowId, gStringVar4.as_mut_ptr(), x as u16, 17);
    InitMenuInUpperLeftCornerNormal(windowId, numChoices, 0);
    CopyWindowToVram(windowId, COPYWIN_FULL);
    InitMultichoiceCheckWrap(FALSE, numChoices, windowId, MULTI_PC);
}
#[unsafe(no_mangle)]
pub unsafe fn ScriptMenu_DisplayPCStartupPrompt() {
    LoadMessageBoxAndFrameGfx(0, TRUE);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        (*crate::asmdata::gText_WhichPCShouldBeAccessed.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        None,
        TEXT_COLOR_DARK_GRAY,
        TEXT_COLOR_WHITE,
        TEXT_COLOR_LIGHT_GRAY,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ScriptMenu_CreateLilycoveSSTidalMultichoice() -> u8 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE;
    } else {
        gSpecialVar_Result = 0xFF;
        CreateLilycoveSSTidalMultichoice();
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CreateLilycoveSSTidalMultichoice() {
    let mut selectionCount: u8 = 0;
    let mut count: u8 = 0;
    let mut pixelWidth: u32 = 0;
    let mut width: u8 = 0;
    let mut windowId: u8 = 0;
    let mut i: u8 = 0;
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
        for j in 0..(SSTIDAL_SELECTION_COUNT as u32) {
            let selection: u8 = sLilycoveSSTidalSelections[j];
            if selection != 0xFF {
                pixelWidth = DisplayTextAndGetWidth(
                    sLilycoveSSTidalDestinations[selection],
                    pixelWidth as i32,
                ) as u32;
            }
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
        for i in 0..SSTIDAL_SELECTION_COUNT {
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
        }
        InitMenuInUpperLeftCornerNormal(windowId, count, count - 1);
        CopyWindowToVram(windowId, COPYWIN_FULL);
        InitMultichoiceCheckWrap(FALSE, count, windowId, MULTI_SSTIDAL_LILYCOVE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetLilycoveSSTidalSelection() {
    if gSpecialVar_Result != MULTI_B_PRESSED {
        gSpecialVar_Result = sLilycoveSSTidalSelections
            [*(&raw const crate::ffi::gSpecialVar_Result)
                .cast::<u16>()
                .cast_mut()] as u16;
    }
}
pub(crate) unsafe fn Task_PokemonPicWindow(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tState] += 1;
        }
        1 => {}
        2 => {
            FreeResourcesAndDestroySprite(
                &raw mut gSprites[(*task).data[tMonSpriteId]],
                (*task).data[tMonSpriteId] as u8,
            );
            (*task).data[tState] += 1;
        }
        3 => {
            ClearToTransparentAndRemoveWindow((*task).data[5] as u8);
            DestroyTask(taskId);
        }
        _ => {}
    }
}
pub unsafe fn ScriptMenu_ShowPokemonPic(species: u16, x: u8, y: u8) -> u8 {
    let mut taskId: u8 = 0;
    let mut spriteId: u8 = 0;
    if FindTaskIdByFunc(Some(Task_PokemonPicWindow)) != TASK_NONE {
        return FALSE;
    } else {
        spriteId = CreateMonSprite_PicBox(species, x as i16 * 8 + 40, y as i16 * 8 + 40, 0);
        taskId = CreateTask(Some(Task_PokemonPicWindow), 0x50);
        task_set(taskId, 5, CreateWindowFromRect(x, y, 8, 8) as i16);
        task_set(taskId, tState, 0);
        task_set(taskId, tMonSpecies, species as i16);
        task_set(taskId, tMonSpriteId, spriteId as i16);
        gSprites[spriteId].callback = Some(SpriteCallbackDummy);
        gSprites[spriteId].oam.set_priority(0);
        SetStandardWindowBorderStyle(task_get(taskId, 5) as u8, TRUE);
        ScheduleBgCopyTilemapToVram(0);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScriptMenu_HidePokemonPic() -> Option<unsafe fn() -> u8> {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_PokemonPicWindow));
    if taskId == TASK_NONE {
        return None;
    }
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    Some(IsPicboxClosed)
}
pub(crate) unsafe fn IsPicboxClosed() -> u8 {
    if FindTaskIdByFunc(Some(Task_PokemonPicWindow)) == TASK_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn CreateWindowFromRect(x: u8, y: u8, width: u8, height: u8) -> u8 {
    let mut template: WindowTemplate =
        CreateWindowTemplate(0, x + 1, y + 1, width, height, 15, 100);
    let windowId: u8 = AddWindow(&raw mut template) as u8;
    PutWindowTilemap(windowId);
    windowId
}
pub unsafe fn ClearToTransparentAndRemoveWindow(windowId: u8) {
    ClearStdWindowAndFrameToTransparent(windowId, TRUE);
    RemoveWindow(windowId);
}
unsafe fn DrawLinkServicesMultichoiceMenu(multichoiceId: u8) {
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
pub unsafe fn ScriptMenu_CreateStartMenuForPokenavTutorial() -> u16 {
    if FuncIsActiveTask(Some(Task_HandleMultichoiceInput)) == TRUE {
        return FALSE as u16;
    } else {
        gSpecialVar_Result = 0xFF;
        CreateStartMenuForPokenavTutorial();
        return TRUE as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CreateStartMenuForPokenavTutorial() {
    let windowId: u8 = CreateWindowFromRect(21, 0, 7, 18);
    SetStandardWindowBorderStyle(windowId, FALSE);
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MenuOptionPokedex).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        9,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MenuOptionPokemon).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        25,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MenuOptionBag).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        41,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MenuOptionPokenav).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
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
        (*(&raw const crate::data::strings::gText_MenuOptionSave).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        89,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MenuOptionOption).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        105,
        TEXT_SKIP_DRAW,
        None,
    );
    AddTextPrinterParameterized(
        windowId,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_MenuOptionExit).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        8,
        121,
        TEXT_SKIP_DRAW,
        None,
    );
    InitMenuNormal(windowId, FONT_NORMAL, 0, 9, 16, 8, 0);
    InitMultichoiceNoWrap(FALSE, 8, windowId, MULTI_FORCED_START_MENU);
    CopyWindowToVram(windowId, COPYWIN_FULL);
}
unsafe fn InitMultichoiceNoWrap(
    ignoreBPress: u8,
    unusedCount: u8,
    windowId: u8,
    multichoiceId: u8,
) {
    sProcessInputDelay.set(2);
    let taskId: u8 = CreateTask(Some(Task_HandleMultichoiceInput), 80);
    task_set(taskId, tIgnoreBPress, ignoreBPress as i16);
    task_set(taskId, tDoWrap, 0);
    task_set(taskId, 6, windowId as i16);
    task_set(taskId, tMultichoiceId, multichoiceId as i16);
}
unsafe fn DisplayTextAndGetWidthInternal(str: *mut u8) -> i32 {
    let mut temp: CArray<u8, 64> = zeroed();
    StringExpandPlaceholders(temp.as_mut_ptr(), str);
    GetStringWidth(FONT_NORMAL, temp.as_mut_ptr(), 0)
}
pub unsafe fn DisplayTextAndGetWidth(str: *mut u8, prevWidth: i32) -> i32 {
    let mut width: i32 = DisplayTextAndGetWidthInternal(str);
    if width < prevWidth {
        width = prevWidth;
    }
    width
}
#[unsafe(no_mangle)]
pub unsafe fn ConvertPixelWidthToTileWidth(width: i32) -> i32 {
    if (width + 9) / 8 + 1 > MAX_MULTICHOICE_WIDTH {
        MAX_MULTICHOICE_WIDTH
    } else {
        (width + 9) / 8 + 1
    }
}
pub fn ScriptMenu_AdjustLeftCoordFromWidth(left: i32, width: i32) -> i32 {
    let mut adjustedLeft: i32 = left;
    if left + width > MAX_MULTICHOICE_WIDTH {
        if MAX_MULTICHOICE_WIDTH - width < 0 {
            adjustedLeft = 0;
        } else {
            adjustedLeft = MAX_MULTICHOICE_WIDTH - width;
        }
    }
    adjustedLeft
}
