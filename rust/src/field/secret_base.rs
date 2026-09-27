//! Translated from `src/secret_base.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sSecretBaseEntranceMetatiles sSecretBaseEntrancePositions sRegistryMenuActions sDeleteRegistryYesNoFuncs sSecretBaseOwnerGfxIds sRegistryWindowTemplates sRegistryListMenuTemplate
#[allow(unused_imports)]
use crate::data::secret_base::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sCurSecretBaseId: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sInFriendSecretBase: u8 = 0u8;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRegistryMenu: *mut u8 = core::ptr::null_mut();

unsafe extern "C" {
    static mut SecretBase_EventScript_Enter: u8;
    static mut SecretBase_EventScript_PCCancel: u8;
    static mut SecretBase_EventScript_ShowRegisterMenu: u8;
    static mut SecretBase_Text_Trainer0Defeated: u8;
    static mut SecretBase_Text_Trainer1Defeated: u8;
    static mut SecretBase_Text_Trainer2Defeated: u8;
    static mut SecretBase_Text_Trainer3Defeated: u8;
    static mut SecretBase_Text_Trainer4Defeated: u8;
    static mut SecretBase_Text_Trainer5Defeated: u8;
    static mut SecretBase_Text_Trainer6Defeated: u8;
    static mut SecretBase_Text_Trainer7Defeated: u8;
    static mut SecretBase_Text_Trainer8Defeated: u8;
    static mut SecretBase_Text_Trainer9Defeated: u8;
    static mut gBattleTypeFlags: u8;
    static mut gDecorations: u8;
    static mut gFieldCallback: u8;
    static mut gLinkPlayers: u8;
    static mut gMapHeader: u8;
    static mut gMultiuseListMenuTemplate: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerAvatar: u8;
    static mut gPlayerParty: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSaveBlock2Ptr: u8;
    static mut gSpecialVar_0x8004: u8;
    static mut gSpecialVar_0x8006: u8;
    static mut gSpecialVar_0x8007: u8;
    static mut gSpecialVar_Result: u8;
    static mut gStringVar1: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_ApostropheSBase: u8;
    static mut gText_Cancel: u8;
    static mut gText_NoRegistry: u8;
    static mut gText_OkayToDeleteFromRegistry: u8;
    static mut gText_RegisteredDataDeleted: u8;
    static mut gTrainerBattleOpponent_A: u8;
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
    fn AddWindow(a0: *mut u8) -> u16;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn CB2_LoadMap();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn ClearStdWindowAndFrame(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn CpuFastSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentMapDrawMetatileAt(a0: i32, a1: i32);
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroyTask(a0: u8);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn DisplayYesNoMenuDefaultYes();
    fn DoSecretBaseDecorationMenu(a0: u8);
    fn DoYesNoFuncWithChoice(a0: u8, a1: *mut u8);
    fn DrawWholeMapView();
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ContinueScriptHandleMusic();
    fn FieldCB_DefaultWarpExit();
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut u8);
    fn GetLinkPlayerCount() -> u8;
    fn GetMaxWidthInMenuTable(a0: *mut u8, a1: i32) -> i32;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn HideMapNamePopUpWindow();
    fn IncrementGameStat(a0: u8);
    fn InitMenuInUpperLeftCornerNormal(a0: u8, a1: u8, a2: u8) -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut u8, a1: u16, a2: u16) -> u8;
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
    fn ObjectEventTurn(a0: *mut u8, a1: u8);
    fn OverrideSecretBaseDecorationSpriteScript(a0: u8, a1: u8, a2: u8, a3: u8);
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PopSecretBaseBalloon(a0: i16, a1: i16, a2: i16);
    fn PrintMenuTable(a0: u8, a1: u8, a2: *mut u8);
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

