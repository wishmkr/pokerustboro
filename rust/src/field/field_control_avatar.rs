//! Translated from `src/field_control_avatar.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_setup::IncrementRematchStepCounter;
use crate::bike::GetPlayerSpeed;
use crate::braille_puzzles::ShouldDoBrailleRegicePuzzle;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::coord_event_weather::DoCoordEventWeather;
use crate::daycare::ShouldEggHatch;
use crate::event_data::{FlagGet, GetVarPointer, VarGet, VarSet};
use crate::event_object_movement::{
    GetObjectEventIdByPosition, GetObjectEventScriptPointerByObjectEventId,
};
use crate::faraway_island::UpdateFarawayIslandStepCounter;
use crate::ffi::{
    gSpecialVar_0x8004, gSpecialVar_0x8005, gSpecialVar_Facing, gSpecialVar_LastTalked,
};
use crate::field_player_avatar::{
    GetPlayerFacingDirection, GetPlayerMovementDirection, GetXYCoordsOneStepInFrontOfPlayer,
    IsPlayerFacingSurfableFishableWater, IsPlayerSurfingNorth, PartyHasMonWithSurf,
    PlayerGetDestCoords, PlayerGetElevation, gObjectEvents, gPlayerAvatar,
};
use crate::field_poison::DoPoisonFieldEffect;
use crate::field_screen_effect::{
    DoDiveWarp, DoDoorWarp, DoEscalatorWarp, DoLavaridgeGym1FWarp, DoLavaridgeGymB1FWarp,
    DoMossdeepGymWarp, DoSpinExitWarp, DoTeleportTileWarp, DoWarp,
};
use crate::field_specials::{
    AbnormalWeatherHasExpired, CountSSTidalStep, IncrementBirthIslandRockStepCount,
    ShouldDoRivalRayquazaCall, ShouldDoRoxanneCall, ShouldDoScottBattleFrontierCall,
    ShouldDoScottFortreeCall, ShouldDoWallyCall,
};
use crate::fieldmap::{
    MapGridGetElevationAt, MapGridGetMetatileBehaviorAt, MapGridGetMetatileIdAt, gMapHeader,
};
use crate::fldeff_misc::{DoSecretBaseGlitterMatSparkle, PlaySecretBaseMusicNoteMatSound};
use crate::item_menu::UseRegisteredKeyItemOnField;
use crate::load_save::gSaveBlock1Ptr;
use crate::match_call::TryStartMatchCall;
use crate::metatile_behavior::{
    MetatileBehavior_HoldsLargeDecoration, MetatileBehavior_HoldsSmallDecoration,
    MetatileBehavior_IsAquaHideoutWarp, MetatileBehavior_IsBattlePyramidWarp,
    MetatileBehavior_IsBlueprint, MetatileBehavior_IsBookShelf,
    MetatileBehavior_IsCableBoxResults1, MetatileBehavior_IsCableBoxResults2,
    MetatileBehavior_IsClosedSootopolisDoor, MetatileBehavior_IsCounter,
    MetatileBehavior_IsCrackedFloorHole, MetatileBehavior_IsDiveable,
    MetatileBehavior_IsEastArrowWarp, MetatileBehavior_IsEscalator,
    MetatileBehavior_IsForcedMovementTile, MetatileBehavior_IsLadder,
    MetatileBehavior_IsLavaridge1FWarp, MetatileBehavior_IsLavaridgeB1FWarp,
    MetatileBehavior_IsMossdeepGymWarp, MetatileBehavior_IsMtPyreHole,
    MetatileBehavior_IsNonAnimDoor, MetatileBehavior_IsNorthArrowWarp,
    MetatileBehavior_IsOpenSecretBaseDoor, MetatileBehavior_IsPC,
    MetatileBehavior_IsPictureBookShelf, MetatileBehavior_IsPlayerFacingTVScreen,
    MetatileBehavior_IsPlayerFacingWirelessBoxResults, MetatileBehavior_IsPokeCenterBookShelf,
    MetatileBehavior_IsPokeblockFeeder, MetatileBehavior_IsQuestionnaire,
    MetatileBehavior_IsRecordMixingSecretBasePC, MetatileBehavior_IsRegionMap,
    MetatileBehavior_IsRunningShoesManual, MetatileBehavior_IsSecretBaseDecorationBase,
    MetatileBehavior_IsSecretBaseGlitterMat, MetatileBehavior_IsSecretBasePC,
    MetatileBehavior_IsSecretBasePoster, MetatileBehavior_IsSecretBaseSandOrnament,
    MetatileBehavior_IsSecretBaseShieldOrToyTV, MetatileBehavior_IsSecretBaseSoundMat,
    MetatileBehavior_IsShopShelf, MetatileBehavior_IsSkyPillarClosedDoor,
    MetatileBehavior_IsSouthArrowWarp, MetatileBehavior_IsTrainerHillTimer,
    MetatileBehavior_IsTrashCan, MetatileBehavior_IsTrickHousePuzzleDoor,
    MetatileBehavior_IsUnableToEmerge, MetatileBehavior_IsUnionRoomWarp, MetatileBehavior_IsVase,
    MetatileBehavior_IsWarpDoor, MetatileBehavior_IsWaterfall, MetatileBehavior_IsWestArrowWarp,
};
use crate::overworld::{
    IncrementGameStat, Overworld_GetMapHeaderByGroupAndId, SetDiveWarpDive, SetDiveWarpEmerge,
    SetDynamicWarp, SetWarpDestinationToDynamicWarp, SetWarpDestinationToMapWarp,
    StoreInitialPlayerAvatarState, UpdateEscapeWarp, gLinkPlayerObjectEvents,
};
use crate::pokemon::{AdjustFriendship, gPlayerParty};
use crate::safari_zone::SafariZoneTakeStep;
use crate::script::TryRunOnFrameMapScript;
use crate::script::{RunScriptImmediately, ScriptContext_SetupScript};
use crate::secret_base::{
    CheckInteractedWithFriendsFurnitureBottom, CheckInteractedWithFriendsFurnitureMiddle,
    CheckInteractedWithFriendsFurnitureTop, CheckInteractedWithFriendsPosterDecor,
    TrySetCurSecretBase, WarpIntoSecretBase,
};
use crate::sound::PlaySE;
use crate::start_menu::ShowStartMenu;
use crate::trainer_hill::{
    GetCurrentTrainerHillMapId, GetNumFloorsInTrainerHillChallenge, GetTrainerHillTrainerScript,
    InTrainerHill, SetWarpDestinationTrainerHill4F, SetWarpDestinationTrainerHillFinalFloor,
};
use crate::trainer_see::CheckForTrainersWantingBattle;
#[allow(unused_imports)]
use crate::types::*;
use crate::union_room::InUnionRoom;
use crate::wild_encounter::{StandardWildEncounter, UpdateRepelCounter};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;

