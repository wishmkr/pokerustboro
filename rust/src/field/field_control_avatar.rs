//! Translated from `src/field_control_avatar.c` by tools/rustport/c2rs.py.
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

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sWildEncounterImmunitySteps: u8 = 0;
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sPrevMetatileBehavior: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSelectedObjectEvent: u8 = 0;

unsafe extern "C" {
    static AbnormalWeather_EventScript_EndEventAndCleanup_1: CArray<u8, 0>;
    static BattlePyramid_WarpToNextFloor: CArray<u8, 0>;
    static EventScript_Blueprint: CArray<u8, 0>;
    static EventScript_BookShelf: CArray<u8, 0>;
    static EventScript_CableBoxResults: CArray<u8, 0>;
    static EventScript_CannotUseWaterfall: CArray<u8, 0>;
    static EventScript_ClosedSootopolisDoor: CArray<u8, 0>;
    static EventScript_EggHatch: CArray<u8, 0>;
    static EventScript_EmptyTrashCan: CArray<u8, 0>;
    static EventScript_FallDownHole: CArray<u8, 0>;
    static EventScript_FallDownHoleMtPyre: CArray<u8, 0>;
    static EventScript_FieldPoison: CArray<u8, 0>;
    static EventScript_HiddenItemScript: CArray<u8, 0>;
    static EventScript_PC: CArray<u8, 0>;
    static EventScript_PictureBookShelf: CArray<u8, 0>;
    static EventScript_PokeBlockFeeder: CArray<u8, 0>;
    static EventScript_PokemonCenterBookShelf: CArray<u8, 0>;
    static EventScript_Questionnaire: CArray<u8, 0>;
    static EventScript_RegionMap: CArray<u8, 0>;
    static EventScript_RunningShoesManual: CArray<u8, 0>;
    static EventScript_ShopShelf: CArray<u8, 0>;
    static EventScript_TV: CArray<u8, 0>;
    static EventScript_TestSignpostMsg: CArray<u8, 0>;
    static EventScript_TrainerHillTimer: CArray<u8, 0>;
    static EventScript_UseDive: CArray<u8, 0>;
    static EventScript_UseDiveUnderwater: CArray<u8, 0>;
    static EventScript_UseSurf: CArray<u8, 0>;
    static EventScript_UseWaterfall: CArray<u8, 0>;
    static EventScript_Vase: CArray<u8, 0>;
    static EventScript_WirelessBoxResults: CArray<u8, 0>;
    static IslandCave_EventScript_OpenRegiEntrance: CArray<u8, 0>;
    static LittlerootTown_BrendansHouse_2F_EventScript_PC: CArray<u8, 0>;
    static LittlerootTown_MaysHouse_2F_EventScript_PC: CArray<u8, 0>;
    static LittlerootTown_ProfessorBirchsLab_EventScript_ScottAboardSSTidalCall: CArray<u8, 0>;
    static MauvilleCity_EventScript_RegisterWallyCall: CArray<u8, 0>;
    static MossdeepCity_SpaceCenter_2F_EventScript_RivalRayquazaCall: CArray<u8, 0>;
    static Route110_TrickHousePuzzle_EventScript_Door: CArray<u8, 0>;
    static Route119_EventScript_ScottWonAtFortreeGymCall: CArray<u8, 0>;
    static RustboroCity_Gym_EventScript_RegisterRoxanne: CArray<u8, 0>;
    static SSTidalCorridor_EventScript_ReachedStepCount: CArray<u8, 0>;
    static SecretBase_EventScript_CheckEntrance: CArray<u8, 0>;
    static SecretBase_EventScript_CushionInteract: CArray<u8, 0>;
    static SecretBase_EventScript_DollInteract: CArray<u8, 0>;
    static SecretBase_EventScript_PC: CArray<u8, 0>;
    static SecretBase_EventScript_RecordMixingPC: CArray<u8, 0>;
    static SecretBase_EventScript_SandOrnament: CArray<u8, 0>;
    static SecretBase_EventScript_ShieldOrToyTV: CArray<u8, 0>;
    static SkyPillar_Outside_EventScript_ClosedDoor: CArray<u8, 0>;
    static gDirectionToVectors: CArray<UCoords32, 0>;
    static mut gLinkPlayerObjectEvents: CArray<LinkPlayerObjectEvent, 4>;
    static mut gMapHeader: MapHeader;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gPlayerParty: CArray<Pokemon, 6>;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_0x8005: u16;
    static mut gSpecialVar_Facing: u16;
    static mut gSpecialVar_LastTalked: u16;
    fn AbnormalWeatherHasExpired() -> u8;
    fn AdjustFriendship(a0: *mut Pokemon, a1: u8);
    fn CheckForTrainersWantingBattle() -> u8;
    fn CheckInteractedWithFriendsFurnitureBottom();
    fn CheckInteractedWithFriendsFurnitureMiddle();
    fn CheckInteractedWithFriendsFurnitureTop();
    fn CheckInteractedWithFriendsPosterDecor();
    fn CountSSTidalStep(a0: u16) -> u32;
    fn DoCoordEventWeather(a0: u8);
    fn DoDiveWarp();
    fn DoDoorWarp();
    fn DoEscalatorWarp(a0: u8);
    fn DoLavaridgeGym1FWarp();
    fn DoLavaridgeGymB1FWarp();
    fn DoMossdeepGymWarp();
    fn DoPoisonFieldEffect() -> i32;
    fn DoSecretBaseGlitterMatSparkle();
    fn DoSpinExitWarp();
    fn DoTeleportTileWarp();
    fn DoWarp();
    fn FlagGet(a0: u16) -> u8;
    fn GetCurrentTrainerHillMapId() -> u8;
    fn GetNumFloorsInTrainerHillChallenge() -> u8;
    fn GetObjectEventIdByPosition(a0: u16, a1: u16, a2: u8) -> u8;
    fn GetObjectEventScriptPointerByObjectEventId(a0: u8) -> *mut u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetPlayerMovementDirection() -> u8;
    fn GetPlayerSpeed() -> i16;
    fn GetRamScript(a0: u8, a1: *mut u8) -> *mut u8;
    fn GetTrainerHillTrainerScript() -> *mut u8;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn InTrainerHill() -> u32;
    fn InUnionRoom() -> u32;
    fn IncrementBirthIslandRockStepCount();
    fn IncrementGameStat(a0: u8);
    fn IncrementRematchStepCounter();
    fn IsPlayerFacingSurfableFishableWater() -> u8;
    fn IsPlayerSurfingNorth() -> u8;
    fn MapGridGetElevationAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MapGridGetMetatileIdAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_HoldsLargeDecoration(a0: u8) -> u8;
    fn MetatileBehavior_HoldsSmallDecoration(a0: u8) -> u8;
    fn MetatileBehavior_IsAquaHideoutWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsBattlePyramidWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsBlueprint(a0: u8) -> u8;
    fn MetatileBehavior_IsBookShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsCableBoxResults1(a0: u8) -> u8;
    fn MetatileBehavior_IsCableBoxResults2(a0: u8, a1: u8) -> u8;
    fn MetatileBehavior_IsClosedSootopolisDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsCounter(a0: u8) -> u8;
    fn MetatileBehavior_IsCrackedFloorHole(a0: u8) -> u8;
    fn MetatileBehavior_IsDiveable(a0: u8) -> u8;
    fn MetatileBehavior_IsEastArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsEscalator(a0: u8) -> u8;
    fn MetatileBehavior_IsForcedMovementTile(a0: u8) -> u8;
    fn MetatileBehavior_IsLadder(a0: u8) -> u8;
    fn MetatileBehavior_IsLavaridge1FWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsLavaridgeB1FWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsMossdeepGymWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsMtPyreHole(a0: u8) -> u8;
    fn MetatileBehavior_IsNonAnimDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsNorthArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsOpenSecretBaseDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsPC(a0: u8) -> u8;
    fn MetatileBehavior_IsPictureBookShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsPlayerFacingTVScreen(a0: u8, a1: u8) -> u8;
    fn MetatileBehavior_IsPlayerFacingWirelessBoxResults(a0: u8, a1: u8) -> u8;
    fn MetatileBehavior_IsPokeCenterBookShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsPokeblockFeeder(a0: u8) -> u8;
    fn MetatileBehavior_IsQuestionnaire(a0: u8) -> u8;
    fn MetatileBehavior_IsRecordMixingSecretBasePC(a0: u8) -> u8;
    fn MetatileBehavior_IsRegionMap(a0: u8) -> u8;
    fn MetatileBehavior_IsRunningShoesManual(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseDecorationBase(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseGlitterMat(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBasePC(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBasePoster(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseSandOrnament(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseShieldOrToyTV(a0: u8) -> u8;
    fn MetatileBehavior_IsSecretBaseSoundMat(a0: u8) -> u8;
    fn MetatileBehavior_IsShopShelf(a0: u8) -> u8;
    fn MetatileBehavior_IsSkyPillarClosedDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsSouthArrowWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsTrainerHillTimer(a0: u8) -> u8;
    fn MetatileBehavior_IsTrashCan(a0: u8) -> u8;
    fn MetatileBehavior_IsTrickHousePuzzleDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsUnableToEmerge(a0: u8) -> u8;
    fn MetatileBehavior_IsUnionRoomWarp(a0: u8) -> u8;
    fn MetatileBehavior_IsVase(a0: u8) -> u8;
    fn MetatileBehavior_IsWarpDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsWaterfall(a0: u8) -> u8;
    fn MetatileBehavior_IsWestArrowWarp(a0: u8) -> u8;
    fn Overworld_GetMapHeaderByGroupAndId(a0: u16, a1: u16) -> *mut MapHeader;
    fn PartyHasMonWithSurf() -> u8;
    fn PlaySE(a0: u16);
    fn PlaySecretBaseMusicNoteMatSound(a0: i16);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerGetElevation() -> u8;
    fn RunScriptImmediately(a0: *mut u8);
    fn SafariZoneTakeStep() -> u8;
    fn ScriptContext_SetupScript(a0: *mut u8);
    fn SetDiveWarpDive(a0: u16, a1: u16) -> u8;
    fn SetDiveWarpEmerge(a0: u16, a1: u16) -> u8;
    fn SetDynamicWarp(a0: i32, a1: i8, a2: i8, a3: i8);
    fn SetWarpDestinationToDynamicWarp(a0: u8);
    fn SetWarpDestinationToMapWarp(a0: i8, a1: i8, a2: i8);
    fn SetWarpDestinationTrainerHill4F() -> *mut WarpEvent;
    fn SetWarpDestinationTrainerHillFinalFloor(a0: u8) -> *mut WarpEvent;
    fn ShouldDoBrailleRegicePuzzle() -> u8;
    fn ShouldDoRivalRayquazaCall() -> u32;
    fn ShouldDoRoxanneCall() -> u32;
    fn ShouldDoScottBattleFrontierCall() -> u32;
    fn ShouldDoScottFortreeCall() -> u32;
    fn ShouldDoWallyCall() -> u32;
    fn ShouldEggHatch() -> u8;
    fn ShowStartMenu();
    fn StandardWildEncounter(a0: u16, a1: u16) -> u8;
    fn StoreInitialPlayerAvatarState();
    fn TryRunOnFrameMapScript() -> u8;
    fn TrySetCurSecretBase() -> u8;
    fn TryStartMatchCall() -> u32;
    fn UpdateEscapeWarp(a0: i16, a1: i16);
    fn UpdateFarawayIslandStepCounter();
    fn UpdateRepelCounter() -> u8;
    fn UseRegisteredKeyItemOnField() -> u8;
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn WarpIntoSecretBase(a0: *mut MapPosition, a1: *mut MapEvents);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldClearPlayerInput(input: *mut FieldInput) {
    (*input).set_pressedAButton(FALSE);
    (*input).set_checkStandardWildEncounter(FALSE);
    (*input).set_pressedStartButton(FALSE);
    (*input).set_pressedSelectButton(FALSE);
    (*input).set_heldDirection(FALSE);
    (*input).set_heldDirection2(FALSE);
    (*input).set_tookStep(FALSE);
    (*input).set_pressedBButton(FALSE);
    (*input).set_input_field_1_0(FALSE);
    (*input).set_input_field_1_1(FALSE);
    (*input).set_input_field_1_2(FALSE);
    (*input).set_input_field_1_3(FALSE);
    (*input).dpadDirection = 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FieldGetPlayerInput(input: *mut FieldInput, newKeys: u16, heldKeys: u16) {
    let mut tileTransitionState: u8 = gPlayerAvatar.tileTransitionState;
    let mut runningState: u8 = gPlayerAvatar.runningState;
    let mut forcedMove: u8 = MetatileBehavior_IsForcedMovementTile(GetPlayerCurMetatileBehavior(
        runningState as i32,
    ) as u8);
    if tileTransitionState == T_TILE_CENTER && forcedMove == FALSE
        || tileTransitionState == T_NOT_MOVING
    {
        if GetPlayerSpeed() != PLAYER_SPEED_FASTEST {
            if newKeys as i32 & START_BUTTON != 0 {
                (*input).set_pressedStartButton(TRUE);
            }
            if newKeys as i32 & SELECT_BUTTON != 0 {
                (*input).set_pressedSelectButton(TRUE);
            }
            if newKeys as i32 & A_BUTTON != 0 {
                (*input).set_pressedAButton(TRUE);
            }
            if newKeys as i32 & B_BUTTON != 0 {
                (*input).set_pressedBButton(TRUE);
            }
        }
        if heldKeys as i32 & 240 != 0 {
            (*input).set_heldDirection(TRUE);
            (*input).set_heldDirection2(TRUE);
        }
    }
    if forcedMove == FALSE {
        if tileTransitionState == T_TILE_CENTER && runningState == MOVING {
            (*input).set_tookStep(TRUE);
        }
        if forcedMove == FALSE && tileTransitionState == T_TILE_CENTER {
            (*input).set_checkStandardWildEncounter(TRUE);
        }
    }
    if heldKeys as i32 & DPAD_UP != 0 {
        (*input).dpadDirection = DIR_NORTH;
    } else if heldKeys as i32 & DPAD_DOWN != 0 {
        (*input).dpadDirection = DIR_SOUTH;
    } else if heldKeys as i32 & DPAD_LEFT != 0 {
        (*input).dpadDirection = DIR_WEST;
    } else if heldKeys as i32 & DPAD_RIGHT != 0 {
        (*input).dpadDirection = DIR_EAST;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ProcessPlayerFieldInput(input: *mut FieldInput) -> i32 {
    let mut position: MapPosition = zeroed();
    let mut playerDirection: u8 = 0;
    let mut metatileBehavior: u16 = 0;
    gSpecialVar_LastTalked = LOCALID_NONE as u16;
    gSelectedObjectEvent = 0;
    playerDirection = GetPlayerFacingDirection();
    GetPlayerPosition(&raw mut position);
    metatileBehavior = MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32) as u16;
    if CheckForTrainersWantingBattle() == TRUE {
        return TRUE as i32;
    }
    if TryRunOnFrameMapScript() == TRUE {
        return TRUE as i32;
    }
    if (*input).pressedBButton() != 0 && TrySetupDiveEmergeScript() == TRUE as u32 {
        return TRUE as i32;
    }
    if (*input).tookStep() != 0 {
        IncrementGameStat(GAME_STAT_STEPS);
        IncrementBirthIslandRockStepCount();
        if TryStartStepBasedScript(&raw mut position, metatileBehavior, playerDirection as u16)
            == TRUE
        {
            return TRUE as i32;
        }
    }
    if (*input).checkStandardWildEncounter() != 0
        && CheckStandardWildEncounter(metatileBehavior) == TRUE
    {
        return TRUE as i32;
    }
    if (*input).heldDirection() != 0 && (*input).dpadDirection == playerDirection {
        if TryArrowWarp(&raw mut position, metatileBehavior, playerDirection) == TRUE {
            return TRUE as i32;
        }
    }
    GetInFrontOfPlayerPosition(&raw mut position);
    metatileBehavior = MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32) as u16;
    if (*input).pressedAButton() != 0
        && TryStartInteractionScript(&raw mut position, metatileBehavior, playerDirection) == TRUE
    {
        return TRUE as i32;
    }
    if (*input).heldDirection2() != 0 && (*input).dpadDirection == playerDirection {
        if TryDoorWarp(&raw mut position, metatileBehavior, playerDirection) == TRUE {
            return TRUE as i32;
        }
    }
    if (*input).pressedAButton() != 0 && TrySetupDiveDownScript() == TRUE as u32 {
        return TRUE as i32;
    }
    if (*input).pressedStartButton() != 0 {
        PlaySE(SE_WIN_OPEN);
        ShowStartMenu();
        return TRUE as i32;
    }
    if (*input).pressedSelectButton() != 0 && UseRegisteredKeyItemOnField() == TRUE {
        return TRUE as i32;
    }
    return FALSE as i32;
}
pub(crate) unsafe extern "C" fn GetPlayerPosition(position: *mut MapPosition) {
    PlayerGetDestCoords(&raw mut (*position).x, &raw mut (*position).y);
    (*position).elevation = PlayerGetElevation() as i8;
}
pub(crate) unsafe extern "C" fn GetInFrontOfPlayerPosition(position: *mut MapPosition) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut (*position).x, &raw mut (*position).y);
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    if MapGridGetElevationAt(x as i32, y as i32) != ELEVATION_TRANSITION {
        (*position).elevation = PlayerGetElevation() as i8;
    } else {
        (*position).elevation = ELEVATION_TRANSITION as i8;
    }
}
pub(crate) unsafe extern "C" fn GetPlayerCurMetatileBehavior(runningState: i32) -> u16 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    return MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16;
}
pub(crate) unsafe extern "C" fn TryStartInteractionScript(
    position: *mut MapPosition,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    let mut script: *mut u8 = GetInteractionScript(position, metatileBehavior as u8, direction);
    if script.is_null() {
        return FALSE;
    }
    if script
        != LittlerootTown_BrendansHouse_2F_EventScript_PC
            .as_ptr()
            .cast_mut()
        && script
            != LittlerootTown_MaysHouse_2F_EventScript_PC
                .as_ptr()
                .cast_mut()
        && script != SecretBase_EventScript_PC.as_ptr().cast_mut()
        && script != SecretBase_EventScript_RecordMixingPC.as_ptr().cast_mut()
        && script != SecretBase_EventScript_DollInteract.as_ptr().cast_mut()
        && script != SecretBase_EventScript_CushionInteract.as_ptr().cast_mut()
        && script != EventScript_PC.as_ptr().cast_mut()
    {
        PlaySE(SE_SELECT);
    }
    ScriptContext_SetupScript(script);
    return TRUE;
}
pub(crate) unsafe extern "C" fn GetInteractionScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut script: *mut u8 = GetInteractedObjectEventScript(position, metatileBehavior, direction);
    if !script.is_null() {
        return script;
    }
    script = GetInteractedBackgroundEventScript(position, metatileBehavior, direction);
    if !script.is_null() {
        return script;
    }
    script = GetInteractedMetatileScript(position, metatileBehavior, direction);
    if !script.is_null() {
        return script;
    }
    script = GetInteractedWaterScript(position, metatileBehavior, direction);
    if !script.is_null() {
        return script;
    }
    return null_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetInteractedLinkPlayerScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut objectEventId: u8 = 0;
    let mut i: i32 = 0;
    if MetatileBehavior_IsCounter(MapGridGetMetatileBehaviorAt(
        (*position).x as i32,
        (*position).y as i32,
    ) as u8)
        == 0
    {
        objectEventId = GetObjectEventIdByPosition(
            (*position).x as u16,
            (*position).y as u16,
            (*position).elevation as u8,
        );
    } else {
        objectEventId = GetObjectEventIdByPosition(
            (*position).x as u16 + gDirectionToVectors[direction].x as u16,
            (*position).y as u16 + gDirectionToVectors[direction].y as u16,
            (*position).elevation as u8,
        );
    }
    if objectEventId == OBJECT_EVENTS_COUNT
        || gObjectEvents[objectEventId].localId == LOCALID_PLAYER
    {
        return null_mut();
    }
    i = 0;
    while i < 4 {
        if gLinkPlayerObjectEvents[i].active == TRUE
            && gLinkPlayerObjectEvents[i].objEventId == objectEventId
        {
            return null_mut();
        }
        i += 1;
    }
    gSelectedObjectEvent = objectEventId;
    gSpecialVar_LastTalked = gObjectEvents[objectEventId].localId as u16;
    gSpecialVar_Facing = direction as u16;
    return GetObjectEventScriptPointerByObjectEventId(objectEventId);
}
pub(crate) unsafe extern "C" fn GetInteractedObjectEventScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut objectEventId: u8 = 0;
    let mut script: *mut u8 = null_mut();
    objectEventId = GetObjectEventIdByPosition(
        (*position).x as u16,
        (*position).y as u16,
        (*position).elevation as u8,
    );
    if objectEventId == OBJECT_EVENTS_COUNT
        || gObjectEvents[objectEventId].localId == LOCALID_PLAYER
    {
        if MetatileBehavior_IsCounter(metatileBehavior) != TRUE {
            return null_mut();
        }
        objectEventId = GetObjectEventIdByPosition(
            (*position).x as u16 + gDirectionToVectors[direction].x as u16,
            (*position).y as u16 + gDirectionToVectors[direction].y as u16,
            (*position).elevation as u8,
        );
        if objectEventId == OBJECT_EVENTS_COUNT
            || gObjectEvents[objectEventId].localId == LOCALID_PLAYER
        {
            return null_mut();
        }
    }
    gSelectedObjectEvent = objectEventId;
    gSpecialVar_LastTalked = gObjectEvents[objectEventId].localId as u16;
    gSpecialVar_Facing = direction as u16;
    if InTrainerHill() == TRUE as u32 {
        script = GetTrainerHillTrainerScript();
    } else {
        script = GetObjectEventScriptPointerByObjectEventId(objectEventId);
    }
    script = GetRamScript(gSpecialVar_LastTalked as u8, script);
    return script;
}
pub(crate) unsafe extern "C" fn GetInteractedBackgroundEventScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut bgEvent: *mut BgEvent = GetBackgroundEventAtPosition(
        &raw mut gMapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    );
    if bgEvent.is_null() {
        return null_mut();
    }
    if (*bgEvent).bgUnion.script.is_null() {
        return EventScript_TestSignpostMsg.as_ptr().cast_mut();
    }
    match (*bgEvent).kind {
        BG_EVENT_PLAYER_FACING_NORTH => {
            if direction != DIR_NORTH {
                return null_mut();
            }
        }
        BG_EVENT_PLAYER_FACING_SOUTH => {
            if direction != DIR_SOUTH {
                return null_mut();
            }
        }
        BG_EVENT_PLAYER_FACING_EAST => {
            if direction != DIR_EAST {
                return null_mut();
            }
        }
        BG_EVENT_PLAYER_FACING_WEST => {
            if direction != DIR_WEST {
                return null_mut();
            }
        }
        5 | 6 | BG_EVENT_HIDDEN_ITEM => {
            gSpecialVar_0x8004 =
                ((*bgEvent).bgUnion.script as usize as u32 >> 16) as u16 + FLAG_HIDDEN_ITEMS_START;
            gSpecialVar_0x8005 = (*bgEvent).bgUnion.script as usize as u32 as u16;
            if FlagGet(gSpecialVar_0x8004) == TRUE {
                return null_mut();
            }
            return EventScript_HiddenItemScript.as_ptr().cast_mut();
        }
        BG_EVENT_SECRET_BASE => {
            if direction == DIR_NORTH {
                gSpecialVar_0x8004 = (*bgEvent).bgUnion.secretBaseId as u16;
                if TrySetCurSecretBase() != 0 {
                    return SecretBase_EventScript_CheckEntrance.as_ptr().cast_mut();
                }
            }
            return null_mut();
        }
        _ => {
            return (*bgEvent).bgUnion.script;
        }
    }
    return (*bgEvent).bgUnion.script;
}
pub(crate) unsafe extern "C" fn GetInteractedMetatileScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut elevation: i8 = 0;
    if MetatileBehavior_IsPlayerFacingTVScreen(metatileBehavior, direction) == TRUE {
        return EventScript_TV.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsPC(metatileBehavior) == TRUE {
        return EventScript_PC.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsClosedSootopolisDoor(metatileBehavior) == TRUE {
        return EventScript_ClosedSootopolisDoor.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsSkyPillarClosedDoor(metatileBehavior) == TRUE {
        return SkyPillar_Outside_EventScript_ClosedDoor.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsCableBoxResults1(metatileBehavior) == TRUE {
        return EventScript_CableBoxResults.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsPokeblockFeeder(metatileBehavior) == TRUE {
        return EventScript_PokeBlockFeeder.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsTrickHousePuzzleDoor(metatileBehavior) == TRUE {
        return Route110_TrickHousePuzzle_EventScript_Door
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsRegionMap(metatileBehavior) == TRUE {
        return EventScript_RegionMap.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsRunningShoesManual(metatileBehavior) == TRUE {
        return EventScript_RunningShoesManual.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsPictureBookShelf(metatileBehavior) == TRUE {
        return EventScript_PictureBookShelf.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsBookShelf(metatileBehavior) == TRUE {
        return EventScript_BookShelf.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsPokeCenterBookShelf(metatileBehavior) == TRUE {
        return EventScript_PokemonCenterBookShelf.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsVase(metatileBehavior) == TRUE {
        return EventScript_Vase.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsTrashCan(metatileBehavior) == TRUE {
        return EventScript_EmptyTrashCan.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsShopShelf(metatileBehavior) == TRUE {
        return EventScript_ShopShelf.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsBlueprint(metatileBehavior) == TRUE {
        return EventScript_Blueprint.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsPlayerFacingWirelessBoxResults(metatileBehavior, direction) == TRUE {
        return EventScript_WirelessBoxResults.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsCableBoxResults2(metatileBehavior, direction) == TRUE {
        return EventScript_CableBoxResults.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsQuestionnaire(metatileBehavior) == TRUE {
        return EventScript_Questionnaire.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsTrainerHillTimer(metatileBehavior) == TRUE {
        return EventScript_TrainerHillTimer.as_ptr().cast_mut();
    }
    elevation = (*position).elevation;
    if elevation as i32 == MapGridGetElevationAt((*position).x as i32, (*position).y as i32) as i32
    {
        if MetatileBehavior_IsSecretBasePC(metatileBehavior) == TRUE {
            return SecretBase_EventScript_PC.as_ptr().cast_mut();
        }
        if MetatileBehavior_IsRecordMixingSecretBasePC(metatileBehavior) == TRUE {
            return SecretBase_EventScript_RecordMixingPC.as_ptr().cast_mut();
        }
        if MetatileBehavior_IsSecretBaseSandOrnament(metatileBehavior) == TRUE {
            return SecretBase_EventScript_SandOrnament.as_ptr().cast_mut();
        }
        if MetatileBehavior_IsSecretBaseShieldOrToyTV(metatileBehavior) == TRUE {
            return SecretBase_EventScript_ShieldOrToyTV.as_ptr().cast_mut();
        }
        if MetatileBehavior_IsSecretBaseDecorationBase(metatileBehavior) == TRUE {
            CheckInteractedWithFriendsFurnitureBottom();
            return null_mut();
        }
        if MetatileBehavior_HoldsLargeDecoration(metatileBehavior) == TRUE {
            CheckInteractedWithFriendsFurnitureMiddle();
            return null_mut();
        }
        if MetatileBehavior_HoldsSmallDecoration(metatileBehavior) == TRUE {
            CheckInteractedWithFriendsFurnitureTop();
            return null_mut();
        }
    } else if MetatileBehavior_IsSecretBasePoster(metatileBehavior) == TRUE {
        CheckInteractedWithFriendsPosterDecor();
        return null_mut();
    }
    return null_mut();
}
pub(crate) unsafe extern "C" fn GetInteractedWaterScript(
    unused1: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    if FlagGet(FLAG_BADGE05_GET) == TRUE
        && PartyHasMonWithSurf() == TRUE
        && IsPlayerFacingSurfableFishableWater() == TRUE
    {
        return EventScript_UseSurf.as_ptr().cast_mut();
    }
    if MetatileBehavior_IsWaterfall(metatileBehavior) == TRUE {
        if FlagGet(FLAG_BADGE08_GET) == TRUE && IsPlayerSurfingNorth() == TRUE {
            return EventScript_UseWaterfall.as_ptr().cast_mut();
        } else {
            return EventScript_CannotUseWaterfall.as_ptr().cast_mut();
        }
    }
    return null_mut();
}
pub(crate) unsafe extern "C" fn TrySetupDiveDownScript() -> u32 {
    if FlagGet(FLAG_BADGE07_GET) != 0 && TrySetDiveWarp() == 2 {
        ScriptContext_SetupScript(EventScript_UseDive.as_ptr().cast_mut());
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn TrySetupDiveEmergeScript() -> u32 {
    if FlagGet(FLAG_BADGE07_GET) != 0
        && gMapHeader.mapType == MAP_TYPE_UNDERWATER
        && TrySetDiveWarp() == 1
    {
        ScriptContext_SetupScript(EventScript_UseDiveUnderwater.as_ptr().cast_mut());
        return TRUE as u32;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn TryStartStepBasedScript(
    position: *mut MapPosition,
    metatileBehavior: u16,
    direction: u16,
) -> u8 {
    if TryStartCoordEventScript(position) == TRUE {
        return TRUE;
    }
    if TryStartWarpEventScript(position, metatileBehavior) == TRUE {
        return TRUE;
    }
    if TryStartMiscWalkingScripts(metatileBehavior) == TRUE {
        return TRUE;
    }
    if TryStartStepCountScript(metatileBehavior) == TRUE {
        return TRUE;
    }
    if UpdateRepelCounter() == TRUE {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn TryStartCoordEventScript(position: *mut MapPosition) -> u8 {
    let mut script: *mut u8 = GetCoordEventScriptAtPosition(
        &raw mut gMapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    );
    if script.is_null() {
        return FALSE;
    }
    ScriptContext_SetupScript(script);
    return TRUE;
}
pub(crate) unsafe extern "C" fn TryStartMiscWalkingScripts(metatileBehavior: u16) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if MetatileBehavior_IsCrackedFloorHole(metatileBehavior as u8) != 0 {
        ScriptContext_SetupScript(EventScript_FallDownHole.as_ptr().cast_mut());
        return TRUE;
    } else if MetatileBehavior_IsBattlePyramidWarp(metatileBehavior as u8) != 0 {
        ScriptContext_SetupScript(BattlePyramid_WarpToNextFloor.as_ptr().cast_mut());
        return TRUE;
    } else if MetatileBehavior_IsSecretBaseGlitterMat(metatileBehavior as u8) == TRUE {
        DoSecretBaseGlitterMatSparkle();
        return FALSE;
    } else if MetatileBehavior_IsSecretBaseSoundMat(metatileBehavior as u8) == TRUE {
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        PlaySecretBaseMusicNoteMatSound(MapGridGetMetatileIdAt(x as i32, y as i32) as i16);
        return FALSE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn TryStartStepCountScript(metatileBehavior: u16) -> u8 {
    if InUnionRoom() == TRUE as u32 {
        return FALSE;
    }
    IncrementRematchStepCounter();
    UpdateFriendshipStepCounter();
    UpdateFarawayIslandStepCounter();
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_FORCED_MOVE == 0
        && MetatileBehavior_IsForcedMovementTile(metatileBehavior as u8) == 0
    {
        if UpdatePoisonStepCounter() == TRUE {
            ScriptContext_SetupScript(EventScript_FieldPoison.as_ptr().cast_mut());
            return TRUE;
        }
        if ShouldEggHatch() != 0 {
            IncrementGameStat(GAME_STAT_HATCHED_EGGS);
            ScriptContext_SetupScript(EventScript_EggHatch.as_ptr().cast_mut());
            return TRUE;
        }
        if AbnormalWeatherHasExpired() == TRUE {
            ScriptContext_SetupScript(
                AbnormalWeather_EventScript_EndEventAndCleanup_1
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoBrailleRegicePuzzle() == TRUE {
            ScriptContext_SetupScript(IslandCave_EventScript_OpenRegiEntrance.as_ptr().cast_mut());
            return TRUE;
        }
        if ShouldDoWallyCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                MauvilleCity_EventScript_RegisterWallyCall
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoScottFortreeCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                Route119_EventScript_ScottWonAtFortreeGymCall
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoScottBattleFrontierCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                LittlerootTown_ProfessorBirchsLab_EventScript_ScottAboardSSTidalCall
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoRoxanneCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                RustboroCity_Gym_EventScript_RegisterRoxanne
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoRivalRayquazaCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                MossdeepCity_SpaceCenter_2F_EventScript_RivalRayquazaCall
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
    }
    if SafariZoneTakeStep() == TRUE {
        return TRUE;
    }
    if CountSSTidalStep(1) == 1 {
        ScriptContext_SetupScript(
            SSTidalCorridor_EventScript_ReachedStepCount
                .as_ptr()
                .cast_mut(),
        );
        return TRUE;
    }
    if TryStartMatchCall() != 0 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ClearFriendshipStepCounter() {
    VarSet(VAR_FRIENDSHIP_STEP_COUNTER, 0);
}
pub(crate) unsafe extern "C" fn UpdateFriendshipStepCounter() {
    let mut ptr: *mut u16 = GetVarPointer(VAR_FRIENDSHIP_STEP_COUNTER);
    let mut i: i32 = 0;
    *ptr += 1;
    *ptr = (*ptr as i32 % 128) as u16;
    if *ptr == 0 {
        let mut mon: *mut Pokemon = gPlayerParty.as_mut_ptr();
        i = 0;
        while i < PARTY_SIZE {
            AdjustFriendship(mon, FRIENDSHIP_EVENT_WALKING);
            mon = mon.at(1);
            i += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ClearPoisonStepCounter() {
    VarSet(VAR_POISON_STEP_COUNTER, 0);
}
pub(crate) unsafe extern "C" fn UpdatePoisonStepCounter() -> u8 {
    let mut ptr: *mut u16 = null_mut();
    if gMapHeader.mapType != MAP_TYPE_SECRET_BASE {
        ptr = GetVarPointer(VAR_POISON_STEP_COUNTER);
        *ptr += 1;
        *ptr = (*ptr as i32 % 4) as u16;
        if *ptr == 0 {
            match DoPoisonFieldEffect() {
                FLDPSN_NONE => {
                    return FALSE;
                }
                FLDPSN_PSN => {
                    return FALSE;
                }
                FLDPSN_FNT => {
                    return TRUE;
                }
                _ => {}
            }
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RestartWildEncounterImmunitySteps() {
    sWildEncounterImmunitySteps = 0;
}
pub(crate) unsafe extern "C" fn CheckStandardWildEncounter(metatileBehavior: u16) -> u8 {
    if sWildEncounterImmunitySteps < 4 {
        sWildEncounterImmunitySteps += 1;
        sPrevMetatileBehavior = metatileBehavior;
        return FALSE;
    }
    if StandardWildEncounter(metatileBehavior, sPrevMetatileBehavior) == TRUE {
        sWildEncounterImmunitySteps = 0;
        sPrevMetatileBehavior = metatileBehavior;
        return TRUE;
    }
    sPrevMetatileBehavior = metatileBehavior;
    return FALSE;
}
pub(crate) unsafe extern "C" fn TryArrowWarp(
    position: *mut MapPosition,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    let mut warpEventId: i8 = GetWarpEventAtMapPosition(&raw mut gMapHeader, position);
    if IsArrowWarpMetatileBehavior(metatileBehavior, direction) == TRUE
        && warpEventId != WARP_ID_NONE
    {
        StoreInitialPlayerAvatarState();
        SetupWarp(&raw mut gMapHeader, warpEventId, position);
        DoWarp();
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn TryStartWarpEventScript(
    position: *mut MapPosition,
    metatileBehavior: u16,
) -> u8 {
    let mut warpEventId: i8 = GetWarpEventAtMapPosition(&raw mut gMapHeader, position);
    if warpEventId != WARP_ID_NONE && IsWarpMetatileBehavior(metatileBehavior) == TRUE {
        StoreInitialPlayerAvatarState();
        SetupWarp(&raw mut gMapHeader, warpEventId, position);
        if MetatileBehavior_IsEscalator(metatileBehavior as u8) == TRUE {
            DoEscalatorWarp(metatileBehavior as u8);
            return TRUE;
        }
        if MetatileBehavior_IsLavaridgeB1FWarp(metatileBehavior as u8) == TRUE {
            DoLavaridgeGymB1FWarp();
            return TRUE;
        }
        if MetatileBehavior_IsLavaridge1FWarp(metatileBehavior as u8) == TRUE {
            DoLavaridgeGym1FWarp();
            return TRUE;
        }
        if MetatileBehavior_IsAquaHideoutWarp(metatileBehavior as u8) == TRUE {
            DoTeleportTileWarp();
            return TRUE;
        }
        if MetatileBehavior_IsUnionRoomWarp(metatileBehavior as u8) == TRUE {
            DoSpinExitWarp();
            return TRUE;
        }
        if MetatileBehavior_IsMtPyreHole(metatileBehavior as u8) == TRUE {
            ScriptContext_SetupScript(EventScript_FallDownHoleMtPyre.as_ptr().cast_mut());
            return TRUE;
        }
        if MetatileBehavior_IsMossdeepGymWarp(metatileBehavior as u8) == TRUE {
            DoMossdeepGymWarp();
            return TRUE;
        }
        DoWarp();
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn IsWarpMetatileBehavior(metatileBehavior: u16) -> u8 {
    if MetatileBehavior_IsWarpDoor(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsLadder(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsEscalator(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsNonAnimDoor(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsLavaridgeB1FWarp(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsLavaridge1FWarp(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsAquaHideoutWarp(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsMtPyreHole(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsMossdeepGymWarp(metatileBehavior as u8) != TRUE
        && MetatileBehavior_IsUnionRoomWarp(metatileBehavior as u8) != TRUE
    {
        return FALSE;
    }
    return TRUE;
}
pub(crate) unsafe extern "C" fn IsArrowWarpMetatileBehavior(
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    match direction {
        DIR_NORTH => {
            return MetatileBehavior_IsNorthArrowWarp(metatileBehavior as u8);
        }
        DIR_SOUTH => {
            return MetatileBehavior_IsSouthArrowWarp(metatileBehavior as u8);
        }
        DIR_WEST => {
            return MetatileBehavior_IsWestArrowWarp(metatileBehavior as u8);
        }
        DIR_EAST => {
            return MetatileBehavior_IsEastArrowWarp(metatileBehavior as u8);
        }
        _ => {}
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetWarpEventAtMapPosition(
    mapHeader: *mut MapHeader,
    position: *mut MapPosition,
) -> i8 {
    return GetWarpEventAtPosition(
        mapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    );
}
pub(crate) unsafe extern "C" fn SetupWarp(
    unused: *mut MapHeader,
    warpEventId: i8,
    position: *mut MapPosition,
) {
    let mut warpEvent: *mut WarpEvent = null_mut();
    let mut trainerHillMapId: u8 = GetCurrentTrainerHillMapId();
    if trainerHillMapId != 0 {
        if trainerHillMapId == GetNumFloorsInTrainerHillChallenge() {
            if warpEventId == 0 {
                warpEvent = (*gMapHeader.events).warps;
            } else {
                warpEvent = SetWarpDestinationTrainerHill4F();
            }
        } else if trainerHillMapId == TRAINER_HILL_ROOF {
            warpEvent = SetWarpDestinationTrainerHillFinalFloor(warpEventId as u8);
        } else {
            warpEvent = (*gMapHeader.events).warps.at(warpEventId);
        }
    } else {
        warpEvent = (*gMapHeader.events).warps.at(warpEventId);
    }
    if (*warpEvent).mapNum == 127 {
        SetWarpDestinationToDynamicWarp((*warpEvent).warpId);
    } else {
        let mut mapHeader: *mut MapHeader = null_mut();
        SetWarpDestinationToMapWarp(
            (*warpEvent).mapGroup as i8,
            (*warpEvent).mapNum as i8,
            (*warpEvent).warpId as i8,
        );
        UpdateEscapeWarp((*position).x, (*position).y);
        mapHeader = Overworld_GetMapHeaderByGroupAndId(
            (*warpEvent).mapGroup as u16,
            (*warpEvent).mapNum as u16,
        );
        if (*(*(*mapHeader).events).warps.at((*warpEvent).warpId)).mapNum == 127 {
            SetDynamicWarp(
                (*(*(*mapHeader).events).warps.at(warpEventId)).warpId as i32,
                (*gSaveBlock1Ptr).location.mapGroup,
                (*gSaveBlock1Ptr).location.mapNum,
                warpEventId,
            );
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoorWarp(
    position: *mut MapPosition,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    let mut warpEventId: i8 = 0;
    if direction == DIR_NORTH {
        if MetatileBehavior_IsOpenSecretBaseDoor(metatileBehavior as u8) == TRUE {
            WarpIntoSecretBase(position, gMapHeader.events);
            return TRUE;
        }
        if MetatileBehavior_IsWarpDoor(metatileBehavior as u8) == TRUE {
            warpEventId = GetWarpEventAtMapPosition(&raw mut gMapHeader, position);
            if warpEventId != WARP_ID_NONE && IsWarpMetatileBehavior(metatileBehavior) == TRUE {
                StoreInitialPlayerAvatarState();
                SetupWarp(&raw mut gMapHeader, warpEventId, position);
                DoDoorWarp();
                return TRUE;
            }
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetWarpEventAtPosition(
    mapHeader: *mut MapHeader,
    x: u16,
    y: u16,
    elevation: u8,
) -> i8 {
    let mut i: i32 = 0;
    let mut warpEvent: *mut WarpEvent = (*(*mapHeader).events).warps;
    let mut warpCount: u8 = (*(*mapHeader).events).warpCount;
    i = 0;
    while i < warpCount as i32 {
        if (*warpEvent).x as u16 == x && (*warpEvent).y as u16 == y {
            if (*warpEvent).elevation == elevation || (*warpEvent).elevation == ELEVATION_TRANSITION
            {
                return i as i8;
            }
        }
        i += 1;
        warpEvent = warpEvent.at(1);
    }
    return WARP_ID_NONE;
}
pub(crate) unsafe extern "C" fn TryRunCoordEventScript(coordEvent: *mut CoordEvent) -> *mut u8 {
    if !coordEvent.is_null() {
        if (*coordEvent).script.is_null() {
            DoCoordEventWeather((*coordEvent).trigger as u8);
            return null_mut();
        }
        if (*coordEvent).trigger == TRIGGER_RUN_IMMEDIATELY {
            RunScriptImmediately((*coordEvent).script);
            return null_mut();
        }
        if VarGet((*coordEvent).trigger) == (*coordEvent).index as u8 as u16 {
            return (*coordEvent).script;
        }
    }
    return null_mut();
}
pub(crate) unsafe extern "C" fn GetCoordEventScriptAtPosition(
    mapHeader: *mut MapHeader,
    x: u16,
    y: u16,
    elevation: u8,
) -> *mut u8 {
    let mut i: i32 = 0;
    let mut coordEvents: *mut CoordEvent = (*(*mapHeader).events).coordEvents;
    let mut coordEventCount: u8 = (*(*mapHeader).events).coordEventCount;
    i = 0;
    while i < coordEventCount as i32 {
        if (*coordEvents.at(i)).x as u16 == x && (*coordEvents.at(i)).y as u16 == y {
            if (*coordEvents.at(i)).elevation == elevation
                || (*coordEvents.at(i)).elevation == ELEVATION_TRANSITION
            {
                let mut script: *mut u8 = TryRunCoordEventScript(coordEvents.at(i));
                if !script.is_null() {
                    return script;
                }
            }
        }
        i += 1;
    }
    return null_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCoordEventScriptAtMapPosition(position: *mut MapPosition) -> *mut u8 {
    return GetCoordEventScriptAtPosition(
        &raw mut gMapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    );
}
pub(crate) unsafe extern "C" fn GetBackgroundEventAtPosition(
    mapHeader: *mut MapHeader,
    x: u16,
    y: u16,
    elevation: u8,
) -> *mut BgEvent {
    let mut i: u8 = 0;
    let mut bgEvents: *mut BgEvent = (*(*mapHeader).events).bgEvents;
    let mut bgEventCount: u8 = (*(*mapHeader).events).bgEventCount;
    i = 0;
    while i < bgEventCount {
        if (*bgEvents.at(i)).x == x && (*bgEvents.at(i)).y == y {
            if (*bgEvents.at(i)).elevation == elevation
                || (*bgEvents.at(i)).elevation == ELEVATION_TRANSITION
            {
                return bgEvents.at(i);
            }
        }
        i += 1;
    }
    return null_mut();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryDoDiveWarp(position: *mut MapPosition, metatileBehavior: u16) -> u8 {
    if gMapHeader.mapType == MAP_TYPE_UNDERWATER
        && MetatileBehavior_IsUnableToEmerge(metatileBehavior as u8) == 0
    {
        if SetDiveWarpEmerge(
            (*position).x as u16 - MAP_OFFSET as u16,
            (*position).y as u16 - MAP_OFFSET as u16,
        ) != 0
        {
            StoreInitialPlayerAvatarState();
            DoDiveWarp();
            PlaySE(SE_M_DIVE);
            return TRUE;
        }
    } else if MetatileBehavior_IsDiveable(metatileBehavior as u8) == TRUE {
        if SetDiveWarpDive(
            (*position).x as u16 - MAP_OFFSET as u16,
            (*position).y as u16 - MAP_OFFSET as u16,
        ) != 0
        {
            StoreInitialPlayerAvatarState();
            DoDiveWarp();
            PlaySE(SE_M_DIVE);
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TrySetDiveWarp() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut metatileBehavior: u8 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    metatileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    if gMapHeader.mapType == MAP_TYPE_UNDERWATER
        && MetatileBehavior_IsUnableToEmerge(metatileBehavior) == 0
    {
        if SetDiveWarpEmerge(x as u16 - MAP_OFFSET as u16, y as u16 - MAP_OFFSET as u16) == TRUE {
            return 1;
        }
    } else if MetatileBehavior_IsDiveable(metatileBehavior) == TRUE {
        if SetDiveWarpDive(x as u16 - MAP_OFFSET as u16, y as u16 - MAP_OFFSET as u16) == TRUE {
            return 2;
        }
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetObjectEventScriptPointerPlayerFacing() -> *mut u8 {
    let mut direction: u8 = 0;
    let mut position: MapPosition = zeroed();
    direction = GetPlayerMovementDirection();
    GetInFrontOfPlayerPosition(&raw mut position);
    return GetInteractedObjectEventScript(
        &raw mut position,
        MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32) as u8,
        direction,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetCableClubWarp() -> i32 {
    let mut position: MapPosition = zeroed();
    GetPlayerMovementDirection();
    GetPlayerPosition(&raw mut position);
    MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32);
    SetupWarp(
        &raw mut gMapHeader,
        GetWarpEventAtMapPosition(&raw mut gMapHeader, &raw mut position),
        &raw mut position,
    );
    return 0;
}
