//! Translated from `src/bike.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sMachBikeTransitions sMachBikeSpeedCallbacks sAcroBikeTransitions sAcroBikeInputHandlers sMachBikeSpeeds sAcroBikeJumpTimerList sAcroBikeTricksList

static sAcroBikeInputHandlers: Table<
    CArray<Option<unsafe extern "C" fn(*mut u8, u16, u16) -> u8>, 7>,
> = Table((&raw const crate::data::bike::sAcroBikeInputHandlers).cast());
static sAcroBikeTransitions: Table<CArray<Option<unsafe extern "C" fn(u8)>, 13>> =
    Table((&raw const crate::data::bike::sAcroBikeTransitions).cast());
static sAcroBikeTricksList: Table<CArray<BikeHistoryInputInfo, 4>> =
    Table((&raw const crate::data::bike::sAcroBikeTricksList).cast());
static sMachBikeSpeedCallbacks: Table<CArray<Option<unsafe extern "C" fn(u8)>, 3>> =
    Table((&raw const crate::data::bike::sMachBikeSpeedCallbacks).cast());
static sMachBikeSpeeds: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::bike::sMachBikeSpeeds).cast());
static sMachBikeTransitions: Table<CArray<Option<unsafe extern "C" fn(u8)>, 4>> =
    Table((&raw const crate::data::bike::sMachBikeTransitions).cast());

