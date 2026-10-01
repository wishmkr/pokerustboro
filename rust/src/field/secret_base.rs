//! Translated from `src/secret_base.c` by tools/rustport/c2rs.py.
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
    clippy::erasing_op,
    clippy::missing_transmute_annotations,
    clippy::useless_transmute,
    unused_assignments,
    unused_variables
)]

use crate::battle_main::gBattleTypeFlags;
use crate::battle_setup::gTrainerBattleOpponent_A;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::decoration::{DoSecretBaseDecorationMenu, ShowDecorationOnMap};
use crate::event_data::{FlagClear, FlagGet, FlagSet, VarGet, VarSet};
use crate::event_object_movement::{
    ObjectEventTurn, OverrideSecretBaseDecorationSpriteScript, RemoveObjectEventByLocalIdAndMap,
    TryMoveObjectEventToMapCoords, TryOverrideObjectEventTemplateCoords, TrySpawnObjectEvent,
};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_0x8006, gSpecialVar_0x8007, gSpecialVar_Result};
use crate::field_camera::{CurrentMapDrawMetatileAt, DrawWholeMapView};
use crate::field_effect::FieldEffectActiveListContains;
use crate::field_player_avatar::{
    GetXYCoordsOneStepInFrontOfPlayer, PlayerGetDestCoords, gObjectEvents, gPlayerAvatar,
};
use crate::field_screen_effect::{
    FadeInFromBlack, FieldCB_ContinueScriptHandleMusic, FieldCB_DefaultWarpExit,
};
use crate::field_specials::TryGainNewFanFromCounter;
use crate::field_weather::{FadeScreen, IsWeatherNotFadingIn};
use crate::fieldmap::{
    MapGridGetMetatileBehaviorAt, MapGridGetMetatileIdAt, MapGridSetMetatileIdAt, gMapHeader,
};
use crate::fldeff_misc::{PopSecretBaseBalloon, ShatterSecretBaseBreakableDoor};
use crate::link::{GetLinkPlayerCount, gLinkPlayers};
use crate::list_menu::{
    AddScrollIndicatorArrowPairParameterized, DestroyListMenuTask, ListMenu_ProcessInput,
    ListMenuGetScrollAndRow, ListMenuInit, RemoveScrollIndicatorArrowPair,
    gMultiuseListMenuTemplate,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::map_name_popup::HideMapNamePopUpWindow;
use crate::menu::{
    ClearDialogWindowAndFrame, ClearStdWindowAndFrame, DisplayItemMessageOnField,
    DisplayYesNoMenuDefaultYes, InitMenuInUpperLeftCornerNormal, Menu_ProcessInputNoWrap,
    PrintMenuTable, ScheduleBgCopyTilemapToVram, SetStandardWindowBorderStyle,
};
use crate::menu_helpers::{DoYesNoFuncWithChoice, SetCursorWithinListBounds};
use crate::metatile_behavior::{
    MetatileBehavior_HoldsLargeDecoration, MetatileBehavior_HoldsSmallDecoration,
    MetatileBehavior_IsSecretBaseBalloon, MetatileBehavior_IsSecretBaseBreakableDoor,
    MetatileBehavior_IsSecretBaseGlitterMat, MetatileBehavior_IsSecretBaseJumpMat,
    MetatileBehavior_IsSecretBaseSoundMat, MetatileBehavior_IsSecretBaseSpinMat,
};
use crate::overworld::{
    CB2_LoadMap, IncrementGameStat, SetDynamicWarp, SetWarpDestination,
    SetWarpDestinationToDynamicWarp, SetWarpDestinationToMapWarp, WarpIntoMap, gFieldCallback,
};
use crate::palette::gPaletteFade;
use crate::pokemon::{GetMonData2, gPlayerParty};
use crate::script::ScriptContext_SetupScript;
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable, UnlockPlayerFieldControls};
use crate::sound::PlaySE;
use crate::string_util::ConvertInternationalString;
use crate::string_util::{StringAppend, StringCopyN, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar4};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set, task_set_func};
use crate::tv::TryPutSecretBaseSecretsOnAir;
#[allow(unused_imports)]
use crate::types::*;
use crate::window::{ClearWindowTilemap, RemoveWindow};
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `GetMaxWidthInMenuTable` with this module's view of its types.
#[inline]
unsafe fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32 {
    unsafe { crate::international_string_util::GetMaxWidthInMenuTable(a0 as _, a1) }
}
// Data tables (translate with cdata.py): sSecretBaseEntranceMetatiles sSecretBaseEntrancePositions sRegistryMenuActions sDeleteRegistryYesNoFuncs sSecretBaseOwnerGfxIds sRegistryWindowTemplates sRegistryListMenuTemplate

/// `struct SecretBaseRegistryMenu`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecretBaseRegistryMenu {
    pub items: CArray<ListMenuItem, 11>,
    pub names: CArray<CArray<u8, 32>, 11>,
}

unsafe impl Sync for SecretBaseRegistryMenu {}

/// `struct SecretBaseRecordMixer`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SecretBaseRecordMixer {
    pub secretBases: *mut SecretBase,
    pub version: u32,
    pub language: u32,
}

unsafe impl Sync for SecretBaseRecordMixer {}

/// `struct SecretBaseEntranceMetatiles`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct SecretBaseEntranceMetatiles {
    pub closedMetatileId: u16,
    pub openMetatileId: u16,
}

unsafe impl Sync for SecretBaseEntranceMetatiles {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<SecretBaseRegistryMenu>() == 440);
    assert!(offset_of!(SecretBaseRegistryMenu, items) == 0);
    assert!(offset_of!(SecretBaseRegistryMenu, names) == 88);
    assert!(size_of::<SecretBaseRecordMixer>() == 12);
    assert!(offset_of!(SecretBaseRecordMixer, secretBases) == 0);
    assert!(offset_of!(SecretBaseRecordMixer, version) == 4);
    assert!(offset_of!(SecretBaseRecordMixer, language) == 8);
    assert!(size_of::<SecretBaseEntranceMetatiles>() == 4);
    assert!(offset_of!(SecretBaseEntranceMetatiles, closedMetatileId) == 0);
    assert!(offset_of!(SecretBaseEntranceMetatiles, openMetatileId) == 2);
};

const DELETED_BASE_A: i32 = 1;
const DELETED_BASE_B: i32 = 2;
const DELETED_BASE_C: i32 = 4;
const NEW: u8 = 2;
const REGISTERED: u8 = 1;
const TAG_SCROLL_ARROW: i32 = 5112;
const UNREGISTERED: u8 = 0;

static sDeleteRegistryYesNoFuncs: Table<YesNoFuncTable> =
    Table((&raw const crate::data::secret_base::sDeleteRegistryYesNoFuncs).cast());
static sRegistryListMenuTemplate: Table<ListMenuTemplate> =
    Table((&raw const crate::data::secret_base::sRegistryListMenuTemplate).cast());
static sRegistryMenuActions: Table<CArray<MenuAction, 2>> =
    Table((&raw const crate::data::secret_base::sRegistryMenuActions).cast());
static sRegistryWindowTemplates: Table<CArray<WindowTemplate, 2>> =
    Table((&raw const crate::data::secret_base::sRegistryWindowTemplates).cast());
static sSecretBaseEntranceMetatiles: Table<CArray<SecretBaseEntranceMetatiles, 7>> =
    Table((&raw const crate::data::secret_base::sSecretBaseEntranceMetatiles).cast());
static sSecretBaseEntrancePositions: Table<CArray<u8, 96>> =
    Table((&raw const crate::data::secret_base::sSecretBaseEntrancePositions).cast());
static sSecretBaseOwnerGfxIds: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::secret_base::sSecretBaseOwnerGfxIds).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sCurSecretBaseId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sInFriendSecretBase: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRegistryMenu: *mut SecretBaseRegistryMenu = null_mut();

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `CpuFastSet` with this module's view of its types.
#[inline]
unsafe fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuFastSet(a0 as _, a1 as _, a2);
    }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

