//! Translated from `src/bike.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sMachBikeTransitions sMachBikeSpeedCallbacks sAcroBikeTransitions sAcroBikeInputHandlers sMachBikeSpeeds sAcroBikeJumpTimerList sAcroBikeTricksList
#[allow(unused_imports)]
use crate::data::bike::*;

unsafe extern "C" {
    static mut gBikeCollisions: u8;
    static mut gBikeCyclingChallenge: u8;
    static mut gMapHeader: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gUnusedBikeCameraAheadPanback: u8;
    fn CheckForObjectEventCollision(a0: *mut u8, a1: i16, a2: i16, a3: u8, a4: u8) -> u8;
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
    fn SetObjectEventDirection(a0: *mut u8, a1: u8);
    fn SetPlayerAvatarTransitionFlags(a0: u16);
    fn TestPlayerAvatarFlags(a0: u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MovePlayerOnBike(direction: u8, newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            MovePlayerOnMachBike(direction, newKeys, heldKeys);
        } else {
            MovePlayerOnAcroBike(direction, newKeys, heldKeys);
        }
    }
}
pub(crate) unsafe extern "C" fn MovePlayerOnMachBike(direction: u8, newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        (((((&raw const sMachBikeTransitions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .wrapping_offset(((GetMachBikeTransition(&raw mut direction)) as i32) as isize))
        .read())
        .unwrap_unchecked()(direction);
    }
}
pub(crate) unsafe extern "C" fn GetMachBikeTransition(dirTraveling: *mut u8) -> u8 {
    unsafe {
        let mut dirTraveling = dirTraveling;
        let mut direction: u8 = GetPlayerMovementDirection();
        if (((dirTraveling).read()) as i32) == 0i32 {
            (dirTraveling).write(direction);
            if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).read()) as i32) == 0i32
            {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
                return 0u8;
            }
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
            return 3u8;
        }
        if ((((dirTraveling).read()) as i32) != ((direction) as i32))
            && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32) != 2i32)
        {
            if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).read()) as i32) != 0i32
            {
                (dirTraveling).write(direction);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
                return 3u8;
            }
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(1u8);
            return 1u8;
        } else {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
            return 2u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn MachBikeTransition_FaceDirection(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerFaceDirection(direction);
        Bike_SetBikeStill();
    }
}
pub(crate) unsafe extern "C" fn MachBikeTransition_TurnDirection(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read())) != 0 {
            PlayerTurnInPlace(direction);
            Bike_SetBikeStill();
        } else {
            MachBikeTransition_FaceDirection(
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn MachBikeTransition_TrySpeedUp(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        let mut collision: u8 = 0u8;
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            if ((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).read()) != 0 {
                MachBikeTransition_TrySlowDown(
                    ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16)
                        as u8),
                );
            } else {
                MachBikeTransition_FaceDirection(
                    ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16)
                        as u8),
                );
            }
        } else {
            collision = GetBikeCollision(direction);
            if (((collision) as i32) > 0i32) && (((collision) as i32) < 12i32) {
                if ((collision) as i32) == 6i32 {
                    PlayerJumpLedge(direction);
                } else {
                    Bike_SetBikeStill();
                    if (((collision) as i32) == 4i32)
                        && ((IsPlayerCollidingWithFarawayIslandMew(direction)) != 0)
                    {
                        PlayerOnBikeCollideWithFarawayIslandMew(direction);
                    } else {
                        if (((collision) as i32) < 5i32) || (((collision) as i32) > 8i32) {
                            PlayerOnBikeCollide(direction);
                        }
                    }
                }
            } else {
                (((((&raw const sMachBikeSpeedCallbacks)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn(u8)>>())
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                        as isize,
                ))
                .read())
                .unwrap_unchecked()(direction);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(
                    (((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read())
                                as i32)
                                >> 1),
                        )) as u8),
                );
                if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                    < 2i32
                {
                    let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10);
                    (__p1).write(((__p1).read()).wrapping_add(1));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn MachBikeTransition_TrySlowDown(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).read()) as i32) != 0i32 {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).write({
                let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            });
        }
        collision = GetBikeCollision(direction);
        if (((collision) as i32) > 0i32) && (((collision) as i32) < 12i32) {
            if ((collision) as i32) == 6i32 {
                PlayerJumpLedge(direction);
            } else {
                Bike_SetBikeStill();
                if (((collision) as i32) == 4i32)
                    && ((IsPlayerCollidingWithFarawayIslandMew(direction)) != 0)
                {
                    PlayerOnBikeCollideWithFarawayIslandMew(direction);
                } else {
                    if (((collision) as i32) < 5i32) || (((collision) as i32) > 8i32) {
                        PlayerOnBikeCollide(direction);
                    }
                }
            }
        } else {
            (((((&raw const sMachBikeSpeedCallbacks)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .cast::<Option<unsafe extern "C" fn(u8)>>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                    as isize,
            ))
            .read())
            .unwrap_unchecked()(direction);
        }
    }
}
pub(crate) unsafe extern "C" fn MovePlayerOnAcroBike(
    newDirection: u8,
    newKeys: u16,
    heldKeys: u16,
) {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        (((((&raw const sAcroBikeTransitions)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .cast::<Option<unsafe extern "C" fn(u8)>>())
        .wrapping_offset(
            ((CheckMovementInputAcroBike(&raw mut newDirection, newKeys, heldKeys)) as i32)
                as isize,
        ))
        .read())
        .unwrap_unchecked()(newDirection);
    }
}
pub(crate) unsafe extern "C" fn CheckMovementInputAcroBike(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        return (((((&raw const sAcroBikeInputHandlers)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(*mut u8, u16, u16) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(*mut u8, u16, u16) -> u8>>())
        .wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).read()) as i32) as isize,
        ))
        .read())
        .unwrap_unchecked()(newDirection, newKeys, heldKeys);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputNormal(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut direction: u8 = GetPlayerMovementDirection();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).write(0u8);
        if (((newDirection).read()) as i32) == 0i32 {
            if (((newKeys) as i32) & 2i32) != 0 {
                (newDirection).write(direction);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(2u8);
                return 3u8;
            } else {
                (newDirection).write(direction);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
                return 0u8;
            }
        }
        if (((((newDirection).read()) as i32) == ((direction) as i32))
            && ((((heldKeys) as i32) & 2i32) != 0))
            && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).read()) as i32)
                == 0i32)
        {
            let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11);
            (__p1).write(((__p1).read()).wrapping_add(1));
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(4u8);
            return 11u8;
        }
        if ((((newDirection).read()) as i32) != ((direction) as i32))
            && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32) != 2i32)
        {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(1u8);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(9)).write((newDirection).read());
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
            return CheckMovementInputAcroBike(newDirection, newKeys, heldKeys);
        }
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
        return 2u8;
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputTurning(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut direction: u8 = 0u8;
        (newDirection).write((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(9)).read());
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32) > 6i32 {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(1u8);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
            Bike_SetBikeStill();
            return 1u8;
        }
        direction = GetPlayerMovementDirection();
        if (((newDirection).read()) as i32) == ((AcroBike_GetJumpDirection()) as i32) {
            Bike_SetBikeStill();
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(1u8);
            if (((newDirection).read()) as i32) == ((GetOppositeDirection(direction)) as i32) {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(6u8);
                return 9u8;
            } else {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(5u8);
                return 8u8;
            }
        }
        (newDirection).write(direction);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputWheelieStanding(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut direction: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = core::ptr::null_mut();
        direction = GetPlayerMovementDirection();
        playerObjEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
        if (((heldKeys) as i32) & 2i32) != 0 {
            let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).write(0u8);
            if !((MetatileBehavior_IsBumpySlope(((playerObjEvent).wrapping_add(30)).read())) != 0) {
                (newDirection).write(direction);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
                Bike_SetBikeStill();
                return 4u8;
            }
        }
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32) >= 40i32 {
            (newDirection).write(direction);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(3u8);
            Bike_SetBikeStill();
            return 6u8;
        }
        if (((newDirection).read()) as i32) == ((direction) as i32) {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(4u8);
            Bike_SetBikeStill();
            return 10u8;
        }
        if (((newDirection).read()) as i32) == 0i32 {
            (newDirection).write(direction);
            return 5u8;
        }
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(1u8);
        return 5u8;
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputBunnyHop(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut direction: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = core::ptr::null_mut();
        direction = GetPlayerMovementDirection();
        playerObjEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if !((((heldKeys) as i32) & 2i32) != 0) {
            Bike_SetBikeStill();
            if (MetatileBehavior_IsBumpySlope(((playerObjEvent).wrapping_add(30)).read())) != 0 {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(2u8);
                return CheckMovementInputAcroBike(newDirection, newKeys, heldKeys);
            } else {
                (newDirection).write(direction);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
                return 4u8;
            }
        }
        if (((newDirection).read()) as i32) == 0i32 {
            (newDirection).write(direction);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
            return 6u8;
        }
        if ((((newDirection).read()) as i32) != ((direction) as i32))
            && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32) != 2i32)
        {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(1u8);
            return 6u8;
        }
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
        return 7u8;
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputWheelieMoving(
    newDirection: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut newDirection = newDirection;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut direction: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = core::ptr::null_mut();
        direction = GetPlayerFacingDirection();
        playerObjEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if !((((heldKeys) as i32) & 2i32) != 0) {
            Bike_SetBikeStill();
            if !((MetatileBehavior_IsBumpySlope(((playerObjEvent).wrapping_add(30)).read())) != 0) {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
                if (((newDirection).read()) as i32) == 0i32 {
                    (newDirection).write(direction);
                    (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
                    return 4u8;
                }
                if ((((newDirection).read()) as i32) != ((direction) as i32))
                    && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32)
                        != 2i32)
                {
                    (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
                    return 4u8;
                }
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
                return 12u8;
            }
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(2u8);
            return CheckMovementInputAcroBike(newDirection, newKeys, heldKeys);
        }
        if (((newDirection).read()) as i32) == 0i32 {
            (newDirection).write(direction);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(2u8);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
            Bike_SetBikeStill();
            return 5u8;
        }
        if (((direction) as i32) != (((newDirection).read()) as i32))
            && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32) != 2i32)
        {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
            return 5u8;
        }
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
        return 10u8;
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputSidewaysJump(
    ptr: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut ptr = ptr;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        crate::c::bf_write((playerObjEvent).wrapping_add(1), 1, 1, (0u32) as i32);
        SetObjectEventDirection(
            playerObjEvent,
            ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
        );
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
        return CheckMovementInputAcroBike(ptr, newKeys, heldKeys);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeHandleInputTurnJump(
    ptr: *mut u8,
    newKeys: u16,
    heldKeys: u16,
) -> u8 {
    unsafe {
        let mut ptr = ptr;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
        return CheckMovementInputAcroBike(ptr, newKeys, heldKeys);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_FaceDirection(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerFaceDirection(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_TurnDirection(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            direction =
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8);
        }
        PlayerFaceDirection(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_Moving(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            AcroBikeTransition_FaceDirection(
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            );
            return;
        }
        collision = GetBikeCollision(direction);
        if (((collision) as i32) > 0i32) && (((collision) as i32) < 12i32) {
            if ((collision) as i32) == 6i32 {
                PlayerJumpLedge(direction);
            } else {
                if (((collision) as i32) == 4i32)
                    && ((IsPlayerCollidingWithFarawayIslandMew(direction)) != 0)
                {
                    PlayerOnBikeCollideWithFarawayIslandMew(direction);
                } else {
                    if (((collision) as i32) < 5i32) || (((collision) as i32) > 8i32) {
                        PlayerOnBikeCollide(direction);
                    }
                }
            }
        } else {
            PlayerRideWaterCurrent(direction);
        }
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_NormalToWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            direction =
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8);
        }
        PlayerStartWheelie(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieToNormal(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            direction =
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8);
        }
        PlayerEndWheelie(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieIdle(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            direction =
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8);
        }
        PlayerIdleWheelie(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieHoppingStanding(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            direction =
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8);
        }
        PlayerStandingHoppingWheelie(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieHoppingMoving(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            AcroBikeTransition_WheelieHoppingStanding(
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            );
            return;
        }
        collision = GetBikeCollision(direction);
        if ((collision) != 0) && (((collision) as i32) != 9i32) {
            if ((collision) as i32) == 6i32 {
                PlayerLedgeHoppingWheelie(direction);
                return;
            }
            if (((collision) as i32) >= 5i32) && (((collision) as i32) <= 8i32) {
                return;
            }
            if ((collision) as i32) < 12i32 {
                AcroBikeTransition_WheelieHoppingStanding(direction);
                return;
            }
        }
        PlayerMovingHoppingWheelie(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_SideJump(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = core::ptr::null_mut();
        collision = GetBikeCollision(direction);
        if (collision) != 0 {
            if ((collision) as i32) == 7i32 {
                return;
            }
            if ((collision) as i32) < 10i32 {
                AcroBikeTransition_TurnDirection(direction);
                return;
            }
            if ((WillPlayerCollideWithCollision(collision, direction)) as i32) == 0i32 {
                AcroBikeTransition_TurnDirection(direction);
                return;
            }
        }
        playerObjEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        PlaySE(34u16);
        crate::c::bf_write((playerObjEvent).wrapping_add(1), 1, 1, (1u32) as i32);
        PlayerSetAnimId(GetJumpMovementAction(((direction) as u32)), 2u8);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_TurnJump(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerAcroTurnJump(direction);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieMoving(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            PlayerIdleWheelie(
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            );
            return;
        }
        collision = GetBikeCollision(direction);
        if (((collision) as i32) > 0i32) && (((collision) as i32) < 12i32) {
            if ((collision) as i32) == 6i32 {
                PlayerLedgeHoppingWheelie(direction);
            } else {
                if ((collision) as i32) == 9i32 {
                    PlayerIdleWheelie(direction);
                } else {
                    if ((collision) as i32) < 5i32 {
                        if (MetatileBehavior_IsBumpySlope(
                            ((playerObjEvent).wrapping_add(30)).read(),
                        )) != 0
                        {
                            PlayerIdleWheelie(direction);
                        } else {
                            PlayerWheelieInPlace(direction);
                        }
                    }
                }
            }
            return;
        }
        PlayerWheelieMove(direction);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieRisingMoving(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            PlayerStartWheelie(
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            );
            return;
        }
        collision = GetBikeCollision(direction);
        if (((collision) as i32) > 0i32) && (((collision) as i32) < 12i32) {
            if ((collision) as i32) == 6i32 {
                PlayerLedgeHoppingWheelie(direction);
            } else {
                if ((collision) as i32) == 9i32 {
                    PlayerIdleWheelie(direction);
                } else {
                    if ((collision) as i32) < 5i32 {
                        if (MetatileBehavior_IsBumpySlope(
                            ((playerObjEvent).wrapping_add(30)).read(),
                        )) != 0
                        {
                            PlayerIdleWheelie(direction);
                        } else {
                            PlayerWheelieInPlace(direction);
                        }
                    }
                }
            }
            return;
        }
        PlayerPopWheelieWhileMoving(direction);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(2u8);
    }
}
pub(crate) unsafe extern "C" fn AcroBikeTransition_WheelieLoweringMoving(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut collision: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((CanBikeFaceDirOnMetatile(direction, ((playerObjEvent).wrapping_add(30)).read()))
            as i32)
            == 0i32
        {
            PlayerEndWheelie(
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            );
            return;
        }
        collision = GetBikeCollision(direction);
        if (((collision) as i32) > 0i32) && (((collision) as i32) < 12i32) {
            if ((collision) as i32) == 6i32 {
                PlayerJumpLedge(direction);
            } else {
                if (((collision) as i32) < 5i32) || (((collision) as i32) > 8i32) {
                    PlayerEndWheelie(direction);
                }
            }
            return;
        }
        PlayerEndWheelieWhileMoving(direction);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bike_TryAcroBikeHistoryUpdate(newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 4i32) != 0 {
            AcroBike_TryHistoryUpdate(newKeys, heldKeys);
        }
    }
}
pub(crate) unsafe extern "C" fn AcroBike_TryHistoryUpdate(newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut direction: u8 = Bike_DPadToDirection(heldKeys);
        if ((direction) as u32)
            == ((((&raw mut gPlayerAvatar).cast::<u8>())
                .wrapping_add(12)
                .cast::<u32>())
            .read()
                & 15u32)
        {
            if ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20)).cast::<u8>()).read())
                as i32)
                < 255i32
            {
                let __p1 = (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20)).cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
            }
        } else {
            Bike_UpdateDirTimerHistory(direction);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(0u8);
        }
        direction = ((((heldKeys) as i32) & 15i32) as u8);
        if ((direction) as u32)
            == ((((&raw mut gPlayerAvatar).cast::<u8>())
                .wrapping_add(16)
                .cast::<u32>())
            .read()
                & 15u32)
        {
            if ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28)).cast::<u8>()).read())
                as i32)
                < 255i32
            {
                let __p2 = (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28)).cast::<u8>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        } else {
            Bike_UpdateABStartSelectHistory(direction);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn HasPlayerInputTakenLongerThanList(
    dirTimerList: *mut u8,
    abStartSelectTimerList: *mut u8,
) -> u8 {
    unsafe {
        let mut dirTimerList = dirTimerList;
        let mut abStartSelectTimerList = abStartSelectTimerList;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((((dirTimerList).wrapping_offset(((i) as i32) as isize)).read()) as i32)
                    != 0i32)
                {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20))
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        > ((((dirTimerList).wrapping_offset(((i) as i32) as isize)).read()) as i32)
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
                if !(((((abStartSelectTimerList).wrapping_offset(((i) as i32) as isize)).read())
                    as i32)
                    != 0i32)
                {
                    break 'l3;
                }
                'l4: {
                    if (((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28))
                        .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read()) as i32)
                        > ((((abStartSelectTimerList).wrapping_offset(((i) as i32) as isize))
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
pub(crate) unsafe extern "C" fn AcroBike_GetJumpDirection() -> u8 {
    unsafe {
        let mut i: u32 = 0u32;
        {
            i = 0u32;
            'l1: loop {
                if !(i < crate::c::div_u32(112u32, 28u32)) {
                    break 'l1;
                }
                'l2: {
                    let mut historyInputInfo: *mut u8 =
                        (((&raw const sAcroBikeTricksList).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 28);
                    let mut dirHistory: u32 = (((&raw mut gPlayerAvatar).cast::<u8>())
                        .wrapping_add(12)
                        .cast::<u32>())
                    .read();
                    let mut abStartSelectHistory: u32 = (((&raw mut gPlayerAvatar).cast::<u8>())
                        .wrapping_add(16)
                        .cast::<u32>())
                    .read();
                    dirHistory =
                        (dirHistory & ((historyInputInfo).wrapping_add(8).cast::<u32>()).read());
                    abStartSelectHistory = (abStartSelectHistory
                        & ((historyInputInfo).wrapping_add(12).cast::<u32>()).read());
                    if ((dirHistory == ((historyInputInfo).cast::<u32>()).read())
                        && (abStartSelectHistory
                            == ((historyInputInfo).wrapping_add(4).cast::<u32>()).read()))
                        && ((HasPlayerInputTakenLongerThanList(
                            ((historyInputInfo).wrapping_add(16).cast::<*mut u8>()).read(),
                            ((historyInputInfo).wrapping_add(20).cast::<*mut u8>()).read(),
                        )) != 0)
                    {
                        return ((((historyInputInfo).wrapping_add(24).cast::<u32>()).read())
                            as u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Bike_UpdateDirTimerHistory(dir: u8) {
    unsafe {
        let mut dir = dir;
        let mut i: u8 = 0u8;
        (((&raw mut gPlayerAvatar).cast::<u8>())
            .wrapping_add(12)
            .cast::<u32>())
        .write(
            (((((&raw mut gPlayerAvatar).cast::<u8>())
                .wrapping_add(12)
                .cast::<u32>())
            .read()
                << 4)
                | ((((dir) as i32) & 15i32) as u32)),
        );
        {
            i = (((crate::c::div_u32(8u32, 1u32)).wrapping_sub(1u32)) as u8);
            'l1: loop {
                if !(((i) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20)).cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_sub(1);
            }
        }
        ((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20)).cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Bike_UpdateABStartSelectHistory(input: u8) {
    unsafe {
        let mut input = input;
        let mut i: u8 = 0u8;
        (((&raw mut gPlayerAvatar).cast::<u8>())
            .wrapping_add(16)
            .cast::<u32>())
        .write(
            (((((&raw mut gPlayerAvatar).cast::<u8>())
                .wrapping_add(16)
                .cast::<u32>())
            .read()
                << 4)
                | ((((input) as i32) & 15i32) as u32)),
        );
        {
            i = (((crate::c::div_u32(8u32, 1u32)).wrapping_sub(1u32)) as u8);
            'l1: loop {
                if !(((i) as i32) != 0i32) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28)).cast::<u8>())
                            .wrapping_offset((((i) as i32).wrapping_sub(1i32)) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_sub(1);
            }
        }
        ((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28)).cast::<u8>()).write(1u8);
    }
}
pub(crate) unsafe extern "C" fn Bike_DPadToDirection(heldKeys: u16) -> u8 {
    unsafe {
        let mut heldKeys = heldKeys;
        if (((heldKeys) as i32) & 64i32) != 0 {
            return 2u8;
        }
        if (((heldKeys) as i32) & 128i32) != 0 {
            return 1u8;
        }
        if (((heldKeys) as i32) & 32i32) != 0 {
            return 3u8;
        }
        if (((heldKeys) as i32) & 16i32) != 0 {
            return 4u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetBikeCollision(direction: u8) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut metatileBehavior: u8 = 0u8;
        let mut playerObjEvent: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        let mut x: i16 = (((playerObjEvent).wrapping_add(16)).cast::<i16>()).read();
        let mut y: i16 = (((playerObjEvent).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read();
        MoveCoords(direction, &raw mut x, &raw mut y);
        metatileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
        return GetBikeCollisionAt(playerObjEvent, x, y, direction, metatileBehavior);
    }
}
pub(crate) unsafe extern "C" fn GetBikeCollisionAt(
    objectEvent: *mut u8,
    x: i16,
    y: i16,
    direction: u8,
    metatileBehavior: u8,
) -> u8 {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut x = x;
        let mut y = y;
        let mut direction = direction;
        let mut metatileBehavior = metatileBehavior;
        let mut collision: u8 =
            CheckForObjectEventCollision(objectEvent, x, y, direction, metatileBehavior);
        if ((collision) as i32) > 4i32 {
            return collision;
        }
        if (((collision) as i32) == 0i32)
            && ((IsRunningDisallowedByMetatile(metatileBehavior)) != 0)
        {
            collision = 2u8;
        }
        if (collision) != 0 {
            Bike_TryAdvanceCyclingRoadCollisions();
        }
        return collision;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RS_IsRunningDisallowed(tile: u8) -> u8 {
    unsafe {
        let mut tile = tile;
        if (((IsRunningDisallowedByMetatile(tile)) as i32) != 0i32)
            || ((((((&raw mut gMapHeader).cast::<u8>()).wrapping_add(23)).read()) as i32) == 8i32)
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
pub(crate) unsafe extern "C" fn IsRunningDisallowedByMetatile(tile: u8) -> u8 {
    unsafe {
        let mut tile = tile;
        if (MetatileBehavior_IsRunningDisallowed(tile)) != 0 {
            return 1u8;
        }
        if ((MetatileBehavior_IsFortreeBridge(tile)) != 0)
            && ((((PlayerGetElevation()) as i32) & 1i32) == 0i32)
        {
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Bike_TryAdvanceCyclingRoadCollisions() {
    unsafe {
        if (((((&raw mut gBikeCyclingChallenge).cast::<u8>()).read()) as i32) != 0i32)
            && (((((&raw mut gBikeCollisions).cast::<u8>()).read()) as i32) < 100i32)
        {
            let __p1 = (&raw mut gBikeCollisions).cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn CanBikeFaceDirOnMetatile(direction: u8, tile: u8) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut tile = tile;
        if (((direction) as i32) == 4i32) || (((direction) as i32) == 3i32) {
            if ((MetatileBehavior_IsIsolatedVerticalRail(tile)) != 0)
                || ((MetatileBehavior_IsVerticalRail(tile)) != 0)
            {
                return 0u8;
            }
        } else {
            if ((MetatileBehavior_IsIsolatedHorizontalRail(tile)) != 0)
                || ((MetatileBehavior_IsHorizontalRail(tile)) != 0)
            {
                return 0u8;
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WillPlayerCollideWithCollision(
    newTileCollision: u8,
    direction: u8,
) -> u8 {
    unsafe {
        let mut newTileCollision = newTileCollision;
        let mut direction = direction;
        if (((direction) as i32) == 2i32) || (((direction) as i32) == 1i32) {
            if (((newTileCollision) as i32) == 10i32) || (((newTileCollision) as i32) == 12i32) {
                return 0u8;
            }
        } else {
            if (((newTileCollision) as i32) == 11i32) || (((newTileCollision) as i32) == 13i32) {
                return 0u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsBikingDisallowedByPlayer() -> u8 {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut tileBehavior: u8 = 0u8;
        if !((((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 24i32) != 0) {
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            tileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
            if !((IsRunningDisallowedByMetatile(tileBehavior)) != 0) {
                return 0u8;
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerNotUsingAcroBikeOnBumpySlope() -> u8 {
    unsafe {
        if ((TestPlayerAvatarFlags(4u8)) != 0)
            && ((MetatileBehavior_IsBumpySlope(
                ((((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(30))
                .read(),
            )) != 0)
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetOnOffBike(transitionFlags: u8) {
    unsafe {
        let mut transitionFlags = transitionFlags;
        ((&raw mut gUnusedBikeCameraAheadPanback).cast::<u8>()).write(0u8);
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 6i32) != 0 {
            SetPlayerAvatarTransitionFlags(1u16);
            Overworld_ClearSavedMusic();
            Overworld_PlaySpecialMapMusic();
        } else {
            SetPlayerAvatarTransitionFlags(((transitionFlags) as u16));
            Overworld_SetSavedMusic(403u16);
            Overworld_ChangeMusicTo(403u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BikeClearState(newDirHistory: i32, newAbStartHistory: i32) {
    unsafe {
        let mut newDirHistory = newDirHistory;
        let mut newAbStartHistory = newAbStartHistory;
        let mut i: u8 = 0u8;
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(9)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>())
            .wrapping_add(12)
            .cast::<u32>())
        .write(((newDirHistory) as u32));
        (((&raw mut gPlayerAvatar).cast::<u8>())
            .wrapping_add(16)
            .cast::<u32>())
        .write(((newAbStartHistory) as u32));
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(20)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0u8;
            'l3: loop {
                if !(((i) as u32) < crate::c::div_u32(8u32, 1u32)) {
                    break 'l3;
                }
                'l4: {
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(28)).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bike_UpdateBikeCounterSpeed(counter: u8) {
    unsafe {
        let mut counter = counter;
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).write(counter);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(
            (((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                        >> 1),
                )) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn Bike_SetBikeStill() {
    unsafe {
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(11)).write(0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerSpeed() -> i16 {
    unsafe {
        let mut machSpeeds = crate::ffi::Align4([0u8; 6]);
        crate::c::memcpy(
            ((&raw mut machSpeeds).cast::<i16>()).cast::<u8>(),
            (((&raw const sMachBikeSpeeds)
                .cast::<u8>()
                .cast_mut()
                .cast::<u16>())
            .cast::<u16>())
            .cast::<u8>(),
            6u32,
        );
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 2i32) != 0 {
            return (((&raw mut machSpeeds).cast::<i16>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(10)).read()) as i32)
                    as isize,
            ))
            .read();
        } else {
            if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 4i32) != 0 {
                return 3i16;
            } else {
                if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 136i32) != 0 {
                    return 2i16;
                } else {
                    return 1i16;
                }
            }
        }
        #[allow(unreachable_code)]
        {
            return 0i16;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Bike_HandleBumpySlopeJump() {
    unsafe {
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut tileBehavior: u8 = 0u8;
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 4i32) != 0 {
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            tileBehavior = ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8);
            if (MetatileBehavior_IsBumpySlope(tileBehavior)) != 0 {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(8)).write(2u8);
                PlayerUseAcroBikeOnBumpySlope(GetPlayerMovementDirection());
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsRunningDisallowed(metatile: u8) -> u32 {
    unsafe {
        let mut metatile = metatile;
        if (!((crate::c::bf_read(
            ((&raw mut gMapHeader).cast::<u8>()).wrapping_add(26),
            2,
            1,
            false,
        ) as u8)
            != 0))
            || (((IsRunningDisallowedByMetatile(metatile)) as i32) == 1i32)
        {
            return 1u32;
        } else {
            return 0u32;
        }
        #[allow(unreachable_code)]
        {
            return 0u32;
        }
    }
}
