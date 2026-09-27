//! Translated from `src/item_use.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sItemUseCallbacks sClockwiseDirections sUseTMHMYesNoFuncTable
#[allow(unused_imports)]
use crate::data::item_use::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemUseOnFieldCB: Option<unsafe extern "C" fn(u8)> = None;

unsafe extern "C" {
    static mut BattleFrontier_OutsideEast_EventScript_WaterSudowoodo: u8;
    static mut BerryTree_EventScript_ItemUsePlantBerry: u8;
    static mut BerryTree_EventScript_ItemUseWailmerPail: u8;
    static mut gBagMenu: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gFieldCallback: u8;
    static mut gItemUseCB: u8;
    static mut gMain: u8;
    static mut gMapHeader: u8;
    static mut gMoveNames: u8;
    static mut gObjectEvents: u8;
    static mut gPaletteFade: u8;
    static mut gPlayerParty: u8;
    static mut gPyramidBagMenu: u8;
    static mut gSpecialVar_ItemId: u8;
    static mut gStringVar1: u8;
    static mut gStringVar2: u8;
    static mut gStringVar4: u8;
    static mut gTasks: u8;
    static mut gText_BootedUpHM: u8;
    static mut gText_BootedUpTM: u8;
    static mut gText_BoxFull: u8;
    static mut gText_CantDismountBike: u8;
    static mut gText_CoinCase: u8;
    static mut gText_DadsAdvice: u8;
    static mut gText_ItemFinderNearby: u8;
    static mut gText_ItemFinderNothing: u8;
    static mut gText_ItemFinderOnTop: u8;
    static mut gText_PlayerUsedVar2: u8;
    static mut gText_PowderQty: u8;
    static mut gText_RepelEffectsLingered: u8;
    static mut gText_TMHMContainedVar1: u8;
    static mut gText_UsedVar2WildLured: u8;
    static mut gText_UsedVar2WildRepelled: u8;
    static mut gText_WontHaveEffect: u8;
    fn BagMenu_YesNo(a0: u8, a1: u8, a2: *mut u8);
    fn CB2_ReturnToBagMenuPocket();
    fn CB2_ReturnToField();
    fn ChooseMonForInBattleItem();
    fn CleanupOverworldWindowsAndTilemaps();
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn CloseBattlePyramidBag(a0: u8);
    fn CloseItemMessage(a0: u8);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn CopyItemName(a0: u16, a1: *mut u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn DisplayItemMessage(a0: u8, a1: u8, a2: *mut u8, a3: Option<unsafe extern "C" fn(u8)>);
    fn DisplayItemMessageInBattlePyramid(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn DisplayItemMessageOnField(a0: u8, a1: *mut u8, a2: Option<unsafe extern "C" fn(u8)>);
    fn ExecuteTableBasedItemEffect(a0: *mut u8, a1: u16, a2: u8, a3: u8) -> u8;
    fn FadeInFromBlack();
    fn FadeScreen(a0: u8, a1: i8);
    fn FieldCB_ReturnToFieldNoScript();
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn GetBerryPowder() -> u32;
    fn GetCoins() -> u16;
    fn GetItemEffectType(a0: u16) -> u8;
    fn GetItemFieldFunc(a0: u16) -> Option<unsafe extern "C" fn(u8)>;
    fn GetItemHoldEffectParam(a0: u16) -> u8;
    fn GetItemPocket(a0: u16) -> u8;
    fn GetItemSecondaryId(a0: u16) -> u8;
    fn GetItemType(a0: u16) -> u8;
    fn GetMapConnectionAtPos(a0: i16, a1: i16) -> *mut u8;
    fn GetMapHeaderFromConnection(a0: *mut u8) -> *mut u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetObjectEventIdByPosition(a0: u16, a1: u16, a2: u8) -> u8;
    fn GetOnOffBike(a0: u8);
    fn GetPlayerFacingDirection() -> u8;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn IncrementGameStat(a0: u8);
    fn IsBikingDisallowedByPlayer() -> u8;
    fn IsPlayerFacingEmptyBerryTreePatch() -> u8;
    fn IsPlayerFacingSurfableFishableWater() -> u8;
    fn IsPlayerPartyAndPokemonStorageFull() -> u8;
    fn IsSEPlaying() -> u8;
    fn IsWeatherNotFadingIn() -> u8;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn ItemUseCB_EvolutionStone(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_Medicine(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_PPRecovery(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_PPUp(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_RareCandy(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_ReduceEV(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_SacredAsh(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn ItemUseCB_TMHM(a0: u8, a1: Option<unsafe extern "C" fn(u8)>);
    fn LockPlayerFieldControls();
    fn MapGridGetCollisionAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MenuHelpers_IsLinkActive() -> u8;
    fn MetatileBehavior_IsBridgeOverWaterNoEdge(a0: u8) -> u8;
    fn MetatileBehavior_IsHorizontalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsIsolatedHorizontalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsIsolatedVerticalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableWaterOrUnderwater(a0: u8) -> u8;
    fn MetatileBehavior_IsVerticalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsWaterfall(a0: u8) -> u8;
    fn ObjectEventCheckHeldMovementStatus(a0: *mut u8) -> u8;
    fn ObjectEventClearHeldMovement(a0: *mut u8);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn OpenPokeblockCase(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn Overworld_IsBikingAllowed() -> u32;
    fn Overworld_ResetStateAfterDigEscRope();
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerGetElevation() -> u8;
    fn PlayerTurnInPlace(a0: u8);
    fn ReadMail(a0: *mut u8, a1: Option<unsafe extern "C" fn()>, a2: u8);
    fn RemoveBagItem(a0: u16, a1: u16) -> u8;
    fn ResetInitialPlayerAvatarState();
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn ScriptUnfreezeObjectEvents();
    fn StartEscapeRopeFieldEffect();
    fn StartFishing(a0: u8);
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringExpandPlaceholders(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn Task_CloseBattlePyramidBagMessage(a0: u8);
    fn Task_FadeAndCloseBagMenu(a0: u8);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TryToWaterBerryTree() -> u8;
    fn UnfreezeObjectEvent(a0: *mut u8);
    fn UnlockPlayerFieldControls();
    fn UpdatePocketItemList(a0: u8);
    fn UpdatePocketListPosition(a0: u8);
    fn UpdatePyramidBagCursorPos();
    fn UpdatePyramidBagList();
    fn UseStatIncreaseItem(a0: u16) -> *mut u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn SetUpItemUseCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut r#type: u8 = 0u8;
        if ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) == 175i32 {
            r#type = ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as i32)
                .wrapping_sub(1i32)) as u8);
        } else {
            r#type = ((((GetItemType(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32)
                .wrapping_sub(1i32)) as u8);
        }
        if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
            ((((&raw mut gBagMenu).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(
                ((((&raw const sItemUseCallbacks)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(((r#type) as i32) as isize))
                .read(),
            );
            Task_FadeAndCloseBagMenu(taskId);
        } else {
            ((((&raw mut gPyramidBagMenu).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(
                ((((&raw const sItemUseCallbacks)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn()>>())
                .cast::<Option<unsafe extern "C" fn()>>())
                .wrapping_offset(((r#type) as i32) as isize))
                .read(),
            );
            CloseBattlePyramidBag(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn SetUpItemUseOnFieldCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            != 1i32
        {
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_UseItemOnField));
            SetUpItemUseCallback(taskId);
        } else {
            (((&raw mut sItemUseOnFieldCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read())
            .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn FieldCB_UseItemOnField() {
    unsafe {
        FadeInFromBlack();
        CreateTask(Some(Task_CallItemUseOnFieldCallback), 8u8);
    }
}
pub(crate) unsafe extern "C" fn Task_CallItemUseOnFieldCallback(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsWeatherNotFadingIn()) as i32) == 1i32 {
            (((&raw mut sItemUseOnFieldCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .read())
            .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayCannotUseItemMessage(
    taskId: u8,
    isUsingRegisteredKeyItemOnField: u8,
    str: *mut u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut isUsingRegisteredKeyItemOnField = isUsingRegisteredKeyItemOnField;
        let mut str = str;
        StringExpandPlaceholders((&raw mut gStringVar4).cast::<u8>(), str);
        if !((isUsingRegisteredKeyItemOnField) != 0) {
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(CloseItemMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gText_DadsAdvice).cast::<u8>(),
                    Some(Task_CloseBattlePyramidBagMessage),
                );
            }
        } else {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(Task_CloseCantUseKeyItemMessage),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn DisplayDadsAdviceCannotUseItemMessage(
    taskId: u8,
    isUsingRegisteredKeyItemOnField: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut isUsingRegisteredKeyItemOnField = isUsingRegisteredKeyItemOnField;
        DisplayCannotUseItemMessage(
            taskId,
            isUsingRegisteredKeyItemOnField,
            (&raw mut gText_DadsAdvice).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayCannotDismountBikeMessage(
    taskId: u8,
    isUsingRegisteredKeyItemOnField: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut isUsingRegisteredKeyItemOnField = isUsingRegisteredKeyItemOnField;
        DisplayCannotUseItemMessage(
            taskId,
            isUsingRegisteredKeyItemOnField,
            (&raw mut gText_CantDismountBike).cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_CloseCantUseKeyItemMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 1u8);
        DestroyTask(taskId);
        ScriptUnfreezeObjectEvents();
        UnlockPlayerFieldControls();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckIfItemIsTMHMOrEvolutionStone(itemId: u16) -> u8 {
    unsafe {
        let mut itemId = itemId;
        if core::mem::transmute::<_, usize>(GetItemFieldFunc(itemId))
            == (ItemUseOutOfBattle_TMHM as *const () as usize)
        {
            return 1u8;
        } else {
            if core::mem::transmute::<_, usize>(GetItemFieldFunc(itemId))
                == (ItemUseOutOfBattle_EvolutionStone as *const () as usize)
            {
                return 2u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_CheckMail() {
    unsafe {
        let mut mail = crate::ffi::Align4([0u8; 36]);
        (((&raw mut mail).cast::<u8>())
            .wrapping_add(32)
            .cast::<u16>())
        .write(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read());
        ReadMail(
            (&raw mut mail).cast::<u8>(),
            Some(CB2_ReturnToBagMenuPocket),
            0u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Mail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBagMenu).cast::<*mut u8>()).read()).cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_CheckMail));
        Task_FadeAndCloseBagMenu(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Bike(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut coordsY: i16 = 0i16;
        let mut coordsX: i16 = 0i16;
        let mut behavior: u8 = 0u8;
        PlayerGetDestCoords(&raw mut coordsX, &raw mut coordsY);
        behavior = ((MapGridGetMetatileBehaviorAt(((coordsX) as i32), ((coordsY) as i32))) as u8);
        if ((((((FlagGet(2187u16)) as i32) == 1i32)
            || (((MetatileBehavior_IsVerticalRail(behavior)) as i32) == 1i32))
            || (((MetatileBehavior_IsHorizontalRail(behavior)) as i32) == 1i32))
            || (((MetatileBehavior_IsIsolatedVerticalRail(behavior)) as i32) == 1i32))
            || (((MetatileBehavior_IsIsolatedHorizontalRail(behavior)) as i32) == 1i32)
        {
            DisplayCannotDismountBikeMessage(taskId, ((((data).wrapping_offset(3)).read()) as u8));
        } else {
            if (Overworld_IsBikingAllowed() == 1u32)
                && (((IsBikingDisallowedByPlayer()) as i32) == 0i32)
            {
                ((&raw mut sItemUseOnFieldCB)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(ItemUseOnFieldCB_Bike));
                SetUpItemUseOnFieldCallback(taskId);
            } else {
                DisplayDadsAdviceCannotUseItemMessage(
                    taskId,
                    ((((data).wrapping_offset(3)).read()) as u8),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Bike(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetItemSecondaryId(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32)
            == 0i32
        {
            GetOnOffBike(2u8);
        } else {
            GetOnOffBike(4u8);
        }
        ScriptUnfreezeObjectEvents();
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn CanFish() -> u32 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut tileBehavior: u16 = 0u16;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        tileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u16);
        if (MetatileBehavior_IsWaterfall(((tileBehavior) as u8))) != 0 {
            return 0u32;
        }
        if (TestPlayerAvatarFlags(16u8)) != 0 {
            return 0u32;
        }
        if !((TestPlayerAvatarFlags(8u8)) != 0) {
            if (IsPlayerFacingSurfableFishableWater()) != 0 {
                return 1u32;
            }
        } else {
            if ((MetatileBehavior_IsSurfableWaterOrUnderwater(((tileBehavior) as u8))) != 0)
                && (((MapGridGetCollisionAt(((x) as i32), ((y) as i32))) as i32) == 0i32)
            {
                return 1u32;
            }
            if ((MetatileBehavior_IsBridgeOverWaterNoEdge(((tileBehavior) as u8))) as i32) == 1i32 {
                return 1u32;
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Rod(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if CanFish() == 1u32 {
            ((&raw mut sItemUseOnFieldCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemUseOnFieldCB_Rod));
            SetUpItemUseOnFieldCallback(taskId);
        } else {
            DisplayDadsAdviceCannotUseItemMessage(
                taskId,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Rod(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        StartFishing(GetItemSecondaryId(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
        ));
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Itemfinder(var: u8) {
    unsafe {
        let mut var = var;
        IncrementGameStat(39u8);
        ((&raw mut sItemUseOnFieldCB)
            .cast::<u8>()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(ItemUseOnFieldCB_Itemfinder));
        SetUpItemUseOnFieldCallback(var);
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Itemfinder(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((ItemfinderCheckForHiddenItems(
            (((&raw mut gMapHeader).cast::<u8>())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read(),
            taskId,
        )) as i32)
            == 1i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UseItemfinder));
        } else {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_ItemFinderNothing).cast::<u8>(),
                Some(Task_CloseItemfinderMessage),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UseItemfinder(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerDir: u8 = 0u8;
        let mut playerDirToItem: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if ((((data).wrapping_offset(3)).read()) as i32) == 0i32 {
            if ((((data).wrapping_offset(4)).read()) as i32) == 4i32 {
                playerDirToItem =
                    GetDirectionToHiddenItem((data).read(), ((data).wrapping_offset(1)).read());
                if ((playerDirToItem) as i32) != 0i32 {
                    PlayerFaceHiddenItem(
                        ((((&raw const sClockwiseDirections).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((((playerDirToItem) as i32).wrapping_sub(1i32)) as isize))
                        .read(),
                    );
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_HiddenItemNearby));
                } else {
                    playerDir = GetPlayerFacingDirection();
                    {
                        i = 0u8;
                        'l1: loop {
                            if !(((i) as u32) < crate::c::div_u32(4u32, 1u32)) {
                                break 'l1;
                            }
                            'l2: {
                                if ((playerDir) as i32)
                                    == ((((((&raw const sClockwiseDirections)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32)
                                {
                                    ((data).wrapping_offset(5))
                                        .write(((((i) as i32).wrapping_add(1i32) & 3i32) as i16));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(Task_StandingOnHiddenItem));
                    ((data).wrapping_offset(3)).write(0i16);
                    ((data).wrapping_offset(2)).write(0i16);
                }
                return;
            }
            PlaySE(72u16);
            let __p1 = (data).wrapping_offset(4);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        ((data).wrapping_offset(3)).write(
            ((((((data).wrapping_offset(3)).read()) as i32).wrapping_add(1i32) & 31i32) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_CloseItemfinderMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ClearDialogWindowAndFrame(0u8, 1u8);
        ScriptUnfreezeObjectEvents();
        UnlockPlayerFieldControls();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemfinderCheckForHiddenItems(events: *mut u8, taskId: u8) -> u8 {
    unsafe {
        let mut events = events;
        let mut taskId = taskId;
        let mut playerX: i16 = 0i16;
        let mut playerY: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut distanceX: i16 = 0i16;
        let mut distanceY: i16 = 0i16;
        PlayerGetDestCoords(&raw mut playerX, &raw mut playerY);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < ((((events).wrapping_add(3)).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                        .wrapping_offset(((i) as i32) as isize * 12))
                    .wrapping_add(5))
                    .read()) as i32)
                        == 7i32)
                        && (!((FlagGet(
                            ((((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                                .wrapping_offset(((i) as i32) as isize * 12))
                            .wrapping_add(8))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(500i32)) as u16),
                        )) != 0))
                    {
                        distanceX = ((((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 12))
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(7i32))
                        .wrapping_sub(((playerX) as i32)))
                            as i16);
                        distanceY = ((((((((((events).wrapping_add(16).cast::<*mut u8>()).read())
                            .wrapping_offset(((i) as i32) as isize * 12))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            .wrapping_add(7i32))
                        .wrapping_sub(((playerY) as i32)))
                            as i16);
                        if (((((distanceX) as i32) >= (-7i32)) && (((distanceX) as i32) <= 7i32))
                            && (((distanceY) as i32) >= (-5i32)))
                            && (((distanceY) as i32) <= 5i32)
                        {
                            SetDistanceOfClosestHiddenItem(taskId, distanceX, distanceY);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        CheckForHiddenItemsInMapConnection(taskId);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == 1i32
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
pub(crate) unsafe extern "C" fn IsHiddenItemPresentAtCoords(events: *mut u8, x: i16, y: i16) -> u8 {
    unsafe {
        let mut events = events;
        let mut x = x;
        let mut y = y;
        let mut bgEventCount: u8 = ((events).wrapping_add(3)).read();
        let mut bgEvent: *mut u8 = ((events).wrapping_add(16).cast::<*mut u8>()).read();
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < ((bgEventCount) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((((bgEvent).wrapping_offset((i) as isize * 12)).wrapping_add(5)).read())
                        as i32)
                        == 7i32)
                        && (((x) as i32)
                            == (((((bgEvent).wrapping_offset((i) as isize * 12)).cast::<u16>())
                                .read()) as i32)))
                        && (((y) as i32)
                            == (((((bgEvent).wrapping_offset((i) as isize * 12))
                                .wrapping_add(2)
                                .cast::<u16>())
                            .read()) as i32))
                    {
                        if !((FlagGet(
                            ((((((((bgEvent).wrapping_offset((i) as isize * 12)).wrapping_add(8))
                                .wrapping_add(2)
                                .cast::<u16>())
                            .read()) as i32)
                                .wrapping_add(500i32)) as u16),
                        )) != 0)
                        {
                            return 1u8;
                        } else {
                            return 0u8;
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn IsHiddenItemPresentInConnection(
    connection: *mut u8,
    x: i32,
    y: i32,
) -> u8 {
    unsafe {
        let mut connection = connection;
        let mut x = x;
        let mut y = y;
        let mut connectionX: i16 = 0i16;
        let mut connectionY: i16 = 0i16;
        let mut connectionHeader: *mut u8 = GetMapHeaderFromConnection(connection);
        'l1: {
            let __sw1 = (((connection).read()) as i32);
            let __matched = __sw1 == 2i32 || __sw1 == 1i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 2i32 {
                connectionX = ((((x).wrapping_sub(7i32))
                    .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                    as i16);
                connectionY = (((((((connectionHeader).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<i32>())
                .read())
                .wrapping_add((y).wrapping_sub(7i32))) as i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                connectionX = ((((x).wrapping_sub(7i32))
                    .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                    as i16);
                connectionY = ((((y).wrapping_sub(7i32)).wrapping_sub(
                    (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<i32>())
                    .read(),
                )) as i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                connectionX = (((((((connectionHeader).cast::<*mut u8>()).read()).cast::<i32>())
                    .read())
                .wrapping_add((x).wrapping_sub(7i32))) as i16);
                connectionY = ((((y).wrapping_sub(7i32))
                    .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                    as i16);
                break 'l1;
            }
            if __sw1 == 4i32 {
                connectionX = ((((x).wrapping_sub(7i32)).wrapping_sub(
                    (((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read())
                        .cast::<i32>())
                    .read(),
                )) as i16);
                connectionY = ((((y).wrapping_sub(7i32))
                    .wrapping_sub(((connection).wrapping_add(4).cast::<i32>()).read()))
                    as i16);
                break 'l1;
            }
            if !__matched {
                return 0u8;
            }
        }
        return IsHiddenItemPresentAtCoords(
            ((connectionHeader).wrapping_add(4).cast::<*mut u8>()).read(),
            connectionX,
            connectionY,
        );
    }
}
pub(crate) unsafe extern "C" fn CheckForHiddenItemsInMapConnection(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerX: i16 = 0i16;
        let mut playerY: i16 = 0i16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut width: i16 =
            ((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>()).read()).cast::<i32>())
                .read())
            .wrapping_add(7i32)) as i16);
        let mut height: i16 = ((((((((&raw mut gMapHeader).cast::<u8>()).cast::<*mut u8>())
            .read())
        .wrapping_add(4)
        .cast::<i32>())
        .read())
        .wrapping_add(7i32)) as i16);
        let mut var1: i16 = 7i16;
        let mut var2: i16 = 7i16;
        PlayerGetDestCoords(&raw mut playerX, &raw mut playerY);
        {
            x = ((((playerX) as i32).wrapping_sub(7i32)) as i16);
            'l1: loop {
                if !(((x) as i32) <= ((playerX) as i32).wrapping_add(7i32)) {
                    break 'l1;
                }
                'l2: {
                    {
                        y = ((((playerY) as i32).wrapping_sub(5i32)) as i16);
                        'l3: loop {
                            if !(((y) as i32) <= ((playerY) as i32).wrapping_add(5i32)) {
                                break 'l3;
                            }
                            'l4: {
                                if (((((var1) as i32) > ((x) as i32))
                                    || (((x) as i32) >= ((width) as i32)))
                                    || (((var2) as i32) > ((y) as i32)))
                                    || (((y) as i32) >= ((height) as i32))
                                {
                                    let mut conn: *mut u8 = GetMapConnectionAtPos(x, y);
                                    if (!(conn).is_null())
                                        && (((IsHiddenItemPresentInConnection(
                                            conn,
                                            ((x) as i32),
                                            ((y) as i32),
                                        )) as i32)
                                            == 1i32)
                                    {
                                        SetDistanceOfClosestHiddenItem(
                                            taskId,
                                            ((((x) as i32).wrapping_sub(((playerX) as i32)))
                                                as i16),
                                            ((((y) as i32).wrapping_sub(((playerY) as i32)))
                                                as i16),
                                        );
                                    }
                                }
                            }
                            y = (y).wrapping_add(1);
                        }
                    }
                }
                x = (x).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDistanceOfClosestHiddenItem(
    taskId: u8,
    itemDistanceX: i16,
    itemDistanceY: i16,
) {
    unsafe {
        let mut taskId = taskId;
        let mut itemDistanceX = itemDistanceX;
        let mut itemDistanceY = itemDistanceY;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        let mut oldItemAbsX: i16 = 0i16;
        let mut oldItemAbsY: i16 = 0i16;
        let mut newItemAbsX: i16 = 0i16;
        let mut newItemAbsY: i16 = 0i16;
        if ((((data).wrapping_offset(2)).read()) as i32) == 0i32 {
            (data).write(itemDistanceX);
            ((data).wrapping_offset(1)).write(itemDistanceY);
            ((data).wrapping_offset(2)).write(1i16);
            return;
        }
        if (((data).read()) as i32) < 0i32 {
            oldItemAbsX = (((((data).read()) as i32).wrapping_mul((-1i32))) as i16);
        } else {
            oldItemAbsX = (data).read();
        }
        if ((((data).wrapping_offset(1)).read()) as i32) < 0i32 {
            oldItemAbsY =
                ((((((data).wrapping_offset(1)).read()) as i32).wrapping_mul((-1i32))) as i16);
        } else {
            oldItemAbsY = ((data).wrapping_offset(1)).read();
        }
        if ((itemDistanceX) as i32) < 0i32 {
            newItemAbsX = ((((itemDistanceX) as i32).wrapping_mul((-1i32))) as i16);
        } else {
            newItemAbsX = itemDistanceX;
        }
        if ((itemDistanceY) as i32) < 0i32 {
            newItemAbsY = ((((itemDistanceY) as i32).wrapping_mul((-1i32))) as i16);
        } else {
            newItemAbsY = itemDistanceY;
        }
        if ((oldItemAbsX) as i32).wrapping_add(((oldItemAbsY) as i32))
            > ((newItemAbsX) as i32).wrapping_add(((newItemAbsY) as i32))
        {
            (data).write(itemDistanceX);
            ((data).wrapping_offset(1)).write(itemDistanceY);
        } else {
            if (((oldItemAbsX) as i32).wrapping_add(((oldItemAbsY) as i32))
                == ((newItemAbsX) as i32).wrapping_add(((newItemAbsY) as i32)))
                && ((((oldItemAbsY) as i32) > ((newItemAbsY) as i32))
                    || ((((oldItemAbsY) as i32) == ((newItemAbsY) as i32))
                        && (((((data).wrapping_offset(1)).read()) as i32)
                            < ((itemDistanceY) as i32))))
            {
                (data).write(itemDistanceX);
                ((data).wrapping_offset(1)).write(itemDistanceY);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetDirectionToHiddenItem(
    itemDistanceX: i16,
    itemDistanceY: i16,
) -> u8 {
    unsafe {
        let mut itemDistanceX = itemDistanceX;
        let mut itemDistanceY = itemDistanceY;
        let mut absX: i16 = 0i16;
        let mut absY: i16 = 0i16;
        if (((itemDistanceX) as i32) == 0i32) && (((itemDistanceY) as i32) == 0i32) {
            return 0u8;
        }
        if ((itemDistanceX) as i32) < 0i32 {
            absX = ((((itemDistanceX) as i32).wrapping_mul((-1i32))) as i16);
        } else {
            absX = itemDistanceX;
        }
        if ((itemDistanceY) as i32) < 0i32 {
            absY = ((((itemDistanceY) as i32).wrapping_mul((-1i32))) as i16);
        } else {
            absY = itemDistanceY;
        }
        if ((absX) as i32) > ((absY) as i32) {
            if ((itemDistanceX) as i32) < 0i32 {
                return 4u8;
            } else {
                return 2u8;
            }
        } else {
            if ((absX) as i32) < ((absY) as i32) {
                if ((itemDistanceY) as i32) < 0i32 {
                    return 1u8;
                } else {
                    return 3u8;
                }
            } else {
                if ((absX) as i32) == ((absY) as i32) {
                    if ((itemDistanceY) as i32) < 0i32 {
                        return 1u8;
                    } else {
                        return 3u8;
                    }
                } else {
                    return 0u8;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerFaceHiddenItem(direction: u8) {
    unsafe {
        let mut direction = direction;
        ObjectEventClearHeldMovementIfFinished(
            ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8)) as i32) as isize * 36,
            ),
        );
        ObjectEventClearHeldMovement(((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8)) as i32) as isize * 36,
        ));
        UnfreezeObjectEvent(((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8)) as i32) as isize * 36,
        ));
        PlayerTurnInPlace(direction);
    }
}
pub(crate) unsafe extern "C" fn Task_HiddenItemNearby(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((ObjectEventCheckHeldMovementStatus(
            ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8)) as i32) as isize * 36,
            ),
        )) as i32)
            == 1i32
        {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gText_ItemFinderNearby).cast::<u8>(),
                Some(Task_CloseItemfinderMessage),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StandingOnHiddenItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (((ObjectEventCheckHeldMovementStatus(
            ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((GetObjectEventIdByLocalIdAndMap(255u8, 0u8, 0u8)) as i32) as isize * 36,
            ),
        )) as i32)
            == 1i32)
            || (((((data).wrapping_offset(2)).read()) as i32) == 0i32)
        {
            PlayerFaceHiddenItem(
                ((((&raw const sClockwiseDirections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((((data).wrapping_offset(5)).read()) as i32) as isize))
                .read(),
            );
            ((data).wrapping_offset(2)).write(1i16);
            ((data).wrapping_offset(5)).write(
                ((((((data).wrapping_offset(5)).read()) as i32).wrapping_add(1i32) & 3i32) as i16),
            );
            let __p1 = (data).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((data).wrapping_offset(3)).read()) as i32) == 4i32 {
                DisplayItemMessageOnField(
                    taskId,
                    (&raw mut gText_ItemFinderOnTop).cast::<u8>(),
                    Some(Task_CloseItemfinderMessage),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PokeblockCase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((MenuHelpers_IsLinkActive()) as i32) == 1i32 {
            DisplayDadsAdviceCannotUseItemMessage(
                taskId,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u8),
            );
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                != 1i32
            {
                ((((&raw mut gBagMenu).cast::<*mut u8>()).read())
                    .cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(CB2_OpenPokeblockFromBag));
                Task_FadeAndCloseBagMenu(taskId);
            } else {
                ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                    .write(Some(FieldCB_ReturnToFieldNoScript));
                FadeScreen(1u8, 0i8);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(Task_OpenRegisteredPokeblockCase));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CB2_OpenPokeblockFromBag() {
    unsafe {
        OpenPokeblockCase(0u8, Some(CB2_ReturnToBagMenuPocket));
    }
}
pub(crate) unsafe extern "C" fn Task_OpenRegisteredPokeblockCase(taskId: u8) {
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
            CleanupOverworldWindowsAndTilemaps();
            OpenPokeblockCase(0u8, Some(CB2_ReturnToField));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_CoinCase(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((GetCoins()) as i32),
            0i32,
            4u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_CoinCase).cast::<u8>(),
        );
        if !((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read())
            != 0)
        {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(Task_CloseCantUseKeyItemMessage),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PowderJar(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ConvertIntToDecimalStringN(
            (&raw mut gStringVar1).cast::<u8>(),
            ((GetBerryPowder()) as i32),
            0i32,
            5u8,
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PowderQty).cast::<u8>(),
        );
        if !((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read())
            != 0)
        {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageOnField(
                taskId,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(Task_CloseCantUseKeyItemMessage),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Berry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPlayerFacingEmptyBerryTreePatch()) as i32) == 1i32 {
            ((&raw mut sItemUseOnFieldCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemUseOnFieldCB_Berry));
            ((&raw mut gFieldCallback).cast::<Option<unsafe extern "C" fn()>>())
                .write(Some(FieldCB_UseItemOnField));
            ((((&raw mut gBagMenu).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(CB2_ReturnToField));
            Task_FadeAndCloseBagMenu(taskId);
        } else {
            (GetItemFieldFunc(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()))
                .unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Berry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
        LockPlayerFieldControls();
        ScriptContext_SetupScript((&raw mut BerryTree_EventScript_ItemUsePlantBerry).cast::<u8>());
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_WailmerPail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((TryToWaterSudowoodo()) as i32) == 1i32 {
            ((&raw mut sItemUseOnFieldCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemUseOnFieldCB_WailmerPailSudowoodo));
            SetUpItemUseOnFieldCallback(taskId);
        } else {
            if ((TryToWaterBerryTree()) as i32) == 1i32 {
                ((&raw mut sItemUseOnFieldCB)
                    .cast::<u8>()
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(ItemUseOnFieldCB_WailmerPailBerry));
                SetUpItemUseOnFieldCallback(taskId);
            } else {
                DisplayDadsAdviceCannotUseItemMessage(
                    taskId,
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_WailmerPailBerry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LockPlayerFieldControls();
        ScriptContext_SetupScript((&raw mut BerryTree_EventScript_ItemUseWailmerPail).cast::<u8>());
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn TryToWaterSudowoodo() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut elevation: u8 = 0u8;
        let mut objId: u8 = 0u8;
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        elevation = PlayerGetElevation();
        objId = GetObjectEventIdByPosition(((x) as u16), ((y) as u16), elevation);
        if (((objId) as i32) == 16i32)
            || (((((((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objId) as i32) as isize * 36))
            .wrapping_add(5))
            .read()) as i32)
                != 228i32)
        {
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
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_WailmerPailSudowoodo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        LockPlayerFieldControls();
        ScriptContext_SetupScript(
            (&raw mut BattleFrontier_OutsideEast_EventScript_WaterSudowoodo).cast::<u8>(),
        );
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Medicine(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_Medicine));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_ReduceEV(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_ReduceEV));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_SacredAsh(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_SacredAsh));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PPRecovery(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_PPRecovery));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PPUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_PPUp));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_RareCandy(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_RareCandy));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_TMHM(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) >= 339i32 {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gText_BootedUpHM).cast::<u8>(),
                Some(BootUpSoundTMHM),
            );
        } else {
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gText_BootedUpTM).cast::<u8>(),
                Some(BootUpSoundTMHM),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn BootUpSoundTMHM(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PlaySE(2u16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_ShowTMHMContainedMessage));
    }
}
pub(crate) unsafe extern "C" fn Task_ShowTMHMContainedMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            StringCopy(
                (&raw mut gStringVar1).cast::<u8>(),
                (((&raw mut gMoveNames).cast::<u8>()).wrapping_offset(
                    ((ItemIdToBattleMoveId(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()))
                        as i32) as isize
                        * 13,
                ))
                .cast::<u8>(),
            );
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_TMHMContainedVar1).cast::<u8>(),
            );
            DisplayItemMessage(
                taskId,
                1u8,
                (&raw mut gStringVar4).cast::<u8>(),
                Some(UseTMHMYesNo),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn UseTMHMYesNo(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        BagMenu_YesNo(
            taskId,
            6u8,
            (&raw const sUseTMHMYesNoFuncTable).cast::<u8>().cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn UseTMHM(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_TMHM));
        SetUpItemUseCallback(taskId);
    }
}
pub(crate) unsafe extern "C" fn RemoveUsedItem() {
    unsafe {
        RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        StringExpandPlaceholders(
            (&raw mut gStringVar4).cast::<u8>(),
            (&raw mut gText_PlayerUsedVar2).cast::<u8>(),
        );
        if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
            UpdatePocketItemList(GetItemPocket(
                ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            ));
            UpdatePocketListPosition(GetItemPocket(
                ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            ));
        } else {
            UpdatePyramidBagList();
            UpdatePyramidBagCursorPos();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Repel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((VarGet(16417u16)) as i32) == 0i32 {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_StartUseRepel));
        } else {
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gText_RepelEffectsLingered).cast::<u8>(),
                    Some(CloseItemMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gText_RepelEffectsLingered).cast::<u8>(),
                    Some(Task_CloseBattlePyramidBagMessage),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_StartUseRepel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        if (({
            let __p1 = (data).wrapping_offset(8);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 7i32
        {
            ((data).wrapping_offset(8)).write(0i16);
            PlaySE(47u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UseRepel));
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UseRepel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((IsSEPlaying()) != 0) {
            VarSet(
                16417u16,
                ((GetItemHoldEffectParam(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()))
                    as u16),
            );
            RemoveUsedItem();
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(CloseItemMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(Task_CloseBattlePyramidBagMessage),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UsedBlackWhiteFlute(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 7i32
        {
            PlaySE(117u16);
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(CloseItemMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(Task_CloseBattlePyramidBagMessage),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_BlackWhiteFlute(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        CopyItemName(
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            (&raw mut gStringVar2).cast::<u8>(),
        );
        if ((((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()) as i32) == 43i32 {
            FlagSet(2221u16);
            FlagClear(2222u16);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_UsedVar2WildLured).cast::<u8>(),
            );
        } else {
            FlagSet(2222u16);
            FlagClear(2221u16);
            StringExpandPlaceholders(
                (&raw mut gStringVar4).cast::<u8>(),
                (&raw mut gText_UsedVar2WildRepelled).cast::<u8>(),
            );
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_UsedBlackWhiteFlute));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_UseDigEscapeRopeOnField(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ResetInitialPlayerAvatarState();
        StartEscapeRopeFieldEffect();
        DestroyTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_EscapeRope(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        Overworld_ResetStateAfterDigEscRope();
        RemoveUsedItem();
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        DisplayItemMessageOnField(
            taskId,
            (&raw mut gStringVar4).cast::<u8>(),
            Some(Task_UseDigEscapeRopeOnField),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanUseDigOrEscapeRopeOnCurMap() -> u8 {
    unsafe {
        if (crate::c::bf_read(
            ((&raw mut gMapHeader).cast::<u8>()).wrapping_add(26),
            1,
            1,
            false,
        ) as u8)
            != 0
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
pub unsafe extern "C" fn ItemUseOutOfBattle_EscapeRope(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((CanUseDigOrEscapeRopeOnCurMap()) as i32) == 1i32 {
            ((&raw mut sItemUseOnFieldCB)
                .cast::<u8>()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(ItemUseOnFieldCB_EscapeRope));
            SetUpItemUseOnFieldCallback(taskId);
        } else {
            DisplayDadsAdviceCannotUseItemMessage(
                taskId,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_EvolutionStone(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_EvolutionStone));
        SetUpItemUseCallback(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_PokeBall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsPlayerPartyAndPokemonStorageFull()) as i32) == 0i32 {
            RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                Task_FadeAndCloseBagMenu(taskId);
            } else {
                CloseBattlePyramidBag(taskId);
            }
        } else {
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gText_BoxFull).cast::<u8>(),
                    Some(CloseItemMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gText_BoxFull).cast::<u8>(),
                    Some(Task_CloseBattlePyramidBagMessage),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_CloseStatIncreaseMessage(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 3i32)
            != 0
        {
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                Task_FadeAndCloseBagMenu(taskId);
            } else {
                CloseBattlePyramidBag(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UseStatIncreaseItem(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 7i32
        {
            PlaySE(1u16);
            RemoveBagItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(), 1u16);
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    UseStatIncreaseItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()),
                    Some(Task_CloseStatIncreaseMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    UseStatIncreaseItem(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read()),
                    Some(Task_CloseStatIncreaseMessage),
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_StatIncrease(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut partyId: u16 = ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
            .wrapping_offset(
                ((((&raw mut gBattlerInMenuId).cast::<u8>()).read()) as i32) as isize,
            ))
        .read();
        if ((ExecuteTableBasedItemEffect(
            ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyId) as i32) as isize * 100),
            ((&raw mut gSpecialVar_ItemId).cast::<u16>()).read(),
            ((partyId) as u8),
            0u8,
        )) as i32)
            != 0i32
        {
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gText_WontHaveEffect).cast::<u8>(),
                    Some(CloseItemMessage),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gText_WontHaveEffect).cast::<u8>(),
                    Some(Task_CloseBattlePyramidBagMessage),
                );
            }
        } else {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(Task_UseStatIncreaseItem));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn ItemUseInBattle_ShowPartyMenu(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
            ((((&raw mut gBagMenu).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(ChooseMonForInBattleItem));
            Task_FadeAndCloseBagMenu(taskId);
        } else {
            ((((&raw mut gPyramidBagMenu).cast::<*mut u8>()).read())
                .cast::<Option<unsafe extern "C" fn()>>())
            .write(Some(ChooseMonForInBattleItem));
            CloseBattlePyramidBag(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_Medicine(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_Medicine));
        ItemUseInBattle_ShowPartyMenu(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_SacredAsh(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_SacredAsh));
        ItemUseInBattle_ShowPartyMenu(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_PPRecovery(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((&raw mut gItemUseCB)
            .cast::<Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>>())
        .write(Some(ItemUseCB_PPRecovery));
        ItemUseInBattle_ShowPartyMenu(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_Escape(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 8u32) == 0u32 {
            RemoveUsedItem();
            if ((CurrentBattlePyramidLocation()) as i32) == 0i32 {
                DisplayItemMessage(
                    taskId,
                    1u8,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(Task_FadeAndCloseBagMenu),
                );
            } else {
                DisplayItemMessageInBattlePyramid(
                    taskId,
                    (&raw mut gStringVar4).cast::<u8>(),
                    Some(CloseBattlePyramidBag),
                );
            }
        } else {
            DisplayDadsAdviceCannotUseItemMessage(
                taskId,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u8),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_EnigmaBerry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                ((GetItemEffectType(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32);
            let __matched = __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 10i32
                || __sw1 == 1i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32;
            if __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 11i32
                || __sw1 == 12i32
                || __sw1 == 13i32
                || __sw1 == 14i32
                || __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                ItemUseOutOfBattle_Medicine(taskId);
                break 'l1;
            }
            if __sw1 == 10i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                ItemUseOutOfBattle_SacredAsh(taskId);
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                ItemUseOutOfBattle_RareCandy(taskId);
                break 'l1;
            }
            if __sw1 == 19i32 || __sw1 == 20i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                ItemUseOutOfBattle_PPUp(taskId);
                break 'l1;
            }
            if __sw1 == 21i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                ItemUseOutOfBattle_PPRecovery(taskId);
                break 'l1;
            }
            if !__matched {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(4i16);
                ItemUseOutOfBattle_CannotUse(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_EnigmaBerry(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 =
                ((GetItemEffectType(((&raw mut gSpecialVar_ItemId).cast::<u16>()).read())) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 11i32
                || __sw1 == 8i32
                || __sw1 == 9i32
                || __sw1 == 21i32;
            if __sw1 == 0i32 {
                ItemUseInBattle_StatIncrease(taskId);
                break 'l1;
            }
            if __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32
                || __sw1 == 11i32
                || __sw1 == 8i32
                || __sw1 == 9i32
            {
                ItemUseInBattle_Medicine(taskId);
                break 'l1;
            }
            if __sw1 == 21i32 {
                ItemUseInBattle_PPRecovery(taskId);
                break 'l1;
            }
            if !__matched {
                ItemUseOutOfBattle_CannotUse(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_CannotUse(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DisplayDadsAdviceCannotUseItemMessage(
            taskId,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as u8),
        );
    }
}