unsafe fn ClearSecretBase(secretBase: *mut SecretBase) {
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            secretBase as *mut c_void,
            0x1000028,
        );
    }
    for i in 0..(PLAYER_NAME_LENGTH as u16) {
        (*secretBase).trainerName[i] = EOS;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ClearSecretBases() {
    for i in 0..(SECRET_BASES_COUNT as u16) {
        ClearSecretBase(&raw mut (*gSaveBlock1Ptr).secretBases[i]);
    }
}
unsafe fn SetCurSecretBaseId() {
    sCurSecretBaseId.set(gSpecialVar_0x8004 as u8);
}
pub unsafe fn TrySetCurSecretBaseIndex() {
    gSpecialVar_Result = FALSE as u16;
    for i in 0..(SECRET_BASES_COUNT as u16) {
        if sCurSecretBaseId.get() == (*gSaveBlock1Ptr).secretBases[i].secretBaseId {
            gSpecialVar_Result = TRUE as u16;
            VarSet(VAR_CURRENT_SECRET_BASE, i);
            break;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckPlayerHasSecretBase() {
    if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != 0 {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
unsafe fn GetSecretBaseTypeInFrontOfPlayer_() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let behavior: i16 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as i16 & 0xFFF;
    if behavior == MB_SECRET_BASE_SPOT_RED_CAVE as i16
        || behavior == MB_SECRET_BASE_SPOT_RED_CAVE_OPEN as i16
    {
        return SECRET_BASE_RED_CAVE;
    }
    if behavior == MB_SECRET_BASE_SPOT_BROWN_CAVE as i16
        || behavior == MB_SECRET_BASE_SPOT_BROWN_CAVE_OPEN as i16
    {
        return SECRET_BASE_BROWN_CAVE;
    }
    if behavior == MB_SECRET_BASE_SPOT_BLUE_CAVE as i16
        || behavior == MB_SECRET_BASE_SPOT_BLUE_CAVE_OPEN as i16
    {
        return SECRET_BASE_BLUE_CAVE;
    }
    if behavior == MB_SECRET_BASE_SPOT_YELLOW_CAVE as i16
        || behavior == MB_SECRET_BASE_SPOT_YELLOW_CAVE_OPEN as i16
    {
        return SECRET_BASE_YELLOW_CAVE;
    }
    if behavior == MB_SECRET_BASE_SPOT_TREE_LEFT
        || behavior == MB_SECRET_BASE_SPOT_TREE_LEFT_OPEN as i16
        || behavior == MB_SECRET_BASE_SPOT_TREE_RIGHT
        || behavior == MB_SECRET_BASE_SPOT_TREE_RIGHT_OPEN as i16
    {
        return SECRET_BASE_TREE;
    }
    if behavior == MB_SECRET_BASE_SPOT_SHRUB as i16
        || behavior == MB_SECRET_BASE_SPOT_SHRUB_OPEN as i16
    {
        return SECRET_BASE_SHRUB;
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn GetSecretBaseTypeInFrontOfPlayer() {
    gSpecialVar_0x8007 = GetSecretBaseTypeInFrontOfPlayer_() as u16;
}
unsafe fn FindMetatileIdMapCoords(x: *mut i16, y: *mut i16, metatileId: u16) {
    let mut i: i16 = 0;
    let mapLayout: *mut MapLayout = gMapHeader.mapLayout;
    let mut j: i16 = 0;
    while (j as i32) < (*mapLayout).height {
        i = 0;
        while (i as i32) < (*mapLayout).width {
            if *(*mapLayout)
                .map
                .at(j as i32 * (*mapLayout).width + i as i32) as i32
                & MAPGRID_METATILE_ID_MASK
                == metatileId as i32
            {
                *x = i;
                *y = j;
                return;
            }
            i += 1;
        }
        j += 1;
    }
}
pub unsafe fn ToggleSecretBaseEntranceMetatile() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let metatileId: i16 = MapGridGetMetatileIdAt(x as i32, y as i32) as i16;
    for i in 0..7u16 {
        if sSecretBaseEntranceMetatiles[i].closedMetatileId as i32 == metatileId as i32 {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                sSecretBaseEntranceMetatiles[i].openMetatileId | MAPGRID_IMPASSABLE,
            );
            CurrentMapDrawMetatileAt(x as i32, y as i32);
            return;
        }
    }
    for i in 0..7u16 {
        if sSecretBaseEntranceMetatiles[i].openMetatileId as i32 == metatileId as i32 {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                sSecretBaseEntranceMetatiles[i].closedMetatileId | MAPGRID_IMPASSABLE,
            );
            CurrentMapDrawMetatileAt(x as i32, y as i32);
            return;
        }
    }
}
unsafe fn GetNameLength(secretBaseOwnerName: *mut u8) -> u8 {
    for i in 0..(PLAYER_NAME_LENGTH as u8) {
        if *secretBaseOwnerName.at(i) == EOS {
            return i;
        }
    }
    PLAYER_NAME_LENGTH as u8
}
#[unsafe(no_mangle)]
pub unsafe fn SetPlayerSecretBase() {
    (*gSaveBlock1Ptr).secretBases[0].secretBaseId = sCurSecretBaseId.get();
    for i in 0..(TRAINER_ID_LENGTH as u16) {
        (*gSaveBlock1Ptr).secretBases[0].trainerId[i] = (*gSaveBlock2Ptr).playerTrainerId[i];
    }
    VarSet(VAR_CURRENT_SECRET_BASE, 0);
    StringCopyN(
        (*gSaveBlock1Ptr).secretBases[0].trainerName.as_mut_ptr(),
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        GetNameLength((*gSaveBlock2Ptr).playerName.as_mut_ptr()),
    );
    (*gSaveBlock1Ptr).secretBases[0].set_gender((*gSaveBlock2Ptr).playerGender);
    (*gSaveBlock1Ptr).secretBases[0].language = GAME_LANGUAGE;
    VarSet(VAR_SECRET_BASE_MAP, gMapHeader.regionMapSectionId as u16);
}
pub unsafe fn SetOccupiedSecretBaseEntranceMetatiles(events: *mut MapEvents) {
    let mut bgId: u16 = 0;
    while bgId < (*events).bgEventCount as u16 {
        if (*(*events).bgEvents.at(bgId)).kind == BG_EVENT_SECRET_BASE {
            for j in 0..(SECRET_BASES_COUNT as u16) {
                if (*gSaveBlock1Ptr).secretBases[j].secretBaseId as u32
                    == (*(*events).bgEvents.at(bgId)).bgUnion.secretBaseId
                {
                    let x: i16 = (*(*events).bgEvents.at(bgId)).x as i16 + MAP_OFFSET as i16;
                    let y: i16 = (*(*events).bgEvents.at(bgId)).y as i16 + MAP_OFFSET as i16;
                    let tile_id: i16 = MapGridGetMetatileIdAt(x as i32, y as i32) as i16;
                    for i in 0..7u16 {
                        if sSecretBaseEntranceMetatiles[i].closedMetatileId as i32 == tile_id as i32
                        {
                            MapGridSetMetatileIdAt(
                                x as i32,
                                y as i32,
                                sSecretBaseEntranceMetatiles[i].openMetatileId | MAPGRID_IMPASSABLE,
                            );
                            break;
                        }
                    }
                    break;
                }
            }
        }
        bgId += 1;
    }
}
unsafe fn SetSecretBaseWarpDestination() {
    let secretBaseGroup: i8 = (sCurSecretBaseId.get() as i32 / 10) as i8 * 4;
    SetWarpDestinationToMapWarp(
        25,
        sSecretBaseEntrancePositions[secretBaseGroup as i32] as i8,
        sSecretBaseEntrancePositions[secretBaseGroup as i32 + 1] as i8,
    );
}
pub(crate) unsafe fn Task_EnterSecretBase(taskId: u8) {
    let mut secretBaseIdx: u16 = 0;
    match task_get(taskId, 0) {
        0 => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, 0, 1);
            }
        }
        1 => {
            secretBaseIdx = VarGet(VAR_CURRENT_SECRET_BASE);
            (*gSaveBlock1Ptr).secretBases[secretBaseIdx].numTimesEntered = (*gSaveBlock1Ptr)
                .secretBases[secretBaseIdx]
                .numTimesEntered
                .saturating_add(1);
            SetSecretBaseWarpDestination();
            WarpIntoMap();
            gFieldCallback = Some(FieldCB_ContinueScriptHandleMusic);
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn EnterSecretBase() {
    CreateTask(Some(Task_EnterSecretBase), 0);
    FadeScreen(FADE_TO_BLACK, 0);
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
}
pub unsafe fn SecretBaseMapPopupEnabled() -> u8 {
    if gMapHeader.mapType == MAP_TYPE_SECRET_BASE && VarGet(VAR_INIT_SECRET_BASE) == 0 {
        return FALSE;
    }
    TRUE
}
pub(crate) unsafe fn EnterNewlyCreatedSecretBase_WaitFadeIn(taskId: u8) {
    ObjectEventTurn(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        DIR_NORTH,
    );
    if IsWeatherNotFadingIn() == TRUE {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn EnterNewlyCreatedSecretBase_StartFadeIn() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    LockPlayerFieldControls();
    HideMapNamePopUpWindow();
    FindMetatileIdMapCoords(&raw mut x, &raw mut y, METATILE_SecretBase_PC);
    x += MAP_OFFSET as i16;
    y += MAP_OFFSET as i16;
    MapGridSetMetatileIdAt(x as i32, y as i32, 3616);
    CurrentMapDrawMetatileAt(x as i32, y as i32);
    FadeInFromBlack();
    CreateTask(Some(EnterNewlyCreatedSecretBase_WaitFadeIn), 0);
}
pub(crate) unsafe fn Task_EnterNewlyCreatedSecretBase(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let secretBaseGroup: i8 = (sCurSecretBaseId.get() as i32 / 10) as i8 * 4;
        SetWarpDestination(
            (*gSaveBlock1Ptr).location.mapGroup,
            (*gSaveBlock1Ptr).location.mapNum,
            WARP_ID_NONE,
            sSecretBaseEntrancePositions[secretBaseGroup as i32 + 2] as i8,
            sSecretBaseEntrancePositions[secretBaseGroup as i32 + 3] as i8,
        );
        WarpIntoMap();
        gFieldCallback = Some(EnterNewlyCreatedSecretBase_StartFadeIn);
        SetMainCallback2(Some(CB2_LoadMap));
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn EnterNewlyCreatedSecretBase() {
    CreateTask(Some(Task_EnterNewlyCreatedSecretBase), 0);
    FadeScreen(FADE_TO_BLACK, 0);
}
#[unsafe(no_mangle)]
pub unsafe fn CurMapIsSecretBase() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup == 25 && (*gSaveBlock1Ptr).location.mapNum as u8 <= 23 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn InitSecretBaseAppearance(hidePC: u8) {
    let mut secretBaseIdx: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut decorations: *mut u8 = null_mut();
    let mut decorPos: *mut u8 = null_mut();
    if CurMapIsSecretBase() != 0 {
        secretBaseIdx = VarGet(VAR_CURRENT_SECRET_BASE);
        decorations = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .decorations
            .as_mut_ptr();
        decorPos = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .decorationPositions
            .as_mut_ptr();
        x = 0;
        while x < DECOR_MAX_SECRET_BASE as u16 {
            if *decorations.at(x) > 0
                && *decorations.at(x) <= NUM_DECORATIONS
                && (*(&raw const crate::data::decoration::gDecorations)
                    .cast::<CArray<Decoration, 0>>())[*decorations.at(x)]
                .permission
                    != DECORPERM_SPRITE
            {
                ShowDecorationOnMap(
                    (*decorPos.at(x) >> 4) as u16 + MAP_OFFSET as u16,
                    (*decorPos.at(x) as u16 & 0xF) + MAP_OFFSET as u16,
                    *decorations.at(x) as u16,
                );
            }
            x += 1;
        }
        if secretBaseIdx != 0 {
            FindMetatileIdMapCoords(
                &raw mut x as *mut i16,
                &raw mut y as *mut i16,
                METATILE_SecretBase_PC,
            );
            MapGridSetMetatileIdAt(x as i32 + MAP_OFFSET, y as i32 + MAP_OFFSET, 3617);
        } else if hidePC == 1 && VarGet(VAR_SECRET_BASE_INITIALIZED) == 1 {
            FindMetatileIdMapCoords(
                &raw mut x as *mut i16,
                &raw mut y as *mut i16,
                METATILE_SecretBase_PC,
            );
            MapGridSetMetatileIdAt(x as i32 + MAP_OFFSET, y as i32 + MAP_OFFSET, 3594);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn InitSecretBaseDecorationSprites() {
    let mut decorations: *mut u8 = null_mut();
    let mut decorationPositions: *mut u8 = null_mut();
    let mut metatileBehavior: u8 = 0;
    let mut category: u8 = 0;
    let mut permission: u8 = 0;
    let mut numDecorations: u8 = 0;
    let mut objectEventId: u8 = 0;
    if CurMapIsSecretBase() == 0 {
        decorations = (*gSaveBlock1Ptr).playerRoomDecorations.as_mut_ptr();
        decorationPositions = (*gSaveBlock1Ptr).playerRoomDecorationPositions.as_mut_ptr();
        numDecorations = DECOR_MAX_PLAYERS_HOUSE;
    } else {
        let secretBaseIdx: u16 = VarGet(VAR_CURRENT_SECRET_BASE);
        decorations = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .decorations
            .as_mut_ptr();
        decorationPositions = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .decorationPositions
            .as_mut_ptr();
        numDecorations = DECOR_MAX_SECRET_BASE;
    }
    for i in 0..numDecorations {
        'l1: {
            if *decorations.at(i) == DECOR_NONE {
                break 'l1;
            }
            permission = (*(&raw const crate::data::decoration::gDecorations)
                .cast::<CArray<Decoration, 0>>())[*decorations.at(i)]
            .permission;
            category = (*(&raw const crate::data::decoration::gDecorations)
                .cast::<CArray<Decoration, 0>>())[*decorations.at(i)]
            .category;
            if permission == DECORPERM_SPRITE {
                objectEventId = 0;
                while objectEventId < (*gMapHeader.events).objectEventCount {
                    if (*(*gMapHeader.events).objectEvents.at(objectEventId)).flagId as i32
                        == FLAG_DECORATION_1 as i32 + gSpecialVar_0x8004 as i32
                    {
                        break;
                    }
                    objectEventId += 1;
                }
                if objectEventId == (*gMapHeader.events).objectEventCount {
                    break 'l1;
                }
                gSpecialVar_0x8006 = (*decorationPositions.at(i) >> 4) as u16;
                gSpecialVar_0x8007 = *decorationPositions.at(i) as u16 & 0xF;
                metatileBehavior = MapGridGetMetatileBehaviorAt(
                    gSpecialVar_0x8006 as i32 + MAP_OFFSET,
                    gSpecialVar_0x8007 as i32 + MAP_OFFSET,
                ) as u8;
                if MetatileBehavior_HoldsSmallDecoration(metatileBehavior) == TRUE
                    || MetatileBehavior_HoldsLargeDecoration(metatileBehavior) == TRUE
                {
                    gSpecialVar_Result = VAR_OBJ_GFX_ID_0
                        + ((*(*gMapHeader.events).objectEvents.at(objectEventId)).graphicsId
                            as u16
                            - OBJ_EVENT_GFX_VAR_0);
                    VarSet(
                        *(&raw const crate::ffi::gSpecialVar_Result)
                            .cast::<u16>()
                            .cast_mut(),
                        *(*(&raw const crate::data::decoration::gDecorations)
                            .cast::<CArray<Decoration, 0>>())[*decorations.at(i)]
                        .tiles,
                    );
                    gSpecialVar_Result =
                        (*(*gMapHeader.events).objectEvents.at(objectEventId)).localId as u16;
                    FlagClear(FLAG_DECORATION_1 + gSpecialVar_0x8004);
                    TrySpawnObjectEvent(
                        gSpecialVar_Result as u8,
                        (*gSaveBlock1Ptr).location.mapNum as u8,
                        (*gSaveBlock1Ptr).location.mapGroup as u8,
                    );
                    TryMoveObjectEventToMapCoords(
                        gSpecialVar_Result as u8,
                        (*gSaveBlock1Ptr).location.mapNum as u8,
                        (*gSaveBlock1Ptr).location.mapGroup as u8,
                        gSpecialVar_0x8006 as i16,
                        gSpecialVar_0x8007 as i16,
                    );
                    TryOverrideObjectEventTemplateCoords(
                        gSpecialVar_Result as u8,
                        (*gSaveBlock1Ptr).location.mapNum as u8,
                        (*gSaveBlock1Ptr).location.mapGroup as u8,
                    );
                    if CurMapIsSecretBase() == TRUE && VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                        if category == DECORCAT_DOLL {
                            OverrideSecretBaseDecorationSpriteScript(
                                gSpecialVar_Result as u8,
                                (*gSaveBlock1Ptr).location.mapNum as u8,
                                (*gSaveBlock1Ptr).location.mapGroup as u8,
                                DECORCAT_DOLL,
                            );
                        } else if category == DECORCAT_CUSHION {
                            OverrideSecretBaseDecorationSpriteScript(
                                gSpecialVar_Result as u8,
                                (*gSaveBlock1Ptr).location.mapNum as u8,
                                (*gSaveBlock1Ptr).location.mapGroup as u8,
                                DECORCAT_CUSHION,
                            );
                        }
                    }
                    gSpecialVar_0x8004 += 1;
                }
            }
        }
    }
}
pub unsafe fn HideSecretBaseDecorationSprites() {
    let mut flag: u16 = 0;
    let mut objectEventId: u8 = 0;
    while objectEventId < (*gMapHeader.events).objectEventCount {
        flag = (*(*gMapHeader.events).objectEvents.at(objectEventId)).flagId;
        if (FLAG_DECORATION_1..=FLAG_DECORATION_14).contains(&flag) {
            RemoveObjectEventByLocalIdAndMap(
                (*(*gMapHeader.events).objectEvents.at(objectEventId)).localId,
                (*gSaveBlock1Ptr).location.mapNum as u8,
                (*gSaveBlock1Ptr).location.mapGroup as u8,
            );
            FlagSet(flag);
        }
        objectEventId += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetSecretBaseOwnerGfxId() {
    VarSet(
        VAR_OBJ_GFX_ID_F,
        sSecretBaseOwnerGfxIds[GetSecretBaseOwnerType(VarGet(VAR_CURRENT_SECRET_BASE) as u8)]
            as u16,
    );
}
pub unsafe fn SetCurSecretBaseIdFromPosition(position: *mut MapPosition, events: *mut MapEvents) {
    for i in 0..((*events).bgEventCount as i16) {
        if (*(*events).bgEvents.at(i)).kind == BG_EVENT_SECRET_BASE
            && (*position).x as i32 == (*(*events).bgEvents.at(i)).x as i32 + MAP_OFFSET
            && (*position).y as i32 == (*(*events).bgEvents.at(i)).y as i32 + MAP_OFFSET
        {
            sCurSecretBaseId.set((*(*events).bgEvents.at(i)).bgUnion.secretBaseId as u8);
            break;
        }
    }
}
pub unsafe fn WarpIntoSecretBase(position: *mut MapPosition, events: *mut MapEvents) {
    SetCurSecretBaseIdFromPosition(position, events);
    TrySetCurSecretBaseIndex();
    ScriptContext_SetupScript(
        (*crate::asmdata::SecretBase_EventScript_Enter.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
pub unsafe fn TrySetCurSecretBase() -> u8 {
    SetCurSecretBaseId();
    TrySetCurSecretBaseIndex();
    if gSpecialVar_Result == TRUE as u16 {
        return FALSE;
    }
    TRUE
}
pub(crate) unsafe fn Task_WarpOutOfSecretBase(taskId: u8) {
    match task_get(taskId, 0) {
        0 => {
            LockPlayerFieldControls();
            task_set(taskId, 0, 1);
        }
        1 => {
            if gPaletteFade.active() == 0 {
                task_set(taskId, 0, 2);
            }
        }
        2 => {
            SetWarpDestinationToDynamicWarp(WARP_ID_SECRET_BASE);
            WarpIntoMap();
            gFieldCallback = Some(FieldCB_DefaultWarpExit);
            SetMainCallback2(Some(CB2_LoadMap));
            UnlockPlayerFieldControls();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
unsafe fn WarpOutOfSecretBase() {
    CreateTask(Some(Task_WarpOutOfSecretBase), 0);
    FadeScreen(FADE_TO_BLACK, 0);
}
#[unsafe(no_mangle)]
pub unsafe fn IsCurSecretBaseOwnedByAnotherPlayer() {
    if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != sCurSecretBaseId.get() {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
unsafe fn GetSecretBaseName(dest: *mut u8, secretBaseIdx: u8) -> *mut u8 {
    *StringCopyN(
        dest,
        (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .trainerName
            .as_mut_ptr(),
        GetNameLength(
            (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
                .trainerName
                .as_mut_ptr(),
        ),
    ) = EOS;
    ConvertInternationalString(dest, (*gSaveBlock1Ptr).secretBases[secretBaseIdx].language);
    StringAppend(
        dest,
        (*(&raw const crate::data::strings::gText_ApostropheSBase).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    )
}
pub unsafe fn GetSecretBaseMapName(dest: *mut u8) -> *mut u8 {
    GetSecretBaseName(dest, VarGet(VAR_CURRENT_SECRET_BASE) as u8)
}
#[unsafe(no_mangle)]
pub unsafe fn CopyCurSecretBaseOwnerName_StrVar1() {
    let secretBaseIdx: u8 = VarGet(VAR_CURRENT_SECRET_BASE) as u8;
    let name: *mut u8 = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
        .trainerName
        .as_mut_ptr();
    *StringCopyN(gStringVar1.as_mut_ptr(), name, GetNameLength(name)) = EOS;
    ConvertInternationalString(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock1Ptr).secretBases[secretBaseIdx].language,
    );
}
unsafe fn IsSecretBaseRegistered(secretBaseIdx: u8) -> u8 {
    if (*gSaveBlock1Ptr).secretBases[secretBaseIdx].registryStatus() != 0 {
        return TRUE;
    }
    FALSE
}
unsafe fn GetAverageEVs(pokemon: *mut Pokemon) -> u8 {
    let mut evTotal: u16 = GetMonData2(pokemon, MON_DATA_HP_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_ATK_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_DEF_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_SPEED_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_SPATK_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_SPDEF_EV) as u16;
    (evTotal as i32 / 6) as u8
}
pub unsafe fn SetPlayerSecretBaseParty() {
    let mut moveIndex: u16 = 0;
    let mut party: *mut SecretBaseParty = null_mut();
    let mut partyId: u16 = 0;
    party = &raw mut (*gSaveBlock1Ptr).secretBases[0].party;
    if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != 0 {
        for i in 0..(PARTY_SIZE as u16) {
            moveIndex = 0;
            while moveIndex < MAX_MON_MOVES as u16 {
                (*party).moves[i as i32 * MAX_MON_MOVES + moveIndex as i32] = MOVE_NONE;
                moveIndex += 1;
            }
            (*party).species[i] = SPECIES_NONE;
            (*party).heldItems[i] = ITEM_NONE;
            (*party).levels[i] = 0;
            (*party).personality[i] = 0;
            (*party).EVs[i] = 0;
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) != SPECIES_NONE as u32
                && GetMonData2(&raw mut gPlayerParty[i], MON_DATA_IS_EGG) == 0
            {
                for moveIndex in 0..(MAX_MON_MOVES as u16) {
                    (*party).moves[partyId as i32 * MAX_MON_MOVES + moveIndex as i32] =
                        GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE1 + moveIndex as i32)
                            as u16;
                }
                (*party).species[partyId] =
                    GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) as u16;
                (*party).heldItems[partyId] =
                    GetMonData2(&raw mut gPlayerParty[i], MON_DATA_HELD_ITEM) as u16;
                (*party).levels[partyId] =
                    GetMonData2(&raw mut gPlayerParty[i], MON_DATA_LEVEL) as u8;
                (*party).personality[partyId] =
                    GetMonData2(&raw mut gPlayerParty[i], MON_DATA_PERSONALITY);
                (*party).EVs[partyId] = GetAverageEVs(&raw mut gPlayerParty[i]);
                partyId += 1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ClearAndLeaveSecretBase() {
    let temp: u16 = (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived;
    ClearSecretBase(&raw mut (*gSaveBlock1Ptr).secretBases[0]);
    (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived = temp;
    WarpOutOfSecretBase();
}
#[unsafe(no_mangle)]
pub unsafe fn MoveOutOfSecretBase() {
    IncrementGameStat(GAME_STAT_MOVED_SECRET_BASE);
    ClearAndLeaveSecretBase();
}
unsafe fn ClosePlayerSecretBaseEntrance() {
    let mut metatileId: i16 = 0;
    let events: *mut MapEvents = gMapHeader.events;
    let mut i: u16 = 0;
    while i < (*events).bgEventCount as u16 {
        if (*(*events).bgEvents.at(i)).kind == BG_EVENT_SECRET_BASE
            && (*gSaveBlock1Ptr).secretBases[0].secretBaseId as u32
                == (*(*events).bgEvents.at(i)).bgUnion.secretBaseId
        {
            metatileId = MapGridGetMetatileIdAt(
                (*(*events).bgEvents.at(i)).x as i32 + MAP_OFFSET,
                (*(*events).bgEvents.at(i)).y as i32 + MAP_OFFSET,
            ) as i16;
            for j in 0..7u16 {
                if sSecretBaseEntranceMetatiles[j].openMetatileId as i32 == metatileId as i32 {
                    MapGridSetMetatileIdAt(
                        (*(*events).bgEvents.at(i)).x as i32 + MAP_OFFSET,
                        (*(*events).bgEvents.at(i)).y as i32 + MAP_OFFSET,
                        sSecretBaseEntranceMetatiles[j].closedMetatileId | MAPGRID_IMPASSABLE,
                    );
                    break;
                }
            }
            DrawWholeMapView();
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn MoveOutOfSecretBaseFromOutside() {
    ClosePlayerSecretBaseEntrance();
    IncrementGameStat(GAME_STAT_MOVED_SECRET_BASE);
    let temp: u16 = (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived;
    ClearSecretBase(&raw mut (*gSaveBlock1Ptr).secretBases[0]);
    (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived = temp;
}
unsafe fn GetNumRegisteredSecretBases() -> u8 {
    let mut count: u8 = 0;
    for i in 1..(SECRET_BASES_COUNT as i16) {
        if IsSecretBaseRegistered(i as u8) == TRUE {
            count += 1;
        }
    }
    count
}
#[unsafe(no_mangle)]
pub unsafe fn GetCurSecretBaseRegistrationValidity() {
    if IsSecretBaseRegistered(VarGet(VAR_CURRENT_SECRET_BASE) as u8) == TRUE {
        gSpecialVar_Result = 1;
    } else if GetNumRegisteredSecretBases() >= 10 {
        gSpecialVar_Result = 2;
    } else {
        gSpecialVar_Result = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ToggleCurSecretBaseRegistry() {
    (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)].set_registryStatus(
        (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)].registryStatus() ^ 1,
    );
    FlagSet(FLAG_SECRET_BASE_REGISTRY_ENABLED);
}
#[unsafe(no_mangle)]
pub unsafe fn ShowSecretBaseDecorationMenu() {
    CreateTask(Some(DoSecretBaseDecorationMenu), 0);
}
#[unsafe(no_mangle)]
pub unsafe fn ShowSecretBaseRegistryMenu() {
    CreateTask(Some(Task_ShowSecretBaseRegistryMenu), 0);
}
pub(crate) unsafe fn Task_ShowSecretBaseRegistryMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    LockPlayerFieldControls();
    *data = GetNumRegisteredSecretBases() as i16;
    if *data != 0 {
        *data.at(1) = 0;
        *data.at(2) = 0;
        ClearDialogWindowAndFrame(0, 0);
        sRegistryMenu = AllocZeroed(440) as *mut SecretBaseRegistryMenu;
        *data.at(6) = AddWindow((&raw const sRegistryWindowTemplates[0]).cast_mut()) as i16;
        BuildRegistryMenuItems(taskId);
        FinalizeRegistryMenu(taskId);
        task_set_func(taskId, Some(HandleRegistryMenuInput));
    } else {
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_NoRegistry).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(GoToSecretBasePCRegisterMenu),
        );
    }
}
unsafe fn BuildRegistryMenuItems(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut count: u8 = 0;
    for i in 1..SECRET_BASES_COUNT {
        if IsSecretBaseRegistered(i) != 0 {
            GetSecretBaseName((*sRegistryMenu).names[count].as_mut_ptr(), i);
            (*sRegistryMenu).items[count].name = (*sRegistryMenu).names[count].as_mut_ptr();
            (*sRegistryMenu).items[count].id = i as i32;
            count += 1;
        }
    }
    (*sRegistryMenu).items[count].name = (*(&raw const crate::data::strings::gText_Cancel)
        .cast::<CArray<u8, 0>>())
    .as_ptr()
    .cast_mut();
    (*sRegistryMenu).items[count].id = LIST_CANCEL;
    *data = count as i16 + 1;
    if *data < 8 {
        *data.at(3) = *data;
    } else {
        *data.at(3) = 8;
    }
    gMultiuseListMenuTemplate = *sRegistryListMenuTemplate;
    gMultiuseListMenuTemplate.windowId = *data.at(6) as u8;
    gMultiuseListMenuTemplate.totalItems = *data as u16;
    gMultiuseListMenuTemplate.items = (*sRegistryMenu).items.as_mut_ptr();
    gMultiuseListMenuTemplate.maxShowed = *data.at(3) as u16;
}
pub(crate) unsafe fn RegistryMenu_OnCursorMove(unused: i32, flag: u8, menu: *mut ListMenu) {
    if flag != TRUE {
        PlaySE(SE_SELECT);
    }
}
unsafe fn FinalizeRegistryMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    SetStandardWindowBorderStyle(*data.at(6) as u8, FALSE);
    *data.at(5) = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        *data.at(2) as u16,
        *data.at(1) as u16,
    ) as i16;
    AddRegistryMenuScrollArrows(taskId);
    ScheduleBgCopyTilemapToVram(0);
}
unsafe fn AddRegistryMenuScrollArrows(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(8) = AddScrollIndicatorArrowPairParameterized(
        SCROLL_ARROW_UP,
        188,
        12,
        148,
        *data as i32 - *data.at(3) as i32,
        TAG_SCROLL_ARROW,
        TAG_SCROLL_ARROW,
        data.at(2) as *mut u16,
    ) as i16;
}
pub(crate) unsafe fn HandleRegistryMenuInput(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let input: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
    ListMenuGetScrollAndRow(
        *data.at(5) as u8,
        data.at(2) as *mut u16,
        data.at(1) as *mut u16,
    );
    match input {
        LIST_NOTHING_CHOSEN => {}
        LIST_CANCEL => {
            PlaySE(SE_SELECT);
            DestroyListMenuTask(*data.at(5) as u8, null_mut(), null_mut());
            RemoveScrollIndicatorArrowPair(*data.at(8) as u8);
            ClearStdWindowAndFrame(*data.at(6) as u8, FALSE);
            ClearWindowTilemap(*data.at(6) as u8);
            RemoveWindow(*data.at(6) as u8);
            ScheduleBgCopyTilemapToVram(0);
            Free(sRegistryMenu as *mut c_void);
            GoToSecretBasePCRegisterMenu(taskId);
        }
        _ => {
            PlaySE(SE_SELECT);
            *data.at(4) = input as i16;
            ShowRegistryMenuActions(taskId);
        }
    }
}
unsafe fn ShowRegistryMenuActions(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    RemoveScrollIndicatorArrowPair(*data.at(8) as u8);
    let mut template: WindowTemplate = sRegistryWindowTemplates[1];
    template.width = GetMaxWidthInMenuTable(sRegistryMenuActions.as_ptr().cast_mut(), 2) as u8;
    *data.at(7) = AddWindow(&raw mut template) as i16;
    SetStandardWindowBorderStyle(*data.at(7) as u8, FALSE);
    PrintMenuTable(
        *data.at(7) as u8,
        2,
        sRegistryMenuActions.as_ptr().cast_mut(),
    );
    InitMenuInUpperLeftCornerNormal(*data.at(7) as u8, 2, 0);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(HandleRegistryMenuActionsInput));
}
pub(crate) unsafe fn HandleRegistryMenuActionsInput(taskId: u8) {
    let input: i8 = Menu_ProcessInputNoWrap();
    match input {
        MENU_B_PRESSED => {
            PlaySE(SE_SELECT);
            ReturnToMainRegistryMenu(taskId);
        }
        MENU_NOTHING_CHOSEN => {}
        _ => {
            PlaySE(SE_SELECT);
            sRegistryMenuActions[input].func.void_u8.unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe fn ShowRegistryMenuDeleteConfirmation(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ClearStdWindowAndFrame(*data.at(6) as u8, FALSE);
    ClearStdWindowAndFrame(*data.at(7) as u8, FALSE);
    ClearWindowTilemap(*data.at(6) as u8);
    ClearWindowTilemap(*data.at(7) as u8);
    RemoveWindow(*data.at(7) as u8);
    ScheduleBgCopyTilemapToVram(0);
    GetSecretBaseName(gStringVar1.as_mut_ptr(), *data.at(4) as u8);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_OkayToDeleteFromRegistry)
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(ShowRegistryMenuDeleteYesNo),
    );
}
pub(crate) unsafe fn ShowRegistryMenuDeleteYesNo(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(taskId, (&raw const *sDeleteRegistryYesNoFuncs).cast_mut());
}
pub unsafe fn DeleteRegistry_Yes_Callback(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ClearDialogWindowAndFrame(0, 0);
    DestroyListMenuTask(
        *data.at(5) as u8,
        data.at(2) as *mut u16,
        data.at(1) as *mut u16,
    );
    (*gSaveBlock1Ptr).secretBases[*data.at(4)].set_registryStatus(UNREGISTERED);
    BuildRegistryMenuItems(taskId);
    SetCursorWithinListBounds(
        data.at(2) as *mut u16,
        data.at(1) as *mut u16,
        *data.at(3) as u8,
        *data as u8,
    );
    FinalizeRegistryMenu(taskId);
    task_set_func(taskId, Some(HandleRegistryMenuInput));
}
pub(crate) unsafe fn DeleteRegistry_Yes(taskId: u8) {
    DisplayItemMessageOnField(
        taskId,
        (*(&raw const crate::data::strings::gText_RegisteredDataDeleted).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        Some(DeleteRegistry_Yes_Callback),
    );
}
pub(crate) unsafe fn DeleteRegistry_No(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    ClearDialogWindowAndFrame(0, 0);
    DestroyListMenuTask(
        *data.at(5) as u8,
        data.at(2) as *mut u16,
        data.at(1) as *mut u16,
    );
    FinalizeRegistryMenu(taskId);
    task_set_func(taskId, Some(HandleRegistryMenuInput));
}
pub(crate) unsafe fn ReturnToMainRegistryMenu(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    AddRegistryMenuScrollArrows(taskId);
    ClearStdWindowAndFrame(*data.at(7) as u8, FALSE);
    ClearWindowTilemap(*data.at(7) as u8);
    RemoveWindow(*data.at(7) as u8);
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(HandleRegistryMenuInput));
}
pub(crate) unsafe fn GoToSecretBasePCRegisterMenu(taskId: u8) {
    if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
        ScriptContext_SetupScript(
            (*crate::asmdata::SecretBase_EventScript_PCCancel.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        ScriptContext_SetupScript(
            (*crate::asmdata::SecretBase_EventScript_ShowRegisterMenu.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    DestroyTask(taskId);
}
unsafe fn GetSecretBaseOwnerType(secretBaseIdx: u8) -> u8 {
    ((*gSaveBlock1Ptr).secretBases[secretBaseIdx].trainerId[0] as i32 % 5) as u8
        + (*gSaveBlock1Ptr).secretBases[secretBaseIdx].gender() * 5
}
pub unsafe fn GetSecretBaseTrainerLoseText() -> *mut u8 {
    let ownerType: u8 = GetSecretBaseOwnerType(VarGet(VAR_CURRENT_SECRET_BASE) as u8);
    if ownerType == 0 {
        return (*crate::asmdata::SecretBase_Text_Trainer0Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 1 {
        return (*crate::asmdata::SecretBase_Text_Trainer1Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 2 {
        return (*crate::asmdata::SecretBase_Text_Trainer2Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 3 {
        return (*crate::asmdata::SecretBase_Text_Trainer3Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 4 {
        return (*crate::asmdata::SecretBase_Text_Trainer4Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 5 {
        return (*crate::asmdata::SecretBase_Text_Trainer5Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 6 {
        return (*crate::asmdata::SecretBase_Text_Trainer6Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 7 {
        return (*crate::asmdata::SecretBase_Text_Trainer7Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else if ownerType == 8 {
        return (*crate::asmdata::SecretBase_Text_Trainer8Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    } else {
        return (*crate::asmdata::SecretBase_Text_Trainer9Defeated.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    #[allow(unreachable_code)]
    {
        null_mut()
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PrepSecretBaseBattleFlags() {
    TryGainNewFanFromCounter(FANCOUNTER_BATTLED_AT_BASE);
    gTrainerBattleOpponent_A = TRAINER_SECRET_BASE;
    gBattleTypeFlags = 0x8000008;
}
#[unsafe(no_mangle)]
pub unsafe fn SetBattledOwnerFromResult() {
    (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)]
        .set_battledOwnerToday(gSpecialVar_Result as u8);
}
#[unsafe(no_mangle)]
pub unsafe fn GetSecretBaseOwnerAndState() {
    let secretBaseIdx: u16 = VarGet(VAR_CURRENT_SECRET_BASE);
    if FlagGet(2338) == 0 {
        for i in 0..SECRET_BASES_COUNT {
            (*gSaveBlock1Ptr).secretBases[i].set_battledOwnerToday(FALSE);
        }
        FlagSet(2338);
    }
    gSpecialVar_0x8004 = GetSecretBaseOwnerType(secretBaseIdx as u8) as u16;
    gSpecialVar_Result = (*gSaveBlock1Ptr).secretBases[secretBaseIdx].battledOwnerToday() as u16;
}
pub unsafe fn SecretBasePerStepCallback(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut behavior: u8 = 0;
    let mut tileId: u16 = 0;
    let mut data: *mut i16 = null_mut();
    data = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    match *data.at(1) {
        0 => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                sInFriendSecretBase.set(TRUE);
            } else {
                sInFriendSecretBase.set(FALSE);
            }
            PlayerGetDestCoords(data.at(2), data.at(3));
            *data.at(1) = 1;
        }
        1 => {
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            if x == *data.at(2) && y == *data.at(3) {
                return;
            }
            *data.at(2) = x;
            *data.at(3) = y;
            VarSet(
                VAR_SECRET_BASE_STEP_COUNTER,
                VarGet(VAR_SECRET_BASE_STEP_COUNTER) + 1,
            );
            behavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
            tileId = MapGridGetMetatileIdAt(x as i32, y as i32) as u16;
            if tileId == METATILE_SecretBase_SolidBoard_Top
                || tileId == METATILE_SecretBase_SolidBoard_Bottom
            {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_SOLID_BOARD,
                    );
                }
            } else if tileId == METATILE_SecretBase_SmallChair
                || tileId == METATILE_SecretBase_PokemonChair
                || tileId == METATILE_SecretBase_HeavyChair
                || tileId == METATILE_SecretBase_PrettyChair
                || tileId == METATILE_SecretBase_ComfortChair
                || tileId == METATILE_SecretBase_RaggedChair
                || tileId == METATILE_SecretBase_BrickChair
                || tileId == METATILE_SecretBase_CampChair
                || tileId == METATILE_SecretBase_HardChair
            {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_LOW_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_CHAIR,
                    );
                }
            } else if tileId == METATILE_SecretBase_RedTent_DoorTop
                || tileId == METATILE_SecretBase_RedTent_Door
                || tileId == METATILE_SecretBase_BlueTent_DoorTop
                || tileId == METATILE_SecretBase_BlueTent_Door
            {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_LOW_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_TENT,
                    );
                }
            } else if behavior == MB_IMPASSABLE_NORTHEAST
                && tileId == METATILE_SecretBase_Stand_CornerRight
                || behavior == MB_IMPASSABLE_NORTHWEST
                    && MapGridGetMetatileIdAt(x as i32, y as i32)
                        == METATILE_SecretBase_Stand_CornerLeft
            {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_STAND,
                    );
                }
            } else if behavior == MB_IMPASSABLE_WEST_AND_EAST
                && tileId == METATILE_SecretBase_Slide_StairLanding
            {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) ^ SECRET_BASE_USED_SLIDE,
                    );
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_DECLINED_SLIDE,
                    );
                }
            } else if behavior == MB_SLIDE_SOUTH && tileId == METATILE_SecretBase_Slide_SlideTop {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_SLIDE,
                    );
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) ^ SECRET_BASE_DECLINED_SLIDE,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseGlitterMat(behavior) == TRUE {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_GLITTER_MAT,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseBalloon(behavior) == TRUE {
                PopSecretBaseBalloon(MapGridGetMetatileIdAt(x as i32, y as i32) as i16, x, y);
                if sInFriendSecretBase.get() == TRUE {
                    match MapGridGetMetatileIdAt(x as i32, y as i32) {
                        824 | 828 | 832 => {
                            VarSet(
                                VAR_SECRET_BASE_LOW_TV_FLAGS,
                                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_BALLOON,
                            );
                        }
                        552 => {
                            VarSet(
                                VAR_SECRET_BASE_LOW_TV_FLAGS,
                                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_MUD_BALL,
                            );
                        }
                        _ => {}
                    }
                }
            } else if MetatileBehavior_IsSecretBaseBreakableDoor(behavior) == TRUE {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_BREAKABLE_DOOR,
                    );
                }
                ShatterSecretBaseBreakableDoor(x, y);
            } else if MetatileBehavior_IsSecretBaseSoundMat(behavior) == TRUE {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_LOW_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_NOTE_MAT,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseJumpMat(behavior) == TRUE {
                if sInFriendSecretBase.get() == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_JUMP_MAT,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseSpinMat(behavior) == TRUE
                && sInFriendSecretBase.get() == TRUE
            {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_SPIN_MAT,
                );
            }
        }
        2 if FieldEffectActiveListContains(*data.at(4) as u8) == 0 => {
            *data.at(1) = 1;
        }
        _ => {}
    }
}
unsafe fn SaveSecretBase(
    secretBaseIdx: u8,
    secretBase: *mut SecretBase,
    version: u32,
    language: u32,
) {
    let mut stringLength: i32 = 0;
    let mut name: *mut u8 = null_mut();
    (*gSaveBlock1Ptr).secretBases[secretBaseIdx] = *secretBase;
    (*gSaveBlock1Ptr).secretBases[secretBaseIdx].set_registryStatus(NEW);
    if version == VERSION_SAPPHIRE as u32 || version == VERSION_RUBY as u32 {
        (*gSaveBlock1Ptr).secretBases[secretBaseIdx].language = GAME_LANGUAGE;
    }
    if version == VERSION_EMERALD as u32 && language == LANGUAGE_JAPANESE as u32 {
        name = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .trainerName
            .as_mut_ptr();
        stringLength = 0;
        while stringLength < PLAYER_NAME_LENGTH {
            if *name.at(stringLength) == EOS {
                break;
            }
            stringLength += 1;
        }
        if stringLength > 5 {
            (*gSaveBlock1Ptr).secretBases[secretBaseIdx].language = GAME_LANGUAGE;
        }
    }
}
unsafe fn SecretBasesHaveSameTrainerId(
    secretBase1: *mut SecretBase,
    secretBase2: *mut SecretBase,
) -> u8 {
    for i in 0..TRAINER_ID_LENGTH {
        if (*secretBase1).trainerId[i] != (*secretBase2).trainerId[i] {
            return FALSE;
        }
    }
    TRUE
}
unsafe fn SecretBasesHaveSameTrainerName(sbr1: *mut SecretBase, sbr2: *mut SecretBase) -> u8 {
    let mut i: u8 = 0;
    while i < PLAYER_NAME_LENGTH as u8
        && ((*sbr1).trainerName[i] != EOS || (*sbr2).trainerName[i] != EOS)
    {
        if (*sbr1).trainerName[i] != (*sbr2).trainerName[i] {
            return FALSE;
        }
        i += 1;
    }
    TRUE
}
unsafe fn SecretBasesBelongToSamePlayer(
    secretBase1: *mut SecretBase,
    secretBase2: *mut SecretBase,
) -> u8 {
    if (*secretBase1).gender() == (*secretBase2).gender()
        && SecretBasesHaveSameTrainerId(secretBase1, secretBase2) != 0
        && SecretBasesHaveSameTrainerName(secretBase1, secretBase2) != 0
    {
        return TRUE;
    }
    FALSE
}
unsafe fn GetSecretBaseIndexFromId(secretBaseId: u8) -> i16 {
    for i in 0..(SECRET_BASES_COUNT as i16) {
        if (*gSaveBlock1Ptr).secretBases[i].secretBaseId == secretBaseId {
            return i;
        }
    }
    -1
}
unsafe fn FindAvailableSecretBaseIndex() -> u8 {
    for i in 1..(SECRET_BASES_COUNT as i16) {
        if (*gSaveBlock1Ptr).secretBases[i].secretBaseId == 0 {
            return i as u8;
        }
    }
    0
}
unsafe fn FindUnregisteredSecretBaseIndex() -> u8 {
    for i in 1..(SECRET_BASES_COUNT as i16) {
        if (*gSaveBlock1Ptr).secretBases[i].registryStatus() == UNREGISTERED
            && (*gSaveBlock1Ptr).secretBases[i].toRegister() == FALSE
        {
            return i as u8;
        }
    }
    0
}
unsafe fn TrySaveFriendsSecretBase(secretBase: *mut SecretBase, version: u32, language: u32) -> u8 {
    if (*secretBase).secretBaseId == 0 {
        return 0;
    }
    let mut index: i16 = GetSecretBaseIndexFromId((*secretBase).secretBaseId);
    if index != 0 {
        if index != -1 {
            if (*gSaveBlock1Ptr).secretBases[index].toRegister() == TRUE {
                return 0;
            }
            if (*gSaveBlock1Ptr).secretBases[index].registryStatus() != NEW
                || (*secretBase).toRegister() == TRUE
            {
                SaveSecretBase(index as u8, secretBase, version, language);
                return index as u8;
            }
        } else {
            index = FindAvailableSecretBaseIndex() as i16;
            if index != 0 {
                SaveSecretBase(index as u8, secretBase, version, language);
                return index as u8;
            }
            index = FindUnregisteredSecretBaseIndex() as i16;
            if index != 0 {
                SaveSecretBase(index as u8, secretBase, version, language);
                return index as u8;
            }
        }
    }
    0
}
unsafe fn SortSecretBasesByRegistryStatus() {
    let mut secretBases: *mut SecretBase = null_mut();
    secretBases = (*gSaveBlock1Ptr).secretBases.as_mut_ptr();
    for i in 1..19u8 {
        for j in (i + 1)..SECRET_BASES_COUNT {
            if (*secretBases.at(i)).registryStatus() == UNREGISTERED
                && (*secretBases.at(j)).registryStatus() == REGISTERED
                || (*secretBases.at(i)).registryStatus() == NEW
                    && (*secretBases.at(j)).registryStatus() != NEW
            {
                let temp: SecretBase = *secretBases.at(i);
                *secretBases.at(i) = *secretBases.at(j);
                *secretBases.at(j) = temp;
            }
        }
    }
}
unsafe fn TrySaveFriendsSecretBases(mixer: *mut SecretBaseRecordMixer, registryStatus: u8) {
    for i in 1..(SECRET_BASES_COUNT as u16) {
        if (*(*mixer).secretBases.at(i)).registryStatus() == registryStatus {
            TrySaveFriendsSecretBase(
                (*mixer).secretBases.at(i),
                (*mixer).version,
                (*mixer).language,
            );
        }
    }
}
unsafe fn SecretBaseBelongsToPlayer(secretBase: *mut SecretBase) -> u8 {
    if (*secretBase).secretBaseId == 0 {
        return FALSE;
    }
    if (*secretBase).secretBaseId != 0 && (*secretBase).gender() != (*gSaveBlock2Ptr).playerGender {
        return FALSE;
    }
    for i in 0..TRAINER_ID_LENGTH {
        if (*secretBase).trainerId[i] != (*gSaveBlock2Ptr).playerTrainerId[i] {
            return FALSE;
        }
    }
    let mut i: u8 = 0;
    while i < PLAYER_NAME_LENGTH as u8
        && ((*secretBase).trainerName[i] != EOS || (*gSaveBlock2Ptr).playerName[i] != EOS)
    {
        if (*secretBase).trainerName[i] != (*gSaveBlock2Ptr).playerName[i] {
            return FALSE;
        }
        i += 1;
    }
    TRUE
}
unsafe fn DeleteFirstOldBaseFromPlayerInRecordMixingFriendsRecords(
    basesA: *mut SecretBase,
    basesB: *mut SecretBase,
    basesC: *mut SecretBase,
) {
    let mut sbFlags: u8 = 0;
    for i in 0..SECRET_BASES_COUNT {
        if sbFlags as i32 & DELETED_BASE_A == 0 && SecretBaseBelongsToPlayer(basesA.at(i)) == TRUE {
            ClearSecretBase(basesA.at(i));
            sbFlags |= DELETED_BASE_A as u8;
        }
        if sbFlags as i32 & DELETED_BASE_B == 0 && SecretBaseBelongsToPlayer(basesB.at(i)) == TRUE {
            ClearSecretBase(basesB.at(i));
            sbFlags |= DELETED_BASE_B as u8;
        }
        if sbFlags as i32 & DELETED_BASE_C == 0 && SecretBaseBelongsToPlayer(basesC.at(i)) == TRUE {
            ClearSecretBase(basesC.at(i));
            sbFlags |= DELETED_BASE_C as u8;
        }
        if sbFlags == 7 {
            break;
        }
    }
}
unsafe fn ClearDuplicateOwnedSecretBase(
    secretBase: *mut SecretBase,
    secretBases: *mut SecretBase,
    idx: u8,
) -> u8 {
    for i in 0..SECRET_BASES_COUNT {
        if (*secretBases.at(i)).secretBaseId != 0
            && SecretBasesBelongToSamePlayer(secretBase, secretBases.at(i)) == TRUE
        {
            if idx == 0 {
                ClearSecretBase(secretBases.at(i));
                return FALSE;
            }
            if (*secretBase).numSecretBasesReceived > (*secretBases.at(i)).numSecretBasesReceived {
                ClearSecretBase(secretBases.at(i));
                return FALSE;
            }
            (*secretBases.at(i)).set_toRegister((*secretBase).toRegister());
            ClearSecretBase(secretBase);
            return TRUE;
        }
    }
    FALSE
}
unsafe fn ClearDuplicateOwnedSecretBases(
    playersBases: *mut SecretBase,
    friendsBasesA: *mut SecretBase,
    friendsBasesB: *mut SecretBase,
    friendsBasesC: *mut SecretBase,
) {
    for i in 1..SECRET_BASES_COUNT {
        if (*playersBases.at(i)).secretBaseId != 0 {
            if (*playersBases.at(i)).registryStatus() == REGISTERED {
                (*playersBases.at(i)).set_toRegister(TRUE);
            }
            if ClearDuplicateOwnedSecretBase(playersBases.at(i), friendsBasesA, i) == 0
                && ClearDuplicateOwnedSecretBase(playersBases.at(i), friendsBasesB, i) == 0
            {
                ClearDuplicateOwnedSecretBase(playersBases.at(i), friendsBasesC, i);
            }
        }
    }
    let mut i: u8 = 0;
    while i < SECRET_BASES_COUNT {
        if (*friendsBasesA.at(i)).secretBaseId != 0 {
            (*friendsBasesA.at(i)).set_battledOwnerToday(0);
            if ClearDuplicateOwnedSecretBase(friendsBasesA.at(i), friendsBasesB, i) == 0 {
                ClearDuplicateOwnedSecretBase(friendsBasesA.at(i), friendsBasesC, i);
            }
        }
        i += 1;
    }
    for i in 0..SECRET_BASES_COUNT {
        if (*friendsBasesB.at(i)).secretBaseId != 0 {
            (*friendsBasesB.at(i)).set_battledOwnerToday(0);
            ClearDuplicateOwnedSecretBase(friendsBasesB.at(i), friendsBasesC, i);
        }
        if (*friendsBasesC.at(i)).secretBaseId != 0 {
            (*friendsBasesC.at(i)).set_battledOwnerToday(0);
        }
    }
}
unsafe fn TrySaveRegisteredDuplicate(base: *mut SecretBase, version: u32, language: u32) {
    if (*base).toRegister() == TRUE {
        TrySaveFriendsSecretBase(base, version, language);
        ClearSecretBase(base);
    }
}
unsafe fn TrySaveRegisteredDuplicates(mixers: *mut SecretBaseRecordMixer) {
    for i in 0..(SECRET_BASES_COUNT as u16) {
        TrySaveRegisteredDuplicate(
            (*mixers).secretBases.at(i),
            (*mixers).version,
            (*mixers).language,
        );
        TrySaveRegisteredDuplicate(
            (*mixers.at(1)).secretBases.at(i),
            (*mixers.at(1)).version,
            (*mixers.at(1)).language,
        );
        TrySaveRegisteredDuplicate(
            (*mixers.at(2)).secretBases.at(i),
            (*mixers.at(2)).version,
            (*mixers.at(2)).language,
        );
    }
}
unsafe fn SaveRecordMixBases(mixers: *mut SecretBaseRecordMixer) {
    DeleteFirstOldBaseFromPlayerInRecordMixingFriendsRecords(
        (*mixers).secretBases,
        (*mixers.at(1)).secretBases,
        (*mixers.at(2)).secretBases,
    );
    ClearDuplicateOwnedSecretBases(
        (*gSaveBlock1Ptr).secretBases.as_mut_ptr(),
        (*mixers).secretBases,
        (*mixers.at(1)).secretBases,
        (*mixers.at(2)).secretBases,
    );
    TrySaveRegisteredDuplicates(mixers);
    TrySaveFriendsSecretBase((*mixers).secretBases, (*mixers).version, (*mixers).language);
    TrySaveFriendsSecretBase(
        (*mixers.at(1)).secretBases,
        (*mixers.at(1)).version,
        (*mixers.at(1)).language,
    );
    TrySaveFriendsSecretBase(
        (*mixers.at(2)).secretBases,
        (*mixers.at(2)).version,
        (*mixers.at(2)).language,
    );
    TrySaveFriendsSecretBases(mixers, REGISTERED);
    TrySaveFriendsSecretBases(mixers.at(1), REGISTERED);
    TrySaveFriendsSecretBases(mixers.at(2), REGISTERED);
    TrySaveFriendsSecretBases(mixers, UNREGISTERED);
    TrySaveFriendsSecretBases(mixers.at(1), UNREGISTERED);
    TrySaveFriendsSecretBases(mixers.at(2), UNREGISTERED);
}
pub unsafe fn ReceiveSecretBasesData(secretBases: *mut c_void, recordSize: u32, linkIdx: u8) {
    let mut mixers: CArray<SecretBaseRecordMixer, 3> = zeroed();
    let mut i: u16 = 0;
    if FlagGet(FLAG_RECEIVED_SECRET_POWER) != 0 {
        match GetLinkPlayerCount() {
            2 => {
                memset(
                    (secretBases as *mut u8).at(2 * recordSize) as *mut c_void as *mut u8,
                    0,
                    recordSize,
                );
                memset(
                    (secretBases as *mut u8).at(3 * recordSize) as *mut c_void as *mut u8,
                    0,
                    recordSize,
                );
            }
            3 => {
                memset(
                    (secretBases as *mut u8).at(3 * recordSize) as *mut c_void as *mut u8,
                    0,
                    recordSize,
                );
            }
            _ => {}
        }
        match linkIdx {
            0 => {
                mixers[0].secretBases =
                    (secretBases as *mut u8).at(recordSize) as *mut c_void as *mut SecretBase;
                mixers[0].version = gLinkPlayers[1].version as u32 & 0xFF;
                mixers[0].language = gLinkPlayers[1].language as u32;
                mixers[1].secretBases =
                    (secretBases as *mut u8).at(2 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[1].version = gLinkPlayers[2].version as u32 & 0xFF;
                mixers[1].language = gLinkPlayers[2].language as u32;
                mixers[2].secretBases =
                    (secretBases as *mut u8).at(3 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[2].version = gLinkPlayers[3].version as u32 & 0xFF;
                mixers[2].language = gLinkPlayers[3].language as u32;
            }
            1 => {
                mixers[0].secretBases =
                    (secretBases as *mut u8).at(2 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[0].version = gLinkPlayers[2].version as u32 & 0xFF;
                mixers[0].language = gLinkPlayers[2].language as u32;
                mixers[1].secretBases =
                    (secretBases as *mut u8).at(3 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[1].version = gLinkPlayers[3].version as u32 & 0xFF;
                mixers[1].language = gLinkPlayers[3].language as u32;
                mixers[2].secretBases =
                    (secretBases as *mut u8).at(0 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[2].version = gLinkPlayers[0].version as u32 & 0xFF;
                mixers[2].language = gLinkPlayers[0].language as u32;
            }
            2 => {
                mixers[0].secretBases =
                    (secretBases as *mut u8).at(3 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[0].version = gLinkPlayers[3].version as u32 & 0xFF;
                mixers[0].language = gLinkPlayers[3].language as u32;
                mixers[1].secretBases =
                    (secretBases as *mut u8).at(0 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[1].version = gLinkPlayers[0].version as u32 & 0xFF;
                mixers[1].language = gLinkPlayers[0].language as u32;
                mixers[2].secretBases =
                    (secretBases as *mut u8).at(recordSize) as *mut c_void as *mut SecretBase;
                mixers[2].version = gLinkPlayers[1].version as u32 & 0xFF;
                mixers[2].language = gLinkPlayers[1].language as u32;
            }
            3 => {
                mixers[0].secretBases =
                    (secretBases as *mut u8).at(0 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[0].version = gLinkPlayers[0].version as u32 & 0xFF;
                mixers[0].language = gLinkPlayers[0].language as u32;
                mixers[1].secretBases =
                    (secretBases as *mut u8).at(recordSize) as *mut c_void as *mut SecretBase;
                mixers[1].version = gLinkPlayers[1].version as u32 & 0xFF;
                mixers[1].language = gLinkPlayers[1].language as u32;
                mixers[2].secretBases =
                    (secretBases as *mut u8).at(2 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[2].version = gLinkPlayers[2].version as u32 & 0xFF;
                mixers[2].language = gLinkPlayers[2].language as u32;
            }
            _ => {}
        }
        SaveRecordMixBases(mixers.as_mut_ptr());
        i = 1;
        while i < SECRET_BASES_COUNT as u16 {
            if (*gSaveBlock1Ptr).secretBases[i].toRegister() == TRUE {
                (*gSaveBlock1Ptr).secretBases[i].set_registryStatus(REGISTERED);
                (*gSaveBlock1Ptr).secretBases[i].set_toRegister(FALSE);
            }
            i += 1;
        }
        SortSecretBasesByRegistryStatus();
        for i in 1..(SECRET_BASES_COUNT as u16) {
            if (*gSaveBlock1Ptr).secretBases[i].registryStatus() == NEW {
                (*gSaveBlock1Ptr).secretBases[i].set_registryStatus(UNREGISTERED);
            }
        }
        if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != 0
            && (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived != 0xFFFF
        {
            (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived += 1;
        }
    }
}
pub unsafe fn ClearJapaneseSecretBases(bases: *mut SecretBase) {
    for i in 0..(SECRET_BASES_COUNT as u32) {
        if (*bases.at(i)).language == LANGUAGE_JAPANESE {
            ClearSecretBase(bases.at(i));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn InitSecretBaseVars() {
    VarSet(VAR_SECRET_BASE_STEP_COUNTER, 0);
    VarSet(VAR_SECRET_BASE_LAST_ITEM_USED, 0);
    VarSet(VAR_SECRET_BASE_LOW_TV_FLAGS, 0);
    VarSet(VAR_SECRET_BASE_HIGH_TV_FLAGS, 0);
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, TRUE as u16);
    } else {
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, FALSE as u16);
    }
    sInFriendSecretBase.set(FALSE);
}
pub unsafe fn CheckLeftFriendsSecretBase() {
    if VarGet(VAR_SECRET_BASE_IS_NOT_LOCAL) != 0
        && sInFriendSecretBase.get() == TRUE
        && CurMapIsSecretBase() == 0
    {
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, FALSE as u16);
        sInFriendSecretBase.set(FALSE);
        TryPutSecretBaseSecretsOnAir();
        VarSet(VAR_SECRET_BASE_STEP_COUNTER, 0);
        VarSet(VAR_SECRET_BASE_LAST_ITEM_USED, 0);
        VarSet(VAR_SECRET_BASE_LOW_TV_FLAGS, 0);
        VarSet(VAR_SECRET_BASE_HIGH_TV_FLAGS, 0);
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, FALSE as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsDollDecor() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DOLL,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsCushionDecor() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_CUSHION,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DeclinedSecretBaseBattle() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) & 51199,
        );
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) & 65534,
        );
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_DECLINED_BATTLE,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn WonSecretBaseBattle() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) & 51199,
        );
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) & 65534,
        );
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_BATTLED_WON,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LostSecretBaseBattle() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) & 51199,
        );
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) & 65534,
        );
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_BATTLED_LOST,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DrewSecretBaseBattle() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) & 51199,
        );
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) & 65534,
        );
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_BATTLED_DRAW,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsPosterDecor() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        METATILE_SecretBase_PikaPoster_Left
        | METATILE_SecretBase_PikaPoster_Right
        | METATILE_SecretBase_LongPoster_Left
        | METATILE_SecretBase_LongPoster_Right
        | METATILE_SecretBase_SeaPoster_Left
        | METATILE_SecretBase_SeaPoster_Right
        | METATILE_SecretBase_SkyPoster_Left
        | METATILE_SecretBase_SkyPoster_Right
        | METATILE_SecretBase_KissPoster_Left
        | METATILE_SecretBase_KissPoster_Right
        | METATILE_SecretBase_BallPoster
        | METATILE_SecretBase_GreenPoster
        | METATILE_SecretBase_RedPoster
        | METATILE_SecretBase_BluePoster
        | METATILE_SecretBase_CutePoster
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 =>
        {
            VarSet(
                VAR_SECRET_BASE_LOW_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_POSTER,
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsFurnitureBottom() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        METATILE_SecretBase_GlassOrnament_Base1 | METATILE_SecretBase_GlassOrnament_Base2 => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_LOW_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_GLASS_ORNAMENT,
                );
            }
        }
        METATILE_SecretBase_RedPlant_Base1
        | METATILE_SecretBase_RedPlant_Base2
        | METATILE_SecretBase_TropicalPlant_Base1
        | METATILE_SecretBase_TropicalPlant_Base2
        | METATILE_SecretBase_PrettyFlowers_Base1
        | METATILE_SecretBase_PrettyFlowers_Base2
        | METATILE_SecretBase_ColorfulPlant_BaseLeft1
        | METATILE_SecretBase_ColorfulPlant_BaseRight1
        | METATILE_SecretBase_ColorfulPlant_BaseLeft2
        | METATILE_SecretBase_ColorfulPlant_BaseRight2
        | METATILE_SecretBase_BigPlant_BaseLeft1
        | METATILE_SecretBase_BigPlant_BaseRight1
        | METATILE_SecretBase_BigPlant_BaseLeft2
        | METATILE_SecretBase_BigPlant_BaseRight2
        | METATILE_SecretBase_GorgeousPlant_BaseLeft1
        | METATILE_SecretBase_GorgeousPlant_BaseRight1
        | METATILE_SecretBase_GorgeousPlant_BaseLeft2
        | METATILE_SecretBase_GorgeousPlant_BaseRight2 => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_LOW_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_PLANT,
                );
            }
        }
        METATILE_SecretBase_Fence_Horizontal | METATILE_SecretBase_Fence_Vertical => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_FENCE,
                );
            }
        }
        METATILE_SecretBase_Tire_BottomLeft | METATILE_SecretBase_Tire_BottomRight => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_TIRE,
                );
            }
        }
        METATILE_SecretBase_RedBrick_Bottom
        | METATILE_SecretBase_YellowBrick_Bottom
        | METATILE_SecretBase_BlueBrick_Bottom => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_BRICK,
                );
            }
        }
        METATILE_SecretBase_SmallDesk
        | METATILE_SecretBase_PokemonDesk
        | METATILE_SecretBase_HeavyDesk_BottomLeft
        | METATILE_SecretBase_HeavyDesk_BottomMid
        | METATILE_SecretBase_HeavyDesk_BottomRight
        | METATILE_SecretBase_RaggedDesk_BottomLeft
        | METATILE_SecretBase_RaggedDesk_BottomMid
        | METATILE_SecretBase_RaggedDesk_BottomRight
        | METATILE_SecretBase_ComfortDesk_BottomLeft
        | METATILE_SecretBase_ComfortDesk_BottomMid
        | METATILE_SecretBase_ComfortDesk_BottomRight
        | METATILE_SecretBase_BrickDesk_BottomLeft
        | METATILE_SecretBase_BrickDesk_BottomMid
        | METATILE_SecretBase_BrickDesk_BottomRight
        | METATILE_SecretBase_CampDesk_BottomLeft
        | METATILE_SecretBase_CampDesk_BottomMid
        | METATILE_SecretBase_CampDesk_BottomRight
        | METATILE_SecretBase_HardDesk_BottomLeft
        | METATILE_SecretBase_HardDesk_BottomMid
        | METATILE_SecretBase_HardDesk_BottomRight
        | METATILE_SecretBase_PrettyDesk_BottomLeft
        | METATILE_SecretBase_PrettyDesk_BottomMid
        | METATILE_SecretBase_PrettyDesk_BottomRight
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 =>
        {
            VarSet(
                VAR_SECRET_BASE_HIGH_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DESK,
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsFurnitureMiddle() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        METATILE_SecretBase_HeavyDesk_TopMid
        | METATILE_SecretBase_RaggedDesk_TopMid
        | METATILE_SecretBase_ComfortDesk_TopMid
        | METATILE_SecretBase_BrickDesk_TopMid
        | METATILE_SecretBase_BrickDesk_Center
        | METATILE_SecretBase_CampDesk_TopMid
        | METATILE_SecretBase_CampDesk_Center
        | METATILE_SecretBase_HardDesk_TopMid
        | METATILE_SecretBase_HardDesk_Center
        | METATILE_SecretBase_PrettyDesk_TopMid
        | METATILE_SecretBase_PrettyDesk_Center
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 =>
        {
            VarSet(
                VAR_SECRET_BASE_HIGH_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DESK,
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsFurnitureTop() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        METATILE_SecretBase_HeavyDesk_TopLeft
        | METATILE_SecretBase_HeavyDesk_TopRight
        | METATILE_SecretBase_RaggedDesk_TopLeft
        | METATILE_SecretBase_RaggedDesk_TopRight
        | METATILE_SecretBase_ComfortDesk_TopLeft
        | METATILE_SecretBase_ComfortDesk_TopRight
        | METATILE_SecretBase_BrickDesk_TopLeft
        | METATILE_SecretBase_BrickDesk_TopRight
        | METATILE_SecretBase_BrickDesk_MidLeft
        | METATILE_SecretBase_BrickDesk_MidRight
        | METATILE_SecretBase_CampDesk_TopLeft
        | METATILE_SecretBase_CampDesk_TopRight
        | METATILE_SecretBase_CampDesk_MidLeft
        | METATILE_SecretBase_CampDesk_MidRight
        | METATILE_SecretBase_HardDesk_TopLeft
        | METATILE_SecretBase_HardDesk_TopRight
        | METATILE_SecretBase_HardDesk_MidLeft
        | METATILE_SecretBase_HardDesk_MidRight
        | METATILE_SecretBase_PrettyDesk_TopLeft
        | METATILE_SecretBase_PrettyDesk_TopRight
        | METATILE_SecretBase_PrettyDesk_MidLeft
        | METATILE_SecretBase_PrettyDesk_MidRight => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DESK,
                );
            }
        }
        METATILE_SecretBase_Tire_TopLeft | METATILE_SecretBase_Tire_TopRight => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_TIRE,
                );
            }
        }
        METATILE_SecretBase_RedBrick_Top
        | METATILE_SecretBase_YellowBrick_Top
        | METATILE_SecretBase_BlueBrick_Top
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 =>
        {
            VarSet(
                VAR_SECRET_BASE_HIGH_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_BRICK,
            );
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CheckInteractedWithFriendsSandOrnament() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        653 | METATILE_SecretBase_SandOrnament_Base2 if VarGet(VAR_CURRENT_SECRET_BASE) != 0 => {
            VarSet(
                VAR_SECRET_BASE_HIGH_TV_FLAGS,
                VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_SAND_ORNAMENT,
            );
        }
        _ => {}
    }
}
