//! Translated from `src/bike.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::{
    GetJumpMovementAction, GetOppositeDirection, MoveCoords, SetObjectEventDirection,
};
use crate::field_camera::gUnusedBikeCameraAheadPanback;
use crate::field_player_avatar::{
    CheckForObjectEventCollision, GetPlayerFacingDirection, GetPlayerMovementDirection,
    IsPlayerCollidingWithFarawayIslandMew, PlayerAcroTurnJump, PlayerEndWheelie,
    PlayerEndWheelieWhileMoving, PlayerFaceDirection, PlayerGetDestCoords, PlayerGetElevation,
    PlayerIdleWheelie, PlayerJumpLedge, PlayerLedgeHoppingWheelie, PlayerMovingHoppingWheelie,
    PlayerOnBikeCollide, PlayerOnBikeCollideWithFarawayIslandMew, PlayerPopWheelieWhileMoving,
    PlayerRideWaterCurrent, PlayerSetAnimId, PlayerStandingHoppingWheelie, PlayerStartWheelie,
    PlayerTurnInPlace, PlayerUseAcroBikeOnBumpySlope, PlayerWheelieInPlace, PlayerWheelieMove,
    SetPlayerAvatarTransitionFlags, TestPlayerAvatarFlags, gObjectEvents, gPlayerAvatar,
};
use crate::field_specials::{gBikeCollisions, gBikeCyclingChallenge};
use crate::fieldmap::{MapGridGetMetatileBehaviorAt, gMapHeader};
use crate::metatile_behavior::{
    MetatileBehavior_IsBumpySlope, MetatileBehavior_IsFortreeBridge,
    MetatileBehavior_IsHorizontalRail, MetatileBehavior_IsIsolatedHorizontalRail,
    MetatileBehavior_IsIsolatedVerticalRail, MetatileBehavior_IsRunningDisallowed,
    MetatileBehavior_IsVerticalRail,
};
use crate::overworld::{
    Overworld_ChangeMusicTo, Overworld_ClearSavedMusic, Overworld_PlaySpecialMapMusic,
    Overworld_SetSavedMusic,
};
use crate::sound::PlaySE;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): sMachBikeTransitions sMachBikeSpeedCallbacks sAcroBikeTransitions sAcroBikeInputHandlers sMachBikeSpeeds sAcroBikeJumpTimerList sAcroBikeTricksList

static sAcroBikeInputHandlers: Table<CArray<Option<unsafe fn(*mut u8, u16, u16) -> u8>, 7>> =
    Table((&raw const crate::data::bike::sAcroBikeInputHandlers).cast());
static sAcroBikeTransitions: Table<CArray<Option<unsafe fn(u8)>, 13>> =
    Table((&raw const crate::data::bike::sAcroBikeTransitions).cast());
static sAcroBikeTricksList: Table<CArray<BikeHistoryInputInfo, 4>> =
    Table((&raw const crate::data::bike::sAcroBikeTricksList).cast());
static sMachBikeSpeedCallbacks: Table<CArray<Option<unsafe fn(u8)>, 3>> =
    Table((&raw const crate::data::bike::sMachBikeSpeedCallbacks).cast());
static sMachBikeSpeeds: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::bike::sMachBikeSpeeds).cast());
static sMachBikeTransitions: Table<CArray<Option<unsafe fn(u8)>, 4>> =
    Table((&raw const crate::data::bike::sMachBikeTransitions).cast());

