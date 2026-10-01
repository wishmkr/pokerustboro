//! Translated from `src/start_menu.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sWindowTemplate_SafariBalls sPyramidFloorNames sWindowTemplate_PyramidFloor sWindowTemplate_PyramidPeak sStartMenuItems sBgTemplates_LinkBattleSave sWindowTemplates_LinkBattleSave sSaveInfoWindowTemplate

const MENU_ACTION_BAG: u8 = 2;
const MENU_ACTION_EXIT: u8 = 7;
const MENU_ACTION_OPTION: u8 = 6;
const MENU_ACTION_PLAYER: u8 = 4;
const MENU_ACTION_PLAYER_LINK: u8 = 9;
const MENU_ACTION_POKEDEX: u8 = 0;
const MENU_ACTION_POKEMON: u8 = 1;
const MENU_ACTION_POKENAV: u8 = 3;
const MENU_ACTION_PYRAMID_BAG: u8 = 12;
const MENU_ACTION_REST_FRONTIER: u8 = 10;
const MENU_ACTION_RETIRE_FRONTIER: u8 = 11;
const MENU_ACTION_RETIRE_SAFARI: u8 = 8;
const MENU_ACTION_SAVE: u8 = 5;
const SAVE_CANCELED: u8 = 2;
const SAVE_ERROR: u8 = 3;
const SAVE_IN_PROGRESS: u8 = 0;
const SAVE_SUCCESS: u8 = 1;

static sBgTemplates_LinkBattleSave: Table<CArray<BgTemplate, 1>> =
    Table((&raw const crate::data::start_menu::sBgTemplates_LinkBattleSave).cast());
static sPyramidFloorNames: Table<CArray<*mut u8, 8>> =
    Table((&raw const crate::data::start_menu::sPyramidFloorNames).cast());
static sSaveInfoWindowTemplate: Table<WindowTemplate> =
    Table((&raw const crate::data::start_menu::sSaveInfoWindowTemplate).cast());
static sStartMenuItems: Table<CArray<MenuAction, 13>> =
    Table((&raw const crate::data::start_menu::sStartMenuItems).cast());
static sWindowTemplate_PyramidFloor: Table<WindowTemplate> =
    Table((&raw const crate::data::start_menu::sWindowTemplate_PyramidFloor).cast());
static sWindowTemplate_PyramidPeak: Table<WindowTemplate> =
    Table((&raw const crate::data::start_menu::sWindowTemplate_PyramidPeak).cast());
static sWindowTemplate_SafariBalls: Table<WindowTemplate> =
    Table((&raw const crate::data::start_menu::sWindowTemplate_SafariBalls).cast());
static sWindowTemplates_LinkBattleSave: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::start_menu::sWindowTemplates_LinkBattleSave).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gMenuCallback: Option<unsafe extern "C" fn() -> u8> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSafariBallsWindowId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlePyramidFloorWindowId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sStartMenuCursorPos: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sNumStartMenuActions: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurrentStartMenuActions: Aligned<CArray<u8, 9>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInitStartMenuData: Aligned<CArray<i8, 2>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSaveDialogCallback: Option<unsafe extern "C" fn() -> u8> = None;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSaveDialogTimer: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSavingComplete: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSaveInfoWindowId: u8 = 0;

