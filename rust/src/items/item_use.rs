//! Translated from `src/item_use.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::battle_main::gBattlerPartyIndexes;
use crate::battle_main::{gBattleTypeFlags, gBattlerInMenuId};
use crate::battle_pyramid::CurrentBattlePyramidLocation;
use crate::battle_pyramid_bag::{
    CloseBattlePyramidBag, DisplayItemMessageInBattlePyramid, Task_CloseBattlePyramidBagMessage,
    UpdatePyramidBagCursorPos, UpdatePyramidBagList, gPyramidBagMenu,
};
use crate::berry::{IsPlayerFacingEmptyBerryTreePatch, TryToWaterBerryTree};
use crate::berry_powder::GetBerryPowder;
use crate::bike::{GetOnOffBike, IsBikingDisallowedByPlayer};
#[allow(unused_imports)]
use crate::c::*;
use crate::coins::GetCoins;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagClear, FlagGet, FlagSet, VarGet, VarSet};
use crate::event_object_lock::ScriptUnfreezeObjectEvents;
use crate::event_object_movement::{
    GetObjectEventIdByLocalIdAndMap, GetObjectEventIdByPosition,
    ObjectEventCheckHeldMovementStatus, ObjectEventClearHeldMovement,
    ObjectEventClearHeldMovementIfFinished, UnfreezeObjectEvent,
};
use crate::field_effect::StartEscapeRopeFieldEffect;
use crate::field_player_avatar::{
    GetPlayerFacingDirection, GetXYCoordsOneStepInFrontOfPlayer,
    IsPlayerFacingSurfableFishableWater, PlayerGetDestCoords, PlayerGetElevation,
    PlayerTurnInPlace, StartFishing, TestPlayerAvatarFlags, gObjectEvents,
};
use crate::field_screen_effect::{FadeInFromBlack, FieldCB_ReturnToFieldNoScript};
use crate::field_weather::{FadeScreen, IsWeatherNotFadingIn};
use crate::fieldmap::{
    GetMapConnectionAtPos, GetMapHeaderFromConnection, MapGridGetCollisionAt,
    MapGridGetMetatileBehaviorAt, gMapHeader,
};
use crate::item::{
    CopyItemName, GetItemFieldFunc, GetItemHoldEffectParam, GetItemPocket, GetItemSecondaryId,
    GetItemType, RemoveBagItem,
};
use crate::item_menu::{
    BagMenu_YesNo, CB2_ReturnToBagMenuPocket, CloseItemMessage, DisplayItemMessage,
    Task_FadeAndCloseBagMenu, UpdatePocketItemList, UpdatePocketListPosition, gBagMenu,
    gSpecialVar_ItemId,
};
use crate::mail::ReadMail;
use crate::menu::{ClearDialogWindowAndFrame, DisplayItemMessageOnField};
use crate::menu_helpers::MenuHelpers_IsLinkActive;
use crate::metatile_behavior::{
    MetatileBehavior_IsBridgeOverWaterNoEdge, MetatileBehavior_IsHorizontalRail,
    MetatileBehavior_IsIsolatedHorizontalRail, MetatileBehavior_IsIsolatedVerticalRail,
    MetatileBehavior_IsSurfableWaterOrUnderwater, MetatileBehavior_IsVerticalRail,
    MetatileBehavior_IsWaterfall,
};
use crate::overworld::{
    CB2_ReturnToField, CleanupOverworldWindowsAndTilemaps, IncrementGameStat,
    Overworld_IsBikingAllowed, Overworld_ResetStateAfterDigEscRope, ResetInitialPlayerAvatarState,
    gFieldCallback,
};
use crate::palette::gPaletteFade;
use crate::party_menu::{
    ChooseMonForInBattleItem, GetItemEffectType, ItemIdToBattleMoveId, ItemUseCB_EvolutionStone,
    ItemUseCB_Medicine, ItemUseCB_PPRecovery, ItemUseCB_PPUp, ItemUseCB_RareCandy,
    ItemUseCB_ReduceEV, ItemUseCB_SacredAsh, ItemUseCB_TMHM, gItemUseCB,
};
use crate::pokeblock::OpenPokeblockCase;
use crate::pokemon::{
    ExecuteTableBasedItemEffect, IsPlayerPartyAndPokemonStorageFull, UseStatIncreaseItem,
    gPlayerParty,
};
use crate::script::ScriptContext_SetupScript;
use crate::script::{LockPlayerFieldControls, UnlockPlayerFieldControls};
use crate::sound::{IsSEPlaying, PlaySE};
use crate::string_util::{ConvertIntToDecimalStringN, StringCopy, StringExpandPlaceholders};
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set, task_set_func};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
// The C's names for task and sprite data slots.
const tItemFound: usize = 2;
const tUsingRegisteredKeyItem: usize = 3;
const tEnigmaBerryType: usize = 4;
// Data tables (translate with cdata.py): sItemUseCallbacks sClockwiseDirections sUseTMHMYesNoFuncTable