#[unsafe(link_section = "ewram_data")]
pub(crate) static sWildEncounterImmunitySteps: crate::global::Global<u8> =
    crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub(crate) static sPrevMetatileBehavior: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gSelectedObjectEvent: u8 = 0;

/// `GetRamScript` with this module's view of its types.
#[inline]
unsafe fn GetRamScript(a0: u8, a1: *mut u8) -> *mut u8 {
    unsafe { crate::script::GetRamScript(a0, a1 as _) as *mut u8 }
}

pub unsafe fn FieldClearPlayerInput(input: *mut FieldInput) {
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
pub unsafe fn FieldGetPlayerInput(input: *mut FieldInput, newKeys: u16, heldKeys: u16) {
    let tileTransitionState: u8 = gPlayerAvatar.tileTransitionState;
    let runningState: u8 = gPlayerAvatar.runningState;
    let forcedMove: u8 = MetatileBehavior_IsForcedMovementTile(GetPlayerCurMetatileBehavior(
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
pub unsafe fn ProcessPlayerFieldInput(input: *mut FieldInput) -> i32 {
    let mut position: MapPosition = zeroed();
    gSpecialVar_LastTalked = LOCALID_NONE as u16;
    gSelectedObjectEvent = 0;
    let playerDirection: u8 = GetPlayerFacingDirection();
    GetPlayerPosition(&raw mut position);
    let mut metatileBehavior: u16 =
        MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32) as u16;
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
    if (*input).heldDirection() != 0
        && (*input).dpadDirection == playerDirection
        && TryArrowWarp(&raw mut position, metatileBehavior, playerDirection) == TRUE
    {
        return TRUE as i32;
    }
    GetInFrontOfPlayerPosition(&raw mut position);
    metatileBehavior = MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32) as u16;
    if (*input).pressedAButton() != 0
        && TryStartInteractionScript(&raw mut position, metatileBehavior, playerDirection) == TRUE
    {
        return TRUE as i32;
    }
    if (*input).heldDirection2() != 0
        && (*input).dpadDirection == playerDirection
        && TryDoorWarp(&raw mut position, metatileBehavior, playerDirection) == TRUE
    {
        return TRUE as i32;
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
    FALSE as i32
}
unsafe fn GetPlayerPosition(position: *mut MapPosition) {
    PlayerGetDestCoords(&raw mut (*position).x, &raw mut (*position).y);
    (*position).elevation = PlayerGetElevation() as i8;
}
unsafe fn GetInFrontOfPlayerPosition(position: *mut MapPosition) {
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
unsafe fn GetPlayerCurMetatileBehavior(runningState: i32) -> u16 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u16
}
unsafe fn TryStartInteractionScript(
    position: *mut MapPosition,
    metatileBehavior: u16,
    direction: u8,
) -> u8 {
    let script: *mut u8 = GetInteractionScript(position, metatileBehavior as u8, direction);
    if script.is_null() {
        return FALSE;
    }
    if script
        != (*crate::asmdata::LittlerootTown_BrendansHouse_2F_EventScript_PC.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut()
        && script
            != (*crate::asmdata::LittlerootTown_MaysHouse_2F_EventScript_PC.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        && script
            != (*crate::asmdata::SecretBase_EventScript_PC.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        && script
            != (*crate::asmdata::SecretBase_EventScript_RecordMixingPC.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        && script
            != (*crate::asmdata::SecretBase_EventScript_DollInteract.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        && script
            != (*crate::asmdata::SecretBase_EventScript_CushionInteract.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
        && script
            != (*crate::asmdata::EventScript_PC.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut()
    {
        PlaySE(SE_SELECT);
    }
    ScriptContext_SetupScript(script);
    TRUE
}
unsafe fn GetInteractionScript(
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
    null_mut()
}
pub unsafe fn GetInteractedLinkPlayerScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut objectEventId: u8 = 0;
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
            (*position).x as u16
                + (*(&raw const crate::data::overworld::gDirectionToVectors)
                    .cast::<CArray<UCoords32, 0>>())[direction]
                    .x as u16,
            (*position).y as u16
                + (*(&raw const crate::data::overworld::gDirectionToVectors)
                    .cast::<CArray<UCoords32, 0>>())[direction]
                    .y as u16,
            (*position).elevation as u8,
        );
    }
    if objectEventId == OBJECT_EVENTS_COUNT
        || gObjectEvents[objectEventId].localId == LOCALID_PLAYER
    {
        return null_mut();
    }
    for i in 0..4i32 {
        if gLinkPlayerObjectEvents[i].active == TRUE
            && gLinkPlayerObjectEvents[i].objEventId == objectEventId
        {
            return null_mut();
        }
    }
    gSelectedObjectEvent = objectEventId;
    gSpecialVar_LastTalked = gObjectEvents[objectEventId].localId as u16;
    gSpecialVar_Facing = direction as u16;
    GetObjectEventScriptPointerByObjectEventId(objectEventId)
}
unsafe fn GetInteractedObjectEventScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut script: *mut u8 = null_mut();
    let mut objectEventId: u8 = GetObjectEventIdByPosition(
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
            (*position).x as u16
                + (*(&raw const crate::data::overworld::gDirectionToVectors)
                    .cast::<CArray<UCoords32, 0>>())[direction]
                    .x as u16,
            (*position).y as u16
                + (*(&raw const crate::data::overworld::gDirectionToVectors)
                    .cast::<CArray<UCoords32, 0>>())[direction]
                    .y as u16,
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
    script
}
unsafe fn GetInteractedBackgroundEventScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let bgEvent: *mut BgEvent = GetBackgroundEventAtPosition(
        &raw mut gMapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    );
    if bgEvent.is_null() {
        return null_mut();
    }
    if (*bgEvent).bgUnion.script.is_null() {
        return (*crate::asmdata::EventScript_TestSignpostMsg.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
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
            if FlagGet(
                *(&raw const crate::ffi::gSpecialVar_0x8004)
                    .cast::<u16>()
                    .cast_mut(),
            ) == TRUE
            {
                return null_mut();
            }
            return (*crate::asmdata::EventScript_HiddenItemScript.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        BG_EVENT_SECRET_BASE => {
            if direction == DIR_NORTH {
                gSpecialVar_0x8004 = (*bgEvent).bgUnion.secretBaseId as u16;
                if TrySetCurSecretBase() != 0 {
                    return (*crate::asmdata::SecretBase_EventScript_CheckEntrance
                        .cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut();
                }
            }
            return null_mut();
        }
        _ => {
            return (*bgEvent).bgUnion.script;
        }
    }
    (*bgEvent).bgUnion.script
}
unsafe fn GetInteractedMetatileScript(
    position: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    let mut elevation: i8 = 0;
    if MetatileBehavior_IsPlayerFacingTVScreen(metatileBehavior, direction) == TRUE {
        return (*crate::asmdata::EventScript_TV.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsPC(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_PC.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsClosedSootopolisDoor(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_ClosedSootopolisDoor.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsSkyPillarClosedDoor(metatileBehavior) == TRUE {
        return (*crate::asmdata::SkyPillar_Outside_EventScript_ClosedDoor.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsCableBoxResults1(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_CableBoxResults.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsPokeblockFeeder(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_PokeBlockFeeder.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsTrickHousePuzzleDoor(metatileBehavior) == TRUE {
        return (*crate::asmdata::Route110_TrickHousePuzzle_EventScript_Door
            .cast::<CArray<u8, 0>>())
        .as_ptr()
        .cast_mut();
    }
    if MetatileBehavior_IsRegionMap(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_RegionMap.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsRunningShoesManual(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_RunningShoesManual.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsPictureBookShelf(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_PictureBookShelf.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsBookShelf(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_BookShelf.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsPokeCenterBookShelf(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_PokemonCenterBookShelf.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsVase(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_Vase.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsTrashCan(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_EmptyTrashCan.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsShopShelf(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_ShopShelf.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsBlueprint(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_Blueprint.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsPlayerFacingWirelessBoxResults(metatileBehavior, direction) == TRUE {
        return (*crate::asmdata::EventScript_WirelessBoxResults.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsCableBoxResults2(metatileBehavior, direction) == TRUE {
        return (*crate::asmdata::EventScript_CableBoxResults.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsQuestionnaire(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_Questionnaire.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsTrainerHillTimer(metatileBehavior) == TRUE {
        return (*crate::asmdata::EventScript_TrainerHillTimer.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    elevation = (*position).elevation;
    if elevation as i32 == MapGridGetElevationAt((*position).x as i32, (*position).y as i32) as i32
    {
        if MetatileBehavior_IsSecretBasePC(metatileBehavior) == TRUE {
            return (*crate::asmdata::SecretBase_EventScript_PC.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        if MetatileBehavior_IsRecordMixingSecretBasePC(metatileBehavior) == TRUE {
            return (*crate::asmdata::SecretBase_EventScript_RecordMixingPC
                .cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
        }
        if MetatileBehavior_IsSecretBaseSandOrnament(metatileBehavior) == TRUE {
            return (*crate::asmdata::SecretBase_EventScript_SandOrnament.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
        if MetatileBehavior_IsSecretBaseShieldOrToyTV(metatileBehavior) == TRUE {
            return (*crate::asmdata::SecretBase_EventScript_ShieldOrToyTV.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
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
    null_mut()
}
unsafe fn GetInteractedWaterScript(
    unused1: *mut MapPosition,
    metatileBehavior: u8,
    direction: u8,
) -> *mut u8 {
    if FlagGet(FLAG_BADGE05_GET) == TRUE
        && PartyHasMonWithSurf() == TRUE
        && IsPlayerFacingSurfableFishableWater() == TRUE
    {
        return (*crate::asmdata::EventScript_UseSurf.cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut();
    }
    if MetatileBehavior_IsWaterfall(metatileBehavior) == TRUE {
        if FlagGet(FLAG_BADGE08_GET) == TRUE && IsPlayerSurfingNorth() == TRUE {
            return (*crate::asmdata::EventScript_UseWaterfall.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        } else {
            return (*crate::asmdata::EventScript_CannotUseWaterfall.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut();
        }
    }
    null_mut()
}
unsafe fn TrySetupDiveDownScript() -> u32 {
    if FlagGet(FLAG_BADGE07_GET) != 0 && TrySetDiveWarp() == 2 {
        ScriptContext_SetupScript(
            (*crate::asmdata::EventScript_UseDive.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn TrySetupDiveEmergeScript() -> u32 {
    if FlagGet(FLAG_BADGE07_GET) != 0
        && gMapHeader.mapType == MAP_TYPE_UNDERWATER
        && TrySetDiveWarp() == 1
    {
        ScriptContext_SetupScript(
            (*crate::asmdata::EventScript_UseDiveUnderwater.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return TRUE as u32;
    }
    FALSE as u32
}
unsafe fn TryStartStepBasedScript(
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
    FALSE
}
unsafe fn TryStartCoordEventScript(position: *mut MapPosition) -> u8 {
    let script: *mut u8 = GetCoordEventScriptAtPosition(
        &raw mut gMapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    );
    if script.is_null() {
        return FALSE;
    }
    ScriptContext_SetupScript(script);
    TRUE
}
unsafe fn TryStartMiscWalkingScripts(metatileBehavior: u16) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if MetatileBehavior_IsCrackedFloorHole(metatileBehavior as u8) != 0 {
        ScriptContext_SetupScript(
            (*crate::asmdata::EventScript_FallDownHole.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return TRUE;
    } else if MetatileBehavior_IsBattlePyramidWarp(metatileBehavior as u8) != 0 {
        ScriptContext_SetupScript(
            (*crate::asmdata::BattlePyramid_WarpToNextFloor.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return TRUE;
    } else if MetatileBehavior_IsSecretBaseGlitterMat(metatileBehavior as u8) == TRUE {
        DoSecretBaseGlitterMatSparkle();
        return FALSE;
    } else if MetatileBehavior_IsSecretBaseSoundMat(metatileBehavior as u8) == TRUE {
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        PlaySecretBaseMusicNoteMatSound(MapGridGetMetatileIdAt(x as i32, y as i32) as i16);
        return FALSE;
    }
    FALSE
}
unsafe fn TryStartStepCountScript(metatileBehavior: u16) -> u8 {
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
            ScriptContext_SetupScript(
                (*crate::asmdata::EventScript_FieldPoison.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldEggHatch() != 0 {
            IncrementGameStat(GAME_STAT_HATCHED_EGGS);
            ScriptContext_SetupScript(
                (*crate::asmdata::EventScript_EggHatch.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if AbnormalWeatherHasExpired() == TRUE {
            ScriptContext_SetupScript(
                (*crate::asmdata::AbnormalWeather_EventScript_EndEventAndCleanup_1
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoBrailleRegicePuzzle() == TRUE {
            ScriptContext_SetupScript(
                (*crate::asmdata::IslandCave_EventScript_OpenRegiEntrance.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoWallyCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                (*crate::asmdata::MauvilleCity_EventScript_RegisterWallyCall
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoScottFortreeCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                (*crate::asmdata::Route119_EventScript_ScottWonAtFortreeGymCall
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoScottBattleFrontierCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                (*crate::asmdata::LittlerootTown_ProfessorBirchsLab_EventScript_ScottAboardSSTidalCall.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoRoxanneCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                (*crate::asmdata::RustboroCity_Gym_EventScript_RegisterRoxanne
                    .cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
            );
            return TRUE;
        }
        if ShouldDoRivalRayquazaCall() == TRUE as u32 {
            ScriptContext_SetupScript(
                (*crate::asmdata::MossdeepCity_SpaceCenter_2F_EventScript_RivalRayquazaCall
                    .cast::<CArray<u8, 0>>())
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
            (*crate::asmdata::SSTidalCorridor_EventScript_ReachedStepCount.cast::<CArray<u8, 0>>())
                .as_ptr()
                .cast_mut(),
        );
        return TRUE;
    }
    if TryStartMatchCall() != 0 {
        return TRUE;
    }
    FALSE
}
unsafe fn ClearFriendshipStepCounter() {
    VarSet(VAR_FRIENDSHIP_STEP_COUNTER, 0);
}
unsafe fn UpdateFriendshipStepCounter() {
    let ptr: *mut u16 = GetVarPointer(VAR_FRIENDSHIP_STEP_COUNTER);
    *ptr += 1;
    *ptr = (*ptr as i32 % 128) as u16;
    if *ptr == 0 {
        let mut mon: *mut Pokemon = gPlayerParty.as_mut_ptr();
        for i in 0..PARTY_SIZE {
            AdjustFriendship(mon, FRIENDSHIP_EVENT_WALKING);
            mon = mon.at(1);
        }
    }
}
pub unsafe fn ClearPoisonStepCounter() {
    VarSet(VAR_POISON_STEP_COUNTER, 0);
}
unsafe fn UpdatePoisonStepCounter() -> u8 {
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
    FALSE
}
pub fn RestartWildEncounterImmunitySteps() {
    sWildEncounterImmunitySteps.set(0);
}
unsafe fn CheckStandardWildEncounter(metatileBehavior: u16) -> u8 {
    if sWildEncounterImmunitySteps.get() < 4 {
        sWildEncounterImmunitySteps.set(sWildEncounterImmunitySteps.get() + 1);
        sPrevMetatileBehavior.set(metatileBehavior);
        return FALSE;
    }
    if StandardWildEncounter(metatileBehavior, sPrevMetatileBehavior.get()) == TRUE {
        sWildEncounterImmunitySteps.set(0);
        sPrevMetatileBehavior.set(metatileBehavior);
        return TRUE;
    }
    sPrevMetatileBehavior.set(metatileBehavior);
    FALSE
}
unsafe fn TryArrowWarp(position: *mut MapPosition, metatileBehavior: u16, direction: u8) -> u8 {
    let warpEventId: i8 = GetWarpEventAtMapPosition(&raw mut gMapHeader, position);
    if IsArrowWarpMetatileBehavior(metatileBehavior, direction) == TRUE
        && warpEventId != WARP_ID_NONE
    {
        StoreInitialPlayerAvatarState();
        SetupWarp(&raw mut gMapHeader, warpEventId, position);
        DoWarp();
        return TRUE;
    }
    FALSE
}
unsafe fn TryStartWarpEventScript(position: *mut MapPosition, metatileBehavior: u16) -> u8 {
    let warpEventId: i8 = GetWarpEventAtMapPosition(&raw mut gMapHeader, position);
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
            ScriptContext_SetupScript(
                (*crate::asmdata::EventScript_FallDownHoleMtPyre.cast::<CArray<u8, 0>>())
                    .as_ptr()
                    .cast_mut(),
            );
            return TRUE;
        }
        if MetatileBehavior_IsMossdeepGymWarp(metatileBehavior as u8) == TRUE {
            DoMossdeepGymWarp();
            return TRUE;
        }
        DoWarp();
        return TRUE;
    }
    FALSE
}
fn IsWarpMetatileBehavior(metatileBehavior: u16) -> u8 {
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
    TRUE
}
unsafe fn IsArrowWarpMetatileBehavior(metatileBehavior: u16, direction: u8) -> u8 {
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
    FALSE
}
unsafe fn GetWarpEventAtMapPosition(mapHeader: *mut MapHeader, position: *mut MapPosition) -> i8 {
    GetWarpEventAtPosition(
        mapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    )
}
unsafe fn SetupWarp(unused: *mut MapHeader, warpEventId: i8, position: *mut MapPosition) {
    let mut warpEvent: *mut WarpEvent = null_mut();
    let trainerHillMapId: u8 = GetCurrentTrainerHillMapId();
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
        SetWarpDestinationToMapWarp(
            (*warpEvent).mapGroup as i8,
            (*warpEvent).mapNum as i8,
            (*warpEvent).warpId as i8,
        );
        UpdateEscapeWarp((*position).x, (*position).y);
        let mapHeader: *mut MapHeader = Overworld_GetMapHeaderByGroupAndId(
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
unsafe fn TryDoorWarp(position: *mut MapPosition, metatileBehavior: u16, direction: u8) -> u8 {
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
    FALSE
}
unsafe fn GetWarpEventAtPosition(mapHeader: *mut MapHeader, x: u16, y: u16, elevation: u8) -> i8 {
    let mut warpEvent: *mut WarpEvent = (*(*mapHeader).events).warps;
    let warpCount: u8 = (*(*mapHeader).events).warpCount;
    let mut i: i32 = 0;
    while i < warpCount as i32 {
        if (*warpEvent).x as u16 == x
            && (*warpEvent).y as u16 == y
            && ((*warpEvent).elevation == elevation
                || (*warpEvent).elevation == ELEVATION_TRANSITION)
        {
            return i as i8;
        }
        i += 1;
        warpEvent = warpEvent.at(1);
    }
    WARP_ID_NONE
}
unsafe fn TryRunCoordEventScript(coordEvent: *mut CoordEvent) -> *mut u8 {
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
    null_mut()
}
unsafe fn GetCoordEventScriptAtPosition(
    mapHeader: *mut MapHeader,
    x: u16,
    y: u16,
    elevation: u8,
) -> *mut u8 {
    let coordEvents: *mut CoordEvent = (*(*mapHeader).events).coordEvents;
    let coordEventCount: u8 = (*(*mapHeader).events).coordEventCount;
    for i in 0..(coordEventCount as i32) {
        if (*coordEvents.at(i)).x as u16 == x
            && (*coordEvents.at(i)).y as u16 == y
            && ((*coordEvents.at(i)).elevation == elevation
                || (*coordEvents.at(i)).elevation == ELEVATION_TRANSITION)
        {
            let script: *mut u8 = TryRunCoordEventScript(coordEvents.at(i));
            if !script.is_null() {
                return script;
            }
        }
    }
    null_mut()
}
pub unsafe fn GetCoordEventScriptAtMapPosition(position: *mut MapPosition) -> *mut u8 {
    GetCoordEventScriptAtPosition(
        &raw mut gMapHeader,
        (*position).x as u16 - MAP_OFFSET as u16,
        (*position).y as u16 - MAP_OFFSET as u16,
        (*position).elevation as u8,
    )
}
unsafe fn GetBackgroundEventAtPosition(
    mapHeader: *mut MapHeader,
    x: u16,
    y: u16,
    elevation: u8,
) -> *mut BgEvent {
    let bgEvents: *mut BgEvent = (*(*mapHeader).events).bgEvents;
    let bgEventCount: u8 = (*(*mapHeader).events).bgEventCount;
    for i in 0..bgEventCount {
        if (*bgEvents.at(i)).x == x
            && (*bgEvents.at(i)).y == y
            && ((*bgEvents.at(i)).elevation == elevation
                || (*bgEvents.at(i)).elevation == ELEVATION_TRANSITION)
        {
            return bgEvents.at(i);
        }
    }
    null_mut()
}
pub unsafe fn TryDoDiveWarp(position: *mut MapPosition, metatileBehavior: u16) -> u8 {
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
    } else if MetatileBehavior_IsDiveable(metatileBehavior as u8) == TRUE
        && SetDiveWarpDive(
            (*position).x as u16 - MAP_OFFSET as u16,
            (*position).y as u16 - MAP_OFFSET as u16,
        ) != 0
    {
        StoreInitialPlayerAvatarState();
        DoDiveWarp();
        PlaySE(SE_M_DIVE);
        return TRUE;
    }
    FALSE
}
pub unsafe fn TrySetDiveWarp() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    let metatileBehavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    if gMapHeader.mapType == MAP_TYPE_UNDERWATER
        && MetatileBehavior_IsUnableToEmerge(metatileBehavior) == 0
    {
        if SetDiveWarpEmerge(x as u16 - MAP_OFFSET as u16, y as u16 - MAP_OFFSET as u16) == TRUE {
            return 1;
        }
    } else if MetatileBehavior_IsDiveable(metatileBehavior) == TRUE
        && SetDiveWarpDive(x as u16 - MAP_OFFSET as u16, y as u16 - MAP_OFFSET as u16) == TRUE
    {
        return 2;
    }
    0
}
pub unsafe fn GetObjectEventScriptPointerPlayerFacing() -> *mut u8 {
    let mut position: MapPosition = zeroed();
    let direction: u8 = GetPlayerMovementDirection();
    GetInFrontOfPlayerPosition(&raw mut position);
    GetInteractedObjectEventScript(
        &raw mut position,
        MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32) as u8,
        direction,
    )
}
#[unsafe(no_mangle)]
pub unsafe fn SetCableClubWarp() -> i32 {
    let mut position: MapPosition = zeroed();
    GetPlayerMovementDirection();
    GetPlayerPosition(&raw mut position);
    MapGridGetMetatileBehaviorAt(position.x as i32, position.y as i32);
    SetupWarp(
        &raw mut gMapHeader,
        GetWarpEventAtMapPosition(&raw mut gMapHeader, &raw mut position),
        &raw mut position,
    );
    0
}
