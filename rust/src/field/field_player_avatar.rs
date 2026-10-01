//! Translated from `src/field_player_avatar.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations,
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::bike::{
    Bike_HandleBumpySlopeJump, Bike_TryAcroBikeHistoryUpdate, Bike_UpdateBikeCounterSpeed,
    BikeClearState, GetPlayerSpeed, IsPlayerNotUsingAcroBikeOnBumpySlope, IsRunningDisallowed,
    MovePlayerOnBike,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::FlagGet;
use crate::event_object_movement::{
    CameraObjectFreeze, CameraObjectReset, GetAcroEndWheelieFaceDirectionMovementAction,
    GetAcroEndWheelieMoveDirectionMovementAction, GetAcroPopWheelieFaceDirectionMovementAction,
    GetAcroPopWheelieMoveDirectionMovementAction, GetAcroWheelieDirectionAnimNum,
    GetAcroWheelieFaceDirectionMovementAction, GetAcroWheelieHopDirectionMovementAction,
    GetAcroWheelieHopFaceDirectionMovementAction, GetAcroWheelieInPlaceDirectionMovementAction,
    GetAcroWheelieJumpDirectionMovementAction, GetAcroWheelieMoveDirectionMovementAction,
    GetCollisionAtCoords, GetFaceDirectionAnimNum, GetFaceDirectionMovementAction,
    GetFishingBiteDirectionAnimNum, GetFishingDirectionAnimNum, GetFishingNoCatchDirectionAnimNum,
    GetJump2MovementAction, GetJumpInPlaceMovementAction, GetJumpInPlaceTurnAroundMovementAction,
    GetJumpSpecialMovementAction, GetLedgeJumpDirection, GetObjectEventIdByLocalIdAndMap,
    GetObjectEventIdByPosition, GetObjectEventIdByXY, GetOppositeDirection,
    GetPlayerRunMovementAction, GetRideWaterCurrentMovementAction, GetWalkFastMovementAction,
    GetWalkFasterMovementAction, GetWalkInPlaceFastMovementAction,
    GetWalkInPlaceNormalMovementAction, GetWalkInPlaceSlowMovementAction,
    GetWalkNormalMovementAction, GetWalkSlowMovementAction, MoveCoords, MoveObjectEventToMapCoords,
    ObjectEventCheckHeldMovementStatus, ObjectEventClearHeldMovement,
    ObjectEventClearHeldMovementIfActive, ObjectEventClearHeldMovementIfFinished,
    ObjectEventForceSetHeldMovement, ObjectEventGetHeldMovementActionId,
    ObjectEventIsHeldMovementActive, ObjectEventIsMovementOverridden, ObjectEventSetGraphicsId,
    ObjectEventSetHeldMovement, ObjectEventTurn, SetObjectEventDirection, SpawnSpecialObjectEvent,
    UnfreezeObjectEvents, UpdateObjectEventCurrentMovement,
};
use crate::field_camera::gTotalCameraPixelOffsetY;
use crate::field_effect::{FieldEffectStart, gFieldEffectArguments};
use crate::field_effect_helpers::{
    CreateWarpArrowSprite, SetSpriteInvisible, SetSurfBlob_BobState, SetSurfBlob_PlayerOffset,
    ShowWarpArrowSprite, StartUnderwaterSurfBlobBobbing,
};
use crate::fieldmap::{MapGridGetElevationAt, MapGridGetMetatileBehaviorAt};
use crate::menu::{
    AddTextPrinterParameterized2, ClearDialogWindowAndFrame, LoadMessageBoxAndFrameGfx,
};
use crate::metatile_behavior::{
    MetatileBehavior_IsNonAnimDoor, MetatileBehavior_IsSurfableFishableWater,
    MetatileBehavior_IsWarpDoor,
};
use crate::overworld::{
    IncrementGameStat, Overworld_ChangeMusicToDefault, Overworld_ClearSavedMusic,
};
use crate::party_menu::MonKnowsMove;
use crate::pokemon::{GetMonAbility, GetMonData2, gPlayerParty};
use crate::random::Random;
use crate::rotating_gate::{
    CheckForRotatingGatePuzzleCollision, CheckForRotatingGatePuzzleCollisionWithoutAnimation,
};
use crate::script::{LockPlayerFieldControls, UnlockPlayerFieldControls};
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
use crate::task::gTasks;
use crate::task::{task_get, task_set, task_set_func};
use crate::text::{IsTextPrinterActive, RunTextPrinters};
use crate::tv::RecordFishingAttemptForTV;
#[allow(unused_imports)]
use crate::types::*;
use crate::wild_encounter::{DoesCurrentMapHaveFishingMons, FishingWildEncounter};
use crate::window::FillWindowPixelBuffer;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AnimateSprite` with this module's view of its types.
#[inline]
unsafe fn AnimateSprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::AnimateSprite(a0 as _);
    }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
}
/// `SeekSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn SeekSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::SeekSpriteAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tStep: usize = 0;
const tBoulderObjId: usize = 1;
const tFrameCounter: usize = 1;
const tDirection: usize = 2;
const tNumDots: usize = 2;
const tDotsRequired: usize = 3;
const tRoundsPlayed: usize = 12;
const tMinRoundsRequired: usize = 13;
const tPlayerGfxId: usize = 14;
const tFishingRod: usize = 15;
// Data tables (translate with cdata.py): sForcedMovementTestFuncs sForcedMovementFuncs sPlayerNotOnBikeFuncs sAcroBikeTrickMetatiles sAcroBikeTrickCollisionTypes sPlayerAvatarTransitionFuncs sArrowWarpMetatileBehaviorChecks sRivalAvatarGfxIds sPlayerAvatarGfxIds sFRLGAvatarGfxIds sRSAvatarGfxIds sPlayerAvatarGfxToStateFlag sArrowWarpMetatileBehaviorChecks2 sPushBoulderFuncs sPlayerAvatarSecretBaseMatJump sPlayerAvatarSecretBaseMatSpin sFishingStateFuncs sSpinDirections

/// `__typeof__(sPlayerAvatarGfxToStateFlag[0][0])`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct sPlayerAvatarGfxToStateFlag_0_0_t {
    pub graphicsId: u8,
    pub playerFlag: u8,
}

unsafe impl Sync for sPlayerAvatarGfxToStateFlag_0_0_t {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sPlayerAvatarGfxToStateFlag_0_0_t>() == 2);
    assert!(offset_of!(sPlayerAvatarGfxToStateFlag_0_0_t, graphicsId) == 0);
    assert!(offset_of!(sPlayerAvatarGfxToStateFlag_0_0_t, playerFlag) == 1);
};

const FISHING_GOT_AWAY: i16 = 12;
const FISHING_NO_BITE: i16 = 11;
const FISHING_SHOW_RESULT: i16 = 13;
const FISHING_START_ROUND: i16 = 3;
const NUM_ACRO_BIKE_COLLISIONS: u8 = 5;
const NUM_FORCED_MOVEMENTS: u8 = 18;

static sAcroBikeTrickCollisionTypes: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::field_player_avatar::sAcroBikeTrickCollisionTypes).cast());
static sAcroBikeTrickMetatiles: Table<CArray<Option<unsafe fn(u8) -> u8>, 5>> =
    Table((&raw const crate::data::field_player_avatar::sAcroBikeTrickMetatiles).cast());
static sArrowWarpMetatileBehaviorChecks: Table<CArray<Option<unsafe fn(u8) -> u8>, 4>> =
    Table((&raw const crate::data::field_player_avatar::sArrowWarpMetatileBehaviorChecks).cast());
static sArrowWarpMetatileBehaviorChecks2: Table<CArray<Option<unsafe fn(u8) -> u8>, 4>> =
    Table((&raw const crate::data::field_player_avatar::sArrowWarpMetatileBehaviorChecks2).cast());
static sFRLGAvatarGfxIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::field_player_avatar::sFRLGAvatarGfxIds).cast());
static sFishingStateFuncs: Table<CArray<Option<unsafe fn(*mut Task) -> u8>, 16>> =
    Table((&raw const crate::data::field_player_avatar::sFishingStateFuncs).cast());
static sForcedMovementFuncs: Table<CArray<Option<unsafe fn() -> u8>, 19>> =
    Table((&raw const crate::data::field_player_avatar::sForcedMovementFuncs).cast());
static sForcedMovementTestFuncs: Table<CArray<Option<unsafe fn(u8) -> u8>, 18>> =
    Table((&raw const crate::data::field_player_avatar::sForcedMovementTestFuncs).cast());
static sPlayerAvatarGfxIds: Table<CArray<CArray<u8, 2>, 8>> =
    Table((&raw const crate::data::field_player_avatar::sPlayerAvatarGfxIds).cast());
static sPlayerAvatarGfxToStateFlag: Table<CArray<CArray<sPlayerAvatarGfxToStateFlag_0_0_t, 5>, 2>> =
    Table((&raw const crate::data::field_player_avatar::sPlayerAvatarGfxToStateFlag).cast());
static sPlayerAvatarSecretBaseMatJump: Table<
    CArray<Option<unsafe fn(*mut Task, *mut ObjectEvent) -> u8>, 1>,
> = Table((&raw const crate::data::field_player_avatar::sPlayerAvatarSecretBaseMatJump).cast());
static sPlayerAvatarSecretBaseMatSpin: Table<
    CArray<Option<unsafe fn(*mut Task, *mut ObjectEvent) -> u8>, 4>,
> = Table((&raw const crate::data::field_player_avatar::sPlayerAvatarSecretBaseMatSpin).cast());
static sPlayerAvatarTransitionFuncs: Table<CArray<Option<unsafe fn(*mut ObjectEvent)>, 8>> =
    Table((&raw const crate::data::field_player_avatar::sPlayerAvatarTransitionFuncs).cast());
static sPlayerNotOnBikeFuncs: Table<CArray<Option<unsafe fn(u8, u16)>, 3>> =
    Table((&raw const crate::data::field_player_avatar::sPlayerNotOnBikeFuncs).cast());