unsafe extern "C" {
    static mut gBikeCollisions: u8;
    static mut gBikeCyclingChallenge: u8;
    static mut gMapHeader: MapHeader;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gUnusedBikeCameraAheadPanback: u8;
    fn CheckForObjectEventCollision(a0: *mut ObjectEvent, a1: i16, a2: i16, a3: u8, a4: u8) -> u8;
    fn GetJumpMovementAction(a0: u32) -> u8;
    fn GetOppositeDirection(a0: u8) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetPlayerMovementDirection() -> u8;
    fn IsPlayerCollidingWithFarawayIslandMew(a0: u8) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsBumpySlope(a0: u8) -> u8;
    fn MetatileBehavior_IsFortreeBridge(a0: u8) -> u8;
    fn MetatileBehavior_IsHorizontalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsIsolatedHorizontalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsIsolatedVerticalRail(a0: u8) -> u8;
    fn MetatileBehavior_IsRunningDisallowed(a0: u8) -> u8;
    fn MetatileBehavior_IsVerticalRail(a0: u8) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn Overworld_ChangeMusicTo(a0: u16);
    fn Overworld_ClearSavedMusic();
    fn Overworld_PlaySpecialMapMusic();
    fn Overworld_SetSavedMusic(a0: u16);
    fn PlaySE(a0: u16);
    fn PlayerAcroTurnJump(a0: u8);
    fn PlayerEndWheelie(a0: u8);
    fn PlayerEndWheelieWhileMoving(a0: u8);
    fn PlayerFaceDirection(a0: u8);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn PlayerGetElevation() -> u8;
    fn PlayerIdleWheelie(a0: u8);
    fn PlayerJumpLedge(a0: u8);
    fn PlayerLedgeHoppingWheelie(a0: u8);
    fn PlayerMovingHoppingWheelie(a0: u8);
    fn PlayerOnBikeCollide(a0: u8);
    fn PlayerOnBikeCollideWithFarawayIslandMew(a0: u8);
    fn PlayerPopWheelieWhileMoving(a0: u8);
    fn PlayerRideWaterCurrent(a0: u8);
    fn PlayerSetAnimId(a0: u8, a1: u8);
    fn PlayerStandingHoppingWheelie(a0: u8);
    fn PlayerStartWheelie(a0: u8);
    fn PlayerTurnInPlace(a0: u8);
    fn PlayerUseAcroBikeOnBumpySlope(a0: u8);
    fn PlayerWheelieInPlace(a0: u8);
    fn PlayerWheelieMove(a0: u8);
    fn SetObjectEventDirection(a0: *mut ObjectEvent, a1: u8);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MovePlayerOnBike(direction: u8, newKeys: u16, heldKeys: u16) {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_MACH_BIKE as i32 != 0 {
        MovePlayerOnMachBike(direction, newKeys, heldKeys);
    } else {
        MovePlayerOnAcroBike(direction, newKeys, heldKeys);
    }
}
pub(crate) unsafe extern "C" fn MovePlayerOnMachBike(
    mut direction: u8,
    newKeys: u16,
    heldKeys: u16,
) {
    sMachBikeTransitions[GetMachBikeTransition(&raw mut direction)].unwrap_unchecked()(direction);
}
pub(crate) unsafe extern "C" fn GetMachBikeTransition(dirTraveling: *mut u8) -> u8 {
    let mut direction: u8 = GetPlayerMovementDirection();
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn MachBikeTransition_FaceDirection(direction: u8) {
    PlayerFaceDirection(direction);
    Bike_SetBikeStill();
}
pub(crate) unsafe extern "C" fn MachBikeTransition_TurnDirection(direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) != 0 {
        PlayerTurnInPlace(direction);
        Bike_SetBikeStill();
    } else {
        MachBikeTransition_FaceDirection((*playerObjEvent).facingDirection() as u8);
    }
}
pub(crate) unsafe extern "C" fn MachBikeTransition_TrySpeedUp(direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
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
                } else if collision < COLLISION_STOP_SURFING || collision > COLLISION_ROTATING_GATE
                {
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
pub(crate) unsafe extern "C" fn MachBikeTransition_TrySlowDown(direction: u8) {
    let mut collision: u8 = 0;
    if gPlayerAvatar.bikeSpeed != PLAYER_SPEED_STANDING {
        gPlayerAvatar.bikeFrameCounter = {
            gPlayerAvatar.bikeSpeed -= 1;
            gPlayerAvatar.bikeSpeed
        };
    }
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
            } else if collision < COLLISION_STOP_SURFING || collision > COLLISION_ROTATING_GATE {
                PlayerOnBikeCollide(direction);
            }
        }
    } else {
        sMachBikeSpeedCallbacks[gPlayerAvatar.bikeFrameCounter].unwrap_unchecked()(direction);
    }
}
pub(crate) unsafe extern "C" fn MovePlayerOnAcroBike(
    mut newDirection: u8,
    newKeys: u16,
    heldKeys: u16,
) {
    sAcroBikeTransitions[CheckMovementInputAcroBike(&raw mut newDirection, newKeys, heldKeys)]
        .unwrap_unchecked()(newDirection);
}
pub(crate) unsafe extern "C" fn CheckMovementInputAcroBike(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    return sAcroBikeInputHandlers[gPlayerAvatar.acroBikeState].unwrap_unchecked()(
        newDirection,
        newKeys,
        heldKeys,
    );
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputNormal(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let mut direction: u8 = GetPlayerMovementDirection();
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
    return ACRO_TRANS_MOVING;
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputTurning(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let mut direction: u8 = 0;
    *newDirection = gPlayerAvatar.newDirBackup;
    gPlayerAvatar.bikeFrameCounter += 1;
    if gPlayerAvatar.bikeFrameCounter > 6 {
        gPlayerAvatar.runningState = TURN_DIRECTION;
        gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
        Bike_SetBikeStill();
        return ACRO_TRANS_TURN_DIRECTION;
    }
    direction = GetPlayerMovementDirection();
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
    return ACRO_TRANS_FACE_DIRECTION;
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputWheelieStanding(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let mut direction: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = null_mut();
    direction = GetPlayerMovementDirection();
    playerObjEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
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
    return ACRO_TRANS_WHEELIE_IDLE;
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputBunnyHop(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let mut direction: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = null_mut();
    direction = GetPlayerMovementDirection();
    playerObjEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
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
    return ACRO_TRANS_WHEELIE_HOPPING_MOVING;
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputWheelieMoving(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let mut direction: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = null_mut();
    direction = GetPlayerFacingDirection();
    playerObjEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
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
    return ACRO_TRANS_WHEELIE_MOVING;
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputSidewaysJump(
    ptr: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    (*playerObjEvent).set_facingDirectionLocked(0);
    SetObjectEventDirection(playerObjEvent, (*playerObjEvent).facingDirection() as u8);
    gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
    return CheckMovementInputAcroBike(ptr, newKeys, heldKeys);
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputTurnJump(
    ptr: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
    return CheckMovementInputAcroBike(ptr, newKeys, heldKeys);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_FaceDirection(direction: u8) {
    PlayerFaceDirection(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_TurnDirection(mut direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerFaceDirection(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_Moving(direction: u8) {
    let mut collision: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        AcroBikeTransition_FaceDirection((*playerObjEvent).movementDirection() as u8);
        return;
    }
    collision = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerJumpLedge(direction);
        } else if collision == COLLISION_OBJECT_EVENT
            && IsPlayerCollidingWithFarawayIslandMew(direction) != 0
        {
            PlayerOnBikeCollideWithFarawayIslandMew(direction);
        } else if collision < COLLISION_STOP_SURFING || collision > COLLISION_ROTATING_GATE {
            PlayerOnBikeCollide(direction);
        }
    } else {
        PlayerRideWaterCurrent(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_NormalToWheelie(mut direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerStartWheelie(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieToNormal(mut direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerEndWheelie(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieIdle(mut direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerIdleWheelie(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieHoppingStanding(mut direction: u8) {
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        direction = (*playerObjEvent).movementDirection() as u8;
    }
    PlayerStandingHoppingWheelie(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieHoppingMoving(direction: u8) {
    let mut collision: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        AcroBikeTransition_WheelieHoppingStanding((*playerObjEvent).movementDirection() as u8);
        return;
    }
    collision = GetBikeCollision(direction);
    if collision != 0 && collision != COLLISION_WHEELIE_HOP {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerLedgeHoppingWheelie(direction);
            return;
        }
        if collision >= COLLISION_STOP_SURFING && collision <= COLLISION_ROTATING_GATE {
            return;
        }
        if collision < COLLISION_VERTICAL_RAIL {
            AcroBikeTransition_WheelieHoppingStanding(direction);
            return;
        }
    }
    PlayerMovingHoppingWheelie(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_SideJump(direction: u8) {
    let mut collision: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = null_mut();
    collision = GetBikeCollision(direction);
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
    playerObjEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    PlaySE(SE_BIKE_HOP);
    (*playerObjEvent).set_facingDirectionLocked(1);
    PlayerSetAnimId(GetJumpMovementAction(direction as u32), COPY_MOVE_WALK);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_TurnJump(direction: u8) {
    PlayerAcroTurnJump(direction);
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieMoving(direction: u8) {
    let mut collision: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        PlayerIdleWheelie((*playerObjEvent).movementDirection() as u8);
        return;
    }
    collision = GetBikeCollision(direction);
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
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieRisingMoving(direction: u8) {
    let mut collision: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        PlayerStartWheelie((*playerObjEvent).movementDirection() as u8);
        return;
    }
    collision = GetBikeCollision(direction);
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
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieLoweringMoving(direction: u8) {
    let mut collision: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if CanBikeFaceDirOnMetatile(direction, (*playerObjEvent).currentMetatileBehavior) == 0 {
        PlayerEndWheelie((*playerObjEvent).movementDirection() as u8);
        return;
    }
    collision = GetBikeCollision(direction);
    if collision > 0 && collision < COLLISION_VERTICAL_RAIL {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerJumpLedge(direction);
        } else if collision < COLLISION_STOP_SURFING || collision > COLLISION_ROTATING_GATE {
            PlayerEndWheelie(direction);
        }
        return;
    }
    PlayerEndWheelieWhileMoving(direction);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bike_TryAcroBikeHistoryUpdate(newKeys: u16, heldKeys: u16) {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_ACRO_BIKE as i32 != 0 {
        AcroBike_TryHistoryUpdate(newKeys, heldKeys);
    }
}
pub(crate) unsafe extern "C" fn AcroBike_TryHistoryUpdate(newKeys: u16, heldKeys: u16) {
    let mut direction: u8 = Bike_DPadToDirection(heldKeys);
    if direction as u32 == gPlayerAvatar.directionHistory & 0xF {
        if gPlayerAvatar.dirTimerHistory[0] < 0xFF {
            gPlayerAvatar.dirTimerHistory[0] += 1;
        }
    } else {
        Bike_UpdateDirTimerHistory(direction);
        gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
    }
    direction = heldKeys as u8 & 15;
    if direction as u32 == gPlayerAvatar.abStartSelectHistory & 0xF {
        if gPlayerAvatar.abStartSelectTimerHistory[0] < 0xFF {
            gPlayerAvatar.abStartSelectTimerHistory[0] += 1;
        }
    } else {
        Bike_UpdateABStartSelectHistory(direction);
        gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
    }
}
pub(crate) unsafe extern "C" fn HasPlayerInputTakenLongerThanList(
    dirTimerList: *mut u8,
    abStartSelectTimerList: *mut u8,
) -> u8 {
    let mut i: u8 = 0;
    i = 0;
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn AcroBike_GetJumpDirection() -> u8 {
    let mut i: u32 = 0;
    i = 0;
    while i < 4 {
        let mut historyInputInfo: *mut BikeHistoryInputInfo =
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
        i += 1;
    }
    return 0;
}
pub(crate) unsafe extern "C" fn Bike_UpdateDirTimerHistory(dir: u8) {
    let mut i: u8 = 0;
    gPlayerAvatar.directionHistory = gPlayerAvatar.directionHistory << 4 | dir as u32 & 0xF;
    i = 7;
    while i != 0 {
        gPlayerAvatar.dirTimerHistory[i] = gPlayerAvatar.dirTimerHistory[i as i32 - 1];
        i -= 1;
    }
    gPlayerAvatar.dirTimerHistory[0] = 1;
}
pub(crate) unsafe extern "C" fn Bike_UpdateABStartSelectHistory(input: u8) {
    let mut i: u8 = 0;
    gPlayerAvatar.abStartSelectHistory =
        gPlayerAvatar.abStartSelectHistory << 4 | input as u32 & 0xF;
    i = 7;
    while i != 0 {
        gPlayerAvatar.abStartSelectTimerHistory[i] =
            gPlayerAvatar.abStartSelectTimerHistory[i as i32 - 1];
        i -= 1;
    }
    gPlayerAvatar.abStartSelectTimerHistory[0] = 1;
}
pub(crate) unsafe extern "C" fn Bike_DPadToDirection(heldKeys: u16) -> u8 {
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
    return DIR_NONE;
}
pub(crate) unsafe extern "C" fn GetBikeCollision(direction: u8) -> u8 {
    let mut metatileBehavior: u8 = 0;
    let mut playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut x: i16 = (*playerObjEvent).currentCoords.x;
    let mut y: i16 = (*playerObjEvent).currentCoords.y;
    MoveCoords(direction, &raw mut x, &raw mut y);
    metatileBehavior = MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8;
    return GetBikeCollisionAt(playerObjEvent, x, y, direction, metatileBehavior);
}
pub(crate) unsafe extern "C" fn GetBikeCollisionAt(
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
    return collision;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RS_IsRunningDisallowed(tile: u8) -> u8 {
    if IsRunningDisallowedByMetatile(tile) != FALSE || gMapHeader.mapType == MAP_TYPE_INDOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsRunningDisallowedByMetatile(tile: u8) -> u8 {
    if MetatileBehavior_IsRunningDisallowed(tile) != 0 {
        return TRUE;
    }
    if MetatileBehavior_IsFortreeBridge(tile) != 0 && PlayerGetElevation() as i32 & 1 == 0 {
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Bike_TryAdvanceCyclingRoadCollisions() {
    if gBikeCyclingChallenge != FALSE && gBikeCollisions < 100 {
        gBikeCollisions += 1;
    }
}
pub(crate) unsafe extern "C" fn CanBikeFaceDirOnMetatile(direction: u8, tile: u8) -> u8 {
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
    return TRUE;
}
pub(crate) unsafe extern "C" fn WillPlayerCollideWithCollision(
    newTileCollision: u8,
    direction: u8,
) -> u8 {
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBikingDisallowedByPlayer() -> u8 {
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
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerNotUsingAcroBikeOnBumpySlope() -> u8 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetOnOffBike(transitionFlags: u8) {
    gUnusedBikeCameraAheadPanback = FALSE;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BikeClearState(newDirHistory: i32, newAbStartHistory: i32) {
    let mut i: u8 = 0;
    gPlayerAvatar.acroBikeState = ACRO_STATE_NORMAL;
    gPlayerAvatar.newDirBackup = DIR_NONE;
    gPlayerAvatar.bikeFrameCounter = 0;
    gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
    gPlayerAvatar.directionHistory = newDirHistory as u32;
    gPlayerAvatar.abStartSelectHistory = newAbStartHistory as u32;
    i = 0;
    while i < 8 {
        gPlayerAvatar.dirTimerHistory[i] = 0;
        i += 1;
    }
    i = 0;
    while i < 8 {
        gPlayerAvatar.abStartSelectTimerHistory[i] = 0;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bike_UpdateBikeCounterSpeed(counter: u8) {
    gPlayerAvatar.bikeFrameCounter = counter;
    gPlayerAvatar.bikeSpeed =
        gPlayerAvatar.bikeFrameCounter + (gPlayerAvatar.bikeFrameCounter >> 1);
}
pub(crate) unsafe extern "C" fn Bike_SetBikeStill() {
    gPlayerAvatar.bikeFrameCounter = 0;
    gPlayerAvatar.bikeSpeed = PLAYER_SPEED_STANDING;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerSpeed() -> i16 {
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bike_HandleBumpySlopeJump() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRunningDisallowed(metatile: u8) -> u32 {
    if gMapHeader.allowRunning() == 0 || IsRunningDisallowedByMetatile(metatile) == TRUE {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
