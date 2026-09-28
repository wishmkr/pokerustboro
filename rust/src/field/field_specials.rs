//! Translated from `src/field_specials.c` by tools/rustport/c2rs.py.
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

#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBikeCyclingChallenge: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gBikeCollisions: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBikeCyclingTimer: u32 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlidingDoorNextFrameCounter: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSlidingDoorFrame: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sTutorMoveAndElevatorWindowId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLilycoveDeptStore_NeverRead: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sLilycoveDeptStore_DefaultFloorChoice: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ListMenuItem: *mut ListMenuItem = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ScrollOffset: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierExchangeCorner_NeverRead: u16 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sScrollableMultichoice_ItemSpriteId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattlePointsWindowId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrontierExchangeCorner_ItemIconWindowId: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPCBoxToSendMon: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sBattleTowerMultiBattleTypeFlags: u32 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gScrollableMultichoice_ListMenuTemplate: ListMenuTemplate = unsafe { zeroed() };

unsafe extern "C" {
    static mut gBattleOutcome: u8;
    static mut gBattleTypeFlags: u32;
    static mut gBlockRecvBuffer: CArray<CArray<u16, 128>, 5>;
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gLastUsedWarp: WarpData;
    static mut gLinkPlayers: CArray<LinkPlayer, 5>;
    static mut gLocalTime: Time;
    static mut gMain: Main;
    static mut gMapHeader: MapHeader;
    static gMoveNames: CArray<CArray<u8, 13>, 355>;
    static gObjectEventPal_Brendan: CArray<u16, 0>;
    static gObjectEventPal_May: CArray<u16, 0>;
    static gObjectEventPal_RubySapphireBrendan: CArray<u16, 0>;
    static gObjectEventPal_RubySapphireMay: CArray<u16, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gReceivedRemoteLinkPlayers: u8;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSaveBlock2Ptr: *mut SaveBlock2;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_0x8006: u16;
    static mut gSpecialVar_0x8007: u16;
    static mut gSpecialVar_Result: u16;
    static gSpeciesInfo: CArray<SpeciesInfo, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gStringVar1: CArray<u8, 256>;
    static mut gStringVar2: CArray<u8, 256>;
    static mut gStringVar4: CArray<u8, 1000>;
    static gTVStringVarPtrs: CArray<*mut u8, 3>;
    static mut gTasks: CArray<Task, 0>;
    static gText_1MinutePlus: CArray<u8, 0>;
    static gText_99TimesPlus: CArray<u8, 0>;
    static gText_BP: CArray<u8, 0>;
    static gText_BigGirl: CArray<u8, 0>;
    static gText_BigGuy: CArray<u8, 0>;
    static gText_Brawly: CArray<u8, 0>;
    static gText_Daughter: CArray<u8, 0>;
    static gText_ElevatorNowOn: CArray<u8, 0>;
    static gText_Glacia: CArray<u8, 0>;
    static gText_Phoebe: CArray<u8, 0>;
    static gText_SelectorArrow: CArray<u8, 0>;
    static gText_Son: CArray<u8, 0>;
    static gText_SpaceSeconds: CArray<u8, 0>;
    static gText_SpaceTimes: CArray<u8, 0>;
    static gText_Steven: CArray<u8, 0>;
    static gText_Wallace: CArray<u8, 0>;
    static gText_Winona: CArray<u8, 0>;
    static gText_YourPartnerHasRetired: CArray<u8, 0>;
    static gTutorMoves: CArray<u16, 0>;
    static mut gWirelessCommType: u8;
    fn AddDecorationIconObject(a0: u8, a1: i16, a2: i16, a3: u8, a4: u16, a5: u16) -> u8;
    fn AddItemIconSprite(a0: u16, a1: u16, a2: u16) -> u8;
    fn AddScrollIndicatorArrowPair(a0: *mut ScrollArrowsTemplate, a1: *mut u16) -> u8;
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
    fn AddTextPrinterParameterized5(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut TextPrinterTemplate, u16)>,
        a7: u8,
        a8: u8,
    );
    fn AddWindow(a0: *mut WindowTemplate) -> u16;
    fn AllocZeroed(a0: u32) -> *mut c_void;
    fn BitmaskAllOtherLinkPlayers() -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CB2_ReturnToField();
    fn CB2_ReturnToFieldContinueScriptPlayMapMusic();
    fn CB2_ShowDiploma();
    fn CB2_ViewWallClock();
    fn CalculatePlayerPartyCount() -> u8;
    fn CameraObjectSetFollowedSpriteId(a0: u8);
    fn CheckFreePokemonStorageSpace() -> u8;
    fn CheckPartyPokerus(a0: *mut Pokemon, a1: u8) -> u8;
    fn ClearStdWindowAndFrameToTransparent(a0: u8, a1: u8);
    fn ClearWindowTilemap(a0: u8);
    fn ConvertIntToDecimalString(a0: u8, a1: i32);
    fn ConvertIntToDecimalStringN(a0: *mut u8, a1: i32, a2: i32, a3: u8) -> *mut u8;
    fn ConvertInternationalString(a0: *mut u8, a1: u8);
    fn ConvertPixelWidthToTileWidth(a0: i32) -> i32;
    fn CopyMonFavoritePokeblockName(a0: u8, a1: *mut u8) -> u8;
    fn CopyWindowToVram(a0: u8, a1: u8);
    fn CountDigits(a0: i32) -> u32;
    fn CreateMon(a0: *mut Pokemon, a1: u16, a2: u8, a3: u8, a4: u8, a5: u32, a6: u8, a7: u32);
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
    fn DestroyListMenuTask(a0: u8, a1: *mut u16, a2: *mut u16);
    fn DestroySpriteAndFreeResources(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn DisplayTextAndGetWidth(a0: *mut u8, a1: i32) -> i32;
    fn DoRayquazaScene(a0: u8, a1: u8, a2: Option<unsafe extern "C" fn()>);
    fn DrawWholeMapView();
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldInitRegionMap(a0: Option<unsafe extern "C" fn()>);
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FillWindowPixelRect(a0: u8, a1: u8, a2: u16, a3: u16, a4: u16, a5: u16);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn Free(a0: *mut c_void);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetBlockReceivedStatus() -> u8;
    fn GetBoxMonData3(a0: *mut BoxPokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetBoxedMonPtr(a0: u8, a1: u8) -> *mut BoxPokemon;
    fn GetEreaderTrainerName(a0: *mut u8);
    fn GetGameStat(a0: u8) -> u32;
    fn GetLastUsedWarpMapType() -> u8;
    fn GetLinkPlayerCount() -> u8;
    fn GetMapName(a0: *mut u8, a1: u16, a2: u16) -> *mut u8;
    fn GetMonData2(a0: *mut Pokemon, a1: i32) -> u32;
    fn GetMonData3(a0: *mut Pokemon, a1: i32, a2: *mut u8) -> u32;
    fn GetMonEVCount(a0: *mut Pokemon) -> u16;
    fn GetMultiplayerId() -> u8;
    fn GetNature(a0: *mut Pokemon) -> u8;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetPlayerAvatarSpriteId() -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetRematchIdxByTrainerIdx(a0: i32) -> i32;
    fn GetRibbonCount(a0: *mut Pokemon) -> u8;
    fn GetStarterPokemon(a0: u16) -> u16;
    fn GetStringCenterAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetStringRightAlignXOffset(a0: i32, a1: *mut u8, a2: i32) -> i32;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn IncrementGameStat(a0: u8);
    fn InstallCameraPanAheadCallback();
    fn IsLinkTaskFinished() -> u8;
    fn IsMapTypeOutdoors(a0: u8) -> u8;
    fn IsPokeNewsActive(a0: u8) -> u8;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn ItemIdToBattleMoveId(a0: u16) -> u16;
    fn ListMenuGetCurrentItemArrayId(a0: u8, a1: *mut u16);
    fn ListMenuGetScrollAndRow(a0: u8, a1: *mut u16, a2: *mut u16);
    fn ListMenuInit(a0: *mut ListMenuTemplate, a1: u16, a2: u16) -> u8;
    fn ListMenu_ProcessInput(a0: u8) -> i32;
    fn LoadPalette(a0: *mut c_void, a1: u16, a2: u16);
    fn LockPlayerFieldControls();
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MapGridSetMetatileIdAt(a0: i32, a1: i32, a2: u16);
    fn MysteryGift_GetCardStat(a0: u32) -> u16;
    fn Overworld_SetSavedMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PutWindowTilemap(a0: u8);
    fn Random() -> u16;
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn RemoveScrollIndicatorArrowPair(a0: u8);
    fn RemoveWindow(a0: u8);
    fn ResetBlockReceivedFlag(a0: u8);
    fn ScheduleBgCopyTilemapToVram(a0: u8);
    fn ScriptContext_Enable();
    fn SendBlock(a0: u8, a1: *mut c_void, a2: u16) -> u8;
    fn SetCameraPanning(a0: i16, a1: i16);
    fn SetCameraPanningCallback(a0: Option<unsafe extern "C" fn()>);
    fn SetCloseLinkCallback();
    fn SetCurrentAndNextWeather(a0: u8);
    fn SetLastHealLocationWarp(a0: u8);
    fn SetLinkStandbyCallback();
    fn SetMainCallback2(a0: Option<unsafe extern "C" fn()>);
    fn SetMonData(a0: *mut Pokemon, a1: i32, a2: *mut c_void);
    fn SetObjEventTemplateCoords(a0: u8, a1: i16, a2: i16);
    fn SetSavedWeather(a0: u32);
    fn SetStandardWindowBorderStyle(a0: u8, a1: u8);
    fn SetWarpDestination(a0: i8, a1: i8, a2: i8, a3: i8, a4: i8);
    fn ShowFieldAutoScrollMessage(a0: *mut u8) -> u8;
    fn ShowFieldMessage(a0: *mut u8) -> u8;
    fn SpawnSpecialObjectEventParameterized(a0: u8, a1: u8, a2: u8, a3: i16, a4: i16, a5: u8)
    -> u8;
    fn StorageGetCurrentBox() -> u8;
    fn StringAppend(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCompare(a0: *mut u8, a1: *mut u8) -> i32;
    fn StringCopy(a0: *mut u8, a1: *mut u8) -> *mut u8;
    fn StringCopyN(a0: *mut u8, a1: *mut u8, a2: u8) -> *mut u8;
    fn Task_ReconnectWithLinkPlayers(a0: u8);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn TryPutSpotTheCutiesOnAir(a0: *mut Pokemon, a1: u8);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_ShowDiploma() {
    SetMainCallback2(Some(CB2_ShowDiploma));
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_ViewWallClock() {
    gMain.savedCallback = Some(CB2_ReturnToField);
    SetMainCallback2(Some(CB2_ViewWallClock));
    LockPlayerFieldControls();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetCyclingRoadChallengeData() {
    gBikeCyclingChallenge = FALSE;
    gBikeCollisions = 0;
    sBikeCyclingTimer = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Special_BeginCyclingRoadChallenge() {
    gBikeCyclingChallenge = TRUE;
    gBikeCollisions = 0;
    sBikeCyclingTimer = gMain.vblankCounter1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarBike() -> u16 {
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_ACRO_BIKE) != 0 {
        return 1;
    }
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_MACH_BIKE) != 0 {
        return 2;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn DetermineCyclingRoadResults(numFrames: u32, numBikeCollisions: u8) {
    let mut result: u8 = 0;
    if numBikeCollisions < 100 {
        ConvertIntToDecimalStringN(
            gStringVar1.as_mut_ptr(),
            numBikeCollisions as i32,
            STR_CONV_MODE_LEFT_ALIGN,
            2,
        );
        StringAppend(
            gStringVar1.as_mut_ptr(),
            gText_SpaceTimes.as_ptr().cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gText_99TimesPlus.as_ptr().cast_mut(),
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
            &raw mut gStringVar2[3],
            (numFrames % 60 * 100 / 60) as i32,
            STR_CONV_MODE_LEADING_ZEROS,
            2,
        );
        StringAppend(
            gStringVar2.as_mut_ptr(),
            gText_SpaceSeconds.as_ptr().cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar2.as_mut_ptr(),
            gText_1MinutePlus.as_ptr().cast_mut(),
        );
    }
    result = 0;
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
pub unsafe extern "C" fn FinishCyclingRoadChallenge() {
    let mut numFrames: u32 = gMain.vblankCounter1 - sBikeCyclingTimer;
    DetermineCyclingRoadResults(numFrames, gBikeCollisions);
    RecordCyclingRoadResults(numFrames, gBikeCollisions);
}
pub(crate) unsafe extern "C" fn RecordCyclingRoadResults(numFrames: u32, numBikeCollisions: u8) {
    let mut low: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_L);
    let mut high: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_H);
    let mut framesRecord: u32 = low as u32 + ((high as u32) << 16);
    if framesRecord > numFrames || framesRecord == 0 {
        VarSet(VAR_CYCLING_ROAD_RECORD_TIME_L, numFrames as u16);
        VarSet(VAR_CYCLING_ROAD_RECORD_TIME_H, (numFrames >> 16) as u16);
        VarSet(VAR_CYCLING_ROAD_RECORD_COLLISIONS, numBikeCollisions as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRecordedCyclingRoadResults() -> u16 {
    let mut low: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_L);
    let mut high: u16 = VarGet(VAR_CYCLING_ROAD_RECORD_TIME_H);
    let mut framesRecord: u32 = low as u32 + ((high as u32) << 16);
    if framesRecord == 0 {
        return FALSE as u16;
    }
    DetermineCyclingRoadResults(
        framesRecord,
        VarGet(VAR_CYCLING_ROAD_RECORD_COLLISIONS) as u8,
    );
    return TRUE as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateCyclingRoadState() {
    if gLastUsedWarp.mapNum == 12 && gLastUsedWarp.mapGroup == 29 {
        return;
    }
    if VarGet(VAR_CYCLING_CHALLENGE_STATE) == 2 || VarGet(VAR_CYCLING_CHALLENGE_STATE) == 3 {
        VarSet(VAR_CYCLING_CHALLENGE_STATE, 0);
        Overworld_SetSavedMusic(MUS_DUMMY);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSSTidalFlag() {
    FlagSet(FLAG_SYS_CRUISE_MODE);
    *GetVarPointer(VAR_CRUISE_STEP_COUNT) = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetSSTidalFlag() {
    FlagClear(FLAG_SYS_CRUISE_MODE);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CountSSTidalStep(delta: u16) -> u32 {
    if FlagGet(FLAG_SYS_CRUISE_MODE) == 0
        || ({
            *GetVarPointer(VAR_CRUISE_STEP_COUNT) += delta;
            *GetVarPointer(VAR_CRUISE_STEP_COUNT)
        }) < SS_TIDAL_MAX_STEPS
    {
        return FALSE as u32;
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSSTidalLocation(
    mapGroup: *mut i8,
    mapNum: *mut i8,
    x: *mut i16,
    y: *mut i16,
) -> u8 {
    let mut varCruiseStepCount: *mut u16 = GetVarPointer(VAR_CRUISE_STEP_COUNT);
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
    return SS_TIDAL_LOCATION_CURRENTS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoWallyCall() -> u32 {
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
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoScottFortreeCall() -> u32 {
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
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoScottBattleFrontierCall() -> u32 {
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
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoRoxanneCall() -> u32 {
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
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDoRivalRayquazaCall() -> u32 {
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
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLinkPartnerNames() -> u8 {
    let mut i: u8 = 0;
    let mut j: u8 = 0;
    let mut myLinkPlayerNumber: u8 = GetMultiplayerId();
    let mut nLinkPlayers: u8 = GetLinkPlayerCount();
    i = 0;
    while i < nLinkPlayers {
        if myLinkPlayerNumber != i {
            StringCopy(gTVStringVarPtrs[j], gLinkPlayers[i].name.as_mut_ptr());
            j += 1;
        }
        i += 1;
    }
    return nLinkPlayers;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpawnLinkPartnerObjectEvent() {
    let mut j: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut movementTypes: CArray<u8, 4> = CArray([7, 9, 8, 10]);
    let mut coordOffsets: CArray<CArray<i8, 2>, 4> = zeroed();
    coordOffsets[0][0] = 0;
    coordOffsets[0][1] = 1;
    coordOffsets[1][0] = 1;
    coordOffsets[1][1] = 0;
    coordOffsets[2][0] = 0;
    coordOffsets[2][1] = -1;
    coordOffsets[3][0] = -1;
    coordOffsets[3][1] = 0;
    let mut myLinkPlayerNumber: u8 = 0;
    let mut playerFacingDirection: u8 = 0;
    let mut linkSpriteId: u8 = 0;
    let mut i: u8 = 0;
    myLinkPlayerNumber = GetMultiplayerId();
    playerFacingDirection = GetPlayerFacingDirection();
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
    i = 0;
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
pub(crate) unsafe extern "C" fn LoadLinkPartnerObjectEventSpritePalette(
    graphicsId: u8,
    localEventId: u8,
    paletteNum: u8,
) {
    let mut adjustedPaletteNum: u8 = 0;
    adjustedPaletteNum = paletteNum + 6;
    if graphicsId == OBJ_EVENT_GFX_LINK_RS_BRENDAN
        || graphicsId == OBJ_EVENT_GFX_LINK_RS_MAY
        || graphicsId == OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL
        || graphicsId == OBJ_EVENT_GFX_RIVAL_MAY_NORMAL
    {
        let mut obj: u8 = GetObjectEventIdByLocalIdAndMap(
            localEventId,
            (*gSaveBlock1Ptr).location.mapNum as u8,
            (*gSaveBlock1Ptr).location.mapGroup as u8,
        );
        if obj != OBJECT_EVENTS_COUNT {
            let mut spriteId: u8 = gObjectEvents[obj].spriteId;
            let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
            (*sprite).oam.set_paletteNum(adjustedPaletteNum as u16);
            match graphicsId {
                OBJ_EVENT_GFX_LINK_RS_BRENDAN => {
                    LoadPalette(
                        gObjectEventPal_RubySapphireBrendan.as_ptr().cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                OBJ_EVENT_GFX_LINK_RS_MAY => {
                    LoadPalette(
                        gObjectEventPal_RubySapphireMay.as_ptr().cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                OBJ_EVENT_GFX_RIVAL_BRENDAN_NORMAL => {
                    LoadPalette(
                        gObjectEventPal_Brendan.as_ptr().cast_mut() as *mut c_void,
                        0x100 + adjustedPaletteNum as u16 * 16,
                        32,
                    );
                }
                OBJ_EVENT_GFX_RIVAL_MAY_NORMAL => {
                    LoadPalette(
                        gObjectEventPal_May.as_ptr().cast_mut() as *mut c_void,
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
pub unsafe extern "C" fn MauvilleGymPressSwitch() {
    let mut i: u8 = 0;
    i = 0;
    while i < 4 {
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
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MauvilleGymSetDefaultBarriers() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    y = 12;
    while y < 24 {
        x = MAP_OFFSET;
        while x < 16 {
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
            x += 1;
        }
        y += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MauvilleGymDeactivatePuzzle() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut switchCoords: *mut UCoords8 = sMauvilleGymSwitchCoords.as_ptr().cast_mut();
    i = 3;
    while i >= 0 {
        MapGridSetMetatileIdAt(
            (*switchCoords).x as i32,
            (*switchCoords).y as i32,
            METATILE_MauvilleGym_PressedSwitch,
        );
        switchCoords = switchCoords.at(1);
        i -= 1;
    }
    y = 12;
    while y < 24 {
        x = MAP_OFFSET;
        while x < 16 {
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
            x += 1;
        }
        y += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PetalburgGymSlideOpenRoomDoors() {
    sSlidingDoorNextFrameCounter = 0;
    sSlidingDoorFrame = 0;
    PlaySE(SE_UNLOCK);
    CreateTask(Some(Task_PetalburgGymSlideOpenRoomDoors), 8);
}
pub(crate) unsafe extern "C" fn Task_PetalburgGymSlideOpenRoomDoors(taskId: u8) {
    if sSlidingDoorNextFrameDelay[sSlidingDoorFrame] == sSlidingDoorNextFrameCounter {
        PetalburgGymSetDoorMetatiles(
            gSpecialVar_0x8004 as u8,
            sPetalburgGymSlidingDoorMetatiles[sSlidingDoorFrame],
        );
        sSlidingDoorNextFrameCounter = 0;
        if ({
            sSlidingDoorFrame += 1;
            sSlidingDoorFrame
        }) == 5
        {
            DestroyTask(taskId);
            ScriptContext_Enable();
        }
    } else {
        sSlidingDoorNextFrameCounter += 1;
    }
}
pub(crate) unsafe extern "C" fn PetalburgGymSetDoorMetatiles(roomNumber: u8, metatileId: u16) {
    let mut doorCoordsX: CArray<u16, 4> = zeroed();
    let mut doorCoordsY: CArray<u16, 4> = zeroed();
    let mut i: u8 = 0;
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
    i = 0;
    while i < nDoors {
        MapGridSetMetatileIdAt(
            doorCoordsX[i] as i32 + MAP_OFFSET,
            doorCoordsY[i] as i32 + MAP_OFFSET,
            metatileId | MAPGRID_IMPASSABLE,
        );
        MapGridSetMetatileIdAt(
            doorCoordsX[i] as i32 + MAP_OFFSET,
            doorCoordsY[i] as i32 + MAP_OFFSET + 1,
            metatileId + METATILE_ROW_WIDTH | MAPGRID_IMPASSABLE,
        );
        i += 1;
    }
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PetalburgGymUnlockRoomDoors() {
    PetalburgGymSetDoorMetatiles(
        gSpecialVar_0x8004 as u8,
        sPetalburgGymSlidingDoorMetatiles[4],
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFieldMessageStringVar4() {
    ShowFieldMessage(gStringVar4.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StorePlayerCoordsInVars() {
    gSpecialVar_0x8004 = (*gSaveBlock1Ptr).pos.x as u16;
    gSpecialVar_0x8005 = (*gSaveBlock1Ptr).pos.y as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerTrainerIdOnesDigit() -> u8 {
    return ((((*gSaveBlock2Ptr).playerTrainerId[1] as u16 as i32) << 8
        | (*gSaveBlock2Ptr).playerTrainerId[0] as u16 as i32)
        % 10) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerBigGuyGirlString() {
    if (*gSaveBlock2Ptr).playerGender == MALE {
        StringCopy(gStringVar1.as_mut_ptr(), gText_BigGuy.as_ptr().cast_mut());
    } else {
        StringCopy(gStringVar1.as_mut_ptr(), gText_BigGirl.as_ptr().cast_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRivalSonDaughterString() {
    if (*gSaveBlock2Ptr).playerGender == MALE {
        StringCopy(gStringVar1.as_mut_ptr(), gText_Daughter.as_ptr().cast_mut());
    } else {
        StringCopy(gStringVar1.as_mut_ptr(), gText_Son.as_ptr().cast_mut());
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleOutcome() -> u8 {
    return gBattleOutcome;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CableCarWarp() {
    if gSpecialVar_0x8004 != 0 {
        SetWarpDestination(19, 0, WARP_ID_NONE, 6, 4);
    } else {
        SetWarpDestination(19, 1, WARP_ID_NONE, 6, 4);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetHiddenItemFlag() {
    FlagSet(gSpecialVar_0x8004);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetWeekCount() -> u16 {
    let mut weekCount: u16 = (gLocalTime.days / 7) as u16;
    if weekCount > 9999 {
        weekCount = 9999;
    }
    return weekCount;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLeadMonFriendshipScore() -> u8 {
    let mut pokemon: *mut Pokemon = &raw mut gPlayerParty[GetLeadMonIndex()];
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
    return FRIENDSHIP_NONE;
}
pub(crate) unsafe extern "C" fn CB2_FieldShowRegionMap() {
    FieldInitRegionMap(Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldShowRegionMap() {
    SetMainCallback2(Some(CB2_FieldShowRegionMap));
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPCTurnOnEffect() {
    if FuncIsActiveTask(Some(Task_PCTurnOnEffect)) != TRUE {
        let mut taskId: u8 = CreateTask(Some(Task_PCTurnOnEffect), 8);
        gTasks[taskId].data[0] = FALSE as i16;
        gTasks[taskId].data[1] = taskId as i16;
        gTasks[taskId].data[2] = 0;
        gTasks[taskId].data[3] = 0;
        gTasks[taskId].data[4] = FALSE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_PCTurnOnEffect(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if (*task).data[0] == 0 {
        PCTurnOnEffect(task);
    }
}
pub(crate) unsafe extern "C" fn PCTurnOnEffect(task: *mut Task) {
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
        PCTurnOnEffect_SetMetatile((*task).data[4], dx, dy);
        DrawWholeMapView();
        (*task).data[4] ^= 1;
        if ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) == 5
        {
            DestroyTask((*task).data[1] as u8);
        }
    }
    (*task).data[3] += 1;
}
pub(crate) unsafe extern "C" fn PCTurnOnEffect_SetMetatile(isScreenOn: i16, dx: i8, dy: i8) {
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
pub unsafe extern "C" fn DoPCTurnOffEffect() {
    PCTurnOffEffect();
}
pub(crate) unsafe extern "C" fn PCTurnOffEffect() {
    let mut dx: i8 = 0;
    let mut dy: i8 = 0;
    let mut metatileId: u16 = 0;
    let mut playerDirection: u8 = GetPlayerFacingDirection();
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
pub unsafe extern "C" fn DoLotteryCornerComputerEffect() {
    if FuncIsActiveTask(Some(Task_LotteryCornerComputerEffect)) != TRUE {
        let mut taskId: u8 = CreateTask(Some(Task_LotteryCornerComputerEffect), 8);
        gTasks[taskId].data[0] = FALSE as i16;
        gTasks[taskId].data[1] = taskId as i16;
        gTasks[taskId].data[2] = 0;
        gTasks[taskId].data[3] = 0;
        gTasks[taskId].data[4] = FALSE as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_LotteryCornerComputerEffect(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if (*task).data[0] == 0 {
        LotteryCornerComputerEffect(task);
    }
}
pub(crate) unsafe extern "C" fn LotteryCornerComputerEffect(task: *mut Task) {
    if (*task).data[3] == 6 {
        (*task).data[3] = 0;
        if (*task).data[4] != 0 {
            MapGridSetMetatileIdAt(18, 8, 3741);
            MapGridSetMetatileIdAt(18, 9, 3749);
        } else {
            MapGridSetMetatileIdAt(18, 8, 3672);
            MapGridSetMetatileIdAt(18, 9, 3680);
        }
        DrawWholeMapView();
        (*task).data[4] ^= 1;
        if ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) == 5
        {
            DestroyTask((*task).data[1] as u8);
        }
    }
    (*task).data[3] += 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EndLotteryCornerComputerEffect() {
    MapGridSetMetatileIdAt(18, 8, 3741);
    MapGridSetMetatileIdAt(18, 9, 3749);
    DrawWholeMapView();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTrickHouseNuggetFlag() {
    let mut specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let mut flag: u16 = FLAG_HIDDEN_ITEM_TRICK_HOUSE_NUGGET;
    *specVar = flag;
    FlagSet(flag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetTrickHouseNuggetFlag() {
    let mut specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let mut flag: u16 = FLAG_HIDDEN_ITEM_TRICK_HOUSE_NUGGET;
    *specVar = flag;
    FlagClear(flag);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonCool() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_COOL) < 200 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonBeauty() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_BEAUTY) < 200 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonCute() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_CUTE) < 200 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonSmart() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_SMART) < 200 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckLeadMonTough() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[GetLeadMonIndex()], MON_DATA_TOUGH) < 200 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsGrassTypeInParty() {
    let mut i: u8 = 0;
    let mut species: u16 = 0;
    let mut pokemon: *mut Pokemon = null_mut();
    i = 0;
    while i < PARTY_SIZE as u8 {
        pokemon = &raw mut gPlayerParty[i];
        if GetMonData2(pokemon, MON_DATA_SANITY_HAS_SPECIES) != 0
            && GetMonData2(pokemon, MON_DATA_IS_EGG) == 0
        {
            species = GetMonData2(pokemon, MON_DATA_SPECIES) as u16;
            if gSpeciesInfo[species].types[0] == TYPE_GRASS
                || gSpeciesInfo[species].types[1] == TYPE_GRASS
            {
                gSpecialVar_Result = TRUE as u16;
                return;
            }
        }
        i += 1;
    }
    gSpecialVar_Result = FALSE as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SpawnCameraObject() {
    let mut obj: u8 = SpawnSpecialObjectEventParameterized(
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
pub unsafe extern "C" fn RemoveCameraObject() {
    CameraObjectSetFollowedSpriteId(GetPlayerAvatarSpriteId());
    RemoveObjectEventByLocalIdAndMap(
        LOCALID_CAMERA,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPokeblockNameByMonNature() -> u8 {
    return CopyMonFavoritePokeblockName(
        GetNature(&raw mut gPlayerParty[GetLeadMonIndex()]),
        gStringVar1.as_mut_ptr(),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSecretBaseNearbyMapName() {
    GetMapName(gStringVar1.as_mut_ptr(), VarGet(VAR_SECRET_BASE_MAP), 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleTowerSinglesStreak() -> u16 {
    return GetGameStat(GAME_STAT_BATTLE_TOWER_SINGLES_STREAK) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferEReaderTrainerName() {
    GetEreaderTrainerName(gStringVar1.as_mut_ptr());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetSlotMachineId() -> u16 {
    let mut rnd: u32 = (*gSaveBlock1Ptr).dewfordTrends[0].trendiness() as u32
        + (*gSaveBlock1Ptr).dewfordTrends[0].rand as u32
        + sSlotMachineRandomSeeds_34[gSpecialVar_0x8004] as u32;
    if IsPokeNewsActive(POKENEWS_GAME_CORNER) != 0 {
        return sSlotMachineServiceDayIds_33[rnd % 12] as u16;
    }
    return sSlotMachineIds_32[rnd % 12] as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom1Key() -> u8 {
    let mut specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let mut flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_1_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom2Key() -> u8 {
    let mut specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let mut flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_2_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom4Key() -> u8 {
    let mut specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let mut flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_4_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundAbandonedShipRoom6Key() -> u8 {
    let mut specVar: *mut u16 = &raw mut gSpecialVar_0x8004;
    let mut flag: u16 = FLAG_HIDDEN_ITEM_ABANDONED_SHIP_RM_6_KEY;
    *specVar = flag;
    if FlagGet(flag) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LeadMonHasEffortRibbon() -> u8 {
    return GetMonData3(
        &raw mut gPlayerParty[GetLeadMonIndex()],
        MON_DATA_EFFORT_RIBBON,
        null_mut(),
    ) as u8;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveLeadMonEffortRibbon() {
    let mut ribbonSet: u8 = 0;
    let mut leadMon: *mut Pokemon = null_mut();
    IncrementGameStat(GAME_STAT_RECEIVED_RIBBONS);
    FlagSet(FLAG_SYS_RIBBON_GET);
    ribbonSet = TRUE;
    leadMon = &raw mut gPlayerParty[GetLeadMonIndex()];
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
pub unsafe extern "C" fn Special_AreLeadMonEVsMaxedOut() -> u8 {
    if GetMonEVCount(&raw mut gPlayerParty[GetLeadMonIndex()]) >= MAX_TOTAL_EVS as u16 {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryUpdateRusturfTunnelState() -> u8 {
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
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetShoalItemFlag(unused: u16) {
    FlagSet(FLAG_SYS_SHOAL_ITEM);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadWallyZigzagoon() {
    let mut monData: u16 = 0;
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
    monData = TRUE as u16;
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
pub unsafe extern "C" fn IsStarterInParty() -> u8 {
    let mut i: u8 = 0;
    let mut starter: u16 = GetStarterPokemon(VarGet(VAR_STARTER_MON));
    let mut partyCount: u8 = CalculatePlayerPartyCount();
    i = 0;
    while i < partyCount {
        if GetMonData3(
            &raw mut gPlayerParty[i],
            MON_DATA_SPECIES_OR_EGG,
            null_mut(),
        ) == starter as u32
        {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptCheckFreePokemonStorageSpace() -> u8 {
    return CheckFreePokemonStorageSpace();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPokerusInParty() -> u8 {
    if CheckPartyPokerus(gPlayerParty.as_mut_ptr(), 63) == 0 {
        return FALSE;
    }
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShakeCamera() {
    let mut taskId: u8 = CreateTask(Some(Task_ShakeCamera), 9);
    gTasks[taskId].data[0] = gSpecialVar_0x8005 as i16;
    gTasks[taskId].data[1] = 0;
    gTasks[taskId].data[2] = gSpecialVar_0x8006 as i16;
    gTasks[taskId].data[3] = gSpecialVar_0x8007 as i16;
    gTasks[taskId].data[4] = gSpecialVar_0x8004 as i16;
    SetCameraPanningCallback(None);
    PlaySE(SE_M_STRENGTH);
}
pub(crate) unsafe extern "C" fn Task_ShakeCamera(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub(crate) unsafe extern "C" fn StopCameraShake(taskId: u8) {
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FoundBlackGlasses() -> u8 {
    return FlagGet(FLAG_HIDDEN_ITEM_ROUTE_116_BLACK_GLASSES);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRoute119Weather() {
    if IsMapTypeOutdoors(GetLastUsedWarpMapType()) != TRUE {
        SetSavedWeather(WEATHER_ROUTE119_CYCLE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetRoute123Weather() {
    if IsMapTypeOutdoors(GetLastUsedWarpMapType()) != TRUE {
        SetSavedWeather(WEATHER_ROUTE123_CYCLE);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetLeadMonIndex() -> u8 {
    let mut i: u8 = 0;
    let mut partyCount: u8 = CalculatePlayerPartyCount();
    i = 0;
    while i < partyCount {
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
        i += 1;
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScriptGetPartyMonSpecies() -> u16 {
    return GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
        MON_DATA_SPECIES_OR_EGG,
        null_mut(),
    ) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryInitBattleTowerAwardManObjectEvent() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetDaysUntilPacifidlogTMAvailable() -> u16 {
    let mut tmReceivedDay: u16 = VarGet(VAR_PACIFIDLOG_TM_RECEIVED_DAY);
    if gLocalTime.days as i32 - tmReceivedDay as i32 >= 7 {
        return 0;
    } else if gLocalTime.days < 0 {
        return 8;
    }
    return 7 - (gLocalTime.days as u16 - tmReceivedDay);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPacifidlogTMReceivedDay() -> u16 {
    VarSet(VAR_PACIFIDLOG_TM_RECEIVED_DAY, gLocalTime.days as u16);
    return gLocalTime.days as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MonOTNameNotPlayer() -> u8 {
    if GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_LANGUAGE)
        != GAME_LANGUAGE as u32
    {
        return TRUE;
    }
    GetMonData3(
        &raw mut gPlayerParty[gSpecialVar_0x8004],
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferLottoTicketNumber() {
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
pub unsafe extern "C" fn GetMysteryGiftCardStat() -> u16 {
    match gSpecialVar_Result {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferTMHMMoveName() -> u8 {
    if gSpecialVar_0x8004 >= ITEM_TM01 && gSpecialVar_0x8004 <= ITEM_HM08 {
        StringCopy(
            gStringVar2.as_mut_ptr(),
            gMoveNames[ItemIdToBattleMoveId(gSpecialVar_0x8004)]
                .as_ptr()
                .cast_mut(),
        );
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBadEggInParty() -> u8 {
    let mut partyCount: u8 = CalculatePlayerPartyCount();
    let mut i: u8 = 0;
    i = 0;
    while i < partyCount {
        if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SANITY_IS_BAD_EGG) == TRUE as u32 {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InMultiPartnerRoom() -> u8 {
    if (*gSaveBlock1Ptr).location.mapGroup == 26
        && (*gSaveBlock1Ptr).location.mapNum == 15
        && VarGet(VAR_FRONTIER_BATTLE_MODE) == FRONTIER_MODE_MULTIS
    {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn OffsetCameraForBattle() {
    SetCameraPanningCallback(None);
    SetCameraPanning(8, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetDeptStoreFloor() {
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
pub unsafe extern "C" fn GetDeptStoreDefaultFloorChoice() -> u16 {
    sLilycoveDeptStore_NeverRead = 0;
    sLilycoveDeptStore_DefaultFloorChoice = 0;
    if (*gSaveBlock1Ptr).dynamicWarp.mapGroup == 13 {
        match (*gSaveBlock1Ptr).dynamicWarp.mapNum {
            20 => {
                sLilycoveDeptStore_NeverRead = 0;
                sLilycoveDeptStore_DefaultFloorChoice = 0;
            }
            19 => {
                sLilycoveDeptStore_NeverRead = 0;
                sLilycoveDeptStore_DefaultFloorChoice = 1;
            }
            18 => {
                sLilycoveDeptStore_NeverRead = 0;
                sLilycoveDeptStore_DefaultFloorChoice = 2;
            }
            17 => {
                sLilycoveDeptStore_NeverRead = 0;
                sLilycoveDeptStore_DefaultFloorChoice = 3;
            }
            16 => {
                sLilycoveDeptStore_NeverRead = 0;
                sLilycoveDeptStore_DefaultFloorChoice = 4;
            }
            _ => {}
        }
    }
    return sLilycoveDeptStore_DefaultFloorChoice;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MoveElevator() {
    let mut data: *mut i16 = gTasks[CreateTask(Some(Task_MoveElevator), 9)]
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
pub(crate) unsafe extern "C" fn Task_MoveElevator(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub unsafe extern "C" fn ShowDeptStoreElevatorFloorSelect() {
    let mut xPos: i32 = 0;
    sTutorMoveAndElevatorWindowId =
        AddWindow((&raw const *sWindowTemplate_ElevatorFloor).cast_mut()) as u8;
    SetStandardWindowBorderStyle(sTutorMoveAndElevatorWindowId, FALSE);
    xPos = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        gText_ElevatorNowOn.as_ptr().cast_mut(),
        64,
    );
    AddTextPrinterParameterized(
        sTutorMoveAndElevatorWindowId,
        FONT_NORMAL,
        gText_ElevatorNowOn.as_ptr().cast_mut(),
        xPos as u8,
        1,
        TEXT_SKIP_DRAW,
        None,
    );
    xPos = GetStringCenterAlignXOffset(
        FONT_NORMAL as i32,
        sDeptStoreFloorNames[gSpecialVar_0x8005],
        64,
    );
    AddTextPrinterParameterized(
        sTutorMoveAndElevatorWindowId,
        FONT_NORMAL,
        sDeptStoreFloorNames[gSpecialVar_0x8005],
        xPos as u8,
        17,
        TEXT_SKIP_DRAW,
        None,
    );
    PutWindowTilemap(sTutorMoveAndElevatorWindowId);
    CopyWindowToVram(sTutorMoveAndElevatorWindowId, COPYWIN_FULL);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseDeptStoreElevatorWindow() {
    ClearStdWindowAndFrameToTransparent(sTutorMoveAndElevatorWindowId, TRUE);
    RemoveWindow(sTutorMoveAndElevatorWindowId);
}
pub(crate) unsafe extern "C" fn MoveElevatorWindowLights(floorDelta: u16, descending: u8) {
    if FuncIsActiveTask(Some(Task_MoveElevatorWindowLights)) != TRUE {
        let mut taskId: u8 = CreateTask(Some(Task_MoveElevatorWindowLights), 8);
        gTasks[taskId].data[0] = 0;
        gTasks[taskId].data[1] = 0;
        gTasks[taskId].data[2] = descending as i16;
        gTasks[taskId].data[3] = sElevatorLightCycles_30[floorDelta] as i16;
    }
}
pub(crate) unsafe extern "C" fn Task_MoveElevatorWindowLights(taskId: u8) {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    if *data.at(1) == 6 {
        *data += 1;
        if *data.at(2) == 0 {
            y = 0;
            while y < ELEVATOR_WINDOW_HEIGHT {
                x = 0;
                while x < ELEVATOR_WINDOW_WIDTH {
                    MapGridSetMetatileIdAt(
                        x as i32 + MAP_OFFSET + 1,
                        y as i32 + MAP_OFFSET,
                        sElevatorWindowTiles_Ascending[y][*data % 3] | MAPGRID_IMPASSABLE,
                    );
                    x += 1;
                }
                y += 1;
            }
        } else {
            y = 0;
            while y < ELEVATOR_WINDOW_HEIGHT {
                x = 0;
                while x < ELEVATOR_WINDOW_WIDTH {
                    MapGridSetMetatileIdAt(
                        x as i32 + MAP_OFFSET + 1,
                        y as i32 + MAP_OFFSET,
                        sElevatorWindowTiles_Descending[y][*data % 3] | MAPGRID_IMPASSABLE,
                    );
                    x += 1;
                }
                y += 1;
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
pub unsafe extern "C" fn BufferVarsForIVRater() {
    let mut i: u8 = 0;
    let mut ivStorage: CArray<u32, 6> = zeroed();
    ivStorage[0] = GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_HP_IV);
    ivStorage[1] = GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_ATK_IV);
    ivStorage[2] = GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_DEF_IV);
    ivStorage[3] = GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_SPEED_IV);
    ivStorage[4] = GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_SPATK_IV);
    ivStorage[5] = GetMonData2(&raw mut gPlayerParty[gSpecialVar_0x8004], MON_DATA_SPDEF_IV);
    gSpecialVar_0x8005 = 0;
    i = 0;
    while i < NUM_STATS as u8 {
        gSpecialVar_0x8005 += ivStorage[i] as u16;
        i += 1;
    }
    gSpecialVar_0x8006 = 0;
    gSpecialVar_0x8007 = ivStorage[0] as u16;
    i = 1;
    while i < NUM_STATS as u8 {
        if ivStorage[gSpecialVar_0x8006] < ivStorage[i] {
            gSpecialVar_0x8006 = i as u16;
            gSpecialVar_0x8007 = ivStorage[i] as u16;
        } else if ivStorage[gSpecialVar_0x8006] == ivStorage[i] {
            let mut randomNumber: u16 = Random();
            if randomNumber as i32 & 1 != 0 {
                gSpecialVar_0x8006 = i as u16;
                gSpecialVar_0x8007 = ivStorage[i] as u16;
            }
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UsedPokemonCenterWarp() -> u8 {
    let mut i: i32 = 0;
    let mut map: u16 = ((gLastUsedWarp.mapGroup as u16) << 8) + gLastUsedWarp.mapNum as u16;
    i = 0;
    while sPokemonCenters_29[i] != MAP_UNDEFINED {
        if sPokemonCenters_29[i] == map {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerNotAtTrainerHillEntrance() -> u32 {
    if (*gSaveBlock1Ptr).location.mapGroup == 26 && (*gSaveBlock1Ptr).location.mapNum == 60 {
        return FALSE as u32;
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFrontierManiac(daysSince: u16) {
    let mut var: *mut u16 = GetVarPointer(VAR_FRONTIER_MANIAC_FACILITY);
    *var += daysSince;
    *var = (*var as i32 % 10) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierManiacMessage() {
    let mut i: u8 = 0;
    let mut winStreak: u16 = 0;
    let mut facility: u16 = VarGet(VAR_FRONTIER_MANIAC_FACILITY);
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
    i = 0;
    while i < 2 && (sFrontierManiacStreakThresholds_28[facility][i] as u16) < winStreak {
        i += 1;
    }
    ShowFieldMessage(sFrontierManiacMessages_27[facility][i]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattleTowerElevatorFloors() {
    let mut i: u8 = 0;
    let mut battleMode: u16 = VarGet(VAR_FRONTIER_BATTLE_MODE);
    let mut lvlMode: u8 = (*gSaveBlock2Ptr).frontier.lvlMode();
    if battleMode == FRONTIER_MODE_MULTIS && FlagGet(FLAG_CHOSEN_MULTI_BATTLE_NPC_PARTNER) == 0 {
        gSpecialVar_0x8005 = 5;
        gSpecialVar_0x8006 = 4;
        return;
    }
    i = 0;
    while i < 9 {
        if sBattleTowerStreakThresholds_26[i]
            > (*gSaveBlock2Ptr).frontier.towerWinStreaks[battleMode][lvlMode]
        {
            gSpecialVar_0x8005 = 4;
            gSpecialVar_0x8006 = i as u16 + 5;
            return;
        }
        i += 1;
    }
    gSpecialVar_0x8005 = 4;
    gSpecialVar_0x8006 = 12;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowScrollableMultichoice() {
    let mut taskId: u8 = CreateTask(Some(Task_ShowScrollableMultichoice), 8);
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[11] = gSpecialVar_0x8004 as i16;
    match gSpecialVar_0x8004 {
        SCROLL_MULTI_NONE => {
            (*task).data[0] = 1;
            (*task).data[1] = 1;
            (*task).data[2] = 1;
            (*task).data[3] = 1;
            (*task).data[4] = 1;
            (*task).data[5] = 1;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_GLASS_WORKSHOP_VENDOR => {
            (*task).data[0] = 5;
            (*task).data[1] = 8;
            (*task).data[2] = 1;
            (*task).data[3] = 1;
            (*task).data[4] = 9;
            (*task).data[5] = 10;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_POKEMON_FAN_CLUB_RATER => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 12;
            (*task).data[2] = 1;
            (*task).data[3] = 1;
            (*task).data[4] = 7;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1 => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 11;
            (*task).data[2] = 14;
            (*task).data[3] = 1;
            (*task).data[4] = 15;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_2 => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 6;
            (*task).data[2] = 14;
            (*task).data[3] = 1;
            (*task).data[4] = 15;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_VITAMIN_VENDOR => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 7;
            (*task).data[2] = 14;
            (*task).data[3] = 1;
            (*task).data[4] = 15;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 10;
            (*task).data[2] = 14;
            (*task).data[3] = 1;
            (*task).data[4] = 15;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BERRY_POWDER_VENDOR => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 12;
            (*task).data[2] = 15;
            (*task).data[3] = 1;
            (*task).data[4] = 14;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BF_RECEPTIONIST => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 10;
            (*task).data[2] = 17;
            (*task).data[3] = 1;
            (*task).data[4] = 11;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        9 | 10 => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 11;
            (*task).data[2] = 15;
            (*task).data[3] = 1;
            (*task).data[4] = 14;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_SS_TIDAL_DESTINATION => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 7;
            (*task).data[2] = 19;
            (*task).data[3] = 1;
            (*task).data[4] = 10;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        SCROLL_MULTI_BATTLE_TENT_RULES => {
            (*task).data[0] = MAX_SCROLL_MULTI_ON_SCREEN;
            (*task).data[1] = 7;
            (*task).data[2] = 17;
            (*task).data[3] = 1;
            (*task).data[4] = 12;
            (*task).data[5] = 12;
            (*task).data[6] = FALSE as i16;
            (*task).data[15] = taskId as i16;
        }
        _ => {
            gSpecialVar_Result = MULTI_B_PRESSED;
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn Task_ShowScrollableMultichoice(taskId: u8) {
    let mut width: u32 = 0;
    let mut i: u8 = 0;
    let mut windowId: u8 = 0;
    let mut template: WindowTemplate = zeroed();
    let mut task: *mut Task = &raw mut gTasks[taskId];
    LockPlayerFieldControls();
    sScrollableMultichoice_ScrollOffset = 0;
    sScrollableMultichoice_ItemSpriteId = MAX_SPRITES;
    FillFrontierExchangeCornerWindowAndItemIcon((*task).data[11] as u16, 0);
    ShowBattleFrontierTutorWindow((*task).data[11] as u8, 0);
    sScrollableMultichoice_ListMenuItem =
        AllocZeroed((*task).data[1] as u32 * 8) as *mut ListMenuItem;
    sFrontierExchangeCorner_NeverRead = 0;
    InitScrollableMultichoice();
    width = 0;
    i = 0;
    while (i as i16) < (*task).data[1] {
        let mut text: *mut u8 = sScrollableMultichoiceOptions[gSpecialVar_0x8004][i];
        (*sScrollableMultichoice_ListMenuItem.at(i)).name = text;
        (*sScrollableMultichoice_ListMenuItem.at(i)).id = i as i32;
        width = DisplayTextAndGetWidth(text, width as i32) as u32;
        i += 1;
    }
    (*task).data[4] = ConvertPixelWidthToTileWidth(width as i32) as i16;
    if (*task).data[2] as i32 + (*task).data[4] as i32 > 29 {
        let mut adjustedLeft: i32 = 29 - (*task).data[4] as i32;
        if adjustedLeft < 0 {
            (*task).data[2] = 0;
        } else {
            (*task).data[2] = adjustedLeft as i16;
        }
    }
    template = CreateWindowTemplate(
        0,
        (*task).data[2] as u8,
        (*task).data[3] as u8,
        (*task).data[4] as u8,
        (*task).data[5] as u8,
        0xF,
        0x64,
    );
    windowId = AddWindow(&raw mut template) as u8;
    (*task).data[13] = windowId as i16;
    SetStandardWindowBorderStyle(windowId, FALSE);
    gScrollableMultichoice_ListMenuTemplate.totalItems = (*task).data[1] as u16;
    gScrollableMultichoice_ListMenuTemplate.maxShowed = (*task).data[0] as u16;
    gScrollableMultichoice_ListMenuTemplate.windowId = (*task).data[13] as u8;
    ScrollableMultichoice_UpdateScrollArrows(taskId);
    (*task).data[14] = ListMenuInit(
        &raw mut gScrollableMultichoice_ListMenuTemplate,
        (*task).data[7] as u16,
        (*task).data[8] as u16,
    ) as i16;
    ScheduleBgCopyTilemapToVram(0);
    gTasks[taskId].func = Some(ScrollableMultichoice_ProcessInput);
}
pub(crate) unsafe extern "C" fn InitScrollableMultichoice() {
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
pub(crate) unsafe extern "C" fn ScrollableMultichoice_MoveCursor(
    itemIndex: i32,
    onInit: u8,
    list: *mut ListMenu,
) {
    let mut taskId: u8 = 0;
    PlaySE(SE_SELECT);
    taskId = FindTaskIdByFunc(Some(ScrollableMultichoice_ProcessInput));
    if taskId != TASK_NONE {
        let mut selection: u16 = 0;
        let mut task: *mut Task = &raw mut gTasks[taskId];
        ListMenuGetScrollAndRow((*task).data[14] as u8, &raw mut selection, null_mut());
        sScrollableMultichoice_ScrollOffset = selection;
        ListMenuGetCurrentItemArrayId((*task).data[14] as u8, &raw mut selection);
        HideFrontierExchangeCornerItemIcon(
            (*task).data[11] as u16,
            sFrontierExchangeCorner_NeverRead,
        );
        FillFrontierExchangeCornerWindowAndItemIcon((*task).data[11] as u16, selection);
        ShowBattleFrontierTutorMoveDescription((*task).data[11] as u8, selection);
        sFrontierExchangeCorner_NeverRead = selection;
    }
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_ProcessInput(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut input: i32 = ListMenu_ProcessInput((*task).data[14] as u8);
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
            if (*task).data[6] == 0 {
                CloseScrollableMultichoice(taskId);
            } else if input == (*task).data[1] as i32 - 1 {
                CloseScrollableMultichoice(taskId);
            } else {
                ScrollableMultichoice_RemoveScrollArrows(taskId);
                (*task).func = Some(Task_ScrollableMultichoice_WaitReturnToList);
                ScriptContext_Enable();
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CloseScrollableMultichoice(taskId: u8) {
    let mut selection: u16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    ListMenuGetCurrentItemArrayId((*task).data[14] as u8, &raw mut selection);
    HideFrontierExchangeCornerItemIcon((*task).data[11] as u16, selection);
    ScrollableMultichoice_RemoveScrollArrows(taskId);
    DestroyListMenuTask((*task).data[14] as u8, null_mut(), null_mut());
    Free(sScrollableMultichoice_ListMenuItem as *mut c_void);
    ClearStdWindowAndFrameToTransparent((*task).data[13] as u8, TRUE);
    FillWindowPixelBuffer((*task).data[13] as u8, 0);
    CopyWindowToVram((*task).data[13] as u8, COPYWIN_GFX);
    RemoveWindow((*task).data[13] as u8);
    DestroyTask(taskId);
    ScriptContext_Enable();
}
pub(crate) unsafe extern "C" fn Task_ScrollableMultichoice_WaitReturnToList(taskId: u8) {
    match gTasks[taskId].data[6] {
        2 => {
            gTasks[taskId].data[6] = 1;
            gTasks[taskId].func = Some(Task_ScrollableMultichoice_ReturnToList);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollableMultichoice_TryReturnToList() {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
    if taskId == TASK_NONE {
        ScriptContext_Enable();
    } else {
        gTasks[taskId].data[6] += 1;
    }
}
pub(crate) unsafe extern "C" fn Task_ScrollableMultichoice_ReturnToList(taskId: u8) {
    LockPlayerFieldControls();
    ScrollableMultichoice_UpdateScrollArrows(taskId);
    gTasks[taskId].func = Some(ScrollableMultichoice_ProcessInput);
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_UpdateScrollArrows(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut template: ScrollArrowsTemplate = zeroed();
    template = *sScrollableMultichoice_ScrollArrowsTemplate_25;
    if (*task).data[0] != (*task).data[1] {
        template.firstX = ((*task).data[4] / 2) as u8 * 8 + 12 + ((*task).data[2] as u8 - 1) * 8;
        template.firstY = 8;
        template.secondX = ((*task).data[4] / 2) as u8 * 8 + 12 + ((*task).data[2] as u8 - 1) * 8;
        template.secondY = (*task).data[5] as u8 * 8 + 10;
        template.fullyUpThreshold = 0;
        template.fullyDownThreshold = (*task).data[1] as u16 - (*task).data[0] as u16;
        (*task).data[12] = AddScrollIndicatorArrowPair(
            &raw mut template,
            &raw mut sScrollableMultichoice_ScrollOffset,
        ) as i16;
    }
}
pub(crate) unsafe extern "C" fn ScrollableMultichoice_RemoveScrollArrows(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if (*task).data[0] != (*task).data[1] {
        RemoveScrollIndicatorArrowPair((*task).data[12] as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowGlassWorkshopMenu() {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBattleTowerLinkPlayerGfx() {
    let mut i: u8 = 0;
    i = 0;
    while i < 2 {
        if gLinkPlayers[i].gender == MALE {
            VarSet(VAR_OBJ_GFX_ID_F - i as u16, OBJ_EVENT_GFX_BRENDAN_NORMAL);
        } else {
            VarSet(
                VAR_OBJ_GFX_ID_F - i as u16,
                OBJ_EVENT_GFX_RIVAL_MAY_NORMAL as u16,
            );
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowNatureGirlMessage() {
    let mut nature: u8 = 0;
    if gSpecialVar_0x8004 >= PARTY_SIZE as u16 {
        gSpecialVar_0x8004 = 0;
    }
    nature = GetNature(&raw mut gPlayerParty[gSpecialVar_0x8004]);
    ShowFieldMessage(sNatureGirlMessages_24[nature]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFrontierGambler(daysSince: u16) {
    let mut var: *mut u16 = GetVarPointer(VAR_FRONTIER_GAMBLER_CHALLENGE);
    *var += daysSince;
    *var = (*var as i32 % 12) as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierGamblerLookingMessage() {
    let mut challenge: u16 = VarGet(VAR_FRONTIER_GAMBLER_CHALLENGE);
    ShowFieldMessage(sFrontierGamblerLookingMessages_23[challenge]);
    VarSet(VAR_FRONTIER_GAMBLER_SET_CHALLENGE, challenge);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierGamblerGoMessage() {
    ShowFieldMessage(sFrontierGamblerGoMessages_22[VarGet(VAR_FRONTIER_GAMBLER_SET_CHALLENGE)]);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FrontierGamblerSetWonOrLost(won: u8) {
    let mut battleMode: u16 = VarGet(VAR_FRONTIER_BATTLE_MODE);
    let mut challenge: u16 = VarGet(VAR_FRONTIER_GAMBLER_SET_CHALLENGE);
    let mut frontierFacilityId: u16 = VarGet(VAR_FRONTIER_FACILITY);
    if VarGet(VAR_FRONTIER_GAMBLER_STATE) == FRONTIER_GAMBLER_PLACED_BET {
        if sFrontierChallenges_21[challenge] as i32
            == ((frontierFacilityId as i32) << 8) + battleMode as i32
        {
            if won != 0 {
                VarSet(VAR_FRONTIER_GAMBLER_STATE, FRONTIER_GAMBLER_WON);
            } else {
                VarSet(VAR_FRONTIER_GAMBLER_STATE, FRONTIER_GAMBLER_LOST);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateBattlePointsWindow() {
    let mut string: CArray<u8, 32> = zeroed();
    let mut x: u32 = 0;
    StringCopy(
        ConvertIntToDecimalStringN(
            string.as_mut_ptr(),
            (*gSaveBlock2Ptr).frontier.battlePoints as i32,
            STR_CONV_MODE_RIGHT_ALIGN,
            4,
        ),
        gText_BP.as_ptr().cast_mut(),
    );
    x = GetStringRightAlignXOffset(FONT_NORMAL as i32, string.as_mut_ptr(), 48) as u32;
    AddTextPrinterParameterized(
        sBattlePointsWindowId,
        FONT_NORMAL,
        string.as_mut_ptr(),
        x as u8,
        1,
        0,
        None,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowBattlePointsWindow() {
    sBattlePointsWindowId =
        AddWindow((&raw const *sBattlePoints_WindowTemplate_20).cast_mut()) as u8;
    SetStandardWindowBorderStyle(sBattlePointsWindowId, FALSE);
    UpdateBattlePointsWindow();
    CopyWindowToVram(sBattlePointsWindowId, COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseBattlePointsWindow() {
    ClearStdWindowAndFrameToTransparent(sBattlePointsWindowId, TRUE);
    RemoveWindow(sBattlePointsWindowId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TakeFrontierBattlePoints() {
    if (*gSaveBlock2Ptr).frontier.battlePoints < gSpecialVar_0x8004 {
        (*gSaveBlock2Ptr).frontier.battlePoints = 0;
    } else {
        (*gSaveBlock2Ptr).frontier.battlePoints -= gSpecialVar_0x8004;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GiveFrontierBattlePoints() {
    if (*gSaveBlock2Ptr).frontier.battlePoints as i32 + gSpecialVar_0x8004 as i32
        > MAX_BATTLE_FRONTIER_POINTS as i32
    {
        (*gSaveBlock2Ptr).frontier.battlePoints = MAX_BATTLE_FRONTIER_POINTS;
    } else {
        (*gSaveBlock2Ptr).frontier.battlePoints =
            (*gSaveBlock2Ptr).frontier.battlePoints + gSpecialVar_0x8004;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFrontierBattlePoints() -> u16 {
    return (*gSaveBlock2Ptr).frontier.battlePoints;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShowFrontierExchangeCornerItemIconWindow() {
    sFrontierExchangeCorner_ItemIconWindowId =
        AddWindow((&raw const *sFrontierExchangeCorner_ItemIconWindowTemplate_19).cast_mut()) as u8;
    SetStandardWindowBorderStyle(sFrontierExchangeCorner_ItemIconWindowId, FALSE);
    CopyWindowToVram(sFrontierExchangeCorner_ItemIconWindowId, COPYWIN_GFX);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CloseFrontierExchangeCornerItemIconWindow() {
    ClearStdWindowAndFrameToTransparent(sFrontierExchangeCorner_ItemIconWindowId, TRUE);
    RemoveWindow(sFrontierExchangeCorner_ItemIconWindowId);
}
pub(crate) unsafe extern "C" fn FillFrontierExchangeCornerWindowAndItemIcon(
    menu: u16,
    selection: u16,
) {
    if menu >= SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1
        && menu <= SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR
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
                    sScrollableMultichoice_ItemSpriteId = AddDecorationIconObject(
                        sFrontierExchangeCorner_Decor1_17[selection] as u8,
                        33,
                        88,
                        0,
                        TAG_ITEM_ICON,
                        TAG_ITEM_ICON,
                    );
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
                    sScrollableMultichoice_ItemSpriteId = AddDecorationIconObject(
                        sFrontierExchangeCorner_Decor2_15[selection] as u8,
                        33,
                        88,
                        0,
                        TAG_ITEM_ICON,
                        TAG_ITEM_ICON,
                    );
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
pub(crate) unsafe extern "C" fn ShowFrontierExchangeCornerItemIcon(item: u16) {
    FreeSpriteTilesByTag(TAG_ITEM_ICON);
    FreeSpritePaletteByTag(TAG_ITEM_ICON);
    sScrollableMultichoice_ItemSpriteId = AddItemIconSprite(TAG_ITEM_ICON, TAG_ITEM_ICON, item);
    if sScrollableMultichoice_ItemSpriteId != MAX_SPRITES {
        gSprites[sScrollableMultichoice_ItemSpriteId]
            .oam
            .set_priority(0);
        gSprites[sScrollableMultichoice_ItemSpriteId].x = 36;
        gSprites[sScrollableMultichoice_ItemSpriteId].y = 92;
    }
}
pub(crate) unsafe extern "C" fn HideFrontierExchangeCornerItemIcon(menu: u16, unused: u16) {
    if sScrollableMultichoice_ItemSpriteId != MAX_SPRITES {
        match menu {
            SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_1
            | SCROLL_MULTI_BF_EXCHANGE_CORNER_DECOR_VENDOR_2
            | SCROLL_MULTI_BF_EXCHANGE_CORNER_VITAMIN_VENDOR
            | SCROLL_MULTI_BF_EXCHANGE_CORNER_HOLD_ITEM_VENDOR => {
                DestroySpriteAndFreeResources(
                    &raw mut gSprites[sScrollableMultichoice_ItemSpriteId],
                );
            }
            _ => {}
        }
        sScrollableMultichoice_ItemSpriteId = MAX_SPRITES;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferBattleFrontierTutorMoveName() {
    if gSpecialVar_0x8005 != 0 {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gMoveNames[sBattleFrontier_TutorMoves2[gSpecialVar_0x8004]]
                .as_ptr()
                .cast_mut(),
        );
    } else {
        StringCopy(
            gStringVar1.as_mut_ptr(),
            gMoveNames[sBattleFrontier_TutorMoves1[gSpecialVar_0x8004]]
                .as_ptr()
                .cast_mut(),
        );
    }
}
pub(crate) unsafe extern "C" fn ShowBattleFrontierTutorWindow(menu: u8, selection: u16) {
    if menu == SCROLL_MULTI_BF_MOVE_TUTOR_1 || menu == SCROLL_MULTI_BF_MOVE_TUTOR_2 {
        if gSpecialVar_0x8006 == 0 {
            sTutorMoveAndElevatorWindowId =
                AddWindow((&raw const *sBattleFrontierTutor_WindowTemplate_10).cast_mut()) as u8;
            SetStandardWindowBorderStyle(sTutorMoveAndElevatorWindowId, FALSE);
        }
        ShowBattleFrontierTutorMoveDescription(menu, selection);
    }
}
pub(crate) unsafe extern "C" fn ShowBattleFrontierTutorMoveDescription(menu: u8, selection: u16) {
    if menu == SCROLL_MULTI_BF_MOVE_TUTOR_1 || menu == SCROLL_MULTI_BF_MOVE_TUTOR_2 {
        FillWindowPixelRect(sTutorMoveAndElevatorWindowId, 17, 0, 0, 96, 48);
        if menu == SCROLL_MULTI_BF_MOVE_TUTOR_2 {
            AddTextPrinterParameterized(
                sTutorMoveAndElevatorWindowId,
                FONT_NORMAL,
                sBattleFrontier_TutorMoveDescriptions2_9[selection],
                0,
                1,
                0,
                None,
            );
        } else {
            AddTextPrinterParameterized(
                sTutorMoveAndElevatorWindowId,
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
pub unsafe extern "C" fn CloseBattleFrontierTutorWindow() {
    ClearStdWindowAndFrameToTransparent(sTutorMoveAndElevatorWindowId, TRUE);
    RemoveWindow(sTutorMoveAndElevatorWindowId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScrollableMultichoice_RedrawPersistentMenu() {
    let mut scrollOffset: u16 = 0;
    let mut selectedRow: u16 = 0;
    let mut i: u8 = 0;
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
    if taskId != TASK_NONE {
        let mut task: *mut Task = &raw mut gTasks[taskId];
        ListMenuGetScrollAndRow(
            (*task).data[14] as u8,
            &raw mut scrollOffset,
            &raw mut selectedRow,
        );
        SetStandardWindowBorderStyle((*task).data[13] as u8, FALSE);
        i = 0;
        while i < MAX_SCROLL_MULTI_ON_SCREEN as u8 {
            AddTextPrinterParameterized5(
                (*task).data[13] as u8,
                FONT_NORMAL,
                sScrollableMultichoiceOptions[gSpecialVar_0x8004][scrollOffset as i32 + i as i32],
                10,
                i * 16,
                TEXT_SKIP_DRAW,
                None,
                0,
                0,
            );
            i += 1;
        }
        AddTextPrinterParameterized(
            (*task).data[13] as u8,
            FONT_NORMAL,
            gText_SelectorArrow.as_ptr().cast_mut(),
            0,
            selectedRow as u8 * 16,
            TEXT_SKIP_DRAW,
            None,
        );
        PutWindowTilemap((*task).data[13] as u8);
        CopyWindowToVram((*task).data[13] as u8, COPYWIN_FULL);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetBattleFrontierTutorMoveIndex() {
    let mut i: u8 = 0;
    let mut moveTutor: u16 = 0;
    let mut moveIndex: u16 = 0;
    gSpecialVar_0x8005 = 0;
    moveTutor = VarGet(VAR_TEMP_FRONTIER_TUTOR_ID);
    moveIndex = VarGet(VAR_TEMP_FRONTIER_TUTOR_SELECTION);
    if moveTutor != 0 {
        i = 0;
        loop {
            if gTutorMoves[i] == sBattleFrontier_TutorMoves2[moveIndex] {
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
            if gTutorMoves[i] == sBattleFrontier_TutorMoves1[moveIndex] {
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
pub unsafe extern "C" fn ScrollableMultichoice_ClosePersistentMenu() {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_ScrollableMultichoice_WaitReturnToList));
    if taskId != TASK_NONE {
        let mut task: *mut Task = &raw mut gTasks[taskId];
        DestroyListMenuTask((*task).data[14] as u8, null_mut(), null_mut());
        Free(sScrollableMultichoice_ListMenuItem as *mut c_void);
        ClearStdWindowAndFrameToTransparent((*task).data[13] as u8, TRUE);
        FillWindowPixelBuffer((*task).data[13] as u8, 0);
        ClearWindowTilemap((*task).data[13] as u8);
        CopyWindowToVram((*task).data[13] as u8, COPYWIN_GFX);
        RemoveWindow((*task).data[13] as u8);
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoDeoxysRockInteraction() {
    CreateTask(Some(Task_DeoxysRockInteraction), 8);
}
pub(crate) unsafe extern "C" fn Task_DeoxysRockInteraction(taskId: u8) {
    if FlagGet(FLAG_DEOXYS_ROCK_COMPLETE) == TRUE {
        gSpecialVar_Result = DEOXYS_ROCK_COMPLETE;
        ScriptContext_Enable();
        DestroyTask(taskId);
    } else {
        let mut rockLevel: u16 = VarGet(VAR_DEOXYS_ROCK_LEVEL);
        let mut stepCount: u16 = VarGet(VAR_DEOXYS_ROCK_STEP_COUNT);
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
pub(crate) unsafe extern "C" fn ChangeDeoxysRockLevel(rockLevel: u8) {
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
pub(crate) unsafe extern "C" fn WaitForDeoxysRockMovement(taskId: u8) {
    if FieldEffectActiveListContains(FLDEFF_MOVE_DEOXYS_ROCK) == FALSE {
        ScriptContext_Enable();
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IncrementBirthIslandRockStepCount() {
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
pub unsafe extern "C" fn SetDeoxysRockPalette() {
    LoadPalette(
        (&raw const sDeoxysRockPalettes[VarGet(VAR_DEOXYS_ROCK_LEVEL) as u8]).cast_mut()
            as *mut c_void,
        416,
        8,
    );
    BlendPalettes(0x4000000, 16, 0);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPCBoxToSendMon(boxId: u8) {
    sPCBoxToSendMon = boxId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPCBoxToSendMon() -> u16 {
    return sPCBoxToSendMon as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldShowBoxWasFullMessage() -> u8 {
    if FlagGet(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE) == 0 {
        if StorageGetCurrentBox() as u16 != VarGet(VAR_PC_BOX_TO_SEND_MON) {
            FlagSet(FLAG_SHOWN_BOX_WAS_FULL_MESSAGE);
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsDestinationBoxFull() -> u8 {
    let mut r#box: i32 = 0;
    let mut i: i32 = 0;
    SetPCBoxToSendMon(VarGet(VAR_PC_BOX_TO_SEND_MON) as u8);
    r#box = StorageGetCurrentBox() as i32;
    loop {
        i = 0;
        while i < IN_BOX_COUNT {
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
            i += 1;
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
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateAbnormalWeatherEvent() {
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
pub unsafe extern "C" fn GetAbnormalWeatherMapNameAndType() -> u32 {
    let mut abnormalWeather: u16 = VarGet(VAR_ABNORMAL_WEATHER_LOCATION);
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AbnormalWeatherHasExpired() -> u8 {
    let mut steps: u16 = VarGet(VAR_ABNORMAL_WEATHER_STEP_COUNTER);
    let mut abnormalWeather: u16 = VarGet(VAR_ABNORMAL_WEATHER_LOCATION);
    if abnormalWeather == ABNORMAL_WEATHER_NONE {
        return FALSE;
    }
    if ({
        steps += 1;
        steps
    }) > 999
    {
        VarSet(VAR_ABNORMAL_WEATHER_STEP_COUNTER, 0);
        if (*gSaveBlock1Ptr).location.mapGroup == 24 {
            match (*gSaveBlock1Ptr).location.mapNum {
                101 | 102 | 103 | 104 | 105 => {
                    VarSet(VAR_SHOULD_END_ABNORMAL_WEATHER, 1);
                    return FALSE;
                }
                _ => {}
            }
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unused_SetWeatherSunny() {
    SetCurrentAndNextWeather(WEATHER_SUNNY);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMartEmployeeObjectEventId() -> u32 {
    let mut i: u8 = 0;
    i = 0;
    while i < 12 {
        if (*gSaveBlock1Ptr).location.mapGroup as i32 == sPokeMarts_4[i][0] as i32 {
            if (*gSaveBlock1Ptr).location.mapNum as i32 == sPokeMarts_4[i][1] as i32 {
                return sPokeMarts_4[i][2] as u32;
            }
        }
        i += 1;
    }
    return 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsTrainerRegistered() -> u32 {
    let mut index: i32 = GetRematchIdxByTrainerIdx(gSpecialVar_0x8004 as i32);
    if index >= 0 {
        if FlagGet(TRAINER_REGISTERED_FLAGS_START + index as u16) == TRUE {
            return TRUE as u32;
        }
    }
    return FALSE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldDistributeEonTicket() -> u32 {
    if VarGet(VAR_DISTRIBUTE_EON_TICKET) == 0 {
        return FALSE as u32;
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BattleTowerReconnectLink() {
    sBattleTowerMultiBattleTypeFlags = gBattleTypeFlags;
    gBattleTypeFlags = 0;
    if gReceivedRemoteLinkPlayers == 0 {
        CreateTask(Some(Task_ReconnectWithLinkPlayers), 5);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LinkRetireStatusWithBattleTowerPartner() {
    CreateTask(Some(Task_LinkRetireStatusWithBattleTowerPartner), 5);
}
pub(crate) unsafe extern "C" fn Task_LinkRetireStatusWithBattleTowerPartner(taskId: u8) {
    match gTasks[taskId].data[0] {
        0 => {
            if FuncIsActiveTask(Some(Task_ReconnectWithLinkPlayers)) == 0 {
                gTasks[taskId].data[0] += 1;
            }
        }
        1 => {
            if IsLinkTaskFinished() == TRUE {
                if GetMultiplayerId() == 0 {
                    gTasks[taskId].data[0] += 1;
                } else {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        &raw mut gSpecialVar_0x8004 as *mut c_void,
                        2,
                    );
                    gTasks[taskId].data[0] += 1;
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
                gTasks[taskId].data[0] += 1;
            }
        }
        3 => {
            if IsLinkTaskFinished() == TRUE {
                if GetMultiplayerId() != 0 {
                    gTasks[taskId].data[0] += 1;
                } else {
                    SendBlock(
                        BitmaskAllOtherLinkPlayers(),
                        &raw mut gSpecialVar_Result as *mut c_void,
                        2,
                    );
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        4 => {
            if GetBlockReceivedStatus() as i32 & 1 != 0 {
                if GetMultiplayerId() != 0 {
                    gSpecialVar_Result = gBlockRecvBuffer[0][0];
                    ResetBlockReceivedFlag(0);
                    gTasks[taskId].data[0] += 1;
                } else {
                    gTasks[taskId].data[0] += 1;
                }
            }
        }
        5 => {
            if GetMultiplayerId() == 0 {
                if gSpecialVar_Result == BATTLE_TOWER_LINKSTAT_MEMBER_RETIRE {
                    ShowFieldAutoScrollMessage(gText_YourPartnerHasRetired.as_ptr().cast_mut());
                }
            } else {
                if gSpecialVar_Result == BATTLE_TOWER_LINKSTAT_LEADER_RETIRE {
                    ShowFieldAutoScrollMessage(gText_YourPartnerHasRetired.as_ptr().cast_mut());
                }
            }
            gTasks[taskId].data[0] += 1;
        }
        6 => {
            if IsTextPrinterActive(0) == 0 {
                gTasks[taskId].data[0] += 1;
            }
        }
        7 => {
            if IsLinkTaskFinished() == TRUE {
                SetLinkStandbyCallback();
                gTasks[taskId].data[0] += 1;
            }
        }
        8 => {
            if IsLinkTaskFinished() == TRUE {
                gTasks[taskId].data[0] += 1;
            }
        }
        9 => {
            if gWirelessCommType == 0 {
                SetCloseLinkCallback();
            }
            gBattleTypeFlags = sBattleTowerMultiBattleTypeFlags;
            ScriptContext_Enable();
            DestroyTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_DoRayquazaScene() {
    if gSpecialVar_0x8004 == 0 {
        DoRayquazaScene(0, TRUE, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    } else {
        DoRayquazaScene(1, FALSE, Some(CB2_ReturnToFieldContinueScriptPlayMapMusic));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoopWingFlapSE() {
    CreateTask(Some(Task_LoopWingFlapSE), 8);
    PlaySE(SE_M_WING_ATTACK);
}
pub(crate) unsafe extern "C" fn Task_LoopWingFlapSE(taskId: u8) {
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
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
pub unsafe extern "C" fn CloseBattlePikeCurtain() {
    let mut taskId: u8 = CreateTask(Some(Task_CloseBattlePikeCurtain), 8);
    gTasks[taskId].data[0] = 4;
    gTasks[taskId].data[1] = 4;
    gTasks[taskId].data[2] = 4;
    gTasks[taskId].data[3] = 0;
}
pub(crate) unsafe extern "C" fn Task_CloseBattlePikeCurtain(taskId: u8) {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut data: *mut i16 = gTasks[taskId].data.as_mut_ptr();
    *data.at(*data.at(3)) -= 1;
    if *data.at(*data.at(3)) == 0 {
        y = 0;
        while y < CURTAIN_HEIGHT {
            x = 0;
            while x < CURTAIN_WIDTH {
                MapGridSetMetatileIdAt(
                    (*gSaveBlock1Ptr).pos.x as i32 + x as i32 + MAP_OFFSET - 1,
                    (*gSaveBlock1Ptr).pos.y as i32 + y as i32 + MAP_OFFSET - 3,
                    x as u16
                        + METATILE_BattlePike_CurtainFrames_Start
                        + y as u16 * METATILE_ROW_WIDTH
                        + *data.at(3) as u16 * CURTAIN_HEIGHT as u16 * METATILE_ROW_WIDTH,
                );
                x += 1;
            }
            y += 1;
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
pub unsafe extern "C" fn GetBattlePyramidHint() {
    gSpecialVar_Result = (gSpecialVar_0x8004 as i32 / 7) as u16;
    gSpecialVar_Result -= (gSpecialVar_Result as i32 / 20) as u16 * TOTAL_PYRAMID_ROUNDS;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetHealLocationFromDewford() {
    if (*gSaveBlock1Ptr).lastHealLocation.mapGroup == 0
        && (*gSaveBlock1Ptr).lastHealLocation.mapNum == 11
    {
        SetLastHealLocationWarp(HEAL_LOCATION_PETALBURG_CITY);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InPokemonCenter() -> u8 {
    let mut i: i32 = 0;
    let mut map: u16 = (((*gSaveBlock1Ptr).location.mapGroup as u16) << 8)
        + (*gSaveBlock1Ptr).location.mapNum as u16;
    i = 0;
    while sPokemonCenters_3[i] != MAP_UNDEFINED {
        if sPokemonCenters_3[i] == map {
            return TRUE;
        }
        i += 1;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ResetFanClub() {
    (*gSaveBlock1Ptr).vars[65] = 0;
    (*gSaveBlock1Ptr).vars[66] = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoseFansFromPlayTimeAfterLinkBattle() {
    if DidPlayerGetFirstFans() != 0 {
        TryLoseFansFromPlayTime();
        (*gSaveBlock1Ptr).vars[66] = (*gSaveBlock2Ptr).playTimeHours;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTrainerFanClubGameClear() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryGainNewFanFromCounter(incrementId: u8) -> u8 {
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
    return (*gSaveBlock1Ptr).vars[65] as u8 & 0x007F;
}
pub(crate) unsafe extern "C" fn PlayerGainRandomTrainerFan() -> u16 {
    let mut i: u8 = 0;
    let mut idx: u8 = 0;
    i = 0;
    while i < 8 {
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
        i += 1;
    }
    (*gSaveBlock1Ptr).vars[65] |= shl_i32(1, sFanClubMemberIds_1[idx] as u32) as u16;
    return idx as u16;
}
pub(crate) unsafe extern "C" fn PlayerLoseRandomTrainerFan() -> u16 {
    let mut i: u8 = 0;
    let mut idx: u8 = 0;
    if GetNumFansOfPlayerInTrainerFanClub() == 1 {
        return 0;
    }
    i = 0;
    while i < 8 {
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
        i += 1;
    }
    if shr_i32(
        (*gSaveBlock1Ptr).vars[65] as i32,
        sFanClubMemberIds_0[idx] as u32,
    ) & 1
        != 0
    {
        (*gSaveBlock1Ptr).vars[65] ^= shl_i32(1, sFanClubMemberIds_0[idx] as u32) as u16;
    }
    return idx as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetNumFansOfPlayerInTrainerFanClub() -> u16 {
    let mut i: u8 = 0;
    let mut numFans: u8 = 0;
    i = 0;
    while i < NUM_TRAINER_FAN_CLUB_MEMBERS {
        if shr_i32(
            (*gSaveBlock1Ptr).vars[65] as i32,
            i as u32 + FANCLUB_MEMBER1,
        ) & 1
            != 0
        {
            numFans += 1;
        }
        i += 1;
    }
    return numFans as u16;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryLoseFansFromPlayTime() {
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
pub unsafe extern "C" fn IsFanClubMemberFanOfPlayer() -> u8 {
    return shr_i32((*gSaveBlock1Ptr).vars[65] as i32, gSpecialVar_0x8004 as u32) as u8 & 1;
}
pub(crate) unsafe extern "C" fn SetInitialFansOfPlayer() {
    (*gSaveBlock1Ptr).vars[65] |= 8192;
    (*gSaveBlock1Ptr).vars[65] |= 256;
    (*gSaveBlock1Ptr).vars[65] |= 1024;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BufferFanClubTrainerName() {
    let mut whichLinkTrainer: u8 = 0;
    let mut whichNPCTrainer: u8 = 0;
    match gSpecialVar_0x8004 {
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
pub(crate) unsafe extern "C" fn BufferFanClubTrainerName_(
    linkRecords: *mut LinkBattleRecords,
    whichLinkTrainer: u8,
    whichNPCTrainer: u8,
) {
    let mut record: *mut LinkBattleRecord = &raw mut (*linkRecords).entries[whichLinkTrainer];
    if (*record).name[0] == EOS {
        match whichNPCTrainer {
            0 => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Wallace.as_ptr().cast_mut());
            }
            1 => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Steven.as_ptr().cast_mut());
            }
            2 => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Brawly.as_ptr().cast_mut());
            }
            3 => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Winona.as_ptr().cast_mut());
            }
            4 => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Phoebe.as_ptr().cast_mut());
            }
            5 => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Glacia.as_ptr().cast_mut());
            }
            _ => {
                StringCopy(gStringVar1.as_mut_ptr(), gText_Wallace.as_ptr().cast_mut());
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateTrainerFansAfterLinkBattle() {
    if VarGet(VAR_LILYCOVE_FAN_CLUB_STATE) == 2 {
        TryLoseFansFromPlayTimeAfterLinkBattle();
        if gBattleOutcome == B_OUTCOME_WON {
            PlayerGainRandomTrainerFan();
        } else {
            PlayerLoseRandomTrainerFan();
        }
    }
}
pub(crate) unsafe extern "C" fn DidPlayerGetFirstFans() -> u8 {
    return ((*gSaveBlock1Ptr).vars[65] >> 7) as u8 & 1;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerGotFirstFans() {
    (*gSaveBlock1Ptr).vars[65] |= 128;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Script_TryGainNewFanFromCounter() -> u8 {
    return TryGainNewFanFromCounter(gSpecialVar_0x8004 as u8);
}