static sPushBoulderFuncs: Table<
    CArray<Option<unsafe fn(*mut Task, *mut ObjectEvent, *mut ObjectEvent) -> u8>, 3>,
> = Table((&raw const crate::data::field_player_avatar::sPushBoulderFuncs).cast());
static sRSAvatarGfxIds: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::field_player_avatar::sRSAvatarGfxIds).cast());
static sRivalAvatarGfxIds: Table<CArray<CArray<u8, 2>, 8>> =
    Table((&raw const crate::data::field_player_avatar::sRivalAvatarGfxIds).cast());
static sSpinDirections: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::field_player_avatar::sSpinDirections).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sSpinStartFacingDir: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gObjectEvents: CArray<ObjectEvent, 16> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gPlayerAvatar: PlayerAvatar = unsafe { zeroed() };

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

pub unsafe fn MovementType_Player(sprite: *mut Sprite) {
    UpdateObjectEventCurrentMovement(
        &raw mut gObjectEvents[(*sprite).data[0]],
        sprite,
        core::mem::transmute::<
            Option<unsafe fn() -> u8>,
            Option<unsafe fn(*mut ObjectEvent, *mut Sprite) -> u8>,
        >(Some(ObjectEventCB2_NoMovement2)),
    );
}
pub(crate) unsafe fn ObjectEventCB2_NoMovement2() -> u8 {
    0
}
pub unsafe fn PlayerStep(direction: u8, newKeys: u16, heldKeys: u16) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    HideShowWarpArrow(playerObjEvent);
    if gPlayerAvatar.preventStep == FALSE {
        Bike_TryAcroBikeHistoryUpdate(newKeys, heldKeys);
        if TryInterruptObjectEventSpecialAnim(playerObjEvent, direction) == 0 {
            npc_clear_strange_bits(playerObjEvent);
            DoPlayerAvatarTransition();
            if TryDoMetatileBehaviorForcedMovement() == 0 {
                MovePlayerAvatarUsingKeypadInput(direction, newKeys, heldKeys);
                PlayerAllowForcedMovementIfMovingSameDirection();
            }
        }
    }
}
unsafe fn TryInterruptObjectEventSpecialAnim(
    playerObjEvent: *mut ObjectEvent,
    direction: u8,
) -> u8 {
    if ObjectEventIsMovementOverridden(playerObjEvent) != 0
        && ObjectEventClearHeldMovementIfFinished(playerObjEvent) == 0
    {
        let heldMovementActionId: u8 = ObjectEventGetHeldMovementActionId(playerObjEvent);
        if heldMovementActionId > MOVEMENT_ACTION_WALK_FAST_RIGHT
            && heldMovementActionId < MOVEMENT_ACTION_WALK_IN_PLACE_NORMAL_DOWN
        {
            if direction == DIR_NONE {
                return TRUE;
            }
            if (*playerObjEvent).movementDirection() != direction as u16 {
                ObjectEventClearHeldMovement(playerObjEvent);
                return FALSE;
            }
            if CheckForPlayerAvatarStaticCollision(direction) == COLLISION_NONE {
                ObjectEventClearHeldMovement(playerObjEvent);
                return FALSE;
            }
        }
        return TRUE;
    }
    FALSE
}
unsafe fn npc_clear_strange_bits(objEvent: *mut ObjectEvent) {
    (*objEvent).set_inanimate(FALSE as u32);
    (*objEvent).set_disableAnim(FALSE as u32);
    (*objEvent).set_facingDirectionLocked(FALSE as u32);
    gPlayerAvatar.flags &= 127;
}
unsafe fn MovePlayerAvatarUsingKeypadInput(direction: u8, newKeys: u16, heldKeys: u16) {
    if gPlayerAvatar.flags as i32 & 6 != 0 {
        MovePlayerOnBike(direction, newKeys, heldKeys);
    } else {
        MovePlayerNotOnBike(direction, heldKeys);
    }
}
unsafe fn PlayerAllowForcedMovementIfMovingSameDirection() {
    if gPlayerAvatar.runningState == MOVING {
        gPlayerAvatar.flags &= 223;
    }
}
unsafe fn TryDoMetatileBehaviorForcedMovement() -> u8 {
    sForcedMovementFuncs[GetForcedMovementByMetatileBehavior()].unwrap_unchecked()()
}
unsafe fn GetForcedMovementByMetatileBehavior() -> u8 {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_CONTROLLABLE as i32 == 0 {
        let metatileBehavior: u8 =
            gObjectEvents[gPlayerAvatar.objectEventId].currentMetatileBehavior;
        for i in 0..NUM_FORCED_MOVEMENTS {
            if sForcedMovementTestFuncs[i].unwrap_unchecked()(metatileBehavior) != 0 {
                return i + 1;
            }
        }
    }
    0
}
pub(crate) unsafe fn ForcedMovement_None() -> u8 {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_FORCED_MOVE != 0 {
        let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        (*playerObjEvent).set_facingDirectionLocked(FALSE as u32);
        (*playerObjEvent).set_enableAnim(TRUE as u32);
        SetObjectEventDirection(playerObjEvent, (*playerObjEvent).facingDirection() as u8);
        gPlayerAvatar.flags &= 191;
    }
    FALSE
}
unsafe fn DoForcedMovement(direction: u8, moveFunc: Option<unsafe fn(u8)>) -> u8 {
    let playerAvatar: *mut PlayerAvatar = &raw mut gPlayerAvatar;
    let collision: u8 = CheckForPlayerAvatarCollision(direction);
    (*playerAvatar).flags |= PLAYER_AVATAR_FLAG_FORCED_MOVE as u8;
    if collision != 0 {
        ForcedMovement_None();
        if collision < COLLISION_STOP_SURFING {
            return FALSE;
        } else {
            if collision == COLLISION_LEDGE_JUMP {
                PlayerJumpLedge(direction);
            }
            (*playerAvatar).flags |= PLAYER_AVATAR_FLAG_FORCED_MOVE as u8;
            (*playerAvatar).runningState = MOVING;
            return TRUE;
        }
    } else {
        (*playerAvatar).runningState = MOVING;
        moveFunc.unwrap_unchecked()(direction);
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn DoForcedMovementInCurrentDirection(moveFunc: Option<unsafe fn(u8)>) -> u8 {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    (*playerObjEvent).set_disableAnim(TRUE as u32);
    DoForcedMovement((*playerObjEvent).movementDirection() as u8, moveFunc)
}
pub(crate) unsafe fn ForcedMovement_Slip() -> u8 {
    DoForcedMovementInCurrentDirection(Some(PlayerWalkFast))
}
pub(crate) unsafe fn ForcedMovement_WalkSouth() -> u8 {
    DoForcedMovement(DIR_SOUTH, Some(PlayerWalkNormal))
}
pub(crate) unsafe fn ForcedMovement_WalkNorth() -> u8 {
    DoForcedMovement(DIR_NORTH, Some(PlayerWalkNormal))
}
pub(crate) unsafe fn ForcedMovement_WalkWest() -> u8 {
    DoForcedMovement(DIR_WEST, Some(PlayerWalkNormal))
}
pub(crate) unsafe fn ForcedMovement_WalkEast() -> u8 {
    DoForcedMovement(DIR_EAST, Some(PlayerWalkNormal))
}
pub(crate) unsafe fn ForcedMovement_PushedSouthByCurrent() -> u8 {
    DoForcedMovement(DIR_SOUTH, Some(PlayerRideWaterCurrent))
}
pub(crate) unsafe fn ForcedMovement_PushedNorthByCurrent() -> u8 {
    DoForcedMovement(DIR_NORTH, Some(PlayerRideWaterCurrent))
}
pub(crate) unsafe fn ForcedMovement_PushedWestByCurrent() -> u8 {
    DoForcedMovement(DIR_WEST, Some(PlayerRideWaterCurrent))
}
pub(crate) unsafe fn ForcedMovement_PushedEastByCurrent() -> u8 {
    DoForcedMovement(DIR_EAST, Some(PlayerRideWaterCurrent))
}
unsafe fn ForcedMovement_Slide(direction: u8, moveFunc: Option<unsafe fn(u8)>) -> u8 {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    (*playerObjEvent).set_disableAnim(TRUE as u32);
    (*playerObjEvent).set_facingDirectionLocked(TRUE as u32);
    DoForcedMovement(direction, moveFunc)
}
pub(crate) unsafe fn ForcedMovement_SlideSouth() -> u8 {
    ForcedMovement_Slide(DIR_SOUTH, Some(PlayerWalkFast))
}
pub(crate) unsafe fn ForcedMovement_SlideNorth() -> u8 {
    ForcedMovement_Slide(DIR_NORTH, Some(PlayerWalkFast))
}
pub(crate) unsafe fn ForcedMovement_SlideWest() -> u8 {
    ForcedMovement_Slide(DIR_WEST, Some(PlayerWalkFast))
}
pub(crate) unsafe fn ForcedMovement_SlideEast() -> u8 {
    ForcedMovement_Slide(DIR_EAST, Some(PlayerWalkFast))
}
pub(crate) unsafe fn ForcedMovement_MatJump() -> u8 {
    DoPlayerMatJump();
    TRUE
}
pub(crate) unsafe fn ForcedMovement_MatSpin() -> u8 {
    DoPlayerMatSpin();
    TRUE
}
pub(crate) unsafe fn ForcedMovement_MuddySlope() -> u8 {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if (*playerObjEvent).movementDirection() != DIR_NORTH as u16
        || GetPlayerSpeed() < PLAYER_SPEED_FASTEST
    {
        Bike_UpdateBikeCounterSpeed(0);
        (*playerObjEvent).set_facingDirectionLocked(TRUE as u32);
        return DoForcedMovement(DIR_SOUTH, Some(PlayerWalkFast));
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn MovePlayerNotOnBike(direction: u8, heldKeys: u16) {
    sPlayerNotOnBikeFuncs[CheckMovementInputNotOnBike(direction)].unwrap_unchecked()(
        direction, heldKeys,
    );
}
unsafe fn CheckMovementInputNotOnBike(direction: u8) -> u8 {
    if direction == DIR_NONE {
        return {
            gPlayerAvatar.runningState = NOT_MOVING;
            gPlayerAvatar.runningState
        };
    } else if direction != GetPlayerMovementDirection() && gPlayerAvatar.runningState != MOVING {
        return {
            gPlayerAvatar.runningState = TURN_DIRECTION;
            gPlayerAvatar.runningState
        };
    } else {
        return {
            gPlayerAvatar.runningState = MOVING;
            gPlayerAvatar.runningState
        };
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn PlayerNotOnBikeNotMoving(direction: u8, heldKeys: u16) {
    PlayerFaceDirection(GetPlayerFacingDirection());
}
pub(crate) unsafe fn PlayerNotOnBikeTurningInPlace(direction: u8, heldKeys: u16) {
    PlayerTurnInPlace(direction);
}
pub(crate) unsafe fn PlayerNotOnBikeMoving(direction: u8, heldKeys: u16) {
    let collision: u8 = CheckForPlayerAvatarCollision(direction);
    if collision != 0 {
        if collision == COLLISION_LEDGE_JUMP {
            PlayerJumpLedge(direction);
            return;
        } else if collision == COLLISION_OBJECT_EVENT
            && IsPlayerCollidingWithFarawayIslandMew(direction) != 0
        {
            PlayerNotOnBikeCollideWithFarawayIslandMew(direction);
            return;
        } else {
            if collision != COLLISION_STOP_SURFING
                && collision != COLLISION_LEDGE_JUMP
                && collision != COLLISION_PUSHED_BOULDER
                && collision != COLLISION_ROTATING_GATE
            {
                PlayerNotOnBikeCollide(direction);
            }
            return;
        }
    }
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
        PlayerWalkFast(direction);
        return;
    }
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_UNDERWATER as i32 == 0
        && heldKeys as i32 & B_BUTTON != 0
        && FlagGet(FLAG_SYS_B_DASH) != 0
        && IsRunningDisallowed(gObjectEvents[gPlayerAvatar.objectEventId].currentMetatileBehavior)
            == 0
    {
        PlayerRun(direction);
        gPlayerAvatar.flags |= PLAYER_AVATAR_FLAG_DASH;
    } else {
        PlayerWalkNormal(direction);
    }
}
unsafe fn CheckForPlayerAvatarCollision(direction: u8) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    x = (*playerObjEvent).currentCoords.x;
    y = (*playerObjEvent).currentCoords.y;
    MoveCoords(direction, &raw mut x, &raw mut y);
    CheckForObjectEventCollision(
        playerObjEvent,
        x,
        y,
        direction,
        MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8,
    )
}
unsafe fn CheckForPlayerAvatarStaticCollision(direction: u8) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    x = (*playerObjEvent).currentCoords.x;
    y = (*playerObjEvent).currentCoords.y;
    MoveCoords(direction, &raw mut x, &raw mut y);
    CheckForObjectEventStaticCollision(
        playerObjEvent,
        x,
        y,
        direction,
        MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8,
    )
}
pub unsafe fn CheckForObjectEventCollision(
    objectEvent: *mut ObjectEvent,
    x: i16,
    y: i16,
    direction: u8,
    metatileBehavior: u8,
) -> u8 {
    let mut collision: u8 = GetCollisionAtCoords(objectEvent, x, y, direction as u32);
    if collision == COLLISION_ELEVATION_MISMATCH && CanStopSurfing(x, y, direction) != 0 {
        return COLLISION_STOP_SURFING;
    }
    if ShouldJumpLedge(x, y, direction) != 0 {
        IncrementGameStat(GAME_STAT_JUMPED_DOWN_LEDGES);
        return COLLISION_LEDGE_JUMP;
    }
    if collision == COLLISION_OBJECT_EVENT && TryPushBoulder(x, y, direction) != 0 {
        return COLLISION_PUSHED_BOULDER;
    }
    if collision == COLLISION_NONE {
        if CheckForRotatingGatePuzzleCollision(direction, x, y) != 0 {
            return COLLISION_ROTATING_GATE;
        }
        CheckAcroBikeCollision(x, y, metatileBehavior, &raw mut collision);
    }
    collision
}
unsafe fn CheckForObjectEventStaticCollision(
    objectEvent: *mut ObjectEvent,
    x: i16,
    y: i16,
    direction: u8,
    metatileBehavior: u8,
) -> u8 {
    let mut collision: u8 = GetCollisionAtCoords(objectEvent, x, y, direction as u32);
    if collision == COLLISION_NONE {
        if CheckForRotatingGatePuzzleCollisionWithoutAnimation(direction, x, y) != 0 {
            return COLLISION_ROTATING_GATE;
        }
        CheckAcroBikeCollision(x, y, metatileBehavior, &raw mut collision);
    }
    collision
}
unsafe fn CanStopSurfing(x: i16, y: i16, direction: u8) -> u8 {
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0
        && MapGridGetElevationAt(x as i32, y as i32) == ELEVATION_DEFAULT
        && GetObjectEventIdByPosition(x as u16, y as u16, ELEVATION_DEFAULT) == OBJECT_EVENTS_COUNT
    {
        CreateStopSurfingTask(direction);
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn ShouldJumpLedge(x: i16, y: i16, direction: u8) -> u8 {
    if GetLedgeJumpDirection(x, y, direction) != DIR_NONE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn TryPushBoulder(mut x: i16, mut y: i16, direction: u8) -> u8 {
    if FlagGet(FLAG_SYS_USE_STRENGTH) != 0 {
        let objectEventId: u8 = GetObjectEventIdByXY(x, y);
        if objectEventId != OBJECT_EVENTS_COUNT
            && gObjectEvents[objectEventId].graphicsId == OBJ_EVENT_GFX_PUSHABLE_BOULDER
        {
            x = gObjectEvents[objectEventId].currentCoords.x;
            y = gObjectEvents[objectEventId].currentCoords.y;
            MoveCoords(direction, &raw mut x, &raw mut y);
            if GetCollisionAtCoords(
                &raw mut gObjectEvents[objectEventId],
                x,
                y,
                direction as u32,
            ) == COLLISION_NONE
                && MetatileBehavior_IsNonAnimDoor(
                    MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8
                ) == FALSE
            {
                StartStrengthAnim(objectEventId, direction);
                return TRUE;
            }
        }
    }
    FALSE
}
unsafe fn CheckAcroBikeCollision(x: i16, y: i16, metatileBehavior: u8, collision: *mut u8) {
    for i in 0..NUM_ACRO_BIKE_COLLISIONS {
        if sAcroBikeTrickMetatiles[i].unwrap_unchecked()(metatileBehavior) != 0 {
            *collision = sAcroBikeTrickCollisionTypes[i];
            return;
        }
    }
}
pub unsafe fn IsPlayerCollidingWithFarawayIslandMew(direction: u8) -> u8 {
    let mut object: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut playerX: i16 = (*object).currentCoords.x;
    let mut playerY: i16 = (*object).currentCoords.y;
    MoveCoords(direction, &raw mut playerX, &raw mut playerY);
    let mewObjectId: u8 = GetObjectEventIdByLocalIdAndMap(LOCALID_FARAWAY_ISLAND_MEW, 57, 26);
    if mewObjectId == OBJECT_EVENTS_COUNT {
        return FALSE;
    }
    object = &raw mut gObjectEvents[mewObjectId];
    let mewPrevX: i16 = (*object).previousCoords.x;
    if mewPrevX == playerX
        && ((*object).previousCoords.y != playerY
            || (*object).currentCoords.x != mewPrevX
            || (*object).currentCoords.y != (*object).previousCoords.y)
        && (*object).previousCoords.x == playerX
        && (*object).previousCoords.y == playerY
    {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn SetPlayerAvatarTransitionFlags(transitionFlags: u16) {
    gPlayerAvatar.transitionFlags |= transitionFlags as u8;
    DoPlayerAvatarTransition();
}
unsafe fn DoPlayerAvatarTransition() {
    let mut i: u8 = 0;
    let mut flags: u8 = gPlayerAvatar.transitionFlags;
    if flags != 0 {
        i = 0;
        while i < 8 {
            if flags as i32 & 1 != 0 {
                sPlayerAvatarTransitionFuncs[i].unwrap_unchecked()(
                    &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
                );
            }
            i += 1;
            flags >>= 1;
        }
        gPlayerAvatar.transitionFlags = 0;
    }
}
pub(crate) fn PlayerAvatarTransition_Dummy(objEvent: *mut ObjectEvent) {}
pub(crate) unsafe fn PlayerAvatarTransition_Normal(objEvent: *mut ObjectEvent) {
    ObjectEventSetGraphicsId(
        objEvent,
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_NORMAL),
    );
    ObjectEventTurn(objEvent, (*objEvent).movementDirection() as u8);
    SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_ON_FOOT);
}
pub(crate) unsafe fn PlayerAvatarTransition_MachBike(objEvent: *mut ObjectEvent) {
    ObjectEventSetGraphicsId(
        objEvent,
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_MACH_BIKE),
    );
    ObjectEventTurn(objEvent, (*objEvent).movementDirection() as u8);
    SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_MACH_BIKE);
    BikeClearState(0, 0);
}
pub(crate) unsafe fn PlayerAvatarTransition_AcroBike(objEvent: *mut ObjectEvent) {
    ObjectEventSetGraphicsId(
        objEvent,
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_ACRO_BIKE),
    );
    ObjectEventTurn(objEvent, (*objEvent).movementDirection() as u8);
    SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_ACRO_BIKE);
    BikeClearState(0, 0);
    Bike_HandleBumpySlopeJump();
}
pub(crate) unsafe fn PlayerAvatarTransition_Surfing(objEvent: *mut ObjectEvent) {
    ObjectEventSetGraphicsId(
        objEvent,
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_SURFING),
    );
    ObjectEventTurn(objEvent, (*objEvent).movementDirection() as u8);
    SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_SURFING);
    gFieldEffectArguments[0] = (*objEvent).currentCoords.x as i32;
    gFieldEffectArguments[1] = (*objEvent).currentCoords.y as i32;
    gFieldEffectArguments[2] = gPlayerAvatar.objectEventId as i32;
    let spriteId: u8 = FieldEffectStart(FLDEFF_SURF_BLOB) as u8;
    (*objEvent).fieldEffectSpriteId = spriteId;
    SetSurfBlob_BobState(spriteId, BOB_PLAYER_AND_MON);
}
pub(crate) unsafe fn PlayerAvatarTransition_Underwater(objEvent: *mut ObjectEvent) {
    ObjectEventSetGraphicsId(
        objEvent,
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_UNDERWATER),
    );
    ObjectEventTurn(objEvent, (*objEvent).movementDirection() as u8);
    SetPlayerAvatarStateMask(PLAYER_AVATAR_FLAG_UNDERWATER);
    (*objEvent).fieldEffectSpriteId = StartUnderwaterSurfBlobBobbing((*objEvent).spriteId);
}
pub(crate) unsafe fn PlayerAvatarTransition_ReturnToField(objEvent: *mut ObjectEvent) {
    gPlayerAvatar.flags |= PLAYER_AVATAR_FLAG_CONTROLLABLE;
}
pub unsafe fn UpdatePlayerAvatarTransitionState() {
    gPlayerAvatar.tileTransitionState = T_NOT_MOVING;
    if PlayerIsAnimActive() != 0 {
        if PlayerCheckIfAnimFinishedOrInactive() == 0 {
            if PlayerAnimIsMultiFrameStationary() == 0 {
                gPlayerAvatar.tileTransitionState = T_TILE_TRANSITION;
            }
        } else {
            if PlayerAnimIsMultiFrameStationaryAndStateNotTurning() == 0 {
                gPlayerAvatar.tileTransitionState = T_TILE_CENTER;
            }
        }
    }
}
unsafe fn PlayerAnimIsMultiFrameStationary() -> u8 {
    let movementActionId: u8 = gObjectEvents[gPlayerAvatar.objectEventId].movementActionId;
    if movementActionId <= MOVEMENT_ACTION_FACE_RIGHT
        || (MOVEMENT_ACTION_DELAY_1..=MOVEMENT_ACTION_DELAY_16).contains(&movementActionId)
        || (MOVEMENT_ACTION_WALK_IN_PLACE_SLOW_DOWN..=MOVEMENT_ACTION_WALK_IN_PLACE_FASTER_RIGHT)
            .contains(&movementActionId)
        || (MOVEMENT_ACTION_ACRO_WHEELIE_FACE_DOWN..=MOVEMENT_ACTION_ACRO_END_WHEELIE_FACE_RIGHT)
            .contains(&movementActionId)
        || (MOVEMENT_ACTION_ACRO_WHEELIE_IN_PLACE_DOWN
            ..=MOVEMENT_ACTION_ACRO_WHEELIE_IN_PLACE_RIGHT)
            .contains(&movementActionId)
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PlayerAnimIsMultiFrameStationaryAndStateNotTurning() -> u8 {
    if PlayerAnimIsMultiFrameStationary() != 0 && gPlayerAvatar.runningState != TURN_DIRECTION {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn PlayerIsAnimActive() -> u8 {
    ObjectEventIsMovementOverridden(&raw mut gObjectEvents[gPlayerAvatar.objectEventId])
}
unsafe fn PlayerCheckIfAnimFinishedOrInactive() -> u8 {
    ObjectEventCheckHeldMovementStatus(&raw mut gObjectEvents[gPlayerAvatar.objectEventId])
}
unsafe fn PlayerSetCopyableMovement(movement: u8) {
    gObjectEvents[gPlayerAvatar.objectEventId].playerCopyableMovement = movement;
}
pub unsafe fn PlayerGetCopyableMovement() -> u8 {
    gObjectEvents[gPlayerAvatar.objectEventId].playerCopyableMovement
}
unsafe fn PlayerForceSetHeldMovement(movementActionId: u8) {
    ObjectEventForceSetHeldMovement(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        movementActionId,
    );
}
pub unsafe fn PlayerSetAnimId(movementActionId: u8, copyableMovement: u8) {
    if PlayerIsAnimActive() == 0 {
        PlayerSetCopyableMovement(copyableMovement);
        ObjectEventSetHeldMovement(
            &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
            movementActionId,
        );
    }
}
pub unsafe fn PlayerWalkNormal(direction: u8) {
    PlayerSetAnimId(
        GetWalkNormalMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerWalkFast(direction: u8) {
    PlayerSetAnimId(GetWalkFastMovementAction(direction as u32), COPY_MOVE_WALK);
}
pub unsafe fn PlayerRideWaterCurrent(direction: u8) {
    PlayerSetAnimId(
        GetRideWaterCurrentMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerWalkFaster(direction: u8) {
    PlayerSetAnimId(
        GetWalkFasterMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
unsafe fn PlayerRun(direction: u8) {
    PlayerSetAnimId(GetPlayerRunMovementAction(direction as u32), COPY_MOVE_WALK);
}
pub unsafe fn PlayerOnBikeCollide(direction: u8) {
    PlayCollisionSoundIfNotFacingWarp(direction);
    PlayerSetAnimId(
        GetWalkInPlaceNormalMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerOnBikeCollideWithFarawayIslandMew(direction: u8) {
    PlayerSetAnimId(
        GetWalkInPlaceNormalMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
unsafe fn PlayerNotOnBikeCollide(direction: u8) {
    PlayCollisionSoundIfNotFacingWarp(direction);
    PlayerSetAnimId(
        GetWalkInPlaceSlowMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
unsafe fn PlayerNotOnBikeCollideWithFarawayIslandMew(direction: u8) {
    PlayerSetAnimId(
        GetWalkInPlaceSlowMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerFaceDirection(direction: u8) {
    PlayerSetAnimId(
        GetFaceDirectionMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerTurnInPlace(direction: u8) {
    PlayerSetAnimId(
        GetWalkInPlaceFastMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerJumpLedge(direction: u8) {
    PlaySE(SE_LEDGE);
    PlayerSetAnimId(GetJump2MovementAction(direction as u32), COPY_MOVE_JUMP2);
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerFreeze() {
    if (gPlayerAvatar.tileTransitionState == T_TILE_CENTER
        || gPlayerAvatar.tileTransitionState == T_NOT_MOVING)
        && IsPlayerNotUsingAcroBikeOnBumpySlope() != 0
    {
        PlayerForceSetHeldMovement(GetFaceDirectionMovementAction(
            gObjectEvents[gPlayerAvatar.objectEventId].facingDirection() as u32,
        ));
    }
}
pub unsafe fn PlayerIdleWheelie(direction: u8) {
    PlayerSetAnimId(
        GetAcroWheelieFaceDirectionMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerStartWheelie(direction: u8) {
    PlayerSetAnimId(
        GetAcroPopWheelieFaceDirectionMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerEndWheelie(direction: u8) {
    PlayerSetAnimId(
        GetAcroEndWheelieFaceDirectionMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerStandingHoppingWheelie(direction: u8) {
    PlaySE(SE_BIKE_HOP);
    PlayerSetAnimId(
        GetAcroWheelieHopFaceDirectionMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerMovingHoppingWheelie(direction: u8) {
    PlaySE(SE_BIKE_HOP);
    PlayerSetAnimId(
        GetAcroWheelieHopDirectionMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerLedgeHoppingWheelie(direction: u8) {
    PlaySE(SE_BIKE_HOP);
    PlayerSetAnimId(
        GetAcroWheelieJumpDirectionMovementAction(direction as u32),
        COPY_MOVE_JUMP2,
    );
}
pub unsafe fn PlayerAcroTurnJump(direction: u8) {
    PlaySE(SE_BIKE_HOP);
    PlayerSetAnimId(
        GetJumpInPlaceTurnAroundMovementAction(direction as u32),
        COPY_MOVE_FACE,
    );
}
pub unsafe fn PlayerWheelieInPlace(direction: u8) {
    PlaySE(SE_WALL_HIT);
    PlayerSetAnimId(
        GetAcroWheelieInPlaceDirectionMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerPopWheelieWhileMoving(direction: u8) {
    PlayerSetAnimId(
        GetAcroPopWheelieMoveDirectionMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerWheelieMove(direction: u8) {
    PlayerSetAnimId(
        GetAcroWheelieMoveDirectionMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
pub unsafe fn PlayerEndWheelieWhileMoving(direction: u8) {
    PlayerSetAnimId(
        GetAcroEndWheelieMoveDirectionMovementAction(direction as u32),
        COPY_MOVE_WALK,
    );
}
unsafe fn PlayCollisionSoundIfNotFacingWarp(direction: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let metatileBehavior: u8 = gObjectEvents[gPlayerAvatar.objectEventId].currentMetatileBehavior;
    if sArrowWarpMetatileBehaviorChecks[direction as i32 - 1].unwrap_unchecked()(metatileBehavior)
        == 0
    {
        if direction == DIR_NORTH {
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            MoveCoords(direction, &raw mut x, &raw mut y);
            if MetatileBehavior_IsWarpDoor(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8)
                != 0
            {
                return;
            }
        }
        PlaySE(SE_WALL_HIT);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetXYCoordsOneStepInFrontOfPlayer(x: *mut i16, y: *mut i16) {
    *x = gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x;
    *y = gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y;
    MoveCoords(GetPlayerFacingDirection(), x, y);
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerGetDestCoords(x: *mut i16, y: *mut i16) {
    *x = gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x;
    *y = gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y;
}
pub unsafe fn player_get_pos_including_state_based_drift(x: *mut i16, y: *mut i16) -> u8 {
    let object: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if (*object).heldMovementActive() != 0
        && (*object).heldMovementFinished() == 0
        && gSprites[(*object).spriteId].data[2] == 0
    {
        *x = (*object).currentCoords.x;
        *y = (*object).currentCoords.y;
        match (*object).movementActionId {
            MOVEMENT_ACTION_WALK_NORMAL_DOWN | MOVEMENT_ACTION_PLAYER_RUN_DOWN => {
                *y += 1;
                return TRUE;
            }
            MOVEMENT_ACTION_WALK_NORMAL_UP | MOVEMENT_ACTION_PLAYER_RUN_UP => {
                *y -= 1;
                return TRUE;
            }
            MOVEMENT_ACTION_WALK_NORMAL_LEFT | MOVEMENT_ACTION_PLAYER_RUN_LEFT => {
                *x -= 1;
                return TRUE;
            }
            MOVEMENT_ACTION_WALK_NORMAL_RIGHT | MOVEMENT_ACTION_PLAYER_RUN_RIGHT => {
                *x += 1;
                return TRUE;
            }
            _ => {}
        }
    }
    *x = -1;
    *y = -1;
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerFacingDirection() -> u8 {
    gObjectEvents[gPlayerAvatar.objectEventId].facingDirection() as u8
}
pub unsafe fn GetPlayerMovementDirection() -> u8 {
    gObjectEvents[gPlayerAvatar.objectEventId].movementDirection() as u8
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerGetElevation() -> u8 {
    gObjectEvents[gPlayerAvatar.objectEventId].previousElevation()
}
pub unsafe fn MovePlayerToMapCoords(x: i16, y: i16) {
    MoveObjectEventToMapCoords(&raw mut gObjectEvents[gPlayerAvatar.objectEventId], x, y);
}
pub unsafe fn TestPlayerAvatarFlags(flag: u8) -> u8 {
    gPlayerAvatar.flags & flag
}
pub unsafe fn GetPlayerAvatarFlags() -> u8 {
    gPlayerAvatar.flags
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerAvatarSpriteId() -> u8 {
    gPlayerAvatar.spriteId
}
pub unsafe fn CancelPlayerForcedMovement() {
    ForcedMovement_None();
}
#[unsafe(no_mangle)]
pub unsafe fn StopPlayerAvatar() {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    npc_clear_strange_bits(playerObjEvent);
    SetObjectEventDirection(playerObjEvent, (*playerObjEvent).facingDirection() as u8);
    if TestPlayerAvatarFlags(6) != 0 {
        Bike_HandleBumpySlopeJump();
        Bike_UpdateBikeCounterSpeed(0);
    }
}
pub fn GetRivalAvatarGraphicsIdByStateIdAndGender(state: u8, gender: u8) -> u8 {
    sRivalAvatarGfxIds[state][gender]
}
pub unsafe fn GetPlayerAvatarGraphicsIdByStateIdAndGender(state: u8, gender: u8) -> u8 {
    sPlayerAvatarGfxIds[state][gender]
}
pub unsafe fn GetFRLGAvatarGraphicsIdByGender(gender: u8) -> u8 {
    sFRLGAvatarGfxIds[gender]
}
pub unsafe fn GetRSAvatarGraphicsIdByGender(gender: u8) -> u8 {
    sRSAvatarGfxIds[gender]
}
pub unsafe fn GetPlayerAvatarGraphicsIdByStateId(state: u8) -> u8 {
    GetPlayerAvatarGraphicsIdByStateIdAndGender(state, gPlayerAvatar.gender)
}
pub fn unref_GetRivalAvatarGenderByGraphicsId(gfxId: u8) -> u8 {
    match gfxId {
        OBJ_EVENT_GFX_RIVAL_MAY_NORMAL
        | OBJ_EVENT_GFX_RIVAL_MAY_MACH_BIKE
        | OBJ_EVENT_GFX_RIVAL_MAY_ACRO_BIKE
        | OBJ_EVENT_GFX_RIVAL_MAY_SURFING
        | OBJ_EVENT_GFX_RIVAL_MAY_FIELD_MOVE
        | OBJ_EVENT_GFX_MAY_UNDERWATER
        | OBJ_EVENT_GFX_MAY_FISHING
        | OBJ_EVENT_GFX_MAY_WATERING => {
            return FEMALE;
        }
        _ => {
            return MALE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn GetPlayerAvatarGenderByGraphicsId(gfxId: u8) -> u8 {
    match gfxId {
        OBJ_EVENT_GFX_MAY_NORMAL
        | OBJ_EVENT_GFX_MAY_MACH_BIKE
        | OBJ_EVENT_GFX_MAY_ACRO_BIKE
        | OBJ_EVENT_GFX_MAY_SURFING
        | OBJ_EVENT_GFX_MAY_FIELD_MOVE
        | OBJ_EVENT_GFX_MAY_UNDERWATER
        | OBJ_EVENT_GFX_MAY_FISHING
        | OBJ_EVENT_GFX_MAY_WATERING => {
            return FEMALE;
        }
        _ => {
            return MALE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn PartyHasMonWithSurf() -> u8 {
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) == 0 {
        for i in 0..(PARTY_SIZE as u8) {
            if GetMonData2(&raw mut gPlayerParty[i], MON_DATA_SPECIES) == SPECIES_NONE as u32 {
                break;
            }
            if MonKnowsMove(&raw mut gPlayerParty[i], MOVE_SURF) != 0 {
                return TRUE;
            }
        }
    }
    FALSE
}
pub unsafe fn IsPlayerSurfingNorth() -> u8 {
    if GetPlayerMovementDirection() == DIR_NORTH
        && TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn IsPlayerFacingSurfableFishableWater() -> u8 {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let mut x: i16 = (*playerObjEvent).currentCoords.x;
    let mut y: i16 = (*playerObjEvent).currentCoords.y;
    MoveCoords(
        (*playerObjEvent).facingDirection() as u8,
        &raw mut x,
        &raw mut y,
    );
    if GetCollisionAtCoords(
        playerObjEvent,
        x,
        y,
        (*playerObjEvent).facingDirection() as u32,
    ) == COLLISION_ELEVATION_MISMATCH
        && PlayerGetElevation() == ELEVATION_DEFAULT
        && MetatileBehavior_IsSurfableFishableWater(
            MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8,
        ) != 0
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn ClearPlayerAvatarInfo() {
    memset(&raw mut gPlayerAvatar as *mut u8, 0, 36);
}
pub unsafe fn SetPlayerAvatarStateMask(flags: u8) {
    gPlayerAvatar.flags &= 224;
    gPlayerAvatar.flags |= flags;
}
unsafe fn GetPlayerAvatarStateTransitionByGraphicsId(graphicsId: u8, gender: u8) -> u8 {
    for i in 0..5u8 {
        if sPlayerAvatarGfxToStateFlag[gender][i].graphicsId == graphicsId {
            return sPlayerAvatarGfxToStateFlag[gender][i].playerFlag;
        }
    }
    PLAYER_AVATAR_FLAG_ON_FOOT
}
#[unsafe(no_mangle)]
pub unsafe fn GetPlayerAvatarGraphicsIdByCurrentState() -> u8 {
    let flags: u8 = gPlayerAvatar.flags;
    for i in 0..5u8 {
        if sPlayerAvatarGfxToStateFlag[gPlayerAvatar.gender][i].playerFlag as i32 & flags as i32
            != 0
        {
            return sPlayerAvatarGfxToStateFlag[gPlayerAvatar.gender][i].graphicsId;
        }
    }
    0
}
pub unsafe fn SetPlayerAvatarExtraStateTransition(graphicsId: u8, transitionFlag: u8) {
    let stateFlag: u8 =
        GetPlayerAvatarStateTransitionByGraphicsId(graphicsId, gPlayerAvatar.gender);
    gPlayerAvatar.transitionFlags |= stateFlag | transitionFlag;
    DoPlayerAvatarTransition();
}
pub unsafe fn InitPlayerAvatar(x: i16, y: i16, direction: u8, gender: u8) {
    let mut playerObjEventTemplate: ObjectEventTemplate = zeroed();
    playerObjEventTemplate.localId = LOCALID_PLAYER;
    playerObjEventTemplate.graphicsId =
        GetPlayerAvatarGraphicsIdByStateIdAndGender(PLAYER_AVATAR_STATE_NORMAL, gender);
    playerObjEventTemplate.x = x - MAP_OFFSET as i16;
    playerObjEventTemplate.y = y - MAP_OFFSET as i16;
    playerObjEventTemplate.elevation = ELEVATION_TRANSITION;
    playerObjEventTemplate.movementType = MOVEMENT_TYPE_PLAYER;
    playerObjEventTemplate.set_movementRangeX(0);
    playerObjEventTemplate.set_movementRangeY(0);
    playerObjEventTemplate.trainerType = TRAINER_TYPE_NONE;
    playerObjEventTemplate.trainerRange_berryTreeId = 0;
    playerObjEventTemplate.script = null_mut();
    playerObjEventTemplate.flagId = 0;
    let objectEventId: u8 = SpawnSpecialObjectEvent(&raw mut playerObjEventTemplate);
    let objectEvent: *mut ObjectEvent = &raw mut gObjectEvents[objectEventId];
    (*objectEvent).set_isPlayer(TRUE as u32);
    (*objectEvent).warpArrowSpriteId = CreateWarpArrowSprite();
    ObjectEventTurn(objectEvent, direction);
    ClearPlayerAvatarInfo();
    gPlayerAvatar.runningState = NOT_MOVING;
    gPlayerAvatar.tileTransitionState = T_NOT_MOVING;
    gPlayerAvatar.objectEventId = objectEventId;
    gPlayerAvatar.spriteId = (*objectEvent).spriteId;
    gPlayerAvatar.gender = gender;
    SetPlayerAvatarStateMask(33);
}
pub unsafe fn SetPlayerInvisibility(invisible: u8) {
    gObjectEvents[gPlayerAvatar.objectEventId].set_invisible(invisible as u32);
    if TestPlayerAvatarFlags(PLAYER_AVATAR_FLAG_SURFING) != 0 {
        gSprites[gObjectEvents[gPlayerAvatar.objectEventId].fieldEffectSpriteId]
            .set_invisible(invisible as u16);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn SetPlayerAvatarFieldMove() {
    ObjectEventSetGraphicsId(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_FIELD_MOVE),
    );
    StartSpriteAnim(&raw mut gSprites[gPlayerAvatar.spriteId], ANIM_FIELD_MOVE);
}
unsafe fn SetPlayerAvatarFishing(direction: u8) {
    ObjectEventSetGraphicsId(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_FISHING),
    );
    StartSpriteAnim(
        &raw mut gSprites[gPlayerAvatar.spriteId],
        GetFishingDirectionAnimNum(direction),
    );
}
pub unsafe fn PlayerUseAcroBikeOnBumpySlope(direction: u8) {
    ObjectEventSetGraphicsId(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_ACRO_BIKE),
    );
    StartSpriteAnim(
        &raw mut gSprites[gPlayerAvatar.spriteId],
        GetAcroWheelieDirectionAnimNum(direction),
    );
    SeekSpriteAnim(&raw mut gSprites[gPlayerAvatar.spriteId], 1);
}
pub unsafe fn SetPlayerAvatarWatering(direction: u8) {
    ObjectEventSetGraphicsId(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_WATERING),
    );
    StartSpriteAnim(
        &raw mut gSprites[gPlayerAvatar.spriteId],
        GetFaceDirectionAnimNum(direction),
    );
}
unsafe fn HideShowWarpArrow(objectEvent: *mut ObjectEvent) {
    let mut y: i16 = 0;
    let metatileBehavior: u8 = (*objectEvent).currentMetatileBehavior;
    let mut x: i16 = 0;
    let mut direction: u8 = DIR_SOUTH;
    while x < 4 {
        if sArrowWarpMetatileBehaviorChecks2[x].unwrap_unchecked()(metatileBehavior) != 0
            && direction as u16 == (*objectEvent).movementDirection()
        {
            x = (*objectEvent).currentCoords.x;
            y = (*objectEvent).currentCoords.y;
            MoveCoords(direction, &raw mut x, &raw mut y);
            ShowWarpArrowSprite((*objectEvent).warpArrowSpriteId, direction, x, y);
            return;
        }
        x += 1;
        direction += 1;
    }
    SetSpriteInvisible((*objectEvent).warpArrowSpriteId);
}
unsafe fn StartStrengthAnim(objectEventId: u8, direction: u8) {
    let taskId: u8 = CreateTask(Some(Task_PushBoulder), 0xFF);
    task_set(taskId, tBoulderObjId, objectEventId as i16);
    task_set(taskId, tDirection, direction as i16);
    Task_PushBoulder(taskId);
}
pub(crate) unsafe fn Task_PushBoulder(taskId: u8) {
    while sPushBoulderFuncs[task_get(taskId, tState)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        &raw mut gObjectEvents[task_get(taskId, tBoulderObjId)],
    ) != 0
    {}
}
pub(crate) unsafe fn PushBoulder_Start(
    task: *mut Task,
    player: *mut ObjectEvent,
    boulder: *mut ObjectEvent,
) -> u8 {
    LockPlayerFieldControls();
    gPlayerAvatar.preventStep = TRUE;
    (*task).data[tState] += 1;
    FALSE
}
pub(crate) unsafe fn PushBoulder_Move(
    task: *mut Task,
    player: *mut ObjectEvent,
    boulder: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsHeldMovementActive(player) != 0 {
        ObjectEventClearHeldMovementIfFinished(player);
    }
    if ObjectEventIsHeldMovementActive(boulder) != 0 {
        ObjectEventClearHeldMovementIfFinished(boulder);
    }
    if ObjectEventIsMovementOverridden(player) == 0 && ObjectEventIsMovementOverridden(boulder) == 0
    {
        ObjectEventClearHeldMovementIfFinished(player);
        ObjectEventClearHeldMovementIfFinished(boulder);
        ObjectEventSetHeldMovement(
            player,
            GetWalkInPlaceNormalMovementAction((*task).data[tDirection] as u8 as u32),
        );
        ObjectEventSetHeldMovement(
            boulder,
            GetWalkSlowMovementAction((*task).data[tDirection] as u8 as u32),
        );
        gFieldEffectArguments[0] = (*boulder).currentCoords.x as i32;
        gFieldEffectArguments[1] = (*boulder).currentCoords.y as i32;
        gFieldEffectArguments[2] = (*boulder).previousElevation() as i32;
        gFieldEffectArguments[3] = gSprites[(*boulder).spriteId].oam.priority() as i32;
        FieldEffectStart(FLDEFF_DUST);
        PlaySE(SE_M_STRENGTH);
        (*task).data[tState] += 1;
    }
    FALSE
}
pub(crate) unsafe fn PushBoulder_End(
    task: *mut Task,
    player: *mut ObjectEvent,
    boulder: *mut ObjectEvent,
) -> u8 {
    if ObjectEventCheckHeldMovementStatus(player) != 0
        && ObjectEventCheckHeldMovementStatus(boulder) != 0
    {
        ObjectEventClearHeldMovementIfFinished(player);
        ObjectEventClearHeldMovementIfFinished(boulder);
        gPlayerAvatar.preventStep = FALSE;
        UnlockPlayerFieldControls();
        DestroyTask(FindTaskIdByFunc(Some(Task_PushBoulder)));
    }
    FALSE
}
unsafe fn DoPlayerMatJump() {
    DoPlayerAvatarSecretBaseMatJump(CreateTask(Some(DoPlayerAvatarSecretBaseMatJump), 0xFF));
}
pub(crate) unsafe fn DoPlayerAvatarSecretBaseMatJump(taskId: u8) {
    while sPlayerAvatarSecretBaseMatJump[task_get(taskId, 0)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
    ) != 0
    {}
}
pub(crate) unsafe fn PlayerAvatar_DoSecretBaseMatJump(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    gPlayerAvatar.preventStep = TRUE;
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        PlaySE(SE_LEDGE);
        ObjectEventSetHeldMovement(
            objectEvent,
            GetJumpInPlaceMovementAction((*objectEvent).facingDirection() as u32),
        );
        (*task).data[1] += 1;
        if (*task).data[1] > 1 {
            gPlayerAvatar.preventStep = FALSE;
            gPlayerAvatar.transitionFlags |= PLAYER_AVATAR_FLAG_CONTROLLABLE;
            DestroyTask(FindTaskIdByFunc(Some(DoPlayerAvatarSecretBaseMatJump)));
        }
    }
    FALSE
}
unsafe fn DoPlayerMatSpin() {
    let taskId: u8 = CreateTask(Some(PlayerAvatar_DoSecretBaseMatSpin), 0xFF);
    PlayerAvatar_DoSecretBaseMatSpin(taskId);
}
pub(crate) unsafe fn PlayerAvatar_DoSecretBaseMatSpin(taskId: u8) {
    while sPlayerAvatarSecretBaseMatSpin[task_get(taskId, 0)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
    ) != 0
    {}
}
pub(crate) unsafe fn PlayerAvatar_SecretBaseMatSpinStep0(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    (*task).data[0] += 1;
    (*task).data[1] = (*objectEvent).movementDirection() as i16;
    gPlayerAvatar.preventStep = TRUE;
    LockPlayerFieldControls();
    PlaySE(SE_WARP_IN);
    TRUE
}
pub(crate) unsafe fn PlayerAvatar_SecretBaseMatSpinStep1(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    let directions: CArray<u8, 4> = CArray([3, 4, 2, 1]);
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        let mut direction: u8 = 0;
        ObjectEventSetHeldMovement(
            objectEvent,
            GetFaceDirectionMovementAction(
                ({
                    direction = directions[(*objectEvent).movementDirection() as i32 - 1];
                    direction
                }) as u32,
            ),
        );
        if direction == (*task).data[1] as u8 {
            (*task).data[2] += 1;
        }
        (*task).data[0] += 1;
        if (*task).data[2] > 3 && direction == GetOppositeDirection((*task).data[1] as u8) {
            (*task).data[0] += 1;
        }
    }
    FALSE
}
pub(crate) unsafe fn PlayerAvatar_SecretBaseMatSpinStep2(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    let actions: CArray<u8, 5> = CArray([16, 16, 17, 18, 19]);
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        ObjectEventSetHeldMovement(objectEvent, actions[(*task).data[2]]);
        (*task).data[0] = 1;
    }
    FALSE
}
pub(crate) unsafe fn PlayerAvatar_SecretBaseMatSpinStep3(
    task: *mut Task,
    objectEvent: *mut ObjectEvent,
) -> u8 {
    if ObjectEventClearHeldMovementIfFinished(objectEvent) != 0 {
        ObjectEventSetHeldMovement(
            objectEvent,
            GetWalkSlowMovementAction(GetOppositeDirection((*task).data[1] as u8) as u32),
        );
        UnlockPlayerFieldControls();
        gPlayerAvatar.preventStep = FALSE;
        DestroyTask(FindTaskIdByFunc(Some(PlayerAvatar_DoSecretBaseMatSpin)));
    }
    FALSE
}
unsafe fn CreateStopSurfingTask(direction: u8) {
    LockPlayerFieldControls();
    Overworld_ClearSavedMusic();
    Overworld_ChangeMusicToDefault();
    gPlayerAvatar.flags &= 247;
    gPlayerAvatar.flags |= PLAYER_AVATAR_FLAG_ON_FOOT;
    gPlayerAvatar.preventStep = TRUE;
    let taskId: u8 = CreateTask(Some(Task_StopSurfingInit), 0xFF);
    task_set(taskId, 0, direction as i16);
    Task_StopSurfingInit(taskId);
}
pub(crate) unsafe fn Task_StopSurfingInit(taskId: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(playerObjEvent) != 0
        && ObjectEventClearHeldMovementIfFinished(playerObjEvent) == 0
    {
        return;
    }
    SetSurfBlob_BobState((*playerObjEvent).fieldEffectSpriteId, BOB_JUST_MON);
    ObjectEventSetHeldMovement(
        playerObjEvent,
        GetJumpSpecialMovementAction(task_get(taskId, 0) as u8 as u32),
    );
    task_set_func(taskId, Some(Task_WaitStopSurfing));
}
pub(crate) unsafe fn Task_WaitStopSurfing(taskId: u8) {
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventClearHeldMovementIfFinished(playerObjEvent) != 0 {
        ObjectEventSetGraphicsId(
            playerObjEvent,
            GetPlayerAvatarGraphicsIdByStateId(PLAYER_AVATAR_STATE_NORMAL),
        );
        ObjectEventSetHeldMovement(
            playerObjEvent,
            GetFaceDirectionMovementAction((*playerObjEvent).facingDirection() as u32),
        );
        gPlayerAvatar.preventStep = FALSE;
        UnlockPlayerFieldControls();
        DestroySprite(&raw mut gSprites[(*playerObjEvent).fieldEffectSpriteId]);
        DestroyTask(taskId);
    }
}
pub unsafe fn StartFishing(rod: u8) {
    let taskId: u8 = CreateTask(Some(Task_Fishing), 0xFF);
    task_set(taskId, tFishingRod, rod as i16);
    Task_Fishing(taskId);
}
pub(crate) unsafe fn Task_Fishing(taskId: u8) {
    while sFishingStateFuncs[task_get(taskId, tStep)].unwrap_unchecked()(
        &raw mut (*gTasks.as_ptr())[taskId],
    ) != 0
    {}
}
pub(crate) unsafe fn Fishing_Init(task: *mut Task) -> u8 {
    LockPlayerFieldControls();
    gPlayerAvatar.preventStep = TRUE;
    (*task).data[tStep] += 1;
    FALSE
}
pub(crate) unsafe fn Fishing_GetRodOut(task: *mut Task) -> u8 {
    let mut minRounds1: CArray<i16, 3> = zeroed();
    minRounds1[0] = 1;
    minRounds1[1] = 1;
    minRounds1[2] = 1;
    let mut minRounds2: CArray<i16, 3> = zeroed();
    minRounds2[0] = 1;
    minRounds2[1] = 3;
    minRounds2[2] = 6;
    (*task).data[tRoundsPlayed] = 0;
    (*task).data[tMinRoundsRequired] = minRounds1[(*task).data[tFishingRod]]
        + rem_i32(
            Random() as i32,
            minRounds2[(*task).data[tFishingRod]] as i32,
        ) as i16;
    (*task).data[tPlayerGfxId] = gObjectEvents[gPlayerAvatar.objectEventId].graphicsId as i16;
    let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    ObjectEventClearHeldMovementIfActive(playerObjEvent);
    (*playerObjEvent).set_enableAnim(TRUE as u32);
    SetPlayerAvatarFishing((*playerObjEvent).facingDirection() as u8);
    (*task).data[tStep] += 1;
    FALSE
}
pub(crate) unsafe fn Fishing_WaitBeforeDots(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    (*task).data[tFrameCounter] += 1;
    if (*task).data[tFrameCounter] >= 60 {
        (*task).data[tStep] += 1;
    }
    FALSE
}
pub(crate) unsafe fn Fishing_InitDots(task: *mut Task) -> u8 {
    LoadMessageBoxAndFrameGfx(0, TRUE);
    (*task).data[tStep] += 1;
    (*task).data[tFrameCounter] = 0;
    (*task).data[tNumDots] = 0;
    let mut randVal: u32 = Random() as u32;
    randVal %= 10;
    (*task).data[tDotsRequired] = randVal as i16 + 1;
    if (*task).data[tRoundsPlayed] == 0 {
        (*task).data[tDotsRequired] = randVal as i16 + 4;
    }
    if (*task).data[tDotsRequired] >= 10 {
        (*task).data[tDotsRequired] = 10;
    }
    TRUE
}
pub(crate) unsafe fn Fishing_ShowDots(task: *mut Task) -> u8 {
    let mut dot: CArray<u8, 2> = CArray([175, 255]);
    AlignFishingAnimationFrames();
    (*task).data[tFrameCounter] += 1;
    if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*task).data[tStep] = FISHING_NO_BITE;
        if (*task).data[tRoundsPlayed] != 0 {
            (*task).data[tStep] = FISHING_GOT_AWAY;
        }
        return TRUE;
    } else {
        if (*task).data[tFrameCounter] >= 20 {
            (*task).data[tFrameCounter] = 0;
            if (*task).data[tNumDots] >= (*task).data[tDotsRequired] {
                (*task).data[tStep] += 1;
                if (*task).data[tRoundsPlayed] != 0 {
                    (*task).data[tStep] += 1;
                }
                (*task).data[tRoundsPlayed] += 1;
            } else {
                AddTextPrinterParameterized(
                    0,
                    FONT_NORMAL,
                    dot.as_mut_ptr(),
                    (*task).data[tNumDots] as u8 * 8,
                    1,
                    0,
                    None,
                );
                (*task).data[tNumDots] += 1;
            }
        }
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn Fishing_CheckForBite(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    (*task).data[tStep] += 1;
    let mut bite: u8 = FALSE;
    if DoesCurrentMapHaveFishingMons() == 0 {
        (*task).data[tStep] = FISHING_NO_BITE;
    } else {
        if GetMonData2(&raw mut gPlayerParty[0], MON_DATA_SANITY_IS_EGG) == 0 {
            let ability: u8 = GetMonAbility(&raw mut gPlayerParty[0]);
            if (ability == ABILITY_SUCTION_CUPS || ability == ABILITY_STICKY_HOLD)
                && Random() as i32 % 100 > 14
            {
                bite = TRUE;
            }
        }
        if bite == 0 {
            if Random() as i32 & 1 != 0 {
                (*task).data[tStep] = FISHING_NO_BITE;
            } else {
                bite = TRUE;
            }
        }
        if bite == TRUE {
            StartSpriteAnim(
                &raw mut gSprites[gPlayerAvatar.spriteId],
                GetFishingBiteDirectionAnimNum(GetPlayerFacingDirection()),
            );
        }
    }
    TRUE
}
pub(crate) unsafe fn Fishing_GotBite(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    AddTextPrinterParameterized(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_OhABite).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        0,
        17,
        0,
        None,
    );
    (*task).data[tStep] += 1;
    (*task).data[tFrameCounter] = 0;
    FALSE
}
pub(crate) unsafe fn Fishing_WaitForA(task: *mut Task) -> u8 {
    let mut reelTimeouts: CArray<i16, 3> = zeroed();
    reelTimeouts[0] = 36;
    reelTimeouts[1] = 33;
    reelTimeouts[2] = 30;
    AlignFishingAnimationFrames();
    (*task).data[tFrameCounter] += 1;
    if (*task).data[tFrameCounter] >= reelTimeouts[(*task).data[tFishingRod]] {
        (*task).data[tStep] = FISHING_GOT_AWAY;
    } else if gMain.newKeys as i32 & A_BUTTON != 0 {
        (*task).data[tStep] += 1;
    }
    FALSE
}
pub(crate) unsafe fn Fishing_CheckMoreDots(task: *mut Task) -> u8 {
    let mut moreDotsChance: CArray<CArray<i16, 2>, 3> = zeroed();
    moreDotsChance[0][0] = 0;
    moreDotsChance[0][1] = 0;
    moreDotsChance[1][0] = 40;
    moreDotsChance[1][1] = 10;
    moreDotsChance[2][0] = 70;
    moreDotsChance[2][1] = 30;
    AlignFishingAnimationFrames();
    (*task).data[tStep] += 1;
    if (*task).data[tRoundsPlayed] < (*task).data[tMinRoundsRequired] {
        (*task).data[tStep] = FISHING_START_ROUND;
    } else if (*task).data[tRoundsPlayed] < 2 {
        let probability: i16 = (Random() as i32 % 100) as i16;
        if moreDotsChance[(*task).data[tFishingRod]][(*task).data[tRoundsPlayed]] > probability {
            (*task).data[tStep] = FISHING_START_ROUND;
        }
    }
    FALSE
}
pub(crate) unsafe fn Fishing_MonOnHook(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_PokemonOnHook).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        1,
        None,
        TEXT_COLOR_DARK_GRAY,
        0x1,
        TEXT_COLOR_LIGHT_GRAY,
    );
    (*task).data[tStep] += 1;
    (*task).data[tFrameCounter] = 0;
    FALSE
}
pub(crate) unsafe fn Fishing_StartEncounter(task: *mut Task) -> u8 {
    if (*task).data[tFrameCounter] == 0 {
        AlignFishingAnimationFrames();
    }
    RunTextPrinters();
    if (*task).data[tFrameCounter] == 0 && IsTextPrinterActive(0) == 0 {
        let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        ObjectEventSetGraphicsId(playerObjEvent, (*task).data[tPlayerGfxId] as u8);
        ObjectEventTurn(playerObjEvent, (*playerObjEvent).movementDirection() as u8);
        if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
            SetSurfBlob_PlayerOffset(
                gObjectEvents[gPlayerAvatar.objectEventId].fieldEffectSpriteId,
                0,
                0,
            );
        }
        gSprites[gPlayerAvatar.spriteId].x2 = 0;
        gSprites[gPlayerAvatar.spriteId].y2 = 0;
        ClearDialogWindowAndFrame(0, TRUE);
        (*task).data[tFrameCounter] += 1;
        return FALSE;
    }
    if (*task).data[tFrameCounter] != 0 {
        gPlayerAvatar.preventStep = FALSE;
        UnlockPlayerFieldControls();
        FishingWildEncounter((*task).data[tFishingRod] as u8);
        RecordFishingAttemptForTV(TRUE);
        DestroyTask(FindTaskIdByFunc(Some(Task_Fishing)));
    }
    FALSE
}
pub(crate) unsafe fn Fishing_NotEvenNibble(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    StartSpriteAnim(
        &raw mut gSprites[gPlayerAvatar.spriteId],
        GetFishingNoCatchDirectionAnimNum(GetPlayerFacingDirection()),
    );
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_NotEvenANibble).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        1,
        None,
        TEXT_COLOR_DARK_GRAY,
        0x1,
        TEXT_COLOR_LIGHT_GRAY,
    );
    (*task).data[tStep] = FISHING_SHOW_RESULT;
    TRUE
}
pub(crate) unsafe fn Fishing_GotAway(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    StartSpriteAnim(
        &raw mut gSprites[gPlayerAvatar.spriteId],
        GetFishingNoCatchDirectionAnimNum(GetPlayerFacingDirection()),
    );
    FillWindowPixelBuffer(0, 17);
    AddTextPrinterParameterized2(
        0,
        FONT_NORMAL,
        (*(&raw const crate::data::strings::gText_ItGotAway).cast::<CArray<u8, 0>>())
            .as_ptr()
            .cast_mut(),
        1,
        None,
        TEXT_COLOR_DARK_GRAY,
        0x1,
        TEXT_COLOR_LIGHT_GRAY,
    );
    (*task).data[tStep] += 1;
    TRUE
}
pub(crate) unsafe fn Fishing_NoMon(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    (*task).data[tStep] += 1;
    FALSE
}
pub(crate) unsafe fn Fishing_PutRodAway(task: *mut Task) -> u8 {
    AlignFishingAnimationFrames();
    if gSprites[gPlayerAvatar.spriteId].animEnded() != 0 {
        let playerObjEvent: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        ObjectEventSetGraphicsId(playerObjEvent, (*task).data[tPlayerGfxId] as u8);
        ObjectEventTurn(playerObjEvent, (*playerObjEvent).movementDirection() as u8);
        if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
            SetSurfBlob_PlayerOffset(
                gObjectEvents[gPlayerAvatar.objectEventId].fieldEffectSpriteId,
                0,
                0,
            );
        }
        gSprites[gPlayerAvatar.spriteId].x2 = 0;
        gSprites[gPlayerAvatar.spriteId].y2 = 0;
        (*task).data[tStep] += 1;
    }
    FALSE
}
pub(crate) unsafe fn Fishing_EndNoMon(task: *mut Task) -> u8 {
    RunTextPrinters();
    if IsTextPrinterActive(0) == 0 {
        gPlayerAvatar.preventStep = FALSE;
        UnlockPlayerFieldControls();
        UnfreezeObjectEvents();
        ClearDialogWindowAndFrame(0, TRUE);
        RecordFishingAttemptForTV(FALSE);
        DestroyTask(FindTaskIdByFunc(Some(Task_Fishing)));
    }
    FALSE
}
unsafe fn AlignFishingAnimationFrames() {
    let playerSprite: *mut Sprite = &raw mut gSprites[gPlayerAvatar.spriteId];
    let mut animCmdIndex: u8 = 0;
    AnimateSprite(playerSprite);
    (*playerSprite).x2 = 0;
    (*playerSprite).y2 = 0;
    animCmdIndex = (*playerSprite).animCmdIndex;
    if (*(*(*playerSprite).anims.at((*playerSprite).animNum)).at(animCmdIndex)).r#type == -1 {
        animCmdIndex -= 1;
    } else {
        (*playerSprite).set_animDelayCounter((*playerSprite).animDelayCounter() + 1);
        if (*(*(*playerSprite).anims.at((*playerSprite).animNum)).at(animCmdIndex)).r#type == -1 {
            animCmdIndex -= 1;
        }
    }
    let animType: u8 =
        (*(*(*playerSprite).anims.at((*playerSprite).animNum)).at(animCmdIndex)).r#type as u8;
    if animType == 1 || animType == 2 || animType == 3 {
        (*playerSprite).x2 = 8;
        if GetPlayerFacingDirection() == 3 {
            (*playerSprite).x2 = -8;
        }
    }
    if animType == 5 {
        (*playerSprite).y2 = -8;
    }
    if animType == 10 || animType == 11 {
        (*playerSprite).y2 = 8;
    }
    if gPlayerAvatar.flags as i32 & PLAYER_AVATAR_FLAG_SURFING as i32 != 0 {
        SetSurfBlob_PlayerOffset(
            gObjectEvents[gPlayerAvatar.objectEventId].fieldEffectSpriteId,
            TRUE,
            (*playerSprite).y2,
        );
    }
}
pub unsafe fn SetSpinStartFacingDir(direction: u8) {
    sSpinStartFacingDir.set(direction);
}
unsafe fn GetSpinStartFacingDir() -> u8 {
    if sSpinStartFacingDir.get() == DIR_NONE {
        return DIR_SOUTH;
    }
    sSpinStartFacingDir.get()
}
pub(crate) unsafe fn Task_DoPlayerSpinExit(taskId: u8) {
    let object: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let sprite: *mut Sprite = &raw mut gSprites[(*object).spriteId];
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if ObjectEventClearHeldMovementIfFinished(object) == 0 {
                return;
            }
            SetSpinStartFacingDir((*object).facingDirection() as u8);
            *data.at(1) = 0;
            *data.at(2) = 1;
            *data.at(3) = ((*sprite).y as u16 as i16 + (*sprite).y2 as u16 as i16) << 4;
            (*sprite).y2 = 0;
            CameraObjectFreeze();
            (*object).set_fixedPriority(TRUE as u32);
            (*sprite).oam.set_priority(0);
            (*sprite).subpriority = 0;
            (*sprite).set_subspriteMode(SUBSPRITES_OFF);
            *data += 1;
        }
        if fall || sw1 == 1 {
            TrySpinPlayerForWarp(object, data.at(1));
            *data.at(3) -= *data.at(2);
            *data.at(2) += 3;
            (*sprite).y = *data.at(3) >> 4;
            if ((*sprite).y as i32 + gTotalCameraPixelOffsetY as i16 as i32) < -32 {
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            DestroyTask(taskId);
            break 'l1;
        }
    }
}
pub unsafe fn DoPlayerSpinEntrance() {
    Task_DoPlayerSpinEntrance(CreateTask(Some(Task_DoPlayerSpinEntrance), 0));
}
pub unsafe fn IsPlayerSpinEntranceActive() -> u32 {
    FuncIsActiveTask(Some(Task_DoPlayerSpinEntrance)) as u32
}
pub unsafe fn DoPlayerSpinExit() {
    Task_DoPlayerSpinExit(CreateTask(Some(Task_DoPlayerSpinExit), 0));
}
pub unsafe fn IsPlayerSpinExitActive() -> u32 {
    FuncIsActiveTask(Some(Task_DoPlayerSpinExit)) as u32
}
pub(crate) unsafe fn Task_DoPlayerSpinEntrance(taskId: u8) {
    let object: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    let sprite: *mut Sprite = &raw mut gSprites[(*object).spriteId];
    let data: *mut i16 = (*gTasks.as_ptr())[taskId].data.as_mut_ptr();
    'l1: {
        let sw1: i16 = *data;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            *data.at(5) = GetSpinStartFacingDir() as i16;
            ObjectEventForceSetHeldMovement(
                object,
                GetFaceDirectionMovementAction(sSpinDirections[*data.at(5)] as u32),
            );
            *data.at(1) = 0;
            *data.at(2) = 116;
            *data.at(4) = (*sprite).y;
            *data.at(6) = (*sprite).oam.priority() as i16;
            *data.at(7) = (*sprite).subpriority as i16;
            *data.at(3) = -((*sprite).y2 as u16 as i16 + 32) * 16;
            (*sprite).y2 = 0;
            CameraObjectFreeze();
            (*object).set_fixedPriority(TRUE as u32);
            (*sprite).oam.set_priority(1);
            (*sprite).subpriority = 0;
            (*sprite).set_subspriteMode(SUBSPRITES_OFF);
            *data += 1;
        }
        if fall || sw1 == 1 {
            TrySpinPlayerForWarp(object, data.at(1));
            *data.at(3) += *data.at(2);
            *data.at(2) -= 3;
            if *data.at(2) < 4 {
                *data.at(2) = 4;
            }
            (*sprite).y = *data.at(3) >> 4;
            if (*sprite).y >= *data.at(4) {
                (*sprite).y = *data.at(4);
                *data.at(8) = 0;
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            TrySpinPlayerForWarp(object, data.at(1));
            if ({
                *data.at(8) += 1;
                *data.at(8)
            }) > 8
            {
                *data += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if *data.at(5) == TrySpinPlayerForWarp(object, data.at(1)) as i16 {
                (*object).set_fixedPriority(0);
                (*sprite).oam.set_priority(*data.at(6) as u16);
                (*sprite).subpriority = *data.at(7) as u8;
                CameraObjectReset();
                DestroyTask(taskId);
            }
            break 'l1;
        }
    }
}
unsafe fn TrySpinPlayerForWarp(object: *mut ObjectEvent, delayTimer: *mut i16) -> u8 {
    if *delayTimer < 8
        && ({
            *delayTimer += 1;
            *delayTimer
        }) < 8
    {
        return (*object).facingDirection() as u8;
    }
    if ObjectEventCheckHeldMovementStatus(object) == 0 {
        return (*object).facingDirection() as u8;
    }
    ObjectEventForceSetHeldMovement(
        object,
        GetFaceDirectionMovementAction(sSpinDirections[(*object).facingDirection()] as u32),
    );
    *delayTimer = 0;
    sSpinDirections[(*object).facingDirection()]
}
