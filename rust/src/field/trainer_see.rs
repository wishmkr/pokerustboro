//! Translated from `src/trainer_see.c` by tools/rustport/c2rs.py.
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
    unused_assignments,
    unused_variables
)]

use crate::battle_pyramid::{CurrentBattlePyramidLocation, GetBattlePyramidTrainerFlag};
use crate::battle_setup::{
    ConfigureAndSetUpOneTrainerBattle, ConfigureTwoTrainersBattle, GetTrainerFlagFromScriptPointer,
    ResetTrainerOpponentIds, SetUpTwoTrainersBattle,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_object_movement::{
    FreezeObjectEventsExceptOne, GetCollisionAtCoords, GetCollisionFlagsAtCoords,
    GetFaceDirectionMovementAction, GetJumpInPlaceMovementAction,
    GetObjectEventScriptPointerByObjectEventId, GetOppositeDirection,
    GetTrainerFacingDirectionMovementType, GetWalkNormalMovementAction, MoveCoords,
    ObjectEventCheckHeldMovementStatus, ObjectEventClearHeldMovement,
    ObjectEventClearHeldMovementIfFinished, ObjectEventGetLocalIdAndMap,
    ObjectEventIsMovementOverridden, ObjectEventSetHeldMovement,
    OverrideTemplateCoordsForObjectEvent, SetTrainerMovementType,
    TryGetObjectEventIdByLocalIdAndMap, TryOverrideTemplateCoordsForObjectEvent,
    UnfreezeObjectEvents,
};
use crate::ffi::gSpecialVar_Result;
use crate::field_effect::{
    FieldEffectActiveListContains, FieldEffectStart, FieldEffectStop, gFieldEffectArguments,
};
use crate::field_player_avatar::{
    CancelPlayerForcedMovement, PlayerGetDestCoords, gObjectEvents, gPlayerAvatar,
};
use crate::load_save::gSaveBlock1Ptr;
use crate::pokemon::GetMonsStateToDoubles_2;
use crate::scrcmd::SetMovingNpcId;
use crate::script::ScriptContext_Enable;
use crate::sprite::gSprites;
use crate::task::{DestroyTask, SwitchTaskToFollowupFunc};
use crate::task::{gTasks, task_data_ptr, task_set};
use crate::trainer_hill::{GetHillTrainerFlag, GetTrainerHillTrainerScript, InTrainerHill};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::{LoadWordFromTwoHalfwords, StoreWordInTwoHalfwords};
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSpriteAtEnd` with this module's view of its types.
#[inline]
unsafe fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSpriteAtEnd(a0 as _, a1, a2, a3) }
}
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `ScriptMovement_StartObjectMovementScript` with this module's view of its types.
#[inline]
unsafe fn ScriptMovement_StartObjectMovementScript(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8 {
    unsafe { crate::script_movement::ScriptMovement_StartObjectMovementScript(a0, a1, a2, a3 as _) }
}
/// `SetTaskFuncWithFollowupFunc` with this module's view of its types.
#[inline]
unsafe fn SetTaskFuncWithFollowupFunc(
    a0: u8,
    a1: Option<unsafe fn(u8)>,
    a2: Option<unsafe fn(u8)>,
) {
    unsafe {
        crate::task::SetTaskFuncWithFollowupFunc(
            a0,
            core::mem::transmute(a1),
            core::mem::transmute(a2),
        );
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
const sLocalId: usize = 0;
const tFuncId: usize = 0;
const sMapNum: usize = 1;
const tObjEvent: usize = 1;
const sMapGroup: usize = 2;
const sYVelocity: usize = 3;
const tTrainerRange: usize = 3;
const sYOffset: usize = 4;
const tOutOfAshSpriteId: usize = 4;
const sFldEffId: usize = 7;
const tTrainerObjectEventId: usize = 7;
// Data tables (translate with cdata.py): sEmotion_ExclamationMarkGfx sEmotion_QuestionMarkGfx sEmotion_HeartGfx sDirectionalApproachDistanceFuncs sTrainerSeeFuncList sTrainerSeeFuncList2 sOamData_Icons sSpriteImageTable_ExclamationQuestionMark sSpriteImageTable_HeartIcon sSpriteAnim_Icons1 sSpriteAnim_Icons2 sSpriteAnimTable_Icons sSpriteTemplate_ExclamationQuestionMark sSpriteTemplate_HeartIcon

const TRSEE_EXCLAMATION: i16 = 1;
const TRSEE_MOVE_TO_PLAYER: i16 = 3;
const TRSEE_REVEAL_BURIED: i16 = 8;
const TRSEE_REVEAL_DISGUISE: i16 = 6;

static sDirectionalApproachDistanceFuncs: Table<
    CArray<Option<unsafe fn(*mut ObjectEvent, i16, i16, i16) -> u8>, 4>,
> = Table((&raw const crate::data::trainer_see::sDirectionalApproachDistanceFuncs).cast());
static sSpriteTemplate_ExclamationQuestionMark: Table<SpriteTemplate> =
    Table((&raw const crate::data::trainer_see::sSpriteTemplate_ExclamationQuestionMark).cast());
static sSpriteTemplate_HeartIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::trainer_see::sSpriteTemplate_HeartIcon).cast());
static sTrainerSeeFuncList: Table<
    CArray<Option<unsafe fn(u8, *mut Task, *mut ObjectEvent) -> u8>, 12>,
> = Table((&raw const crate::data::trainer_see::sTrainerSeeFuncList).cast());
static sTrainerSeeFuncList2: Table<
    CArray<Option<unsafe fn(u8, *mut Task, *mut ObjectEvent) -> u8>, 4>,
> = Table((&raw const crate::data::trainer_see::sTrainerSeeFuncList2).cast());

#[unsafe(link_section = "common_data")]
pub static mut gWhichTrainerToFaceAfterBattle: u16 = 0;
#[unsafe(link_section = "common_data")]
pub static mut gPostBattleMovementScript: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(link_section = "common_data")]
pub static mut gApproachingTrainers: CArray<ApproachingTrainer, 2> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gNoOfApproachingTrainers: u8 = 0;
#[unsafe(link_section = "common_data")]
pub static gTrainerApproachedPlayer: crate::global::Global<u8> = crate::global::Global::new(0);
#[unsafe(link_section = "ewram_data")]
pub static mut gApproachingTrainerId: u8 = 0;

pub unsafe fn CheckForTrainersWantingBattle() -> u8 {
    gNoOfApproachingTrainers = 0;
    gApproachingTrainerId = 0;
    let mut i: u8 = 0;
    'l2: while i < OBJECT_EVENTS_COUNT {
        'l1: {
            if gObjectEvents[i].active() == 0 {
                break 'l1;
            }
            if gObjectEvents[i].trainerType != TRAINER_TYPE_NORMAL
                && gObjectEvents[i].trainerType != TRAINER_TYPE_BURIED
            {
                break 'l1;
            }
            let numTrainers: u8 = CheckTrainer(i);
            if numTrainers == 2 {
                break 'l2;
            }
            if numTrainers == 0 {
                break 'l1;
            }
            if gNoOfApproachingTrainers > 1 {
                break 'l2;
            }
            if GetMonsStateToDoubles_2() != PLAYER_HAS_TWO_USABLE_MONS as u8 {
                break 'l2;
            }
        }
        i += 1;
    }
    if gNoOfApproachingTrainers == 1 {
        ResetTrainerOpponentIds();
        ConfigureAndSetUpOneTrainerBattle(
            gApproachingTrainers[gNoOfApproachingTrainers as i32 - 1].objectEventId,
            gApproachingTrainers[gNoOfApproachingTrainers as i32 - 1].trainerScriptPtr,
        );
        gTrainerApproachedPlayer.set(TRUE);
        return TRUE;
    } else if gNoOfApproachingTrainers == 2 {
        ResetTrainerOpponentIds();
        i = 0;
        while i < gNoOfApproachingTrainers {
            ConfigureTwoTrainersBattle(
                gApproachingTrainers[i].objectEventId,
                gApproachingTrainers[i].trainerScriptPtr,
            );
            i += 1;
            gApproachingTrainerId += 1;
        }
        SetUpTwoTrainersBattle();
        gApproachingTrainerId = 0;
        gTrainerApproachedPlayer.set(TRUE);
        return TRUE;
    } else {
        gTrainerApproachedPlayer.set(FALSE);
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CheckTrainer(objectEventId: u8) -> u8 {
    let mut scriptPtr: *mut u8 = null_mut();
    let mut numTrainers: u8 = 1;
    if InTrainerHill() == TRUE as u32 {
        scriptPtr = GetTrainerHillTrainerScript();
    } else {
        scriptPtr = GetObjectEventScriptPointerByObjectEventId(objectEventId);
    }
    if CurrentBattlePyramidLocation() != PYRAMID_LOCATION_NONE {
        if GetBattlePyramidTrainerFlag(objectEventId) != 0 {
            return 0;
        }
    } else if InTrainerHill() == TRUE as u32 {
        if GetHillTrainerFlag(objectEventId) != 0 {
            return 0;
        }
    } else {
        if GetTrainerFlagFromScriptPointer(scriptPtr) != 0 {
            return 0;
        }
    }
    let approachDistance: u8 = GetTrainerApproachDistance(&raw mut gObjectEvents[objectEventId]);
    if approachDistance != 0 {
        if *scriptPtr.at(1) == TRAINER_BATTLE_DOUBLE as u8
            || *scriptPtr.at(1) == TRAINER_BATTLE_REMATCH_DOUBLE as u8
            || *scriptPtr.at(1) == TRAINER_BATTLE_CONTINUE_SCRIPT_DOUBLE as u8
        {
            if GetMonsStateToDoubles_2() != PLAYER_HAS_TWO_USABLE_MONS as u8 {
                return 0;
            }
            numTrainers = 2;
        }
        gApproachingTrainers[gNoOfApproachingTrainers].objectEventId = objectEventId;
        gApproachingTrainers[gNoOfApproachingTrainers].trainerScriptPtr = scriptPtr;
        gApproachingTrainers[gNoOfApproachingTrainers].radius = approachDistance;
        InitTrainerApproachTask(&raw mut gObjectEvents[objectEventId], approachDistance - 1);
        gNoOfApproachingTrainers += 1;
        return numTrainers;
    }
    0
}
unsafe fn GetTrainerApproachDistance(trainerObj: *mut ObjectEvent) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut approachDistance: u8 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    if (*trainerObj).trainerType == TRAINER_TYPE_NORMAL {
        approachDistance = sDirectionalApproachDistanceFuncs
            [(*trainerObj).facingDirection() as i32 - 1]
            .unwrap_unchecked()(
            trainerObj,
            (*trainerObj).trainerRange_berryTreeId as i16,
            x,
            y,
        );
        return CheckPathBetweenTrainerAndPlayer(
            trainerObj,
            approachDistance,
            (*trainerObj).facingDirection() as u8,
        );
    } else {
        for i in 0..4u8 {
            approachDistance = sDirectionalApproachDistanceFuncs[i].unwrap_unchecked()(
                trainerObj,
                (*trainerObj).trainerRange_berryTreeId as i16,
                x,
                y,
            );
            if CheckPathBetweenTrainerAndPlayer(trainerObj, approachDistance, i + 1) != 0 {
                return approachDistance;
            }
        }
    }
    0
}
pub(crate) unsafe fn GetTrainerApproachDistanceSouth(
    trainerObj: *mut ObjectEvent,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    if (*trainerObj).currentCoords.x == x
        && y > (*trainerObj).currentCoords.y
        && y as i32 <= (*trainerObj).currentCoords.y as i32 + range as i32
    {
        return y as u8 - (*trainerObj).currentCoords.y as u8;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn GetTrainerApproachDistanceNorth(
    trainerObj: *mut ObjectEvent,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    if (*trainerObj).currentCoords.x == x
        && y < (*trainerObj).currentCoords.y
        && y as i32 >= (*trainerObj).currentCoords.y as i32 - range as i32
    {
        return (*trainerObj).currentCoords.y as u8 - y as u8;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn GetTrainerApproachDistanceWest(
    trainerObj: *mut ObjectEvent,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    if (*trainerObj).currentCoords.y == y
        && x < (*trainerObj).currentCoords.x
        && x as i32 >= (*trainerObj).currentCoords.x as i32 - range as i32
    {
        return (*trainerObj).currentCoords.x as u8 - x as u8;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn GetTrainerApproachDistanceEast(
    trainerObj: *mut ObjectEvent,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    if (*trainerObj).currentCoords.y == y
        && x > (*trainerObj).currentCoords.x
        && x as i32 <= (*trainerObj).currentCoords.x as i32 + range as i32
    {
        return x as u8 - (*trainerObj).currentCoords.x as u8;
    } else {
        return 0;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn CheckPathBetweenTrainerAndPlayer(
    trainerObj: *mut ObjectEvent,
    approachDistance: u8,
    direction: u8,
) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut rangeX: u8 = 0;
    let mut rangeY: u8 = 0;
    let mut collision: u8 = 0;
    if approachDistance == 0 {
        return 0;
    }
    x = (*trainerObj).currentCoords.x;
    y = (*trainerObj).currentCoords.y;
    MoveCoords(direction, &raw mut x, &raw mut y);
    let mut i: u8 = 0;
    while (i as i32) < approachDistance as i32 - 1 {
        collision = GetCollisionFlagsAtCoords(trainerObj, x, y, direction);
        if collision != 0 && collision as i32 & -2 != 0 {
            return 0;
        }
        i += 1;
        MoveCoords(direction, &raw mut x, &raw mut y);
    }
    rangeX = (*trainerObj).range.rangeX();
    rangeY = (*trainerObj).range.rangeY();
    (*trainerObj).range.set_rangeX(0);
    (*trainerObj).range.set_rangeY(0);
    collision = GetCollisionAtCoords(trainerObj, x, y, direction as u32);
    (*trainerObj).range.set_rangeX(rangeX);
    (*trainerObj).range.set_rangeY(rangeY);
    if collision == COLLISION_OBJECT_EVENT {
        return approachDistance;
    }
    0
}
unsafe fn InitTrainerApproachTask(trainerObj: *mut ObjectEvent, range: u8) {
    gApproachingTrainers[gNoOfApproachingTrainers].taskId =
        CreateTask(Some(Task_RunTrainerSeeFuncList), 0x50);
    let task: *mut Task =
        &raw mut (*gTasks.as_ptr())[gApproachingTrainers[gNoOfApproachingTrainers].taskId];
    (*task).data[tTrainerRange] = range as i16;
    (*task).data[tTrainerObjectEventId] =
        gApproachingTrainers[gNoOfApproachingTrainers].objectEventId as i16;
}
unsafe fn StartTrainerApproach(followupFunc: Option<unsafe fn(u8)>) {
    let mut taskId: u8 = 0;
    let mut taskFunc: Option<unsafe fn(u8)> = None;
    if gApproachingTrainerId == 0 {
        taskId = gApproachingTrainers[0].taskId;
    } else {
        taskId = gApproachingTrainers[1].taskId;
    }
    taskFunc = Some(Task_RunTrainerSeeFuncList);
    SetTaskFuncWithFollowupFunc(taskId, taskFunc, followupFunc);
    task_set(taskId, tFuncId, TRSEE_EXCLAMATION);
    taskFunc.unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn Task_RunTrainerSeeFuncList(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let trainerObj: *mut ObjectEvent = &raw mut gObjectEvents[(*task).data[tTrainerObjectEventId]];
    if (*trainerObj).active() == 0 {
        SwitchTaskToFollowupFunc(taskId);
    } else {
        while sTrainerSeeFuncList[(*task).data[tFuncId]].unwrap_unchecked()(
            taskId, task, trainerObj,
        ) != 0
        {}
    }
}
pub(crate) fn TrainerSeeIdle(taskId: u8, task: *mut Task, trainerObj: *mut ObjectEvent) -> u8 {
    FALSE
}
pub(crate) unsafe fn TrainerExclamationMark(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    ObjectEventGetLocalIdAndMap(
        trainerObj,
        &raw mut gFieldEffectArguments[0] as *mut c_void,
        &raw mut gFieldEffectArguments[1] as *mut c_void,
        &raw mut gFieldEffectArguments[2] as *mut c_void,
    );
    FieldEffectStart(FLDEFF_EXCLAMATION_MARK_ICON);
    let direction: u8 = GetFaceDirectionMovementAction((*trainerObj).facingDirection() as u32);
    ObjectEventSetHeldMovement(trainerObj, direction);
    (*task).data[tFuncId] += 1;
    TRUE
}
pub(crate) unsafe fn WaitTrainerExclamationMark(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_EXCLAMATION_MARK_ICON) != 0 {
        return FALSE;
    } else {
        (*task).data[tFuncId] += 1;
        if (*trainerObj).movementType == MOVEMENT_TYPE_TREE_DISGUISE
            || (*trainerObj).movementType == MOVEMENT_TYPE_MOUNTAIN_DISGUISE
        {
            (*task).data[tFuncId] = TRSEE_REVEAL_DISGUISE;
        }
        if (*trainerObj).movementType == MOVEMENT_TYPE_BURIED {
            (*task).data[tFuncId] = TRSEE_REVEAL_BURIED;
        }
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub(crate) unsafe fn TrainerMoveToPlayer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(trainerObj) != 0
    {
        if (*task).data[tTrainerRange] != 0 {
            ObjectEventSetHeldMovement(
                trainerObj,
                GetWalkNormalMovementAction((*trainerObj).facingDirection() as u32),
            );
            (*task).data[tTrainerRange] -= 1;
        } else {
            ObjectEventSetHeldMovement(trainerObj, MOVEMENT_ACTION_FACE_PLAYER);
            (*task).data[tFuncId] += 1;
        }
    }
    FALSE
}
pub(crate) unsafe fn PlayerFaceApproachingTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) != 0
        && ObjectEventClearHeldMovementIfFinished(trainerObj) == 0
    {
        return FALSE;
    }
    SetTrainerMovementType(
        trainerObj,
        GetTrainerFacingDirectionMovementType((*trainerObj).facingDirection() as u8),
    );
    TryOverrideTemplateCoordsForObjectEvent(
        trainerObj,
        GetTrainerFacingDirectionMovementType((*trainerObj).facingDirection() as u8),
    );
    OverrideTemplateCoordsForObjectEvent(trainerObj);
    let playerObj: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(playerObj) != 0
        && ObjectEventClearHeldMovementIfFinished(playerObj) == 0
    {
        return FALSE;
    }
    CancelPlayerForcedMovement();
    ObjectEventSetHeldMovement(
        &raw mut gObjectEvents[gPlayerAvatar.objectEventId],
        GetFaceDirectionMovementAction(
            GetOppositeDirection((*trainerObj).facingDirection() as u8) as u32
        ),
    );
    (*task).data[tFuncId] += 1;
    FALSE
}
pub(crate) unsafe fn WaitPlayerFaceApproachingTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    let playerObj: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(playerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(playerObj) != 0
    {
        SwitchTaskToFollowupFunc(taskId);
    }
    FALSE
}
pub(crate) unsafe fn RevealDisguisedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(trainerObj) != 0
    {
        ObjectEventSetHeldMovement(trainerObj, MOVEMENT_ACTION_REVEAL_TRAINER);
        (*task).data[tFuncId] += 1;
    }
    FALSE
}
pub(crate) unsafe fn WaitRevealDisguisedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventClearHeldMovementIfFinished(trainerObj) != 0 {
        (*task).data[tFuncId] = TRSEE_MOVE_TO_PLAYER;
    }
    FALSE
}
pub(crate) unsafe fn RevealBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(trainerObj) != 0
    {
        ObjectEventSetHeldMovement(trainerObj, MOVEMENT_ACTION_FACE_PLAYER);
        (*task).data[tFuncId] += 1;
    }
    FALSE
}
pub(crate) unsafe fn PopOutOfAshBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventCheckHeldMovementStatus(trainerObj) != 0 {
        gFieldEffectArguments[0] = (*trainerObj).currentCoords.x as i32;
        gFieldEffectArguments[1] = (*trainerObj).currentCoords.y as i32;
        gFieldEffectArguments[2] = gSprites[(*trainerObj).spriteId].subpriority as i32 - 1;
        gFieldEffectArguments[3] = 2;
        (*task).data[tOutOfAshSpriteId] = FieldEffectStart(FLDEFF_ASH_PUFF) as i16;
        (*task).data[tFuncId] += 1;
    }
    FALSE
}
pub(crate) unsafe fn JumpInPlaceBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    if gSprites[(*task).data[tOutOfAshSpriteId]].animCmdIndex == 2 {
        (*trainerObj).set_fixedPriority(0);
        (*trainerObj).set_triggerGroundEffectsOnMove(TRUE as u32);
        sprite = &raw mut gSprites[(*trainerObj).spriteId];
        (*sprite).oam.set_priority(2);
        ObjectEventClearHeldMovementIfFinished(trainerObj);
        ObjectEventSetHeldMovement(
            trainerObj,
            GetJumpInPlaceMovementAction((*trainerObj).facingDirection() as u32),
        );
        (*task).data[tFuncId] += 1;
    }
    FALSE
}
pub(crate) unsafe fn WaitRevealBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_ASH_PUFF) == 0 {
        (*task).data[tFuncId] = TRSEE_MOVE_TO_PLAYER;
    }
    FALSE
}
pub(crate) unsafe fn Task_SetBuriedTrainerMovement(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let mut objEvent: *mut ObjectEvent = null_mut();
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[tObjEvent] as *mut u16,
        &raw mut objEvent as *mut u32,
    );
    if (*task).data[7] == 0 {
        ObjectEventClearHeldMovement(objEvent);
        (*task).data[7] += 1;
    }
    sTrainerSeeFuncList2[(*task).data[tFuncId]].unwrap_unchecked()(taskId, task, objEvent);
    if (*task).data[tFuncId] == 3 && FieldEffectActiveListContains(FLDEFF_ASH_PUFF) == 0 {
        SetTrainerMovementType(
            objEvent,
            GetTrainerFacingDirectionMovementType((*objEvent).facingDirection() as u8),
        );
        TryOverrideTemplateCoordsForObjectEvent(
            objEvent,
            GetTrainerFacingDirectionMovementType((*objEvent).facingDirection() as u8),
        );
        DestroyTask(taskId);
    } else {
        (*objEvent).set_heldMovementFinished(0);
    }
}
pub unsafe fn SetBuriedTrainerMovement(objEvent: *mut ObjectEvent) {
    StoreWordInTwoHalfwords(
        task_data_ptr(
            CreateTask(Some(Task_SetBuriedTrainerMovement), 0),
            tObjEvent,
        ) as *mut u16,
        objEvent as usize as u32,
    );
}
#[unsafe(no_mangle)]
pub unsafe fn DoTrainerApproach() {
    StartTrainerApproach(Some(Task_EndTrainerApproach));
}
pub(crate) unsafe fn Task_EndTrainerApproach(taskId: u8) {
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe fn TryPrepareSecondApproachingTrainer() {
    if gNoOfApproachingTrainers == 2 {
        if gApproachingTrainerId == 0 {
            gApproachingTrainerId += 1;
            gSpecialVar_Result = TRUE as u16;
            UnfreezeObjectEvents();
            FreezeObjectEventsExceptOne(gApproachingTrainers[1].objectEventId);
        } else {
            gApproachingTrainerId = 0;
            gSpecialVar_Result = FALSE as u16;
        }
    } else {
        gSpecialVar_Result = FALSE as u16;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_ExclamationMarkIcon() -> u8 {
    let spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_ExclamationQuestionMark).cast_mut(),
        0,
        0,
        0x53,
    );
    if spriteId != MAX_SPRITES {
        SetIconSpriteData(&raw mut gSprites[spriteId], 0, 0);
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_QuestionMarkIcon() -> u8 {
    let spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_ExclamationQuestionMark).cast_mut(),
        0,
        0,
        0x52,
    );
    if spriteId != MAX_SPRITES {
        SetIconSpriteData(
            &raw mut gSprites[spriteId],
            FLDEFF_QUESTION_MARK_ICON as u16,
            1,
        );
    }
    0
}
#[unsafe(no_mangle)]
pub unsafe fn FldEff_HeartIcon() -> u8 {
    let spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_HeartIcon).cast_mut(),
        0,
        0,
        0x52,
    );
    if spriteId != MAX_SPRITES {
        let sprite: *mut Sprite = &raw mut gSprites[spriteId];
        SetIconSpriteData(sprite, FLDEFF_HEART_ICON as u16, 0);
        (*sprite).oam.set_paletteNum(2);
    }
    0
}
unsafe fn SetIconSpriteData(sprite: *mut Sprite, fldEffId: u16, spriteAnimNum: u8) {
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(1);
    (*sprite).data[sLocalId] = gFieldEffectArguments[0] as i16;
    (*sprite).data[sMapNum] = gFieldEffectArguments[1] as i16;
    (*sprite).data[sMapGroup] = gFieldEffectArguments[2] as i16;
    (*sprite).data[sYVelocity] = -5;
    (*sprite).data[sFldEffId] = fldEffId as i16;
    StartSpriteAnim(sprite, spriteAnimNum);
}
pub(crate) unsafe fn SpriteCB_TrainerIcons(sprite: *mut Sprite) {
    let mut objEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[sLocalId] as u8,
        (*sprite).data[sMapNum] as u8,
        (*sprite).data[sMapGroup] as u8,
        &raw mut objEventId,
    ) != 0
        || (*sprite).animEnded() != 0
    {
        FieldEffectStop(sprite, (*sprite).data[sFldEffId] as u8);
    } else {
        let objEventSprite: *mut Sprite = &raw mut gSprites[gObjectEvents[objEventId].spriteId];
        (*sprite).data[sYOffset] += (*sprite).data[sYVelocity];
        (*sprite).x = (*objEventSprite).x;
        (*sprite).y = (*objEventSprite).y - 16;
        (*sprite).x2 = (*objEventSprite).x2;
        (*sprite).y2 = (*objEventSprite).y2 + (*sprite).data[sYOffset];
        if (*sprite).data[sYOffset] != 0 {
            (*sprite).data[sYVelocity] += 1;
        } else {
            (*sprite).data[sYVelocity] = 0;
        }
    }
}
pub unsafe fn GetCurrentApproachingTrainerObjectEventId() -> u8 {
    if gApproachingTrainerId == 0 {
        return gApproachingTrainers[0].objectEventId;
    } else {
        return gApproachingTrainers[1].objectEventId;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn GetChosenApproachingTrainerObjectEventId(arrayId: u8) -> u8 {
    if arrayId >= 2 {
        return 0;
    } else if arrayId == 0 {
        return gApproachingTrainers[0].objectEventId;
    } else {
        return gApproachingTrainers[1].objectEventId;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn PlayerFaceTrainerAfterBattle() {
    let mut objEvent: *mut ObjectEvent = null_mut();
    if gTrainerApproachedPlayer.get() == TRUE {
        objEvent = &raw mut gObjectEvents
            [gApproachingTrainers[gWhichTrainerToFaceAfterBattle].objectEventId];
        gPostBattleMovementScript[0] = GetFaceDirectionMovementAction(GetOppositeDirection(
            (*objEvent).facingDirection() as u8,
        ) as u32);
        gPostBattleMovementScript[1] = MOVEMENT_ACTION_STEP_END;
        ScriptMovement_StartObjectMovementScript(
            LOCALID_PLAYER,
            (*gSaveBlock1Ptr).location.mapNum as u8,
            (*gSaveBlock1Ptr).location.mapGroup as u8,
            gPostBattleMovementScript.as_mut_ptr(),
        );
    } else {
        objEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
        gPostBattleMovementScript[0] =
            GetFaceDirectionMovementAction((*objEvent).facingDirection() as u32);
        gPostBattleMovementScript[1] = MOVEMENT_ACTION_STEP_END;
        ScriptMovement_StartObjectMovementScript(
            LOCALID_PLAYER,
            (*gSaveBlock1Ptr).location.mapNum as u8,
            (*gSaveBlock1Ptr).location.mapGroup as u8,
            gPostBattleMovementScript.as_mut_ptr(),
        );
    }
    SetMovingNpcId(LOCALID_PLAYER as u16);
}
