//! Translated from `src/field_player_avatar.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sForcedMovementTestFuncs sForcedMovementFuncs sPlayerNotOnBikeFuncs sAcroBikeTrickMetatiles sAcroBikeTrickCollisionTypes sPlayerAvatarTransitionFuncs sArrowWarpMetatileBehaviorChecks sRivalAvatarGfxIds sPlayerAvatarGfxIds sFRLGAvatarGfxIds sRSAvatarGfxIds sPlayerAvatarGfxToStateFlag sArrowWarpMetatileBehaviorChecks2 sPushBoulderFuncs sPlayerAvatarSecretBaseMatJump sPlayerAvatarSecretBaseMatSpin sFishingStateFuncs sSpinDirections
#[allow(unused_imports)]
use crate::data::field_player_avatar::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sSpinStartFacingDir: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gObjectEvents: crate::ffi::Align4<[u8; 576]> = crate::ffi::Align4([0; 576]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerAvatar: crate::ffi::Align4<[u8; 36]> = crate::ffi::Align4([0; 36]);

unsafe extern "C" {
    static mut gFieldEffectArguments: u8;
    static mut gMain: u8;
    static mut gPlayerParty: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gText_ItGotAway: u8;
    static mut gText_NotEvenANibble: u8;
    static mut gText_OhABite: u8;
    static mut gText_PokemonOnHook: u8;
    static mut gTotalCameraPixelOffsetY: u8;
    fn AddTextPrinterParameterized(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: Option<unsafe extern "C" fn(*mut u8, u16)>,
    ) -> u16;
    fn AddTextPrinterParameterized2(
        a0: u8,
        a1: u8,
        a2: *mut u8,
        a3: u8,
        a4: Option<unsafe extern "C" fn(*mut u8, u16)>,
        a5: u8,
        a6: u8,
        a7: u8,
    ) -> u16;
    fn AnimateSprite(a0: *mut u8);
    fn BikeClearState(a0: i32, a1: i32);
    fn Bike_HandleBumpySlopeJump();
    fn Bike_TryAcroBikeHistoryUpdate(a0: u16, a1: u16);
    fn Bike_UpdateBikeCounterSpeed(a0: u8);
    fn CameraObjectFreeze();
    fn CameraObjectReset();
    fn CheckForRotatingGatePuzzleCollision(a0: u8, a1: i16, a2: i16) -> u32;
    fn CheckForRotatingGatePuzzleCollisionWithoutAnimation(a0: u8, a1: i16, a2: i16) -> u32;
    fn ClearDialogWindowAndFrame(a0: u8, a1: u8);
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateWarpArrowSprite() -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoesCurrentMapHaveFishingMons() -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FillWindowPixelBuffer(a0: u8, a1: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FishingWildEncounter(a0: u8);
    fn FlagGet(a0: u16) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetAcroEndWheelieFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroEndWheelieMoveDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroPopWheelieFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroPopWheelieMoveDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroWheelieDirectionAnimNum(a0: u8) -> u8;
    fn GetAcroWheelieFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroWheelieHopDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroWheelieHopFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroWheelieInPlaceDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroWheelieJumpDirectionMovementAction(a0: u32) -> u8;
    fn GetAcroWheelieMoveDirectionMovementAction(a0: u32) -> u8;
    fn GetCollisionAtCoords(a0: *mut u8, a1: i16, a2: i16, a3: u32) -> u8;
    fn GetFaceDirectionAnimNum(a0: u8) -> u8;
    fn GetFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetFishingBiteDirectionAnimNum(a0: u8) -> u8;
    fn GetFishingDirectionAnimNum(a0: u8) -> u8;
    fn GetFishingNoCatchDirectionAnimNum(a0: u8) -> u8;
    fn GetJump2MovementAction(a0: u32) -> u8;
    fn GetJumpInPlaceMovementAction(a0: u32) -> u8;
    fn GetJumpInPlaceTurnAroundMovementAction(a0: u32) -> u8;
    fn GetJumpSpecialMovementAction(a0: u32) -> u8;
    fn GetLedgeJumpDirection(a0: i16, a1: i16, a2: u8) -> u8;
    fn GetMonAbility(a0: *mut u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8) -> u8;
    fn GetObjectEventIdByPosition(a0: u16, a1: u16, a2: u8) -> u8;
    fn GetObjectEventIdByXY(a0: i16, a1: i16) -> u8;
    fn GetOppositeDirection(a0: u8) -> u8;
    fn GetPlayerRunMovementAction(a0: u32) -> u8;
    fn GetPlayerSpeed() -> i16;
    fn GetRideWaterCurrentMovementAction(a0: u32) -> u8;
    fn GetWalkFastMovementAction(a0: u32) -> u8;
    fn GetWalkFasterMovementAction(a0: u32) -> u8;
    fn GetWalkInPlaceFastMovementAction(a0: u32) -> u8;
    fn GetWalkInPlaceNormalMovementAction(a0: u32) -> u8;
    fn GetWalkInPlaceSlowMovementAction(a0: u32) -> u8;
    fn GetWalkNormalMovementAction(a0: u32) -> u8;
    fn GetWalkSlowMovementAction(a0: u32) -> u8;
    fn IncrementGameStat(a0: u8);
    fn IsPlayerNotUsingAcroBikeOnBumpySlope() -> u8;
    fn IsRunningDisallowed(a0: u8) -> u32;
    fn IsTextPrinterActive(a0: u8) -> u16;
    fn LoadMessageBoxAndFrameGfx(a0: u8, a1: u8);
    fn LockPlayerFieldControls();
    fn MapGridGetElevationAt(a0: i32, a1: i32) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsNonAnimDoor(a0: u8) -> u8;
    fn MetatileBehavior_IsSurfableFishableWater(a0: u8) -> u8;
    fn MetatileBehavior_IsWarpDoor(a0: u8) -> u8;
    fn MonKnowsMove(a0: *mut u8, a1: u16) -> u8;
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn MoveObjectEventToMapCoords(a0: *mut u8, a1: i16, a2: i16);
    fn MovePlayerOnBike(a0: u8, a1: u16, a2: u16);
    fn ObjectEventCheckHeldMovementStatus(a0: *mut u8) -> u8;
    fn ObjectEventClearHeldMovement(a0: *mut u8);
    fn ObjectEventClearHeldMovementIfActive(a0: *mut u8);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventForceSetHeldMovement(a0: *mut u8, a1: u8);
    fn ObjectEventGetHeldMovementActionId(a0: *mut u8) -> u8;
    fn ObjectEventIsHeldMovementActive(a0: *mut u8) -> u8;
    fn ObjectEventIsMovementOverridden(a0: *mut u8) -> u8;
    fn ObjectEventSetGraphicsId(a0: *mut u8, a1: u8);
    fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8;
    fn ObjectEventTurn(a0: *mut u8, a1: u8);
    fn Overworld_ChangeMusicToDefault();
    fn Overworld_ClearSavedMusic();
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn RecordFishingAttemptForTV(a0: u8);
    fn RunTextPrinters();
    fn SeekSpriteAnim(a0: *mut u8, a1: u8);
    fn SetObjectEventDirection(a0: *mut u8, a1: u8);
    fn SetSpriteInvisible(a0: u8);
    fn SetSurfBlob_BobState(a0: u8, a1: u8);
    fn SetSurfBlob_PlayerOffset(a0: u8, a1: u8, a2: i16);
    fn ShowWarpArrowSprite(a0: u8, a1: u8, a2: i16, a3: i16);
    fn SpawnSpecialObjectEvent(a0: *mut u8) -> u8;
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StartUnderwaterSurfBlobBobbing(a0: u8) -> u8;
    fn UnfreezeObjectEvents();
    fn UnlockPlayerFieldControls();
    fn UpdateObjectEventCurrentMovement(
        a0: *mut u8,
        a1: *mut u8,
        a2: Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>,
    );
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MovementType_Player(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateObjectEventCurrentMovement(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 36,
            ),
            sprite,
            core::mem::transmute::<
                Option<unsafe extern "C" fn() -> u8>,
                Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>,
            >(Some(ObjectEventCB2_NoMovement2)),
        );
    }
}
pub(crate) unsafe extern "C" fn ObjectEventCB2_NoMovement2() -> u8 {
    unsafe {
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerStep(direction: u8, newKeys: u16, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        HideShowWarpArrow(playerObjEvent);
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).read()) as i32) == 0i32 {
            Bike_TryAcroBikeHistoryUpdate(newKeys, heldKeys);
            if ((TryInterruptObjectEventSpecialAnim(playerObjEvent, direction)) as i32) == 0i32 {
                npc_clear_strange_bits(playerObjEvent);
                DoPlayerAvatarTransition();
                if ((TryDoMetatileBehaviorForcedMovement()) as i32) == 0i32 {
                    MovePlayerAvatarUsingKeypadInput(direction, newKeys, heldKeys);
                    PlayerAllowForcedMovementIfMovingSameDirection();
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TryInterruptObjectEventSpecialAnim(
    playerObjEvent: *mut u8,
    direction: u8,
) -> u8 {
    unsafe {
        let mut playerObjEvent = playerObjEvent;
        let mut direction = direction;
        if ((ObjectEventIsMovementOverridden(playerObjEvent)) != 0)
            && (!((ObjectEventClearHeldMovementIfFinished(playerObjEvent)) != 0))
        {
            let mut heldMovementActionId: u8 = ObjectEventGetHeldMovementActionId(playerObjEvent);
            if (((heldMovementActionId) as i32) > 24i32)
                && (((heldMovementActionId) as i32) < 29i32)
            {
                if ((direction) as i32) == 0i32 {
                    return 1u8;
                }
                if ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16)
                    as i32)
                    != ((direction) as i32)
                {
                    ObjectEventClearHeldMovement(playerObjEvent);
                    return 0u8;
                }
                if ((CheckForPlayerAvatarStaticCollision(direction)) as i32) == 0i32 {
                    ObjectEventClearHeldMovement(playerObjEvent);
                    return 0u8;
                }
            }
            return 1u8;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn npc_clear_strange_bits(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        crate::c::bf_write((objEvent).wrapping_add(1), 4, 1, (0u32) as i32);
        crate::c::bf_write((objEvent).wrapping_add(1), 2, 1, (0u32) as i32);
        crate::c::bf_write((objEvent).wrapping_add(1), 1, 1, (0u32) as i32);
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
        (__p1).write((((((__p1).read()) as i32) & (-129i32)) as u8));
    }
}
pub(crate) unsafe extern "C" fn MovePlayerAvatarUsingKeypadInput(
    direction: u8,
    newKeys: u16,
    heldKeys: u16,
) {
    unsafe {
        let mut direction = direction;
        let mut newKeys = newKeys;
        let mut heldKeys = heldKeys;
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 6i32) != 0 {
            MovePlayerOnBike(direction, newKeys, heldKeys);
        } else {
            MovePlayerNotOnBike(direction, heldKeys);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerAllowForcedMovementIfMovingSameDirection() {
    unsafe {
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32) == 2i32 {
            let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
            (__p1).write((((((__p1).read()) as i32) & (-33i32)) as u8));
        }
    }
}
pub(crate) unsafe extern "C" fn TryDoMetatileBehaviorForcedMovement() -> u8 {
    unsafe {
        return (((((&raw const sForcedMovementFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .cast::<Option<unsafe extern "C" fn() -> u8>>())
        .wrapping_offset(((GetForcedMovementByMetatileBehavior()) as i32) as isize))
        .read())
        .unwrap_unchecked()();
    }
}
pub(crate) unsafe extern "C" fn GetForcedMovementByMetatileBehavior() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        if !((((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 32i32) != 0) {
            let mut metatileBehavior: u8 =
                (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(30))
                .read();
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 18i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((((&raw const sForcedMovementTestFuncs)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                        .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .unwrap_unchecked()(metatileBehavior))
                            != 0
                        {
                            return ((((i) as i32).wrapping_add(1i32)) as u8);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_None() -> u8 {
    unsafe {
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 64i32) != 0 {
            let mut playerObjEvent: *mut u8 =
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                );
            crate::c::bf_write((playerObjEvent).wrapping_add(1), 1, 1, (0u32) as i32);
            crate::c::bf_write((playerObjEvent).wrapping_add(1), 3, 1, (1u32) as i32);
            SetObjectEventDirection(
                playerObjEvent,
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
            );
            let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
            (__p1).write((((((__p1).read()) as i32) & (-65i32)) as u8));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoForcedMovement(
    direction: u8,
    moveFunc: Option<unsafe extern "C" fn(u8)>,
) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut moveFunc = moveFunc;
        let mut playerAvatar: *mut u8 = (&raw mut gPlayerAvatar).cast::<u8>();
        let mut collision: u8 = CheckForPlayerAvatarCollision(direction);
        let __p1 = (playerAvatar);
        (__p1).write((((((__p1).read()) as i32) | 64i32) as u8));
        if (collision) != 0 {
            ForcedMovement_None();
            if ((collision) as i32) < 5i32 {
                return 0u8;
            } else {
                if ((collision) as i32) == 6i32 {
                    PlayerJumpLedge(direction);
                }
                let __p2 = (playerAvatar);
                (__p2).write((((((__p2).read()) as i32) | 64i32) as u8));
                ((playerAvatar).wrapping_add(2)).write(2u8);
                return 1u8;
            }
        } else {
            ((playerAvatar).wrapping_add(2)).write(2u8);
            (moveFunc).unwrap_unchecked()(direction);
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn DoForcedMovementInCurrentDirection(
    moveFunc: Option<unsafe extern "C" fn(u8)>,
) -> u8 {
    unsafe {
        let mut moveFunc = moveFunc;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        crate::c::bf_write((playerObjEvent).wrapping_add(1), 2, 1, (1u32) as i32);
        return DoForcedMovement(
            ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            moveFunc,
        );
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_Slip() -> u8 {
    unsafe {
        return DoForcedMovementInCurrentDirection(Some(PlayerWalkFast));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_WalkSouth() -> u8 {
    unsafe {
        return DoForcedMovement(1u8, Some(PlayerWalkNormal));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_WalkNorth() -> u8 {
    unsafe {
        return DoForcedMovement(2u8, Some(PlayerWalkNormal));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_WalkWest() -> u8 {
    unsafe {
        return DoForcedMovement(3u8, Some(PlayerWalkNormal));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_WalkEast() -> u8 {
    unsafe {
        return DoForcedMovement(4u8, Some(PlayerWalkNormal));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_PushedSouthByCurrent() -> u8 {
    unsafe {
        return DoForcedMovement(1u8, Some(PlayerRideWaterCurrent));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_PushedNorthByCurrent() -> u8 {
    unsafe {
        return DoForcedMovement(2u8, Some(PlayerRideWaterCurrent));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_PushedWestByCurrent() -> u8 {
    unsafe {
        return DoForcedMovement(3u8, Some(PlayerRideWaterCurrent));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_PushedEastByCurrent() -> u8 {
    unsafe {
        return DoForcedMovement(4u8, Some(PlayerRideWaterCurrent));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_Slide(
    direction: u8,
    moveFunc: Option<unsafe extern "C" fn(u8)>,
) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut moveFunc = moveFunc;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        crate::c::bf_write((playerObjEvent).wrapping_add(1), 2, 1, (1u32) as i32);
        crate::c::bf_write((playerObjEvent).wrapping_add(1), 1, 1, (1u32) as i32);
        return DoForcedMovement(direction, moveFunc);
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_SlideSouth() -> u8 {
    unsafe {
        return ForcedMovement_Slide(1u8, Some(PlayerWalkFast));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_SlideNorth() -> u8 {
    unsafe {
        return ForcedMovement_Slide(2u8, Some(PlayerWalkFast));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_SlideWest() -> u8 {
    unsafe {
        return ForcedMovement_Slide(3u8, Some(PlayerWalkFast));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_SlideEast() -> u8 {
    unsafe {
        return ForcedMovement_Slide(4u8, Some(PlayerWalkFast));
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_MatJump() -> u8 {
    unsafe {
        DoPlayerMatJump();
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_MatSpin() -> u8 {
    unsafe {
        DoPlayerMatSpin();
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn ForcedMovement_MuddySlope() -> u8 {
    unsafe {
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        if (((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as i32)
            != 2i32)
            || (((GetPlayerSpeed()) as i32) < 4i32)
        {
            Bike_UpdateBikeCounterSpeed(0u8);
            crate::c::bf_write((playerObjEvent).wrapping_add(1), 1, 1, (1u32) as i32);
            return DoForcedMovement(1u8, Some(PlayerWalkFast));
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn MovePlayerNotOnBike(direction: u8, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut heldKeys = heldKeys;
        (((((&raw const sPlayerNotOnBikeFuncs)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8, u16)>>())
        .cast::<Option<unsafe extern "C" fn(u8, u16)>>())
        .wrapping_offset(((CheckMovementInputNotOnBike(direction)) as i32) as isize))
        .read())
        .unwrap_unchecked()(direction, heldKeys);
    }
}
pub(crate) unsafe extern "C" fn CheckMovementInputNotOnBike(direction: u8) -> u8 {
    unsafe {
        let mut direction = direction;
        if ((direction) as i32) == 0i32 {
            return {
                let __v1 = 0u8;
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(__v1);
                __v1
            };
        } else {
            if (((direction) as i32) != ((GetPlayerMovementDirection()) as i32))
                && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32)
                    != 2i32)
            {
                return {
                    let __v2 = 1u8;
                    (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(__v2);
                    __v2
                };
            } else {
                return {
                    let __v3 = 2u8;
                    (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(__v3);
                    __v3
                };
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerNotOnBikeNotMoving(direction: u8, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut heldKeys = heldKeys;
        PlayerFaceDirection(GetPlayerFacingDirection());
    }
}
pub(crate) unsafe extern "C" fn PlayerNotOnBikeTurningInPlace(direction: u8, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut heldKeys = heldKeys;
        PlayerTurnInPlace(direction);
    }
}
pub(crate) unsafe extern "C" fn PlayerNotOnBikeMoving(direction: u8, heldKeys: u16) {
    unsafe {
        let mut direction = direction;
        let mut heldKeys = heldKeys;
        let mut collision: u8 = CheckForPlayerAvatarCollision(direction);
        if (collision) != 0 {
            if ((collision) as i32) == 6i32 {
                PlayerJumpLedge(direction);
                return;
            } else {
                if (((collision) as i32) == 4i32)
                    && ((IsPlayerCollidingWithFarawayIslandMew(direction)) != 0)
                {
                    PlayerNotOnBikeCollideWithFarawayIslandMew(direction);
                    return;
                } else {
                    if (((((collision) as i32) != 5i32) && (((collision) as i32) != 6i32))
                        && (((collision) as i32) != 7i32))
                        && (((collision) as i32) != 8i32)
                    {
                        PlayerNotOnBikeCollide(direction);
                    }
                    return;
                }
            }
        }
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 8i32) != 0 {
            PlayerWalkFast(direction);
            return;
        }
        if (((!((((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 16i32) != 0))
            && ((((heldKeys) as i32) & 2i32) != 0))
            && ((FlagGet(2240u16)) != 0))
            && (IsRunningDisallowed(
                (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(30))
                .read(),
            ) == 0u32)
        {
            PlayerRun(direction);
            let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
            (__p1).write((((((__p1).read()) as i32) | 128i32) as u8));
            return;
        } else {
            PlayerWalkNormal(direction);
        }
    }
}
pub(crate) unsafe extern "C" fn CheckForPlayerAvatarCollision(direction: u8) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        x = (((playerObjEvent).wrapping_add(16)).cast::<i16>()).read();
        y = (((playerObjEvent).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read();
        MoveCoords(direction, &raw mut x, &raw mut y);
        return CheckForObjectEventCollision(
            playerObjEvent,
            x,
            y,
            direction,
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn CheckForPlayerAvatarStaticCollision(direction: u8) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        x = (((playerObjEvent).wrapping_add(16)).cast::<i16>()).read();
        y = (((playerObjEvent).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read();
        MoveCoords(direction, &raw mut x, &raw mut y);
        return CheckForObjectEventStaticCollision(
            playerObjEvent,
            x,
            y,
            direction,
            ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForObjectEventCollision(
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
        let mut collision: u8 = GetCollisionAtCoords(objectEvent, x, y, ((direction) as u32));
        if (((collision) as i32) == 3i32) && ((CanStopSurfing(x, y, direction)) != 0) {
            return 5u8;
        }
        if (ShouldJumpLedge(x, y, direction)) != 0 {
            IncrementGameStat(43u8);
            return 6u8;
        }
        if (((collision) as i32) == 4i32) && ((TryPushBoulder(x, y, direction)) != 0) {
            return 7u8;
        }
        if ((collision) as i32) == 0i32 {
            if (CheckForRotatingGatePuzzleCollision(direction, x, y)) != 0 {
                return 8u8;
            }
            CheckAcroBikeCollision(x, y, metatileBehavior, &raw mut collision);
        }
        return collision;
    }
}
pub(crate) unsafe extern "C" fn CheckForObjectEventStaticCollision(
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
        let mut collision: u8 = GetCollisionAtCoords(objectEvent, x, y, ((direction) as u32));
        if ((collision) as i32) == 0i32 {
            if (CheckForRotatingGatePuzzleCollisionWithoutAnimation(direction, x, y)) != 0 {
                return 8u8;
            }
            CheckAcroBikeCollision(x, y, metatileBehavior, &raw mut collision);
        }
        return collision;
    }
}
pub(crate) unsafe extern "C" fn CanStopSurfing(x: i16, y: i16, direction: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut direction = direction;
        if (((((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 8i32) != 0)
            && (((MapGridGetElevationAt(((x) as i32), ((y) as i32))) as i32) == 3i32))
            && (((GetObjectEventIdByPosition(((x) as u16), ((y) as u16), 3u8)) as i32) == 16i32)
        {
            CreateStopSurfingTask(direction);
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
pub(crate) unsafe extern "C" fn ShouldJumpLedge(x: i16, y: i16, direction: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut direction = direction;
        if ((GetLedgeJumpDirection(x, y, direction)) as i32) != 0i32 {
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
pub(crate) unsafe extern "C" fn TryPushBoulder(x: i16, y: i16, direction: u8) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut direction = direction;
        if (FlagGet(2185u16)) != 0 {
            let mut objectEventId: u8 = GetObjectEventIdByXY(x, y);
            if (((objectEventId) as i32) != 16i32)
                && ((((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(5))
                .read()) as i32)
                    == 87i32)
            {
                x = ((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(16))
                .cast::<i16>())
                .read();
                y = ((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36))
                .wrapping_add(16))
                .wrapping_add(2)
                .cast::<i16>())
                .read();
                MoveCoords(direction, &raw mut x, &raw mut y);
                if (((GetCollisionAtCoords(
                    (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((objectEventId) as i32) as isize * 36),
                    x,
                    y,
                    ((direction) as u32),
                )) as i32)
                    == 0i32)
                    && (((MetatileBehavior_IsNonAnimDoor(
                        ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                    )) as i32)
                        == 0i32)
                {
                    StartStrengthAnim(objectEventId, direction);
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CheckAcroBikeCollision(
    x: i16,
    y: i16,
    metatileBehavior: u8,
    collision: *mut u8,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut metatileBehavior = metatileBehavior;
        let mut collision = collision;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((&raw const sAcroBikeTrickMetatiles)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                    .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                    .wrapping_offset(((i) as i32) as isize))
                    .read())
                    .unwrap_unchecked()(metatileBehavior))
                        != 0
                    {
                        (collision).write(
                            ((((&raw const sAcroBikeTrickCollisionTypes)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                        return;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerCollidingWithFarawayIslandMew(direction: u8) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut mewObjectId: u8 = 0u8;
        let mut object: *mut u8 = core::ptr::null_mut();
        let mut playerX: i16 = 0i16;
        let mut playerY: i16 = 0i16;
        let mut mewPrevX: i16 = 0i16;
        object = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        playerX = (((object).wrapping_add(16)).cast::<i16>()).read();
        playerY = (((object).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read();
        MoveCoords(direction, &raw mut playerX, &raw mut playerY);
        mewObjectId = GetObjectEventIdByLocalIdAndMap(1u8, 57u8, 26u8);
        if ((mewObjectId) as i32) == 16i32 {
            return 0u8;
        }
        object = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((mewObjectId) as i32) as isize * 36);
        mewPrevX = (((object).wrapping_add(20)).cast::<i16>()).read();
        if ((mewPrevX) as i32) == ((playerX) as i32) {
            if (((((((object).wrapping_add(20)).wrapping_add(2).cast::<i16>()).read()) as i32)
                != ((playerY) as i32))
                || ((((((object).wrapping_add(16)).cast::<i16>()).read()) as i32)
                    != ((mewPrevX) as i32)))
                || ((((((object).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32)
                    != (((((object).wrapping_add(20)).wrapping_add(2).cast::<i16>()).read())
                        as i32))
            {
                if ((((((object).wrapping_add(20)).cast::<i16>()).read()) as i32)
                    == ((playerX) as i32))
                    && ((((((object).wrapping_add(20)).wrapping_add(2).cast::<i16>()).read())
                        as i32)
                        == ((playerY) as i32))
                {
                    return 1u8;
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerAvatarTransitionFlags(transitionFlags: u16) {
    unsafe {
        let mut transitionFlags = transitionFlags;
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(1);
        (__p1).write((((((__p1).read()) as i32) | ((transitionFlags) as i32)) as u8));
        DoPlayerAvatarTransition();
    }
}
pub(crate) unsafe extern "C" fn DoPlayerAvatarTransition() {
    unsafe {
        let mut i: u8 = 0u8;
        let mut flags: u8 = (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(1)).read();
        if ((flags) as i32) != 0i32 {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(32u32, 4u32)) {
                        break 'l1;
                    }
                    'l2: {
                        if (((flags) as i32) & 1i32) != 0 {
                            (((((&raw const sPlayerAvatarTransitionFuncs)
                                .cast::<u8>()
                                .cast_mut()
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read())
                            .unwrap_unchecked()(
                                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(
                                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5))
                                            .read())
                                            as i32)
                                            as isize
                                            * 36,
                                    ),
                            );
                        }
                    }
                    i = (i).wrapping_add(1);
                    flags = ((((flags) as i32) >> 1) as u8);
                }
            }
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(1)).write(0u8);
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_Dummy(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_Normal(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        ObjectEventSetGraphicsId(objEvent, GetPlayerAvatarGraphicsIdByStateId(0u8));
        ObjectEventTurn(
            objEvent,
            ((crate::c::bf_read((objEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
        );
        SetPlayerAvatarStateMask(1u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_MachBike(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        ObjectEventSetGraphicsId(objEvent, GetPlayerAvatarGraphicsIdByStateId(1u8));
        ObjectEventTurn(
            objEvent,
            ((crate::c::bf_read((objEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
        );
        SetPlayerAvatarStateMask(2u8);
        BikeClearState(0i32, 0i32);
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_AcroBike(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        ObjectEventSetGraphicsId(objEvent, GetPlayerAvatarGraphicsIdByStateId(2u8));
        ObjectEventTurn(
            objEvent,
            ((crate::c::bf_read((objEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
        );
        SetPlayerAvatarStateMask(4u8);
        BikeClearState(0i32, 0i32);
        Bike_HandleBumpySlopeJump();
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_Surfing(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        let mut spriteId: u8 = 0u8;
        ObjectEventSetGraphicsId(objEvent, GetPlayerAvatarGraphicsIdByStateId(3u8));
        ObjectEventTurn(
            objEvent,
            ((crate::c::bf_read((objEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
        );
        SetPlayerAvatarStateMask(8u8);
        (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
            .write((((((objEvent).wrapping_add(16)).cast::<i16>()).read()) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
            .write((((((objEvent).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32));
        ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
            .write((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32));
        spriteId = ((FieldEffectStart(8u8)) as u8);
        ((objEvent).wrapping_add(26)).write(spriteId);
        SetSurfBlob_BobState(spriteId, 1u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_Underwater(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        ObjectEventSetGraphicsId(objEvent, GetPlayerAvatarGraphicsIdByStateId(4u8));
        ObjectEventTurn(
            objEvent,
            ((crate::c::bf_read((objEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
        );
        SetPlayerAvatarStateMask(16u8);
        ((objEvent).wrapping_add(26)).write(StartUnderwaterSurfBlobBobbing(
            ((objEvent).wrapping_add(4)).read(),
        ));
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatarTransition_ReturnToField(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
        (__p1).write((((((__p1).read()) as i32) | 32i32) as u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdatePlayerAvatarTransitionState() {
    unsafe {
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).write(0u8);
        if (PlayerIsAnimActive()) != 0 {
            if !((PlayerCheckIfAnimFinishedOrInactive()) != 0) {
                if !((PlayerAnimIsMultiFrameStationary()) != 0) {
                    (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).write(1u8);
                }
            } else {
                if !((PlayerAnimIsMultiFrameStationaryAndStateNotTurning()) != 0) {
                    (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).write(2u8);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerAnimIsMultiFrameStationary() -> u8 {
    unsafe {
        let mut movementActionId: u8 = (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
        .wrapping_add(28))
        .read();
        if ((((((movementActionId) as i32) <= 3i32)
            || ((((movementActionId) as i32) >= 16i32) && (((movementActionId) as i32) <= 20i32)))
            || ((((movementActionId) as i32) >= 25i32) && (((movementActionId) as i32) <= 40i32)))
            || ((((movementActionId) as i32) >= 100i32) && (((movementActionId) as i32) <= 111i32)))
            || ((((movementActionId) as i32) >= 124i32) && (((movementActionId) as i32) <= 127i32))
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
pub(crate) unsafe extern "C" fn PlayerAnimIsMultiFrameStationaryAndStateNotTurning() -> u8 {
    unsafe {
        if ((PlayerAnimIsMultiFrameStationary()) != 0)
            && ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).read()) as i32) != 1i32)
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
pub(crate) unsafe extern "C" fn PlayerIsAnimActive() -> u8 {
    unsafe {
        return ObjectEventIsMovementOverridden(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
        );
    }
}
pub(crate) unsafe extern "C" fn PlayerCheckIfAnimFinishedOrInactive() -> u8 {
    unsafe {
        return ObjectEventCheckHeldMovementStatus(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
        );
    }
}
pub(crate) unsafe extern "C" fn PlayerSetCopyableMovement(movement: u8) {
    unsafe {
        let mut movement = movement;
        (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        ))
        .wrapping_add(34))
        .write(movement);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerGetCopyableMovement() -> u8 {
    unsafe {
        return (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        ))
        .wrapping_add(34))
        .read();
    }
}
pub(crate) unsafe extern "C" fn PlayerForceSetHeldMovement(movementActionId: u8) {
    unsafe {
        let mut movementActionId = movementActionId;
        ObjectEventForceSetHeldMovement(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            movementActionId,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerSetAnimId(movementActionId: u8, copyableMovement: u8) {
    unsafe {
        let mut movementActionId = movementActionId;
        let mut copyableMovement = copyableMovement;
        if !((PlayerIsAnimActive()) != 0) {
            PlayerSetCopyableMovement(copyableMovement);
            ObjectEventSetHeldMovement(
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                movementActionId,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerWalkNormal(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetWalkNormalMovementAction(((direction) as u32)), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerWalkFast(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetWalkFastMovementAction(((direction) as u32)), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerRideWaterCurrent(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetRideWaterCurrentMovementAction(((direction) as u32)), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerWalkFaster(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetWalkFasterMovementAction(((direction) as u32)), 2u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerRun(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetPlayerRunMovementAction(((direction) as u32)), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerOnBikeCollide(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayCollisionSoundIfNotFacingWarp(direction);
        PlayerSetAnimId(
            GetWalkInPlaceNormalMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerOnBikeCollideWithFarawayIslandMew(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetWalkInPlaceNormalMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PlayerNotOnBikeCollide(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayCollisionSoundIfNotFacingWarp(direction);
        PlayerSetAnimId(GetWalkInPlaceSlowMovementAction(((direction) as u32)), 2u8);
    }
}
pub(crate) unsafe extern "C" fn PlayerNotOnBikeCollideWithFarawayIslandMew(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetWalkInPlaceSlowMovementAction(((direction) as u32)), 2u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerFaceDirection(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetFaceDirectionMovementAction(((direction) as u32)), 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerTurnInPlace(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(GetWalkInPlaceFastMovementAction(((direction) as u32)), 1u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerJumpLedge(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlaySE(10u16);
        PlayerSetAnimId(GetJump2MovementAction(((direction) as u32)), 8u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerFreeze() {
    unsafe {
        if ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 2i32)
            || ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 0i32)
        {
            if (IsPlayerNotUsingAcroBikeOnBumpySlope()) != 0 {
                PlayerForceSetHeldMovement(GetFaceDirectionMovementAction(
                    ((crate::c::bf_read(
                        ((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize
                                * 36,
                        ))
                        .wrapping_add(24),
                        0,
                        4,
                        false,
                    ) as u16) as u32),
                ));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerIdleWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetAcroWheelieFaceDirectionMovementAction(((direction) as u32)),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerStartWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetAcroPopWheelieFaceDirectionMovementAction(((direction) as u32)),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerEndWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetAcroEndWheelieFaceDirectionMovementAction(((direction) as u32)),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerStandingHoppingWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlaySE(34u16);
        PlayerSetAnimId(
            GetAcroWheelieHopFaceDirectionMovementAction(((direction) as u32)),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerMovingHoppingWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlaySE(34u16);
        PlayerSetAnimId(
            GetAcroWheelieHopDirectionMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerLedgeHoppingWheelie(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlaySE(34u16);
        PlayerSetAnimId(
            GetAcroWheelieJumpDirectionMovementAction(((direction) as u32)),
            8u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerAcroTurnJump(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlaySE(34u16);
        PlayerSetAnimId(
            GetJumpInPlaceTurnAroundMovementAction(((direction) as u32)),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerWheelieInPlace(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlaySE(7u16);
        PlayerSetAnimId(
            GetAcroWheelieInPlaceDirectionMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerPopWheelieWhileMoving(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetAcroPopWheelieMoveDirectionMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerWheelieMove(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetAcroWheelieMoveDirectionMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerEndWheelieWhileMoving(direction: u8) {
    unsafe {
        let mut direction = direction;
        PlayerSetAnimId(
            GetAcroEndWheelieMoveDirectionMovementAction(((direction) as u32)),
            2u8,
        );
    }
}
pub(crate) unsafe extern "C" fn PlayCollisionSoundIfNotFacingWarp(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut metatileBehavior: u8 = (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
        .wrapping_add(30))
        .read();
        if !(((((((&raw const sArrowWarpMetatileBehaviorChecks)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
        .wrapping_offset((((direction) as i32).wrapping_sub(1i32)) as isize))
        .read())
        .unwrap_unchecked()(metatileBehavior))
            != 0)
        {
            if ((direction) as i32) == 2i32 {
                PlayerGetDestCoords(&raw mut x, &raw mut y);
                MoveCoords(direction, &raw mut x, &raw mut y);
                if (MetatileBehavior_IsWarpDoor(
                    ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
                )) != 0
                {
                    return;
                }
            }
            PlaySE(7u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetXYCoordsOneStepInFrontOfPlayer(x: *mut i16, y: *mut i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        (x).write(
            ((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .cast::<i16>())
            .read(),
        );
        (y).write(
            ((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
            .read(),
        );
        MoveCoords(GetPlayerFacingDirection(), x, y);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerGetDestCoords(x: *mut i16, y: *mut i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        (x).write(
            ((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .cast::<i16>())
            .read(),
        );
        (y).write(
            ((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
            .read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn player_get_pos_including_state_based_drift(
    x: *mut i16,
    y: *mut i16,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut object: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        if (((crate::c::bf_read((object).wrapping_add(0), 6, 1, false) as u32) != 0)
            && (!((crate::c::bf_read((object).wrapping_add(0), 7, 1, false) as u32) != 0)))
            && (!((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((object).wrapping_add(4)).read()) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .read())
                != 0))
        {
            (x).write((((object).wrapping_add(16)).cast::<i16>()).read());
            (y).write((((object).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read());
            'l1: {
                let __sw1 = ((((object).wrapping_add(28)).read()) as i32);
                if __sw1 == 8i32 || __sw1 == 53i32 {
                    (y).write(((y).read()).wrapping_add(1));
                    return 1u8;
                }
                if __sw1 == 9i32 || __sw1 == 54i32 {
                    (y).write(((y).read()).wrapping_sub(1));
                    return 1u8;
                }
                if __sw1 == 10i32 || __sw1 == 55i32 {
                    (x).write(((x).read()).wrapping_sub(1));
                    return 1u8;
                }
                if __sw1 == 11i32 || __sw1 == 56i32 {
                    (x).write(((x).read()).wrapping_add(1));
                    return 1u8;
                }
            }
        }
        (x).write((-1i16));
        (y).write((-1i16));
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerFacingDirection() -> u8 {
    unsafe {
        return ((crate::c::bf_read(
            ((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(24),
            0,
            4,
            false,
        ) as u16) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerMovementDirection() -> u8 {
    unsafe {
        return ((crate::c::bf_read(
            ((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(24),
            4,
            4,
            false,
        ) as u16) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerGetElevation() -> u8 {
    unsafe {
        return (crate::c::bf_read(
            ((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(11),
            4,
            4,
            false,
        ) as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MovePlayerToMapCoords(x: i16, y: i16) {
    unsafe {
        let mut x = x;
        let mut y = y;
        MoveObjectEventToMapCoords(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            x,
            y,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TestPlayerAvatarFlags(flag: u8) -> u8 {
    unsafe {
        let mut flag = flag;
        return ((((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & ((flag) as i32))
            as u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarFlags() -> u8 {
    unsafe {
        return ((&raw mut gPlayerAvatar).cast::<u8>()).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarSpriteId() -> u8 {
    unsafe {
        return (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CancelPlayerForcedMovement() {
    unsafe {
        ForcedMovement_None();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StopPlayerAvatar() {
    unsafe {
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        npc_clear_strange_bits(playerObjEvent);
        SetObjectEventDirection(
            playerObjEvent,
            ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
        );
        if (TestPlayerAvatarFlags(6u8)) != 0 {
            Bike_HandleBumpySlopeJump();
            Bike_UpdateBikeCounterSpeed(0u8);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRivalAvatarGraphicsIdByStateIdAndGender(state: u8, gender: u8) -> u8 {
    unsafe {
        let mut state = state;
        let mut gender = gender;
        return ((((((&raw const sRivalAvatarGfxIds).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((state) as i32) as isize * 2))
        .cast::<u8>())
        .wrapping_offset(((gender) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarGraphicsIdByStateIdAndGender(state: u8, gender: u8) -> u8 {
    unsafe {
        let mut state = state;
        let mut gender = gender;
        return ((((((&raw const sPlayerAvatarGfxIds).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((state) as i32) as isize * 2))
        .cast::<u8>())
        .wrapping_offset(((gender) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetFRLGAvatarGraphicsIdByGender(gender: u8) -> u8 {
    unsafe {
        let mut gender = gender;
        return ((((&raw const sFRLGAvatarGfxIds).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((gender) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetRSAvatarGraphicsIdByGender(gender: u8) -> u8 {
    unsafe {
        let mut gender = gender;
        return ((((&raw const sRSAvatarGfxIds).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((gender) as i32) as isize))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarGraphicsIdByStateId(state: u8) -> u8 {
    unsafe {
        let mut state = state;
        return GetPlayerAvatarGraphicsIdByStateIdAndGender(
            state,
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(7)).read(),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn unref_GetRivalAvatarGenderByGraphicsId(gfxId: u8) -> u8 {
    unsafe {
        let mut gfxId = gfxId;
        'l1: {
            let __sw1 = ((gfxId) as i32);
            let __matched = __sw1 == 105i32
                || __sw1 == 106i32
                || __sw1 == 107i32
                || __sw1 == 108i32
                || __sw1 == 109i32
                || __sw1 == 112i32
                || __sw1 == 138i32
                || __sw1 == 192i32;
            if __sw1 == 105i32
                || __sw1 == 106i32
                || __sw1 == 107i32
                || __sw1 == 108i32
                || __sw1 == 109i32
                || __sw1 == 112i32
                || __sw1 == 138i32
                || __sw1 == 192i32
            {
                return 1u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarGenderByGraphicsId(gfxId: u8) -> u8 {
    unsafe {
        let mut gfxId = gfxId;
        'l1: {
            let __sw1 = ((gfxId) as i32);
            let __matched = __sw1 == 89i32
                || __sw1 == 90i32
                || __sw1 == 91i32
                || __sw1 == 92i32
                || __sw1 == 93i32
                || __sw1 == 112i32
                || __sw1 == 138i32
                || __sw1 == 192i32;
            if __sw1 == 89i32
                || __sw1 == 90i32
                || __sw1 == 91i32
                || __sw1 == 92i32
                || __sw1 == 93i32
                || __sw1 == 112i32
                || __sw1 == 138i32
                || __sw1 == 192i32
            {
                return 1u8;
            }
            if !__matched {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PartyHasMonWithSurf() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        if !((TestPlayerAvatarFlags(8u8)) != 0) {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 6i32) {
                        break 'l1;
                    }
                    'l2: {
                        if GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            11i32,
                        ) == 0u32
                        {
                            break 'l1;
                        }
                        if (MonKnowsMove(
                            ((&raw mut gPlayerParty).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 100),
                            57u16,
                        )) != 0
                        {
                            return 1u8;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerSurfingNorth() -> u8 {
    unsafe {
        if (((GetPlayerMovementDirection()) as i32) == 2i32) && ((TestPlayerAvatarFlags(8u8)) != 0)
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
pub unsafe extern "C" fn IsPlayerFacingSurfableFishableWater() -> u8 {
    unsafe {
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        let mut x: i16 = (((playerObjEvent).wrapping_add(16)).cast::<i16>()).read();
        let mut y: i16 = (((playerObjEvent).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read();
        MoveCoords(
            ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
            &raw mut x,
            &raw mut y,
        );
        if ((((GetCollisionAtCoords(
            playerObjEvent,
            x,
            y,
            ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u32),
        )) as i32)
            == 3i32)
            && (((PlayerGetElevation()) as i32) == 3i32))
            && ((MetatileBehavior_IsSurfableFishableWater(
                ((MapGridGetMetatileBehaviorAt(((x) as i32), ((y) as i32))) as u8),
            )) != 0)
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
pub unsafe extern "C" fn ClearPlayerAvatarInfo() {
    unsafe {
        crate::c::memset((&raw mut gPlayerAvatar).cast::<u8>(), 0i32, 36u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerAvatarStateMask(flags: u8) {
    unsafe {
        let mut flags = flags;
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
        (__p1).write((((((__p1).read()) as i32) & 224i32) as u8));
        let __p2 = ((&raw mut gPlayerAvatar).cast::<u8>());
        (__p2).write((((((__p2).read()) as i32) | ((flags) as i32)) as u8));
    }
}
pub(crate) unsafe extern "C" fn GetPlayerAvatarStateTransitionByGraphicsId(
    graphicsId: u8,
    gender: u8,
) -> u8 {
    unsafe {
        let mut graphicsId = graphicsId;
        let mut gender = gender;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(10u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((&raw const sPlayerAvatarGfxToStateFlag)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(((gender) as i32) as isize * 10))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 2))
                    .read()) as i32)
                        == ((graphicsId) as i32)
                    {
                        return (((((((&raw const sPlayerAvatarGfxToStateFlag)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((gender) as i32) as isize * 10))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .wrapping_add(1))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetPlayerAvatarGraphicsIdByCurrentState() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut flags: u8 = ((&raw mut gPlayerAvatar).cast::<u8>()).read();
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(10u32, 2u32)) {
                    break 'l1;
                }
                'l2: {
                    if ((((((((((&raw const sPlayerAvatarGfxToStateFlag)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(7)).read()) as i32)
                            as isize
                            * 10,
                    ))
                    .cast::<u8>())
                    .wrapping_offset(((i) as i32) as isize * 2))
                    .wrapping_add(1))
                    .read()) as i32)
                        & ((flags) as i32))
                        != 0
                    {
                        return ((((((&raw const sPlayerAvatarGfxToStateFlag)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(7)).read())
                                as i32) as isize
                                * 10,
                        ))
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 2))
                        .read();
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerAvatarExtraStateTransition(graphicsId: u8, transitionFlag: u8) {
    unsafe {
        let mut graphicsId = graphicsId;
        let mut transitionFlag = transitionFlag;
        let mut stateFlag: u8 = GetPlayerAvatarStateTransitionByGraphicsId(
            graphicsId,
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(7)).read(),
        );
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(1);
        (__p1).write(
            (((((__p1).read()) as i32) | (((stateFlag) as i32) | ((transitionFlag) as i32))) as u8),
        );
        DoPlayerAvatarTransition();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitPlayerAvatar(x: i16, y: i16, direction: u8, gender: u8) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut direction = direction;
        let mut gender = gender;
        let mut playerObjEventTemplate = crate::ffi::Align4([0u8; 24]);
        let mut objectEventId: u8 = 0u8;
        let mut objectEvent: *mut u8 = core::ptr::null_mut();
        ((&raw mut playerObjEventTemplate).cast::<u8>()).write(255u8);
        (((&raw mut playerObjEventTemplate).cast::<u8>()).wrapping_add(1))
            .write(GetPlayerAvatarGraphicsIdByStateIdAndGender(0u8, gender));
        (((&raw mut playerObjEventTemplate).cast::<u8>())
            .wrapping_add(4)
            .cast::<i16>())
        .write(((((x) as i32).wrapping_sub(7i32)) as i16));
        (((&raw mut playerObjEventTemplate).cast::<u8>())
            .wrapping_add(6)
            .cast::<i16>())
        .write(((((y) as i32).wrapping_sub(7i32)) as i16));
        (((&raw mut playerObjEventTemplate).cast::<u8>()).wrapping_add(8)).write(0u8);
        (((&raw mut playerObjEventTemplate).cast::<u8>()).wrapping_add(9)).write(11u8);
        crate::c::bf_write(
            ((&raw mut playerObjEventTemplate).cast::<u8>()).wrapping_add(10),
            0,
            4,
            (0u16) as i32,
        );
        crate::c::bf_write(
            ((&raw mut playerObjEventTemplate).cast::<u8>()).wrapping_add(10),
            4,
            4,
            (0u16) as i32,
        );
        (((&raw mut playerObjEventTemplate).cast::<u8>())
            .wrapping_add(12)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut playerObjEventTemplate).cast::<u8>())
            .wrapping_add(14)
            .cast::<u16>())
        .write(0u16);
        (((&raw mut playerObjEventTemplate).cast::<u8>())
            .wrapping_add(16)
            .cast::<*mut u8>())
        .write(core::ptr::null_mut());
        (((&raw mut playerObjEventTemplate).cast::<u8>())
            .wrapping_add(20)
            .cast::<u16>())
        .write(0u16);
        objectEventId = SpawnSpecialObjectEvent((&raw mut playerObjEventTemplate).cast::<u8>());
        objectEvent = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((objectEventId) as i32) as isize * 36);
        crate::c::bf_write((objectEvent).wrapping_add(2), 0, 1, (1u32) as i32);
        ((objectEvent).wrapping_add(27)).write(CreateWarpArrowSprite());
        ObjectEventTurn(objectEvent, direction);
        ClearPlayerAvatarInfo();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(2)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).write(0u8);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).write(objectEventId);
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4))
            .write(((objectEvent).wrapping_add(4)).read());
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(7)).write(gender);
        SetPlayerAvatarStateMask(33u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerInvisibility(invisible: u8) {
    unsafe {
        let mut invisible = invisible;
        crate::c::bf_write(
            ((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(1),
            5,
            1,
            ((invisible) as u32) as i32,
        );
        if (TestPlayerAvatarFlags(8u8)) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize
                            * 36,
                    ))
                    .wrapping_add(26))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                ((invisible) as u16) as i32,
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerAvatarFieldMove() {
    unsafe {
        ObjectEventSetGraphicsId(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            GetPlayerAvatarGraphicsIdByStateId(5u8),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            0u8,
        );
    }
}
pub(crate) unsafe extern "C" fn SetPlayerAvatarFishing(direction: u8) {
    unsafe {
        let mut direction = direction;
        ObjectEventSetGraphicsId(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            GetPlayerAvatarGraphicsIdByStateId(6u8),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            GetFishingDirectionAnimNum(direction),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerUseAcroBikeOnBumpySlope(direction: u8) {
    unsafe {
        let mut direction = direction;
        ObjectEventSetGraphicsId(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            GetPlayerAvatarGraphicsIdByStateId(2u8),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            GetAcroWheelieDirectionAnimNum(direction),
        );
        SeekSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            1u8,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetPlayerAvatarWatering(direction: u8) {
    unsafe {
        let mut direction = direction;
        ObjectEventSetGraphicsId(
            (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            GetPlayerAvatarGraphicsIdByStateId(7u8),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            GetFaceDirectionAnimNum(direction),
        );
    }
}
pub(crate) unsafe extern "C" fn HideShowWarpArrow(objectEvent: *mut u8) {
    unsafe {
        let mut objectEvent = objectEvent;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut direction: u8 = 0u8;
        let mut metatileBehavior: u8 = ((objectEvent).wrapping_add(30)).read();
        {
            x = 0i16;
            direction = 1u8;
            'l1: loop {
                if !(((x) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    if (((((((&raw const sArrowWarpMetatileBehaviorChecks2)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                    .cast::<Option<unsafe extern "C" fn(u8) -> u8>>())
                    .wrapping_offset(((x) as i32) as isize))
                    .read())
                    .unwrap_unchecked()(metatileBehavior))
                        != 0)
                        && (((direction) as i32)
                            == ((crate::c::bf_read((objectEvent).wrapping_add(24), 4, 4, false)
                                as u16) as i32))
                    {
                        x = (((objectEvent).wrapping_add(16)).cast::<i16>()).read();
                        y = (((objectEvent).wrapping_add(16))
                            .wrapping_add(2)
                            .cast::<i16>())
                        .read();
                        MoveCoords(direction, &raw mut x, &raw mut y);
                        ShowWarpArrowSprite(
                            ((objectEvent).wrapping_add(27)).read(),
                            direction,
                            x,
                            y,
                        );
                        return;
                    }
                }
                x = (x).wrapping_add(1);
                direction = (direction).wrapping_add(1);
            }
        }
        SetSpriteInvisible(((objectEvent).wrapping_add(27)).read());
    }
}
pub(crate) unsafe extern "C" fn StartStrengthAnim(objectEventId: u8, direction: u8) {
    unsafe {
        let mut objectEventId = objectEventId;
        let mut direction = direction;
        let mut taskId: u8 = CreateTask(Some(Task_PushBoulder), 255u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((objectEventId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((direction) as i16));
        Task_PushBoulder(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_PushBoulder(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sPushBoulderFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32) as isize
                        * 36,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PushBoulder_Start(
    task: *mut u8,
    player: *mut u8,
    boulder: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut player = player;
        let mut boulder = boulder;
        LockPlayerFieldControls();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PushBoulder_Move(
    task: *mut u8,
    player: *mut u8,
    boulder: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut player = player;
        let mut boulder = boulder;
        if (ObjectEventIsHeldMovementActive(player)) != 0 {
            ObjectEventClearHeldMovementIfFinished(player);
        }
        if (ObjectEventIsHeldMovementActive(boulder)) != 0 {
            ObjectEventClearHeldMovementIfFinished(boulder);
        }
        if (!((ObjectEventIsMovementOverridden(player)) != 0))
            && (!((ObjectEventIsMovementOverridden(boulder)) != 0))
        {
            ObjectEventClearHeldMovementIfFinished(player);
            ObjectEventClearHeldMovementIfFinished(boulder);
            ObjectEventSetHeldMovement(
                player,
                GetWalkInPlaceNormalMovementAction(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8)
                        as u32),
                ),
            );
            ObjectEventSetHeldMovement(
                boulder,
                GetWalkSlowMovementAction(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u8)
                        as u32),
                ),
            );
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .write((((((boulder).wrapping_add(16)).cast::<i16>()).read()) as i32));
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .write(
                    (((((boulder).wrapping_add(16)).wrapping_add(2).cast::<i16>()).read()) as i32),
                );
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .write(((crate::c::bf_read((boulder).wrapping_add(11), 4, 4, false) as u8) as i32));
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(3))
                .write(
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((boulder).wrapping_add(4)).read()) as i32) as isize * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        false,
                    ) as u16) as i32),
                );
            FieldEffectStart(10u8);
            PlaySE(214u16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PushBoulder_End(
    task: *mut u8,
    player: *mut u8,
    boulder: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut player = player;
        let mut boulder = boulder;
        if ((ObjectEventCheckHeldMovementStatus(player)) != 0)
            && ((ObjectEventCheckHeldMovementStatus(boulder)) != 0)
        {
            ObjectEventClearHeldMovementIfFinished(player);
            ObjectEventClearHeldMovementIfFinished(boulder);
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            UnlockPlayerFieldControls();
            DestroyTask(FindTaskIdByFunc(Some(Task_PushBoulder)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoPlayerMatJump() {
    unsafe {
        DoPlayerAvatarSecretBaseMatJump(CreateTask(Some(DoPlayerAvatarSecretBaseMatJump), 255u8));
    }
}
pub(crate) unsafe extern "C" fn DoPlayerAvatarSecretBaseMatJump(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sPlayerAvatarSecretBaseMatJump)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatar_DoSecretBaseMatJump(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            PlaySE(10u16);
            ObjectEventSetHeldMovement(
                objectEvent,
                GetJumpInPlaceMovementAction(
                    ((crate::c::bf_read((objectEvent).wrapping_add(24), 0, 4, false) as u16)
                        as u32),
                ),
            );
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 1i32
            {
                (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
                let __p2 = ((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(1);
                (__p2).write((((((__p2).read()) as i32) | 32i32) as u8));
                DestroyTask(FindTaskIdByFunc(Some(DoPlayerAvatarSecretBaseMatJump)));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn DoPlayerMatSpin() {
    unsafe {
        let mut taskId: u8 = CreateTask(Some(PlayerAvatar_DoSecretBaseMatSpin), 255u8);
        PlayerAvatar_DoSecretBaseMatSpin(taskId);
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatar_DoSecretBaseMatSpin(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sPlayerAvatarSecretBaseMatSpin)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, *mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatar_SecretBaseMatSpinStep0(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            ((crate::c::bf_read((objectEvent).wrapping_add(24), 4, 4, false) as u16) as i16),
        );
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        LockPlayerFieldControls();
        PlaySE(45u16);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatar_SecretBaseMatSpinStep1(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut directions = crate::ffi::Align4([0u8; 4]);
        (&raw mut directions)
            .cast::<u8>()
            .wrapping_add(0)
            .write(3u8);
        (&raw mut directions)
            .cast::<u8>()
            .wrapping_add(1)
            .write(4u8);
        (&raw mut directions)
            .cast::<u8>()
            .wrapping_add(2)
            .write(2u8);
        (&raw mut directions)
            .cast::<u8>()
            .wrapping_add(3)
            .write(1u8);
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            let mut direction: u8 = 0u8;
            ObjectEventSetHeldMovement(
                objectEvent,
                GetFaceDirectionMovementAction(
                    (({
                        let __v1 = (((&raw mut directions).cast::<u8>()).wrapping_offset(
                            (((crate::c::bf_read((objectEvent).wrapping_add(24), 4, 4, false)
                                as u16) as i32)
                                .wrapping_sub(1i32)) as isize,
                        ))
                        .read();
                        direction = __v1;
                        __v1
                    }) as u32),
                ),
            );
            if ((direction) as i32)
                == (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u8)
                    as i32)
            {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            let __p3 = ((task).wrapping_add(8)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 3i32)
                && (((direction) as i32)
                    == ((GetOppositeDirection(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as u8),
                    )) as i32))
            {
                let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatar_SecretBaseMatSpinStep2(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        let mut actions = crate::ffi::Align4([0u8; 5]);
        (&raw mut actions).cast::<u8>().wrapping_add(0).write(16u8);
        (&raw mut actions).cast::<u8>().wrapping_add(1).write(16u8);
        (&raw mut actions).cast::<u8>().wrapping_add(2).write(17u8);
        (&raw mut actions).cast::<u8>().wrapping_add(3).write(18u8);
        (&raw mut actions).cast::<u8>().wrapping_add(4).write(19u8);
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            ObjectEventSetHeldMovement(
                objectEvent,
                (((&raw mut actions).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                        as isize,
                ))
                .read(),
            );
            (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PlayerAvatar_SecretBaseMatSpinStep3(
    task: *mut u8,
    objectEvent: *mut u8,
) -> u8 {
    unsafe {
        let mut task = task;
        let mut objectEvent = objectEvent;
        if (ObjectEventClearHeldMovementIfFinished(objectEvent)) != 0 {
            ObjectEventSetHeldMovement(
                objectEvent,
                GetWalkSlowMovementAction(
                    ((GetOppositeDirection(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as u8),
                    )) as u32),
                ),
            );
            UnlockPlayerFieldControls();
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            DestroyTask(FindTaskIdByFunc(Some(PlayerAvatar_DoSecretBaseMatSpin)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn CreateStopSurfingTask(direction: u8) {
    unsafe {
        let mut direction = direction;
        let mut taskId: u8 = 0u8;
        LockPlayerFieldControls();
        Overworld_ClearSavedMusic();
        Overworld_ChangeMusicToDefault();
        let __p1 = ((&raw mut gPlayerAvatar).cast::<u8>());
        (__p1).write((((((__p1).read()) as i32) & (-9i32)) as u8));
        let __p2 = ((&raw mut gPlayerAvatar).cast::<u8>());
        (__p2).write((((((__p2).read()) as i32) | 1i32) as u8));
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        taskId = CreateTask(Some(Task_StopSurfingInit), 255u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((direction) as i16));
        Task_StopSurfingInit(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_StopSurfingInit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        if (ObjectEventIsMovementOverridden(playerObjEvent)) != 0 {
            if !((ObjectEventClearHeldMovementIfFinished(playerObjEvent)) != 0) {
                return;
            }
        }
        SetSurfBlob_BobState(((playerObjEvent).wrapping_add(26)).read(), 2u8);
        ObjectEventSetHeldMovement(
            playerObjEvent,
            GetJumpSpecialMovementAction(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as u8) as u32),
            ),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(Task_WaitStopSurfing));
    }
}
pub(crate) unsafe extern "C" fn Task_WaitStopSurfing(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut playerObjEvent: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        if (ObjectEventClearHeldMovementIfFinished(playerObjEvent)) != 0 {
            ObjectEventSetGraphicsId(playerObjEvent, GetPlayerAvatarGraphicsIdByStateId(0u8));
            ObjectEventSetHeldMovement(
                playerObjEvent,
                GetFaceDirectionMovementAction(
                    ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16)
                        as u32),
                ),
            );
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            UnlockPlayerFieldControls();
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((playerObjEvent).wrapping_add(26)).read()) as i32) as isize * 68,
            ));
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartFishing(rod: u8) {
    unsafe {
        let mut rod = rod;
        let mut taskId: u8 = CreateTask(Some(Task_Fishing), 255u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((rod) as i16));
        Task_Fishing(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_Fishing(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: loop {
            if !(((((((&raw const sFishingStateFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8) -> u8>>())
            .wrapping_offset(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Fishing_Init(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        LockPlayerFieldControls();
        (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(1u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_GetRodOut(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut playerObjEvent: *mut u8 = core::ptr::null_mut();
        let mut minRounds1 = crate::ffi::Align4([0u8; 6]);
        (&raw mut minRounds1)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(1i16);
        (&raw mut minRounds1)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(1i16);
        (&raw mut minRounds1)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(1i16);
        let mut minRounds2 = crate::ffi::Align4([0u8; 6]);
        (&raw mut minRounds2)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(1i16);
        (&raw mut minRounds2)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(3i16);
        (&raw mut minRounds2)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(6i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            (((((((&raw mut minRounds1).cast::<i16>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
                .wrapping_add(crate::c::rem_i32(
                    ((Random()) as i32),
                    (((((&raw mut minRounds2).cast::<i16>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32),
                ))) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            (((((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ))
            .wrapping_add(5))
            .read()) as i16),
        );
        playerObjEvent = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        ObjectEventClearHeldMovementIfActive(playerObjEvent);
        crate::c::bf_write((playerObjEvent).wrapping_add(1), 3, 1, (1u32) as i32);
        SetPlayerAvatarFishing(
            ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_WaitBeforeDots(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) >= 60i32
        {
            let __p2 = ((task).wrapping_add(8)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_InitDots(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut randVal: u32 = 0u32;
        LoadMessageBoxAndFrameGfx(0u8, 1u8);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        randVal = ((Random()) as u32);
        randVal = crate::c::rem_u32(randVal, 10u32);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
            .write((((randVal).wrapping_add(1u32)) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32) == 0i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                .write((((randVal).wrapping_add(4u32)) as i16));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) >= 10i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(10i16);
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_ShowDots(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut dot = crate::ffi::Align4([0u8; 2]);
        (&raw mut dot).cast::<u8>().wrapping_add(0).write(175u8);
        (&raw mut dot).cast::<u8>().wrapping_add(1).write(255u8);
        AlignFishingAnimationFrames();
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((&raw mut gMain).cast::<u8>())
            .wrapping_add(46)
            .cast::<u16>())
        .read()) as i32)
            & 1i32)
            != 0
        {
            (((task).wrapping_add(8)).cast::<i16>()).write(11i16);
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
                != 0i32
            {
                (((task).wrapping_add(8)).cast::<i16>()).write(12i16);
            }
            return 1u8;
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >= 20i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                {
                    let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                        as i32)
                        != 0i32
                    {
                        let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p3).write(((__p3).read()).wrapping_add(1));
                    }
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                } else {
                    AddTextPrinterParameterized(
                        0u8,
                        1u8,
                        (&raw mut dot).cast::<u8>(),
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_mul(8i32)) as u8),
                        1u8,
                        0u8,
                        None,
                    );
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
            }
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn Fishing_CheckForBite(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut bite: u8 = 0u8;
        AlignFishingAnimationFrames();
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        bite = 0u8;
        if !((DoesCurrentMapHaveFishingMons()) != 0) {
            (((task).wrapping_add(8)).cast::<i16>()).write(11i16);
        } else {
            if !((GetMonData2((&raw mut gPlayerParty).cast::<u8>(), 6i32)) != 0) {
                let mut ability: u8 = GetMonAbility((&raw mut gPlayerParty).cast::<u8>());
                if (((ability) as i32) == 21i32) || (((ability) as i32) == 60i32) {
                    if crate::c::rem_i32(((Random()) as i32), 100i32) > 14i32 {
                        bite = 1u8;
                    }
                }
            }
            if !((bite) != 0) {
                if (((Random()) as i32) & 1i32) != 0 {
                    (((task).wrapping_add(8)).cast::<i16>()).write(11i16);
                } else {
                    bite = 1u8;
                }
            }
            if ((bite) as i32) == 1i32 {
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                            as isize
                            * 68,
                    ),
                    GetFishingBiteDirectionAnimNum(GetPlayerFacingDirection()),
                );
            }
        }
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_GotBite(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        AddTextPrinterParameterized(
            0u8,
            1u8,
            (&raw mut gText_OhABite).cast::<u8>(),
            0u8,
            17u8,
            0u8,
            None,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_WaitForA(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut reelTimeouts = crate::ffi::Align4([0u8; 6]);
        (&raw mut reelTimeouts)
            .cast::<u8>()
            .wrapping_add(0)
            .cast::<i16>()
            .write(36i16);
        (&raw mut reelTimeouts)
            .cast::<u8>()
            .wrapping_add(2)
            .cast::<i16>()
            .write(33i16);
        (&raw mut reelTimeouts)
            .cast::<u8>()
            .wrapping_add(4)
            .cast::<i16>()
            .write(30i16);
        AlignFishingAnimationFrames();
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= (((((&raw mut reelTimeouts).cast::<i16>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
        {
            (((task).wrapping_add(8)).cast::<i16>()).write(12i16);
        } else {
            if ((((((&raw mut gMain).cast::<u8>())
                .wrapping_add(46)
                .cast::<u16>())
            .read()) as i32)
                & 1i32)
                != 0
            {
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_CheckMoreDots(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut moreDotsChance = crate::ffi::Align4([0u8; 12]);
        (&raw mut moreDotsChance)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(0)
            .cast::<i16>()
            .write(0i16);
        (&raw mut moreDotsChance)
            .cast::<u8>()
            .wrapping_add(0)
            .wrapping_add(2)
            .cast::<i16>()
            .write(0i16);
        (&raw mut moreDotsChance)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(0)
            .cast::<i16>()
            .write(40i16);
        (&raw mut moreDotsChance)
            .cast::<u8>()
            .wrapping_add(4)
            .wrapping_add(2)
            .cast::<i16>()
            .write(10i16);
        (&raw mut moreDotsChance)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(0)
            .cast::<i16>()
            .write(70i16);
        (&raw mut moreDotsChance)
            .cast::<u8>()
            .wrapping_add(8)
            .wrapping_add(2)
            .cast::<i16>()
            .write(30i16);
        AlignFishingAnimationFrames();
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
        {
            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
                < 2i32
            {
                let mut probability: i16 =
                    ((crate::c::rem_i32(((Random()) as i32), 100i32)) as i16);
                if (((((((&raw mut moreDotsChance).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 4,
                ))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
                        as isize,
                ))
                .read()) as i32)
                    > ((probability) as i32)
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_MonOnHook(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gText_PokemonOnHook).cast::<u8>(),
            1u8,
            None,
            2u8,
            1u8,
            3u8,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_StartEncounter(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            AlignFishingAnimationFrames();
        }
        RunTextPrinters();
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 0i32 {
            if !((IsTextPrinterActive(0u8)) != 0) {
                let mut playerObjEvent: *mut u8 =
                    (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize
                            * 36,
                    );
                ObjectEventSetGraphicsId(
                    playerObjEvent,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
                );
                ObjectEventTurn(
                    playerObjEvent,
                    ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16)
                        as u8),
                );
                if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 8i32) != 0 {
                    SetSurfBlob_PlayerOffset(
                        (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read())
                                as i32) as isize
                                * 36,
                        ))
                        .wrapping_add(26))
                        .read(),
                        0u8,
                        0i16,
                    );
                }
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                ClearDialogWindowAndFrame(0u8, 1u8);
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p1).write(((__p1).read()).wrapping_add(1));
                return 0u8;
            }
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) != 0i32 {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            UnlockPlayerFieldControls();
            FishingWildEncounter(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            );
            RecordFishingAttemptForTV(1u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_Fishing)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_NotEvenNibble(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            GetFishingNoCatchDirectionAnimNum(GetPlayerFacingDirection()),
        );
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gText_NotEvenANibble).cast::<u8>(),
            1u8,
            None,
            2u8,
            1u8,
            3u8,
        );
        (((task).wrapping_add(8)).cast::<i16>()).write(13i16);
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_GotAway(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ),
            GetFishingNoCatchDirectionAnimNum(GetPlayerFacingDirection()),
        );
        FillWindowPixelBuffer(0u8, 17u8);
        AddTextPrinterParameterized2(
            0u8,
            1u8,
            (&raw mut gText_ItGotAway).cast::<u8>(),
            1u8,
            None,
            2u8,
            1u8,
            3u8,
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_NoMon(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_PutRodAway(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        AlignFishingAnimationFrames();
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(63),
            4,
            1,
            false,
        ) as u16)
            != 0
        {
            let mut playerObjEvent: *mut u8 =
                (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                );
            ObjectEventSetGraphicsId(
                playerObjEvent,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
            );
            ObjectEventTurn(
                playerObjEvent,
                ((crate::c::bf_read((playerObjEvent).wrapping_add(24), 4, 4, false) as u16) as u8),
            );
            if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 8i32) != 0 {
                SetSurfBlob_PlayerOffset(
                    (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                            as isize
                            * 36,
                    ))
                    .wrapping_add(26))
                    .read(),
                    0u8,
                    0i16,
                );
            }
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Fishing_EndNoMon(task: *mut u8) -> u8 {
    unsafe {
        let mut task = task;
        RunTextPrinters();
        if !((IsTextPrinterActive(0u8)) != 0) {
            (((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(6)).write(0u8);
            UnlockPlayerFieldControls();
            UnfreezeObjectEvents();
            ClearDialogWindowAndFrame(0u8, 1u8);
            RecordFishingAttemptForTV(0u8);
            DestroyTask(FindTaskIdByFunc(Some(Task_Fishing)));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AlignFishingAnimationFrames() {
    unsafe {
        let mut playerSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(4)).read()) as i32) as isize
                * 68,
        );
        let mut animCmdIndex: u8 = 0u8;
        let mut animType: u8 = 0u8;
        AnimateSprite(playerSprite);
        ((playerSprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ((playerSprite).wrapping_add(38).cast::<i16>()).write(0i16);
        animCmdIndex = ((playerSprite).wrapping_add(43)).read();
        if (((((((((playerSprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
            .wrapping_offset(((((playerSprite).wrapping_add(42)).read()) as i32) as isize))
        .read())
        .wrapping_offset(((animCmdIndex) as i32) as isize * 4))
        .cast::<i16>())
        .read()) as i32)
            == (-1i32)
        {
            animCmdIndex = (animCmdIndex).wrapping_sub(1);
        } else {
            crate::c::bf_write(
                (playerSprite).wrapping_add(44),
                0,
                6,
                ((crate::c::bf_read((playerSprite).wrapping_add(44), 0, 6, false) as u8)
                    .wrapping_add(1)) as i32,
            );
            if (((((((((playerSprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
                .wrapping_offset(((((playerSprite).wrapping_add(42)).read()) as i32) as isize))
            .read())
            .wrapping_offset(((animCmdIndex) as i32) as isize * 4))
            .cast::<i16>())
            .read()) as i32)
                == (-1i32)
            {
                animCmdIndex = (animCmdIndex).wrapping_sub(1);
            }
        }
        animType = (((((((((playerSprite).wrapping_add(8).cast::<*mut *mut u8>()).read())
            .wrapping_offset(((((playerSprite).wrapping_add(42)).read()) as i32) as isize))
        .read())
        .wrapping_offset(((animCmdIndex) as i32) as isize * 4))
        .cast::<i16>())
        .read()) as u8);
        if ((((animType) as i32) == 1i32) || (((animType) as i32) == 2i32))
            || (((animType) as i32) == 3i32)
        {
            ((playerSprite).wrapping_add(36).cast::<i16>()).write(8i16);
            if ((GetPlayerFacingDirection()) as i32) == 3i32 {
                ((playerSprite).wrapping_add(36).cast::<i16>()).write((-8i16));
            }
        }
        if ((animType) as i32) == 5i32 {
            ((playerSprite).wrapping_add(38).cast::<i16>()).write((-8i16));
        }
        if (((animType) as i32) == 10i32) || (((animType) as i32) == 11i32) {
            ((playerSprite).wrapping_add(38).cast::<i16>()).write(8i16);
        }
        if (((((&raw mut gPlayerAvatar).cast::<u8>()).read()) as i32) & 8i32) != 0 {
            SetSurfBlob_PlayerOffset(
                (((((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                        as isize
                        * 36,
                ))
                .wrapping_add(26))
                .read(),
                1u8,
                ((playerSprite).wrapping_add(38).cast::<i16>()).read(),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpinStartFacingDir(direction: u8) {
    unsafe {
        let mut direction = direction;
        ((&raw mut sSpinStartFacingDir).cast::<u8>().cast::<u8>()).write(direction);
    }
}
pub(crate) unsafe extern "C" fn GetSpinStartFacingDir() -> u8 {
    unsafe {
        if ((((&raw mut sSpinStartFacingDir).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            return 1u8;
        }
        return ((&raw mut sSpinStartFacingDir).cast::<u8>().cast::<u8>()).read();
    }
}
pub(crate) unsafe extern "C" fn Task_DoPlayerSpinExit(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut object: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((object).wrapping_add(4)).read()) as i32) as isize * 68);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((ObjectEventClearHeldMovementIfFinished(object)) != 0) {
                    return;
                }
                SetSpinStartFacingDir(
                    ((crate::c::bf_read((object).wrapping_add(24), 0, 4, false) as u16) as u8),
                );
                ((data).wrapping_offset(1)).write(0i16);
                ((data).wrapping_offset(2)).write(1i16);
                ((data).wrapping_offset(3)).write(
                    (((((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as u16) as i32)
                        << 4) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                CameraObjectFreeze();
                crate::c::bf_write((object).wrapping_add(3), 2, 1, (1u32) as i32);
                crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (0u16) as i32);
                ((sprite).wrapping_add(67)).write(0u8);
                crate::c::bf_write((sprite).wrapping_add(66), 6, 2, (0u8) as i32);
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                TrySpinPlayerForWarp(object, (data).wrapping_offset(1));
                let __p2 = (data).wrapping_offset(3);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_sub(((((data).wrapping_offset(2)).read()) as i32)))
                        as i16),
                );
                let __p3 = (data).wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(3i32)) as i16));
                ((sprite).wrapping_add(34).cast::<i16>())
                    .write(((((((data).wrapping_offset(3)).read()) as i32) >> 4) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                    (((((&raw mut gTotalCameraPixelOffsetY).cast::<u16>()).read()) as i16) as i32),
                ) < (-32i32)
                {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPlayerSpinEntrance() {
    unsafe {
        Task_DoPlayerSpinEntrance(CreateTask(Some(Task_DoPlayerSpinEntrance), 0u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerSpinEntranceActive() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_DoPlayerSpinEntrance))) as u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPlayerSpinExit() {
    unsafe {
        Task_DoPlayerSpinExit(CreateTask(Some(Task_DoPlayerSpinExit), 0u8));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsPlayerSpinExitActive() -> u32 {
    unsafe {
        return ((FuncIsActiveTask(Some(Task_DoPlayerSpinExit))) as u32);
    }
}
pub(crate) unsafe extern "C" fn Task_DoPlayerSpinEntrance(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut object: *mut u8 = (((&raw mut gObjectEvents).cast::<u8>()).cast::<u8>())
            .wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((((object).wrapping_add(4)).read()) as i32) as isize * 68);
        let mut data: *mut i16 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        'l1: {
            let __sw1 = (((data).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((data).wrapping_offset(5)).write(((GetSpinStartFacingDir()) as i16));
                ObjectEventForceSetHeldMovement(
                    object,
                    GetFaceDirectionMovementAction(
                        ((((((&raw const sSpinDirections).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((data).wrapping_offset(5)).read()) as i32) as isize,
                            ))
                        .read()) as u32),
                    ),
                );
                ((data).wrapping_offset(1)).write(0i16);
                ((data).wrapping_offset(2)).write(116i16);
                ((data).wrapping_offset(4)).write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((data).wrapping_offset(6)).write(
                    ((crate::c::bf_read((sprite).wrapping_add(5), 2, 2, false) as u16) as i16),
                );
                ((data).wrapping_offset(7)).write(((((sprite).wrapping_add(67)).read()) as i16));
                ((data).wrapping_offset(3)).write(
                    (((((((((sprite).wrapping_add(38).cast::<i16>()).read()) as u16) as i32)
                        .wrapping_add(32i32))
                    .wrapping_neg())
                    .wrapping_mul(16i32)) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                CameraObjectFreeze();
                crate::c::bf_write((object).wrapping_add(3), 2, 1, (1u32) as i32);
                crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
                ((sprite).wrapping_add(67)).write(0u8);
                crate::c::bf_write((sprite).wrapping_add(66), 6, 2, (0u8) as i32);
                (data).write(((data).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                TrySpinPlayerForWarp(object, (data).wrapping_offset(1));
                let __p2 = (data).wrapping_offset(3);
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((data).wrapping_offset(2)).read()) as i32)))
                        as i16),
                );
                let __p3 = (data).wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(3i32)) as i16));
                if ((((data).wrapping_offset(2)).read()) as i32) < 4i32 {
                    ((data).wrapping_offset(2)).write(4i16);
                }
                ((sprite).wrapping_add(34).cast::<i16>())
                    .write(((((((data).wrapping_offset(3)).read()) as i32) >> 4) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    >= ((((data).wrapping_offset(4)).read()) as i32)
                {
                    ((sprite).wrapping_add(34).cast::<i16>())
                        .write(((data).wrapping_offset(4)).read());
                    ((data).wrapping_offset(8)).write(0i16);
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                TrySpinPlayerForWarp(object, (data).wrapping_offset(1));
                if (({
                    let __p4 = (data).wrapping_offset(8);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 8i32
                {
                    (data).write(((data).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if ((((data).wrapping_offset(5)).read()) as i32)
                    == ((TrySpinPlayerForWarp(object, (data).wrapping_offset(1))) as i32)
                {
                    crate::c::bf_write((object).wrapping_add(3), 2, 1, (0u32) as i32);
                    crate::c::bf_write(
                        (sprite).wrapping_add(5),
                        2,
                        2,
                        ((((data).wrapping_offset(6)).read()) as u16) as i32,
                    );
                    ((sprite).wrapping_add(67)).write(((((data).wrapping_offset(7)).read()) as u8));
                    CameraObjectReset();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrySpinPlayerForWarp(object: *mut u8, delayTimer: *mut i16) -> u8 {
    unsafe {
        let mut object = object;
        let mut delayTimer = delayTimer;
        if ((((delayTimer).read()) as i32) < 8i32)
            && ((({
                let __t1 = ((delayTimer).read()).wrapping_add(1);
                (delayTimer).write(__t1);
                __t1
            }) as i32)
                < 8i32)
        {
            return ((crate::c::bf_read((object).wrapping_add(24), 0, 4, false) as u16) as u8);
        }
        if !((ObjectEventCheckHeldMovementStatus(object)) != 0) {
            return ((crate::c::bf_read((object).wrapping_add(24), 0, 4, false) as u16) as u8);
        }
        ObjectEventForceSetHeldMovement(
            object,
            GetFaceDirectionMovementAction(
                ((((((&raw const sSpinDirections).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        ((crate::c::bf_read((object).wrapping_add(24), 0, 4, false) as u16) as i32)
                            as isize,
                    ))
                .read()) as u32),
            ),
        );
        (delayTimer).write(0i16);
        return ((((&raw const sSpinDirections).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(
                ((crate::c::bf_read((object).wrapping_add(24), 0, 4, false) as u16) as i32)
                    as isize,
            ))
        .read();
    }
}
