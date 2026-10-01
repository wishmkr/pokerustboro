//! Translated from `src/secret_base.c` by tools/rustport/c2rs.py.
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
pub(crate) static mut sCurSecretBaseId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInFriendSecretBase: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRegistryMenu: *mut SecretBaseRegistryMenu = null_mut();

unsafe extern "C" {
    static SecretBase_EventScript_Enter: CArray<u8, 0>;
    static SecretBase_EventScript_PCCancel: CArray<u8, 0>;
    static SecretBase_EventScript_ShowRegisterMenu: CArray<u8, 0>;
    static SecretBase_Text_Trainer0Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer1Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer2Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer3Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer4Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer5Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer6Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer7Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer8Defeated: CArray<u8, 0>;
    static SecretBase_Text_Trainer9Defeated: CArray<u8, 0>;
    static mut gBattleTypeFlags: u32;
    static gDecorations: CArray<Decoration, 0>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gMapHeader: MapHeader;
    static mut gMultiuseListMenuTemplate: ListMenuTemplate;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_0x8007: u16;
    static mut gSpecialVar_Result: u16;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_ApostropheSBase: CArray<u8, 0>;
    static gText_Cancel: CArray<u8, 0>;
    static gText_NoRegistry: CArray<u8, 0>;
    static gText_OkayToDeleteFromRegistry: CArray<u8, 0>;
    static gText_RegisteredDataDeleted: CArray<u8, 0>;
    static mut gTrainerBattleOpponent_A: u16;
    fn AddScrollIndicatorArrowPairParameterized(
        a0: u32,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: *mut u16,
    ) -> u8;
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn CB2_LoadMap();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CpuFastSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn DisplayYesNoMenuDefaultYes();
    fn DoSecretBaseDecorationMenu(a0: u8);
    fn DoYesNoFuncWithChoice(a0: u8, a1: *mut YesNoFuncTable);
    fn DrawWholeMapView();
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FieldCB_DefaultWarpExit();
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn GetLinkPlayerCount() -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut MenuAction, a1: i32) -> i32;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn HideMapNamePopUpWindow();
    fn IncrementGameStat(a0: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn Menu_ProcessInputNoWrap() -> i8;
    fn MetatileBehavior_HoldsLargeDecoration(a0: u8) -> u8;
    fn MetatileBehavior_HoldsSmallDecoration(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseBalloon(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseBreakableDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseGlitterMat(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseJumpMat(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseSoundMat(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseSpinMat(a0: u8) -> u8;
    fn ObjectEventTurn(a0: *mut ObjectEvent, a1: u8);
    fn OverrideSecretBaseDecorationSpriteScript(a0: u8, a1: u8, a2: u8, a3: u8);
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PopSecretBaseBalloon(a0: i16, a1: i16, a2: i16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut MenuAction);
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetCursorWithinListBounds(a0: *mut u16, a1: *mut u16, a2: u8, a3: u8);
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn SetWarpDestinationToDynamicWarp(a0: u8);
    fn SetWarpDestinationToMapWarp(a0: i8, a1: i8, a2: i8);
    fn ShatterSecretBaseBreakableDoor(a0: i16, a1: i16);
    fn ShowDecorationOnMap(a0: u16, a1: u16, a2: u16);
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn TryGainNewFanFromCounter(a0: u8) -> u8;
    fn TryMoveObjectEventToMapCoords(a0: u8, a1: u8, a2: u8, a3: i16, a4: i16);
    fn TryOverrideObjectEventTemplateCoords(a0: u8, a1: u8, a2: u8);
    fn TryPutSecretBaseSecretsOnAir();
    fn TrySpawnObjectEvent(a0: u8, a1: u8, a2: u8) -> u8;
    fn UnlockPlayerFieldControls();
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WarpIntoMap();
}

pub(crate) unsafe extern "C" fn ClearSecretBase(secretBase: *mut SecretBase) {
    let mut i: u16 = 0;
    {
        let mut tmp: u32 = 0;
        volatile_write(&raw mut tmp, 0);
        CpuFastSet(
            &raw mut tmp as *mut c_void,
            secretBase as *mut c_void,
            0x1000028,
        );
    }
    i = 0;
    while i < PLAYER_NAME_LENGTH as u16 {
        (*secretBase).trainerName[i] = EOS;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSecretBases() {
    let mut i: u16 = 0;
    i = 0;
    while i < SECRET_BASES_COUNT as u16 {
        ClearSecretBase(&raw mut (*gSaveBlock1Ptr).secretBases[i]);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SetCurSecretBaseId() {
    sCurSecretBaseId = gSpecialVar_0x8004 as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetCurSecretBaseIndex() {
    let mut i: u16 = 0;
    gSpecialVar_Result = FALSE as u16;
    i = 0;
    while i < SECRET_BASES_COUNT as u16 {
        if sCurSecretBaseId == (*gSaveBlock1Ptr).secretBases[i].secretBaseId {
            gSpecialVar_Result = TRUE as u16;
            VarSet(VAR_CURRENT_SECRET_BASE, i);
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPlayerHasSecretBase() {
    if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != 0 {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn GetSecretBaseTypeInFrontOfPlayer_() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut behavior: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    behavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as i16 & 0xFFF;
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
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTypeInFrontOfPlayer() {
    gSpecialVar_0x8007 = GetSecretBaseTypeInFrontOfPlayer_() as u16;
}
pub(crate) unsafe extern "C" fn FindMetatileIdMapCoords(x: *mut i16, y: *mut i16, metatileId: u16) {
    let mut i: i16 = 0;
    let mut j: i16 = 0;
    let mut mapLayout: *mut MapLayout = gMapHeader.mapLayout;
    j = 0;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ToggleSecretBaseEntranceMetatile() {
    let mut i: u16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut metatileId: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    metatileId = MapGridGetMetatileIdAt(x as i32, y as i32) as i16;
    i = 0;
    while i < 7 {
        if sSecretBaseEntranceMetatiles[i].closedMetatileId as i32 == metatileId as i32 {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                sSecretBaseEntranceMetatiles[i].openMetatileId | MAPGRID_IMPASSABLE,
            );
            CurrentMapDrawMetatileAt(x as i32, y as i32);
            return;
        }
        i += 1;
    }
    i = 0;
    while i < 7 {
        if sSecretBaseEntranceMetatiles[i].openMetatileId as i32 == metatileId as i32 {
            MapGridSetMetatileIdAt(
                x as i32,
                y as i32,
                sSecretBaseEntranceMetatiles[i].closedMetatileId | MAPGRID_IMPASSABLE,
            );
            CurrentMapDrawMetatileAt(x as i32, y as i32);
            return;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetNameLength(secretBaseOwnerName: *mut u8) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < PLAYER_NAME_LENGTH as u8 {
        if *secretBaseOwnerName.at(i) == EOS {
            return i;
        }
        i += 1;
    }
    return PLAYER_NAME_LENGTH as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerSecretBase() {
    let mut i: u16 = 0;
    (*gSaveBlock1Ptr).secretBases[0].secretBaseId = sCurSecretBaseId;
    i = 0;
    while i < TRAINER_ID_LENGTH as u16 {
        (*gSaveBlock1Ptr).secretBases[0].trainerId[i] = (*gSaveBlock2Ptr).playerTrainerId[i];
        i += 1;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetOccupiedSecretBaseEntranceMetatiles(events: *mut MapEvents) {
    let mut bgId: u16 = 0;
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    bgId = 0;
    while bgId < (*events).bgEventCount as u16 {
        if (*(*events).bgEvents.at(bgId)).kind == BG_EVENT_SECRET_BASE {
            j = 0;
            while j < SECRET_BASES_COUNT as u16 {
                if (*gSaveBlock1Ptr).secretBases[j].secretBaseId as u32
                    == (*(*events).bgEvents.at(bgId)).bgUnion.secretBaseId
                {
                    let mut x: i16 = (*(*events).bgEvents.at(bgId)).x as i16 + MAP_OFFSET as i16;
                    let mut y: i16 = (*(*events).bgEvents.at(bgId)).y as i16 + MAP_OFFSET as i16;
                    let mut tile_id: i16 = MapGridGetMetatileIdAt(x as i32, y as i32) as i16;
                    i = 0;
                    while i < 7 {
                        if sSecretBaseEntranceMetatiles[i].closedMetatileId as i32 == tile_id as i32
                        {
                            MapGridSetMetatileIdAt(
                                x as i32,
                                y as i32,
                                sSecretBaseEntranceMetatiles[i].openMetatileId | MAPGRID_IMPASSABLE,
                            );
                            break;
                        }
                        i += 1;
                    }
                    break;
                }
                j += 1;
            }
        }
        bgId += 1;
    }
}
pub(crate) unsafe extern "C" fn SetSecretBaseWarpDestination() {
    let mut secretBaseGroup: i8 = (sCurSecretBaseId as i32 / 10) as i8 * 4;
    SetWarpDestinationToMapWarp(
        25,
        sSecretBaseEntrancePositions[secretBaseGroup as i32 + 0] as i8,
        sSecretBaseEntrancePositions[secretBaseGroup as i32 + 1] as i8,
    );
}
pub(crate) unsafe extern "C" fn Task_EnterSecretBase(taskId: u8) {
    let mut secretBaseIdx: u16 = 0;
    match gTasks[taskId].data[0] {
        0 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = 1;
            }
        }
        1 => {
            secretBaseIdx = VarGet(VAR_CURRENT_SECRET_BASE);
            if (*gSaveBlock1Ptr).secretBases[secretBaseIdx].numTimesEntered < 255 {
                (*gSaveBlock1Ptr).secretBases[secretBaseIdx].numTimesEntered += 1;
            }
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
pub unsafe extern "C" fn EnterSecretBase() {
    CreateTask(Some(Task_EnterSecretBase), 0);
    FadeScreen(FADE_TO_BLACK, 0);
    SetDynamicWarp(
        0,
        (*gSaveBlock1Ptr).location.mapGroup,
        (*gSaveBlock1Ptr).location.mapNum,
        WARP_ID_NONE,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SecretBaseMapPopupEnabled() -> u8 {
    if gMapHeader.mapType == MAP_TYPE_SECRET_BASE && VarGet(VAR_INIT_SECRET_BASE) == 0 {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn EnterNewlyCreatedSecretBase_WaitFadeIn(taskId: u8) {
    ObjectEventTurn(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        DIR_NORTH,
    );
    if IsWeatherNotFadingIn() == TRUE {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn EnterNewlyCreatedSecretBase_StartFadeIn() {
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
pub(crate) unsafe extern "C" fn Task_EnterNewlyCreatedSecretBase(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let mut secretBaseGroup: i8 = (sCurSecretBaseId as i32 / 10) as i8 * 4;
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
pub unsafe extern "C" fn EnterNewlyCreatedSecretBase() {
    CreateTask(Some(Task_EnterNewlyCreatedSecretBase), 0);
    FadeScreen(FADE_TO_BLACK, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CurMapIsSecretBase() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup == 25 && (*gSaveBlock1Ptr).location.mapNum as u8 <= 23 {
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
pub unsafe extern "C" fn InitSecretBaseAppearance(hidePC: u8) {
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
                && gDecorations[*decorations.at(x)].permission != DECORPERM_SPRITE
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
pub unsafe extern "C" fn InitSecretBaseDecorationSprites() {
    let mut i: u8 = 0;
    let mut decorations: *mut u8 = null_mut();
    let mut decorationPositions: *mut u8 = null_mut();
    let mut objectEventId: u8 = 0;
    let mut metatileBehavior: u8 = 0;
    let mut category: u8 = 0;
    let mut permission: u8 = 0;
    let mut numDecorations: u8 = 0;
    objectEventId = 0;
    if CurMapIsSecretBase() == 0 {
        decorations = (*gSaveBlock1Ptr).playerRoomDecorations.as_mut_ptr();
        decorationPositions = (*gSaveBlock1Ptr).playerRoomDecorationPositions.as_mut_ptr();
        numDecorations = DECOR_MAX_PLAYERS_HOUSE;
    } else {
        let mut secretBaseIdx: u16 = VarGet(VAR_CURRENT_SECRET_BASE);
        decorations = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .decorations
            .as_mut_ptr();
        decorationPositions = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
            .decorationPositions
            .as_mut_ptr();
        numDecorations = DECOR_MAX_SECRET_BASE;
    }
    i = 0;
    while i < numDecorations {
        'l1: {
            if *decorations.at(i) == DECOR_NONE {
                break 'l1;
            }
            permission = gDecorations[*decorations.at(i)].permission;
            category = gDecorations[*decorations.at(i)].category;
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
                    VarSet(gSpecialVar_Result, *gDecorations[*decorations.at(i)].tiles);
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
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideSecretBaseDecorationSprites() {
    let mut objectEventId: u8 = 0;
    let mut flag: u16 = 0;
    objectEventId = 0;
    while objectEventId < (*gMapHeader.events).objectEventCount {
        flag = (*(*gMapHeader.events).objectEvents.at(objectEventId)).flagId;
        if flag >= FLAG_DECORATION_1 && flag <= FLAG_DECORATION_14 {
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
pub unsafe extern "C" fn SetSecretBaseOwnerGfxId() {
    VarSet(
        VAR_OBJ_GFX_ID_F,
        sSecretBaseOwnerGfxIds[GetSecretBaseOwnerType(VarGet(VAR_CURRENT_SECRET_BASE) as u8)]
            as u16,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurSecretBaseIdFromPosition(
    position: *mut MapPosition,
    events: *mut MapEvents,
) {
    let mut i: i16 = 0;
    i = 0;
    while i < (*events).bgEventCount as i16 {
        if (*(*events).bgEvents.at(i)).kind == BG_EVENT_SECRET_BASE
            && (*position).x as i32 == (*(*events).bgEvents.at(i)).x as i32 + MAP_OFFSET
            && (*position).y as i32 == (*(*events).bgEvents.at(i)).y as i32 + MAP_OFFSET
        {
            sCurSecretBaseId = (*(*events).bgEvents.at(i)).bgUnion.secretBaseId as u8;
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpIntoSecretBase(position: *mut MapPosition, events: *mut MapEvents) {
    SetCurSecretBaseIdFromPosition(position, events);
    TrySetCurSecretBaseIndex();
    ScriptContext_SetupScript(SecretBase_EventScript_Enter.as_ptr().cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetCurSecretBase() -> u8 {
    SetCurSecretBaseId();
    TrySetCurSecretBaseIndex();
    if gSpecialVar_Result == TRUE as u16 {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn Task_WarpOutOfSecretBase(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            LockPlayerFieldControls();
            gTasks[taskId].data[0] = 1;
        }
        1 => {
            if gPaletteFade.active() == 0 {
                gTasks[taskId].data[0] = 2;
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
pub(crate) unsafe extern "C" fn WarpOutOfSecretBase() {
    CreateTask(Some(Task_WarpOutOfSecretBase), 0);
    FadeScreen(FADE_TO_BLACK, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCurSecretBaseOwnedByAnotherPlayer() {
    if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != sCurSecretBaseId {
        gSpecialVar_Result = TRUE as u16;
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
pub(crate) unsafe extern "C" fn GetSecretBaseName(dest: *mut u8, secretBaseIdx: u8) -> *mut u8 {
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
    return StringAppend(dest, gText_ApostropheSBase.as_ptr().cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseMapName(dest: *mut u8) -> *mut u8 {
    return GetSecretBaseName(dest, VarGet(VAR_CURRENT_SECRET_BASE) as u8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyCurSecretBaseOwnerName_StrVar1() {
    let mut secretBaseIdx: u8 = 0;
    let mut name: *mut u8 = null_mut();
    secretBaseIdx = VarGet(VAR_CURRENT_SECRET_BASE) as u8;
    name = (*gSaveBlock1Ptr).secretBases[secretBaseIdx]
        .trainerName
        .as_mut_ptr();
    *StringCopyN(gStringVar1.as_mut_ptr(), name, GetNameLength(name)) = EOS;
    ConvertInternationalString(
        gStringVar1.as_mut_ptr(),
        (*gSaveBlock1Ptr).secretBases[secretBaseIdx].language,
    );
}
pub(crate) unsafe extern "C" fn IsSecretBaseRegistered(secretBaseIdx: u8) -> u8 {
    if (*gSaveBlock1Ptr).secretBases[secretBaseIdx].registryStatus() != 0 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetAverageEVs(pokemon: *mut Pokemon) -> u8 {
    let mut evTotal: u16 = 0;
    evTotal = GetMonData2(pokemon, MON_DATA_HP_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_ATK_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_DEF_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_SPEED_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_SPATK_EV) as u16;
    evTotal += GetMonData2(pokemon, MON_DATA_SPDEF_EV) as u16;
    return (evTotal as i32 / 6) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerSecretBaseParty() {
    let mut i: u16 = 0;
    let mut moveIndex: u16 = 0;
    let mut partyId: u16 = 0;
    let mut party: *mut SecretBaseParty = null_mut();
    partyId = 0;
    party = &raw mut (*gSaveBlock1Ptr).secretBases[0].party;
    if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != 0 {
        i = 0;
        while i < PARTY_SIZE as u16 {
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
                moveIndex = 0;
                while moveIndex < MAX_MON_MOVES as u16 {
                    (*party).moves[partyId as i32 * MAX_MON_MOVES + moveIndex as i32] =
                        GetMonData2(&raw mut gPlayerParty[i], MON_DATA_MOVE1 + moveIndex as i32)
                            as u16;
                    moveIndex += 1;
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
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearAndLeaveSecretBase() {
    let mut temp: u16 = (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived;
    ClearSecretBase(&raw mut (*gSaveBlock1Ptr).secretBases[0]);
    (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived = temp;
    WarpOutOfSecretBase();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveOutOfSecretBase() {
    IncrementGameStat(GAME_STAT_MOVED_SECRET_BASE);
    ClearAndLeaveSecretBase();
}
pub(crate) unsafe extern "C" fn ClosePlayerSecretBaseEntrance() {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut metatileId: i16 = 0;
    let mut events: *mut MapEvents = gMapHeader.events;
    i = 0;
    while i < (*events).bgEventCount as u16 {
        if (*(*events).bgEvents.at(i)).kind == BG_EVENT_SECRET_BASE
            && (*gSaveBlock1Ptr).secretBases[0].secretBaseId as u32
                == (*(*events).bgEvents.at(i)).bgUnion.secretBaseId
        {
            metatileId = MapGridGetMetatileIdAt(
                (*(*events).bgEvents.at(i)).x as i32 + MAP_OFFSET,
                (*(*events).bgEvents.at(i)).y as i32 + MAP_OFFSET,
            ) as i16;
            j = 0;
            while j < 7 {
                if sSecretBaseEntranceMetatiles[j].openMetatileId as i32 == metatileId as i32 {
                    MapGridSetMetatileIdAt(
                        (*(*events).bgEvents.at(i)).x as i32 + MAP_OFFSET,
                        (*(*events).bgEvents.at(i)).y as i32 + MAP_OFFSET,
                        sSecretBaseEntranceMetatiles[j].closedMetatileId | MAPGRID_IMPASSABLE,
                    );
                    break;
                }
                j += 1;
            }
            DrawWholeMapView();
            break;
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveOutOfSecretBaseFromOutside() {
    let mut temp: u16 = 0;
    ClosePlayerSecretBaseEntrance();
    IncrementGameStat(GAME_STAT_MOVED_SECRET_BASE);
    temp = (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived;
    ClearSecretBase(&raw mut (*gSaveBlock1Ptr).secretBases[0]);
    (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived = temp;
}
pub(crate) unsafe extern "C" fn GetNumRegisteredSecretBases() -> u8 {
    let mut i: i16 = 0;
    let mut count: u8 = 0;
    i = 1;
    while i < SECRET_BASES_COUNT as i16 {
        if IsSecretBaseRegistered(i as u8) == TRUE {
            count += 1;
        }
        i += 1;
    }
    return count;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurSecretBaseRegistrationValidity() {
    if IsSecretBaseRegistered(VarGet(VAR_CURRENT_SECRET_BASE) as u8) == TRUE {
        gSpecialVar_Result = 1;
    } else if GetNumRegisteredSecretBases() >= 10 {
        gSpecialVar_Result = 2;
    } else {
        gSpecialVar_Result = 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ToggleCurSecretBaseRegistry() {
    (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)].set_registryStatus(
        (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)].registryStatus() ^ 1,
    );
    FlagSet(FLAG_SECRET_BASE_REGISTRY_ENABLED);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSecretBaseDecorationMenu() {
    CreateTask(Some(DoSecretBaseDecorationMenu), 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSecretBaseRegistryMenu() {
    CreateTask(Some(Task_ShowSecretBaseRegistryMenu), 0);
}
pub(crate) unsafe extern "C" fn Task_ShowSecretBaseRegistryMenu(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
        gTasks[taskId].func = Some(HandleRegistryMenuInput);
    } else {
        DisplayItemMessageOnField(
            taskId,
            gText_NoRegistry.as_ptr().cast_mut(),
            Some(GoToSecretBasePCRegisterMenu),
        );
    }
}
pub(crate) unsafe extern "C" fn BuildRegistryMenuItems(taskId: u8) {
    let mut data: *mut i16 = null_mut();
    let mut i: u8 = 0;
    let mut count: u8 = 0;
    data = gTasks[taskId].data.as_mut_ptr();
    count = 0;
    i = 1;
    while i < SECRET_BASES_COUNT {
        if IsSecretBaseRegistered(i) != 0 {
            GetSecretBaseName((*sRegistryMenu).names[count].as_mut_ptr(), i);
            (*sRegistryMenu).items[count].name = (*sRegistryMenu).names[count].as_mut_ptr();
            (*sRegistryMenu).items[count].id = i as i32;
            count += 1;
        }
        i += 1;
    }
    (*sRegistryMenu).items[count].name = gText_Cancel.as_ptr().cast_mut();
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
pub(crate) unsafe extern "C" fn RegistryMenu_OnCursorMove(
    unused: i32,
    flag: u8,
    menu: *mut ListMenu,
) {
    if flag != TRUE {
        PlaySE(SE_SELECT);
    }
}
pub(crate) unsafe extern "C" fn FinalizeRegistryMenu(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    SetStandardWindowBorderStyle(*data.at(6) as u8, FALSE);
    *data.at(5) = ListMenuInit(
        &raw mut gMultiuseListMenuTemplate,
        *data.at(2) as u16,
        *data.at(1) as u16,
    ) as i16;
    AddRegistryMenuScrollArrows(taskId);
    ScheduleBgCopyTilemapToVram(0);
}
pub(crate) unsafe extern "C" fn AddRegistryMenuScrollArrows(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn HandleRegistryMenuInput(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut input: i32 = ListMenu_ProcessInput(*data.at(5) as u8);
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
pub(crate) unsafe extern "C" fn ShowRegistryMenuActions(taskId: u8) {
    let mut template: WindowTemplate = zeroed();
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    RemoveScrollIndicatorArrowPair(*data.at(8) as u8);
    template = sRegistryWindowTemplates[1];
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
    gTasks[taskId].func = Some(HandleRegistryMenuActionsInput);
}
pub(crate) unsafe extern "C" fn HandleRegistryMenuActionsInput(taskId: u8) {
    let mut input: i8 = Menu_ProcessInputNoWrap();
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
pub(crate) unsafe extern "C" fn ShowRegistryMenuDeleteConfirmation(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    ClearStdWindowAndFrame(*data.at(6) as u8, FALSE);
    ClearStdWindowAndFrame(*data.at(7) as u8, FALSE);
    ClearWindowTilemap(*data.at(6) as u8);
    ClearWindowTilemap(*data.at(7) as u8);
    RemoveWindow(*data.at(7) as u8);
    ScheduleBgCopyTilemapToVram(0);
    GetSecretBaseName(gStringVar1.as_mut_ptr(), *data.at(4) as u8);
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_OkayToDeleteFromRegistry.as_ptr().cast_mut(),
    );
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(ShowRegistryMenuDeleteYesNo),
    );
}
pub(crate) unsafe extern "C" fn ShowRegistryMenuDeleteYesNo(taskId: u8) {
    DisplayYesNoMenuDefaultYes();
    DoYesNoFuncWithChoice(taskId, (&raw const *sDeleteRegistryYesNoFuncs).cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeleteRegistry_Yes_Callback(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
    gTasks[taskId].func = Some(HandleRegistryMenuInput);
}
pub(crate) unsafe extern "C" fn DeleteRegistry_Yes(taskId: u8) {
    DisplayItemMessageOnField(
        taskId,
        gText_RegisteredDataDeleted.as_ptr().cast_mut(),
        Some(DeleteRegistry_Yes_Callback),
    );
}
pub(crate) unsafe extern "C" fn DeleteRegistry_No(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    ClearDialogWindowAndFrame(0, 0);
    DestroyListMenuTask(
        *data.at(5) as u8,
        data.at(2) as *mut u16,
        data.at(1) as *mut u16,
    );
    FinalizeRegistryMenu(taskId);
    gTasks[taskId].func = Some(HandleRegistryMenuInput);
}
pub(crate) unsafe extern "C" fn ReturnToMainRegistryMenu(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    AddRegistryMenuScrollArrows(taskId);
    ClearStdWindowAndFrame(*data.at(7) as u8, FALSE);
    ClearWindowTilemap(*data.at(7) as u8);
    RemoveWindow(*data.at(7) as u8);
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].func = Some(HandleRegistryMenuInput);
}
pub(crate) unsafe extern "C" fn GoToSecretBasePCRegisterMenu(taskId: u8) {
    if VarGet(VAR_CURRENT_SECRET_BASE) == 0 {
        ScriptContext_SetupScript(SecretBase_EventScript_PCCancel.as_ptr().cast_mut());
    } else {
        ScriptContext_SetupScript(SecretBase_EventScript_ShowRegisterMenu.as_ptr().cast_mut());
    }
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn GetSecretBaseOwnerType(secretBaseIdx: u8) -> u8 {
    return ((*gSaveBlock1Ptr).secretBases[secretBaseIdx].trainerId[0] as i32 % 5) as u8
        + (*gSaveBlock1Ptr).secretBases[secretBaseIdx].gender() * 5;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTrainerLoseText() -> *mut u8 {
    let mut ownerType: u8 = GetSecretBaseOwnerType(VarGet(VAR_CURRENT_SECRET_BASE) as u8);
    if ownerType == 0 {
        return SecretBase_Text_Trainer0Defeated.as_ptr().cast_mut();
    } else if ownerType == 1 {
        return SecretBase_Text_Trainer1Defeated.as_ptr().cast_mut();
    } else if ownerType == 2 {
        return SecretBase_Text_Trainer2Defeated.as_ptr().cast_mut();
    } else if ownerType == 3 {
        return SecretBase_Text_Trainer3Defeated.as_ptr().cast_mut();
    } else if ownerType == 4 {
        return SecretBase_Text_Trainer4Defeated.as_ptr().cast_mut();
    } else if ownerType == 5 {
        return SecretBase_Text_Trainer5Defeated.as_ptr().cast_mut();
    } else if ownerType == 6 {
        return SecretBase_Text_Trainer6Defeated.as_ptr().cast_mut();
    } else if ownerType == 7 {
        return SecretBase_Text_Trainer7Defeated.as_ptr().cast_mut();
    } else if ownerType == 8 {
        return SecretBase_Text_Trainer8Defeated.as_ptr().cast_mut();
    } else {
        return SecretBase_Text_Trainer9Defeated.as_ptr().cast_mut();
    }
    #[allow(unreachable_code)]
    {
        return null_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrepSecretBaseBattleFlags() {
    TryGainNewFanFromCounter(FANCOUNTER_BATTLED_AT_BASE);
    gTrainerBattleOpponent_A = TRAINER_SECRET_BASE;
    gBattleTypeFlags = 0x8000008;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattledOwnerFromResult() {
    (*gSaveBlock1Ptr).secretBases[VarGet(VAR_CURRENT_SECRET_BASE)]
        .set_battledOwnerToday(gSpecialVar_Result as u8);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseOwnerAndState() {
    let mut secretBaseIdx: u16 = 0;
    let mut i: u8 = 0;
    secretBaseIdx = VarGet(VAR_CURRENT_SECRET_BASE);
    if FlagGet(2338) == 0 {
        i = 0;
        while i < SECRET_BASES_COUNT {
            (*gSaveBlock1Ptr).secretBases[i].set_battledOwnerToday(FALSE);
            i += 1;
        }
        FlagSet(2338);
    }
    gSpecialVar_0x8004 = GetSecretBaseOwnerType(secretBaseIdx as u8) as u16;
    gSpecialVar_Result = (*gSaveBlock1Ptr).secretBases[secretBaseIdx].battledOwnerToday() as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SecretBasePerStepCallback(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut behavior: u8 = 0;
    let mut tileId: u16 = 0;
    let mut data: *mut i16 = null_mut();
    data = gTasks[taskId].data.as_mut_ptr();
    match *data.at(1) {
        0 => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                sInFriendSecretBase = TRUE;
            } else {
                sInFriendSecretBase = FALSE;
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
                if sInFriendSecretBase == TRUE {
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
                if sInFriendSecretBase == TRUE {
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
                if sInFriendSecretBase == TRUE {
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
                if sInFriendSecretBase == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_STAND,
                    );
                }
            } else if behavior == MB_IMPASSABLE_WEST_AND_EAST
                && tileId == METATILE_SecretBase_Slide_StairLanding
            {
                if sInFriendSecretBase == TRUE {
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
                if sInFriendSecretBase == TRUE {
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
                if sInFriendSecretBase == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_GLITTER_MAT,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseBalloon(behavior) == TRUE {
                PopSecretBaseBalloon(MapGridGetMetatileIdAt(x as i32, y as i32) as i16, x, y);
                if sInFriendSecretBase == TRUE {
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
                if sInFriendSecretBase == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_BREAKABLE_DOOR,
                    );
                }
                ShatterSecretBaseBreakableDoor(x, y);
            } else if MetatileBehavior_IsSecretBaseSoundMat(behavior) == TRUE {
                if sInFriendSecretBase == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_LOW_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_NOTE_MAT,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseJumpMat(behavior) == TRUE {
                if sInFriendSecretBase == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_JUMP_MAT,
                    );
                }
            } else if MetatileBehavior_IsSecretBaseSpinMat(behavior) == TRUE {
                if sInFriendSecretBase == TRUE {
                    VarSet(
                        VAR_SECRET_BASE_HIGH_TV_FLAGS,
                        VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_SPIN_MAT,
                    );
                }
            }
        }
        2 => {
            if FieldEffectActiveListContains(*data.at(4) as u8) == 0 {
                *data.at(1) = 1;
            }
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SaveSecretBase(
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
pub(crate) unsafe extern "C" fn SecretBasesHaveSameTrainerId(
    secretBase1: *mut SecretBase,
    secretBase2: *mut SecretBase,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < TRAINER_ID_LENGTH {
        if (*secretBase1).trainerId[i] != (*secretBase2).trainerId[i] {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SecretBasesHaveSameTrainerName(
    sbr1: *mut SecretBase,
    sbr2: *mut SecretBase,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < PLAYER_NAME_LENGTH as u8
        && ((*sbr1).trainerName[i] != EOS || (*sbr2).trainerName[i] != EOS)
    {
        if (*sbr1).trainerName[i] != (*sbr2).trainerName[i] {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn SecretBasesBelongToSamePlayer(
    secretBase1: *mut SecretBase,
    secretBase2: *mut SecretBase,
) -> u8 {
    if (*secretBase1).gender() == (*secretBase2).gender()
        && SecretBasesHaveSameTrainerId(secretBase1, secretBase2) != 0
        && SecretBasesHaveSameTrainerName(secretBase1, secretBase2) != 0
    {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetSecretBaseIndexFromId(secretBaseId: u8) -> i16 {
    let mut i: i16 = 0;
    i = 0;
    while i < SECRET_BASES_COUNT as i16 {
        if (*gSaveBlock1Ptr).secretBases[i].secretBaseId == secretBaseId {
            return i;
        }
        i += 1;
    }
    return -1;
}
pub(crate) unsafe extern "C" fn FindAvailableSecretBaseIndex() -> u8 {
    let mut i: i16 = 0;
    i = 1;
    while i < SECRET_BASES_COUNT as i16 {
        if (*gSaveBlock1Ptr).secretBases[i].secretBaseId == 0 {
            return i as u8;
        }
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn FindUnregisteredSecretBaseIndex() -> u8 {
    let mut i: i16 = 0;
    i = 1;
    while i < SECRET_BASES_COUNT as i16 {
        if (*gSaveBlock1Ptr).secretBases[i].registryStatus() == UNREGISTERED
            && (*gSaveBlock1Ptr).secretBases[i].toRegister() == FALSE
        {
            return i as u8;
        }
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn TrySaveFriendsSecretBase(
    secretBase: *mut SecretBase,
    version: u32,
    language: u32,
) -> u8 {
    let mut index: i16 = 0;
    if (*secretBase).secretBaseId == 0 {
        return 0;
    }
    index = GetSecretBaseIndexFromId((*secretBase).secretBaseId);
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
    return 0;
}
pub(crate) unsafe extern "C" fn SortSecretBasesByRegistryStatus() {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut secretBases: *mut SecretBase = null_mut();
    secretBases = (*gSaveBlock1Ptr).secretBases.as_mut_ptr();
    i = 1;
    while i < 19 {
        j = i + 1;
        while j < SECRET_BASES_COUNT {
            if (*secretBases.at(i)).registryStatus() == UNREGISTERED
                && (*secretBases.at(j)).registryStatus() == REGISTERED
                || (*secretBases.at(i)).registryStatus() == NEW
                    && (*secretBases.at(j)).registryStatus() != NEW
            {
                let mut temp: SecretBase = zeroed();
                temp = *secretBases.at(i);
                *secretBases.at(i) = *secretBases.at(j);
                *secretBases.at(j) = temp;
            }
            j += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TrySaveFriendsSecretBases(
    mixer: *mut SecretBaseRecordMixer,
    registryStatus: u8,
) {
    let mut i: u16 = 0;
    i = 1;
    while i < SECRET_BASES_COUNT as u16 {
        if (*(*mixer).secretBases.at(i)).registryStatus() == registryStatus {
            TrySaveFriendsSecretBase(
                (*mixer).secretBases.at(i),
                (*mixer).version,
                (*mixer).language,
            );
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SecretBaseBelongsToPlayer(secretBase: *mut SecretBase) -> u8 {
    let mut i: u8 = 0;
    if (*secretBase).secretBaseId == 0 {
        return FALSE;
    }
    if (*secretBase).secretBaseId != 0 && (*secretBase).gender() != (*gSaveBlock2Ptr).playerGender {
        return FALSE;
    }
    i = 0;
    while i < TRAINER_ID_LENGTH {
        if (*secretBase).trainerId[i] != (*gSaveBlock2Ptr).playerTrainerId[i] {
            return FALSE;
        }
        i += 1;
    }
    i = 0;
    while i < PLAYER_NAME_LENGTH as u8
        && ((*secretBase).trainerName[i] != EOS || (*gSaveBlock2Ptr).playerName[i] != EOS)
    {
        if (*secretBase).trainerName[i] != (*gSaveBlock2Ptr).playerName[i] {
            return FALSE;
        }
        i += 1;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn DeleteFirstOldBaseFromPlayerInRecordMixingFriendsRecords(
    mut basesA: *mut SecretBase,
    mut basesB: *mut SecretBase,
    mut basesC: *mut SecretBase,
) {
    let mut i: u8 = 0;
    let mut sbFlags: u8 = 0;
    i = 0;
    while i < SECRET_BASES_COUNT {
        if sbFlags as i32 & DELETED_BASE_A == 0 {
            if SecretBaseBelongsToPlayer(basesA.at(i)) == TRUE {
                ClearSecretBase(basesA.at(i));
                sbFlags |= DELETED_BASE_A as u8;
            }
        }
        if sbFlags as i32 & DELETED_BASE_B == 0 {
            if SecretBaseBelongsToPlayer(basesB.at(i)) == TRUE {
                ClearSecretBase(basesB.at(i));
                sbFlags |= DELETED_BASE_B as u8;
            }
        }
        if sbFlags as i32 & DELETED_BASE_C == 0 {
            if SecretBaseBelongsToPlayer(basesC.at(i)) == TRUE {
                ClearSecretBase(basesC.at(i));
                sbFlags |= DELETED_BASE_C as u8;
            }
        }
        if sbFlags == 7 {
            break;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn ClearDuplicateOwnedSecretBase(
    secretBase: *mut SecretBase,
    mut secretBases: *mut SecretBase,
    idx: u8,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
    while i < SECRET_BASES_COUNT {
        if (*secretBases.at(i)).secretBaseId != 0 {
            if SecretBasesBelongToSamePlayer(secretBase, secretBases.at(i)) == TRUE {
                if idx == 0 {
                    ClearSecretBase(secretBases.at(i));
                    return FALSE;
                }
                if (*secretBase).numSecretBasesReceived
                    > (*secretBases.at(i)).numSecretBasesReceived
                {
                    ClearSecretBase(secretBases.at(i));
                    return FALSE;
                }
                (*secretBases.at(i)).set_toRegister((*secretBase).toRegister());
                ClearSecretBase(secretBase);
                return TRUE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClearDuplicateOwnedSecretBases(
    mut playersBases: *mut SecretBase,
    mut friendsBasesA: *mut SecretBase,
    mut friendsBasesB: *mut SecretBase,
    mut friendsBasesC: *mut SecretBase,
) {
    let mut i: u8 = 0;
    i = 1;
    while i < SECRET_BASES_COUNT {
        if (*playersBases.at(i)).secretBaseId != 0 {
            if (*playersBases.at(i)).registryStatus() == REGISTERED {
                (*playersBases.at(i)).set_toRegister(TRUE);
            }
            if ClearDuplicateOwnedSecretBase(playersBases.at(i), friendsBasesA, i) == 0 {
                if ClearDuplicateOwnedSecretBase(playersBases.at(i), friendsBasesB, i) == 0 {
                    ClearDuplicateOwnedSecretBase(playersBases.at(i), friendsBasesC, i);
                }
            }
        }
        i += 1;
    }
    i = 0;
    while i < SECRET_BASES_COUNT {
        if (*friendsBasesA.at(i)).secretBaseId != 0 {
            (*friendsBasesA.at(i)).set_battledOwnerToday(0);
            if ClearDuplicateOwnedSecretBase(friendsBasesA.at(i), friendsBasesB, i) == 0 {
                ClearDuplicateOwnedSecretBase(friendsBasesA.at(i), friendsBasesC, i);
            }
        }
        i += 1;
    }
    i = 0;
    while i < SECRET_BASES_COUNT {
        if (*friendsBasesB.at(i)).secretBaseId != 0 {
            (*friendsBasesB.at(i)).set_battledOwnerToday(0);
            ClearDuplicateOwnedSecretBase(friendsBasesB.at(i), friendsBasesC, i);
        }
        if (*friendsBasesC.at(i)).secretBaseId != 0 {
            (*friendsBasesC.at(i)).set_battledOwnerToday(0);
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn TrySaveRegisteredDuplicate(
    base: *mut SecretBase,
    version: u32,
    language: u32,
) {
    if (*base).toRegister() == TRUE {
        TrySaveFriendsSecretBase(base, version, language);
        ClearSecretBase(base);
    }
}
pub(crate) unsafe extern "C" fn TrySaveRegisteredDuplicates(
    mut mixers: *mut SecretBaseRecordMixer,
) {
    let mut i: u16 = 0;
    i = 0;
    while i < SECRET_BASES_COUNT as u16 {
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
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SaveRecordMixBases(mut mixers: *mut SecretBaseRecordMixer) {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReceiveSecretBasesData(
    secretBases: *mut c_void,
    recordSize: u32,
    linkIdx: u8,
) {
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
                    (secretBases as *mut u8).at(1 * recordSize) as *mut c_void as *mut SecretBase;
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
                    (secretBases as *mut u8).at(1 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[2].version = gLinkPlayers[1].version as u32 & 0xFF;
                mixers[2].language = gLinkPlayers[1].language as u32;
            }
            3 => {
                mixers[0].secretBases =
                    (secretBases as *mut u8).at(0 * recordSize) as *mut c_void as *mut SecretBase;
                mixers[0].version = gLinkPlayers[0].version as u32 & 0xFF;
                mixers[0].language = gLinkPlayers[0].language as u32;
                mixers[1].secretBases =
                    (secretBases as *mut u8).at(1 * recordSize) as *mut c_void as *mut SecretBase;
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
        i = 1;
        while i < SECRET_BASES_COUNT as u16 {
            if (*gSaveBlock1Ptr).secretBases[i].registryStatus() == NEW {
                (*gSaveBlock1Ptr).secretBases[i].set_registryStatus(UNREGISTERED);
            }
            i += 1;
        }
        if (*gSaveBlock1Ptr).secretBases[0].secretBaseId != 0
            && (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived != 0xFFFF
        {
            (*gSaveBlock1Ptr).secretBases[0].numSecretBasesReceived += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearJapaneseSecretBases(mut bases: *mut SecretBase) {
    let mut i: u32 = 0;
    i = 0;
    while i < SECRET_BASES_COUNT as u32 {
        if (*bases.at(i)).language == LANGUAGE_JAPANESE {
            ClearSecretBase(bases.at(i));
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSecretBaseVars() {
    VarSet(VAR_SECRET_BASE_STEP_COUNTER, 0);
    VarSet(VAR_SECRET_BASE_LAST_ITEM_USED, 0);
    VarSet(VAR_SECRET_BASE_LOW_TV_FLAGS, 0);
    VarSet(VAR_SECRET_BASE_HIGH_TV_FLAGS, 0);
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, TRUE as u16);
    } else {
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, FALSE as u16);
    }
    sInFriendSecretBase = FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeftFriendsSecretBase() {
    if VarGet(VAR_SECRET_BASE_IS_NOT_LOCAL) != 0
        && sInFriendSecretBase == TRUE
        && CurMapIsSecretBase() == 0
    {
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, FALSE as u16);
        sInFriendSecretBase = FALSE;
        TryPutSecretBaseSecretsOnAir();
        VarSet(VAR_SECRET_BASE_STEP_COUNTER, 0);
        VarSet(VAR_SECRET_BASE_LAST_ITEM_USED, 0);
        VarSet(VAR_SECRET_BASE_LOW_TV_FLAGS, 0);
        VarSet(VAR_SECRET_BASE_HIGH_TV_FLAGS, 0);
        VarSet(VAR_SECRET_BASE_IS_NOT_LOCAL, FALSE as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsDollDecor() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_HIGH_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DOLL,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsCushionDecor() {
    if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
        VarSet(
            VAR_SECRET_BASE_LOW_TV_FLAGS,
            VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_CUSHION,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeclinedSecretBaseBattle() {
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
pub unsafe extern "C" fn WonSecretBaseBattle() {
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
pub unsafe extern "C" fn LostSecretBaseBattle() {
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
pub unsafe extern "C" fn DrewSecretBaseBattle() {
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
pub unsafe extern "C" fn CheckInteractedWithFriendsPosterDecor() {
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
        | METATILE_SecretBase_CutePoster => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_LOW_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_LOW_TV_FLAGS) | SECRET_BASE_USED_POSTER,
                );
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsFurnitureBottom() {
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
        | METATILE_SecretBase_PrettyDesk_BottomRight => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DESK,
                );
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsFurnitureMiddle() {
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
        | METATILE_SecretBase_PrettyDesk_Center => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_DESK,
                );
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsFurnitureTop() {
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
        | METATILE_SecretBase_BlueBrick_Top => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_BRICK,
                );
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsSandOrnament() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    match MapGridGetMetatileIdAt(x as i32, y as i32) {
        653 | METATILE_SecretBase_SandOrnament_Base2 => {
            if VarGet(VAR_CURRENT_SECRET_BASE) != 0 {
                VarSet(
                    VAR_SECRET_BASE_HIGH_TV_FLAGS,
                    VarGet(VAR_SECRET_BASE_HIGH_TV_FLAGS) | SECRET_BASE_USED_SAND_ORNAMENT,
                );
            }
        }
        _ => {}
    }
}
