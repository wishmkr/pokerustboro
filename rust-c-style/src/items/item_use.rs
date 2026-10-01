//! Translated from `src/item_use.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sItemUseCallbacks sClockwiseDirections sUseTMHMYesNoFuncTable

static sClockwiseDirections: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_use::sClockwiseDirections).cast());
static sItemUseCallbacks: Table<CArray<Option<unsafe extern "C" fn()>, 3>> =
    Table((&raw const crate::data::item_use::sItemUseCallbacks).cast());
static sUseTMHMYesNoFuncTable: Table<YesNoFuncTable> =
    Table((&raw const crate::data::item_use::sUseTMHMYesNoFuncTable).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemUseOnFieldCB: Option<unsafe extern "C" fn(u8)> = None;

unsafe extern "C" {
    static BattleFrontier_OutsideEast_EventScript_WaterSudowoodo: CArray<u8, 0>;
    static BerryTree_EventScript_ItemUsePlantBerry: CArray<u8, 0>;
    static BerryTree_EventScript_ItemUseWailmerPail: CArray<u8, 0>;
    static mut gBagMenu: *mut BagMenu;
    static mut gBattleTypeFlags: u32;
    static mut gBattlerInMenuId: u8;
    static mut gBattlerPartyIndexes: CArray<u16, 4>;
    static mut gFieldCallback: Option<unsafe extern "C" fn()>;
    static mut gItemUseCB: Option<unsafe extern "C" fn(u8, Option<unsafe extern "C" fn(u8)>)>;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gPyramidBagMenu: *mut PyramidBagMenu;
    static mut gSpecialVar_ItemId: u16;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static mut gTasks: CArray<Task, 0>;
    static gText_BootedUpHM: CArray<u8, 0>;
    static gText_BootedUpTM: CArray<u8, 0>;
    static gText_BoxFull: CArray<u8, 0>;
    static gText_CantDismountBike: CArray<u8, 0>;
    static gText_CoinCase: CArray<u8, 0>;
    static gText_DadsAdvice: CArray<u8, 0>;
    static gText_ItemFinderNearby: CArray<u8, 0>;
    static gText_ItemFinderNothing: CArray<u8, 0>;
    static gText_ItemFinderOnTop: CArray<u8, 0>;
    static gText_PlayerUsedVar2: CArray<u8, 0>;
    static gText_PowderQty: CArray<u8, 0>;
    static gText_RepelEffectsLingered: CArray<u8, 0>;
    static gText_TMHMContainedVar1: CArray<u8, 0>;
    static gText_UsedVar2WildLured: CArray<u8, 0>;
    static gText_UsedVar2WildRepelled: CArray<u8, 0>;
    static gText_WontHaveEffect: CArray<u8, 0>;
    fn BagMenu_YesNo(a0: u8, a1: u8, a2: *mut YesNoFuncTable);
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
    fn ExecuteTableBasedItemEffect(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8) -> u8;
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
    fn GetMapConnectionAtPos(a0: i16, a1: i16) -> *mut MapConnection;
    fn GetMapHeaderFromConnection(a0: *mut MapConnection) -> *mut MapHeader;
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
    fn ObjectEventCheckHeldMovementStatus(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventClearHeldMovement(a0: *mut ObjectEvent);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn OpenPokeblockCase(a0: u8, a1: Option<unsafe extern "C" fn()>);
    fn Overworld_IsBikingAllowed() -> u32;
    fn Overworld_ResetStateAfterDigEscRope();
    fn PlaySE(a0: u16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerGetElevation() -> u8;
    fn PlayerTurnInPlace(a0: u8);
    fn ReadMail(a0: *mut Mail, a1: Option<unsafe extern "C" fn()>, a2: u8);
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
    fn UnfreezeObjectEvent(a0: *mut ObjectEvent);
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
    let mut r#type: u8 = 0;
    if gSpecialVar_ItemId == ITEM_ENIGMA_BERRY {
        r#type = gTasks[taskId].data[4] as u8 - 1;
    } else {
        r#type = GetItemType(gSpecialVar_ItemId) - 1;
    }
    if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        (*gBagMenu).newScreenCallback = sItemUseCallbacks[r#type];
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        (*gPyramidBagMenu).newScreenCallback = sItemUseCallbacks[r#type];
        CloseBattlePyramidBag(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetUpItemUseOnFieldCallback(taskId: u8) {
    if gTasks[taskId].data[3] != TRUE as i16 {
        gFieldCallback = Some(FieldCB_UseItemOnField);
        SetUpItemUseCallback(taskId);
    } else {
        sItemUseOnFieldCB.unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn FieldCB_UseItemOnField() {
    FadeInFromBlack();
    CreateTask(Some(Task_CallItemUseOnFieldCallback), 8);
}
pub(crate) unsafe extern "C" fn Task_CallItemUseOnFieldCallback(taskId: u8) {
    if IsWeatherNotFadingIn() == 1 {
        sItemUseOnFieldCB.unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn DisplayCannotUseItemMessage(
    taskId: u8,
    isUsingRegisteredKeyItemOnField: u8,
    str: *mut u8,
) {
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), str);
    if isUsingRegisteredKeyItemOnField == 0 {
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                gText_DadsAdvice.as_ptr().cast_mut(),
                Some(Task_CloseBattlePyramidBagMessage),
            );
        }
    } else {
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(Task_CloseCantUseKeyItemMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn DisplayDadsAdviceCannotUseItemMessage(
    taskId: u8,
    isUsingRegisteredKeyItemOnField: u8,
) {
    DisplayCannotUseItemMessage(
        taskId,
        isUsingRegisteredKeyItemOnField,
        gText_DadsAdvice.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn DisplayCannotDismountBikeMessage(
    taskId: u8,
    isUsingRegisteredKeyItemOnField: u8,
) {
    DisplayCannotUseItemMessage(
        taskId,
        isUsingRegisteredKeyItemOnField,
        gText_CantDismountBike.as_ptr().cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn Task_CloseCantUseKeyItemMessage(taskId: u8) {
    ClearDialogWindowAndFrame(0, TRUE);
    DestroyTask(taskId);
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckIfItemIsTMHMOrEvolutionStone(itemId: u16) -> u8 {
    if GetItemFieldFunc(itemId) == Some(ItemUseOutOfBattle_TMHM as unsafe extern "C" fn(u8)) {
        return ITEM_IS_TM_HM;
    } else if GetItemFieldFunc(itemId)
        == Some(ItemUseOutOfBattle_EvolutionStone as unsafe extern "C" fn(u8))
    {
        return ITEM_IS_EVOLUTION_STONE;
    } else {
        return ITEM_IS_OTHER;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CB2_CheckMail() {
    let mut mail: Mail = zeroed();
    mail.itemId = gSpecialVar_ItemId;
    ReadMail(&raw mut mail, Some(CB2_ReturnToBagMenuPocket), FALSE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Mail(taskId: u8) {
    (*gBagMenu).newScreenCallback = Some(CB2_CheckMail);
    Task_FadeAndCloseBagMenu(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Bike(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut coordsY: i16 = 0;
    let mut coordsX: i16 = 0;
    let mut behavior: u8 = 0;
    PlayerGetDestCoords(&raw mut coordsX, &raw mut coordsY);
    behavior = MapGridGetMetatileBehaviorAt(coordsX as i32, coordsY as i32) as u8;
    if FlagGet(FLAG_SYS_CYCLING_ROAD) == TRUE
        || MetatileBehavior_IsVerticalRail(behavior) == TRUE
        || MetatileBehavior_IsHorizontalRail(behavior) == TRUE
        || MetatileBehavior_IsIsolatedVerticalRail(behavior) == TRUE
        || MetatileBehavior_IsIsolatedHorizontalRail(behavior) == TRUE
    {
        DisplayCannotDismountBikeMessage(taskId, *data.at(3) as u8);
    } else if Overworld_IsBikingAllowed() == TRUE as u32 && IsBikingDisallowedByPlayer() == 0 {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Bike);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(taskId, *data.at(3) as u8);
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Bike(taskId: u8) {
    if GetItemSecondaryId(gSpecialVar_ItemId) == MACH_BIKE {
        GetOnOffBike(PLAYER_AVATAR_FLAG_MACH_BIKE);
    } else {
        GetOnOffBike(PLAYER_AVATAR_FLAG_ACRO_BIKE);
    }
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn CanFish() -> u32 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut tileBehavior: u16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
    if MetatileBehavior_IsWaterfall(tileBehavior as u8) != 0 {
        return FALSE as u32;
    }
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_UNDERWATER) != 0 {
        return FALSE as u32;
    }
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) == 0 {
        if IsPlayerFacingSurfableFishableWater() != 0 {
            return TRUE as u32;
        }
    } else {
        if MetatileBehavior_IsSurfableWaterOrUnderwater(tileBehavior as u8) != 0
            && MapGridGetCollisionAt(x as i32, y as i32) == 0
        {
            return TRUE as u32;
        }
        if MetatileBehavior_IsBridgeOverWaterNoEdge(tileBehavior as u8) == TRUE {
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Rod(taskId: u8) {
    if CanFish() == TRUE as u32 {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Rod);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(taskId, gTasks[taskId].data[3] as u8);
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Rod(taskId: u8) {
    StartFishing(GetItemSecondaryId(gSpecialVar_ItemId));
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Itemfinder(var: u8) {
    IncrementGameStat(GAME_STAT_USED_ITEMFINDER);
    sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Itemfinder);
    SetUpItemUseOnFieldCallback(var);
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Itemfinder(taskId: u8) {
    if ItemfinderCheckForHiddenItems(gMapHeader.events, taskId) == TRUE {
        gTasks[taskId].func = Some(Task_UseItemfinder);
    } else {
        DisplayItemMessageOnField(
            taskId,
            gText_ItemFinderNothing.as_ptr().cast_mut(),
            Some(Task_CloseItemfinderMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_UseItemfinder(taskId: u8) {
    let mut playerDir: u8 = 0;
    let mut playerDirToItem: u8 = 0;
    let mut i: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(3) == 0 {
        if *data.at(4) == 4 {
            playerDirToItem = GetDirectionToHiddenItem(*data, *data.at(1));
            if playerDirToItem != DIR_NONE {
                PlayerFaceHiddenItem(sClockwiseDirections[playerDirToItem as i32 - 1]);
                gTasks[taskId].func = Some(Task_HiddenItemNearby);
            } else {
                playerDir = GetPlayerFacingDirection();
                i = 0;
                while i < 4 {
                    if playerDir == sClockwiseDirections[i] {
                        *data.at(5) = i as i16 + 1 & 3;
                    }
                    i += 1;
                }
                gTasks[taskId].func = Some(Task_StandingOnHiddenItem);
                *data.at(3) = 0;
                *data.at(2) = 0;
            }
            return;
        }
        PlaySE(SE_ITEMFINDER);
        *data.at(4) += 1;
    }
    *data.at(3) = *data.at(3) + 1 & 0x1F;
}
pub(crate) unsafe extern "C" fn Task_CloseItemfinderMessage(taskId: u8) {
    ClearDialogWindowAndFrame(0, TRUE);
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn ItemfinderCheckForHiddenItems(
    events: *mut MapEvents,
    taskId: u8,
) -> u8 {
    let mut playerX: i16 = 0;
    let mut playerY: i16 = 0;
    let mut i: i16 = 0;
    let mut distanceX: i16 = 0;
    let mut distanceY: i16 = 0;
    PlayerGetDestCoords(&raw mut playerX, &raw mut playerY);
    gTasks[taskId].data[2] = FALSE as i16;
    i = 0;
    while i < (*events).bgEventCount as i16 {
        if (*(*events).bgEvents.at(i)).kind == BG_EVENT_HIDDEN_ITEM
            && FlagGet(
                (*(*events).bgEvents.at(i)).bgUnion.hiddenItem.hiddenItemId
                    + FLAG_HIDDEN_ITEMS_START,
            ) == 0
        {
            distanceX = (*(*events).bgEvents.at(i)).x as i16 + MAP_OFFSET as i16 - playerX;
            distanceY = (*(*events).bgEvents.at(i)).y as i16 + MAP_OFFSET as i16 - playerY;
            if distanceX >= -7 && distanceX <= 7 && distanceY >= -5 && distanceY <= 5 {
                SetDistanceOfClosestHiddenItem(taskId, distanceX, distanceY);
            }
        }
        i += 1;
    }
    CheckForHiddenItemsInMapConnection(taskId);
    if gTasks[taskId].data[2] == TRUE as i16 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsHiddenItemPresentAtCoords(
    events: *mut MapEvents,
    x: i16,
    y: i16,
) -> u8 {
    let mut bgEventCount: u8 = (*events).bgEventCount;
    let mut bgEvent: *mut BgEvent = (*events).bgEvents;
    let mut i: i32 = 0;
    i = 0;
    while i < bgEventCount as i32 {
        if (*bgEvent.at(i)).kind == BG_EVENT_HIDDEN_ITEM
            && x as i32 == (*bgEvent.at(i)).x as i32
            && y as i32 == (*bgEvent.at(i)).y as i32
        {
            if FlagGet((*bgEvent.at(i)).bgUnion.hiddenItem.hiddenItemId + FLAG_HIDDEN_ITEMS_START)
                == 0
            {
                return TRUE;
            } else {
                return FALSE;
            }
        }
        i += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsHiddenItemPresentInConnection(
    connection: *mut MapConnection,
    x: i32,
    y: i32,
) -> u8 {
    let mut connectionX: i16 = 0;
    let mut connectionY: i16 = 0;
    let mut connectionHeader: *mut MapHeader = GetMapHeaderFromConnection(connection);
    match (*connection).direction {
        CONNECTION_NORTH => {
            connectionX = x as i16 - 7 - (*connection).offset as i16;
            connectionY = (*(*connectionHeader).mapLayout).height as i16 + (y as i16 - 7);
        }
        CONNECTION_SOUTH => {
            connectionX = x as i16 - 7 - (*connection).offset as i16;
            connectionY = y as i16 - 7 - (*gMapHeader.mapLayout).height as i16;
        }
        CONNECTION_WEST => {
            connectionX = (*(*connectionHeader).mapLayout).width as i16 + (x as i16 - 7);
            connectionY = y as i16 - 7 - (*connection).offset as i16;
        }
        CONNECTION_EAST => {
            connectionX = x as i16 - 7 - (*gMapHeader.mapLayout).width as i16;
            connectionY = y as i16 - 7 - (*connection).offset as i16;
        }
        _ => {
            return FALSE;
        }
    }
    return IsHiddenItemPresentAtCoords((*connectionHeader).events, connectionX, connectionY);
}
pub(crate) unsafe extern "C" fn CheckForHiddenItemsInMapConnection(taskId: u8) {
    let mut playerX: i16 = 0;
    let mut playerY: i16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut width: i16 = (*gMapHeader.mapLayout).width as i16 + MAP_OFFSET as i16;
    let mut height: i16 = (*gMapHeader.mapLayout).height as i16 + MAP_OFFSET as i16;
    let mut var1: i16 = MAP_OFFSET as i16;
    let mut var2: i16 = MAP_OFFSET as i16;
    PlayerGetDestCoords(&raw mut playerX, &raw mut playerY);
    x = playerX - 7;
    while x as i32 <= playerX as i32 + 7 {
        y = playerY - 5;
        while y as i32 <= playerY as i32 + 5 {
            if var1 > x || x >= width || var2 > y || y >= height {
                let mut conn: *mut MapConnection = GetMapConnectionAtPos(x, y);
                if !conn.is_null()
                    && IsHiddenItemPresentInConnection(conn, x as i32, y as i32) == TRUE
                {
                    SetDistanceOfClosestHiddenItem(taskId, x - playerX, y - playerY);
                }
            }
            y += 1;
        }
        x += 1;
    }
}
pub(crate) unsafe extern "C" fn SetDistanceOfClosestHiddenItem(
    taskId: u8,
    itemDistanceX: i16,
    itemDistanceY: i16,
) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    let mut oldItemAbsX: i16 = 0;
    let mut oldItemAbsY: i16 = 0;
    let mut newItemAbsX: i16 = 0;
    let mut newItemAbsY: i16 = 0;
    if *data.at(2) == FALSE as i16 {
        *data = itemDistanceX;
        *data.at(1) = itemDistanceY;
        *data.at(2) = TRUE as i16;
        return;
    }
    if *data < 0 {
        oldItemAbsX = *data * -1;
    } else {
        oldItemAbsX = *data;
    }
    if *data.at(1) < 0 {
        oldItemAbsY = *data.at(1) * -1;
    } else {
        oldItemAbsY = *data.at(1);
    }
    if itemDistanceX < 0 {
        newItemAbsX = itemDistanceX * -1;
    } else {
        newItemAbsX = itemDistanceX;
    }
    if itemDistanceY < 0 {
        newItemAbsY = itemDistanceY * -1;
    } else {
        newItemAbsY = itemDistanceY;
    }
    if oldItemAbsX as i32 + oldItemAbsY as i32 > newItemAbsX as i32 + newItemAbsY as i32 {
        *data = itemDistanceX;
        *data.at(1) = itemDistanceY;
    } else if oldItemAbsX as i32 + oldItemAbsY as i32 == newItemAbsX as i32 + newItemAbsY as i32
        && (oldItemAbsY > newItemAbsY || oldItemAbsY == newItemAbsY && *data.at(1) < itemDistanceY)
    {
        *data = itemDistanceX;
        *data.at(1) = itemDistanceY;
    }
}
pub(crate) unsafe extern "C" fn GetDirectionToHiddenItem(
    itemDistanceX: i16,
    itemDistanceY: i16,
) -> u8 {
    let mut absX: i16 = 0;
    let mut absY: i16 = 0;
    if itemDistanceX == 0 && itemDistanceY == 0 {
        return DIR_NONE;
    }
    if itemDistanceX < 0 {
        absX = itemDistanceX * -1;
    } else {
        absX = itemDistanceX;
    }
    if itemDistanceY < 0 {
        absY = itemDistanceY * -1;
    } else {
        absY = itemDistanceY;
    }
    if absX > absY {
        if itemDistanceX < 0 {
            return DIR_EAST;
        } else {
            return DIR_NORTH;
        }
    } else if absX < absY {
        if itemDistanceY < 0 {
            return DIR_SOUTH;
        } else {
            return DIR_WEST;
        }
    } else if absX == absY {
        if itemDistanceY < 0 {
            return DIR_SOUTH;
        } else {
            return DIR_WEST;
        }
    } else {
        return DIR_NONE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn PlayerFaceHiddenItem(direction: u8) {
    ObjectEventClearHeldMovementIfFinished(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    );
    ObjectEventClearHeldMovement(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    );
    UnfreezeObjectEvent(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    );
    PlayerTurnInPlace(direction);
}
pub(crate) unsafe extern "C" fn Task_HiddenItemNearby(taskId: u8) {
    if ObjectEventCheckHeldMovementStatus(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    ) == TRUE
    {
        DisplayItemMessageOnField(
            taskId,
            gText_ItemFinderNearby.as_ptr().cast_mut(),
            Some(Task_CloseItemfinderMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StandingOnHiddenItem(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if ObjectEventCheckHeldMovementStatus(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    ) == TRUE
        || *data.at(2) == FALSE as i16
    {
        PlayerFaceHiddenItem(sClockwiseDirections[*data.at(5)]);
        *data.at(2) = TRUE as i16;
        *data.at(5) = *data.at(5) + 1 & 3;
        *data.at(3) += 1;
        if *data.at(3) == 4 {
            DisplayItemMessageOnField(
                taskId,
                gText_ItemFinderOnTop.as_ptr().cast_mut(),
                Some(Task_CloseItemfinderMessage),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PokeblockCase(taskId: u8) {
    if MenuHelpers_IsLinkActive() == TRUE {
        DisplayDadsAdviceCannotUseItemMessage(taskId, gTasks[taskId].data[3] as u8);
    } else if gTasks[taskId].data[3] != TRUE as i16 {
        (*gBagMenu).newScreenCallback = Some(CB2_OpenPokeblockFromBag);
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        gFieldCallback = Some(FieldCB_ReturnToFieldNoScript);
        FadeScreen(FADE_TO_BLACK, 0);
        gTasks[taskId].func = Some(Task_OpenRegisteredPokeblockCase);
    }
}
pub(crate) unsafe extern "C" fn CB2_OpenPokeblockFromBag() {
    OpenPokeblockCase(PBLOCK_CASE_FIELD, Some(CB2_ReturnToBagMenuPocket));
}
pub(crate) unsafe extern "C" fn Task_OpenRegisteredPokeblockCase(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        OpenPokeblockCase(PBLOCK_CASE_FIELD, Some(CB2_ReturnToField));
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_CoinCase(taskId: u8) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetCoins() as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    StringExpandPlaceholders(gStringVar4.as_mut_ptr(), gText_CoinCase.as_ptr().cast_mut());
    if gTasks[taskId].data[3] == 0 {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            Some(CloseItemMessage),
        );
    } else {
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(Task_CloseCantUseKeyItemMessage),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PowderJar(taskId: u8) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetBerryPowder() as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PowderQty.as_ptr().cast_mut(),
    );
    if gTasks[taskId].data[3] == 0 {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            Some(CloseItemMessage),
        );
    } else {
        DisplayItemMessageOnField(
            taskId,
            gStringVar4.as_mut_ptr(),
            Some(Task_CloseCantUseKeyItemMessage),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Berry(taskId: u8) {
    if IsPlayerFacingEmptyBerryTreePatch() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Berry);
        gFieldCallback = Some(FieldCB_UseItemOnField);
        (*gBagMenu).newScreenCallback = Some(CB2_ReturnToField);
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        GetItemFieldFunc(gSpecialVar_ItemId).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_Berry(taskId: u8) {
    RemoveBagItem(gSpecialVar_ItemId, 1);
    LockPlayerFieldControls();
    ScriptContext_SetupScript(BerryTree_EventScript_ItemUsePlantBerry.as_ptr().cast_mut());
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_WailmerPail(taskId: u8) {
    if TryToWaterSudowoodo() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_WailmerPailSudowoodo);
        SetUpItemUseOnFieldCallback(taskId);
    } else if TryToWaterBerryTree() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_WailmerPailBerry);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(taskId, gTasks[taskId].data[3] as u8);
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_WailmerPailBerry(taskId: u8) {
    LockPlayerFieldControls();
    ScriptContext_SetupScript(BerryTree_EventScript_ItemUseWailmerPail.as_ptr().cast_mut());
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn TryToWaterSudowoodo() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut elevation: u8 = 0;
    let mut objId: u8 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    elevation = PlayerGetElevation();
    objId = GetObjectEventIdByPosition(x as u16, y as u16, elevation);
    if objId == OBJECT_EVENTS_COUNT || gObjectEvents[objId].graphicsId != OBJ_EVENT_GFX_SUDOWOODO {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_WailmerPailSudowoodo(taskId: u8) {
    LockPlayerFieldControls();
    ScriptContext_SetupScript(
        BattleFrontier_OutsideEast_EventScript_WaterSudowoodo
            .as_ptr()
            .cast_mut(),
    );
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Medicine(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_Medicine);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_ReduceEV(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_ReduceEV);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_SacredAsh(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_SacredAsh);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PPRecovery(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_PPRecovery);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_PPUp(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_PPUp);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_RareCandy(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_RareCandy);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_TMHM(taskId: u8) {
    if gSpecialVar_ItemId >= ITEM_HM01 {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gText_BootedUpHM.as_ptr().cast_mut(),
            Some(BootUpSoundTMHM),
        );
    } else {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gText_BootedUpTM.as_ptr().cast_mut(),
            Some(BootUpSoundTMHM),
        );
    }
}
pub(crate) unsafe extern "C" fn BootUpSoundTMHM(taskId: u8) {
    PlaySE(SE_PC_LOGIN);
    gTasks[taskId].func = Some(Task_ShowTMHMContainedMessage);
}
pub(crate) unsafe extern "C" fn Task_ShowTMHMContainedMessage(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gMoveNames[ItemIdToBattleMoveId(gSpecialVar_ItemId)]
                .as_ptr()
                .cast_mut(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_TMHMContainedVar1.as_ptr().cast_mut(),
        );
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            Some(UseTMHMYesNo),
        );
    }
}
pub(crate) unsafe extern "C" fn UseTMHMYesNo(taskId: u8) {
    BagMenu_YesNo(
        taskId,
        ITEMWIN_YESNO_HIGH,
        (&raw const *sUseTMHMYesNoFuncTable).cast_mut(),
    );
}
pub(crate) unsafe extern "C" fn UseTMHM(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_TMHM);
    SetUpItemUseCallback(taskId);
}
pub(crate) unsafe extern "C" fn RemoveUsedItem() {
    RemoveBagItem(gSpecialVar_ItemId, 1);
    CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        gText_PlayerUsedVar2.as_ptr().cast_mut(),
    );
    if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        UpdatePocketItemList(GetItemPocket(gSpecialVar_ItemId));
        UpdatePocketListPosition(GetItemPocket(gSpecialVar_ItemId));
    } else {
        UpdatePyramidBagList();
        UpdatePyramidBagCursorPos();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_Repel(taskId: u8) {
    if VarGet(VAR_REPEL_STEP_COUNT) == 0 {
        gTasks[taskId].func = Some(Task_StartUseRepel);
    } else if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gText_RepelEffectsLingered.as_ptr().cast_mut(),
            Some(CloseItemMessage),
        );
    } else {
        DisplayItemMessageInBattlePyramid(
            taskId,
            gText_RepelEffectsLingered.as_ptr().cast_mut(),
            Some(Task_CloseBattlePyramidBagMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_StartUseRepel(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if ({
        *data.at(8) += 1;
        *data.at(8)
    }) > 7
    {
        *data.at(8) = 0;
        PlaySE(SE_REPEL as u16);
        gTasks[taskId].func = Some(Task_UseRepel);
    }
}
pub(crate) unsafe extern "C" fn Task_UseRepel(taskId: u8) {
    if IsSEPlaying() == 0 {
        VarSet(
            VAR_REPEL_STEP_COUNT,
            GetItemHoldEffectParam(gSpecialVar_ItemId) as u16,
        );
        RemoveUsedItem();
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                gStringVar4.as_mut_ptr(),
                Some(Task_CloseBattlePyramidBagMessage),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UsedBlackWhiteFlute(taskId: u8) {
    if ({
        gTasks[taskId].data[8] += 1;
        gTasks[taskId].data[8]
    }) > 7
    {
        PlaySE(SE_GLASS_FLUTE);
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                gStringVar4.as_mut_ptr(),
                Some(Task_CloseBattlePyramidBagMessage),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_BlackWhiteFlute(taskId: u8) {
    CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
    if gSpecialVar_ItemId == ITEM_WHITE_FLUTE {
        FlagSet(FLAG_SYS_ENC_UP_ITEM);
        FlagClear(FLAG_SYS_ENC_DOWN_ITEM);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_UsedVar2WildLured.as_ptr().cast_mut(),
        );
    } else {
        FlagSet(FLAG_SYS_ENC_DOWN_ITEM);
        FlagClear(FLAG_SYS_ENC_UP_ITEM);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            gText_UsedVar2WildRepelled.as_ptr().cast_mut(),
        );
    }
    gTasks[taskId].data[8] = 0;
    gTasks[taskId].func = Some(Task_UsedBlackWhiteFlute);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Task_UseDigEscapeRopeOnField(taskId: u8) {
    ResetInitialPlayerAvatarState();
    StartEscapeRopeFieldEffect();
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn ItemUseOnFieldCB_EscapeRope(taskId: u8) {
    Overworld_ResetStateAfterDigEscRope();
    RemoveUsedItem();
    gTasks[taskId].data[0] = 0;
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(Task_UseDigEscapeRopeOnField),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CanUseDigOrEscapeRopeOnCurMap() -> u8 {
    if gMapHeader.allowEscaping() != 0 {
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
pub unsafe extern "C" fn ItemUseOutOfBattle_EscapeRope(taskId: u8) {
    if CanUseDigOrEscapeRopeOnCurMap() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_EscapeRope);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(taskId, gTasks[taskId].data[3] as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_EvolutionStone(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_EvolutionStone);
    SetUpItemUseCallback(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_PokeBall(taskId: u8) {
    if IsPlayerPartyAndPokemonStorageFull() == FALSE {
        RemoveBagItem(gSpecialVar_ItemId, 1);
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            Task_FadeAndCloseBagMenu(taskId);
        } else {
            CloseBattlePyramidBag(taskId);
        }
    } else if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gText_BoxFull.as_ptr().cast_mut(),
            Some(CloseItemMessage),
        );
    } else {
        DisplayItemMessageInBattlePyramid(
            taskId,
            gText_BoxFull.as_ptr().cast_mut(),
            Some(Task_CloseBattlePyramidBagMessage),
        );
    }
}
pub(crate) unsafe extern "C" fn Task_CloseStatIncreaseMessage(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            Task_FadeAndCloseBagMenu(taskId);
        } else {
            CloseBattlePyramidBag(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_UseStatIncreaseItem(taskId: u8) {
    if ({
        gTasks[taskId].data[8] += 1;
        gTasks[taskId].data[8]
    }) > 7
    {
        PlaySE(SE_USE_ITEM);
        RemoveBagItem(gSpecialVar_ItemId, 1);
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                UseStatIncreaseItem(gSpecialVar_ItemId),
                Some(Task_CloseStatIncreaseMessage),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                UseStatIncreaseItem(gSpecialVar_ItemId),
                Some(Task_CloseStatIncreaseMessage),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_StatIncrease(taskId: u8) {
    let mut partyId: u16 = gBattlerPartyIndexes[gBattlerInMenuId];
    if ExecuteTableBasedItemEffect(
        &raw mut gPlayerParty[partyId],
        gSpecialVar_ItemId,
        partyId as u8,
        0,
    ) != 0
    {
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                gText_WontHaveEffect.as_ptr().cast_mut(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                gText_WontHaveEffect.as_ptr().cast_mut(),
                Some(Task_CloseBattlePyramidBagMessage),
            );
        }
    } else {
        gTasks[taskId].func = Some(Task_UseStatIncreaseItem);
        gTasks[taskId].data[8] = 0;
    }
}
pub(crate) unsafe extern "C" fn ItemUseInBattle_ShowPartyMenu(taskId: u8) {
    if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        (*gBagMenu).newScreenCallback = Some(ChooseMonForInBattleItem);
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        (*gPyramidBagMenu).newScreenCallback = Some(ChooseMonForInBattleItem);
        CloseBattlePyramidBag(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_Medicine(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_Medicine);
    ItemUseInBattle_ShowPartyMenu(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_SacredAsh(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_SacredAsh);
    ItemUseInBattle_ShowPartyMenu(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_PPRecovery(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_PPRecovery);
    ItemUseInBattle_ShowPartyMenu(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_Escape(taskId: u8) {
    if gBattleTypeFlags & BATTLE_TYPE_TRAINER == FALSE as u32 {
        RemoveUsedItem();
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            DisplayItemMessage(
                taskId,
                FONT_NORMAL,
                gStringVar4.as_mut_ptr(),
                Some(Task_FadeAndCloseBagMenu),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                gStringVar4.as_mut_ptr(),
                Some(CloseBattlePyramidBag),
            );
        }
    } else {
        DisplayDadsAdviceCannotUseItemMessage(taskId, gTasks[taskId].data[3] as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_EnigmaBerry(taskId: u8) {
    match GetItemEffectType(gSpecialVar_ItemId) {
        ITEM_EFFECT_HEAL_HP
        | ITEM_EFFECT_CURE_POISON
        | ITEM_EFFECT_CURE_SLEEP
        | ITEM_EFFECT_CURE_BURN
        | ITEM_EFFECT_CURE_FREEZE
        | ITEM_EFFECT_CURE_PARALYSIS
        | ITEM_EFFECT_CURE_ALL_STATUS
        | ITEM_EFFECT_ATK_EV
        | ITEM_EFFECT_HP_EV
        | ITEM_EFFECT_SPATK_EV
        | ITEM_EFFECT_SPDEF_EV
        | ITEM_EFFECT_SPEED_EV
        | ITEM_EFFECT_DEF_EV => {
            gTasks[taskId].data[4] = ITEM_USE_PARTY_MENU;
            ItemUseOutOfBattle_Medicine(taskId);
        }
        ITEM_EFFECT_SACRED_ASH => {
            gTasks[taskId].data[4] = ITEM_USE_PARTY_MENU;
            ItemUseOutOfBattle_SacredAsh(taskId);
        }
        ITEM_EFFECT_RAISE_LEVEL => {
            gTasks[taskId].data[4] = ITEM_USE_PARTY_MENU;
            ItemUseOutOfBattle_RareCandy(taskId);
        }
        ITEM_EFFECT_PP_UP | ITEM_EFFECT_PP_MAX => {
            gTasks[taskId].data[4] = ITEM_USE_PARTY_MENU;
            ItemUseOutOfBattle_PPUp(taskId);
        }
        ITEM_EFFECT_HEAL_PP => {
            gTasks[taskId].data[4] = ITEM_USE_PARTY_MENU;
            ItemUseOutOfBattle_PPRecovery(taskId);
        }
        _ => {
            gTasks[taskId].data[4] = ITEM_USE_BAG_MENU;
            ItemUseOutOfBattle_CannotUse(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseInBattle_EnigmaBerry(taskId: u8) {
    match GetItemEffectType(gSpecialVar_ItemId) {
        ITEM_EFFECT_X_ITEM => {
            ItemUseInBattle_StatIncrease(taskId);
        }
        ITEM_EFFECT_HEAL_HP
        | ITEM_EFFECT_CURE_POISON
        | ITEM_EFFECT_CURE_SLEEP
        | ITEM_EFFECT_CURE_BURN
        | ITEM_EFFECT_CURE_FREEZE
        | ITEM_EFFECT_CURE_PARALYSIS
        | ITEM_EFFECT_CURE_ALL_STATUS
        | ITEM_EFFECT_CURE_CONFUSION
        | ITEM_EFFECT_CURE_INFATUATION => {
            ItemUseInBattle_Medicine(taskId);
        }
        ITEM_EFFECT_HEAL_PP => {
            ItemUseInBattle_PPRecovery(taskId);
        }
        _ => {
            ItemUseOutOfBattle_CannotUse(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ItemUseOutOfBattle_CannotUse(taskId: u8) {
    DisplayDadsAdviceCannotUseItemMessage(taskId, gTasks[taskId].data[3] as u8);
}