pub unsafe fn MovePlayerOnBike(direction: u8, newKeys: u16, heldKeys: u16) {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_MACH_BIKE as i32 != 0 {
        MovePlayerOnMachBike(direction, newKeys, heldKeys);
    } else {
        MovePlayerOnAcroBike(direction, newKeys, heldKeys);
    }
}
unsafe fn MovePlayerOnMachBike(mut direction: u8, newKeys: u16, heldKeys: u16) {
    sMachBikeTransitions[GetMachBikeTransition(&raw mut direction)].unwrap_unchecked()(direction);
}
unsafe fn GetMachBikeTransition(dirTraveling: *mut u8) -> u8 {
    let direction: u8 = GetPlayerMovementDirection();
    if *dirTraveling == 0 {
        *dirTraveling = direction;
        if gPlayerAvatar.bikeSpeed == PLAYER_SPEED_STANDING {
            gPlayerAvatar.runningState = NOT_MOVING;
            return MACH_TRANS_FACE_DIRECTION;
        }
        gPlayerAvatar.runningState = MOVING;
        return MACH_TRANS_START_MOVING;
    }
    if *dirTraveling != direction && gPlayerAvatar.runningState != MOVING {
        if gPlayerAvatar.bikeSpeed != PLAYER_SPEED_STANDING {
            *dirTraveling = direction;
            gPlayerAvatar.runningState = MOVING;
            return MACH_TRANS_START_MOVING;
        }
        gPlayerAvatar.runningState = TURN_DIRECTION;
        return MACH_TRANS_TURN_DIRECTION;
    } else {
        gPlayerAvatar.runningState = MOVING;
        return MACH_TRANS_KEEP_MOVING;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn MachBikeTransition_FaceDirection(direction: u8) {
    PlayerFaceDirection(direction);
    Bike_SetBikeStill();
}
pub(crate) unsafe fn MachBikeTransition_TurnDirection(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) != 0 {
        PlayerTurnInPlace(direction);
        Bike_SetBikeStill();
    } else {
        MachBikeTransition_FaceDirection((*playerObjEvent).facingDirection() as u8);
    }
}
pub(crate) unsafe fn MachBikeTransition_TrySpeedUp(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut collision: u8 = 0;
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == FALSE {
        if gPlayerAvatar.bikeSpeed != 0 {
            MachBikeTransition_TrySlowDown((*playerObjEvent).movementDirection() as u8);
        } else {
            MachBikeTransition_FaceDirection((*playerObjEvent).movementDirection() as u8);
        }
    } else {
        collision = GetBikeCollision(direction);
        if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
            if collision == COLLISION_LEDGE_JUMP {
                PlayerJumpLedge(direction);
            } else {
                Bike_SetBikeStill();
                if collision == COLLISION_OBJECT_EVENT
                    && IsPlayerCollidingWithFarawayIslandMew(direction) != 0
                {
                    PlayerOnBikeCollideWithFarawayIslandMew(direction);
                } else if !(COLLISION_STOP_SURFING..=COLLISION_ROTATING_GATE).contains(&collision) {
                    PlayerOnBikeCollide(direction);
                }
            }
        } else {
            sMachBikeSpeedCallbacks[gPlayerAvatar.bikeFrameCounter].unwrap_unchecked()(direction);
            gPlayerAvatar.bikeSpeed =
                gPlayerAvatar.bikeFrameCounter + (gPlayerAvatar.bikeFrameCounter >> 1);
            if gPlayerAvatar.bikeFrameCounter < 2 {
                gPlayerAvatar.bikeFrameCounter += 1;
            }
        }
    }
}
pub(crate) unsafe fn MachBikeTransition_TrySlowDown(direction: u8) {
    if gPlayerAvatar.bikeSpeed != PLAYER_SPEED_STANDING {
        gPlayerAvatar.bikeFrameCounter = {
            gPlayerAvatar.bikeSpeed -= 1;
            gPlayerAvatar.bikeSpeed
        };
    }
    let collision: u8 = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerJumpLedge(direction);
        } else {
            Bike_SetBikeStill();
            if collision == COLLISION_OBJECT_EVENT
                && IsPlayerCollidingWithFarawayIslandMew(direction) != 0
            {
                PlayerOnBikeCollideWithFarawayIslandMew(direction);
            } else if !(COLLISION_STOP_SURFING..=COLLISION_ROTATING_GATE).contains(&collision) {
                PlayerOnBikeCollide(direction);
            }
        }
    } else {
        sMachBikeSpeedCallbacks[gPlayerAvatar.bikeFrameCounter].unwrap_unchecked()(direction);
    }
}
unsafe fn MovePlayerOnAcroBike(mut newDirection: u8, newKeys: u16, heldKeys: u16) {
    sAcroBikeTransitions[CheckMovementInputAcroBike(&raw mut newDirection, newKeys, heldKeys)]
        .unwrap_unchecked()(newDirection);
}
unsafe fn CheckMovementInputAcroBike(newDirection: *mut u8, newKeys: u16, heldKeys: u16) -> u8 {
    sAcroBikeInputHandlers[gPlayerAvatar.acroBikeState].unwrap_unchecked()(
        newDirection,
        newKeys,
        heldKeys,
    )
}
pub(crate) unsafe fn AcroBikeHandleInputNormal(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let direction: u8 = GetPlayerMovementDirection();
    gPlayerAvatar.bikeFrameCounter = 0;
    if *newDirection == DIR_NONE {
        if newKeys as i32 & B_BUTTON != 0 {
            *newDirection = direction;
            gPlayerAvatar.runningState = NOT_MOVING;
            gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_STANDING;
            return ACRO_TRANS_NORMAL_TO_WHEELIE;
        } else {
            *newDirection = direction;
            gPlayerAvatar.runningState = NOT_MOVING;
            return ACRO_TRANS_FACE_DIRECTION;
        }
    }
    if *newDirection == direction
        && heldKeys as i32 & B_BUTTON != 0
        && gPlayerAvatar.bikeSpeed == PLAYER_SPEED_STANDING
    {
        gPlayerAvatar.bikeSpeed += 1;
        gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_MOVING;
        return ACRO_TRANS_WHEELIE_RISING_MOVING;
    }
    if *newDirection != direction && gPlayerAvatar.runningState != MOVING {
        gPlayerAvatar.acroBikeState = ACRO_STATE_TURNING;
        gPlayerAvatar.newDirBackup = *newDirection;
        gPlayerAvatar.runningState = NOT_MOVING;
        return CheckMovementInputAcroBike(newDirection, newKeys, heldKeys);
    }
    gPlayerAvatar.runningState = MOVING;
    ACRO_TRANS_MOVING
}
pub(crate) unsafe fn AcroBikeHandleInputTurning(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    *newDirection = gPlayerAvatar.newDirBackup;
    gPlayerAvatar.bikeFrameCounter += 1;
    if gPlayerAvatar.bikeFrameCounter > 6 {
        gPlayerAvatar.runningState = TURN_DIRECTION;
        gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
        Bike_SetBikeStill();
        return ACRO_TRANS_TURN_DIRECTION;
    }
    let direction: u8 = GetPlayerMovementDirection();
    if *newDirection == AcroBike_GetJumpDirection() {
        Bike_SetBikeStill();
        gPlayerAvatar.bikeSpeed = PLAYER_SPEED_NORMAL as u8;
        if *newDirection == GetOppositeDirection(direction) {
            gPlayerAvatar.acroBikeState = ACRO_STATE_TURN_JUMP;
            return ACRO_TRANS_TURN_JUMP;
        } else {
            gPlayerAvatar.runningState = MOVING;
            gPlayerAvatar.acroBikeState = ACRO_STATE_SIDE_JUMP;
            return ACRO_TRANS_SIDE_JUMP;
        }
    }
    *newDirection = direction;
    ACRO_TRANS_FACE_DIRECTION
}
pub(crate) unsafe fn AcroBikeHandleInputWheelieStanding(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let direction: u8 = GetPlayerMovementDirection();
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    gPlayerAvatar.runningState = NOT_MOVING;
    if heldKeys as i32 & B_BUTTON != 0 {
        gPlayerAvatar.bikeFrameCounter += 1;
    } else {
        gPlayerAvatar.bikeFrameCounter = 0;
        if MetatileBehavior_IsBumpySlope((*playerObjEvent).currentMetatileBehavior) == 0 {
            *newDirection = direction;
            gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
            Bike_SetBikeStill();
            return ACRO_TRANS_WHEELIE_TO_NORMAL;
        }
    }
    if gPlayerAvatar.bikeFrameCounter >= 40 {
        *newDirection = direction;
        gPlayerAvatar.acroBikeState = ACRO_STATE_BUNNY_HOP;
        Bike_SetBikeStill();
        return ACRO_TRANS_WHEELIE_HOPPING_STANDING;
    }
    if *newDirection == direction {
        gPlayerAvatar.runningState = MOVING;
        gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_MOVING;
        Bike_SetBikeStill();
        return ACRO_TRANS_WHEELIE_MOVING;
    }
    if *newDirection == 0 {
        *newDirection = direction;
        return ACRO_TRANS_WHEELIE_IDLE;
    }
    gPlayerAvatar.runningState = TURN_DIRECTION;
    ACRO_TRANS_WHEELIE_IDLE
}
pub(crate) unsafe fn AcroBikeHandleInputBunnyHop(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let direction: u8 = GetPlayerMovementDirection();
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if heldKeys as i32 & B_BUTTON == 0 {
        Bike_SetBikeStill();
        if MetatileBehavior_IsBumpySlope((*playerObjEvent).currentMetatileBehavior) != 0 {
            gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_STANDING;
            return CheckMovementInputAcroBike(newDirection, newKeys, heldKeys);
        } else {
            *newDirection = direction;
            gPlayerAvatar.runningState = NOT_MOVING;
            gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
            return ACRO_TRANS_WHEELIE_TO_NORMAL;
        }
    }
    if *newDirection == DIR_NONE {
        *newDirection = direction;
        gPlayerAvatar.runningState = NOT_MOVING;
        return ACRO_TRANS_WHEELIE_HOPPING_STANDING;
    }
    if *newDirection != direction && gPlayerAvatar.runningState != MOVING {
        gPlayerAvatar.runningState = TURN_DIRECTION;
        return ACRO_TRANS_WHEELIE_HOPPING_STANDING;
    }
    gPlayerAvatar.runningState = MOVING;
    ACRO_TRANS_WHEELIE_HOPPING_MOVING
}
pub(crate) unsafe fn AcroBikeHandleInputWheelieMoving(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let direction: u8 = GetPlayerFacingDirection();
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if heldKeys as i32 & B_BUTTON == 0 {
        Bike_SetBikeStill();
        if MetatileBehavior_IsBumpySlope((*playerObjEvent).currentMetatileBehavior) == 0 {
            gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
            if *newDirection == DIR_NONE {
                *newDirection = direction;
                gPlayerAvatar.runningState = NOT_MOVING;
                return ACRO_TRANS_WHEELIE_TO_NORMAL;
            }
            if *newDirection != direction && gPlayerAvatar.runningState != MOVING {
                gPlayerAvatar.runningState = NOT_MOVING;
                return ACRO_TRANS_WHEELIE_TO_NORMAL;
            }
            gPlayerAvatar.runningState = MOVING;
            return ACRO_TRANS_WHEELIE_LOWERING_MOVING;
        }
        gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_STANDING;
        return CheckMovementInputAcroBike(newDirection, newKeys, heldKeys);
    }
    if *newDirection == DIR_NONE {
        *newDirection = direction;
        gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_STANDING;
        gPlayerAvatar.runningState = NOT_MOVING;
        Bike_SetBikeStill();
        return ACRO_TRANS_WHEELIE_IDLE;
    }
    if direction != *newDirection && gPlayerAvatar.runningState != MOVING {
        gPlayerAvatar.runningState = NOT_MOVING;
        return ACRO_TRANS_WHEELIE_IDLE;
    }
    gPlayerAvatar.runningState = MOVING;
    ACRO_TRANS_WHEELIE_MOVING
}
pub(crate) unsafe fn AcroBikeHandleInputSidewaysJump(
    ptr: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    (*playerObjEvent).set_facingDirectionLocked(0);
    SetObjectEventDirection(playerObjEvent, (*playerObjEvent).facingDirection() as u8);
    gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
    CheckMovementInputAcroBike(ptr, newKeys, heldKeys)
}
pub(crate) unsafe fn AcroBikeHandleInputTurnJump(ptr: *mut u8, newKeys: u16, heldKeys: u16) -> u8 {
    gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
    CheckMovementInputAcroBike(ptr, newKeys, heldKeys)
}
pub(crate) unsafe fn AcroBikeTransition_FaceDirection(direction: u8) {
    PlayerFaceDirection(direction);
}
pub(crate) unsafe fn AcroBikeTransition_TurnDirection(mut direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerFaceDirection(direction);
}
pub(crate) unsafe fn AcroBikeTransition_Moving(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        AcroBikeTransition_FaceDirection((*playerObjEvent).movementDirection() as u8);
        return;
    }
    let collision: u8 = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerJumpLedge(direction);
        } else if collision == COLLISION_OBJECT_EVENT
            && IsPlayerCollidingWithFarawayIslandMew(direction) != 0
        {
            PlayerOnBikeCollideWithFarawayIslandMew(direction);
        } else if !(COLLISION_STOP_SURFING..=COLLISION_ROTATING_GATE).contains(&collision) {
            PlayerOnBikeCollide(direction);
        }
    } else {
        PlayerRideWaterCurrent(direction);
    }
}
pub(crate) unsafe fn AcroBikeTransition_NormalToWheelie(mut direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerStartWheelie(direction);
}
pub(crate) unsafe fn AcroBikeTransition_WheelieToNormal(mut direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerEndWheelie(direction);
}
pub(crate) unsafe fn AcroBikeTransition_WheelieIdle(mut direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerIdleWheelie(direction);
}
pub(crate) unsafe fn AcroBikeTransition_WheelieHoppingStanding(mut direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerStandingHoppingWheelie(direction);
}
pub(crate) unsafe fn AcroBikeTransition_WheelieHoppingMoving(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        AcroBikeTransition_WheelieHoppingStanding((*playerObjEvent).movementDirection() as u8);
        return;
    }
    let collision: u8 = GetBikeCollision(direction);
    if collision != 0 && collision != COLLISION_WHEELIE_HOP {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerLedgeHoppingWheelie(direction);
            return;
        }
        if (COLLISION_STOP_SURFING..=COLLISION_ROTATING_GATE).contains(&collision) {
            return;
        }
        if collision < COLLISION_VERTICAL_RAIL {
            AcroBikeTransition_WheelieHoppingStanding(direction);
            return;
        }
    }
    PlayerMovingHoppingWheelie(direction);
}
pub(crate) unsafe fn AcroBikeTransition_SideJump(direction: u8) {
    let collision: u8 = GetBikeCollision(direction);
    if collision != 0 {
        if collision == COLLISION_PUSHED_BOULDER {
            return;
        }
        if collision < COLLISION_ISOLATED_VERTICAL_RAIL {
            AcroBikeTransition_TurnDirection(direction);
            return;
        }
        if WillPlayerCollideWithCollision(collision, direction) == FALSE {
            AcroBikeTransition_TurnDirection(direction);
            return;
        }
    }
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    PlaySE(SE_BIKE_HOP);
    (*playerObjEvent).set_facingDirectionLocked(1);
    PlayerSetAnimId(GetJumpMovementAction(direction as u32), COPY_MOVE_WALK);
}
pub(crate) unsafe fn AcroBikeTransition_TurnJump(direction: u8) {
    PlayerAcroTurnJump(direction);
}
pub(crate) unsafe fn AcroBikeTransition_WheelieMoving(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        PlayerIdleWheelie((*playerObjEvent).movementDirection() as u8);
        return;
    }
    let collision: u8 = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerLedgeHoppingWheelie(direction);
        } else if collision == COLLISION_WHEELIE_HOP {
            PlayerIdleWheelie(direction);
        } else if collision < COLLISION_STOP_SURFING {
            if MetatileBehavior_IsBumpySlope((*playerObjEvent).currentMetatileBehavior) != 0 {
                PlayerIdleWheelie(direction);
            } else {
                PlayerWheelieInPlace(direction);
            }
        }
        return;
    }
    PlayerWheelieMove(direction);
    gPlayerAvatar.runningState = MOVING;
}
pub(crate) unsafe fn AcroBikeTransition_WheelieRisingMoving(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        PlayerStartWheelie((*playerObjEvent).movementDirection() as u8);
        return;
    }
    let collision: u8 = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerLedgeHoppingWheelie(direction);
        } else if collision == COLLISION_WHEELIE_HOP {
            PlayerIdleWheelie(direction);
        } else if collision < COLLISION_STOP_SURFING {
            if MetatileBehavior_IsBumpySlope((*playerObjEvent).currentMetatileBehavior) != 0 {
                PlayerIdleWheelie(direction);
            } else {
                PlayerWheelieInPlace(direction);
            }
        }
        return;
    }
    PlayerPopWheelieWhileMoving(direction);
    gPlayerAvatar.runningState = MOVING;
}
pub(crate) unsafe fn AcroBikeTransition_WheelieLoweringMoving(direction: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        PlayerEndWheelie((*playerObjEvent).movementDirection() as u8);
        return;
    }
    let collision: u8 = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerJumpLedge(direction);
        } else if !(COLLISION_STOP_SURFING..=COLLISION_ROTATING_GATE).contains(&collision) {
            PlayerEndWheelie(direction);
        }
        return;
    }
    PlayerEndWheelieWhileMoving(direction);
}
pub unsafe fn Bike_TryAcroBikeHistoryUpdate(newKeys: u16, heldKeys: u16) {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_ACRO_BIKE as i32 != 0 {
        AcroBike_TryHistoryUpdate(newKeys, heldKeys);
    }
}
unsafe fn AcroBike_TryHistoryUpdate(newKeys: u16, heldKeys: u16) {
    let mut direction: u8 = Bike_DPadToDirection(heldKeys);
    if direction as u32 == gPlayerAvatar.directionHistory & 0xF {
        gPlayerAvatar.dirTimerHistory[0] = gPlayerAvatar.dirTimerHistory[0].saturating_add(1);
    } else {
        Bike_UpdateDirTimerHistory(direction);
        gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
    }
    direction = heldKeys as u8 & 15;
    if direction as u32 == gPlayerAvatar.abStartSelectHistory & 0xF {
        gPlayerAvatar.abStartSelectTimerHistory[0] =
            gPlayerAvatar.abStartSelectTimerHistory[0].saturating_add(1);
    } else {
        Bike_UpdateABStartSelectHistory(direction);
        gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
    }
}
unsafe fn HasPlayerInputTakenLongerThanList(
    dirTimerList: *mut u8,
    abStartSelectTimerList: *mut u8,
) -> u8 {
    let mut i: u8 = 0;
    while *dirTimerList.at(i) != 0 {
        if gPlayerAvatar.dirTimerHistory[i] > *dirTimerList.at(i) {
            return FALSE;
        }
        i += 1;
    }
    i = 0;
    while *abStartSelectTimerList.at(i) != 0 {
        if gPlayerAvatar.abStartSelectTimerHistory[i] > *abStartSelectTimerList.at(i) {
            return FALSE;
        }
        i += 1;
    }
    TRUE
}
unsafe fn AcroBike_GetJumpDirection() -> u8 {
    for i in 0..4u32 {
        let historyInputInfo: *mut BikeHistoryInputInfo =
            (&raw const sAcroBikeTricksList[i]).cast_mut();
        let mut dirHistory: u32 = gPlayerAvatar.directionHistory;
        let mut abStartSelectHistory: u32 = gPlayerAvatar.abStartSelectHistory;
        dirHistory &= (*historyInputInfo).dirHistoryMask;
        abStartSelectHistory &= (*historyInputInfo).abStartSelectHistoryMask;
        if dirHistory == (*historyInputInfo).dirHistoryMatch
            && abStartSelectHistory == (*historyInputInfo).abStartSelectHistoryMatch
            && HasPlayerInputTakenLongerThanList(
                (*historyInputInfo).dirTimerHistoryList,
                (*historyInputInfo).abStartSelectHistoryList,
            ) != 0
        {
            return (*historyInputInfo).direction as u8;
        }
    }
    0
}
unsafe fn Bike_UpdateDirTimerHistory(dir: u8) {
    gPlayerAvatar.directionHistory = gPlayerAvatar.directionHistory << 4 | dir as u32 & 0xF;
    let mut i: u8 = 7;
    while i != 0 {
        gPlayerAvatar.dirTimerHistory[i] = gPlayerAvatar.dirTimerHistory[i as i32 - 1];
        i -= 1;
    }
    gPlayerAvatar.dirTimerHistory[0] = 1;
}
unsafe fn Bike_UpdateABStartSelectHistory(input: u8) {
    gPlayerAvatar.abStartSelectHistory =
        gPlayerAvatar.abStartSelectHistory << 4 | input as u32 & 0xF;
    let mut i: u8 = 7;
    while i != 0 {
        gPlayerAvatar.abStartSelectTimerHistory[i] =
            gPlayerAvatar.abStartSelectTimerHistory[i as i32 - 1];
        i -= 1;
    }
    gPlayerAvatar.abStartSelectTimerHistory[0] = 1;
}
fn Bike_DPadToDirection(heldKeys: u16) -> u8 {
    if heldKeys as i32 & DPAD_UP != 0 {
        return DIR_NORTH;
    }
    if heldKeys as i32 & DPAD_DOWN != 0 {
        return DIR_SOUTH;
    }
    if heldKeys as i32 & DPAD_LEFT != 0 {
        return DIR_WEST;
    }
    if heldKeys as i32 & DPAD_RIGHT != 0 {
        return DIR_EAST;
    }
    DIR_NONE
}
unsafe fn GetBikeCollision(direction: u8) -> u8 {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut x: i16 = (*playerObjEvent).currentCoords.x;
    let mut y: i16 = (*playerObjEvent).currentCoords.y;
    MoveCoords(direction, &raw mut x, &raw mut y);
    let metatileBehavior: u8 = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    GetBikeCollisionAt(playerObjEvent, x, y, direction, metatileBehavior)
}
unsafe fn GetBikeCollisionAt(
    objectEvent: *mut ObjectEvent,
    x: i16,
    y: i16,
    direction: u8,
    metatileBehavior: u8,
) -> u8 {
    let mut collision: u8 =
        CheckForObjectEventCollision(objectEvent, x, y, direction, metatileBehavior);
    if collision > COLLISION_OBJECT_EVENT {
        return collision;
    }
    if collision == COLLISION_NONE && IsRunningDisallowedByMetatile(metatileBehavior) != 0 {
        collision = COLLISION_IMPASSABLE;
    }
    if collision != 0 {
        Bike_TryAdvanceCyclingRoadCollisions();
    }
    collision
}
pub unsafe fn RS_IsRunningDisallowed(tile: u8) -> u8 {
    if IsRunningDisallowedByMetatile(tile) != FALSE || gMapHeader.mapType == MAP_TYPE_INDOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsRunningDisallowedByMetatile(tile: u8) -> u8 {
    if MetatileBehavior_IsRunningDisallowed(tile) != 0 {
        return TRUE;
    }
    if MetatileBehavior_IsFortreeBridge(tile) != 0 && PlayerGetElevation() as i32 & 1 == 0 {
        return TRUE;
    }
    FALSE
}
fn Bike_TryAdvanceCyclingRoadCollisions() {
    if gBikeCyclingChallenge.get() != FALSE && gBikeCollisions.get() < 100 {
        gBikeCollisions.set(gBikeCollisions.get() + 1);
    }
}
unsafe fn CanBikeFaceDirOnMetatile(direction: u8, tile: u8) -> u8 {
    if direction == DIR_EAST || direction == DIR_WEST {
        if MetatileBehavior_IsIsolatedVerticalRail(tile) != 0
            || MetatileBehavior_IsVerticalRail(tile) != 0
        {
            return FALSE;
        }
    } else {
        if MetatileBehavior_IsIsolatedHorizontalRail(tile) != 0
            || MetatileBehavior_IsHorizontalRail(tile) != 0
        {
            return FALSE;
        }
    }
    TRUE
}
fn WillPlayerCollideWithCollision(newTileCollision: u8, direction: u8) -> u8 {
    if direction == DIR_NORTH || direction == DIR_SOUTH {
        if newTileCollision == COLLISION_ISOLATED_VERTICAL_RAIL
            || newTileCollision == COLLISION_VERTICAL_RAIL
        {
            return FALSE;
        }
    } else if newTileCollision == COLLISION_ISOLATED_HORIZONTAL_RAIL
        || newTileCollision == COLLISION_HORIZONTAL_RAIL
    {
        return FALSE;
    }
    TRUE
}
pub unsafe fn IsBikingDisallowedByPlayer() -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut tileBehavior: u8 = 0;
    if gPlayerAvatar.flags as i32 & 24 == 0 {
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
        if IsRunningDisallowedByMetatile(tileBehavior) == 0 {
            return FALSE;
        }
    }
    TRUE
}
pub unsafe fn IsPlayerNotUsingAcroBikeOnBumpySlope() -> u8 {
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_ACRO_BIKE) != 0
        && MetatileBehavior_IsBumpySlope(
            gObjectEvents[gPlayerAvatar.objectEventId].currentMetatileBehavior,
        ) != 0
    {
        return FALSE;
    } else {
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetOnOffBike(transitionFlags: u8) {
    gUnusedBikeCameraAheadPanback.set(FALSE);
    if gPlayerAvatar.flags as i32 & 6 != 0 {
        SetPlayerAvatarTransitionFlags(PLAYER_AVATAR_FLAG_ON_FOOT as u16);
        Overworld_ClearSavedMusic();
        Overworld_PlaySpecialMapMusic();
    } else {
        SetPlayerAvatarTransitionFlags(transitionFlags as u16);
        Overworld_SetSavedMusic(MUS_CYCLING);
        Overworld_ChangeMusicTo(MUS_CYCLING);
    }
}
pub unsafe fn BikeClearState(newDirHistory: i32, newAbStartHistory: i32) {
    gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
    gPlayerAvatar.newDirBackup = DIR_NONE;
    gPlayerAvatar.bikeFrameCounter = 0;
    gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
    gPlayerAvatar.directionHistory = newDirHistory as u32;
    gPlayerAvatar.abStartSelectHistory = newAbStartHistory as u32;
    for i in 0..8u8 {
        gPlayerAvatar.dirTimerHistory[i] = 0;
    }
    for i in 0..8u8 {
        gPlayerAvatar.abStartSelectTimerHistory[i] = 0;
    }
}
pub unsafe fn Bike_UpdateBikeCounterSpeed(counter: u8) {
    gPlayerAvatar.bikeFrameCounter = counter;
    gPlayerAvatar.bikeSpeed =
        gPlayerAvatar.bikeFrameCounter + (gPlayerAvatar.bikeFrameCounter >> 1);
}
unsafe fn Bike_SetBikeStill() {
    gPlayerAvatar.bikeFrameCounter = 0;
    gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
}
pub unsafe fn GetPlayerSpeed() -> i16 {
    let mut machSpeeds: CArray<i16, 3> = zeroed();
    memcpy(
        machSpeeds.as_mut_ptr() as *mut u8,
        sMachBikeSpeeds.as_ptr().cast_mut() as *mut u8,
        6,
    );
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_MACH_BIKE as i32 != 0 {
        return machSpeeds[gPlayerAvatar.bikeFrameCounter];
    } else if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_ACRO_BIKE as i32 != 0 {
        return PLAYER_SPEED_FASTER;
    } else if gPlayerAvatar.flags as i32 & 136 != 0 {
        return PLAYER_SPEED_FAST;
    } else {
        return PLAYER_SPEED_NORMAL;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn Bike_HandleBumpySlopeJump() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut tileBehavior: u8 = 0;
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_ACRO_BIKE as i32 != 0 {
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        tileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
        if MetatileBehavior_IsBumpySlope(tileBehavior) != 0 {
            gPlayerAvatar.acroBikeState = ACRO_STATE_WHEELIE_STANDING;
            PlayerUseAcroBikeOnBumpySlope(GetPlayerMovementDirection());
        }
    }
}
pub unsafe fn IsRunningDisallowed(metatile: u8) -> u32 {
    if gMapHeader.allowRunning() == 0 || IsRunningDisallowedByMetatile(metatile) == TRUE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