pub(crate) unsafe extern "C" fn ClearSecretBase(secretBase: *mut u8) {
    unsafe {
        let mut secretBase = secretBase;
        let mut i: u16 = 0u16;
        {
            let mut tmp: u32 = 0u32;
            (&raw mut tmp).write_volatile(0u32);
            'l1: loop {
                'l2: {
                    CpuFastSet(
                        (&raw mut tmp).cast::<u8>(),
                        secretBase,
                        (16777216u32
                            | (crate::c::div_u32(
                                160u32,
                                ((crate::c::div_i32(32i32, 8i32)) as u32),
                            ) & 2097151u32)),
                    );
                }
                if !((0i32) != 0) {
                    break 'l1;
                }
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l3;
                }
                'l4: {
                    ((((secretBase).wrapping_add(2)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(255u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearSecretBases() {
    unsafe {
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    ClearSecretBase(
                        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(6812))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 160),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetCurSecretBaseId() {
    unsafe {
        ((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>())
            .write(((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetCurSecretBaseIndex() {
    unsafe {
        let mut i: u16 = 0u16;
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>()).read()) as i32)
                        == ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(6812))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 160))
                        .read()) as i32)
                    {
                        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                        VarSet(16468u16, i);
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckPlayerHasSecretBase() {
    unsafe {
        if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .read())
            != 0
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetSecretBaseTypeInFrontOfPlayer_() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut behavior: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        behavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32)) & 4095i32) as i16);
        if (((behavior) as i32) == 144i32) || (((behavior) as i32) == 145i32) {
            return 1u8;
        }
        if (((behavior) as i32) == 146i32) || (((behavior) as i32) == 147i32) {
            return 2u8;
        }
        if (((behavior) as i32) == 154i32) || (((behavior) as i32) == 155i32) {
            return 3u8;
        }
        if (((behavior) as i32) == 148i32) || (((behavior) as i32) == 149i32) {
            return 4u8;
        }
        if (((((behavior) as i32) == 150i32) || (((behavior) as i32) == 151i32))
            || (((behavior) as i32) == 156i32))
            || (((behavior) as i32) == 157i32)
        {
            return 5u8;
        }
        if (((behavior) as i32) == 152i32) || (((behavior) as i32) == 153i32) {
            return 6u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTypeInFrontOfPlayer() {
    unsafe {
        ((&raw mut gSpecialVar_0x8007).cast::<u16>())
            .write(((GetSecretBaseTypeInFrontOfPlayer_()) as u16));
    }
}
pub(crate) unsafe extern "C" fn FindMetatileIdMapCoords(x: *mut i16, y: *mut i16, metatileId: u16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatileId = metatileId;
        let mut i: i16 = 0i16;
        let mut j: i16 = 0i16;
        let mut mapLayout: *mut u8 =
            (((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read();
        {
            j = 0i16;
            'l1: loop {
                if !(((j) as i32) < ((mapLayout).wrapping_add(4).cast::<i32>()).read()) {
                    break 'l1;
                }
                'l2: {
                    {
                        i = 0i16;
                        'l3: loop {
                            if !(((i) as i32) < ((mapLayout).cast::<i32>()).read()) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((((mapLayout).wrapping_add(12).cast::<*mut u16>()).read())
                                    .wrapping_offset(
                                        ((((j) as i32)
                                            .wrapping_mul(((mapLayout).cast::<i32>()).read()))
                                        .wrapping_add(((i) as i32)))
                                            as isize,
                                    ))
                                .read()) as i32)
                                    & 1023i32)
                                    == ((metatileId) as i32)
                                {
                                    (x).write(i);
                                    (y).write(j);
                                    return;
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ToggleSecretBaseEntranceMetatile() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut metatileId: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        metatileId = ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as i16);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 4u32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sSecretBaseEntranceMetatiles)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .cast::<u16>())
                    .read()) as i32)
                        == ((metatileId) as i32)
                    {
                        MapGridSetMetatileIdAt(
                            ((x) as i32),
                            ((y) as i32),
                            (((((((((&raw const sSecretBaseEntranceMetatiles)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read()) as i32)
                                | 3072i32) as u16),
                        );
                        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u16;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(28u32, 4u32)) {
                    break 'l3;
                }
                'l4: {
                    if (((((((&raw const sSecretBaseEntranceMetatiles)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 4))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read()) as i32)
                        == ((metatileId) as i32)
                    {
                        MapGridSetMetatileIdAt(
                            ((x) as i32),
                            ((y) as i32),
                            (((((((((&raw const sSecretBaseEntranceMetatiles)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<u16>())
                            .read()) as i32)
                                | 3072i32) as u16),
                        );
                        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNameLength(secretBaseOwnerName: *mut u8) -> u8 {
    unsafe {
        let mut secretBaseOwnerName = secretBaseOwnerName;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 7i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((secretBaseOwnerName).wrapping_offset(((i) as i32) as isize)).read())
                        as i32)
                        == 255i32
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 7u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerSecretBase() {
    unsafe {
        let mut i: u16 = 0u16;
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812)).cast::<u8>())
            .write(((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>()).read());
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_add(9))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        VarSet(16468u16, 0u16);
        StringCopyN(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_add(2))
            .cast::<u8>(),
            (((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>(),
            GetNameLength((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).cast::<u8>()),
        );
        crate::c::bf_write(
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_add(1),
            4,
            1,
            (((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8)).read())
                as i32,
        );
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .wrapping_add(13))
        .write(2u8);
        VarSet(
            16422u16,
            (((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(20)).read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetOccupiedSecretBaseEntranceMetatiles(events: *mut u8) {
    unsafe {
        let mut events = events;
        let mut bgId: u16 = 0u16;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        {
            bgId = 0u16;
            'l1: loop {
                if !(((bgId) as i32) < ((((events).wrapping_add(3)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                        .wrapping_offset(((bgId) as i32) as isize * 12))
                    .wrapping_add(5))
                    .read()) as i32)
                        == 8i32
                    {
                        {
                            j = 0u16;
                            'l3: loop {
                                if !(((j) as i32) < 20i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                        .read())
                                    .wrapping_add(6812))
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 160))
                                    .read()) as u32)
                                        == ((((((events).wrapping_add(16).cast::<*mut u8>())
                                            .read())
                                        .wrapping_offset(((bgId) as i32) as isize * 12))
                                        .wrapping_add(8))
                                        .cast::<u32>())
                                        .read()
                                    {
                                        let mut x: i16 =
                                            (((((((((events).wrapping_add(16).cast::<*mut u8>())
                                                .read())
                                            .wrapping_offset(((bgId) as i32) as isize * 12))
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(7i32))
                                                as i16);
                                        let mut y: i16 =
                                            (((((((((events).wrapping_add(16).cast::<*mut u8>())
                                                .read())
                                            .wrapping_offset(((bgId) as i32) as isize * 12))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(7i32))
                                                as i16);
                                        let mut tile_id: i16 =
                                            ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32)))
                                                as i16);
                                        {
                                            i = 0u16;
                                            'l5: loop {
                                                if !(((i) as u32) < crate::c::div_u32(28u32, 4u32))
                                                {
                                                    break 'l5;
                                                }
                                                'l6: {
                                                    if ((((((((&raw const sSecretBaseEntranceMetatiles).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((i) as i32)) as isize * 4)).cast::<u16>()).read()) as i32)) == (((tile_id) as i32)) {
MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), (((((((((((&raw const sSecretBaseEntranceMetatiles).cast::<u8>().cast_mut()).cast::<u8>()).wrapping_offset((((i) as i32)) as isize * 4)).wrapping_add(2).cast::<u16>()).read()) as i32)) | 3072i32)) as u16));
break 'l5;
}
                                                }
                                                i = (i).wrapping_add(1);
                                            }
                                        }
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                    }
                }
                bgId = (bgId).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetSecretBaseWarpDestination() {
    unsafe {
        let mut secretBaseGroup: i8 = (((crate::c::div_i32(
            ((((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>()).read()) as i32),
            10i32,
        ))
        .wrapping_mul(4i32)) as i8);
        SetWarpDestinationToMapWarp(
            25i8,
            ((((((&raw const sSecretBaseEntrancePositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((((secretBaseGroup) as i32).wrapping_add(0i32)) as isize))
            .read()) as i8),
            ((((((&raw const sSecretBaseEntrancePositions)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset((((secretBaseGroup) as i32).wrapping_add(1i32)) as isize))
            .read()) as i8),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_EnterSecretBase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut secretBaseIdx: u16 = 0u16;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                secretBaseIdx = VarGet(16468u16);
                if (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(6812))
                .cast::<u8>())
                .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
                .wrapping_add(16))
                .read()) as i32)
                    < 255i32
                {
                    let __p2 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
                    .wrapping_add(16);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                SetSecretBaseWarpDestination();
                WarpIntoMap();
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCB_ContinueScriptHandleMusic));
                SetMainCallback2(Some(CB2_LoadMap));
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnterSecretBase() {
    unsafe {
        CreateTask(Some(Task_EnterSecretBase), 0u8);
        FadeScreen(1u8, 0i8);
        SetDynamicWarp(
            0i32,
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4)).cast::<i8>())
                .read(),
            (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read(),
            (-1i8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SecretBaseMapPopupEnabled() -> u8 {
    unsafe {
        if ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32) == 9i32)
            && (((VarGet(16535u16)) as i32) == 0i32)
        {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn EnterNewlyCreatedSecretBase_WaitFadeIn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ObjectEventTurn(
            ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            2u8,
        );
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn EnterNewlyCreatedSecretBase_StartFadeIn() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        LockPlayerFieldControls();
        HideMapNamePopUpWindow();
        FindMetatileIdMapCoords(&raw mut x, &raw mut y, 544u16);
        x = ((((x) as i32).wrapping_add(7i32)) as i16);
        y = ((((y) as i32).wrapping_add(7i32)) as i16);
        MapGridSetMetatileIdAt(((x) as i32), ((y) as i32), 3616u16);
        CurrentMapDrawMetatileAt(((x) as i32), ((y) as i32));
        FadeInFromBlack();
        CreateTask(Some(EnterNewlyCreatedSecretBase_WaitFadeIn), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_EnterNewlyCreatedSecretBase(taskId: u8) {
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
            let mut secretBaseGroup: i8 = (((crate::c::div_i32(
                ((((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>()).read()) as i32),
                10i32,
            ))
            .wrapping_mul(4i32)) as i8);
            SetWarpDestination(
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read(),
                (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read(),
                (-1i8),
                ((((((&raw const sSecretBaseEntrancePositions)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((((secretBaseGroup) as i32).wrapping_add(2i32)) as isize))
                .read()) as i8),
                ((((((&raw const sSecretBaseEntrancePositions)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset((((secretBaseGroup) as i32).wrapping_add(3i32)) as isize))
                .read()) as i8),
            );
            WarpIntoMap();
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(EnterNewlyCreatedSecretBase_StartFadeIn));
            SetMainCallback2(Some(CB2_LoadMap));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EnterNewlyCreatedSecretBase() {
    unsafe {
        CreateTask(Some(Task_EnterNewlyCreatedSecretBase), 0u8);
        FadeScreen(1u8, 0i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CurMapIsSecretBase() -> u8 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 25i32)
            && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8) as i32)
                <= 23i32)
        {
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
pub unsafe extern "C" fn InitSecretBaseAppearance(hidePC: u8) {
    unsafe {
        let mut hidePC = hidePC;
        let mut secretBaseIdx: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut decorations: *mut u8 = core::ptr::null_mut();
        let mut decorPos: *mut u8 = core::ptr::null_mut();
        if (CurMapIsSecretBase()) != 0 {
            secretBaseIdx = VarGet(16468u16);
            decorations = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(6812))
            .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(18))
            .cast::<u8>();
            decorPos = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(6812))
            .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(34))
            .cast::<u8>();
            {
                x = 0u16;
                'l1: loop {
                    if !(((x) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((decorations).wrapping_offset(((x) as i32) as isize)).read())
                            as i32)
                            > 0i32)
                            && (((((decorations).wrapping_offset(((x) as i32) as isize)).read())
                                as i32)
                                <= 120i32))
                            && (((((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                                ((((decorations).wrapping_offset(((x) as i32) as isize)).read())
                                    as i32) as isize
                                    * 32,
                            ))
                            .wrapping_add(17))
                            .read()) as i32)
                                != 4i32)
                        {
                            ShowDecorationOnMap(
                                (((((((decorPos).wrapping_offset(((x) as i32) as isize)).read())
                                    as i32)
                                    >> 4)
                                    .wrapping_add(7i32)) as u16),
                                (((((((decorPos).wrapping_offset(((x) as i32) as isize)).read())
                                    as i32)
                                    & 15i32)
                                    .wrapping_add(7i32)) as u16),
                                ((((decorations).wrapping_offset(((x) as i32) as isize)).read())
                                    as u16),
                            );
                        }
                    }
                    x = (x).wrapping_add(1);
                }
            }
            if ((secretBaseIdx) as i32) != 0i32 {
                FindMetatileIdMapCoords(
                    (&raw mut x).cast::<i16>(),
                    (&raw mut y).cast::<i16>(),
                    544u16,
                );
                MapGridSetMetatileIdAt(
                    ((x) as i32).wrapping_add(7i32),
                    ((y) as i32).wrapping_add(7i32),
                    3617u16,
                );
            } else {
                if (((hidePC) as i32) == 1i32) && (((VarGet(16521u16)) as i32) == 1i32) {
                    FindMetatileIdMapCoords(
                        (&raw mut x).cast::<i16>(),
                        (&raw mut y).cast::<i16>(),
                        544u16,
                    );
                    MapGridSetMetatileIdAt(
                        ((x) as i32).wrapping_add(7i32),
                        ((y) as i32).wrapping_add(7i32),
                        3594u16,
                    );
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSecretBaseDecorationSprites() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut decorations: *mut u8 = core::ptr::null_mut();
        let mut decorationPositions: *mut u8 = core::ptr::null_mut();
        let mut objectEventId: u8 = 0u8;
        let mut metatileBehavior: u8 = 0u8;
        let mut category: u8 = 0u8;
        let mut permission: u8 = 0u8;
        let mut numDecorations: u8 = 0u8;
        objectEventId = 0u8;
        if !((CurMapIsSecretBase()) != 0) {
            decorations = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(10012))
            .cast::<u8>();
            decorationPositions = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(10024))
            .cast::<u8>();
            numDecorations = 12u8;
        } else {
            let mut secretBaseIdx: u16 = VarGet(16468u16);
            decorations = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(6812))
            .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(18))
            .cast::<u8>();
            decorationPositions = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(6812))
            .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(34))
            .cast::<u8>();
            numDecorations = 16u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((numDecorations) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((decorations).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                        == 0i32
                    {
                        break 'l2;
                    }
                    permission = ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        ((((decorations).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            as isize
                            * 32,
                    ))
                    .wrapping_add(17))
                    .read();
                    category = ((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                        ((((decorations).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                            as isize
                            * 32,
                    ))
                    .wrapping_add(19))
                    .read();
                    if ((permission) as i32) == 4i32 {
                        {
                            objectEventId = 0u8;
                            'l3: loop {
                                if !(((objectEventId) as i32)
                                    < ((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .read()) as i32))
                                {
                                    break 'l3;
                                }
                                'l4: {
                                    if ((((((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((objectEventId) as i32) as isize * 24))
                                    .wrapping_add(20)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        == (174i32).wrapping_add(
                                            ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read())
                                                as i32),
                                        )
                                    {
                                        break 'l3;
                                    }
                                }
                                objectEventId = (objectEventId).wrapping_add(1);
                            }
                        }
                        if ((objectEventId) as i32)
                            == ((((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .read()) as i32)
                        {
                            break 'l2;
                        }
                        ((&raw mut gSpecialVar_0x8006).cast::<u16>()).write(
                            ((((((decorationPositions).wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                >> 4) as u16),
                        );
                        ((&raw mut gSpecialVar_0x8007).cast::<u16>()).write(
                            ((((((decorationPositions).wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                & 15i32) as u16),
                        );
                        metatileBehavior = ((MapGridGetMetatileBehaviorAt(
                            ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i32)
                                .wrapping_add(7i32),
                            ((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as i32)
                                .wrapping_add(7i32),
                        )) as u8);
                        if (((MetatileBehavior_HoldsSmallDecoration(metatileBehavior)) as i32)
                            == 1i32)
                            || (((MetatileBehavior_HoldsLargeDecoration(metatileBehavior)) as i32)
                                == 1i32)
                        {
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                                (((16400i32).wrapping_add(
                                    ((((((((((&raw mut gMapHeader).cast::<u8>())
                                        .wrapping_add(4)
                                        .cast::<*mut u8>())
                                    .read())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                    .read())
                                    .wrapping_offset(((objectEventId) as i32) as isize * 24))
                                    .wrapping_add(1))
                                    .read()) as i32)
                                        .wrapping_sub(240i32),
                                )) as u16),
                            );
                            VarSet(
                                ((&raw mut gSpecialVar_Result).cast::<u16>()).read(),
                                (((((&raw mut gDecorations).cast::<u8>()).wrapping_offset(
                                    ((((decorations).wrapping_offset(((i) as i32) as isize)).read())
                                        as i32) as isize
                                        * 32,
                                ))
                                .wrapping_add(28)
                                .cast::<*mut u16>())
                                .read())
                                .read(),
                            );
                            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
                                (((((((((&raw mut gMapHeader).cast::<u8>())
                                    .wrapping_add(4)
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(((objectEventId) as i32) as isize * 24))
                                .read()) as u16),
                            );
                            FlagClear(
                                (((174i32).wrapping_add(
                                    ((((&raw mut gSpecialVar_0x8004).cast::<u16>()).read()) as i32),
                                )) as u16),
                            );
                            TrySpawnObjectEvent(
                                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .wrapping_add(1)
                                .cast::<i8>())
                                .read()) as u8),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<i8>())
                                .read()) as u8),
                            );
                            TryMoveObjectEventToMapCoords(
                                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .wrapping_add(1)
                                .cast::<i8>())
                                .read()) as u8),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<i8>())
                                .read()) as u8),
                                ((((&raw mut gSpecialVar_0x8006).cast::<u16>()).read()) as i16),
                                ((((&raw mut gSpecialVar_0x8007).cast::<u16>()).read()) as i16),
                            );
                            TryOverrideObjectEventTemplateCoords(
                                ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .wrapping_add(1)
                                .cast::<i8>())
                                .read()) as u8),
                                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(4))
                                .cast::<i8>())
                                .read()) as u8),
                            );
                            if (((CurMapIsSecretBase()) as i32) == 1i32)
                                && (((VarGet(16468u16)) as i32) != 0i32)
                            {
                                if ((category) as i32) == 6i32 {
                                    OverrideSecretBaseDecorationSpriteScript(
                                        ((((&raw mut gSpecialVar_Result).cast::<u16>()).read())
                                            as u8),
                                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .wrapping_add(1)
                                        .cast::<i8>())
                                        .read()) as u8),
                                        (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                            .read())
                                        .wrapping_add(4))
                                        .cast::<i8>())
                                        .read()) as u8),
                                        6u8,
                                    );
                                } else {
                                    if ((category) as i32) == 7i32 {
                                        OverrideSecretBaseDecorationSpriteScript(
                                            ((((&raw mut gSpecialVar_Result).cast::<u16>()).read())
                                                as u8),
                                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(4))
                                            .wrapping_add(1)
                                            .cast::<i8>())
                                            .read())
                                                as u8),
                                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>())
                                                .read())
                                            .wrapping_add(4))
                                            .cast::<i8>())
                                            .read())
                                                as u8),
                                            7u8,
                                        );
                                    }
                                }
                            }
                            let __p1 = (&raw mut gSpecialVar_0x8004).cast::<u16>();
                            (__p1).write(((__p1).read()).wrapping_add(1));
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HideSecretBaseDecorationSprites() {
    unsafe {
        let mut objectEventId: u8 = 0u8;
        let mut flag: u16 = 0u16;
        {
            objectEventId = 0u8;
            'l1: loop {
                if !(((objectEventId) as i32)
                    < ((((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    flag = ((((((((&raw mut gMapHeader).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((objectEventId) as i32) as isize * 24))
                    .wrapping_add(20)
                    .cast::<u16>())
                    .read();
                    if (((flag) as i32) >= 174i32) && (((flag) as i32) <= 187i32) {
                        RemoveObjectEventByLocalIdAndMap(
                            (((((((&raw mut gMapHeader).cast::<u8>())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((objectEventId) as i32) as isize * 24))
                            .read(),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .wrapping_add(1)
                            .cast::<i8>())
                            .read()) as u8),
                            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(4))
                            .cast::<i8>())
                            .read()) as u8),
                        );
                        FlagSet(flag);
                    }
                }
                objectEventId = (objectEventId).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSecretBaseOwnerGfxId() {
    unsafe {
        VarSet(
            16415u16,
            ((((((&raw const sSecretBaseOwnerGfxIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    ((GetSecretBaseOwnerType(((VarGet(16468u16)) as u8))) as i32) as isize,
                ))
            .read()) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCurSecretBaseIdFromPosition(position: *mut u8, events: *mut u8) {
    unsafe {
        let mut position = position;
        let mut events = events;
        let mut i: i16 = 0i16;
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < ((((events).wrapping_add(3)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .wrapping_add(5))
                    .read()) as i32)
                        == 8i32)
                        && (((((position).cast::<i16>()).read()) as i32)
                            == (((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(7i32)))
                        && (((((position).wrapping_add(2).cast::<i16>()).read()) as i32)
                            == (((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(7i32))
                    {
                        ((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>()).write(
                            ((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(8))
                            .cast::<u32>())
                            .read()) as u8),
                        );
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WarpIntoSecretBase(position: *mut u8, events: *mut u8) {
    unsafe {
        let mut position = position;
        let mut events = events;
        SetCurSecretBaseIdFromPosition(position, events);
        TrySetCurSecretBaseIndex();
        ScriptContext_SetupScript((&raw mut SecretBase_EventScript_Enter).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetCurSecretBase() -> u8 {
    unsafe {
        SetCurSecretBaseId();
        TrySetCurSecretBaseIndex();
        if ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as i32) == 1i32 {
            return 0u8;
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Task_WarpOutOfSecretBase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                LockPlayerFieldControls();
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(1i16);
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
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                SetWarpDestinationToDynamicWarp(126u8);
                WarpIntoMap();
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCB_DefaultWarpExit));
                SetMainCallback2(Some(CB2_LoadMap));
                UnlockPlayerFieldControls();
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn WarpOutOfSecretBase() {
    unsafe {
        CreateTask(Some(Task_WarpOutOfSecretBase), 0u8);
        FadeScreen(1u8, 0i8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsCurSecretBaseOwnedByAnotherPlayer() {
    unsafe {
        if (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .read()) as i32)
            != ((((&raw mut sCurSecretBaseId).cast::<u8>().cast::<u8>()).read()) as i32)
        {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn GetSecretBaseName(dest: *mut u8, secretBaseIdx: u8) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        let mut secretBaseIdx = secretBaseIdx;
        (StringCopyN(
            dest,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(2))
            .cast::<u8>(),
            GetNameLength(
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
                .wrapping_add(2))
                .cast::<u8>(),
            ),
        ))
        .write(255u8);
        ConvertInternationalString(
            dest,
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(13))
            .read(),
        );
        return StringAppend(dest, (&raw mut gText_ApostropheSBase).cast::<u8>());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseMapName(dest: *mut u8) -> *mut u8 {
    unsafe {
        let mut dest = dest;
        return GetSecretBaseName(dest, ((VarGet(16468u16)) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CopyCurSecretBaseOwnerName_StrVar1() {
    unsafe {
        let mut secretBaseIdx: u8 = 0u8;
        let mut name: *mut u8 = core::ptr::null_mut();
        secretBaseIdx = ((VarGet(16468u16)) as u8);
        name = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
        .wrapping_add(2))
        .cast::<u8>();
        (StringCopyN(
            (&raw mut gStringVar1).cast::<u8>(),
            name,
            GetNameLength(name),
        ))
        .write(255u8);
        ConvertInternationalString(
            (&raw mut gStringVar1).cast::<u8>(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(13))
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn IsSecretBaseRegistered(secretBaseIdx: u8) -> u8 {
    unsafe {
        let mut secretBaseIdx = secretBaseIdx;
        if (crate::c::bf_read(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(1),
            6,
            2,
            false,
        ) as u8)
            != 0
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetAverageEVs(pokemon: *mut u8) -> u8 {
    unsafe {
        let mut pokemon = pokemon;
        let mut evTotal: u16 = 0u16;
        evTotal = ((GetMonData2(pokemon, 26i32)) as u16);
        evTotal = ((((evTotal) as u32).wrapping_add(GetMonData2(pokemon, 27i32))) as u16);
        evTotal = ((((evTotal) as u32).wrapping_add(GetMonData2(pokemon, 28i32))) as u16);
        evTotal = ((((evTotal) as u32).wrapping_add(GetMonData2(pokemon, 29i32))) as u16);
        evTotal = ((((evTotal) as u32).wrapping_add(GetMonData2(pokemon, 30i32))) as u16);
        evTotal = ((((evTotal) as u32).wrapping_add(GetMonData2(pokemon, 31i32))) as u16);
        return ((crate::c::div_i32(((evTotal) as i32), 6i32)) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerSecretBaseParty() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut moveIndex: u16 = 0u16;
        let mut partyId: u16 = 0u16;
        let mut party: *mut u8 = core::ptr::null_mut();
        partyId = 0u16;
        party = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .wrapping_add(52);
        if ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .read())
            != 0
        {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        {
                            moveIndex = 0u16;
                            'l3: loop {
                                if !(((moveIndex) as i32) < 4i32) {
                                    break 'l3;
                                }
                                'l4: {
                                    ((((party).wrapping_add(24)).cast::<u16>()).wrapping_offset(
                                        ((((i) as i32).wrapping_mul(4i32))
                                            .wrapping_add(((moveIndex) as i32)))
                                            as isize,
                                    ))
                                    .write(0u16);
                                }
                                moveIndex = (moveIndex).wrapping_add(1);
                            }
                        }
                        ((((party).wrapping_add(72)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u16);
                        ((((party).wrapping_add(84)).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u16);
                        ((((party).wrapping_add(96)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                        (((party).cast::<u32>()).wrapping_offset(((i) as i32) as isize))
                            .write(0u32);
                        ((((party).wrapping_add(102)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(0u8);
                        if (GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            11i32,
                        ) != 0u32)
                            && (!((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                                45i32,
                            )) != 0))
                        {
                            {
                                moveIndex = 0u16;
                                'l5: loop {
                                    if !(((moveIndex) as i32) < 4i32) {
                                        break 'l5;
                                    }
                                    'l6: {
                                        ((((party).wrapping_add(24)).cast::<u16>())
                                            .wrapping_offset(
                                                ((((partyId) as i32).wrapping_mul(4i32))
                                                    .wrapping_add(((moveIndex) as i32)))
                                                    as isize,
                                            ))
                                        .write(
                                            ((GetMonData2(
                                                ((&raw mut gPlayerParty).cast::<u8>())
                                                    .wrapping_offset(((i) as i32) as isize * 100),
                                                (13i32).wrapping_add(((moveIndex) as i32)),
                                            )) as u16),
                                        );
                                    }
                                    moveIndex = (moveIndex).wrapping_add(1);
                                }
                            }
                            ((((party).wrapping_add(72)).cast::<u16>())
                                .wrapping_offset(((partyId) as i32) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100),
                                    11i32,
                                )) as u16),
                            );
                            ((((party).wrapping_add(84)).cast::<u16>())
                                .wrapping_offset(((partyId) as i32) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100),
                                    12i32,
                                )) as u16),
                            );
                            ((((party).wrapping_add(96)).cast::<u8>())
                                .wrapping_offset(((partyId) as i32) as isize))
                            .write(
                                ((GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100),
                                    56i32,
                                )) as u8),
                            );
                            (((party).cast::<u32>()).wrapping_offset(((partyId) as i32) as isize))
                                .write(GetMonData2(
                                    ((&raw mut gPlayerParty).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 100),
                                    0i32,
                                ));
                            ((((party).wrapping_add(102)).cast::<u8>())
                                .wrapping_offset(((partyId) as i32) as isize))
                            .write(GetAverageEVs(
                                ((&raw mut gPlayerParty).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 100),
                            ));
                            partyId = (partyId).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearAndLeaveSecretBase() {
    unsafe {
        let mut temp: u16 = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(6812))
        .cast::<u8>())
        .wrapping_add(14)
        .cast::<u16>())
        .read();
        ClearSecretBase(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>(),
        );
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .wrapping_add(14)
        .cast::<u16>())
        .write(temp);
        WarpOutOfSecretBase();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveOutOfSecretBase() {
    unsafe {
        IncrementGameStat(20u8);
        ClearAndLeaveSecretBase();
    }
}
pub(crate) unsafe extern "C" fn ClosePlayerSecretBaseEntrance() {
    unsafe {
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut metatileId: i16 = 0i16;
        let mut events: *mut u8 = (((&raw mut gMapHeader).cast::<u8>())
            .wrapping_add(4)
            .cast::<*mut u8>())
        .read();
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < ((((events).wrapping_add(3)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .wrapping_add(5))
                    .read()) as i32)
                        == 8i32)
                        && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(6812))
                        .cast::<u8>())
                        .read()) as u32)
                            == ((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(8))
                            .cast::<u32>())
                            .read())
                    {
                        metatileId = ((MapGridGetMetatileIdAt(
                            (((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(7i32),
                            (((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(7i32),
                        )) as i16);
                        {
                            j = 0u16;
                            'l3: loop {
                                if !(((j) as u32) < crate::c::div_u32(28u32, 4u32)) {
                                    break 'l3;
                                }
                                'l4: {
                                    if (((((((&raw const sSecretBaseEntranceMetatiles)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((j) as i32) as isize * 4))
                                    .wrapping_add(2)
                                    .cast::<u16>())
                                    .read()) as i32)
                                        == ((metatileId) as i32)
                                    {
                                        MapGridSetMetatileIdAt(
                                            (((((((events).wrapping_add(16).cast::<*mut u8>())
                                                .read())
                                            .wrapping_offset(((i) as i32) as isize * 12))
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(7i32),
                                            (((((((events).wrapping_add(16).cast::<*mut u8>())
                                                .read())
                                            .wrapping_offset(((i) as i32) as isize * 12))
                                            .wrapping_add(2)
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                .wrapping_add(7i32),
                                            (((((((((&raw const sSecretBaseEntranceMetatiles)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset(((j) as i32) as isize * 4))
                                            .cast::<u16>())
                                            .read())
                                                as i32)
                                                | 3072i32)
                                                as u16),
                                        );
                                        break 'l3;
                                    }
                                }
                                j = (j).wrapping_add(1);
                            }
                        }
                        DrawWholeMapView();
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveOutOfSecretBaseFromOutside() {
    unsafe {
        let mut temp: u16 = 0u16;
        ClosePlayerSecretBaseEntrance();
        IncrementGameStat(20u8);
        temp = ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .wrapping_add(14)
        .cast::<u16>())
        .read();
        ClearSecretBase(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>(),
        );
        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>())
        .wrapping_add(14)
        .cast::<u16>())
        .write(temp);
    }
}
pub(crate) unsafe extern "C" fn GetNumRegisteredSecretBases() -> u8 {
    unsafe {
        let mut i: i16 = 0i16;
        let mut count: u8 = 0u8;
        {
            i = 1i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if ((IsSecretBaseRegistered(((i) as u8))) as i32) == 1i32 {
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return count;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurSecretBaseRegistrationValidity() {
    unsafe {
        if ((IsSecretBaseRegistered(((VarGet(16468u16)) as u8))) as i32) == 1i32 {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
        } else {
            if ((GetNumRegisteredSecretBases()) as i32) >= 10i32 {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(2u16);
            } else {
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ToggleCurSecretBaseRegistry() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((VarGet(16468u16)) as i32) as isize * 160))
            .wrapping_add(1),
            6,
            2,
            ((((crate::c::bf_read(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_offset(((VarGet(16468u16)) as i32) as isize * 160))
                .wrapping_add(1),
                6,
                2,
                false,
            ) as u8) as i32)
                ^ 1i32) as u8) as i32,
        );
        FlagSet(268u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSecretBaseDecorationMenu() {
    unsafe {
        CreateTask(Some(DoSecretBaseDecorationMenu), 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowSecretBaseRegistryMenu() {
    unsafe {
        CreateTask(Some(Task_ShowSecretBaseRegistryMenu), 0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_ShowSecretBaseRegistryMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        LockPlayerFieldControls();
        (data).write(((GetNumRegisteredSecretBases()) as i16));
        if (((data).read()) as i32) != 0i32 {
            ((data).wrapping_offset(1)).write(0i16);
            ((data).wrapping_offset(2)).write(0i16);
            ClearDialogWindowAndFrame(0u8, 0u8);
            ((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).write(AllocZeroed(440u32));
            ((data).wrapping_offset(6)).write(
                ((AddWindow(
                    ((&raw const sRegistryWindowTemplates)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                )) as i16),
            );
            BuildRegistryMenuItems(taskId);
            FinalizeRegistryMenu(taskId);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(HandleRegistryMenuInput));
        } else {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_NoRegistry).cast::<u8>(),
                Some(GoToSecretBasePCRegisterMenu),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BuildRegistryMenuItems(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = core::ptr::null_mut();
        let mut i: u8 = 0u8;
        let mut count: u8 = 0u8;
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        count = 0u8;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if (IsSecretBaseRegistered(i)) != 0 {
                        GetSecretBaseName(
                            ((((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(88))
                            .cast::<u8>())
                            .wrapping_offset(((count) as i32) as isize * 32))
                            .cast::<u8>(),
                            i,
                        );
                        ((((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((count) as i32) as isize * 8))
                        .cast::<*mut u8>())
                        .write(
                            ((((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>())
                                .read())
                            .wrapping_add(88))
                            .cast::<u8>())
                            .wrapping_offset(((count) as i32) as isize * 32))
                            .cast::<u8>(),
                        );
                        ((((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((count) as i32) as isize * 8))
                        .wrapping_add(4)
                        .cast::<i32>())
                        .write(((i) as i32));
                        count = (count).wrapping_add(1);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
            .wrapping_offset(((count) as i32) as isize * 8))
        .cast::<*mut u8>())
        .write((&raw mut gText_Cancel).cast::<u8>());
        ((((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>())
            .wrapping_offset(((count) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i32>())
        .write((-2i32));
        (data).write(((((count) as i32).wrapping_add(1i32)) as i16));
        if (((data).read()) as i32) < 8i32 {
            ((data).wrapping_offset(3)).write((data).read());
        } else {
            ((data).wrapping_offset(3)).write(8i16);
        }
        (&raw mut gMultiuseListMenuTemplate)
            .cast::<u8>()
            .cast::<crate::c::Rec4<24>>()
            .write_unaligned(
                (&raw const sRegistryListMenuTemplate)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<crate::c::Rec4<24>>()
                    .read_unaligned(),
            );
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).wrapping_add(16))
            .write(((((data).wrapping_offset(6)).read()) as u8));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write((((data).read()) as u16));
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>()).cast::<*mut u8>())
            .write((((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).read()).cast::<u8>());
        (((&raw mut gMultiuseListMenuTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(((((data).wrapping_offset(3)).read()) as u16));
    }
}
pub(crate) unsafe extern "C" fn RegistryMenu_OnCursorMove(unused: i32, flag: u8, menu: *mut u8) {
    unsafe {
        let mut unused = unused;
        let mut flag = flag;
        let mut menu = menu;
        if ((flag) as i32) != 1i32 {
            PlaySE(5u16);
        }
    }
}
pub(crate) unsafe extern "C" fn FinalizeRegistryMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        SetStandardWindowBorderStyle(((((data).wrapping_offset(6)).read()) as u8), 0u8);
        ((data).wrapping_offset(5)).write(
            ((ListMenuInit(
                (&raw mut gMultiuseListMenuTemplate).cast::<u8>(),
                ((((data).wrapping_offset(2)).read()) as u16),
                ((((data).wrapping_offset(1)).read()) as u16),
            )) as i16),
        );
        AddRegistryMenuScrollArrows(taskId);
        ScheduleBgCopyTilemapToVram(0u8);
    }
}
pub(crate) unsafe extern "C" fn AddRegistryMenuScrollArrows(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ((data).wrapping_offset(8)).write(
            ((AddScrollIndicatorArrowPairParameterized(
                2u32,
                188i32,
                12i32,
                148i32,
                (((data).read()) as i32)
                    .wrapping_sub(((((data).wrapping_offset(3)).read()) as i32)),
                5112i32,
                5112i32,
                ((data).wrapping_offset(2)).cast::<u16>(),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn HandleRegistryMenuInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut input: i32 = ListMenu_ProcessInput(((((data).wrapping_offset(5)).read()) as u8));
        ListMenuGetScrollAndRow(
            ((((data).wrapping_offset(5)).read()) as u8),
            ((data).wrapping_offset(2)).cast::<u16>(),
            ((data).wrapping_offset(1)).cast::<u16>(),
        );
        'l1: {
            let __sw1 = input;
            let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
            if __sw1 == (-1i32) {
                break 'l1;
            }
            if __sw1 == (-2i32) {
                PlaySE(5u16);
                DestroyListMenuTask(
                    ((((data).wrapping_offset(5)).read()) as u8),
                    core::ptr::null_mut(),
                    core::ptr::null_mut(),
                );
                RemoveScrollIndicatorArrowPair(((((data).wrapping_offset(8)).read()) as u8));
                ClearStdWindowAndFrame(((((data).wrapping_offset(6)).read()) as u8), 0u8);
                ClearWindowTilemap(((((data).wrapping_offset(6)).read()) as u8));
                RemoveWindow(((((data).wrapping_offset(6)).read()) as u8));
                ScheduleBgCopyTilemapToVram(0u8);
                Free(((&raw mut sRegistryMenu).cast::<u8>().cast::<*mut u8>()).read());
                GoToSecretBasePCRegisterMenu(taskId);
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                ((data).wrapping_offset(4)).write(((input) as i16));
                ShowRegistryMenuActions(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowRegistryMenuActions(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut template = crate::ffi::Align4([0u8; 8]);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        RemoveScrollIndicatorArrowPair(((((data).wrapping_offset(8)).read()) as u8));
        (&raw mut template)
            .cast::<u8>()
            .cast::<crate::c::Rec4<8>>()
            .write_unaligned(
                (((&raw const sRegistryWindowTemplates)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(8)
                .cast::<crate::c::Rec4<8>>()
                .read_unaligned(),
            );
        (((&raw mut template).cast::<u8>()).wrapping_add(3)).write(
            ((GetMaxWidthInMenuTable(
                ((&raw const sRegistryMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
                2i32,
            )) as u8),
        );
        ((data).wrapping_offset(7)).write(((AddWindow((&raw mut template).cast::<u8>())) as i16));
        SetStandardWindowBorderStyle(((((data).wrapping_offset(7)).read()) as u8), 0u8);
        PrintMenuTable(
            ((((data).wrapping_offset(7)).read()) as u8),
            ((crate::c::div_u32(16u32, 8u32)) as u8),
            ((&raw const sRegistryMenuActions).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        InitMenuInUpperLeftCornerNormal(
            ((((data).wrapping_offset(7)).read()) as u8),
            ((crate::c::div_u32(16u32, 8u32)) as u8),
            0u8,
        );
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleRegistryMenuActionsInput));
    }
}
pub(crate) unsafe extern "C" fn HandleRegistryMenuActionsInput(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut input: i8 = Menu_ProcessInputNoWrap();
        'l1: {
            let __sw1 = ((input) as i32);
            let __matched = __sw1 == (-1i32) || __sw1 == (-2i32);
            if __sw1 == (-1i32) {
                PlaySE(5u16);
                ReturnToMainRegistryMenu(taskId);
                break 'l1;
            }
            if __sw1 == (-2i32) {
                break 'l1;
            }
            if !__matched {
                PlaySE(5u16);
                (((((((&raw const sRegistryMenuActions).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((input) as i32) as isize * 8))
                .wrapping_add(4))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .read())
                .unwrap_unchecked()(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ShowRegistryMenuDeleteConfirmation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ClearStdWindowAndFrame(((((data).wrapping_offset(6)).read()) as u8), 0u8);
        ClearStdWindowAndFrame(((((data).wrapping_offset(7)).read()) as u8), 0u8);
        ClearWindowTilemap(((((data).wrapping_offset(6)).read()) as u8));
        ClearWindowTilemap(((((data).wrapping_offset(7)).read()) as u8));
        RemoveWindow(((((data).wrapping_offset(7)).read()) as u8));
        ScheduleBgCopyTilemapToVram(0u8);
        GetSecretBaseName(
            (&raw mut gStringVar1).cast::<u8>(),
            ((((data).wrapping_offset(4)).read()) as u8),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_OkayToDeleteFromRegistry).cast::<u8>(),
        );
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(ShowRegistryMenuDeleteYesNo),
        );
    }
}
pub(crate) unsafe extern "C" fn ShowRegistryMenuDeleteYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayYesNoMenuDefaultYes();
        DoYesNoFuncWithChoice(
            taskId,
            (&raw const sDeleteRegistryYesNoFuncs)
                .cast::<u8>()
                .cast_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeleteRegistry_Yes_Callback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ClearDialogWindowAndFrame(0u8, 0u8);
        DestroyListMenuTask(
            ((((data).wrapping_offset(5)).read()) as u8),
            ((data).wrapping_offset(2)).cast::<u16>(),
            ((data).wrapping_offset(1)).cast::<u16>(),
        );
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((((data).wrapping_offset(4)).read()) as i32) as isize * 160))
            .wrapping_add(1),
            6,
            2,
            (0u8) as i32,
        );
        BuildRegistryMenuItems(taskId);
        SetCursorWithinListBounds(
            ((data).wrapping_offset(2)).cast::<u16>(),
            ((data).wrapping_offset(1)).cast::<u16>(),
            ((((data).wrapping_offset(3)).read()) as u8),
            (((data).read()) as u8),
        );
        FinalizeRegistryMenu(taskId);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleRegistryMenuInput));
    }
}
pub(crate) unsafe extern "C" fn DeleteRegistry_Yes(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gText_RegisteredDataDeleted).cast::<u8>(),
            Some(DeleteRegistry_Yes_Callback),
        );
    }
}
pub(crate) unsafe extern "C" fn DeleteRegistry_No(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        ClearDialogWindowAndFrame(0u8, 0u8);
        DestroyListMenuTask(
            ((((data).wrapping_offset(5)).read()) as u8),
            ((data).wrapping_offset(2)).cast::<u16>(),
            ((data).wrapping_offset(1)).cast::<u16>(),
        );
        FinalizeRegistryMenu(taskId);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleRegistryMenuInput));
    }
}
pub(crate) unsafe extern "C" fn ReturnToMainRegistryMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        AddRegistryMenuScrollArrows(taskId);
        ClearStdWindowAndFrame(((((data).wrapping_offset(7)).read()) as u8), 0u8);
        ClearWindowTilemap(((((data).wrapping_offset(7)).read()) as u8));
        RemoveWindow(((((data).wrapping_offset(7)).read()) as u8));
        ScheduleBgCopyTilemapToVram(0u8);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(HandleRegistryMenuInput));
    }
}
pub(crate) unsafe extern "C" fn GoToSecretBasePCRegisterMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((VarGet(16468u16)) as i32) == 0i32 {
            ScriptContext_SetupScript((&raw mut SecretBase_EventScript_PCCancel).cast::<u8>());
        } else {
            ScriptContext_SetupScript(
                (&raw mut SecretBase_EventScript_ShowRegisterMenu).cast::<u8>(),
            );
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn GetSecretBaseOwnerType(secretBaseIdx: u8) -> u8 {
    unsafe {
        let mut secretBaseIdx = secretBaseIdx;
        return (((crate::c::rem_i32(
            ((((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(9))
            .cast::<u8>())
            .read()) as i32),
            5i32,
        ))
        .wrapping_add(
            ((crate::c::bf_read(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
                .wrapping_add(1),
                4,
                1,
                false,
            ) as u8) as i32)
                .wrapping_mul(5i32),
        )) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseTrainerLoseText() -> *mut u8 {
    unsafe {
        let mut ownerType: u8 = GetSecretBaseOwnerType(((VarGet(16468u16)) as u8));
        if ((ownerType) as i32) == 0i32 {
            return (&raw mut SecretBase_Text_Trainer0Defeated).cast::<u8>();
        } else {
            if ((ownerType) as i32) == 1i32 {
                return (&raw mut SecretBase_Text_Trainer1Defeated).cast::<u8>();
            } else {
                if ((ownerType) as i32) == 2i32 {
                    return (&raw mut SecretBase_Text_Trainer2Defeated).cast::<u8>();
                } else {
                    if ((ownerType) as i32) == 3i32 {
                        return (&raw mut SecretBase_Text_Trainer3Defeated).cast::<u8>();
                    } else {
                        if ((ownerType) as i32) == 4i32 {
                            return (&raw mut SecretBase_Text_Trainer4Defeated).cast::<u8>();
                        } else {
                            if ((ownerType) as i32) == 5i32 {
                                return (&raw mut SecretBase_Text_Trainer5Defeated).cast::<u8>();
                            } else {
                                if ((ownerType) as i32) == 6i32 {
                                    return (&raw mut SecretBase_Text_Trainer6Defeated)
                                        .cast::<u8>();
                                } else {
                                    if ((ownerType) as i32) == 7i32 {
                                        return (&raw mut SecretBase_Text_Trainer7Defeated)
                                            .cast::<u8>();
                                    } else {
                                        if ((ownerType) as i32) == 8i32 {
                                            return (&raw mut SecretBase_Text_Trainer8Defeated)
                                                .cast::<u8>();
                                        } else {
                                            return (&raw mut SecretBase_Text_Trainer9Defeated)
                                                .cast::<u8>();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return core::ptr::null_mut();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PrepSecretBaseBattleFlags() {
    unsafe {
        TryGainNewFanFromCounter(1u8);
        ((&raw mut gTrainerBattleOpponent_A).cast::<u16>()).write(1024u16);
        ((&raw mut gBattleTypeFlags).cast::<u32>()).write(134217736u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattledOwnerFromResult() {
    unsafe {
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((VarGet(16468u16)) as i32) as isize * 160))
            .wrapping_add(1),
            5,
            1,
            ((((&raw mut gSpecialVar_Result).cast::<u16>()).read()) as u8) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseOwnerAndState() {
    unsafe {
        let mut secretBaseIdx: u16 = 0u16;
        let mut i: u8 = 0u8;
        secretBaseIdx = VarGet(16468u16);
        if !((FlagGet(
            ((((2335i32).wrapping_add((8i32).wrapping_sub(crate::c::rem_i32(2335i32, 8i32))))
                .wrapping_add(2i32)) as u16),
        )) != 0)
        {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l1;
                    }
                    'l2: {
                        crate::c::bf_write(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(6812))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 160))
                            .wrapping_add(1),
                            5,
                            1,
                            (0u8) as i32,
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            FlagSet(
                ((((2335i32).wrapping_add((8i32).wrapping_sub(crate::c::rem_i32(2335i32, 8i32))))
                    .wrapping_add(2i32)) as u16),
            );
        }
        ((&raw mut gSpecialVar_0x8004).cast::<u16>())
            .write(((GetSecretBaseOwnerType(((secretBaseIdx) as u8))) as u16));
        ((&raw mut gSpecialVar_Result).cast::<u16>()).write(
            ((crate::c::bf_read(
                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
                .wrapping_add(1),
                5,
                1,
                false,
            ) as u8) as u16),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SecretBasePerStepCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut behavior: u8 = 0u8;
        let mut tileId: u16 = 0u16;
        let mut data: *mut i16 = core::ptr::null_mut();
        data = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = ((((data).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                if (VarGet(16468u16)) != 0 {
                    ((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).write(1u8);
                } else {
                    ((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).write(0u8);
                }
                PlayerGetDestCoords((data).wrapping_offset(2), (data).wrapping_offset(3));
                ((data).wrapping_offset(1)).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                PlayerGetDestCoords(&raw mut x, &raw mut y);
                if (((x) as i32) == ((((data).wrapping_offset(2)).read()) as i32))
                    && (((y) as i32) == ((((data).wrapping_offset(3)).read()) as i32))
                {
                    return;
                }
                ((data).wrapping_offset(2)).write(x);
                ((data).wrapping_offset(3)).write(y);
                VarSet(
                    16620u16,
                    ((((VarGet(16620u16)) as i32).wrapping_add(1i32)) as u16),
                );
                behavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
                tileId = ((MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))) as u16);
                if (((tileId) as i32) == 564i32) || (((tileId) as i32) == 572i32) {
                    if ((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).read()) as i32)
                        == 1i32
                    {
                        VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 32i32) as u16));
                    }
                } else {
                    if ((((((((((tileId) as i32) == 696i32) || (((tileId) as i32) == 697i32))
                        || (((tileId) as i32) == 698i32))
                        || (((tileId) as i32) == 704i32))
                        || (((tileId) as i32) == 705i32))
                        || (((tileId) as i32) == 706i32))
                        || (((tileId) as i32) == 712i32))
                        || (((tileId) as i32) == 713i32))
                        || (((tileId) as i32) == 714i32)
                    {
                        if ((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).read())
                            as i32)
                            == 1i32
                        {
                            VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 1i32) as u16));
                        }
                    } else {
                        if (((((tileId) as i32) == 569i32) || (((tileId) as i32) == 577i32))
                            || (((tileId) as i32) == 593i32))
                            || (((tileId) as i32) == 601i32)
                        {
                            if ((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).read())
                                as i32)
                                == 1i32
                            {
                                VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 4i32) as u16));
                            }
                        } else {
                            if ((((behavior) as i32) == 52i32) && (((tileId) as i32) == 621i32))
                                || ((((behavior) as i32) == 53i32)
                                    && (MapGridGetMetatileIdAt(((x) as i32), ((y) as i32))
                                        == 618i32))
                            {
                                if ((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>())
                                    .read()) as i32)
                                    == 1i32
                                {
                                    VarSet(
                                        16623u16,
                                        ((((VarGet(16623u16)) as i32) | 512i32) as u16),
                                    );
                                }
                            } else {
                                if (((behavior) as i32) == 193i32) && (((tileId) as i32) == 573i32)
                                {
                                    if ((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>())
                                        .read()) as i32)
                                        == 1i32
                                    {
                                        VarSet(
                                            16623u16,
                                            ((((VarGet(16623u16)) as i32) ^ 4096i32) as u16),
                                        );
                                        VarSet(
                                            16623u16,
                                            ((((VarGet(16623u16)) as i32) | 8192i32) as u16),
                                        );
                                    }
                                } else {
                                    if (((behavior) as i32) == 71i32)
                                        && (((tileId) as i32) == 574i32)
                                    {
                                        if ((((&raw mut sInFriendSecretBase)
                                            .cast::<u8>()
                                            .cast::<u8>())
                                        .read()) as i32)
                                            == 1i32
                                        {
                                            VarSet(
                                                16623u16,
                                                ((((VarGet(16623u16)) as i32) | 4096i32) as u16),
                                            );
                                            VarSet(
                                                16623u16,
                                                ((((VarGet(16623u16)) as i32) ^ 8192i32) as u16),
                                            );
                                        }
                                    } else {
                                        if ((MetatileBehavior_IsSecretBaseGlitterMat(behavior))
                                            as i32)
                                            == 1i32
                                        {
                                            if ((((&raw mut sInFriendSecretBase)
                                                .cast::<u8>()
                                                .cast::<u8>())
                                            .read())
                                                as i32)
                                                == 1i32
                                            {
                                                VarSet(
                                                    16623u16,
                                                    ((((VarGet(16623u16)) as i32) | 128i32) as u16),
                                                );
                                            }
                                        } else {
                                            if ((MetatileBehavior_IsSecretBaseBalloon(behavior))
                                                as i32)
                                                == 1i32
                                            {
                                                PopSecretBaseBalloon(
                                                    ((MapGridGetMetatileIdAt(
                                                        ((x) as i32),
                                                        ((y) as i32),
                                                    ))
                                                        as i16),
                                                    x,
                                                    y,
                                                );
                                                if ((((&raw mut sInFriendSecretBase)
                                                    .cast::<u8>()
                                                    .cast::<u8>())
                                                .read())
                                                    as i32)
                                                    == 1i32
                                                {
                                                    'l2: {
                                                        let __sw2 = MapGridGetMetatileIdAt(
                                                            ((x) as i32),
                                                            ((y) as i32),
                                                        );
                                                        if __sw2 == 824i32
                                                            || __sw2 == 828i32
                                                            || __sw2 == 832i32
                                                        {
                                                            VarSet(
                                                                16622u16,
                                                                ((((VarGet(16622u16)) as i32)
                                                                    | 2i32)
                                                                    as u16),
                                                            );
                                                            break 'l2;
                                                        }
                                                        if __sw2 == 552i32 {
                                                            VarSet(
                                                                16622u16,
                                                                ((((VarGet(16622u16)) as i32)
                                                                    | 256i32)
                                                                    as u16),
                                                            );
                                                            break 'l2;
                                                        }
                                                    }
                                                }
                                            } else {
                                                if ((MetatileBehavior_IsSecretBaseBreakableDoor(
                                                    behavior,
                                                ))
                                                    as i32)
                                                    == 1i32
                                                {
                                                    if ((((&raw mut sInFriendSecretBase)
                                                        .cast::<u8>()
                                                        .cast::<u8>())
                                                    .read())
                                                        as i32)
                                                        == 1i32
                                                    {
                                                        VarSet(
                                                            16623u16,
                                                            ((((VarGet(16623u16)) as i32) | 1024i32)
                                                                as u16),
                                                        );
                                                    }
                                                    ShatterSecretBaseBreakableDoor(x, y);
                                                } else {
                                                    if ((MetatileBehavior_IsSecretBaseSoundMat(
                                                        behavior,
                                                    ))
                                                        as i32)
                                                        == 1i32
                                                    {
                                                        if ((((&raw mut sInFriendSecretBase)
                                                            .cast::<u8>()
                                                            .cast::<u8>())
                                                        .read())
                                                            as i32)
                                                            == 1i32
                                                        {
                                                            VarSet(
                                                                16622u16,
                                                                ((((VarGet(16622u16)) as i32)
                                                                    | 32768i32)
                                                                    as u16),
                                                            );
                                                        }
                                                    } else {
                                                        if ((MetatileBehavior_IsSecretBaseJumpMat(
                                                            behavior,
                                                        ))
                                                            as i32)
                                                            == 1i32
                                                        {
                                                            if ((((&raw mut sInFriendSecretBase)
                                                                .cast::<u8>()
                                                                .cast::<u8>())
                                                            .read())
                                                                as i32)
                                                                == 1i32
                                                            {
                                                                VarSet(
                                                                    16623u16,
                                                                    ((((VarGet(16623u16)) as i32)
                                                                        | 16384i32)
                                                                        as u16),
                                                                );
                                                            }
                                                        } else {
                                                            if (((MetatileBehavior_IsSecretBaseSpinMat(behavior)) as i32)) == 1i32 {
if (((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).read()) as i32)) == 1i32 {
VarSet(16623u16, ((((((VarGet(16623u16)) as i32)) | 2i32)) as u16));
}
}
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((FieldEffectActiveListContains(((((data).wrapping_offset(4)).read()) as u8)))
                    != 0)
                {
                    ((data).wrapping_offset(1)).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveSecretBase(
    secretBaseIdx: u8,
    secretBase: *mut u8,
    version: u32,
    language: u32,
) {
    unsafe {
        let mut secretBaseIdx = secretBaseIdx;
        let mut secretBase = secretBase;
        let mut version = version;
        let mut language = language;
        let mut stringLength: i32 = 0i32;
        let mut name: *mut u8 = core::ptr::null_mut();
        (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812)).cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160)
            .cast::<crate::c::Rec4<160>>()
            .write_unaligned(secretBase.cast::<crate::c::Rec4<160>>().read_unaligned());
        crate::c::bf_write(
            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(1),
            6,
            2,
            (2u8) as i32,
        );
        if (version == 1u32) || (version == 2u32) {
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(13))
            .write(2u8);
        }
        if (version == 3u32) && (language == 1u32) {
            name = (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                .wrapping_add(6812))
            .cast::<u8>())
            .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
            .wrapping_add(2))
            .cast::<u8>();
            {
                stringLength = 0i32;
                'l1: loop {
                    if !(stringLength < 7i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((name).wrapping_offset((stringLength) as isize)).read()) as i32)
                            == 255i32
                        {
                            break 'l1;
                        }
                    }
                    stringLength = (stringLength).wrapping_add(1);
                }
            }
            if stringLength > 5i32 {
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                    .cast::<u8>())
                .wrapping_offset(((secretBaseIdx) as i32) as isize * 160))
                .wrapping_add(13))
                .write(2u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SecretBasesHaveSameTrainerId(
    secretBase1: *mut u8,
    secretBase2: *mut u8,
) -> u8 {
    unsafe {
        let mut secretBase1 = secretBase1;
        let mut secretBase2 = secretBase2;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((secretBase1).wrapping_add(9)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((((((secretBase2).wrapping_add(9)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SecretBasesHaveSameTrainerName(sbr1: *mut u8, sbr2: *mut u8) -> u8 {
    unsafe {
        let mut sbr1 = sbr1;
        let mut sbr2 = sbr2;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !((((i) as i32) < 7i32)
                    && ((((((((sbr1).wrapping_add(2)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 255i32)
                        || (((((((sbr2).wrapping_add(2)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 255i32)))
                {
                    break 'l1;
                }
                'l2: {
                    if ((((((sbr1).wrapping_add(2)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((((((sbr2).wrapping_add(2)).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn SecretBasesBelongToSamePlayer(
    secretBase1: *mut u8,
    secretBase2: *mut u8,
) -> u8 {
    unsafe {
        let mut secretBase1 = secretBase1;
        let mut secretBase2 = secretBase2;
        if ((((crate::c::bf_read((secretBase1).wrapping_add(1), 4, 1, false) as u8) as i32)
            == ((crate::c::bf_read((secretBase2).wrapping_add(1), 4, 1, false) as u8) as i32))
            && ((SecretBasesHaveSameTrainerId(secretBase1, secretBase2)) != 0))
            && ((SecretBasesHaveSameTrainerName(secretBase1, secretBase2)) != 0)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetSecretBaseIndexFromId(secretBaseId: u8) -> i16 {
    unsafe {
        let mut secretBaseId = secretBaseId;
        let mut i: i16 = 0i16;
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 160))
                    .read()) as i32)
                        == ((secretBaseId) as i32)
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return (-1i16);
    }
}
pub(crate) unsafe extern "C" fn FindAvailableSecretBaseIndex() -> u8 {
    unsafe {
        let mut i: i16 = 0i16;
        {
            i = 1i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                        .wrapping_add(6812))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 160))
                    .read()) as i32)
                        == 0i32
                    {
                        return ((i) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn FindUnregisteredSecretBaseIndex() -> u8 {
    unsafe {
        let mut i: i16 = 0i16;
        {
            i = 1i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if (((crate::c::bf_read(
                        ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(6812))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 160))
                        .wrapping_add(1),
                        6,
                        2,
                        false,
                    ) as u8) as i32)
                        == 0i32)
                        && (((crate::c::bf_read(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(6812))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 160))
                            .wrapping_add(1),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            == 0i32)
                    {
                        return ((i) as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TrySaveFriendsSecretBase(
    secretBase: *mut u8,
    version: u32,
    language: u32,
) -> u8 {
    unsafe {
        let mut secretBase = secretBase;
        let mut version = version;
        let mut language = language;
        let mut index: i16 = 0i16;
        if !(((secretBase).read()) != 0) {
            return 0u8;
        }
        index = GetSecretBaseIndexFromId((secretBase).read());
        if ((index) as i32) != 0i32 {
            if ((index) as i32) != (-1i32) {
                if ((crate::c::bf_read(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                        .cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize * 160))
                    .wrapping_add(1),
                    0,
                    4,
                    false,
                ) as u8) as i32)
                    == 1i32
                {
                    return 0u8;
                }
                if (((crate::c::bf_read(
                    ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                        .cast::<u8>())
                    .wrapping_offset(((index) as i32) as isize * 160))
                    .wrapping_add(1),
                    6,
                    2,
                    false,
                ) as u8) as i32)
                    != 2i32)
                    || (((crate::c::bf_read((secretBase).wrapping_add(1), 0, 4, false) as u8)
                        as i32)
                        == 1i32)
                {
                    SaveSecretBase(((index) as u8), secretBase, version, language);
                    return ((index) as u8);
                }
            } else {
                index = ((FindAvailableSecretBaseIndex()) as i16);
                if ((index) as i32) != 0i32 {
                    SaveSecretBase(((index) as u8), secretBase, version, language);
                    return ((index) as u8);
                }
                index = ((FindUnregisteredSecretBaseIndex()) as i16);
                if ((index) as i32) != 0i32 {
                    SaveSecretBase(((index) as u8), secretBase, version, language);
                    return ((index) as u8);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SortSecretBasesByRegistryStatus() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut j: u8 = 0u8;
        let mut secretBases: *mut u8 = core::ptr::null_mut();
        secretBases = ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
            .cast::<u8>();
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 19i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = ((((i) as i32).wrapping_add(1i32)) as u8);
                        'l3: loop {
                            if !(((j) as i32) < 20i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((crate::c::bf_read(
                                    ((secretBases).wrapping_offset(((i) as i32) as isize * 160))
                                        .wrapping_add(1),
                                    6,
                                    2,
                                    false,
                                ) as u8) as i32)
                                    == 0i32)
                                    && (((crate::c::bf_read(
                                        ((secretBases)
                                            .wrapping_offset(((j) as i32) as isize * 160))
                                        .wrapping_add(1),
                                        6,
                                        2,
                                        false,
                                    ) as u8) as i32)
                                        == 1i32))
                                    || ((((crate::c::bf_read(
                                        ((secretBases)
                                            .wrapping_offset(((i) as i32) as isize * 160))
                                        .wrapping_add(1),
                                        6,
                                        2,
                                        false,
                                    ) as u8) as i32)
                                        == 2i32)
                                        && (((crate::c::bf_read(
                                            ((secretBases)
                                                .wrapping_offset(((j) as i32) as isize * 160))
                                            .wrapping_add(1),
                                            6,
                                            2,
                                            false,
                                        ) as u8)
                                            as i32)
                                            != 2i32))
                                {
                                    let mut temp = crate::ffi::Align4([0u8; 160]);
                                    {
                                        (&raw mut temp)
                                            .cast::<u8>()
                                            .cast::<crate::c::Rec4<160>>()
                                            .write_unaligned(
                                                (secretBases)
                                                    .wrapping_offset(((i) as i32) as isize * 160)
                                                    .cast::<crate::c::Rec4<160>>()
                                                    .read_unaligned(),
                                            );
                                        (secretBases)
                                            .wrapping_offset(((i) as i32) as isize * 160)
                                            .cast::<crate::c::Rec4<160>>()
                                            .write_unaligned(
                                                (secretBases)
                                                    .wrapping_offset(((j) as i32) as isize * 160)
                                                    .cast::<crate::c::Rec4<160>>()
                                                    .read_unaligned(),
                                            );
                                        (secretBases)
                                            .wrapping_offset(((j) as i32) as isize * 160)
                                            .cast::<crate::c::Rec4<160>>()
                                            .write_unaligned(
                                                (&raw mut temp)
                                                    .cast::<u8>()
                                                    .cast::<crate::c::Rec4<160>>()
                                                    .read_unaligned(),
                                            );
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrySaveFriendsSecretBases(mixer: *mut u8, registryStatus: u8) {
    unsafe {
        let mut mixer = mixer;
        let mut registryStatus = registryStatus;
        let mut i: u16 = 0u16;
        {
            i = 1u16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        ((((mixer).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 160))
                        .wrapping_add(1),
                        6,
                        2,
                        false,
                    ) as u8) as i32)
                        == ((registryStatus) as i32)
                    {
                        TrySaveFriendsSecretBase(
                            (((mixer).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 160),
                            ((mixer).wrapping_add(4).cast::<u32>()).read(),
                            ((mixer).wrapping_add(8).cast::<u32>()).read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SecretBaseBelongsToPlayer(secretBase: *mut u8) -> u8 {
    unsafe {
        let mut secretBase = secretBase;
        let mut i: u8 = 0u8;
        if (((secretBase).read()) as i32) == 0i32 {
            return 0u8;
        }
        if (((secretBase).read()) != 0)
            && (((crate::c::bf_read((secretBase).wrapping_add(1), 4, 1, false) as u8) as i32)
                != ((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read()).wrapping_add(8))
                    .read()) as i32))
        {
            return 0u8;
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((secretBase).wrapping_add(9)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .wrapping_add(10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !((((i) as i32) < 7i32)
                    && ((((((((secretBase).wrapping_add(2)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != 255i32)
                        || ((((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            != 255i32)))
                {
                    break 'l3;
                }
                'l4: {
                    if ((((((secretBase).wrapping_add(2)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        != (((((((&raw mut gSaveBlock2Ptr).cast::<*mut u8>()).read())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                    {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn DeleteFirstOldBaseFromPlayerInRecordMixingFriendsRecords(
    basesA: *mut u8,
    basesB: *mut u8,
    basesC: *mut u8,
) {
    unsafe {
        let mut basesA = basesA;
        let mut basesB = basesB;
        let mut basesC = basesC;
        let mut i: u8 = 0u8;
        let mut sbFlags: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if !((((sbFlags) as i32) & 1i32) != 0) {
                        if ((SecretBaseBelongsToPlayer(
                            (basesA).wrapping_offset(((i) as i32) as isize * 160),
                        )) as i32)
                            == 1i32
                        {
                            ClearSecretBase((basesA).wrapping_offset(((i) as i32) as isize * 160));
                            sbFlags = ((((sbFlags) as i32) | 1i32) as u8);
                        }
                    }
                    if !((((sbFlags) as i32) & 2i32) != 0) {
                        if ((SecretBaseBelongsToPlayer(
                            (basesB).wrapping_offset(((i) as i32) as isize * 160),
                        )) as i32)
                            == 1i32
                        {
                            ClearSecretBase((basesB).wrapping_offset(((i) as i32) as isize * 160));
                            sbFlags = ((((sbFlags) as i32) | 2i32) as u8);
                        }
                    }
                    if !((((sbFlags) as i32) & 4i32) != 0) {
                        if ((SecretBaseBelongsToPlayer(
                            (basesC).wrapping_offset(((i) as i32) as isize * 160),
                        )) as i32)
                            == 1i32
                        {
                            ClearSecretBase((basesC).wrapping_offset(((i) as i32) as isize * 160));
                            sbFlags = ((((sbFlags) as i32) | 4i32) as u8);
                        }
                    }
                    if ((sbFlags) as i32) == 7i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClearDuplicateOwnedSecretBase(
    secretBase: *mut u8,
    secretBases: *mut u8,
    idx: u8,
) -> u8 {
    unsafe {
        let mut secretBase = secretBase;
        let mut secretBases = secretBases;
        let mut idx = idx;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((secretBases).wrapping_offset(((i) as i32) as isize * 160)).read())
                        as i32)
                        != 0i32
                    {
                        if ((SecretBasesBelongToSamePlayer(
                            secretBase,
                            (secretBases).wrapping_offset(((i) as i32) as isize * 160),
                        )) as i32)
                            == 1i32
                        {
                            if ((idx) as i32) == 0i32 {
                                ClearSecretBase(
                                    (secretBases).wrapping_offset(((i) as i32) as isize * 160),
                                );
                                return 0u8;
                            }
                            if ((((secretBase).wrapping_add(14).cast::<u16>()).read()) as i32)
                                > (((((secretBases).wrapping_offset(((i) as i32) as isize * 160))
                                    .wrapping_add(14)
                                    .cast::<u16>())
                                .read()) as i32)
                            {
                                ClearSecretBase(
                                    (secretBases).wrapping_offset(((i) as i32) as isize * 160),
                                );
                                return 0u8;
                            }
                            crate::c::bf_write(
                                ((secretBases).wrapping_offset(((i) as i32) as isize * 160))
                                    .wrapping_add(1),
                                0,
                                4,
                                (crate::c::bf_read((secretBase).wrapping_add(1), 0, 4, false) as u8)
                                    as i32,
                            );
                            ClearSecretBase(secretBase);
                            return 1u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ClearDuplicateOwnedSecretBases(
    playersBases: *mut u8,
    friendsBasesA: *mut u8,
    friendsBasesB: *mut u8,
    friendsBasesC: *mut u8,
) {
    unsafe {
        let mut playersBases = playersBases;
        let mut friendsBasesA = friendsBasesA;
        let mut friendsBasesB = friendsBasesB;
        let mut friendsBasesC = friendsBasesC;
        let mut i: u8 = 0u8;
        {
            i = 1u8;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    if (((playersBases).wrapping_offset(((i) as i32) as isize * 160)).read()) != 0 {
                        if ((crate::c::bf_read(
                            ((playersBases).wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                            6,
                            2,
                            false,
                        ) as u8) as i32)
                            == 1i32
                        {
                            crate::c::bf_write(
                                ((playersBases).wrapping_offset(((i) as i32) as isize * 160))
                                    .wrapping_add(1),
                                0,
                                4,
                                (1u8) as i32,
                            );
                        }
                        if !((ClearDuplicateOwnedSecretBase(
                            (playersBases).wrapping_offset(((i) as i32) as isize * 160),
                            friendsBasesA,
                            i,
                        )) != 0)
                        {
                            if !((ClearDuplicateOwnedSecretBase(
                                (playersBases).wrapping_offset(((i) as i32) as isize * 160),
                                friendsBasesB,
                                i,
                            )) != 0)
                            {
                                ClearDuplicateOwnedSecretBase(
                                    (playersBases).wrapping_offset(((i) as i32) as isize * 160),
                                    friendsBasesC,
                                    i,
                                );
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l3;
                }
                'l4: {
                    if (((friendsBasesA).wrapping_offset(((i) as i32) as isize * 160)).read()) != 0
                    {
                        crate::c::bf_write(
                            ((friendsBasesA).wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                            5,
                            1,
                            (0u8) as i32,
                        );
                        if !((ClearDuplicateOwnedSecretBase(
                            (friendsBasesA).wrapping_offset(((i) as i32) as isize * 160),
                            friendsBasesB,
                            i,
                        )) != 0)
                        {
                            ClearDuplicateOwnedSecretBase(
                                (friendsBasesA).wrapping_offset(((i) as i32) as isize * 160),
                                friendsBasesC,
                                i,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l5: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l5;
                }
                'l6: {
                    if (((friendsBasesB).wrapping_offset(((i) as i32) as isize * 160)).read()) != 0
                    {
                        crate::c::bf_write(
                            ((friendsBasesB).wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                            5,
                            1,
                            (0u8) as i32,
                        );
                        ClearDuplicateOwnedSecretBase(
                            (friendsBasesB).wrapping_offset(((i) as i32) as isize * 160),
                            friendsBasesC,
                            i,
                        );
                    }
                    if (((friendsBasesC).wrapping_offset(((i) as i32) as isize * 160)).read()) != 0
                    {
                        crate::c::bf_write(
                            ((friendsBasesC).wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                            5,
                            1,
                            (0u8) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrySaveRegisteredDuplicate(
    base: *mut u8,
    version: u32,
    language: u32,
) {
    unsafe {
        let mut base = base;
        let mut version = version;
        let mut language = language;
        if ((crate::c::bf_read((base).wrapping_add(1), 0, 4, false) as u8) as i32) == 1i32 {
            TrySaveFriendsSecretBase(base, version, language);
            ClearSecretBase(base);
        }
    }
}
pub(crate) unsafe extern "C" fn TrySaveRegisteredDuplicates(mixers: *mut u8) {
    unsafe {
        let mut mixers = mixers;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    TrySaveRegisteredDuplicate(
                        (((mixers).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 160),
                        ((mixers).wrapping_add(4).cast::<u32>()).read(),
                        ((mixers).wrapping_add(8).cast::<u32>()).read(),
                    );
                    TrySaveRegisteredDuplicate(
                        ((((mixers).wrapping_offset(12)).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 160),
                        (((mixers).wrapping_offset(12)).wrapping_add(4).cast::<u32>()).read(),
                        (((mixers).wrapping_offset(12)).wrapping_add(8).cast::<u32>()).read(),
                    );
                    TrySaveRegisteredDuplicate(
                        ((((mixers).wrapping_offset(24)).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 160),
                        (((mixers).wrapping_offset(24)).wrapping_add(4).cast::<u32>()).read(),
                        (((mixers).wrapping_offset(24)).wrapping_add(8).cast::<u32>()).read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SaveRecordMixBases(mixers: *mut u8) {
    unsafe {
        let mut mixers = mixers;
        DeleteFirstOldBaseFromPlayerInRecordMixingFriendsRecords(
            ((mixers).cast::<*mut u8>()).read(),
            (((mixers).wrapping_offset(12)).cast::<*mut u8>()).read(),
            (((mixers).wrapping_offset(24)).cast::<*mut u8>()).read(),
        );
        ClearDuplicateOwnedSecretBases(
            ((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>(),
            ((mixers).cast::<*mut u8>()).read(),
            (((mixers).wrapping_offset(12)).cast::<*mut u8>()).read(),
            (((mixers).wrapping_offset(24)).cast::<*mut u8>()).read(),
        );
        TrySaveRegisteredDuplicates(mixers);
        TrySaveFriendsSecretBase(
            ((mixers).cast::<*mut u8>()).read(),
            ((mixers).wrapping_add(4).cast::<u32>()).read(),
            ((mixers).wrapping_add(8).cast::<u32>()).read(),
        );
        TrySaveFriendsSecretBase(
            (((mixers).wrapping_offset(12)).cast::<*mut u8>()).read(),
            (((mixers).wrapping_offset(12)).wrapping_add(4).cast::<u32>()).read(),
            (((mixers).wrapping_offset(12)).wrapping_add(8).cast::<u32>()).read(),
        );
        TrySaveFriendsSecretBase(
            (((mixers).wrapping_offset(24)).cast::<*mut u8>()).read(),
            (((mixers).wrapping_offset(24)).wrapping_add(4).cast::<u32>()).read(),
            (((mixers).wrapping_offset(24)).wrapping_add(8).cast::<u32>()).read(),
        );
        TrySaveFriendsSecretBases(mixers, 1u8);
        TrySaveFriendsSecretBases((mixers).wrapping_offset(12), 1u8);
        TrySaveFriendsSecretBases((mixers).wrapping_offset(24), 1u8);
        TrySaveFriendsSecretBases(mixers, 0u8);
        TrySaveFriendsSecretBases((mixers).wrapping_offset(12), 0u8);
        TrySaveFriendsSecretBases((mixers).wrapping_offset(24), 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ReceiveSecretBasesData(
    secretBases: *mut u8,
    recordSize: u32,
    linkIdx: u8,
) {
    unsafe {
        let mut secretBases = secretBases;
        let mut recordSize = recordSize;
        let mut linkIdx = linkIdx;
        let mut mixers = crate::ffi::Align4([0u8; 36]);
        let mut i: u16 = 0u16;
        if (FlagGet(96u16)) != 0 {
            'l1: {
                let __sw1 = ((GetLinkPlayerCount()) as i32);
                if __sw1 == 2i32 {
                    crate::c::memset(
                        (secretBases).wrapping_offset(
                            (((2u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                        0i32,
                        recordSize,
                    );
                    crate::c::memset(
                        (secretBases).wrapping_offset(
                            (((3u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                        0i32,
                        recordSize,
                    );
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    crate::c::memset(
                        (secretBases).wrapping_offset(
                            (((3u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                        0i32,
                        recordSize,
                    );
                    break 'l1;
                }
            }
            'l2: {
                let __sw2 = ((linkIdx) as i32);
                if __sw2 == 0i32 {
                    (((&raw mut mixers).cast::<u8>()).cast::<*mut u8>()).write(
                        (secretBases).wrapping_offset(
                            (((1u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((2u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((3u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    break 'l2;
                }
                if __sw2 == 1i32 {
                    (((&raw mut mixers).cast::<u8>()).cast::<*mut u8>()).write(
                        (secretBases).wrapping_offset(
                            (((2u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((3u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((0u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        (((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u16>()).read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        (((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    break 'l2;
                }
                if __sw2 == 2i32 {
                    (((&raw mut mixers).cast::<u8>()).cast::<*mut u8>()).write(
                        (secretBases).wrapping_offset(
                            (((3u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(84))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((0u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        (((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u16>()).read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        (((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((1u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    break 'l2;
                }
                if __sw2 == 3i32 {
                    (((&raw mut mixers).cast::<u8>()).cast::<*mut u8>()).write(
                        (secretBases).wrapping_offset(
                            (((0u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        (((((((&raw mut gLinkPlayers).cast::<u8>()).cast::<u16>()).read()) as i32)
                            & 255i32) as u32),
                    );
                    (((&raw mut mixers).cast::<u8>())
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        (((((&raw mut gLinkPlayers).cast::<u8>())
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((1u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(12))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(28))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24)).cast::<*mut u8>())
                        .write((secretBases).wrapping_offset(
                            (((2u32).wrapping_mul(recordSize)) as i32) as isize * 1,
                        ));
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(4)
                        .cast::<u32>())
                    .write(
                        ((((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .cast::<u16>())
                        .read()) as i32)
                            & 255i32) as u32),
                    );
                    ((((&raw mut mixers).cast::<u8>()).wrapping_offset(24))
                        .wrapping_add(8)
                        .cast::<u32>())
                    .write(
                        ((((((&raw mut gLinkPlayers).cast::<u8>()).wrapping_offset(56))
                            .wrapping_add(26)
                            .cast::<u16>())
                        .read()) as u32),
                    );
                    break 'l2;
                }
            }
            SaveRecordMixBases((&raw mut mixers).cast::<u8>());
            {
                i = 1u16;
                'l3: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l3;
                    }
                    'l4: {
                        if ((crate::c::bf_read(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(6812))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 160))
                            .wrapping_add(1),
                            0,
                            4,
                            false,
                        ) as u8) as i32)
                            == 1i32
                        {
                            crate::c::bf_write(
                                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(6812))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                                6,
                                2,
                                (1u8) as i32,
                            );
                            crate::c::bf_write(
                                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(6812))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                                0,
                                4,
                                (0u8) as i32,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            SortSecretBasesByRegistryStatus();
            {
                i = 1u16;
                'l5: loop {
                    if !(((i) as i32) < 20i32) {
                        break 'l5;
                    }
                    'l6: {
                        if ((crate::c::bf_read(
                            ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                .wrapping_add(6812))
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 160))
                            .wrapping_add(1),
                            6,
                            2,
                            false,
                        ) as u8) as i32)
                            == 2i32
                        {
                            crate::c::bf_write(
                                ((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                                    .wrapping_add(6812))
                                .cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 160))
                                .wrapping_add(1),
                                6,
                                2,
                                (0u8) as i32,
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(6812))
                .cast::<u8>())
            .read()) as i32)
                != 0i32)
                && (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(6812))
                .cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>())
                .read()) as i32)
                    != 65535i32)
            {
                let __p3 = (((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
                    .wrapping_add(6812))
                .cast::<u8>())
                .wrapping_add(14)
                .cast::<u16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearJapaneseSecretBases(bases: *mut u8) {
    unsafe {
        let mut bases = bases;
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < 20u32) {
                    break 'l1;
                }
                'l2: {
                    if (((((bases).wrapping_offset(((i) as i32) as isize * 160)).wrapping_add(13))
                        .read()) as i32)
                        == 1i32
                    {
                        ClearSecretBase((bases).wrapping_offset(((i) as i32) as isize * 160));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitSecretBaseVars() {
    unsafe {
        VarSet(16620u16, 0u16);
        VarSet(16621u16, 0u16);
        VarSet(16622u16, 0u16);
        VarSet(16623u16, 0u16);
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(16624u16, 1u16);
        } else {
            VarSet(16624u16, 0u16);
        }
        ((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeftFriendsSecretBase() {
    unsafe {
        if (((VarGet(16624u16)) != 0)
            && (((((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).read()) as i32)
                == 1i32))
            && (!((CurMapIsSecretBase()) != 0))
        {
            VarSet(16624u16, 0u16);
            ((&raw mut sInFriendSecretBase).cast::<u8>().cast::<u8>()).write(0u8);
            TryPutSecretBaseSecretsOnAir();
            VarSet(16620u16, 0u16);
            VarSet(16621u16, 0u16);
            VarSet(16622u16, 0u16);
            VarSet(16623u16, 0u16);
            VarSet(16624u16, 0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsDollDecor() {
    unsafe {
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 2048i32) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsCushionDecor() {
    unsafe {
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 1024i32) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DeclinedSecretBaseBattle() {
    unsafe {
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(
                16622u16,
                ((((VarGet(16622u16)) as i32) & (-14337i32)) as u16),
            );
            VarSet(16623u16, ((((VarGet(16623u16)) as i32) & (-2i32)) as u16));
            VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 8192i32) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn WonSecretBaseBattle() {
    unsafe {
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(
                16622u16,
                ((((VarGet(16622u16)) as i32) & (-14337i32)) as u16),
            );
            VarSet(16623u16, ((((VarGet(16623u16)) as i32) & (-2i32)) as u16));
            VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 2048i32) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LostSecretBaseBattle() {
    unsafe {
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(
                16622u16,
                ((((VarGet(16622u16)) as i32) & (-14337i32)) as u16),
            );
            VarSet(16623u16, ((((VarGet(16623u16)) as i32) & (-2i32)) as u16));
            VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 4096i32) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DrewSecretBaseBattle() {
    unsafe {
        if ((VarGet(16468u16)) as i32) != 0i32 {
            VarSet(
                16622u16,
                ((((VarGet(16622u16)) as i32) & (-14337i32)) as u16),
            );
            VarSet(16623u16, ((((VarGet(16623u16)) as i32) & (-2i32)) as u16));
            VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 1i32) as u16));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsPosterDecor() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
            if __sw1 == 796i32
                || __sw1 == 797i32
                || __sw1 == 798i32
                || __sw1 == 799i32
                || __sw1 == 804i32
                || __sw1 == 805i32
                || __sw1 == 806i32
                || __sw1 == 807i32
                || __sw1 == 812i32
                || __sw1 == 813i32
                || __sw1 == 816i32
                || __sw1 == 817i32
                || __sw1 == 818i32
                || __sw1 == 819i32
                || __sw1 == 820i32
            {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 16384i32) as u16));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsFurnitureBottom() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
            if __sw1 == 650i32 || __sw1 == 651i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 64i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 728i32
                || __sw1 == 729i32
                || __sw1 == 730i32
                || __sw1 == 731i32
                || __sw1 == 732i32
                || __sw1 == 733i32
                || __sw1 == 744i32
                || __sw1 == 745i32
                || __sw1 == 746i32
                || __sw1 == 747i32
                || __sw1 == 748i32
                || __sw1 == 749i32
                || __sw1 == 750i32
                || __sw1 == 751i32
                || __sw1 == 760i32
                || __sw1 == 761i32
                || __sw1 == 762i32
                || __sw1 == 763i32
            {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16622u16, ((((VarGet(16622u16)) as i32) | 8i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 556i32 || __sw1 == 563i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 64i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 648i32 || __sw1 == 649i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 256i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 557i32 || __sw1 == 558i32 || __sw1 == 559i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 16i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 647i32
                || __sw1 == 655i32
                || __sw1 == 664i32
                || __sw1 == 665i32
                || __sw1 == 666i32
                || __sw1 == 667i32
                || __sw1 == 668i32
                || __sw1 == 669i32
                || __sw1 == 670i32
                || __sw1 == 671i32
                || __sw1 == 683i32
                || __sw1 == 688i32
                || __sw1 == 689i32
                || __sw1 == 690i32
                || __sw1 == 692i32
                || __sw1 == 693i32
                || __sw1 == 694i32
                || __sw1 == 695i32
                || __sw1 == 715i32
                || __sw1 == 716i32
                || __sw1 == 717i32
                || __sw1 == 718i32
                || __sw1 == 719i32
            {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 8i32) as u16));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsFurnitureMiddle() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
            if __sw1 == 657i32
                || __sw1 == 660i32
                || __sw1 == 663i32
                || __sw1 == 673i32
                || __sw1 == 681i32
                || __sw1 == 677i32
                || __sw1 == 685i32
                || __sw1 == 699i32
                || __sw1 == 707i32
                || __sw1 == 702i32
                || __sw1 == 710i32
            {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 8i32) as u16));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsFurnitureTop() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
            if __sw1 == 656i32
                || __sw1 == 658i32
                || __sw1 == 659i32
                || __sw1 == 661i32
                || __sw1 == 662i32
                || __sw1 == 675i32
                || __sw1 == 672i32
                || __sw1 == 674i32
                || __sw1 == 680i32
                || __sw1 == 682i32
                || __sw1 == 676i32
                || __sw1 == 678i32
                || __sw1 == 684i32
                || __sw1 == 686i32
                || __sw1 == 679i32
                || __sw1 == 700i32
                || __sw1 == 687i32
                || __sw1 == 708i32
                || __sw1 == 701i32
                || __sw1 == 703i32
                || __sw1 == 709i32
                || __sw1 == 711i32
            {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 8i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 640i32 || __sw1 == 641i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 256i32) as u16));
                }
                break 'l1;
            }
            if __sw1 == 549i32 || __sw1 == 550i32 || __sw1 == 551i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 16i32) as u16));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckInteractedWithFriendsSandOrnament() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        'l1: {
            let __sw1 = MapGridGetMetatileIdAt(((x) as i32), ((y) as i32));
            if __sw1 == 653i32 || __sw1 == 654i32 {
                if ((VarGet(16468u16)) as i32) != 0i32 {
                    VarSet(16623u16, ((((VarGet(16623u16)) as i32) | 4i32) as u16));
                }
                break 'l1;
            }
        }
    }
}
