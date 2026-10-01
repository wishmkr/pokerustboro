//! Translated from `src/field_specials.c` by tools/rustport/c2rs.py.
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
    clippy::unnecessary_cast,
    clippy::useless_transmute,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_main::{gBattleOutcome, gBattleTypeFlags};
use crate::battle_tower::GetEreaderTrainerName;
use crate::box_mon::GetBoxMonData3;
#[allow(unused_imports)]
use crate::c::*;
use crate::cable_club::Task_ReconnectWithLinkPlayers;
#[allow(unused_imports)]
use crate::consts::*;
use crate::decoration::AddDecorationIconObject;
use crate::diploma::CB2_ShowDiploma;
use crate::event_data::{FlagClear, FlagGet, FlagSet, GetVarPointer, VarGet, VarSet};
use crate::event_object_movement::{
    CameraObjectSetFollowedSpriteId, GetObjectEventIdByLocalIdAndMap,
    RemoveObjectEventByLocalIdAndMap, SpawnSpecialObjectEventParameterized,
    TryGetObjectEventIdByLocalIdAndMap,
};
use crate::ffi::{
    gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_0x8006, gSpecialVar_0x8007,
    gSpecialVar_Result,
};
use crate::field_camera::{
    DrawWholeMapView, InstallCameraPanAheadCallback, SetCameraPanning, SetCameraPanningCallback,
};
use crate::field_effect::{FieldEffectActiveListContains, FieldEffectStart, gFieldEffectArguments};
use crate::field_player_avatar::{
    GetPlayerAvatarSpriteId, GetPlayerFacingDirection, TestPlayerAvatarFlags, gObjectEvents,
};
use crate::field_weather::SetCurrentAndNextWeather;
use crate::field_weather_effect::SetSavedWeather;
use crate::fieldmap::{MapGridGetMetatileIdAt, MapGridSetMetatileIdAt, gMapHeader};
use crate::item_icon::AddItemIconSprite;
use crate::link::gBlockRecvBuffer;
use crate::link::{
    BitmaskAllOtherLinkPlayers, GetBlockReceivedStatus, GetLinkPlayerCount, GetMultiplayerId,
    IsLinkTaskFinished, ResetBlockReceivedFlag, SendBlock, SetCloseLinkCallback,
    SetLinkStandbyCallback, gLinkPlayers, gReceivedRemoteLinkPlayers, gWirelessCommType,
};
use crate::list_menu::{
    AddScrollIndicatorArrowPair, DestroyListMenuTask, ListMenu_ProcessInput,
    ListMenuGetCurrentItemArrayId, ListMenuGetScrollAndRow, ListMenuInit,
    RemoveScrollIndicatorArrowPair,
};
use crate::load_save::{gSaveBlock1Ptr, gSaveBlock2Ptr};
use crate::menu::{
    AddTextPrinterParameterized2, AddTextPrinterParameterized5,
    ClearStdWindowAndFrameToTransparent, CreateWindowTemplate, ScheduleBgCopyTilemapToVram,
    SetStandardWindowBorderStyle,
};
use crate::mystery_gift::MysteryGift_GetCardStat;
use crate::overworld::{
    CB2_ReturnToField, CB2_ReturnToFieldContinueScriptPlayMapMusic, GetGameStat,
    GetLastUsedWarpMapType, IncrementGameStat, IsMapTypeOutdoors, Overworld_SetSavedMusic,
    SetLastHealLocationWarp, SetObjEventTemplateCoords, SetWarpDestination, gLastUsedWarp,
};
use crate::palette::{BlendPalettes, LoadPalette};
use crate::party_menu::ItemIdToBattleMoveId;
use crate::pokeblock::CopyMonFavoritePokeblockName;
use crate::pokemon::{
    CalculatePlayerPartyCount, CheckPartyPokerus, CreateMon, GetMonData2, GetMonData3,
    GetMonEVCount, GetNature, SetMonData, gPlayerParty,
};
use crate::pokemon_storage_system::{
    CheckFreePokemonStorageSpace, GetBoxedMonPtr, StorageGetCurrentBox,
};
use crate::pokenav_match_call_data::GetRematchIdxByTrainerIdx;
use crate::random::Random;
use crate::rayquaza_scene::DoRayquazaScene;
use crate::region_map::GetMapName;
use crate::rtc::gLocalTime;
use crate::script::{LockPlayerFieldControls, ScriptContext_Enable};
use crate::script_menu::{ConvertPixelWidthToTileWidth, DisplayTextAndGetWidth};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{FreeSpritePaletteByTag, FreeSpriteTilesByTag};
use crate::starter_choose::GetStarterPokemon;
use crate::string_util::ConvertInternationalString;
use crate::string_util::{gStringVar1, gStringVar2, gStringVar4};
use crate::task::DestroyTask;
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::text::IsTextPrinterActive;
use crate::tv::{
    ConvertIntToDecimalString, CountDigits, GetRibbonCount, IsPokeNewsActive,
    TryPutSpotTheCutiesOnAir,
};
#[allow(unused_imports)]
use crate::types::*;
use crate::wallclock::CB2_ViewWallClock;
use crate::window::{
    ClearWindowTilemap, CopyWindowToVram, FillWindowPixelBuffer, FillWindowPixelRect,
    PutWindowTilemap, RemoveWindow,
};
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
/// `ConvertIntToDecimalStringN` with this module's view of its types.
#[inline]
unsafe fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8 {
    unsafe { crate::string_util::ConvertIntToDecimalStringN(a0 as _, a1, a2, a3) as *mut u8 }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySpriteAndFreeResources` with this module's view of its types.
#[inline]
unsafe fn DestroySpriteAndFreeResources(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySpriteAndFreeResources(a0 as _);
    }
}
/// `FieldInitRegionMap` with this module's view of its types.
#[inline]
unsafe fn FieldInitRegionMap(a0: Option<unsafe fn()>) {
    unsafe {
        crate::field_region_map::FieldInitRegionMap(core::mem::transmute(a0));
    }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `GetStringCenterAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringCenterAlignXOffset(a0, a1 as _, a2) }
}
/// `GetStringRightAlignXOffset` with this module's view of its types.
#[inline]
unsafe fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32 {
    unsafe { crate::international_string_util::GetStringRightAlignXOffset(a0, a1 as _, a2) }
}
/// `ShowFieldAutoScrollMessage` with this module's view of its types.
#[inline]
unsafe fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8 {
    unsafe { crate::field_message_box::ShowFieldAutoScrollMessage(a0 as _) }
}
/// `ShowFieldMessage` with this module's view of its types.
#[inline]
unsafe fn ShowFieldMessage(a0: *mut u8) -> u8 {
    unsafe { crate::field_message_box::ShowFieldMessage(a0 as _) }
}
/// `StringAppend` with this module's view of its types.
#[inline]
unsafe fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringAppend(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCompare` with this module's view of its types.
#[inline]
unsafe fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32 {
    unsafe { crate::string_util::StringCompare(a0 as _, a1 as _) }
}
/// `StringCopy` with this module's view of its types.
#[inline]
unsafe fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopy(a0 as _, a1 as _) as *mut u8 }
}
/// `StringCopyN` with this module's view of its types.
#[inline]
unsafe fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8 {
    unsafe { crate::string_util::StringCopyN(a0 as _, a1 as _, a2) as *mut u8 }
}
// The C's names for task and sprite data slots.
const tHorizontalPan: usize = 0;
const tMaxItemsOnScreen: usize = 0;
const tPaused: usize = 0;
const tState: usize = 0;
const tDelayCounter: usize = 1;
const tNumItems: usize = 1;
const tFlickerCount: usize = 2;
const tLeft: usize = 2;
const tNumShakes: usize = 2;
const tCurrentFrame: usize = 3;
const tDelay: usize = 3;
const tTop: usize = 3;
const tIsScreenOn: usize = 4;
const tVerticalPan: usize = 4;
const tWidth: usize = 4;
const tHeight: usize = 5;
const tKeepOpenAfterSelect: usize = 6;
const tScrollOffset: usize = 7;
const tSelectedRow: usize = 8;
const tScrollMultiId: usize = 11;
const tScrollArrowId: usize = 12;
const tWindowId: usize = 13;
const tListTaskId: usize = 14;
// Data tables (translate with cdata.py): sMauvilleGymSwitchCoords sSlidingDoorNextFrameDelay sPetalburgGymSlidingDoorMetatiles sWindowTemplate_ElevatorFloor sDeptStoreFloorNames sElevatorWindowTiles_Ascending sElevatorWindowTiles_Descending sScrollableMultichoiceOptions sBattleFrontier_TutorMoves1 sBattleFrontier_TutorMoves2 sDeoxysRockPalettes sDeoxysRockCoords sAbnormalWeatherMapNumbers.5 sAbnormalWeatherMapNumbers.6 sBattleFrontierTutor_WindowTemplate.10 sBattleFrontier_TutorMoveDescriptions1.8 sBattleFrontier_TutorMoveDescriptions2.9 sBattlePoints_WindowTemplate.20 sBattleTowerStreakThresholds.26 sCounterIncrements.2 sElevatorLightCycles.30 sElevatorTripLength.31 sFanClubMemberIds.0 sFanClubMemberIds.1 sFrontierChallenges.21 sFrontierExchangeCorner_Decor1.17 sFrontierExchangeCorner_Decor1Descriptions.18 sFrontierExchangeCorner_Decor2.15 sFrontierExchangeCorner_Decor2Descriptions.16 sFrontierExchangeCorner_HoldItems.11 sFrontierExchangeCorner_HoldItemsDescriptions.12 sFrontierExchangeCorner_ItemIconWindowTemplate.19 sFrontierExchangeCorner_Vitamins.13 sFrontierExchangeCorner_VitaminsDescriptions.14 sFrontierGamblerGoMessages.22 sFrontierGamblerLookingMessages.23 sFrontierManiacMessages.27 sFrontierManiacStreakThresholds.28 sNatureGirlMessages.24 sPokeMarts.4 sPokemonCenters.29 sPokemonCenters.3 sScrollableMultichoice_ScrollArrowsTemplate.25 sSlotMachineIds.32 sSlotMachineRandomSeeds.34 sSlotMachineServiceDayIds.33 sStoneMaxStepCounts.7

const CURTAIN_HEIGHT: u8 = 4;
const CURTAIN_WIDTH: u8 = 3;
const DEOXYS_ROCK_LEVELS: i32 = 11;
const ELEVATOR_LIGHT_STAGES: i32 = 3;
const ELEVATOR_WINDOW_HEIGHT: u8 = 3;
const ELEVATOR_WINDOW_WIDTH: u8 = 3;
const MAX_ELEVATOR_TRIP: i32 = 9;
const ROCK_PAL_ID: i32 = 10;
const TAG_ITEM_ICON: u16 = 5500;

static sAbnormalWeatherMapNumbers_5: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::field_specials::sAbnormalWeatherMapNumbers_5).cast());
static sAbnormalWeatherMapNumbers_6: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::field_specials::sAbnormalWeatherMapNumbers_6).cast());
static sBattleFrontierTutor_WindowTemplate_10: Table<WindowTemplate> =
    Table((&raw const crate::data::field_specials::sBattleFrontierTutor_WindowTemplate_10).cast());
static sBattleFrontier_TutorMoveDescriptions1_8: Table<CArray<*mut u8, 11>> = Table(
    (&raw const crate::data::field_specials::sBattleFrontier_TutorMoveDescriptions1_8).cast(),
);
static sBattleFrontier_TutorMoveDescriptions2_9: Table<CArray<*mut u8, 11>> = Table(
    (&raw const crate::data::field_specials::sBattleFrontier_TutorMoveDescriptions2_9).cast(),
);
static sBattleFrontier_TutorMoves1: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::field_specials::sBattleFrontier_TutorMoves1).cast());
static sBattleFrontier_TutorMoves2: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::field_specials::sBattleFrontier_TutorMoves2).cast());
static sBattlePoints_WindowTemplate_20: Table<WindowTemplate> =
    Table((&raw const crate::data::field_specials::sBattlePoints_WindowTemplate_20).cast());
static sBattleTowerStreakThresholds_26: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::field_specials::sBattleTowerStreakThresholds_26).cast());
static sCounterIncrements_2: Table<CArray<u8, 4>> =
    Table((&raw const crate::data::field_specials::sCounterIncrements_2).cast());
static sDeoxysRockCoords: Table<CArray<CArray<u8, 2>, 11>> =
    Table((&raw const crate::data::field_specials::sDeoxysRockCoords).cast());
static sDeoxysRockPalettes: Table<CArray<CArray<u16, 16>, 11>> =
    Table((&raw const crate::data::field_specials::sDeoxysRockPalettes).cast());
static sDeptStoreFloorNames: Table<CArray<*mut u8, 16>> =
    Table((&raw const crate::data::field_specials::sDeptStoreFloorNames).cast());
static sElevatorLightCycles_30: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::field_specials::sElevatorLightCycles_30).cast());
static sElevatorTripLength_31: Table<CArray<u8, 9>> =
    Table((&raw const crate::data::field_specials::sElevatorTripLength_31).cast());
static sElevatorWindowTiles_Ascending: Table<CArray<CArray<u16, 3>, 3>> =
    Table((&raw const crate::data::field_specials::sElevatorWindowTiles_Ascending).cast());
static sElevatorWindowTiles_Descending: Table<CArray<CArray<u16, 3>, 3>> =
    Table((&raw const crate::data::field_specials::sElevatorWindowTiles_Descending).cast());
static sFanClubMemberIds_0: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::field_specials::sFanClubMemberIds_0).cast());
static sFanClubMemberIds_1: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::field_specials::sFanClubMemberIds_1).cast());
static sFrontierChallenges_21: Table<CArray<u16, 12>> =
    Table((&raw const crate::data::field_specials::sFrontierChallenges_21).cast());
static sFrontierExchangeCorner_Decor1Descriptions_18: Table<CArray<*mut u8, 11>> = Table(
    (&raw const crate::data::field_specials::sFrontierExchangeCorner_Decor1Descriptions_18).cast(),
);
static sFrontierExchangeCorner_Decor1_17: Table<CArray<u16, 11>> =
    Table((&raw const crate::data::field_specials::sFrontierExchangeCorner_Decor1_17).cast());
static sFrontierExchangeCorner_Decor2Descriptions_16: Table<CArray<*mut u8, 6>> = Table(
    (&raw const crate::data::field_specials::sFrontierExchangeCorner_Decor2Descriptions_16).cast(),
);
static sFrontierExchangeCorner_Decor2_15: Table<CArray<u16, 6>> =
    Table((&raw const crate::data::field_specials::sFrontierExchangeCorner_Decor2_15).cast());
static sFrontierExchangeCorner_HoldItemsDescriptions_12: Table<CArray<*mut u8, 10>> = Table(
    (&raw const crate::data::field_specials::sFrontierExchangeCorner_HoldItemsDescriptions_12)
        .cast(),
);
static sFrontierExchangeCorner_HoldItems_11: Table<CArray<u16, 10>> =
    Table((&raw const crate::data::field_specials::sFrontierExchangeCorner_HoldItems_11).cast());
static sFrontierExchangeCorner_ItemIconWindowTemplate_19: Table<WindowTemplate> = Table(
    (&raw const crate::data::field_specials::sFrontierExchangeCorner_ItemIconWindowTemplate_19)
        .cast(),
);
static sFrontierExchangeCorner_VitaminsDescriptions_14: Table<CArray<*mut u8, 7>> = Table(
    (&raw const crate::data::field_specials::sFrontierExchangeCorner_VitaminsDescriptions_14)
        .cast(),
);
static sFrontierExchangeCorner_Vitamins_13: Table<CArray<u16, 7>> =
    Table((&raw const crate::data::field_specials::sFrontierExchangeCorner_Vitamins_13).cast());
static sFrontierGamblerGoMessages_22: Table<CArray<*mut u8, 12>> =
    Table((&raw const crate::data::field_specials::sFrontierGamblerGoMessages_22).cast());
static sFrontierGamblerLookingMessages_23: Table<CArray<*mut u8, 12>> =
    Table((&raw const crate::data::field_specials::sFrontierGamblerLookingMessages_23).cast());
static sFrontierManiacMessages_27: Table<CArray<CArray<*mut u8, 3>, 10>> =
    Table((&raw const crate::data::field_specials::sFrontierManiacMessages_27).cast());
static sFrontierManiacStreakThresholds_28: Table<CArray<CArray<u8, 2>, 10>> =
    Table((&raw const crate::data::field_specials::sFrontierManiacStreakThresholds_28).cast());
static sMauvilleGymSwitchCoords: Table<CArray<UCoords8, 4>> =
    Table((&raw const crate::data::field_specials::sMauvilleGymSwitchCoords).cast());
static sNatureGirlMessages_24: Table<CArray<*mut u8, 25>> =
    Table((&raw const crate::data::field_specials::sNatureGirlMessages_24).cast());
static sPetalburgGymSlidingDoorMetatiles: Table<CArray<u16, 5>> =
    Table((&raw const crate::data::field_specials::sPetalburgGymSlidingDoorMetatiles).cast());
static sPokeMarts_4: Table<CArray<CArray<u8, 3>, 12>> =
    Table((&raw const crate::data::field_specials::sPokeMarts_4).cast());
static sPokemonCenters_29: Table<CArray<u16, 19>> =
    Table((&raw const crate::data::field_specials::sPokemonCenters_29).cast());
static sPokemonCenters_3: Table<CArray<u16, 22>> =
    Table((&raw const crate::data::field_specials::sPokemonCenters_3).cast());
static sScrollableMultichoiceOptions: Table<CArray<CArray<*mut u8, 16>, 13>> =
    Table((&raw const crate::data::field_specials::sScrollableMultichoiceOptions).cast());
static sScrollableMultichoice_ScrollArrowsTemplate_25: Table<ScrollArrowsTemplate> = Table(
    (&raw const crate::data::field_specials::sScrollableMultichoice_ScrollArrowsTemplate_25).cast(),
);
static sSlidingDoorNextFrameDelay: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::field_specials::sSlidingDoorNextFrameDelay).cast());
static sSlotMachineIds_32: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::field_specials::sSlotMachineIds_32).cast());
static sSlotMachineRandomSeeds_34: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::field_specials::sSlotMachineRandomSeeds_34).cast());
static sSlotMachineServiceDayIds_33: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::field_specials::sSlotMachineServiceDayIds_33).cast());
static sStoneMaxStepCounts_7: Table<CArray<u8, 10>> =
    Table((&raw const crate::data::field_specials::sStoneMaxStepCounts_7).cast());
static sWindowTemplate_ElevatorFloor: Table<WindowTemplate> =
    Table((&raw const crate::data::field_specials::sWindowTemplate_ElevatorFloor).cast());

#[unsafe(link_section = "ewram_data")]
pub static gBikeCyclingChallenge: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static gBikeCollisions: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sBikeCyclingTimer: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sSlidingDoorNextFrameCounter: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sSlidingDoorFrame: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sTutorMoveAndElevatorWindowId: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sLilycoveDeptStore_NeverRead: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sLilycoveDeptStore_DefaultFloorChoice: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ListMenuItem: *mut ListMenuItem = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sScrollableMultichoice_ScrollOffset: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sFrontierExchangeCorner_NeverRead: crate::global::Global<u16> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sScrollableMultichoice_ItemSpriteId: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sBattlePointsWindowId: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sFrontierExchangeCorner_ItemIconWindowId: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sPCBoxToSendMon: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sBattleTowerMultiBattleTypeFlags: crate::global::Global<u32> =
    crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static mut gScrollableMultichoice_ListMenuTemplate: ListMenuTemplate = unsafe { zeroed() };

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
/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}
/// `SetMainCallback2` with this module's view of its types.
#[inline]
unsafe fn SetMainCallback2(a0: Option<unsafe fn()>) {
    unsafe {
        crate::agb_main::SetMainCallback2(core::mem::transmute(a0));
    }
}

#[unsafe(no_mangle)]
pub unsafe fn Special_ShowDiploma() {
    SetMainCallback2(Some(CB2_ShowDiploma));
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe fn Special_ViewWallClock() {
    gMain.savedCallback = Some(CB2_ReturnToField);
    SetMainCallback2(Some(CB2_ViewWallClock));
    LockPlayerFieldControls();
}
pub fn ResetCyclingRoadChallengeData() {
    gBikeCyclingChallenge.set(FALSE);
    gBikeCollisions.set(0);
    sBikeCyclingTimer.set(0);
}
#[unsafe(no_mangle)]
pub unsafe fn Special_BeginCyclingRoadChallenge() {
    gBikeCyclingChallenge.set(TRUE);
    gBikeCollisions.set(0);
    sBikeCyclingTimer.set(gMain.vblankCounter1);
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerAvatarBike() -> u16 {
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_ACRO_BIKE) != 0 {
        return 1;
    }
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_MACH_BIKE) != 0 {
        return 2;
    }
    0
}
unsafe fn DetermineCyclingRoadResults(numFrames: u32, numBikeCollisions: u8) {
    if numBikeCollisions < 100 {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            numBikeCollisions as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            2,
        );
        StringAppend(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_SpaceTimes).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_99TimesPlus).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    if numFrames < 3600 {
        ConvertIntToDecimalStringN(
            gStringVar2.as_mut_ptr(),
            (numFrames / 60) as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            2,
        );
        gStringVar2[2] = CHAR_DEC_SEPARATOR;
        ConvertIntToDecimalStringN(
            &raw mut (*(&raw const crate::string_util::gStringVar2)
                .cast::<CArray<u8, 256>>()
                .cast_mut())[3],
            (numFrames % 60 * 100 / 60) as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        StringAppend(
            gStringVar2.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_SpaceSeconds).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar2.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_1MinutePlus).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
    let mut result: u8 = 0;
    if numBikeCollisions == 0 {
        result = 5;
    } else if numBikeCollisions < 4 {
        result = 4;
    } else if numBikeCollisions < 10 {
        result = 3;
    } else if numBikeCollisions < 20 {
        result = 2;
    } else if numBikeCollisions < 100 {
        result = 1;
    }
    if numFrames / 60 <= 10 {
        result += 5;
    } else if numFrames / 60 <= 15 {
        result += 4;
    } else if numFrames / 60 <= 20 {
        result += 3;
    } else if numFrames / 60 <= 40 {
        result += 2;
    } else if numFrames / 60 < 60 {
        result += 1;
    }
    gSpecialVar_Result = result as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn FinishCyclingRoadChallenge() {
    let numFrames: u32 = gMain.vblankCounter1 - sBikeCyclingTimer.get();
    DetermineCyclingRoadResults(numFrames, gBikeCollisions.get());
    RecordCyclingRoadResults(numFrames, gBikeCollisions.get());
}
unsafe fn RecordCyclingRoadResults(numFrames: u32, numBikeCollisions: u8) {
    let low: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_L);
    let high: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_H);
    let framesRecord: u32 = low as u32 + ((high as u32) << 16);
    if framesRecord > numFrames || framesRecord == 0 {
        VarSet(VAR_CYCLING_ROAD_RECORD_TIME_L, numFrames as u16);
        VarSet(VAR_CYCLING_ROAD_RECORD_TIME_H, (numFrames >> 16) as u16);
        VarSet(VAR_CYCLING_ROAD_RECORD_COLLISIONS, numBikeCollisions as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetRecordedCyclingRoadResults() -> u16 {
    let low: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_L);
    let high: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_H);
    let framesRecord: u32 = low as u32 + ((high as u32) << 16);
    if framesRecord == 0 {
        return FALSE as u16;
    }
    DetermineCyclingRoadResults(
        framesRecord,
        VarGet(VAR_CYCLING_ROAD_RECORD_COLLISIONS) as u8,
    );
    TRUE as u16
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateCyclingRoadState() {
    if gLastUsedWarp.mapNum == 12 && gLastUsedWarp.mapGroup == 29 {
        return;
    }
    if VarGet(VAR_CYCLING_CHALLENGE_STATE) == 2 || VarGet(VAR_CYCLING_CHALLENGE_STATE) == 3 {
        VarSet(VAR_CYCLING_CHALLENGE_STATE, 0);
        Overworld_SetSavedMusic(MUS_DUMMY);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetSSTidalFlag() {
    FlagSet(FLAG_SYS_CRUISE_MODE);
    *GetVarPointer(VAR_CRUISE_STEP_COUNT) = 0;
}
#[unsafe(no_mangle)]
pub unsafe fn ResetSSTidalFlag() {
    FlagClear(FLAG_SYS_CRUISE_MODE);
}
#[unsafe(no_mangle)]
pub unsafe fn CountSSTidalStep(delta: u16) -> u32 {
    if FlagGet(FLAG_SYS_CRUISE_MODE) == 0
        || ({
            *GetVarPointer(VAR_CRUISE_STEP_COUNT) += delta;
            *GetVarPointer(VAR_CRUISE_STEP_COUNT)
        }) < SS_TIDAL_MAX_STEPS
    {
        return FALSE as u32;
    }
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn GetSSTidalLocation(
    mapGroup: *mut i8,
    mapNum: *mut i8,
    x: *mut i16,
    y: *mut i16,
) -> u8 {
    let varCruiseStepCount: *mut u16 = GetVarPointer(VAR_CRUISE_STEP_COUNT);
    match *GetVarPointer(VAR_SS_TIDAL_STATE) {
        SS_TIDAL_BOARD_SLATEPORT | SS_TIDAL_LAND_SLATEPORT => {
            return SS_TIDAL_LOCATION_SLATEPORT;
        }
        SS_TIDAL_HALFWAY_LILYCOVE | SS_TIDAL_EXIT_CURRENTS_RIGHT => {
            return SS_TIDAL_LOCATION_ROUTE131;
        }
        SS_TIDAL_LAND_LILYCOVE | SS_TIDAL_BOARD_LILYCOVE => {
            return SS_TIDAL_LOCATION_LILYCOVE;
        }
        SS_TIDAL_DEPART_LILYCOVE | SS_TIDAL_EXIT_CURRENTS_LEFT => {
            return SS_TIDAL_LOCATION_ROUTE124;
        }
        SS_TIDAL_DEPART_SLATEPORT => {
            if *varCruiseStepCount < 60 {
                *mapNum = 49;
                *x = *varCruiseStepCount as i16 + 19;
            } else if *varCruiseStepCount < 140 {
                *mapNum = 48;
                *x = *varCruiseStepCount as i16 - 60;
            } else {
                *mapNum = 47;
                *x = *varCruiseStepCount as i16 - 140;
            }
        }
        SS_TIDAL_HALFWAY_SLATEPORT => {
            if *varCruiseStepCount < 66 {
                *mapNum = 47;
                *x = 65 - *varCruiseStepCount as i16;
            } else if *varCruiseStepCount < 146 {
                *mapNum = 48;
                *x = 145 - *varCruiseStepCount as i16;
            } else {
                *mapNum = 49;
                *x = 224 - *varCruiseStepCount as i16;
            }
        }
        _ => {}
    }
    *mapGroup = 0;
    *y = 20;
    SS_TIDAL_LOCATION_CURRENTS
}
pub unsafe fn ShouldDoWallyCall() -> u32 {
    if FlagGet(FLAG_ENABLE_FIRST_WALLY_POKENAV_CALL) != 0 {
        match gMapHeader.mapType {
            MAP_TYPE_TOWN | MAP_TYPE_CITY | MAP_TYPE_ROUTE | MAP_TYPE_OCEAN_ROUTE => {
                if ({
                    let p1 = GetVarPointer(VAR_WALLY_CALL_STEP_COUNTER);
                    *p1 += 1;
                    *p1
                }) < 250
                {
                    return FALSE as u32;
                }
            }
            _ => {
                return FALSE as u32;
            }
        }
    } else {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn ShouldDoScottFortreeCall() -> u32 {
    if FlagGet(FLAG_SCOTT_CALL_FORTREE_GYM) != 0 {
        match gMapHeader.mapType {
            MAP_TYPE_TOWN | MAP_TYPE_CITY | MAP_TYPE_ROUTE | MAP_TYPE_OCEAN_ROUTE => {
                if ({
                    let p1 = GetVarPointer(VAR_SCOTT_FORTREE_CALL_STEP_COUNTER);
                    *p1 += 1;
                    *p1
                }) < 10
                {
                    return FALSE as u32;
                }
            }
            _ => {
                return FALSE as u32;
            }
        }
    } else {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn ShouldDoScottBattleFrontierCall() -> u32 {
    if FlagGet(FLAG_SCOTT_CALL_BATTLE_FRONTIER) != 0 {
        match gMapHeader.mapType {
            MAP_TYPE_TOWN | MAP_TYPE_CITY | MAP_TYPE_ROUTE | MAP_TYPE_OCEAN_ROUTE => {
                if ({
                    let p1 = GetVarPointer(VAR_SCOTT_BF_CALL_STEP_COUNTER);
                    *p1 += 1;
                    *p1
                }) < 10
                {
                    return FALSE as u32;
                }
            }
            _ => {
                return FALSE as u32;
            }
        }
    } else {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn ShouldDoRoxanneCall() -> u32 {
    if FlagGet(FLAG_ENABLE_ROXANNE_FIRST_CALL) != 0 {
        match gMapHeader.mapType {
            MAP_TYPE_TOWN | MAP_TYPE_CITY | MAP_TYPE_ROUTE | MAP_TYPE_OCEAN_ROUTE => {
                if ({
                    let p1 = GetVarPointer(VAR_ROXANNE_CALL_STEP_COUNTER);
                    *p1 += 1;
                    *p1
                }) < 250
                {
                    return FALSE as u32;
                }
            }
            _ => {
                return FALSE as u32;
            }
        }
    } else {
        return FALSE as u32;
    }
    TRUE as u32
}
pub unsafe fn ShouldDoRivalRayquazaCall() -> u32 {
    if FlagGet(FLAG_DEFEATED_MAGMA_SPACE_CENTER) != 0 {
        match gMapHeader.mapType {
            MAP_TYPE_TOWN | MAP_TYPE_CITY | MAP_TYPE_ROUTE | MAP_TYPE_OCEAN_ROUTE => {
                if ({
                    let p1 = GetVarPointer(VAR_RIVAL_RAYQUAZA_CALL_STEP_COUNTER);
                    *p1 += 1;
                    *p1
                }) < 250
                {
                    return FALSE as u32;
                }
            }
            _ => {
                return FALSE as u32;
            }
        }
    } else {
        return FALSE as u32;
    }
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn GetLinkPartnerNames() -> u8 {
    let mut j: u8 = 0;
    let myLinkPlayerNumber: u8 = GetMultiplayerId();
    let nLinkPlayers: u8 = GetLinkPlayerCount();
    for i in 0..nLinkPlayers {
        if myLinkPlayerNumber != i {
            StringCopy(
                (*(&raw const crate::data::tv::gTVStringVarPtrs).cast::<CArray<*mut u8, 3>>())[j],
                gLinkPlayers[i].name.as_mut_ptr(),
            );
            j += 1;
        }
    }
    nLinkPlayers
}
#[unsafe(no_mangle)]
pub unsafe fn SpawnLinkPartnerObjectEvent() {
    let mut j: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let movementTypes: CArray<u8, 4> = CArray([7, 9, 8, 10]);
    let mut coordOffsets: CArray<CArray<i8, 2>, 4> = zeroed();
    coordOffsets[0][0] = 0;
    coordOffsets[0][1] = 1;
    coordOffsets[1][0] = 1;
    coordOffsets[1][1] = 0;
    coordOffsets[2][0] = 0;
    coordOffsets[2][1] = -1;
    coordOffsets[3][0] = -1;
    coordOffsets[3][1] = 0;
    let mut linkSpriteId: u8 = 0;
    let myLinkPlayerNumber: u8 = GetMultiplayerId();
    let playerFacingDirection: u8 = GetPlayerFacingDirection();
    match playerFacingDirection {
        DIR_WEST => {
            j = 2;
            x = (*gSaveBlock1Ptr).pos.x - 1;
            y = (*gSaveBlock1Ptr).pos.y;
        }
        DIR_NORTH => {
            j = 1;
            x = (*gSaveBlock1Ptr).pos.x;
            y = (*gSaveBlock1Ptr).pos.y - 1;
        }
        DIR_EAST => {
            x = (*gSaveBlock1Ptr).pos.x + 1;
            y = (*gSaveBlock1Ptr).pos.y;
        }
        DIR_SOUTH => {
            j = 3;
            x = (*gSaveBlock1Ptr).pos.x;
            y = (*gSaveBlock1Ptr).pos.y + 1;
        }
        _ => {}
    }
    let mut i: u8 = 0;
    while (i as u16) < gSpecialVar_0x8004 {
        if myLinkPlayerNumber != i {
            match gLinkPlayers[i].version as u8 {
                2 | 1 => {
                    if gLinkPlayers[i].gender == 0 {
                        linkSpriteId = OBJ_EVENT_GFX_LINK_RS_BRENDAN;
                    } else {
                        linkSpriteId = OBJ_EVENT_GFX_LINK_RS_MAY;
                    }
                }
                VERSION_EMERALD => {
                    if gLinkPlayers[i].gender == 0 {
                        linkSpriteId = OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL;
                    } else {
                        linkSpriteId = OBJ_EVENT_GFX_RIVAL_MAY_NORMAL;
                    }
                }
                _ => {
                    if gLinkPlayers[i].gender == 0 {
                        linkSpriteId = OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL;
                    } else {
                        linkSpriteId = OBJ_EVENT_GFX_RIVAL_MAY_NORMAL;
                    }
                }
            }
            SpawnSpecialObjectEventParameterized(
                linkSpriteId,
                movementTypes[j],
                LOCALID_BERRY_BLENDER_PLAYER_END - i,
                coordOffsets[j][0] as i16 + x + MAP_OFFSET as i16,
                coordOffsets[j][1] as i16 + y + MAP_OFFSET as i16,
                0,
            );
            LoadLinkPartnerObjectEventSpritePalette(
                linkSpriteId,
                LOCALID_BERRY_BLENDER_PLAYER_END - i,
                i,
            );
            j += 1;
            if j == MAX_LINK_PLAYERS as u8 {
                j = 0;
            }
        }
        i += 1;
    }
}
unsafe fn LoadLinkPartnerObjectEventSpritePalette(
    graphicsId: u8,
    localEventId: u8,
    paletteNum: u8,
) {
    let adjustedPaletteNum: u8 = paletteNum + 6;
    if graphicsId == OBJ_EVENT_GFX_LINK_RS_BRENDAN
        || graphicsId == OBJ_EVENT_GFX_LINK_RS_MAY
        || graphicsId == OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL
        || graphicsId == OBJ_EVENT_GFX_RIVAL_MAY_NORMAL
    {
        let obj: u8 = GetObjectEventIdByLocalIdAndMap(
            localEventId,
            (*gSaveBlock1Ptr).location.mapNum as u8,
            (*gSaveBlock1Ptr).location.mapGroup as u8,
        );
        if obj != OBJECT_EVENTS_COUNT {
            let spriteId: u8 = gObjectEvents[obj].spriteId;
            let sprite: *mut Sprite = &raw mut gSprites[spriteId];
            (*sprite).oam.set_paletteNum(adjustedPaletteNum as u16);
            match graphicsId {
                OBJ_EVENT_GFX_LINK_RS_BRENDAN => {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_RubySapphireBrendan).cast::<CArray<u16, 0>>()).as_ptr().cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                OBJ_EVENT_GFX_LINK_RS_MAY => {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_RubySapphireMay).cast::<CArray<u16, 0>>()).as_ptr().cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL => {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_Brendan)
                            .cast::<CArray<u16, 0>>())
                        .as_ptr()
                        .cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                OBJ_EVENT_GFX_RIVAL_MAY_NORMAL => {
                    LoadPalette(
                        (*(&raw const crate::data::event_object_movement::gObjectEventPal_May)
                            .cast::<CArray<u16, 0>>())
                        .as_ptr()
                        .cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                _ => {}
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn MauvilleGymPressSwitch() {
    for i in 0..4u8 {
        if i as u16 == gSpecialVar_0x8004 {
            MapGridSetMetatileIdAt(
                sMauvilleGymSwitchCoords[i].x as i32,
                sMauvilleGymSwitchCoords[i].y as i32,
                METATILE_MauvilleGym_PressedSwitch,
            );
        } else {
            MapGridSetMetatileIdAt(
                sMauvilleGymSwitchCoords[i].x as i32,
                sMauvilleGymSwitchCoords[i].y as i32,
                METATILE_MauvilleGym_RaisedSwitch,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn MauvilleGymSetDefaultBarriers() {
    for y in 12..24i32 {
        for x in MAP_OFFSET..16 {
            match MapGridGetMetatileIdAt(x, y) {
                METATILE_MauvilleGym_GreenBeamH1_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH1_Off);
                }
                METATILE_MauvilleGym_GreenBeamH2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH2_Off);
                }
                METATILE_MauvilleGym_GreenBeamH3_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH3_Off);
                }
                METATILE_MauvilleGym_GreenBeamH4_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH4_Off);
                }
                560 => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH1_On as u16);
                }
                561 => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH2_On as u16);
                }
                568 => {
                    MapGridSetMetatileIdAt(x, y, 3624);
                }
                569 => {
                    MapGridSetMetatileIdAt(x, y, 3625);
                }
                METATILE_MauvilleGym_RedBeamH1_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH1_Off);
                }
                METATILE_MauvilleGym_RedBeamH2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH2_Off);
                }
                METATILE_MauvilleGym_RedBeamH3_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH3_Off);
                }
                METATILE_MauvilleGym_RedBeamH4_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH4_Off);
                }
                562 => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH1_On as u16);
                }
                563 => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH2_On as u16);
                }
                570 => {
                    MapGridSetMetatileIdAt(x, y, 3626);
                }
                571 => {
                    MapGridSetMetatileIdAt(x, y, 3627);
                }
                METATILE_MauvilleGym_GreenBeamV1_On => {
                    MapGridSetMetatileIdAt(x, y, 3650);
                }
                METATILE_MauvilleGym_GreenBeamV2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_FloorTile);
                }
                METATILE_MauvilleGym_RedBeamV1_On => {
                    MapGridSetMetatileIdAt(x, y, 3651);
                }
                METATILE_MauvilleGym_RedBeamV2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_FloorTile);
                }
                METATILE_MauvilleGym_PoleBottom_On => {
                    MapGridSetMetatileIdAt(x, y, 3648);
                }
                538 => {
                    if MapGridGetMetatileIdAt(x, y - 1) == METATILE_MauvilleGym_GreenBeamV1_On {
                        MapGridSetMetatileIdAt(x, y, 3656);
                    } else {
                        MapGridSetMetatileIdAt(x, y, 3657);
                    }
                }
                METATILE_MauvilleGym_PoleBottom_Off => {
                    MapGridSetMetatileIdAt(x, y, 3649);
                }
                593 => {
                    MapGridSetMetatileIdAt(x, y, 3664);
                }
                METATILE_MauvilleGym_PoleTop_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_PoleTop_Off);
                }
                _ => {}
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn MauvilleGymDeactivatePuzzle() {
    let mut switchCoords: *mut UCoords8 = sMauvilleGymSwitchCoords.as_ptr().cast_mut();
    let mut i: i32 = 3;
    while i >= 0 {
        MapGridSetMetatileIdAt(
            (*switchCoords).x as i32,
            (*switchCoords).y as i32,
            METATILE_MauvilleGym_PressedSwitch,
        );
        switchCoords = switchCoords.at(1);
        i -= 1;
    }
    for y in 12..24i32 {
        for x in MAP_OFFSET..16 {
            match MapGridGetMetatileIdAt(x, y) {
                METATILE_MauvilleGym_GreenBeamH1_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH1_Off);
                }
                METATILE_MauvilleGym_GreenBeamH2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH2_Off);
                }
                METATILE_MauvilleGym_GreenBeamH3_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH3_Off);
                }
                METATILE_MauvilleGym_GreenBeamH4_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_GreenBeamH4_Off);
                }
                METATILE_MauvilleGym_RedBeamH1_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH1_Off);
                }
                METATILE_MauvilleGym_RedBeamH2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH2_Off);
                }
                METATILE_MauvilleGym_RedBeamH3_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH3_Off);
                }
                METATILE_MauvilleGym_RedBeamH4_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_RedBeamH4_Off);
                }
                METATILE_MauvilleGym_GreenBeamV1_On => {
                    MapGridSetMetatileIdAt(x, y, 3650);
                }
                METATILE_MauvilleGym_RedBeamV1_On => {
                    MapGridSetMetatileIdAt(x, y, 3651);
                }
                METATILE_MauvilleGym_GreenBeamV2_On | METATILE_MauvilleGym_RedBeamV2_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_FloorTile);
                }
                METATILE_MauvilleGym_PoleTop_On => {
                    MapGridSetMetatileIdAt(x, y, METATILE_MauvilleGym_PoleTop_Off);
                }
                _ => {}
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PetalburgGymSlideOpenRoomDoors() {
    sSlidingDoorNextFrameCounter.set(0);
    sSlidingDoorFrame.set(0);
    PlaySE(SE_UNLOCK);
    CreateTask(Some(Task_PetalburgGymSlideOpenRoomDoors), 8);
}
pub(crate) unsafe fn Task_PetalburgGymSlideOpenRoomDoors(taskId: u8) {
    if sSlidingDoorNextFrameDelay[sSlidingDoorFrame.get()] == sSlidingDoorNextFrameCounter.get() {
        PetalburgGymSetDoorMetatiles(
            gSpecialVar_0x8004 as u8,
            sPetalburgGymSlidingDoorMetatiles[sSlidingDoorFrame.get()],
        );
        sSlidingDoorNextFrameCounter.set(0);
        if ({
            sSlidingDoorFrame.set(sSlidingDoorFrame.get() + 1);
            sSlidingDoorFrame.get()
        }) == 5
        {
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
    } else {
        sSlidingDoorNextFrameCounter.set(sSlidingDoorNextFrameCounter.get() + 1);
    }
}
unsafe fn PetalburgGymSetDoorMetatiles(roomNumber: u8, metatileId: u16) {
    let mut doorCoordsX: CArray<u16, 4> = zeroed();
    let mut doorCoordsY: CArray<u16, 4> = zeroed();
    let mut nDoors: u8 = 0;
    match roomNumber {
        1 => {
            nDoors = 2;
            doorCoordsX[0] = 1;
            doorCoordsX[1] = 7;
            doorCoordsY[0] = 104;
            doorCoordsY[1] = 104;
        }
        2 => {
            nDoors = 2;
            doorCoordsX[0] = 1;
            doorCoordsX[1] = 7;
            doorCoordsY[0] = 78;
            doorCoordsY[1] = 78;
        }
        3 => {
            nDoors = 2;
            doorCoordsX[0] = 1;
            doorCoordsX[1] = 7;
            doorCoordsY[0] = 91;
            doorCoordsY[1] = 91;
        }
        4 => {
            nDoors = 1;
            doorCoordsX[0] = 7;
            doorCoordsY[0] = 39;
        }
        5 => {
            nDoors = 2;
            doorCoordsX[0] = 1;
            doorCoordsX[1] = 7;
            doorCoordsY[0] = 52;
            doorCoordsY[1] = 52;
        }
        6 => {
            nDoors = 1;
            doorCoordsX[0] = 1;
            doorCoordsY[0] = 65;
        }
        7 => {
            nDoors = 1;
            doorCoordsX[0] = 7;
            doorCoordsY[0] = 13;
        }
        8 => {
            nDoors = 1;
            doorCoordsX[0] = 1;
            doorCoordsY[0] = 26;
        }
        _ => {}
    }
    for i in 0..nDoors {
        MapGridSetMetatileIdAt(
            doorCoordsX[i] as i32 + MAP_OFFSET,
            doorCoordsY[i] as i32 + MAP_OFFSET,
            metatileId | MAPGRID_IMPASSABLE,
        );
        MapGridSetMetatileIdAt(
            doorCoordsX[i] as i32 + MAP_OFFSET,
            doorCoordsY[i] as i32 + MAP_OFFSET + 1,
            (metatileId + METATILE_ROW_WIDTH) | MAPGRID_IMPASSABLE,
        );
    }
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe fn PetalburgGymUnlockRoomDoors() {
    PetalburgGymSetDoorMetatiles(
        gSpecialVar_0x8004 as u8,
        sPetalburgGymSlidingDoorMetatiles[4],
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ShowFieldMessageStringVar4() {
    ShowFieldMessage(gStringVar4.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe fn StorePlayerCoordsInVars() {
    gSpecialVar_0x8004 = (*gSaveBlock1Ptr).pos.x as u16;
    gSpecialVar_0x8005 = (*gSaveBlock1Ptr).pos.y as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerTrainerIdOnesDigit() -> u8 {
    ((((*gSaveBlock2Ptr).playerTrainerId[1] as u16 as i32) << 8
        | (*gSaveBlock2Ptr).playerTrainerId[0] as u16 as i32)
        % 10) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerBigGuyGirlString() {
    if (*gSaveBlock2Ptr).playerGender == MALE {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_BigGuy).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_BigGirl).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetRivalSonDaughterString() {
    if (*gSaveBlock2Ptr).playerGender == MALE {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Daughter).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::strings::gText_Son).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattleOutcome() -> u8 {
    gBattleOutcome
}
#[unsafe(no_mangle)]
pub unsafe fn CableCarWarp() {
    if gSpecialVar_0x8004 != 0 {
        SetWarpDestination(19, 0, WARP_ID_NONE, 6, 4);
    } else {
        SetWarpDestination(19, 1, WARP_ID_NONE, 6, 4);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetHiddenItemFlag() {
    FlagSet(
        *(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut(),
    );
}
#[unsafe(no_mangle)]
pub unsafe fn GetWeekCount() -> u16 {
    let mut weekCount: u16 = (gLocalTime.days / 7) as u16;
    if weekCount > 9999 {
        weekCount = 9999;
    }
    weekCount
}
#[unsafe(no_mangle)]
pub unsafe fn GetLeadMonFriendshipScore() -> u8 {
    let pokemon: *mut Pokemon = &raw mut gPlayerParty[GetLeadMonIndex()];
    if GetMonData2(pokemon, MON_DATA_FRIENDSHIP) == MAX_FRIENDSHIP as u32 {
        return FRIENDSHIP_MAX;
    }
    if GetMonData2(pokemon, MON_DATA_FRIENDSHIP) >= 200 {
        return FRIENDSHIP_200_TO_254;
    }
    if GetMonData2(pokemon, MON_DATA_FRIENDSHIP) >= 150 {
        return FRIENDSHIP_150_TO_199;
    }
    if GetMonData2(pokemon, MON_DATA_FRIENDSHIP) >= 100 {
        return FRIENDSHIP_100_TO_149;
    }
    if GetMonData2(pokemon, MON_DATA_FRIENDSHIP) >= 50 {
        return FRIENDSHIP_50_TO_99;
    }
    if GetMonData2(pokemon, MON_DATA_FRIENDSHIP) >= 1 {
        return FRIENDSHIP_1_TO_49;
    }
    FRIENDSHIP_NONE
}
pub(crate) unsafe fn CB2_FieldShowRegionMap() {
    FieldInitRegionMap(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
#[unsafe(no_mangle)]
pub unsafe fn FieldShowRegionMap() {
    SetMainCallback2(Some(CB2_FieldShowRegionMap));
}
#[unsafe(no_mangle)]
pub unsafe fn DoPCTurnOnEffect() {
    if FuncIsActiveTask(Some(Task_PCTurnOnEffect)) != TRUE {
        let taskId: u8 = CreateTask(Some(Task_PCTurnOnEffect), 8);
        task_set(taskId, tPaused, FALSE as i16);
        task_set(taskId, 1, taskId as i16);
        task_set(taskId, tFlickerCount, 0);
        task_set(taskId, 3, 0);
        task_set(taskId, tIsScreenOn, FALSE as i16);
    }
}
pub(crate) unsafe fn Task_PCTurnOnEffect(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[tPaused] == 0 {
        PCTurnOnEffect(task);
    }
}
unsafe fn PCTurnOnEffect(task: *mut Task) {
    let mut playerDirection: u8 = 0;
    let mut dx: i8 = 0;
    let mut dy: i8 = 0;
    if (*task).data[3] == 6 {
        (*task).data[3] = 0;
        playerDirection = GetPlayerFacingDirection();
        match playerDirection {
            DIR_NORTH => {
                dx = 0;
                dy = -1;
            }
            DIR_WEST => {
                dx = -1;
                dy = -1;
            }
            DIR_EAST => {
                dx = 1;
                dy = -1;
            }
            _ => {}
        }
        PCTurnOnEffect_SetMetatile((*task).data[tIsScreenOn], dx, dy);
        DrawWholeMapView();
        (*task).data[tIsScreenOn] ^= 1;
        if ({
            (*task).data[tFlickerCount] += 1;
            (*task).data[tFlickerCount]
        }) == 5
        {
            DestroyTask((*task).data[1] as u8);
        }
    }
    (*task).data[3] += 1;
}
unsafe fn PCTurnOnEffect_SetMetatile(isScreenOn: i16, dx: i8, dy: i8) {
    let mut metatileId: u16 = 0;
    if isScreenOn != 0 {
        if gSpecialVar_0x8004 == PC_LOCATION_OTHER {
            metatileId = METATILE_Building_PC_Off;
        } else if gSpecialVar_0x8004 == PC_LOCATION_BRENDANS_HOUSE {
            metatileId = METATILE_BrendansMaysHouse_BrendanPC_Off;
        } else if gSpecialVar_0x8004 == PC_LOCATION_MAYS_HOUSE {
            metatileId = METATILE_BrendansMaysHouse_MayPC_Off;
        }
    } else {
        if gSpecialVar_0x8004 == PC_LOCATION_OTHER {
            metatileId = METATILE_Building_PC_On;
        } else if gSpecialVar_0x8004 == PC_LOCATION_BRENDANS_HOUSE {
            metatileId = METATILE_BrendansMaysHouse_BrendanPC_On;
        } else if gSpecialVar_0x8004 == PC_LOCATION_MAYS_HOUSE {
            metatileId = METATILE_BrendansMaysHouse_MayPC_On;
        }
    }
    MapGridSetMetatileIdAt(
        (*gSaveBlock1Ptr).pos.x as i32 + dx as i32 + MAP_OFFSET,
        (*gSaveBlock1Ptr).pos.y as i32 + dy as i32 + MAP_OFFSET,
        metatileId | MAPGRID_IMPASSABLE,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn DoPCTurnOffEffect() {
    PCTurnOffEffect();
}
unsafe fn PCTurnOffEffect() {
    let mut dx: i8 = 0;
    let mut dy: i8 = 0;
    let mut metatileId: u16 = 0;
    let playerDirection: u8 = GetPlayerFacingDirection();
    match playerDirection {
        DIR_NORTH => {
            dx = 0;
            dy = -1;
        }
        DIR_WEST => {
            dx = -1;
            dy = -1;
        }
        DIR_EAST => {
            dx = 1;
            dy = -1;
        }
        _ => {}
    }
    if gSpecialVar_0x8004 == PC_LOCATION_OTHER {
        metatileId = METATILE_Building_PC_Off;
    } else if gSpecialVar_0x8004 == PC_LOCATION_BRENDANS_HOUSE {
        metatileId = METATILE_BrendansMaysHouse_BrendanPC_Off;
    } else if gSpecialVar_0x8004 == PC_LOCATION_MAYS_HOUSE {
        metatileId = METATILE_BrendansMaysHouse_MayPC_Off;
    }
    MapGridSetMetatileIdAt(
        (*gSaveBlock1Ptr).pos.x as i32 + dx as i32 + MAP_OFFSET,
        (*gSaveBlock1Ptr).pos.y as i32 + dy as i32 + MAP_OFFSET,
        metatileId | MAPGRID_IMPASSABLE,
    );
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe fn DoLotteryCornerComputerEffect() {
    if FuncIsActiveTask(Some(Task_LotteryCornerComputerEffect)) != TRUE {
        let taskId: u8 = CreateTask(Some(Task_LotteryCornerComputerEffect), 8);
        task_set(taskId, tPaused, FALSE as i16);
        task_set(taskId, 1, taskId as i16);
        task_set(taskId, tFlickerCount, 0);
        task_set(taskId, 3, 0);
        task_set(taskId, tIsScreenOn, FALSE as i16);
    }
}
pub(crate) unsafe fn Task_LotteryCornerComputerEffect(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[tPaused] == 0 {
        LotteryCornerComputerEffect(task);
    }
}
unsafe fn LotteryCornerComputerEffect(task: *mut Task) {
    if (*task).data[3] == 6 {
        (*task).data[3] = 0;
        if (*task).data[tIsScreenOn] != 0 {
            MapGridSetMetatileIdAt(18, 8, 3741);
            MapGridSetMetatileIdAt(18, 9, 3749);
        } else {
            MapGridSetMetatileIdAt(18, 8, 3672);
            MapGridSetMetatileIdAt(18, 9, 3680);
        }
        DrawWholeMapView();
        (*task).data[tIsScreenOn] ^= 1;
        if ({
            (*task).data[tFlickerCount] += 1;
            (*task).data[tFlickerCount]
        }) == 5
        {
            DestroyTask((*task).data[1] as u8);
        }
    }
    (*task).data[3] += 1;
}
#[unsafe(no_mangle)]
pub unsafe fn EndLotteryCornerComputerEffect() {
    MapGridSetMetatileIdAt(18, 8, 3741);
    MapGridSetMetatileIdAt(18, 9, 3749);
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe fn SetTrickHouseNuggetFlag() {
    let specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let flag: u16 = FLAG_HIDDEN_ITEM_TRICK_HOUSE_NUGGET;
    *specVar = flag;
    FlagSet(flag);
}
#[unsafe(no_mangle)]
pub unsafe fn ResetTrickHouseNuggetFlag() {
    let specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let flag: u16 = FLAG_HIDDEN_ITEM_TRICK_HOUSE_NUGGET;
    *specVar = flag;
    FlagClear(flag);
}
#[unsafe(no_mangle)]
pub unsafe fn CheckLeadMonCool() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_COOL) < 200 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn CheckLeadMonBeauty() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_BEAUTY) < 200 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn CheckLeadMonCute() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_CUTE) < 200 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn CheckLeadMonSmart() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_SMART) < 200 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn CheckLeadMonTough() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_TOUGH) < 200 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn IsGrassTypeInParty() {
    let mut species: u16 = 0;
    let mut pokemon: *mut Pokemon = null_mut();
    for i in 0..(PARTY_SIZE as u8) {
        pokemon = &raw mut gPlayerParty[i];
        if GetMonData2(pokemon, MON_DATA_SANITY_HAS_SPECIES) != 0
            && GetMonData2(pokemon, MON_DATA_IS_EGG) == 0
        {
            species = GetMonData2(pokemon, MON_DATA_SPECIES) as u16;
            if (*(&raw const crate::data::pokemon::gSpeciesInfo).cast::<CArray<SpeciesInfo, 0>>())
                [species]
                .types[0]
                == TYPE_GRASS
                || (*(&raw const crate::data::pokemon::gSpeciesInfo)
                    .cast::<CArray<SpeciesInfo, 0>>())[species]
                    .types[1]
                    == TYPE_GRASS
            {
                gSpecialVar_Result = TRUE as u16;
                return;
            }
        }
    }
    gSpecialVar_Result = FALSE as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn SpawnCameraObject() {
    let obj: u8 = SpawnSpecialObjectEventParameterized(
        OBJ_EVENT_GFX_BOY_1 as u8,
        MOVEMENT_TYPE_FACE_DOWN,
        LOCALID_CAMERA,
        (*gSaveBlock1Ptr).pos.x + MAP_OFFSET as i16,
        (*gSaveBlock1Ptr).pos.y + MAP_OFFSET as i16,
        ELEVATION_DEFAULT,
    );
    gObjectEvents[obj].set_invisible(TRUE as u32);
    CameraObjectSetFollowedSpriteId(gObjectEvents[obj].spriteId);
}
#[unsafe(no_mangle)]
pub unsafe fn RemoveCameraObject() {
    CameraObjectSetFollowedSpriteId(GetPlayerAvatarSpriteId());
    RemoveObjectEventByLocalIdAndMap(
        LOCALID_CAMERA,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn GetPokeblockNameByMonNature() -> u8 {
    CopyMonFavoritePokeblockName(
        GetNature(&raw mut gPlayerParty[GetLeadMonIndex()]),
        gStringVar1.as_mut_ptr(),
    )
}
#[unsafe(no_mangle)]
pub unsafe fn GetSecretBaseNearbyMapName() {
    GetMapName(gStringVar1.as_mut_ptr(), VarGet(VAR_SECRET_BASE_MAP), 0);
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattleTowerSinglesStreak() -> u16 {
    GetGameStat(GAME_STAT_BATTLE_TOWER_SINGLES_STREAK) as u16
}
#[unsafe(no_mangle)]
pub unsafe fn BufferEReaderTrainerName() {
    GetEreaderTrainerName(gStringVar1.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe fn GetSlotMachineId() -> u16 {
    let rnd: u32 = (*gSaveBlock1Ptr).dewfordTrends[0].trendiness() as u32
        + (*gSaveBlock1Ptr).dewfordTrends[0].rand as u32
        + sSlotMachineRandomSeeds_34[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()] as u32;
    if IsPokeNewsActive(POKENEWS_GAME_CORNER) != 0 {
        return sSlotMachineServiceDayIds_33[rnd % 12] as u16;
    }
    sSlotMachineIds_32[rnd % 12] as u16
}
#[unsafe(no_mangle)]
pub unsafe fn FoundAbandonedShipRoom1Key() -> u8 {
    let specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_1_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn FoundAbandonedShipRoom2Key() -> u8 {
    let specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_2_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn FoundAbandonedShipRoom4Key() -> u8 {
    let specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_4_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn FoundAbandonedShipRoom6Key() -> u8 {
    let specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_6_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn LeadMonHasEffortRibbon() -> u8 {
    GetMonData3(
        &raw mut gPlayerParty[GetLeadMonIndex()],
        MON_DATA_EFFORT_RIBBON,
        null_mut(),
    ) as u8
}
#[unsafe(no_mangle)]
pub unsafe fn GiveLeadMonEffortRibbon() {
    IncrementGameStat(GAME_STAT_RECEIVED_RIBBONS);
    FlagSet(FLAG_SYS_RIBBON_GET);
    let mut ribbonSet: u8 = TRUE;
    let leadMon: *mut Pokemon = &raw mut gPlayerParty[GetLeadMonIndex()];
    SetMonData(
        leadMon,
        MON_DATA_EFFORT_RIBBON,
        &raw mut ribbonSet as *mut c_void,
    );
    if GetRibbonCount(leadMon) > NUM_CUTIES_RIBBONS {
        TryPutSpotTheCutiesOnAir(leadMon, MON_DATA_EFFORT_RIBBON as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Special_AreLeadMonEVsMaxedOut() -> u8 {
    if GetMonEVCount(&raw mut gPlayerParty[GetLeadMonIndex()]) >= MAX_TOTAL_EVS as u16 {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn TryUpdateRusturfTunnelState() -> u8 {
    if FlagGet(FLAG_RUSTURF_TUNNEL_OPENED) == 0
        && (*gSaveBlock1Ptr).location.mapGroup == 24
        && (*gSaveBlock1Ptr).location.mapNum == 4
    {
        if FlagGet(FLAG_HIDE_RUSTURF_TUNNEL_ROCK_1) != 0 {
            VarSet(VAR_RUSTURF_TUNNEL_STATE, 4);
            return TRUE;
        } else if FlagGet(FLAG_HIDE_RUSTURF_TUNNEL_ROCK_2) != 0 {
            VarSet(VAR_RUSTURF_TUNNEL_STATE, 5);
            return TRUE;
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn SetShoalItemFlag(unused: u16) {
    FlagSet(FLAG_SYS_SHOAL_ITEM);
}
#[unsafe(no_mangle)]
pub unsafe fn LoadWallyZigzagoon() {
    CreateMon(
        &raw mut gPlayerParty[0],
        SPECIES_ZIGZAGOON,
        7,
        USE_RANDOM_IVS,
        0,
        0,
        0,
        0,
    );
    let mut monData: u16 = TRUE as u16;
    SetMonData(
        &raw mut gPlayerParty[0],
        MON_DATA_ABILITY_NUM,
        &raw mut monData as *mut c_void,
    );
    monData = MOVE_TACKLE;
    SetMonData(
        &raw mut gPlayerParty[0],
        MON_DATA_MOVE1,
        &raw mut monData as *mut c_void,
    );
    monData = MOVE_NONE;
    SetMonData(
        &raw mut gPlayerParty[0],
        MON_DATA_MOVE2,
        &raw mut monData as *mut c_void,
    );
    SetMonData(
        &raw mut gPlayerParty[0],
        MON_DATA_MOVE3,
        &raw mut monData as *mut c_void,
    );
    SetMonData(
        &raw mut gPlayerParty[0],
        MON_DATA_MOVE4,
        &raw mut monData as *mut c_void,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn IsStarterInParty() -> u8 {
    let starter: u16 = GetStarterPokemon(VarGet(VAR_STARTER_MON));
    let partyCount: u8 = CalculatePlayerPartyCount();
    for i in 0..partyCount {
        if GetMonData3(
            &raw mut gPlayerParty[i],
            MON_DATA_SPECIES_OR_EGG,
            null_mut(),
        ) == starter as u32
        {
            return TRUE;
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ScriptCheckFreePokemonStorageSpace() -> u8 {
    CheckFreePokemonStorageSpace()
}
#[unsafe(no_mangle)]
pub unsafe fn IsPokerusInParty() -> u8 {
    if CheckPartyPokerus(gPlayerParty.as_mut_ptr(), 63) == 0 {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn ShakeCamera() {
    let taskId: u8 = CreateTask(Some(Task_ShakeCamera), 9);
    task_set(taskId, tHorizontalPan, gSpecialVar_0x8005 as i16);
    task_set(taskId, tDelayCounter, 0);
    task_set(taskId, tNumShakes, gSpecialVar_0x8006 as i16);
    task_set(taskId, tDelay, gSpecialVar_0x8007 as i16);
    task_set(taskId, tVerticalPan, gSpecialVar_0x8004 as i16);
    SetCameraPanningCallback(None);
    PlaySE(SE_M_STRENGTH);
}
pub(crate) unsafe fn Task_ShakeCamera(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) += 1;
    if rem_i32(*data.at(1) as i32, *data.at(3) as i32) == 0 {
        *data.at(1) = 0;
        *data.at(2) -= 1;
        *data = -*data;
        *data.at(4) = -*data.at(4);
        SetCameraPanning(*data, *data.at(4));
        if *data.at(2) == 0 {
            StopCameraShake(taskId);
            InstallCameraPanAheadCallback();
        }
    }
}
unsafe fn StopCameraShake(taskId: u8) {
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe fn FoundBlackGlasses() -> u8 {
    FlagGet(FLAG_HIDDEN_ITEM_ROUTE_116_BLACK_GLASSES)
}
#[unsafe(no_mangle)]
pub unsafe fn SetRoute119Weather() {
    if IsMapTypeOutdoors(GetLastUsedWarpMapType()) != TRUE {
        SetSavedWeather(WEATHER_ROUTE119_CYCLE);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetRoute123Weather() {
    if IsMapTypeOutdoors(GetLastUsedWarpMapType()) != TRUE {
        SetSavedWeather(WEATHER_ROUTE123_CYCLE);
    }
}
pub unsafe fn GetLeadMonIndex() -> u8 {
    let partyCount: u8 = CalculatePlayerPartyCount();
    for i in 0..partyCount {
        if GetMonData3(
            &raw mut gPlayerParty[i],
            MON_DATA_SPECIES_OR_EGG,
            null_mut(),
        ) != SPECIES_EGG
            && GetMonData3(
                &raw mut gPlayerParty[i],
                MON_DATA_SPECIES_OR_EGG,
                null_mut(),
            ) != SPECIES_NONE as u32
        {
            return i;
        }
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn ScriptGetPartyMonSpecies() -> u16 {
    GetMonData3(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_SPECIES_OR_EGG,
        null_mut(),
    ) as u16
}
#[unsafe(no_mangle)]
pub fn TryInitBattleTowerAwardManObjectEvent() {}
#[unsafe(no_mangle)]
pub unsafe fn GetDaysUntilPacifidlogTMAvailable() -> u16 {
    let tmReceivedDay: u16 = VarGet(VAR_PACIFIDLOG_TM_RECEIVED_DAY);
    if gLocalTime.days as i32 - tmReceivedDay as i32 >= 7 {
        return 0;
    } else if gLocalTime.days < 0 {
        return 8;
    }
    7 - (gLocalTime.days as u16 - tmReceivedDay)
}
#[unsafe(no_mangle)]
pub unsafe fn SetPacifidlogTMReceivedDay() -> u16 {
    VarSet(VAR_PACIFIDLOG_TM_RECEIVED_DAY, gLocalTime.days as u16);
    gLocalTime.days as u16
}
#[unsafe(no_mangle)]
pub unsafe fn MonOTNameNotPlayer() -> u8 {
    if GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_LANGUAGE,
    ) != GAME_LANGUAGE as u32
    {
        return TRUE;
    }
    GetMonData3(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_OT_NAME,
        gStringVar1.as_mut_ptr(),
    );
    if StringCompare(
        (*gSaveBlock2Ptr).playerName.as_mut_ptr(),
        gStringVar1.as_mut_ptr(),
    ) == 0
    {
        return FALSE;
    }
    TRUE
}
#[unsafe(no_mangle)]
pub unsafe fn BufferLottoTicketNumber() {
    if gSpecialVar_Result >= 10000 {
        ConvertIntToDecimalString(0, gSpecialVar_Result as i32);
    } else if gSpecialVar_Result >= 1000 {
        gStringVar1[0] = CHAR_0;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr().at(1),
            gSpecialVar_Result as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            CountDigits(gSpecialVar_Result as i32) as u8,
        );
    } else if gSpecialVar_Result >= 100 {
        gStringVar1[0] = CHAR_0;
        gStringVar1[1] = CHAR_0;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr().at(2),
            gSpecialVar_Result as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            CountDigits(gSpecialVar_Result as i32) as u8,
        );
    } else if gSpecialVar_Result >= 10 {
        gStringVar1[0] = CHAR_0;
        gStringVar1[1] = CHAR_0;
        gStringVar1[2] = CHAR_0;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr().at(3),
            gSpecialVar_Result as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            CountDigits(gSpecialVar_Result as i32) as u8,
        );
    } else {
        gStringVar1[0] = CHAR_0;
        gStringVar1[1] = CHAR_0;
        gStringVar1[2] = CHAR_0;
        gStringVar1[3] = CHAR_0;
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr().at(4),
            gSpecialVar_Result as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            CountDigits(gSpecialVar_Result as i32) as u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetMysteryGiftCardStat() -> u16 {
    match *(&raw const crate::ffi::gSpecialVar_Result)
        .cast::<u16>()
        .cast_mut()
    {
        GET_NUM_STAMPS => {
            return MysteryGift_GetCardStat(CARD_STAT_NUM_STAMPS);
        }
        GET_MAX_STAMPS => {
            return MysteryGift_GetCardStat(CARD_STAT_MAX_STAMPS);
        }
        GET_CARD_BATTLES_WON => {
            return MysteryGift_GetCardStat(CARD_STAT_BATTLES_WON);
        }
        GET_CARD_BATTLES_LOST => {
            return MysteryGift_GetCardStat(CARD_STAT_BATTLES_LOST);
        }
        GET_CARD_NUM_TRADES => {
            return MysteryGift_GetCardStat(CARD_STAT_NUM_TRADES);
        }
        _ => {
            return 0;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BufferTMHMMoveName() -> u8 {
    if gSpecialVar_0x8004 >= ITEM_TM01 && gSpecialVar_0x8004 <= ITEM_HM08 {
        StringCopy(
            gStringVar2.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[ItemIdToBattleMoveId(
                *(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut(),
            )]
            .as_ptr()
            .cast_mut(),
        );
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn IsBadEggInParty() -> u8 {
    let partyCount: u8 = CalculatePlayerPartyCount();
    for i in 0..partyCount {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_BAD_EGG) == TRUE as u32 {
            return TRUE;
        }
    }
    FALSE
}
pub unsafe fn InMultiPartnerRoom() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup == 26
        && (*gSaveBlock1Ptr).location.mapNum == 15
        && VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_MULTIS
    {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn OffsetCameraForBattle() {
    SetCameraPanningCallback(None);
    SetCameraPanning(8, 0);
}
#[unsafe(no_mangle)]
pub unsafe fn SetDeptStoreFloor() {
    let mut deptStoreFloor: u8 = 0;
    match (*gSaveBlock1Ptr).dynamicWarp.mapNum {
        16 => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_1F;
        }
        17 => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_2F;
        }
        18 => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_3F;
        }
        19 => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_4F;
        }
        20 => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_5F;
        }
        21 => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_ROOFTOP;
        }
        _ => {
            deptStoreFloor = DEPT_STORE_FLOORNUM_1F;
        }
    }
    VarSet(VAR_DEPT_STORE_FLOOR, deptStoreFloor as u16);
}
#[unsafe(no_mangle)]
pub unsafe fn GetDeptStoreDefaultFloorChoice() -> u16 {
    sLilycoveDeptStore_NeverRead.set(0);
    sLilycoveDeptStore_DefaultFloorChoice.set(0);
    if (*gSaveBlock1Ptr).dynamicWarp.mapGroup == 13 {
        match (*gSaveBlock1Ptr).dynamicWarp.mapNum {
            20 => {
                sLilycoveDeptStore_NeverRead.set(0);
                sLilycoveDeptStore_DefaultFloorChoice.set(0);
            }
            19 => {
                sLilycoveDeptStore_NeverRead.set(0);
                sLilycoveDeptStore_DefaultFloorChoice.set(1);
            }
            18 => {
                sLilycoveDeptStore_NeverRead.set(0);
                sLilycoveDeptStore_DefaultFloorChoice.set(2);
            }
            17 => {
                sLilycoveDeptStore_NeverRead.set(0);
                sLilycoveDeptStore_DefaultFloorChoice.set(3);
            }
            16 => {
                sLilycoveDeptStore_NeverRead.set(0);
                sLilycoveDeptStore_DefaultFloorChoice.set(4);
            }
            _ => {}
        }
    }
    sLilycoveDeptStore_DefaultFloorChoice.get()
}
#[unsafe(no_mangle)]
pub unsafe fn MoveElevator() {
    let data: *mut i16 = (*gTasks.as_ptr())[CreateTask(Some(Task_MoveElevator), 9)]
        .data
        .as_mut_ptr();
    let mut floorDelta: u16 = 0;
    *data.at(1) = 0;
    *data.at(2) = 0;
    *data.at(4) = 1;
    if gSpecialVar_0x8005 > gSpecialVar_0x8006 {
        floorDelta = gSpecialVar_0x8005 - gSpecialVar_0x8006;
        *data.at(6) = TRUE as i16;
    } else {
        floorDelta = gSpecialVar_0x8006 - gSpecialVar_0x8005;
        *data.at(6) = FALSE as i16;
    }
    if floorDelta > 8 {
        floorDelta = 8;
    }
    *data.at(5) = sElevatorTripLength_31[floorDelta] as i16;
    SetCameraPanningCallback(None);
    MoveElevatorWindowLights(floorDelta, *data.at(6) as u8);
    PlaySE(SE_ELEVATOR);
}
pub(crate) unsafe fn Task_MoveElevator(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) += 1;
    if *data.at(1) % 3 == 0 {
        *data.at(1) = 0;
        *data.at(2) += 1;
        *data.at(4) = -*data.at(4);
        SetCameraPanning(0, *data.at(4));
        if *data.at(2) == *data.at(5) {
            PlaySE(SE_DING_DONG);
            DestroyTask(taskId);
            ScriptContext_Enable();
            InstallCameraPanAheadCallback();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ShowDeptStoreElevatorFloorSelect() {
    sTutorMoveAndElevatorWindowId
        .set(AddWindow((&raw const *sWindowTemplate_ElevatorFloor).cast_mut()) as u8);
    SetStandardWindowBorderStyle(sTutorMoveAndElevatorWindowId.get(), FALSE);
    let mut xPos: i32 = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        (*(&raw const crate::data::strings::gText_ElevatorNowOn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        64,
    );
    AddTextPrinterParameterized(
        sTutorMoveAndElevatorWindowId.get(),
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_ElevatorNowOn).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        xPos as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    xPos = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        sDeptStoreFloorNames[*(&raw const crate::ffi::gSpecialVar_0x8005)
            .cast::<u16>()
            .cast_mut()],
        64,
    );
    AddTextPrinterParameterized(
        sTutorMoveAndElevatorWindowId.get(),
        FONT_NORMAL,
        sDeptStoreFloorNames[*(&raw const crate::ffi::gSpecialVar_0x8005)
            .cast::<u16>()
            .cast_mut()],
        xPos as u8,
        17,
        TEXT_SKIP_DRAW,
        None,
    );
    PutWindowTilemap(sTutorMoveAndElevatorWindowId.get());
    CopyWindowToVram(sTutorMoveAndElevatorWindowId.get(), COPYWIN_FULL);
}
#[unsafe(no_mangle)]
pub unsafe fn CloseDeptStoreElevatorWindow() {
    ClearStdWindowAndFrameToTransparent(sTutorMoveAndElevatorWindowId.get(), TRUE);
    RemoveWindow(sTutorMoveAndElevatorWindowId.get());
}
unsafe fn MoveElevatorWindowLights(floorDelta: u16, descending: u8) {
    if FuncIsActiveTask(Some(Task_MoveElevatorWindowLights)) != TRUE {
        let taskId: u8 = CreateTask(Some(Task_MoveElevatorWindowLights), 8);
        task_set(taskId, 0, 0);
        task_set(taskId, 1, 0);
        task_set(taskId, 2, descending as i16);
        task_set(taskId, 3, sElevatorLightCycles_30[floorDelta] as i16);
    }
}
pub(crate) unsafe fn Task_MoveElevatorWindowLights(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    if *data.at(1) == 6 {
        *data += 1;
        if *data.at(2) == 0 {
            for y in 0..ELEVATOR_WINDOW_HEIGHT {
                for x in 0..ELEVATOR_WINDOW_WIDTH {
                    MapGridSetMetatileIdAt(
                        x as i32 + MAP_OFFSET + 1,
                        y as i32 + MAP_OFFSET,
                        sElevatorWindowTiles_Ascending[y][*data % 3] | MAPGRID_IMPASSABLE,
                    );
                }
            }
        } else {
            for y in 0..ELEVATOR_WINDOW_HEIGHT {
                for x in 0..ELEVATOR_WINDOW_WIDTH {
                    MapGridSetMetatileIdAt(
                        x as i32 + MAP_OFFSET + 1,
                        y as i32 + MAP_OFFSET,
                        sElevatorWindowTiles_Descending[y][*data % 3] | MAPGRID_IMPASSABLE,
                    );
                }
            }
        }
        DrawWholeMapView();
        *data.at(1) = 0;
        if *data == *data.at(3) {
            DestroyTask(taskId);
        }
    }
    *data.at(1) += 1;
}
#[unsafe(no_mangle)]
pub unsafe fn BufferVarsForIVRater() {
    let mut ivStorage: CArray<u32, 6> = zeroed();
    ivStorage[0] = GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_HP_IV,
    );
    ivStorage[1] = GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_ATK_IV,
    );
    ivStorage[2] = GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_DEF_IV,
    );
    ivStorage[3] = GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_SPEED_IV,
    );
    ivStorage[4] = GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_SPATK_IV,
    );
    ivStorage[5] = GetMonData2(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
        MON_DATA_SPDEF_IV,
    );
    gSpecialVar_0x8005 = 0;
    let mut i: u8 = 0;
    while i < NUM_STATS as u8 {
        gSpecialVar_0x8005 += ivStorage[i] as u16;
        i += 1;
    }
    gSpecialVar_0x8006 = 0;
    gSpecialVar_0x8007 = ivStorage[0] as u16;
    for i in 1..(NUM_STATS as u8) {
        if ivStorage[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
            < ivStorage[i]
        {
            gSpecialVar_0x8006 = i as u16;
            gSpecialVar_0x8007 = ivStorage[i] as u16;
        } else if ivStorage[*(&raw const crate::ffi::gSpecialVar_0x8006)
            .cast::<u16>()
            .cast_mut()]
            == ivStorage[i]
        {
            let randomNumber: u16 = Random();
            if randomNumber as i32 & 1 != 0 {
                gSpecialVar_0x8006 = i as u16;
                gSpecialVar_0x8007 = ivStorage[i] as u16;
            }
        }
    }
}
pub unsafe fn UsedPokemonCenterWarp() -> u8 {
    let map: u16 = ((gLastUsedWarp.mapGroup as u16) << 8) + gLastUsedWarp.mapNum as u16;
    let mut i: i32 = 0;
    while sPokemonCenters_29[i] != MAP_UNDEFINED {
        if sPokemonCenters_29[i] == map {
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerNotAtTrainerHillEntrance() -> u32 {
    if (*gSaveBlock1Ptr).location.mapGroup == 26 && (*gSaveBlock1Ptr).location.mapNum == 60 {
        return FALSE as u32;
    }
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateFrontierManiac(daysSince: u16) {
    let var: *mut u16 = GetVarPointer(VAR_FRONTIER_MANIAC_FACILITY);
    *var += daysSince;
    *var = (*var as i32 % 10) as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn ShowFrontierManiacMessage() {
    let mut winStreak: u16 = 0;
    let facility: u16 = VarGet(VAR_FRONTIER_MANIAC_FACILITY);
    match facility {
        FRONTIER_MANIAC_TOWER_SINGLES
        | FRONTIER_MANIAC_TOWER_DOUBLES
        | FRONTIER_MANIAC_TOWER_MULTIS
        | FRONTIER_MANIAC_TOWER_LINK => {
            if (*gSaveBlock2Ptr).frontier.towerWinStreaks[facility][0]
                >= (*gSaveBlock2Ptr).frontier.towerWinStreaks[facility][1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.towerWinStreaks[facility][0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.towerWinStreaks[facility][1];
            }
        }
        FRONTIER_MANIAC_DOME => {
            if (*gSaveBlock2Ptr).frontier.domeWinStreaks[0][0]
                >= (*gSaveBlock2Ptr).frontier.domeWinStreaks[0][1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.domeWinStreaks[0][0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.domeWinStreaks[0][1];
            }
        }
        FRONTIER_MANIAC_FACTORY => {
            if (*gSaveBlock2Ptr).frontier.factoryWinStreaks[0][0]
                >= (*gSaveBlock2Ptr).frontier.factoryWinStreaks[0][1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.factoryWinStreaks[0][0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.factoryWinStreaks[0][1];
            }
        }
        FRONTIER_MANIAC_PALACE => {
            if (*gSaveBlock2Ptr).frontier.palaceWinStreaks[0][0]
                >= (*gSaveBlock2Ptr).frontier.palaceWinStreaks[0][1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.palaceWinStreaks[0][0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.palaceWinStreaks[0][1];
            }
        }
        FRONTIER_MANIAC_ARENA => {
            if (*gSaveBlock2Ptr).frontier.arenaWinStreaks[0]
                >= (*gSaveBlock2Ptr).frontier.arenaWinStreaks[1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.arenaWinStreaks[0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.arenaWinStreaks[1];
            }
        }
        FRONTIER_MANIAC_PIKE => {
            if (*gSaveBlock2Ptr).frontier.pikeWinStreaks[0]
                >= (*gSaveBlock2Ptr).frontier.pikeWinStreaks[1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.pikeWinStreaks[1];
            }
        }
        FRONTIER_MANIAC_PYRAMID => {
            if (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[0]
                >= (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[1]
            {
                winStreak = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[0];
            } else {
                winStreak = (*gSaveBlock2Ptr).frontier.pyramidWinStreaks[1];
            }
        }
        _ => {}
    }
    let mut i: u8 = 0;
    while i < 2 && (sFrontierManiacStreakThresholds_28[facility][i] as u16) < winStreak {
        i += 1;
    }
    ShowFieldMessage(sFrontierManiacMessages_27[facility][i]);
}
#[unsafe(no_mangle)]
pub unsafe fn BufferBattleTowerElevatorFloors() {
    let battleMode: u16 = VarGet(VAR_FRONTIER_BATTLE_MODE);
    let lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    if battleMode == FRONTIER_MODE_MULTIS && FlagGet(FLAG_CHOSEN_MULTI_BATTLE_NPC_PARTNER) == 0 {
        gSpecialVar_0x8005 = 5;
        gSpecialVar_0x8006 = 4;
        return;
    }
    for i in 0..9u8 {
        if sBattleTowerStreakThresholds_26[i]
            > (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode]
        {
            gSpecialVar_0x8005 = 4;
            gSpecialVar_0x8006 = i as u16 + 5;
            return;
        }
    }
    gSpecialVar_0x8005 = 4;
    gSpecialVar_0x8006 = 12;
}
#[unsafe(no_mangle)]
pub unsafe fn ShowScrollableMultichoice() {
    let taskId: u8 = CreateTask(Some(Task_ShowScrollableMultichoice), 8);
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[tScrollMultiId] = gSpecialVar_0x8004 as i16;
    match *(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()
    {
        SCROLL_MULTI_NONE => {
            (*task).data[tMaxItemsOnScreen] = 1;
            (*task).data[tNumItems] = 1;
            (*task).data[tLeft] = 1;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 1;
            (*task).data[tHeight] = 1;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_GLASS_WORKSHOP_VENDOR => {
            (*task).data[tMaxItemsOnScreen] = 5;
            (*task).data[tNumItems] = 8;
            (*task).data[tLeft] = 1;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 9;
            (*task).data[tHeight] = 10;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_POKEMON_FAN_CLUB_RATER => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 12;
            (*task).data[tLeft] = 1;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 7;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1 => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 11;
            (*task).data[tLeft] = 14;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 15;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_2 => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 6;
            (*task).data[tLeft] = 14;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 15;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_VITAMIN_VENDOR => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 7;
            (*task).data[tLeft] = 14;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 15;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 10;
            (*task).data[tLeft] = 14;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 15;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BERRY_POWDER_VENDOR => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 12;
            (*task).data[tLeft] = 15;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 14;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_RECEPTIONIST => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 10;
            (*task).data[tLeft] = 17;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 11;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        9 | 10 => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 11;
            (*task).data[tLeft] = 15;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 14;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_SS_TIDAL_DESTINATION => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 7;
            (*task).data[tLeft] = 19;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 10;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BATTLE_TENT_RULES => {
            (*task).data[tMaxItemsOnScreen] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[tNumItems] = 7;
            (*task).data[tLeft] = 17;
            (*task).data[tTop] = 1;
            (*task).data[tWidth] = 12;
            (*task).data[tHeight] = 12;
            (*task).data[tKeepOpenAfterSelect] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        _ => {
            gSpecialVar_Result = MULTI_B_PRESSED;
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe fn Task_ShowScrollableMultichoice(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    LockPlayerFieldControls();
    sScrollableMultichoice_ScrollOffset.set(0);
    sScrollableMultichoice_ItemSpriteId.set(MAX_SPRITES);
    FillFrontierExchangeCornerWindowAndItemIcon((*task).data[tScrollMultiId] as u16, 0);
    ShowBattleFrontierTutorWindow((*task).data[tScrollMultiId] as u8, 0);
    sScrollableMultichoice_ListMenuItem =
        AllocZeroed((*task).data[tNumItems] as u32 * 8) as *mut ListMenuItem;
    sFrontierExchangeCorner_NeverRead.set(0);
    InitScrollableMultichoice();
    let mut width: u32 = 0;
    let mut i: u8 = 0;
    while (i as i16) < (*task).data[tNumItems] {
        let text: *mut u8 = sScrollableMultichoiceOptions
            [*(&raw const crate::ffi::gSpecialVar_0x8004)
                .cast::<u16>()
                .cast_mut()][i];
        (*sScrollableMultichoice_ListMenuItem.at(i)).name = text;
        (*sScrollableMultichoice_ListMenuItem.at(i)).id = i as i32;
        width = DisplayTextAndGetWidth(text, width as i32) as u32;
        i += 1;
    }
    (*task).data[tWidth] = ConvertPixelWidthToTileWidth(width as i32) as i16;
    if (*task).data[tLeft] as i32 + (*task).data[tWidth] as i32 > 29 {
        let adjustedLeft: i32 = 29 - (*task).data[tWidth] as i32;
        if adjustedLeft < 0 {
            (*task).data[tLeft] = 0;
        } else {
            (*task).data[tLeft] = adjustedLeft as i16;
        }
    }
    let mut template: WindowTemplate = CreateWindowTemplate(
        0,
        (*task).data[tLeft] as u8,
        (*task).data[tTop] as u8,
        (*task).data[tWidth] as u8,
        (*task).data[tHeight] as u8,
        0xF,
        0x64,
    );
    let windowId: u8 = AddWindow(&raw mut template) as u8;
    (*task).data[tWindowId] = windowId as i16;
    SetStandardWindowBorderStyle(windowId, FALSE);
    gScrollableMultichoice_ListMenuTemplate.totalItems = (*task).data[tNumItems] as u16;
    gScrollableMultichoice_ListMenuTemplate.maxShowed = (*task).data[tMaxItemsOnScreen] as u16;
    gScrollableMultichoice_ListMenuTemplate.windowId = (*task).data[tWindowId] as u8;
    ScrollableMultichoice_UpdateScrollArrows(taskId);
    (*task).data[tListTaskId] = ListMenuInit(
        &raw mut gScrollableMultichoice_ListMenuTemplate,
        (*task).data[tScrollOffset] as u16,
        (*task).data[tSelectedRow] as u16,
    ) as i16;
    ScheduleBgCopyTilemapToVram(0);
    task_set_func(taskId, Some(ScrollableMultichoice_ProcessInput));
}
unsafe fn InitScrollableMultichoice() {
    gScrollableMultichoice_ListMenuTemplate.items = sScrollableMultichoice_ListMenuItem;
    gScrollableMultichoice_ListMenuTemplate.moveCursorFunc = Some(ScrollableMultichoice_MoveCursor);
    gScrollableMultichoice_ListMenuTemplate.itemPrintFunc = None;
    gScrollableMultichoice_ListMenuTemplate.totalItems = 1;
    gScrollableMultichoice_ListMenuTemplate.maxShowed = 1;
    gScrollableMultichoice_ListMenuTemplate.windowId = 0;
    gScrollableMultichoice_ListMenuTemplate.header_X = 0;
    gScrollableMultichoice_ListMenuTemplate.item_X = 8;
    gScrollableMultichoice_ListMenuTemplate.cursor_X = 0;
    gScrollableMultichoice_ListMenuTemplate.set_upText_Y(1);
    gScrollableMultichoice_ListMenuTemplate.set_cursorPal(2);
    gScrollableMultichoice_ListMenuTemplate.set_fillValue(1);
    gScrollableMultichoice_ListMenuTemplate.set_cursorShadowPal(3);
    gScrollableMultichoice_ListMenuTemplate.set_lettersSpacing(0);
    gScrollableMultichoice_ListMenuTemplate.set_itemVerticalPadding(0);
    gScrollableMultichoice_ListMenuTemplate.set_scrollMultiple(LIST_NO_MULTIPLE_SCROLL);
    gScrollableMultichoice_ListMenuTemplate.set_fontId(FONT_NORMAL);
    gScrollableMultichoice_ListMenuTemplate.set_cursorKind(CURSOR_BLACK_ARROW);
}
pub(crate) unsafe fn ScrollableMultichoice_MoveCursor(
    itemIndex: i32,
    onInit: u8,
    list: *mut ListMenu,
) {
    PlaySE(SE_SELECT);
    let taskId: u8 = FindTaskIdByFunc(Some(ScrollableMultichoice_ProcessInput));
    if taskId != TASK_NONE {
        let mut selection: u16 = 0;
        let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
        ListMenuGetScrollAndRow(
            (*task).data[tListTaskId] as u8,
            &raw mut selection,
            null_mut(),
        );
        sScrollableMultichoice_ScrollOffset.set(selection);
        ListMenuGetCurrentItemArrayId((*task).data[tListTaskId] as u8, &raw mut selection);
        HideFrontierExchangeCornerItemIcon(
            (*task).data[tScrollMultiId] as u16,
            sFrontierExchangeCorner_NeverRead.get(),
        );
        FillFrontierExchangeCornerWindowAndItemIcon((*task).data[tScrollMultiId] as u16, selection);
        ShowBattleFrontierTutorMoveDescription((*task).data[tScrollMultiId] as u8, selection);
        sFrontierExchangeCorner_NeverRead.set(selection);
    }
}
pub(crate) unsafe fn ScrollableMultichoice_ProcessInput(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let input: i32 = ListMenu_ProcessInput((*task).data[tListTaskId] as u8);
    match input {
        LIST_NOTHING_CHOSEN => {}
        LIST_CANCEL => {
            gSpecialVar_Result = MULTI_B_PRESSED;
            PlaySE(SE_SELECT);
            CloseScrollableMultichoice(taskId);
        }
        _ => {
            gSpecialVar_Result = input as u16;
            PlaySE(SE_SELECT);
            if (*task).data[tKeepOpenAfterSelect] == 0 {
                CloseScrollableMultichoice(taskId);
            } else if input == (*task).data[tNumItems] as i32 - 1 {
                CloseScrollableMultichoice(taskId);
            } else {
                ScrollableMultichoice_RemoveScrollArrows(taskId);
                (*task).func = Some(Task_ScrollableMultichoice_WaitReturnToList);
                ScriptContext_Enable();
            }
        }
    }
}
unsafe fn CloseScrollableMultichoice(taskId: u8) {
    let mut selection: u16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    ListMenuGetCurrentItemArrayId((*task).data[tListTaskId] as u8, &raw mut selection);
    HideFrontierExchangeCornerItemIcon((*task).data[tScrollMultiId] as u16, selection);
    ScrollableMultichoice_RemoveScrollArrows(taskId);
    DestroyListMenuTask((*task).data[tListTaskId] as u8, null_mut(), null_mut());
    Free(sScrollableMultichoice_ListMenuItem as *mut c_void);
    ClearStdWindowAndFrameToTransparent((*task).data[tWindowId] as u8, TRUE);
    FillWindowPixelBuffer((*task).data[tWindowId] as u8, 0);
    CopyWindowToVram((*task).data[tWindowId] as u8, COPYWIN_GFX);
    RemoveWindow((*task).data[tWindowId] as u8);
    DestroyTask(taskId);
    ScriptContext_Enable();
}
pub(crate) unsafe fn Task_ScrollableMultichoice_WaitReturnToList(taskId: u8) {
    if task_get(taskId, tKeepOpenAfterSelect) == 2 {
        task_set(taskId, tKeepOpenAfterSelect, 1);
        task_set_func(taskId, Some(Task_ScrollableMultichoice_ReturnToList));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrollableMultichoice_TryReturnToList() {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
    if taskId == TASK_NONE {
        ScriptContext_Enable();
    } else {
        task_set(
            taskId,
            tKeepOpenAfterSelect,
            task_get(taskId, tKeepOpenAfterSelect) + 1,
        );
    }
}
pub(crate) unsafe fn Task_ScrollableMultichoice_ReturnToList(taskId: u8) {
    LockPlayerFieldControls();
    ScrollableMultichoice_UpdateScrollArrows(taskId);
    task_set_func(taskId, Some(ScrollableMultichoice_ProcessInput));
}
unsafe fn ScrollableMultichoice_UpdateScrollArrows(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let mut template: ScrollArrowsTemplate = *sScrollableMultichoice_ScrollArrowsTemplate_25;
    if (*task).data[tMaxItemsOnScreen] != (*task).data[tNumItems] {
        template.firstX =
            ((*task).data[tWidth] / 2) as u8 * 8 + 12 + ((*task).data[tLeft] as u8 - 1) * 8;
        template.firstY = 8;
        template.secondX =
            ((*task).data[tWidth] / 2) as u8 * 8 + 12 + ((*task).data[tLeft] as u8 - 1) * 8;
        template.secondY = (*task).data[tHeight] as u8 * 8 + 10;
        template.fullyUpThreshold = 0;
        template.fullyDownThreshold =
            (*task).data[tNumItems] as u16 - (*task).data[tMaxItemsOnScreen] as u16;
        (*task).data[tScrollArrowId] = AddScrollIndicatorArrowPair(
            &raw mut template,
            sScrollableMultichoice_ScrollOffset.as_ptr(),
        ) as i16;
    }
}
unsafe fn ScrollableMultichoice_RemoveScrollArrows(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[tMaxItemsOnScreen] != (*task).data[tNumItems] {
        RemoveScrollIndicatorArrowPair((*task).data[tScrollArrowId] as u8);
    }
}
#[unsafe(no_mangle)]
pub fn ShowGlassWorkshopMenu() {}
#[unsafe(no_mangle)]
pub unsafe fn SetBattleTowerLinkPlayerGfx() {
    for i in 0..2u8 {
        if gLinkPlayers[i].gender == MALE {
            VarSet(VAR_OBJ_GFX_ID_F - i as u16, OBJ_EVENT_GFX_BRENDAN_NORMAL);
        } else {
            VarSet(
                VAR_OBJ_GFX_ID_F - i as u16,
                OBJ_EVENT_GFX_RIVAL_MAY_NORMAL as u16,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ShowNatureGirlMessage() {
    if gSpecialVar_0x8004 >= PARTY_SIZE as u16 {
        gSpecialVar_0x8004 = 0;
    }
    let nature: u8 = GetNature(
        &raw mut gPlayerParty[*(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut()],
    );
    ShowFieldMessage(sNatureGirlMessages_24[nature]);
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateFrontierGambler(daysSince: u16) {
    let var: *mut u16 = GetVarPointer(VAR_FRONTIER_GAMBLER_CHALLENGE);
    *var += daysSince;
    *var = (*var as i32 % 12) as u16;
}
#[unsafe(no_mangle)]
pub unsafe fn ShowFrontierGamblerLookingMessage() {
    let challenge: u16 = VarGet(VAR_FRONTIER_GAMBLER_CHALLENGE);
    ShowFieldMessage(sFrontierGamblerLookingMessages_23[challenge]);
    VarSet(VAR_FRONTIER_GAMBLER_SET_CHALLENGE, challenge);
}
#[unsafe(no_mangle)]
pub unsafe fn ShowFrontierGamblerGoMessage() {
    ShowFieldMessage(sFrontierGamblerGoMessages_22[VarGet(VAR_FRONTIER_GAMBLER_SET_CHALLENGE)]);
}
pub unsafe fn FrontierGamblerSetWonOrLost(won: u8) {
    let battleMode: u16 = VarGet(VAR_FRONTIER_BATTLE_MODE);
    let challenge: u16 = VarGet(VAR_FRONTIER_GAMBLER_SET_CHALLENGE);
    let frontierFacilityId: u16 = VarGet(VAR_FRONTIER_FACILITY);
    if VarGet(VAR_FRONTIER_GAMBLER_STATE) == FRONTIER_GAMBLER_PLACED_BET
        && sFrontierChallenges_21[challenge] as i32
            == ((frontierFacilityId as i32) << 8) + battleMode as i32
    {
        if won != 0 {
            VarSet(VAR_FRONTIER_GAMBLER_STATE, FRONTIER_GAMBLER_WON);
        } else {
            VarSet(VAR_FRONTIER_GAMBLER_STATE, FRONTIER_GAMBLER_LOST);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateBattlePointsWindow() {
    let mut string: CArray<u8, 32> = zeroed();
    StringCopy(
        ConvertIntToDecimalStringN(
            string.as_mut_ptr(),
            (*gSaveBlock2Ptr).frontier.battlePoints as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        ),
        (*(&raw const crate::data::strings::gText_BP).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
    );
    let x: u32 = GetStringRightAlignXOffset(FONT_NORMAL as i32, string.as_mut_ptr(), 48) as u32;
    AddTextPrinterParameterized(
        sBattlePointsWindowId.get(),
        FONT_NORMAL,
        string.as_mut_ptr(),
        x as u8,
        1,
        0,
        None,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn ShowBattlePointsWindow() {
    sBattlePointsWindowId
        .set(AddWindow((&raw const *sBattlePoints_WindowTemplate_20).cast_mut()) as u8);
    SetStandardWindowBorderStyle(sBattlePointsWindowId.get(), FALSE);
    UpdateBattlePointsWindow();
    CopyWindowToVram(sBattlePointsWindowId.get(), COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe fn CloseBattlePointsWindow() {
    ClearStdWindowAndFrameToTransparent(sBattlePointsWindowId.get(), TRUE);
    RemoveWindow(sBattlePointsWindowId.get());
}
#[unsafe(no_mangle)]
pub unsafe fn TakeFrontierBattlePoints() {
    if (*gSaveBlock2Ptr).frontier.battlePoints < gSpecialVar_0x8004 {
        (*gSaveBlock2Ptr).frontier.battlePoints = 0;
    } else {
        (*gSaveBlock2Ptr).frontier.battlePoints -= *(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GiveFrontierBattlePoints() {
    if (*gSaveBlock2Ptr).frontier.battlePoints as i32 + gSpecialVar_0x8004 as i32
        > MAX_BATTLE_FRONTIER_POINTS as i32
    {
        (*gSaveBlock2Ptr).frontier.battlePoints = MAX_BATTLE_FRONTIER_POINTS;
    } else {
        (*gSaveBlock2Ptr).frontier.battlePoints += *(&raw const crate::ffi::gSpecialVar_0x8004)
            .cast::<u16>()
            .cast_mut();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetFrontierBattlePoints() -> u16 {
    (*gSaveBlock2Ptr).frontier.battlePoints
}
#[unsafe(no_mangle)]
pub unsafe fn ShowFrontierExchangeCornerItemIconWindow() {
    sFrontierExchangeCorner_ItemIconWindowId.set(AddWindow(
        (&raw const *sFrontierExchangeCorner_ItemIconWindowTemplate_19).cast_mut(),
    ) as u8);
    SetStandardWindowBorderStyle(sFrontierExchangeCorner_ItemIconWindowId.get(), FALSE);
    CopyWindowToVram(sFrontierExchangeCorner_ItemIconWindowId.get(), COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe fn CloseFrontierExchangeCornerItemIconWindow() {
    ClearStdWindowAndFrameToTransparent(sFrontierExchangeCorner_ItemIconWindowId.get(), TRUE);
    RemoveWindow(sFrontierExchangeCorner_ItemIconWindowId.get());
}
unsafe fn FillFrontierExchangeCornerWindowAndItemIcon(menu: u16, selection: u16) {
    if (SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1
        ..=SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR)
        .contains(&menu)
    {
        FillWindowPixelRect(0, 17, 0, 0, 216, 32);
        match menu {
            SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1 => {
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sFrontierExchangeCorner_Decor1Descriptions_18[selection],
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                if sFrontierExchangeCorner_Decor1_17[selection] == ITEM_LIST_END {
                    ShowFrontierExchangeCornerItemIcon(
                        sFrontierExchangeCorner_Decor1_17[selection],
                    );
                } else {
                    FreeSpriteTilesByTag(TAG_ITEM_ICON);
                    FreeSpritePaletteByTag(TAG_ITEM_ICON);
                    sScrollableMultichoice_ItemSpriteId.set(AddDecorationIconObject(
                        sFrontierExchangeCorner_Decor1_17[selection] as u8,
                        33,
                        88,
                        0,
                        TAG_ITEM_ICON,
                        TAG_ITEM_ICON,
                    ));
                }
            }
            SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_2 => {
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sFrontierExchangeCorner_Decor2Descriptions_16[selection],
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                if sFrontierExchangeCorner_Decor2_15[selection] == ITEM_LIST_END {
                    ShowFrontierExchangeCornerItemIcon(
                        sFrontierExchangeCorner_Decor2_15[selection],
                    );
                } else {
                    FreeSpriteTilesByTag(TAG_ITEM_ICON);
                    FreeSpritePaletteByTag(TAG_ITEM_ICON);
                    sScrollableMultichoice_ItemSpriteId.set(AddDecorationIconObject(
                        sFrontierExchangeCorner_Decor2_15[selection] as u8,
                        33,
                        88,
                        0,
                        TAG_ITEM_ICON,
                        TAG_ITEM_ICON,
                    ));
                }
            }
            SCROLL_MULTI_BF_EXCHANGE_CORNER_VITAMIN_VENDOR => {
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sFrontierExchangeCorner_VitaminsDescriptions_14[selection],
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                ShowFrontierExchangeCornerItemIcon(sFrontierExchangeCorner_Vitamins_13[selection]);
            }
            SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR => {
                AddTextPrinterParameterized2(
                    0,
                    FONT_NORMAL,
                    sFrontierExchangeCorner_HoldItemsDescriptions_12[selection],
                    0,
                    None,
                    TEXT_COLOR_DARK_GRAY,
                    TEXT_COLOR_WHITE,
                    TEXT_COLOR_LIGHT_GRAY,
                );
                ShowFrontierExchangeCornerItemIcon(sFrontierExchangeCorner_HoldItems_11[selection]);
            }
            _ => {}
        }
    }
}
unsafe fn ShowFrontierExchangeCornerItemIcon(item: u16) {
    FreeSpriteTilesByTag(TAG_ITEM_ICON);
    FreeSpritePaletteByTag(TAG_ITEM_ICON);
    sScrollableMultichoice_ItemSpriteId.set(AddItemIconSprite(TAG_ITEM_ICON, TAG_ITEM_ICON, item));
    if sScrollableMultichoice_ItemSpriteId.get() != MAX_SPRITES {
        gSprites[sScrollableMultichoice_ItemSpriteId.get()]
            .oam
            .set_priority(0);
        gSprites[sScrollableMultichoice_ItemSpriteId.get()].x = 36;
        gSprites[sScrollableMultichoice_ItemSpriteId.get()].y = 92;
    }
}
unsafe fn HideFrontierExchangeCornerItemIcon(menu: u16, unused: u16) {
    if sScrollableMultichoice_ItemSpriteId.get() != MAX_SPRITES {
        match menu {
            SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1
            | SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_2
            | SCROLL_MULTI_BF_EXCHANGE_CORNER_VITAMIN_VENDOR
            | SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR => {
                DestroySpriteAndFreeResources(
                    &raw mut gSprites[sScrollableMultichoice_ItemSpriteId.get()],
                );
            }
            _ => {}
        }
        sScrollableMultichoice_ItemSpriteId.set(MAX_SPRITES);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn BufferBattleFrontierTutorMoveName() {
    if gSpecialVar_0x8005 != 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[sBattleFrontier_TutorMoves2
                [*(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut()]]
            .as_ptr()
            .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            (*(&raw const crate::data::data_tables::gMoveNames)
                .cast::<CArray<CArray<u8, 13>, 355>>())[sBattleFrontier_TutorMoves1
                [*(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut()]]
            .as_ptr()
            .cast_mut(),
        );
    }
}
unsafe fn ShowBattleFrontierTutorWindow(menu: u8, selection: u16) {
    if menu == SCROLL_MULTI_BF_MOVE_TUTOR_1 || menu == SCROLL_MULTI_BF_MOVE_TUTOR_2 {
        if gSpecialVar_0x8006 == 0 {
            sTutorMoveAndElevatorWindowId.set(AddWindow(
                (&raw const *sBattleFrontierTutor_WindowTemplate_10).cast_mut(),
            ) as u8);
            SetStandardWindowBorderStyle(sTutorMoveAndElevatorWindowId.get(), FALSE);
        }
        ShowBattleFrontierTutorMoveDescription(menu, selection);
    }
}
unsafe fn ShowBattleFrontierTutorMoveDescription(menu: u8, selection: u16) {
    if menu == SCROLL_MULTI_BF_MOVE_TUTOR_1 || menu == SCROLL_MULTI_BF_MOVE_TUTOR_2 {
        FillWindowPixelRect(sTutorMoveAndElevatorWindowId.get(), 17, 0, 0, 96, 48);
        if menu == SCROLL_MULTI_BF_MOVE_TUTOR_2 {
            AddTextPrinterParameterized(
                sTutorMoveAndElevatorWindowId.get(),
                FONT_NORMAL,
                sBattleFrontier_TutorMoveDescriptions2_9[selection],
                0,
                1,
                0,
                None,
            );
        } else {
            AddTextPrinterParameterized(
                sTutorMoveAndElevatorWindowId.get(),
                FONT_NORMAL,
                sBattleFrontier_TutorMoveDescriptions1_8[selection],
                0,
                1,
                0,
                None,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CloseBattleFrontierTutorWindow() {
    ClearStdWindowAndFrameToTransparent(sTutorMoveAndElevatorWindowId.get(), TRUE);
    RemoveWindow(sTutorMoveAndElevatorWindowId.get());
}
#[unsafe(no_mangle)]
pub unsafe fn ScrollableMultichoice_RedrawPersistentMenu() {
    let mut scrollOffset: u16 = 0;
    let mut selectedRow: u16 = 0;
    let taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
    if taskId != TASK_NONE {
        let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
        ListMenuGetScrollAndRow(
            (*task).data[tListTaskId] as u8,
            &raw mut scrollOffset,
            &raw mut selectedRow,
        );
        SetStandardWindowBorderStyle((*task).data[tWindowId] as u8, FALSE);
        for i in 0..(MAX_SCROLL_MULTI_ON_SCREEN as u8) {
            AddTextPrinterParameterized5(
                (*task).data[tWindowId] as u8,
                FONT_NORMAL,
                sScrollableMultichoiceOptions[*(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut()][scrollOffset as i32 + i as i32],
                10,
                i * 16,
                TEXT_SKIP_DRAW,
                None,
                0,
                0,
            );
        }
        AddTextPrinterParameterized(
            (*task).data[tWindowId] as u8,
            FONT_NORMAL,
            (*(&raw const crate::data::strings::gText_SelectorArrow).cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            0,
            selectedRow as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
        PutWindowTilemap((*task).data[tWindowId] as u8);
        CopyWindowToVram((*task).data[tWindowId] as u8, COPYWIN_FULL);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattleFrontierTutorMoveIndex() {
    let mut i: u8 = 0;
    gSpecialVar_0x8005 = 0;
    let moveTutor: u16 = VarGet(VAR_TEMP_FRONTIER_TUTOR_ID);
    let moveIndex: u16 = VarGet(VAR_TEMP_FRONTIER_TUTOR_SELECTION);
    if moveTutor != 0 {
        i = 0;
        loop {
            if (*(&raw const crate::data::party_menu::gTutorMoves).cast::<CArray<u16, 0>>())[i]
                == sBattleFrontier_TutorMoves2[moveIndex]
            {
                gSpecialVar_0x8005 = i as u16;
                break;
            }
            i += 1;
            if i >= TUTOR_MOVE_COUNT {
                break;
            }
        }
    } else {
        i = 0;
        loop {
            if (*(&raw const crate::data::party_menu::gTutorMoves).cast::<CArray<u16, 0>>())[i]
                == sBattleFrontier_TutorMoves1[moveIndex]
            {
                gSpecialVar_0x8005 = i as u16;
                break;
            }
            i += 1;
            if i >= TUTOR_MOVE_COUNT {
                break;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn ScrollableMultichoice_ClosePersistentMenu() {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
    if taskId != TASK_NONE {
        let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
        DestroyListMenuTask((*task).data[tListTaskId] as u8, null_mut(), null_mut());
        Free(sScrollableMultichoice_ListMenuItem as *mut c_void);
        ClearStdWindowAndFrameToTransparent((*task).data[tWindowId] as u8, TRUE);
        FillWindowPixelBuffer((*task).data[tWindowId] as u8, 0);
        ClearWindowTilemap((*task).data[tWindowId] as u8);
        CopyWindowToVram((*task).data[tWindowId] as u8, COPYWIN_GFX);
        RemoveWindow((*task).data[tWindowId] as u8);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DoDeoxysRockInteraction() {
    CreateTask(Some(Task_DeoxysRockInteraction), 8);
}
pub(crate) unsafe fn Task_DeoxysRockInteraction(taskId: u8) {
    if FlagGet(FLAG_DEOXYS_ROCK_COMPLETE) == TRUE {
        gSpecialVar_Result = DEOXYS_ROCK_COMPLETE;
        ScriptContext_Enable();
        DestroyTask(taskId);
    } else {
        let mut rockLevel: u16 = VarGet(VAR_DEOXYS_ROCK_LEVEL);
        let stepCount: u16 = VarGet(VAR_DEOXYS_ROCK_STEP_COUNT);
        VarSet(VAR_DEOXYS_ROCK_STEP_COUNT, 0);
        if rockLevel != 0 && (sStoneMaxStepCounts_7[rockLevel as i32 - 1] as u16) < stepCount {
            ChangeDeoxysRockLevel(0);
            VarSet(VAR_DEOXYS_ROCK_LEVEL, 0);
            gSpecialVar_Result = DEOXYS_ROCK_FAILED;
            DestroyTask(taskId);
        } else if rockLevel == 10 {
            FlagSet(FLAG_DEOXYS_ROCK_COMPLETE);
            gSpecialVar_Result = DEOXYS_ROCK_SOLVED;
            ScriptContext_Enable();
            DestroyTask(taskId);
        } else {
            rockLevel += 1;
            ChangeDeoxysRockLevel(rockLevel as u8);
            VarSet(VAR_DEOXYS_ROCK_LEVEL, rockLevel);
            gSpecialVar_Result = DEOXYS_ROCK_PROGRESSED;
            DestroyTask(taskId);
        }
    }
}
unsafe fn ChangeDeoxysRockLevel(rockLevel: u8) {
    let mut objectEventId: u8 = 0;
    LoadPalette(
        (&raw const sDeoxysRockPalettes[rockLevel]).cast_mut() as *mut c_void,
        416,
        8,
    );
    TryGetObjectEventIdByLocalIdAndMap(
        LOCALID_BIRTH_ISLAND_EXTERIOR_ROCK,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectEventId,
    );
    if rockLevel == 0 {
        PlaySE(SE_M_CONFUSE_RAY);
    } else {
        PlaySE(SE_RG_DEOXYS_MOVE);
    }
    CreateTask(Some(WaitForDeoxysRockMovement), 8);
    gFieldEffectArguments[0] = LOCALID_BIRTH_ISLAND_EXTERIOR_ROCK as i32;
    gFieldEffectArguments[1] = 58;
    gFieldEffectArguments[2] = 26;
    gFieldEffectArguments[3] = sDeoxysRockCoords[rockLevel][0] as i32;
    gFieldEffectArguments[4] = sDeoxysRockCoords[rockLevel][1] as i32;
    if rockLevel == 0 {
        gFieldEffectArguments[5] = 60;
    } else {
        gFieldEffectArguments[5] = 5;
    }
    FieldEffectStart(FLDEFF_MOVE_DEOXYS_ROCK);
    SetObjEventTemplateCoords(
        1,
        sDeoxysRockCoords[rockLevel][0] as i16,
        sDeoxysRockCoords[rockLevel][1] as i16,
    );
}
pub(crate) unsafe fn WaitForDeoxysRockMovement(taskId: u8) {
    if FieldEffectActiveListContains(FLDEFF_MOVE_DEOXYS_ROCK) == FALSE {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
pub unsafe fn IncrementBirthIslandRockStepCount() {
    let mut stepCount: u16 = VarGet(VAR_DEOXYS_ROCK_STEP_COUNT);
    if (*gSaveBlock1Ptr).location.mapNum == 58 && (*gSaveBlock1Ptr).location.mapGroup == 26 {
        if ({
            stepCount += 1;
            stepCount
        }) > 99
        {
            VarSet(VAR_DEOXYS_ROCK_STEP_COUNT, 0);
        } else {
            VarSet(VAR_DEOXYS_ROCK_STEP_COUNT, stepCount);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetDeoxysRockPalette() {
    LoadPalette(
        (&raw const sDeoxysRockPalettes[VarGet(VAR_DEOXYS_ROCK_LEVEL) as u8]).cast_mut()
            as *mut c_void,
        416,
        8,
    );
    BlendPalettes(0x4000000, 16, 0);
}
pub unsafe fn SetPCBoxToSendMon(boxId: u8) {
    sPCBoxToSendMon.set(boxId);
}
#[unsafe(no_mangle)]
pub unsafe fn GetPCBoxToSendMon() -> u16 {
    sPCBoxToSendMon.get() as u16
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldShowBoxWasFullMessage() -> u8 {
    if FlagGet(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE) == 0
        && StorageGetCurrentBox() as u16 != VarGet(VAR_PC_BOX_TO_SEND_MON)
    {
        FlagSet(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
        return TRUE;
    }
    FALSE
}
pub unsafe fn IsDestinationBoxFull() -> u8 {
    SetPCBoxToSendMon(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8);
    let mut r#box: i32 = StorageGetCurrentBox() as i32;
    loop {
        for i in 0..IN_BOX_COUNT {
            if GetBoxMonData3(
                GetBoxedMonPtr(r#box as u8, i as u8),
                MON_DATA_SPECIES,
                null_mut(),
            ) == 0
            {
                if GetPCBoxToSendMon() as i32 != r#box {
                    FlagClear(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
                }
                VarSet(VAR_PC_BOX_TO_SEND_MON, r#box as u16);
                return ShouldShowBoxWasFullMessage();
            }
        }
        if ({
            r#box += 1;
            r#box
        }) == TOTAL_BOXES_COUNT as i32
        {
            r#box = 0;
        }
        if r#box == StorageGetCurrentBox() as i32 {
            break;
        }
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn CreateAbnormalWeatherEvent() {
    let mut randomValue: u16 = Random();
    VarSet(VAR_ABNORMAL_WEATHER_STEP_COUNTER, 0);
    if FlagGet(FLAG_DEFEATED_KYOGRE) == TRUE {
        VarSet(
            VAR_ABNORMAL_WEATHER_LOCATION,
            (randomValue as i32 % 8) as u16 + TERRA_CAVE_LOCATIONS_START,
        );
    } else if FlagGet(FLAG_DEFEATED_GROUDON) == TRUE {
        VarSet(
            VAR_ABNORMAL_WEATHER_LOCATION,
            (randomValue as i32 % 8) as u16 + MARINE_CAVE_LOCATIONS_START,
        );
    } else if randomValue as i32 & 1 == 0 {
        randomValue = Random();
        VarSet(
            VAR_ABNORMAL_WEATHER_LOCATION,
            (randomValue as i32 % 8) as u16 + TERRA_CAVE_LOCATIONS_START,
        );
    } else {
        randomValue = Random();
        VarSet(
            VAR_ABNORMAL_WEATHER_LOCATION,
            (randomValue as i32 % 8) as u16 + MARINE_CAVE_LOCATIONS_START,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetAbnormalWeatherMapNameAndType() -> u32 {
    let abnormalWeather: u16 = VarGet(VAR_ABNORMAL_WEATHER_LOCATION);
    GetMapName(
        gStringVar1.as_mut_ptr(),
        sAbnormalWeatherMapNumbers_6[abnormalWeather as i32 - 1] as u16,
        0,
    );
    if abnormalWeather < MARINE_CAVE_LOCATIONS_START {
        return FALSE as u32;
    } else {
        return TRUE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn AbnormalWeatherHasExpired() -> u8 {
    let mut steps: u16 = VarGet(VAR_ABNORMAL_WEATHER_STEP_COUNTER);
    let abnormalWeather: u16 = VarGet(VAR_ABNORMAL_WEATHER_LOCATION);
    if abnormalWeather == ABNORMAL_WEATHER_NONE {
        return FALSE;
    }
    if ({
        steps += 1;
        steps
    }) > 999
    {
        VarSet(VAR_ABNORMAL_WEATHER_STEP_COUNTER, 0);
        if (*gSaveBlock1Ptr).location.mapGroup == 24
            && let 101..=105 = (*gSaveBlock1Ptr).location.mapNum
        {
            VarSet(VAR_SHOULD_END_ABNORMAL_WEATHER, 1);
            return FALSE;
        }
        if (*gSaveBlock1Ptr).location.mapGroup == 0 {
            match (*gSaveBlock1Ptr).location.mapNum {
                52 | 54 | 55 | 56 => {
                    VarSet(VAR_SHOULD_END_ABNORMAL_WEATHER, 1);
                    return FALSE;
                }
                _ => {}
            }
        }
        if (*gSaveBlock1Ptr).location.mapNum as i32
            == sAbnormalWeatherMapNumbers_5[abnormalWeather as i32 - 1] as i32
            && (*gSaveBlock1Ptr).location.mapGroup == 0
        {
            return TRUE;
        } else {
            VarSet(VAR_ABNORMAL_WEATHER_LOCATION, ABNORMAL_WEATHER_NONE);
            return FALSE;
        }
    } else {
        VarSet(VAR_ABNORMAL_WEATHER_STEP_COUNTER, steps);
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Unused_SetWeatherSunny() {
    SetCurrentAndNextWeather(WEATHER_SUNNY);
}
#[unsafe(no_mangle)]
pub unsafe fn GetMartEmployeeObjectEventId() -> u32 {
    for i in 0..12u8 {
        if (*gSaveBlock1Ptr).location.mapGroup as i32 == sPokeMarts_4[i][0] as i32
            && (*gSaveBlock1Ptr).location.mapNum as i32 == sPokeMarts_4[i][1] as i32
        {
            return sPokeMarts_4[i][2] as u32;
        }
    }
    1
}
#[unsafe(no_mangle)]
pub unsafe fn IsTrainerRegistered() -> u32 {
    let index: i32 = GetRematchIdxByTrainerIdx(gSpecialVar_0x8004 as i32);
    if index >= 0 && FlagGet(TRAINER_REGISTERED_FLAGS_START + index as u16) == TRUE {
        return TRUE as u32;
    }
    FALSE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn ShouldDistributeEonTicket() -> u32 {
    if VarGet(VAR_DISTRIBUTE_EON_TICKET) == 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
#[unsafe(no_mangle)]
pub unsafe fn BattleTowerReconnectLink() {
    sBattleTowerMultiBattleTypeFlags.set(gBattleTypeFlags);
    gBattleTypeFlags = 0;
    if gReceivedRemoteLinkPlayers == 0 {
        CreateTask(Some(Task_ReconnectWithLinkPlayers), 5);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LinkRetireStatusWithBattleTowerPartner() {
    CreateTask(Some(Task_LinkRetireStatusWithBattleTowerPartner), 5);
}
pub(crate) unsafe fn Task_LinkRetireStatusWithBattleTowerPartner(taskId: u8) {
    match task_get(taskId, tState) {
        0 => {
            if FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers)) == 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        1 => {
            if IsLinkTaskFinished() == TRUE {
                if GetMultiplayerId() == 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                } else {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        &raw mut gSpecialVar_0x8004 as *mut c_void,
                        2,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
        }
        2 => {
            if GetBlockReceivedStatus() as i32 & 2 != 0 {
                if GetMultiplayerId() == 0 {
                    gSpecialVar_0x8005 = gBlockRecvBuffer[1][0];
                    ResetBlockReceivedFlag(1);
                    if gSpecialVar_0x8004 == BATTLE_TOWER_LINK_RETIRE
                        && gSpecialVar_0x8005 == BATTLE_TOWER_LINK_RETIRE
                    {
                        gSpecialVar_Result = BATTLE_TOWER_LINKSTAT_BOTH_RETIRE;
                    } else if gSpecialVar_0x8004 == BATTLE_TOWER_LINK_CONTINUE
                        && gSpecialVar_0x8005 == BATTLE_TOWER_LINK_RETIRE
                    {
                        gSpecialVar_Result = BATTLE_TOWER_LINKSTAT_MEMBER_RETIRE;
                    } else if gSpecialVar_0x8004 == BATTLE_TOWER_LINK_RETIRE
                        && gSpecialVar_0x8005 == BATTLE_TOWER_LINK_CONTINUE
                    {
                        gSpecialVar_Result = BATTLE_TOWER_LINKSTAT_LEADER_RETIRE;
                    } else {
                        gSpecialVar_Result = BATTLE_TOWER_LINKSTAT_CONTINUE;
                    }
                }
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        3 => {
            if IsLinkTaskFinished() == TRUE {
                if GetMultiplayerId() != 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                } else {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        &raw mut gSpecialVar_Result as *mut c_void,
                        2,
                    );
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
        }
        4 => {
            if GetBlockReceivedStatus() as i32 & 1 != 0 {
                if GetMultiplayerId() != 0 {
                    gSpecialVar_Result = gBlockRecvBuffer[0][0];
                    ResetBlockReceivedFlag(0);
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                } else {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                }
            }
        }
        5 => {
            if GetMultiplayerId() == 0 {
                if gSpecialVar_Result == BATTLE_TOWER_LINKSTAT_MEMBER_RETIRE {
                    ShowFieldAutoScrollMessage(
                        (*crate::asmdata::gText_YourPartnerHasRetired.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                }
            } else {
                if gSpecialVar_Result == BATTLE_TOWER_LINKSTAT_LEADER_RETIRE {
                    ShowFieldAutoScrollMessage(
                        (*crate::asmdata::gText_YourPartnerHasRetired.cast::<CArray<u8, 0>>())
                            .as_ptr()
                            .cast_mut(),
                    );
                }
            }
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        6 => {
            if IsTextPrinterActive(0) == 0 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        7 => {
            if IsLinkTaskFinished() == TRUE {
                SetLinkStandbyCallback();
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        8 => {
            if IsLinkTaskFinished() == TRUE {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        9 => {
            if gWirelessCommType == 0 {
                SetCloseLinkCallback();
            }
            gBattleTypeFlags = sBattleTowerMultiBattleTypeFlags.get();
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn Script_DoRayquazaScene() {
    if gSpecialVar_0x8004 == 0 {
        DoRayquazaScene(0, TRUE, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    } else {
        DoRayquazaScene(1, FALSE, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn LoopWingFlapSE() {
    CreateTask(Some(Task_LoopWingFlapSE), 8);
    PlaySE(SE_M_WING_ATTACK);
}
pub(crate) unsafe fn Task_LoopWingFlapSE(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(1) += 1;
    if *data.at(1) as i32 == gSpecialVar_0x8005 as i32 {
        *data += 1;
        *data.at(1) = 0;
        PlaySE(SE_M_WING_ATTACK);
    }
    if *data as i32 == gSpecialVar_0x8004 as i32 - 1 {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn CloseBattlePikeCurtain() {
    let taskId: u8 = CreateTask(Some(Task_CloseBattlePikeCurtain), 8);
    task_set(taskId, 0, 4);
    task_set(taskId, 1, 4);
    task_set(taskId, 2, 4);
    task_set(taskId, tCurrentFrame, 0);
}
pub(crate) unsafe fn Task_CloseBattlePikeCurtain(taskId: u8) {
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    *data.at(*data.at(3)) -= 1;
    if *data.at(*data.at(3)) == 0 {
        for y in 0..CURTAIN_HEIGHT {
            for x in 0..CURTAIN_WIDTH {
                MapGridSetMetatileIdAt(
                    (*gSaveBlock1Ptr).pos.x as i32 + x as i32 + MAP_OFFSET - 1,
                    (*gSaveBlock1Ptr).pos.y as i32 + y as i32 + MAP_OFFSET - 3,
                    x as u16
                        + METATILE_BattlePike_CurtainFrames_Start
                        + y as u16 * METATILE_ROW_WIDTH
                        + *data.at(3) as u16 * CURTAIN_HEIGHT as u16 * METATILE_ROW_WIDTH,
                );
            }
        }
        DrawWholeMapView();
        *data.at(3) += 1;
        if *data.at(3) == 3 {
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetBattlePyramidHint() {
    gSpecialVar_Result = (gSpecialVar_0x8004 as i32 / 7) as u16;
    gSpecialVar_Result -= (gSpecialVar_Result as i32 / 20) as u16 * TOTAL_PYRAMID_ROUNDS;
}
#[unsafe(no_mangle)]
pub unsafe fn ResetHealLocationFromDewford() {
    if (*gSaveBlock1Ptr).lastHealLocation.mapGroup == 0
        && (*gSaveBlock1Ptr).lastHealLocation.mapNum == 11
    {
        SetLastHealLocationWarp(HEAL_LOCATION_PETALBURG_CITY);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn InPokemonCenter() -> u8 {
    let map: u16 = (((*gSaveBlock1Ptr).location.mapGroup as u16) << 8)
        + (*gSaveBlock1Ptr).location.mapNum as u16;
    let mut i: i32 = 0;
    while sPokemonCenters_3[i] != MAP_UNDEFINED {
        if sPokemonCenters_3[i] == map {
            return TRUE;
        }
        i += 1;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn ResetFanClub() {
    (*gSaveBlock1Ptr).vars[65] = 0;
    (*gSaveBlock1Ptr).vars[66] = 0;
}
#[unsafe(no_mangle)]
pub unsafe fn TryLoseFansFromPlayTimeAfterLinkBattle() {
    if DidPlayerGetFirstFans() != 0 {
        TryLoseFansFromPlayTime();
        (*gSaveBlock1Ptr).vars[66] = (*gSaveBlock2Ptr).playTimeHours;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn UpdateTrainerFanClubGameClear() {
    if ((*gSaveBlock1Ptr).vars[65] >> 7) as i32 & 1 == 0 {
        SetPlayerGotFirstFans();
        SetInitialFansOfPlayer();
        (*gSaveBlock1Ptr).vars[66] = (*gSaveBlock2Ptr).playTimeHours;
        FlagClear(FLAG_HIDE_FANCLUB_OLD_LADY);
        FlagClear(FLAG_HIDE_FANCLUB_BOY);
        FlagClear(FLAG_HIDE_FANCLUB_LITTLE_BOY);
        FlagClear(FLAG_HIDE_FANCLUB_LADY);
        FlagClear(FLAG_HIDE_LILYCOVE_FAN_CLUB_INTERVIEWER);
        VarSet(VAR_LILYCOVE_FAN_CLUB_STATE, 1);
    }
}
pub unsafe fn TryGainNewFanFromCounter(incrementId: u8) -> u8 {
    if VarGet(VAR_LILYCOVE_FAN_CLUB_STATE) == 2 {
        if ((*gSaveBlock1Ptr).vars[65] as i32 & 0x007F) + sCounterIncrements_2[incrementId] as i32
            > 19
        {
            if GetNumFansOfPlayerInTrainerFanClub() < 3 {
                PlayerGainRandomTrainerFan();
                (*gSaveBlock1Ptr).vars[65] &= 65408;
            } else {
                (*gSaveBlock1Ptr).vars[65] = (*gSaveBlock1Ptr).vars[65] & 65408 | 20;
            }
        } else {
            (*gSaveBlock1Ptr).vars[65] += sCounterIncrements_2[incrementId] as u16;
        }
    }
    (*gSaveBlock1Ptr).vars[65] as u8 & 0x007F
}
unsafe fn PlayerGainRandomTrainerFan() -> u16 {
    let mut idx: u8 = 0;
    for i in 0..8u8 {
        if shr_i32(
            (*gSaveBlock1Ptr).vars[65] as i32,
            sFanClubMemberIds_1[i] as u32,
        ) & 1
            == 0
        {
            idx = i;
            if Random() as i32 & 1 != 0 {
                (*gSaveBlock1Ptr).vars[65] |= shl_i32(1, sFanClubMemberIds_1[idx] as u32) as u16;
                return idx as u16;
            }
        }
    }
    (*gSaveBlock1Ptr).vars[65] |= shl_i32(1, sFanClubMemberIds_1[idx] as u32) as u16;
    idx as u16
}
unsafe fn PlayerLoseRandomTrainerFan() -> u16 {
    let mut idx: u8 = 0;
    if GetNumFansOfPlayerInTrainerFanClub() == 1 {
        return 0;
    }
    for i in 0..8u8 {
        if shr_i32(
            (*gSaveBlock1Ptr).vars[65] as i32,
            sFanClubMemberIds_0[i] as u32,
        ) & 1
            != 0
        {
            idx = i;
            if Random() as i32 & 1 != 0 {
                (*gSaveBlock1Ptr).vars[65] ^= shl_i32(1, sFanClubMemberIds_0[idx] as u32) as u16;
                return idx as u16;
            }
        }
    }
    if shr_i32(
        (*gSaveBlock1Ptr).vars[65] as i32,
        sFanClubMemberIds_0[idx] as u32,
    ) & 1
        != 0
    {
        (*gSaveBlock1Ptr).vars[65] ^= shl_i32(1, sFanClubMemberIds_0[idx] as u32) as u16;
    }
    idx as u16
}
#[unsafe(no_mangle)]
pub unsafe fn GetNumFansOfPlayerInTrainerFanClub() -> u16 {
    let mut numFans: u8 = 0;
    for i in 0..NUM_TRAINER_FAN_CLUB_MEMBERS {
        if shr_i32(
            (*gSaveBlock1Ptr).vars[65] as i32,
            i as u32 + FANCLUB_MEMBER1,
        ) & 1
            != 0
        {
            numFans += 1;
        }
    }
    numFans as u16
}
#[unsafe(no_mangle)]
pub unsafe fn TryLoseFansFromPlayTime() {
    let mut i: u8 = 0;
    if (*gSaveBlock2Ptr).playTimeHours < 999 {
        loop {
            if GetNumFansOfPlayerInTrainerFanClub() < 5 {
                (*gSaveBlock1Ptr).vars[66] = (*gSaveBlock2Ptr).playTimeHours;
                break;
            } else if i == NUM_TRAINER_FAN_CLUB_MEMBERS {
                break;
            } else if ((*gSaveBlock2Ptr).playTimeHours as i32 - (*gSaveBlock1Ptr).vars[66] as i32)
                < 12
            {
                return;
            }
            PlayerLoseRandomTrainerFan();
            (*gSaveBlock1Ptr).vars[66] += 12;
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn IsFanClubMemberFanOfPlayer() -> u8 {
    shr_i32((*gSaveBlock1Ptr).vars[65] as i32, gSpecialVar_0x8004 as u32) as u8 & 1
}
unsafe fn SetInitialFansOfPlayer() {
    (*gSaveBlock1Ptr).vars[65] |= 8192;
    (*gSaveBlock1Ptr).vars[65] |= 256;
    (*gSaveBlock1Ptr).vars[65] |= 1024;
}
#[unsafe(no_mangle)]
pub unsafe fn BufferFanClubTrainerName() {
    let mut whichLinkTrainer: u8 = 0;
    let mut whichNPCTrainer: u8 = 0;
    match *(&raw const crate::ffi::gSpecialVar_0x8004)
        .cast::<u16>()
        .cast_mut()
    {
        8 => {}
        FANCLUB_MEMBER2 => {}
        FANCLUB_MEMBER3 => {
            whichLinkTrainer = 0;
            whichNPCTrainer = 3;
        }
        FANCLUB_MEMBER4 => {
            whichLinkTrainer = 0;
            whichNPCTrainer = 1;
        }
        FANCLUB_MEMBER5 => {
            whichLinkTrainer = 1;
            whichNPCTrainer = 0;
        }
        FANCLUB_MEMBER6 => {
            whichLinkTrainer = 0;
            whichNPCTrainer = 4;
        }
        FANCLUB_MEMBER7 => {
            whichLinkTrainer = 1;
            whichNPCTrainer = 5;
        }
        FANCLUB_MEMBER8 => {}
        _ => {}
    }
    BufferFanClubTrainerName_(
        &raw mut (*gSaveBlock1Ptr).linkBattleRecords,
        whichLinkTrainer,
        whichNPCTrainer,
    );
}
unsafe fn BufferFanClubTrainerName_(
    linkRecords: *mut LinkBattleRecords,
    whichLinkTrainer: u8,
    whichNPCTrainer: u8,
) {
    let record: *mut LinkBattleRecord = &raw mut (*linkRecords).entries[whichLinkTrainer];
    if (*record).name[0] == EOS {
        match whichNPCTrainer {
            0 => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Wallace).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            1 => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Steven).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            2 => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Brawly).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            3 => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Winona).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            4 => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Phoebe).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            5 => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Glacia).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
            _ => {
                StringCopy(
                    gStringVar1.as_mut_ptr(),
                    (*(&raw const crate::data::strings::gText_Wallace).cast::<CArray<u8, 0>>())
                        .as_ptr()
                        .cast_mut(),
                );
            }
        }
    } else {
        StringCopyN(
            gStringVar1.as_mut_ptr(),
            (*record).name.as_mut_ptr(),
            PLAYER_NAME_LENGTH as u8,
        );
        gStringVar1[7] = EOS;
        ConvertInternationalString(
            gStringVar1.as_mut_ptr(),
            (*linkRecords).languages[whichLinkTrainer],
        );
    }
}
pub unsafe fn UpdateTrainerFansAfterLinkBattle() {
    if VarGet(VAR_LILYCOVE_FAN_CLUB_STATE) == 2 {
        TryLoseFansFromPlayTimeAfterLinkBattle();
        if gBattleOutcome == B_OUTCOME_WON {
            PlayerGainRandomTrainerFan();
        } else {
            PlayerLoseRandomTrainerFan();
        }
    }
}
unsafe fn DidPlayerGetFirstFans() -> u8 {
    ((*gSaveBlock1Ptr).vars[65] >> 7) as u8 & 1
}
#[unsafe(no_mangle)]
pub unsafe fn SetPlayerGotFirstFans() {
    (*gSaveBlock1Ptr).vars[65] |= 128;
}
#[unsafe(no_mangle)]
pub unsafe fn Script_TryGainNewFanFromCounter() -> u8 {
    TryGainNewFanFromCounter(gSpecialVar_0x8004 as u8)
}