unsafe extern "C" {
    static BattlePyramid_Retire: CArray<u8, 0>;
    static mut gDifferentSaveFile: u8;
    static mut gFieldCallback2: Option<unsafe extern "C" fn() -> u8>;
    static mut gLocalLinkPlayerId: u8;
    static mut gMain: Main;
    static mut gNumSafariBalls: u8;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSaveFileStatus: u16;
    static mut gSoftResetDisabled: u8;
    static mut gSpecialVar_Result: u16;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_AlreadySavedFile: CArray<u8, 0>;
    static gText_BattlePyramidConfirmRest: CArray<u8, 0>;
    static gText_BattlePyramidConfirmRetire: CArray<u8, 0>;
    static gText_BattlePyramidFloor: CArray<u8, 0>;
    static gText_ConfirmSave: CArray<u8, 0>;
    static gText_DifferentSaveFile: CArray<u8, 0>;
    static gText_PlayerSavedGame: CArray<u8, 0>;
    static gText_SafariBallStock: CArray<u8, 0>;
    static gText_SaveError: CArray<u8, 0>;
    static gText_SavingBadges: CArray<u8, 0>;
    static gText_SavingDontTurnOff: CArray<u8, 0>;
    static gText_SavingDontTurnOffPower: CArray<u8, 0>;
    static gText_SavingPlayer: CArray<u8, 0>;
    static gText_SavingPokedex: CArray<u8, 0>;
    static gText_SavingTime: CArray<u8, 0>;
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
    fn InitBgsFromTemplates(a0: u8, a1: *mut BgTemplate, a2: u8);
    fn InitMenuNormal(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn InitWindows(a0: *mut WindowTemplate) -> u16;
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
    FlagSet(FLAG_SYS_POKEDEX_GET);
    FlagSet(FLAG_SYS_POKEMON_GET);
    FlagSet(FLAG_SYS_POKENAV_GET);
}
pub(crate) unsafe extern "C" fn BuildStartMenuActions() {
    sNumStartMenuActions = 0;
    if IsOverworldLinkActive() == TRUE as u32 {
        BuildLinkModeStartMenu();
    } else if InUnionRoom() == TRUE as u32 {
        BuildUnionRoomStartMenu();
    } else if GetSafariZoneFlag() == TRUE as u32 {
        BuildSafariZoneStartMenu();
    } else if InBattlePike() != 0 {
        BuildBattlePikeStartMenu();
    } else if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        BuildBattlePyramidStartMenu();
    } else if InMultiPartnerRoom() != 0 {
        BuildMultiPartnerRoomStartMenu();
    } else {
        BuildNormalStartMenu();
    }
}
pub(crate) unsafe extern "C" fn AddStartMenuAction(action: u8) {
    AppendToList(
        sCurrentStartMenuActions.as_mut_ptr(),
        &raw mut sNumStartMenuActions,
        action,
    );
}
pub(crate) unsafe extern "C" fn BuildNormalStartMenu() {
    if FlagGet(FLAG_SYS_POKEDEX_GET) == TRUE {
        AddStartMenuAction(MENU_ACTION_POKEDEX);
    }
    if FlagGet(FLAG_SYS_POKEMON_GET) == TRUE {
        AddStartMenuAction(MENU_ACTION_POKEMON);
    }
    AddStartMenuAction(MENU_ACTION_BAG);
    if FlagGet(FLAG_SYS_POKENAV_GET) == TRUE {
        AddStartMenuAction(MENU_ACTION_POKENAV);
    }
    AddStartMenuAction(MENU_ACTION_PLAYER);
    AddStartMenuAction(MENU_ACTION_SAVE);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn BuildSafariZoneStartMenu() {
    AddStartMenuAction(MENU_ACTION_RETIRE_SAFARI);
    AddStartMenuAction(MENU_ACTION_POKEDEX);
    AddStartMenuAction(MENU_ACTION_POKEMON);
    AddStartMenuAction(MENU_ACTION_BAG);
    AddStartMenuAction(MENU_ACTION_PLAYER);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn BuildLinkModeStartMenu() {
    AddStartMenuAction(MENU_ACTION_POKEMON);
    AddStartMenuAction(MENU_ACTION_BAG);
    if FlagGet(FLAG_SYS_POKENAV_GET) == TRUE {
        AddStartMenuAction(MENU_ACTION_POKENAV);
    }
    AddStartMenuAction(MENU_ACTION_PLAYER_LINK);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn BuildUnionRoomStartMenu() {
    AddStartMenuAction(MENU_ACTION_POKEMON);
    AddStartMenuAction(MENU_ACTION_BAG);
    if FlagGet(FLAG_SYS_POKENAV_GET) == TRUE {
        AddStartMenuAction(MENU_ACTION_POKENAV);
    }
    AddStartMenuAction(MENU_ACTION_PLAYER);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn BuildBattlePikeStartMenu() {
    AddStartMenuAction(MENU_ACTION_POKEDEX);
    AddStartMenuAction(MENU_ACTION_POKEMON);
    AddStartMenuAction(MENU_ACTION_PLAYER);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn BuildBattlePyramidStartMenu() {
    AddStartMenuAction(MENU_ACTION_POKEMON);
    AddStartMenuAction(MENU_ACTION_PYRAMID_BAG);
    AddStartMenuAction(MENU_ACTION_PLAYER);
    AddStartMenuAction(MENU_ACTION_REST_FRONTIER);
    AddStartMenuAction(MENU_ACTION_RETIRE_FRONTIER);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn BuildMultiPartnerRoomStartMenu() {
    AddStartMenuAction(MENU_ACTION_POKEMON);
    AddStartMenuAction(MENU_ACTION_PLAYER);
    AddStartMenuAction(MENU_ACTION_OPTION);
    AddStartMenuAction(MENU_ACTION_EXIT);
}
pub(crate) unsafe extern "C" fn ShowSafariBallsWindow() {
    sSafariBallsWindowId = AddWindow((&raw const *sWindowTemplate_SafariBalls).cast_mut()) as u8;
    PutWindowTilemap(sSafariBallsWindowId);
    DrawStdWindowFrame(sSafariBallsWindowId, FALSE);
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        gNumSafariBalls as i32,
        STR_CONV_MODE_RIGHT_ALIGN,
        2,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_SafariBallStock.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        sSafariBallsWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sSafariBallsWindowId, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn ShowPyramidFloorWindow() {
    if (*gSaveBlock2Ptr).frontier.curChallengeBattleNum == FRONTIER_STAGES_PER_CHALLENGE {
        sBattlePyramidFloorWindowId =
            AddWindow((&raw const *sWindowTemplate_PyramidPeak).cast_mut()) as u8;
    } else {
        sBattlePyramidFloorWindowId =
            AddWindow((&raw const *sWindowTemplate_PyramidFloor).cast_mut()) as u8;
    }
    PutWindowTilemap(sBattlePyramidFloorWindowId);
    DrawStdWindowFrame(sBattlePyramidFloorWindowId, FALSE);
    StringCopy(
        gStringVar1.as_mut_ptr(),
        sPyramidFloorNames[(*gSaveBlock2Ptr).frontier.curChallengeBattleNum],
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_BattlePyramidFloor.as_ptr().cast_mut(),
    );
    AddTextPrinterParameterized(
        sBattlePyramidFloorWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sBattlePyramidFloorWindowId, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn RemoveExtraStartMenuWindows() {
    if GetSafariZoneFlag() != 0 {
        ClearStdWindowAndFrameToTransparent(sSafariBallsWindowId, FALSE);
        CopyWindowToVram(sSafariBallsWindowId, COPYWIN_GFX);
        RemoveWindow(sSafariBallsWindowId);
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        ClearStdWindowAndFrameToTransparent(sBattlePyramidFloorWindowId, FALSE);
        RemoveWindow(sBattlePyramidFloorWindowId);
    }
}
pub(crate) unsafe extern "C" fn PrintStartMenuActions(pIndex: *mut i8, mut count: u32) -> u32 {
    let mut index: i8 = *pIndex;
    loop {
        if sStartMenuItems[sCurrentStartMenuActions[index]]
            .func
            .u8_void
            == Some(StartMenuPlayerNameCallback as unsafe extern "C" fn() -> u8)
        {
            PrintPlayerNameOnWindow(
                GetStartMenuWindowId(),
                sStartMenuItems[sCurrentStartMenuActions[index]].text,
                8,
                ((index as u16) << 4) + 9,
            );
        } else {
            StringExpandPlaceholders(
                gStringVar4.as_mut_ptr(),
                sStartMenuItems[sCurrentStartMenuActions[index]].text,
            );
            AddTextPrinterParameterized(
                GetStartMenuWindowId(),
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                8,
                ((index as u8) << 4) + 9,
                TEXT_SKIP_DRAW,
                None,
            );
        }
        index += 1;
        if index as i32 >= sNumStartMenuActions as i32 {
            *pIndex = index;
            return TRUE as u32;
        }
        count -= 1;
        if count == 0 {
            break;
        }
    }
    *pIndex = index;
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn InitStartMenuStep() -> u32 {
    let mut state: i8 = sInitStartMenuData[0];
    match state {
        0 => {
            sInitStartMenuData[0] += 1;
        }
        1 => {
            BuildStartMenuActions();
            sInitStartMenuData[0] += 1;
        }
        2 => {
            LoadMessageBoxAndBorderGfx();
            DrawStdWindowFrame(AddStartMenuWindow(sNumStartMenuActions), FALSE);
            sInitStartMenuData[1] = 0;
            sInitStartMenuData[0] += 1;
        }
        3 => {
            if GetSafariZoneFlag() != 0 {
                ShowSafariBallsWindow();
            }
            if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
                ShowPyramidFloorWindow();
            }
            sInitStartMenuData[0] += 1;
        }
        4 => {
            if PrintStartMenuActions(&raw mut sInitStartMenuData[1], 2) != 0 {
                sInitStartMenuData[0] += 1;
            }
        }
        5 => {
            sStartMenuCursorPos = InitMenuNormal(
                GetStartMenuWindowId(),
                FONT_NORMAL,
                0,
                9,
                16,
                sNumStartMenuActions,
                sStartMenuCursorPos,
            );
            CopyWindowToVram(GetStartMenuWindowId(), COPYWIN_MAP);
            return TRUE as u32;
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn InitStartMenu() {
    sInitStartMenuData[0] = 0;
    sInitStartMenuData[1] = 0;
    while InitStartMenuStep() == 0 {}
}
pub(crate) unsafe extern "C" fn StartMenuTask(taskId: u8) {
    if InitStartMenuStep() == TRUE as u32 {
        SwitchTaskToFollowupFunc(taskId);
    }
}
pub(crate) unsafe extern "C" fn CreateStartMenuTask(
    followupFunc: Option<unsafe extern "C" fn(u8)>,
) {
    let mut taskId: u8 = 0;
    sInitStartMenuData[0] = 0;
    sInitStartMenuData[1] = 0;
    taskId = CreateTask(Some(StartMenuTask), 0x50);
    SetTaskFuncWithFollowupFunc(taskId, Some(StartMenuTask), followupFunc);
}
pub(crate) unsafe extern "C" fn FieldCB_ReturnToFieldStartMenu() -> u8 {
    if InitStartMenuStep() == FALSE as u32 {
        return FALSE;
    }
    ReturnToFieldOpenStartMenu();
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowReturnToFieldStartMenu() {
    sInitStartMenuData[0] = 0;
    sInitStartMenuData[1] = 0;
    gFieldCallback2 = Some(FieldCB_ReturnToFieldStartMenu);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_ShowStartMenu(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            if InUnionRoom() == TRUE as u32 {
                SetUsingUnionRoomStartMenu();
            }
            gMenuCallback = Some(HandleStartMenuInput);
            (*task).data[0] += 1;
        }
        1 => {
            if gMenuCallback.unwrap_unchecked()() == TRUE {
                DestroyTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowStartMenu() {
    if IsOverworldLinkActive() == 0 {
        FreezeObjectEvents();
        PlayerFreeze();
        StopPlayerAvatar();
    }
    CreateStartMenuTask(Some(Task_ShowStartMenu));
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn HandleStartMenuInput() -> u8 {
    if gMain.newKeys as i32 & DPAD_UP != 0 {
        PlaySE(SE_SELECT);
        sStartMenuCursorPos = Menu_MoveCursor(-1);
    }
    if gMain.newKeys as i32 & DPAD_DOWN != 0 {
        PlaySE(SE_SELECT);
        sStartMenuCursorPos = Menu_MoveCursor(1);
    }
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        if sStartMenuItems[sCurrentStartMenuActions[sStartMenuCursorPos]]
            .func
            .u8_void
            == Some(StartMenuPokedexCallback as unsafe extern "C" fn() -> u8)
        {
            if GetNationalPokedexCount(FLAG_GET_SEEN) == 0 {
                return FALSE;
            }
        }
        gMenuCallback = sStartMenuItems[sCurrentStartMenuActions[sStartMenuCursorPos]]
            .func
            .u8_void;
        if gMenuCallback != Some(StartMenuSaveCallback as unsafe extern "C" fn() -> u8)
            && gMenuCallback != Some(StartMenuExitCallback as unsafe extern "C" fn() -> u8)
            && gMenuCallback
                != Some(StartMenuSafariZoneRetireCallback as unsafe extern "C" fn() -> u8)
            && gMenuCallback
                != Some(StartMenuBattlePyramidRetireCallback as unsafe extern "C" fn() -> u8)
        {
            FadeScreen(FADE_TO_BLACK, 0);
        }
        return FALSE;
    }
    if gMain.newKeys as i32 & 10 != 0 {
        RemoveExtraStartMenuWindows();
        HideStartMenu();
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuPokedexCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        IncrementGameStat(GAME_STAT_CHECKED_POKEDEX);
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_OpenPokedex));
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuPokemonCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_PartyMenuFromStartMenu));
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuBagCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_BagMenuFromStartMenu));
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuPokeNavCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_InitPokeNav));
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuPlayerNameCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        if IsOverworldLinkActive() != 0 || InUnionRoom() != 0 {
            ShowPlayerTrainerCard(Some(CB2_ReturnToFieldWithOpenMenu));
        } else if FlagGet(FLAG_SYS_FRONTIER_PASS) != 0 {
            ShowFrontierPass(Some(CB2_ReturnToFieldWithOpenMenu));
        } else {
            ShowPlayerTrainerCard(Some(CB2_ReturnToFieldWithOpenMenu));
        }
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuSaveCallback() -> u8 {
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        RemoveExtraStartMenuWindows();
    }
    gMenuCallback = Some(SaveStartCallback);
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuOptionCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_InitOptionMenu));
        gMain.savedCallback = Some(CB2_ReturnToFieldWithOpenMenu);
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuExitCallback() -> u8 {
    RemoveExtraStartMenuWindows();
    HideStartMenu();
    return TRUE;
}
pub(crate) unsafe extern "C" fn StartMenuSafariZoneRetireCallback() -> u8 {
    RemoveExtraStartMenuWindows();
    HideStartMenu();
    SafariZoneRetirePrompt();
    return TRUE;
}
pub(crate) unsafe extern "C" fn StartMenuLinkModePlayerNameCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        CleanupOverworldWindowsAndTilemaps();
        ShowTrainerCardInLink(gLocalLinkPlayerId, Some(CB2_ReturnToFieldWithOpenMenu));
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn StartMenuBattlePyramidRetireCallback() -> u8 {
    gMenuCallback = Some(BattlePyramidRetireStartCallback);
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBattlePyramidStartMenu() {
    ClearDialogWindowAndFrameToTransparent(0, 0);
    ScriptUnfreezeObjectEvents();
    CreateStartMenuTask(Some(Task_ShowStartMenu));
    LockPlayerFieldControls();
}
pub(crate) unsafe extern "C" fn StartMenuBattlePyramidBagCallback() -> u8 {
    if gPaletteFade.active() == 0 {
        PlayRainStoppingSoundEffect();
        RemoveExtraStartMenuWindows();
        CleanupOverworldWindowsAndTilemaps();
        SetMainCallback2(Some(CB2_PyramidBagMenuFromStartMenu));
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SaveStartCallback() -> u8 {
    InitSave();
    gMenuCallback = Some(SaveCallback);
    return FALSE;
}
pub(crate) unsafe extern "C" fn SaveCallback() -> u8 {
    match RunSaveCallback() {
        SAVE_IN_PROGRESS => {
            return FALSE;
        }
        SAVE_CANCELED => {
            ClearDialogWindowAndFrameToTransparent(0, 0);
            InitStartMenu();
            gMenuCallback = Some(HandleStartMenuInput);
            return FALSE;
        }
        SAVE_SUCCESS | SAVE_ERROR => {
            ClearDialogWindowAndFrameToTransparent(0, TRUE);
            ScriptUnfreezeObjectEvents();
            UnlockPlayerFieldControls();
            SoftResetInBattlePyramid();
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireStartCallback() -> u8 {
    InitBattlePyramidRetire();
    gMenuCallback = Some(BattlePyramidRetireCallback);
    return FALSE;
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireReturnCallback() -> u8 {
    InitStartMenu();
    gMenuCallback = Some(HandleStartMenuInput);
    return FALSE;
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireCallback() -> u8 {
    match RunSaveCallback() {
        SAVE_SUCCESS => {
            RemoveExtraStartMenuWindows();
            gMenuCallback = Some(BattlePyramidRetireReturnCallback);
            return FALSE;
        }
        SAVE_IN_PROGRESS => {
            return FALSE;
        }
        SAVE_CANCELED => {
            ClearDialogWindowAndFrameToTransparent(0, TRUE);
            ScriptUnfreezeObjectEvents();
            UnlockPlayerFieldControls();
            ScriptContext_SetupScript(BattlePyramid_Retire.as_ptr().cast_mut());
            return TRUE;
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn InitSave() {
    SaveMapView();
    sSaveDialogCallback = Some(SaveConfirmSaveCallback);
    sSavingComplete = FALSE;
}
pub(crate) unsafe extern "C" fn RunSaveCallback() -> u8 {
    if RunTextPrintersAndIsPrinter0Active() == TRUE as u16 {
        return SAVE_IN_PROGRESS;
    }
    sSavingComplete = FALSE;
    return sSaveDialogCallback.unwrap_unchecked()();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveGame() {
    InitSave();
    CreateTask(Some(SaveGameTask), 0x50);
}
pub(crate) unsafe extern "C" fn ShowSaveMessage(
    message: *mut u8,
    saveCallback: Option<unsafe extern "C" fn() -> u8>,
) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), message);
    LoadMessageBoxAndFrameGfx(0, TRUE);
    AddTextPrinterForMessage_2(TRUE);
    sSavingComplete = TRUE;
    sSaveDialogCallback = saveCallback;
}
pub(crate) unsafe extern "C" fn SaveGameTask(taskId: u8) {
    let mut status: u8 = RunSaveCallback();
    match status {
        SAVE_CANCELED | SAVE_ERROR => {
            gSpecialVar_Result = 0;
        }
        SAVE_SUCCESS => {
            gSpecialVar_Result = status as u16;
        }
        SAVE_IN_PROGRESS => {
            return;
        }
        _ => {}
    }
    DestroyTask(taskId);
    ScriptContext_Enable();
}
pub(crate) unsafe extern "C" fn HideSaveMessageWindow() {
    ClearDialogWindowAndFrame(0, TRUE);
}
pub(crate) unsafe extern "C" fn HideSaveInfoWindow() {
    RemoveSaveInfoWindow();
}
pub(crate) unsafe extern "C" fn SaveStartTimer() {
    sSaveDialogTimer = 60;
}
pub(crate) unsafe extern "C" fn SaveSuccesTimer() -> u8 {
    sSaveDialogTimer -= 1;
    if gMain.heldKeys as i32 & A_BUTTON != 0 {
        PlaySE(SE_SELECT);
        return TRUE;
    }
    if sSaveDialogTimer == 0 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SaveErrorTimer() -> u8 {
    if sSaveDialogTimer != 0 {
        sSaveDialogTimer -= 1;
    } else if gMain.heldKeys as i32 & A_BUTTON != 0 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn SaveConfirmSaveCallback() -> u8 {
    ClearStdWindowAndFrame(GetStartMenuWindowId(), FALSE);
    RemoveStartMenuWindow();
    ShowSaveInfoWindow();
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        ShowSaveMessage(
            gText_BattlePyramidConfirmRest.as_ptr().cast_mut(),
            Some(SaveYesNoCallback),
        );
    } else {
        ShowSaveMessage(
            gText_ConfirmSave.as_ptr().cast_mut(),
            Some(SaveYesNoCallback),
        );
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveYesNoCallback() -> u8 {
    DisplayYesNoMenuDefaultYes();
    sSaveDialogCallback = Some(SaveConfirmInputCallback);
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveConfirmInputCallback() -> u8 {
    'l1: {
        let sw1: i8 = Menu_ProcessInputNoWrapClearOnChoose();
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            match gSaveFileStatus {
                0 | SAVE_STATUS_CORRUPT => {
                    if gDifferentSaveFile == FALSE {
                        sSaveDialogCallback = Some(SaveFileExistsCallback);
                        return SAVE_IN_PROGRESS;
                    }
                    sSaveDialogCallback = Some(SaveSavingMessageCallback);
                    return SAVE_IN_PROGRESS;
                }
                _ => {
                    sSaveDialogCallback = Some(SaveFileExistsCallback);
                    return SAVE_IN_PROGRESS;
                }
            }
        }
        if fall || sw1 == MENU_B_PRESSED || sw1 == 1 {
            fall = true;
            HideSaveInfoWindow();
            HideSaveMessageWindow();
            return SAVE_CANCELED;
        }
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveFileExistsCallback() -> u8 {
    if gDifferentSaveFile == TRUE {
        ShowSaveMessage(
            gText_DifferentSaveFile.as_ptr().cast_mut(),
            Some(SaveConfirmOverwriteDefaultNoCallback),
        );
    } else {
        ShowSaveMessage(
            gText_AlreadySavedFile.as_ptr().cast_mut(),
            Some(SaveConfirmOverwriteCallback),
        );
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveConfirmOverwriteDefaultNoCallback() -> u8 {
    DisplayYesNoMenuWithDefault(1);
    sSaveDialogCallback = Some(SaveOverwriteInputCallback);
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveConfirmOverwriteCallback() -> u8 {
    DisplayYesNoMenuDefaultYes();
    sSaveDialogCallback = Some(SaveOverwriteInputCallback);
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveOverwriteInputCallback() -> u8 {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            sSaveDialogCallback = Some(SaveSavingMessageCallback);
            return SAVE_IN_PROGRESS;
        }
        MENU_B_PRESSED | 1 => {
            HideSaveInfoWindow();
            HideSaveMessageWindow();
            return SAVE_CANCELED;
        }
        _ => {}
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveSavingMessageCallback() -> u8 {
    ShowSaveMessage(
        gText_SavingDontTurnOff.as_ptr().cast_mut(),
        Some(SaveDoSaveCallback),
    );
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveDoSaveCallback() -> u8 {
    let mut saveStatus: u8 = 0;
    IncrementGameStat(GAME_STAT_SAVED_GAME);
    PausePyramidChallenge();
    if gDifferentSaveFile == TRUE {
        saveStatus = TrySavingData(SAVE_OVERWRITE_DIFFERENT_FILE);
        gDifferentSaveFile = FALSE;
    } else {
        saveStatus = TrySavingData(SAVE_NORMAL);
    }
    if saveStatus == SAVE_STATUS_OK {
        ShowSaveMessage(
            gText_PlayerSavedGame.as_ptr().cast_mut(),
            Some(SaveSuccessCallback),
        );
    } else {
        ShowSaveMessage(gText_SaveError.as_ptr().cast_mut(), Some(SaveErrorCallback));
    }
    SaveStartTimer();
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveSuccessCallback() -> u8 {
    if IsTextPrinterActive(0) == 0 {
        PlaySE(SE_SAVE);
        sSaveDialogCallback = Some(SaveReturnSuccessCallback);
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveReturnSuccessCallback() -> u8 {
    if IsSEPlaying() == 0 && SaveSuccesTimer() != 0 {
        HideSaveInfoWindow();
        return SAVE_SUCCESS;
    } else {
        return SAVE_IN_PROGRESS;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn SaveErrorCallback() -> u8 {
    if IsTextPrinterActive(0) == 0 {
        PlaySE(SE_BOO);
        sSaveDialogCallback = Some(SaveReturnErrorCallback);
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn SaveReturnErrorCallback() -> u8 {
    if SaveErrorTimer() == 0 {
        return SAVE_IN_PROGRESS;
    } else {
        HideSaveInfoWindow();
        return SAVE_ERROR;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn InitBattlePyramidRetire() {
    sSaveDialogCallback = Some(BattlePyramidConfirmRetireCallback);
    sSavingComplete = FALSE;
}
pub(crate) unsafe extern "C" fn BattlePyramidConfirmRetireCallback() -> u8 {
    ClearStdWindowAndFrame(GetStartMenuWindowId(), FALSE);
    RemoveStartMenuWindow();
    ShowSaveMessage(
        gText_BattlePyramidConfirmRetire.as_ptr().cast_mut(),
        Some(BattlePyramidRetireYesNoCallback),
    );
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireYesNoCallback() -> u8 {
    DisplayYesNoMenuWithDefault(1);
    sSaveDialogCallback = Some(BattlePyramidRetireInputCallback);
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn BattlePyramidRetireInputCallback() -> u8 {
    match Menu_ProcessInputNoWrapClearOnChoose() {
        0 => {
            return SAVE_CANCELED;
        }
        MENU_B_PRESSED | 1 => {
            HideSaveMessageWindow();
            return SAVE_SUCCESS;
        }
        _ => {}
    }
    return SAVE_IN_PROGRESS;
}
pub(crate) unsafe extern "C" fn VBlankCB_LinkBattleSave() {
    TransferPlttBuffer();
}
pub(crate) unsafe extern "C" fn InitSaveWindowAfterLinkBattle(state: *mut u8) -> u32 {
    match *state {
        0 => {
            SetGpuReg(0x0, 0x0000);
            SetVBlankCallback(None);
            ScanlineEffect_Stop();
            {
                {
                    let mut _dest: *mut u16 = PLTT as i32 as usize as *mut u16;
                    let mut _size: u32 = PLTT_SIZE;
                    {
                        {
                            let mut tmp: u16 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                }
            }
            {
                let mut _dest: *mut c_void = VRAM as usize as *mut c_void;
                let mut _size: u32 = VRAM_SIZE;
                loop {
                    {
                        {
                            let mut tmp: u16 = 0;
                            volatile_write(&raw mut tmp, 0);
                            {
                                {
                                    let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                    volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                    volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                    volatile_write(dmaRegs.at(2), 0x81000800);
                                    let _ = (dmaRegs.at(2)).read_volatile();
                                }
                            }
                        }
                    }
                    _dest = (_dest as *mut u8).at(4096) as *mut c_void;
                    _size -= 0x1000;
                    if _size <= 0x1000 {
                        {
                            {
                                let mut tmp: u16 = 0;
                                volatile_write(&raw mut tmp, 0);
                                {
                                    {
                                        let mut dmaRegs: *mut u32 = 67109076 as usize as *mut u32;
                                        volatile_write(dmaRegs, &raw mut tmp as usize as u32);
                                        volatile_write(dmaRegs.at(1), _dest as usize as u32);
                                        volatile_write(dmaRegs.at(2), 0x81000000 | _size / 2);
                                        let _ = (dmaRegs.at(2)).read_volatile();
                                    }
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        1 => {
            ResetSpriteData();
            ResetTasks();
            ResetPaletteFade();
            ScanlineEffect_Clear();
        }
        2 => {
            ResetBgsAndClearDma3BusyFlags(0);
            InitBgsFromTemplates(0, sBgTemplates_LinkBattleSave.as_ptr().cast_mut(), 1);
            InitWindows(sWindowTemplates_LinkBattleSave.as_ptr().cast_mut());
            LoadUserWindowBorderGfx_(0, 8, 224);
            Menu_LoadStdPalAt(240);
        }
        3 => {
            ShowBg(0);
            BlendPalettes(PALETTES_ALL, 16, 0);
            SetVBlankCallback(Some(VBlankCB_LinkBattleSave));
            EnableInterrupts(1);
        }
        4 => {
            return TRUE as u32;
        }
        _ => {}
    }
    *state += 1;
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CB2_SetUpSaveAfterLinkBattle() {
    if InitSaveWindowAfterLinkBattle(&raw mut gMain.state) != 0 {
        CreateTask(Some(Task_SaveAfterLinkBattle), 0x50);
        SetMainCallback2(Some(CB2_SaveAfterLinkBattle));
    }
}
pub(crate) unsafe extern "C" fn CB2_SaveAfterLinkBattle() {
    RunTasks();
    UpdatePaletteFade();
}
pub(crate) unsafe extern "C" fn Task_SaveAfterLinkBattle(taskId: u8) {
    let mut state: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if gPaletteFade.active() == 0 {
        match *state {
            0 => {
                FillWindowPixelBuffer(0, 17);
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    gText_SavingDontTurnOffPower.as_ptr().cast_mut(),
                    TEXT_SKIP_DRAW,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                DrawTextBorderOuter(0, 8, 14);
                PutWindowTilemap(0);
                CopyWindowToVram(0, COPYWIN_FULL);
                BeginNormalPaletteFade(PALETTES_ALL, 0, 16, 0, 0);
                if gWirelessCommType != 0 && InUnionRoom() != 0 {
                    if Link_AnyPartnersPlayingFRLG_JP() != 0 {
                        *state = 1;
                    } else {
                        *state = 5;
                    }
                } else {
                    gSoftResetDisabled = TRUE;
                    *state = 1;
                }
            }
            1 => {
                SetContinueGameWarpStatusToDynamicWarp();
                WriteSaveBlock2();
                *state = 2;
            }
            2 => {
                if WriteSaveBlock1Sector() != 0 {
                    ClearContinueGameWarpStatus2();
                    *state = 3;
                    gSoftResetDisabled = FALSE;
                }
            }
            3 => {
                BeginNormalPaletteFade(PALETTES_ALL, 0, 0, 16, 0);
                *state = 4;
            }
            4 => {
                FreeAllWindowBuffers();
                SetMainCallback2(gMain.savedCallback);
                DestroyTask(taskId);
            }
            5 => {
                CreateTask(Some(Task_LinkFullSave), 5);
                *state = 6;
            }
            6 => {
                if FuncIsActiveTask(Some(Task_LinkFullSave)) == 0 {
                    *state = 3;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe extern "C" fn ShowSaveInfoWindow() {
    let mut saveInfoWindow: WindowTemplate = zeroed();
    saveInfoWindow = *sSaveInfoWindowTemplate;
    let mut gender: u8 = 0;
    let mut color: u8 = 0;
    let mut xOffset: u32 = 0;
    let mut yOffset: u32 = 0;
    if FlagGet(FLAG_SYS_POKEDEX_GET) == 0 {
        saveInfoWindow.height -= 2;
    }
    sSaveInfoWindowId = AddWindow(&raw mut saveInfoWindow) as u8;
    DrawStdWindowFrame(sSaveInfoWindowId, FALSE);
    gender = (*gSaveBlock2Ptr).playerGender;
    color = TEXT_COLOR_RED;
    if gender == MALE {
        color = TEXT_COLOR_BLUE;
    }
    yOffset = 1;
    BufferSaveMenuText(
        SAVE_MENU_LOCATION,
        gStringVar4.as_mut_ptr(),
        TEXT_COLOR_GREEN,
    );
    AddTextPrinterParameterized(
        sSaveInfoWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        0,
        yOffset as u8,
        TEXT_SKIP_DRAW,
        None,
    );
    yOffset += 16;
    AddTextPrinterParameterized(
        sSaveInfoWindowId,
        FONT_NORMAL,
        gText_SavingPlayer.as_ptr().cast_mut(),
        0,
        yOffset as u8,
        TEXT_SKIP_DRAW,
        None,
    );
    BufferSaveMenuText(SAVE_MENU_NAME, gStringVar4.as_mut_ptr(), color);
    xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x70) as u32;
    PrintPlayerNameOnWindow(
        sSaveInfoWindowId,
        gStringVar4.as_mut_ptr(),
        xOffset as u16,
        yOffset as u16,
    );
    yOffset += 16;
    AddTextPrinterParameterized(
        sSaveInfoWindowId,
        FONT_NORMAL,
        gText_SavingBadges.as_ptr().cast_mut(),
        0,
        yOffset as u8,
        TEXT_SKIP_DRAW,
        None,
    );
    BufferSaveMenuText(SAVE_MENU_BADGES, gStringVar4.as_mut_ptr(), color);
    xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x70) as u32;
    AddTextPrinterParameterized(
        sSaveInfoWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        xOffset as u8,
        yOffset as u8,
        TEXT_SKIP_DRAW,
        None,
    );
    if FlagGet(FLAG_SYS_POKEDEX_GET) == TRUE {
        yOffset += 16;
        AddTextPrinterParameterized(
            sSaveInfoWindowId,
            FONT_NORMAL,
            gText_SavingPokedex.as_ptr().cast_mut(),
            0,
            yOffset as u8,
            TEXT_SKIP_DRAW,
            None,
        );
        BufferSaveMenuText(SAVE_MENU_CAUGHT, gStringVar4.as_mut_ptr(), color);
        xOffset =
            GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x70) as u32;
        AddTextPrinterParameterized(
            sSaveInfoWindowId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            xOffset as u8,
            yOffset as u8,
            TEXT_SKIP_DRAW,
            None,
        );
    }
    yOffset += 16;
    AddTextPrinterParameterized(
        sSaveInfoWindowId,
        FONT_NORMAL,
        gText_SavingTime.as_ptr().cast_mut(),
        0,
        yOffset as u8,
        TEXT_SKIP_DRAW,
        None,
    );
    BufferSaveMenuText(SAVE_MENU_PLAY_TIME, gStringVar4.as_mut_ptr(), color);
    xOffset = GetStringRightAlignXOffset(FONT_NORMAL as i32, gStringVar4.as_mut_ptr(), 0x70) as u32;
    AddTextPrinterParameterized(
        sSaveInfoWindowId,
        FONT_NORMAL,
        gStringVar4.as_mut_ptr(),
        xOffset as u8,
        yOffset as u8,
        TEXT_SKIP_DRAW,
        None,
    );
    CopyWindowToVram(sSaveInfoWindowId, COPYWIN_GFX);
}
pub(crate) unsafe extern "C" fn RemoveSaveInfoWindow() {
    ClearStdWindowAndFrame(sSaveInfoWindowId, FALSE);
    RemoveWindow(sSaveInfoWindowId);
}
pub(crate) unsafe extern "C" fn Task_WaitForBattleTowerLinkSave(taskId: u8) {
    if FuncIsActiveTask(Some(Task_LinkFullSave)) == 0 {
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SaveForBattleTowerLink() {
    let mut taskId: u8 = CreateTask(Some(Task_LinkFullSave), 5);
    gTasks[taskId].data[2] = TRUE as i16;
    gTasks[CreateTask(Some(Task_WaitForBattleTowerLinkSave), 6)].data[1] = taskId as i16;
}
pub(crate) unsafe extern "C" fn HideStartMenuWindow() {
    ClearStdWindowAndFrame(GetStartMenuWindowId(), TRUE);
    RemoveStartMenuWindow();
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideStartMenu() {
    PlaySE(SE_SELECT);
    HideStartMenuWindow();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AppendToList(mut list: *mut u8, pos: *mut u8, newEntry: u8) {
    *list.at(*pos) = newEntry;
    *pos += 1;
}