static sClockwiseDirections: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::item_use::sClockwiseDirections).cast());
static sItemUseCallbacks: Table<CArray<Option<unsafe fn()>, 3>> =
    Table((&raw const crate::data::item_use::sItemUseCallbacks).cast());
static sUseTMHMYesNoFuncTable: Table<YesNoFuncTable> =
    Table((&raw const crate::data::item_use::sUseTMHMYesNoFuncTable).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sItemUseOnFieldCB: Option<unsafe fn(u8)> = None;

unsafe fn SetUpItemUseCallback(taskId: u8) {
    let mut r#type: u8 = 0;
    if gSpecialVar_ItemId == ITEM_ENIGMA_BERRY {
        r#type = task_get(taskId, tEnigmaBerryType) as u8 - 1;
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
unsafe fn SetUpItemUseOnFieldCallback(taskId: u8) {
    if task_get(taskId, tUsingRegisteredKeyItem) != TRUE as i16 {
        gFieldCallback = Some(FieldCB_UseItemOnField);
        SetUpItemUseCallback(taskId);
    } else {
        sItemUseOnFieldCB.unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe fn FieldCB_UseItemOnField() {
    FadeInFromBlack();
    CreateTask(Some(Task_CallItemUseOnFieldCallback), 8);
}
pub(crate) unsafe fn Task_CallItemUseOnFieldCallback(taskId: u8) {
    if IsWeatherNotFadingIn() == 1 {
        sItemUseOnFieldCB.unwrap_unchecked()(taskId);
    }
}
unsafe fn DisplayCannotUseItemMessage(
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
                (*(&raw const crate::data::strings::gText_DadsAdvice).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
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
unsafe fn DisplayDadsAdviceCannotUseItemMessage(taskId: u8, isUsingRegisteredKeyItemOnField: u8) {
    DisplayCannotUseItemMessage(
        taskId,
        isUsingRegisteredKeyItemOnField,
        (*(&raw const crate::data::strings::gText_DadsAdvice).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
unsafe fn DisplayCannotDismountBikeMessage(taskId: u8, isUsingRegisteredKeyItemOnField: u8) {
    DisplayCannotUseItemMessage(
        taskId,
        isUsingRegisteredKeyItemOnField,
        (*(&raw const crate::data::strings::gText_CantDismountBike).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
pub(crate) unsafe fn Task_CloseCantUseKeyItemMessage(taskId: u8) {
    ClearDialogWindowAndFrame(0, TRUE);
    DestroyTask(taskId);
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
}
pub fn CheckIfItemIsTMHMOrEvolutionStone(itemId: u16) -> u8 {
    if GetItemFieldFunc(itemId) == Some(ItemUseOutOfBattle_TMHM as unsafe fn(u8)) {
        return ITEM_IS_TM_HM;
    } else if GetItemFieldFunc(itemId) == Some(ItemUseOutOfBattle_EvolutionStone as unsafe fn(u8)) {
        return ITEM_IS_EVOLUTION_STONE;
    } else {
        return ITEM_IS_OTHER;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn CB2_CheckMail() {
    let mut mail: Mail = zeroed();
    mail.itemId = gSpecialVar_ItemId;
    ReadMail(&raw mut mail, Some(CB2_ReturnToBagMenuPocket), FALSE);
}
pub unsafe fn ItemUseOutOfBattle_Mail(taskId: u8) {
    (*gBagMenu).newScreenCallback = Some(CB2_CheckMail);
    Task_FadeAndCloseBagMenu(taskId);
}
pub unsafe fn ItemUseOutOfBattle_Bike(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    let mut coordsY: i16 = 0;
    let mut coordsX: i16 = 0;
    PlayerGetDestCoords(&raw mut coordsX, &raw mut coordsY);
    let behavior: u8 = MapGridGetMetatileBehaviorAt(coordsX as i32, coordsY as i32) as u8;
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
pub(crate) unsafe fn ItemUseOnFieldCB_Bike(taskId: u8) {
    if GetItemSecondaryId(gSpecialVar_ItemId) == MACH_BIKE {
        GetOnOffBike(PLAYER_AVATAR_FLAG_MACH_BIKE);
    } else {
        GetOnOffBike(PLAYER_AVATAR_FLAG_ACRO_BIKE);
    }
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
    DestroyTask(taskId);
}
unsafe fn CanFish() -> u32 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let tileBehavior: u16 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
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
    FALSE as u32
}
pub unsafe fn ItemUseOutOfBattle_Rod(taskId: u8) {
    if CanFish() == TRUE as u32 {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Rod);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(
            taskId,
            task_get(taskId, tUsingRegisteredKeyItem) as u8,
        );
    }
}
pub(crate) unsafe fn ItemUseOnFieldCB_Rod(taskId: u8) {
    StartFishing(GetItemSecondaryId(gSpecialVar_ItemId));
    DestroyTask(taskId);
}
pub unsafe fn ItemUseOutOfBattle_Itemfinder(var: u8) {
    IncrementGameStat(GAME_STAT_USED_ITEMFINDER);
    sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Itemfinder);
    SetUpItemUseOnFieldCallback(var);
}
pub(crate) unsafe fn ItemUseOnFieldCB_Itemfinder(taskId: u8) {
    if ItemfinderCheckForHiddenItems(gMapHeader.events, taskId) == TRUE {
        task_set_func(taskId, Some(Task_UseItemfinder));
    } else {
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_ItemFinderNothing).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Task_CloseItemfinderMessage),
        );
    }
}
pub(crate) unsafe fn Task_UseItemfinder(taskId: u8) {
    let mut playerDir: u8 = 0;
    let mut playerDirToItem: u8 = 0;
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(3) == 0 {
        if *data.at(4) == 4 {
            playerDirToItem = GetDirectionToHiddenItem(*data, *data.at(1));
            if playerDirToItem != DIR_NONE {
                PlayerFaceHiddenItem(sClockwiseDirections[playerDirToItem as i32 - 1]);
                task_set_func(taskId, Some(Task_HiddenItemNearby));
            } else {
                playerDir = GetPlayerFacingDirection();
                for i in 0..4u8 {
                    if playerDir == sClockwiseDirections[i] {
                        *data.at(5) = (i as i16 + 1) & 3;
                    }
                }
                task_set_func(taskId, Some(Task_StandingOnHiddenItem));
                *data.at(3) = 0;
                *data.at(2) = 0;
            }
            return;
        }
        PlaySE(SE_ITEMFINDER);
        *data.at(4) += 1;
    }
    *data.at(3) = (*data.at(3) + 1) & 0x1F;
}
pub(crate) unsafe fn Task_CloseItemfinderMessage(taskId: u8) {
    ClearDialogWindowAndFrame(0, TRUE);
    ScriptUnfreezeObjectEvents();
    UnlockPlayerFieldControls();
    DestroyTask(taskId);
}
unsafe fn ItemfinderCheckForHiddenItems(events: *mut MapEvents, taskId: u8) -> u8 {
    let mut playerX: i16 = 0;
    let mut playerY: i16 = 0;
    let mut distanceX: i16 = 0;
    let mut distanceY: i16 = 0;
    PlayerGetDestCoords(&raw mut playerX, &raw mut playerY);
    task_set(taskId, tItemFound, FALSE as i16);
    let mut i: i16 = 0;
    while i < (*events).bgEventCount as i16 {
        if (*(*events).bgEvents.at(i)).kind == BG_EVENT_HIDDEN_ITEM
            && FlagGet(
                (*(*events).bgEvents.at(i)).bgUnion.hiddenItem.hiddenItemId
                    + FLAG_HIDDEN_ITEMS_START,
            ) == 0
        {
            distanceX = (*(*events).bgEvents.at(i)).x as i16 + MAP_OFFSET as i16 - playerX;
            distanceY = (*(*events).bgEvents.at(i)).y as i16 + MAP_OFFSET as i16 - playerY;
            if (-7..=7).contains(&distanceX) && (-5..=5).contains(&distanceY) {
                SetDistanceOfClosestHiddenItem(taskId, distanceX, distanceY);
            }
        }
        i += 1;
    }
    CheckForHiddenItemsInMapConnection(taskId);
    if task_get(taskId, tItemFound) == TRUE as i16 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsHiddenItemPresentAtCoords(events: *mut MapEvents, x: i16, y: i16) -> u8 {
    let bgEventCount: u8 = (*events).bgEventCount;
    let bgEvent: *mut BgEvent = (*events).bgEvents;
    for i in 0..(bgEventCount as i32) {
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
    }
    FALSE
}
unsafe fn IsHiddenItemPresentInConnection(connection: *mut MapConnection, x: i32, y: i32) -> u8 {
    let mut connectionX: i16 = 0;
    let mut connectionY: i16 = 0;
    let connectionHeader: *mut MapHeader = GetMapHeaderFromConnection(connection);
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
    IsHiddenItemPresentAtCoords((*connectionHeader).events, connectionX, connectionY)
}
unsafe fn CheckForHiddenItemsInMapConnection(taskId: u8) {
    let mut playerX: i16 = 0;
    let mut playerY: i16 = 0;
    let mut y: i16 = 0;
    let width: i16 = (*gMapHeader.mapLayout).width as i16 + MAP_OFFSET as i16;
    let height: i16 = (*gMapHeader.mapLayout).height as i16 + MAP_OFFSET as i16;
    let var1: i16 = MAP_OFFSET as i16;
    let var2: i16 = MAP_OFFSET as i16;
    PlayerGetDestCoords(&raw mut playerX, &raw mut playerY);
    let mut x: i16 = playerX - 7;
    while x as i32 <= playerX as i32 + 7 {
        y = playerY - 5;
        while y as i32 <= playerY as i32 + 5 {
            if var1 > x || x >= width || var2 > y || y >= height {
                let conn: *mut MapConnection = GetMapConnectionAtPos(x, y);
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
unsafe fn SetDistanceOfClosestHiddenItem(taskId: u8, itemDistanceX: i16, itemDistanceY: i16) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
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
        newItemAbsX = -itemDistanceX;
    } else {
        newItemAbsX = itemDistanceX;
    }
    if itemDistanceY < 0 {
        newItemAbsY = -itemDistanceY;
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
unsafe fn GetDirectionToHiddenItem(itemDistanceX: i16, itemDistanceY: i16) -> u8 {
    let mut absX: i16 = 0;
    let mut absY: i16 = 0;
    if itemDistanceX == 0 && itemDistanceY == 0 {
        return DIR_NONE;
    }
    if itemDistanceX < 0 {
        absX = -itemDistanceX;
    } else {
        absX = itemDistanceX;
    }
    if itemDistanceY < 0 {
        absY = -itemDistanceY;
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
        0
    }
}
unsafe fn PlayerFaceHiddenItem(direction: u8) {
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
pub(crate) unsafe fn Task_HiddenItemNearby(taskId: u8) {
    if ObjectEventCheckHeldMovementStatus(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    ) == TRUE
    {
        DisplayItemMessageOnField(
            taskId,
            (*(&raw const crate::data::strings::gText_ItemFinderNearby).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Task_CloseItemfinderMessage),
        );
    }
}
pub(crate) unsafe fn Task_StandingOnHiddenItem(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if ObjectEventCheckHeldMovementStatus(
        &raw mut gObjectEvents[GetObjectEventIdByLocalIdAndMap(LOCALID_PLAYER, 0, 0)],
    ) == TRUE
        || *data.at(2) == FALSE as i16
    {
        PlayerFaceHiddenItem(sClockwiseDirections[*data.at(5)]);
        *data.at(2) = TRUE as i16;
        *data.at(5) = (*data.at(5) + 1) & 3;
        *data.at(3) += 1;
        if *data.at(3) == 4 {
            DisplayItemMessageOnField(
                taskId,
                (*(&raw const crate::data::strings::gText_ItemFinderOnTop).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                Some(Task_CloseItemfinderMessage),
            );
        }
    }
}
pub unsafe fn ItemUseOutOfBattle_PokeblockCase(taskId: u8) {
    if MenuHelpers_IsLinkActive() == TRUE {
        DisplayDadsAdviceCannotUseItemMessage(
            taskId,
            task_get(taskId, tUsingRegisteredKeyItem) as u8,
        );
    } else if task_get(taskId, tUsingRegisteredKeyItem) != TRUE as i16 {
        (*gBagMenu).newScreenCallback = Some(CB2_OpenPokeblockFromBag);
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        gFieldCallback = Some(FieldCB_ReturnToFieldNoScript);
        FadeScreen(FADE_TO_BLACK, 0);
        task_set_func(taskId, Some(Task_OpenRegisteredPokeblockCase));
    }
}
pub(crate) unsafe fn CB2_OpenPokeblockFromBag() {
    OpenPokeblockCase(PBLOCK_CASE_FIELD, Some(CB2_ReturnToBagMenuPocket));
}
pub(crate) unsafe fn Task_OpenRegisteredPokeblockCase(taskId: u8) {
    if gPaletteFade.active() == 0 {
        CleanupOverworldWindowsAndTilemaps();
        OpenPokeblockCase(PBLOCK_CASE_FIELD, Some(CB2_ReturnToField));
        DestroyTask(taskId);
    }
}
pub unsafe fn ItemUseOutOfBattle_CoinCase(taskId: u8) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetCoins() as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        4,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_CoinCase).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    if task_get(taskId, tUsingRegisteredKeyItem) == 0 {
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
pub unsafe fn ItemUseOutOfBattle_PowderJar(taskId: u8) {
    ConvertIntToDecimalStringN(
        gStringVar1.as_mut_ptr(),
        GetBerryPowder() as i32,
        STR_CONV_MODE_LEFT_ALIGN,
        5,
    );
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PowderQty).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    if task_get(taskId, tUsingRegisteredKeyItem) == 0 {
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
pub unsafe fn ItemUseOutOfBattle_Berry(taskId: u8) {
    if IsPlayerFacingEmptyBerryTreePatch() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_Berry);
        gFieldCallback = Some(FieldCB_UseItemOnField);
        (*gBagMenu).newScreenCallback = Some(CB2_ReturnToField);
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        GetItemFieldFunc(gSpecialVar_ItemId).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe fn ItemUseOnFieldCB_Berry(taskId: u8) {
    RemoveBagItem(gSpecialVar_ItemId, 1);
    LockPlayerFieldControls();
    ScriptContext_SetupScript(
        (*crate::asmdata::BerryTree_EventScript_ItemUsePlantBerry.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DestroyTask(taskId);
}
pub unsafe fn ItemUseOutOfBattle_WailmerPail(taskId: u8) {
    if TryToWaterSudowoodo() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_WailmerPailSudowoodo);
        SetUpItemUseOnFieldCallback(taskId);
    } else if TryToWaterBerryTree() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_WailmerPailBerry);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(
            taskId,
            task_get(taskId, tUsingRegisteredKeyItem) as u8,
        );
    }
}
pub(crate) unsafe fn ItemUseOnFieldCB_WailmerPailBerry(taskId: u8) {
    LockPlayerFieldControls();
    ScriptContext_SetupScript(
        (*crate::asmdata::BerryTree_EventScript_ItemUseWailmerPail.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    DestroyTask(taskId);
}
unsafe fn TryToWaterSudowoodo() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let elevation: u8 = PlayerGetElevation();
    let objId: u8 = GetObjectEventIdByPosition(x as u16, y as u16, elevation);
    if objId == OBJECT_EVENTS_COUNT || gObjectEvents[objId].graphicsId != OBJ_EVENT_GFX_SUDOWOODO {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn ItemUseOnFieldCB_WailmerPailSudowoodo(taskId: u8) {
    LockPlayerFieldControls();
    ScriptContext_SetupScript(
        (*crate::asmdata::BattleFrontier_OutsideEast_EventScript_WaterSudowoodo
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut(),
    );
    DestroyTask(taskId);
}
pub unsafe fn ItemUseOutOfBattle_Medicine(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_Medicine);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseOutOfBattle_ReduceEV(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_ReduceEV);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseOutOfBattle_SacredAsh(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_SacredAsh);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseOutOfBattle_PPRecovery(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_PPRecovery);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseOutOfBattle_PPUp(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_PPUp);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseOutOfBattle_RareCandy(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_RareCandy);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseOutOfBattle_TMHM(taskId: u8) {
    if gSpecialVar_ItemId >= ITEM_HM01 {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_BootedUpHM).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(BootUpSoundTMHM),
        );
    } else {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_BootedUpTM).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(BootUpSoundTMHM),
        );
    }
}
pub(crate) unsafe fn BootUpSoundTMHM(taskId: u8) {
    PlaySE(SE_PC_LOGIN);
    task_set_func(taskId, Some(Task_ShowTMHMContainedMessage));
}
pub(crate) unsafe fn Task_ShowTMHMContainedMessage(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[ItemIdToBattleMoveId(gSpecialVar_ItemId)]
            .as_ptr()
            .cast_mut(),
        );
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_TMHMContainedVar1).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            gStringVar4.as_mut_ptr(),
            Some(UseTMHMYesNo),
        );
    }
}
pub(crate) unsafe fn UseTMHMYesNo(taskId: u8) {
    BagMenu_YesNo(
        taskId,
        ITEMWIN_YESNO_HIGH,
        (&raw const *sUseTMHMYesNoFuncTable).cast_mut(),
    );
}
pub(crate) unsafe fn UseTMHM(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_TMHM);
    SetUpItemUseCallback(taskId);
}
unsafe fn RemoveUsedItem() {
    RemoveBagItem(gSpecialVar_ItemId, 1);
    CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
    StringExpandPlaceholders(
        gStringVar4.as_mut_ptr(),
        (*(&raw const crate::data::strings::gText_PlayerUsedVar2).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        UpdatePocketItemList(GetItemPocket(gSpecialVar_ItemId));
        UpdatePocketListPosition(GetItemPocket(gSpecialVar_ItemId));
    } else {
        UpdatePyramidBagList();
        UpdatePyramidBagCursorPos();
    }
}
pub unsafe fn ItemUseOutOfBattle_Repel(taskId: u8) {
    if VarGet(VAR_REPEL_STEP_COUNT) == 0 {
        task_set_func(taskId, Some(Task_StartUseRepel));
    } else if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        DisplayItemMessage(
            taskId,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_RepelEffectsLingered)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            Some(CloseItemMessage),
        );
    } else {
        DisplayItemMessageInBattlePyramid(
            taskId,
            (*(&raw const crate::data::strings::gText_RepelEffectsLingered)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
            Some(Task_CloseBattlePyramidBagMessage),
        );
    }
}
pub(crate) unsafe fn Task_StartUseRepel(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if ({
        *data.at(8) += 1;
        *data.at(8)
    }) > 7
    {
        *data.at(8) = 0;
        PlaySE(SE_REPEL as u16);
        task_set_func(taskId, Some(Task_UseRepel));
    }
}
pub(crate) unsafe fn Task_UseRepel(taskId: u8) {
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
pub(crate) unsafe fn Task_UsedBlackWhiteFlute(taskId: u8) {
    if ({
        task_set(taskId, 8, task_get(taskId, 8) + 1);
        task_get(taskId, 8)
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
pub unsafe fn ItemUseOutOfBattle_BlackWhiteFlute(taskId: u8) {
    CopyItemName(gSpecialVar_ItemId, gStringVar2.as_mut_ptr());
    if gSpecialVar_ItemId == ITEM_WHITE_FLUTE {
        FlagSet(FLAG_SYS_ENC_UP_ITEM);
        FlagClear(FLAG_SYS_ENC_DOWN_ITEM);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_UsedVar2WildLured).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        FlagSet(FLAG_SYS_ENC_DOWN_ITEM);
        FlagClear(FLAG_SYS_ENC_UP_ITEM);
        StringExpandPlaceholders(
            gStringVar4.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_UsedVar2WildRepelled)
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        );
    }
    task_set(taskId, 8, 0);
    task_set_func(taskId, Some(Task_UsedBlackWhiteFlute));
}
#[unsafe(no_mangle)]
pub unsafe fn Task_UseDigEscapeRopeOnField(taskId: u8) {
    ResetInitialPlayerAvatarState();
    StartEscapeRopeFieldEffect();
    DestroyTask(taskId);
}
pub(crate) unsafe fn ItemUseOnFieldCB_EscapeRope(taskId: u8) {
    Overworld_ResetStateAfterDigEscRope();
    RemoveUsedItem();
    task_set(taskId, 0, 0);
    DisplayItemMessageOnField(
        taskId,
        gStringVar4.as_mut_ptr(),
        Some(Task_UseDigEscapeRopeOnField),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn CanUseDigOrEscapeRopeOnCurMap() -> u8 {
    if gMapHeader.allowEscaping() != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ItemUseOutOfBattle_EscapeRope(taskId: u8) {
    if CanUseDigOrEscapeRopeOnCurMap() == TRUE {
        sItemUseOnFieldCB = Some(ItemUseOnFieldCB_EscapeRope);
        SetUpItemUseOnFieldCallback(taskId);
    } else {
        DisplayDadsAdviceCannotUseItemMessage(
            taskId,
            task_get(taskId, tUsingRegisteredKeyItem) as u8,
        );
    }
}
pub unsafe fn ItemUseOutOfBattle_EvolutionStone(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_EvolutionStone);
    SetUpItemUseCallback(taskId);
}
pub unsafe fn ItemUseInBattle_PokeBall(taskId: u8) {
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
            (*(&raw const crate::data::strings::gText_BoxFull).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(CloseItemMessage),
        );
    } else {
        DisplayItemMessageInBattlePyramid(
            taskId,
            (*(&raw const crate::data::strings::gText_BoxFull).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            Some(Task_CloseBattlePyramidBagMessage),
        );
    }
}
pub(crate) unsafe fn Task_CloseStatIncreaseMessage(taskId: u8) {
    if gMain.newKeys as i32 & 3 != 0 {
        if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
            Task_FadeAndCloseBagMenu(taskId);
        } else {
            CloseBattlePyramidBag(taskId);
        }
    }
}
pub(crate) unsafe fn Task_UseStatIncreaseItem(taskId: u8) {
    if ({
        task_set(taskId, 8, task_get(taskId, 8) + 1);
        task_get(taskId, 8)
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
pub unsafe fn ItemUseInBattle_StatIncrease(taskId: u8) {
    let partyId: u16 = gBattlerPartyIndexes[gBattlerInMenuId];
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
                (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                Some(CloseItemMessage),
            );
        } else {
            DisplayItemMessageInBattlePyramid(
                taskId,
                (*(&raw const crate::data::strings::gText_WontHaveEffect).cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
                Some(Task_CloseBattlePyramidBagMessage),
            );
        }
    } else {
        task_set_func(taskId, Some(Task_UseStatIncreaseItem));
        task_set(taskId, 8, 0);
    }
}
unsafe fn ItemUseInBattle_ShowPartyMenu(taskId: u8) {
    if CurrentBattlePyramidLocation() == PYRAMID_LOCATION_NONE {
        (*gBagMenu).newScreenCallback = Some(ChooseMonForInBattleItem);
        Task_FadeAndCloseBagMenu(taskId);
    } else {
        (*gPyramidBagMenu).newScreenCallback = Some(ChooseMonForInBattleItem);
        CloseBattlePyramidBag(taskId);
    }
}
pub unsafe fn ItemUseInBattle_Medicine(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_Medicine);
    ItemUseInBattle_ShowPartyMenu(taskId);
}
pub unsafe fn ItemUseInBattle_SacredAsh(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_SacredAsh);
    ItemUseInBattle_ShowPartyMenu(taskId);
}
pub unsafe fn ItemUseInBattle_PPRecovery(taskId: u8) {
    gItemUseCB = Some(ItemUseCB_PPRecovery);
    ItemUseInBattle_ShowPartyMenu(taskId);
}
pub unsafe fn ItemUseInBattle_Escape(taskId: u8) {
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
        DisplayDadsAdviceCannotUseItemMessage(
            taskId,
            task_get(taskId, tUsingRegisteredKeyItem) as u8,
        );
    }
}
pub unsafe fn ItemUseOutOfBattle_EnigmaBerry(taskId: u8) {
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
            task_set(taskId, tEnigmaBerryType, ITEM_USE_PARTY_MENU);
            ItemUseOutOfBattle_Medicine(taskId);
        }
        ITEM_EFFECT_SACRED_ASH => {
            task_set(taskId, tEnigmaBerryType, ITEM_USE_PARTY_MENU);
            ItemUseOutOfBattle_SacredAsh(taskId);
        }
        ITEM_EFFECT_RAISE_LEVEL => {
            task_set(taskId, tEnigmaBerryType, ITEM_USE_PARTY_MENU);
            ItemUseOutOfBattle_RareCandy(taskId);
        }
        ITEM_EFFECT_PP_UP | ITEM_EFFECT_PP_MAX => {
            task_set(taskId, tEnigmaBerryType, ITEM_USE_PARTY_MENU);
            ItemUseOutOfBattle_PPUp(taskId);
        }
        ITEM_EFFECT_HEAL_PP => {
            task_set(taskId, tEnigmaBerryType, ITEM_USE_PARTY_MENU);
            ItemUseOutOfBattle_PPRecovery(taskId);
        }
        _ => {
            task_set(taskId, tEnigmaBerryType, ITEM_USE_BAG_MENU);
            ItemUseOutOfBattle_CannotUse(taskId);
        }
    }
}
pub unsafe fn ItemUseInBattle_EnigmaBerry(taskId: u8) {
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
pub unsafe fn ItemUseOutOfBattle_CannotUse(taskId: u8) {
    DisplayDadsAdviceCannotUseItemMessage(taskId, task_get(taskId, tUsingRegisteredKeyItem) as u8);
}
