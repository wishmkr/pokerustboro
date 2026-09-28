//! Translated from `src/trainer_see.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sEmotion_ExclamationMarkGfx sEmotion_QuestionMarkGfx sEmotion_HeartGfx sDirectionalApproachDistanceFuncs sTrainerSeeFuncList sTrainerSeeFuncList2 sOamData_Icons sSpriteImageTable_ExclamationQuestionMark sSpriteImageTable_HeartIcon sSpriteAnim_Icons1 sSpriteAnim_Icons2 sSpriteAnimTable_Icons sSpriteTemplate_ExclamationQuestionMark sSpriteTemplate_HeartIcon

const TRSEE_EXCLAMATION: i16 = 1;
const TRSEE_MOVE_TO_PLAYER: i16 = 3;
const TRSEE_REVEAL_BURIED: i16 = 8;
const TRSEE_REVEAL_DISGUISE: i16 = 6;

static sDirectionalApproachDistanceFuncs: Table<
    CArray<Option<unsafe extern "C" fn(*mut ObjectEvent, i16, i16, i16) -> u8>, 4>,
> = Table((&raw const crate::data::trainer_see::sDirectionalApproachDistanceFuncs).cast());
static sSpriteTemplate_ExclamationQuestionMark: Table<SpriteTemplate> =
    Table((&raw const crate::data::trainer_see::sSpriteTemplate_ExclamationQuestionMark).cast());
static sSpriteTemplate_HeartIcon: Table<SpriteTemplate> =
    Table((&raw const crate::data::trainer_see::sSpriteTemplate_HeartIcon).cast());
static sTrainerSeeFuncList: Table<
    CArray<Option<unsafe extern "C" fn(u8, *mut Task, *mut ObjectEvent) -> u8>, 12>,
> = Table((&raw const crate::data::trainer_see::sTrainerSeeFuncList).cast());
static sTrainerSeeFuncList2: Table<
    CArray<Option<unsafe extern "C" fn(u8, *mut Task, *mut ObjectEvent) -> u8>, 4>,
> = Table((&raw const crate::data::trainer_see::sTrainerSeeFuncList2).cast());

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gWhichTrainerToFaceAfterBattle: u16 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPostBattleMovementScript: Aligned<CArray<u8, 4>> = Aligned(unsafe { zeroed() });
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApproachingTrainers: CArray<ApproachingTrainer, 2> = unsafe { zeroed() };
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gNoOfApproachingTrainers: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTrainerApproachedPlayer: u8 = 0;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gApproachingTrainerId: u8 = 0;

unsafe extern "C" {
    static mut gFieldEffectArguments: CArray<i32, 8>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_Result: u16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn CancelPlayerForcedMovement();
    fn ConfigureAndSetUpOneTrainerBattle(a0: u8, a1: *mut u8);
    fn ConfigureTwoTrainersBattle(a0: u8, a1: *mut u8);
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut Sprite, a1: u8);
    fn FreezeObjectEventsExceptOne(a0: u8);
    fn GetBattlePyramidTrainerFlag(a0: u8) -> u8;
    fn GetCollisionAtCoords(a0: *mut ObjectEvent, a1: i16, a2: i16, a3: u32) -> u8;
    fn GetCollisionFlagsAtCoords(a0: *mut ObjectEvent, a1: i16, a2: i16, a3: u8) -> u8;
    fn GetFaceDirectionMovementAction(a0: u32) -> u8;
    fn GetHillTrainerFlag(a0: u8) -> u8;
    fn GetJumpInPlaceMovementAction(a0: u32) -> u8;
    fn GetMonsStateToDoubles_2() -> u8;
    fn GetObjectEventScriptPointerByObjectEventId(a0: u8) -> *mut u8;
    fn GetOppositeDirection(a0: u8) -> u8;
    fn GetTrainerFacingDirectionMovementType(a0: u8) -> u8;
    fn GetTrainerFlagFromScriptPointer(a0: *mut u8) -> u32;
    fn GetTrainerHillTrainerScript() -> *mut u8;
    fn GetWalkNormalMovementAction(a0: u32) -> u8;
    fn InTrainerHill() -> u32;
    fn LoadWordFromTwoHalfwords(a0: *mut u16, a1: *mut u32);
    fn MoveCoords(a0: u8, a1: *mut i16, a2: *mut i16);
    fn ObjectEventCheckHeldMovementStatus(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventClearHeldMovement(a0: *mut ObjectEvent);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventGetLocalIdAndMap(
        a0: *mut ObjectEvent,
        a1: *mut c_void,
        a2: *mut c_void,
        a3: *mut c_void,
    );
    fn ObjectEventIsMovementOverridden(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut ObjectEvent, a1: u8) -> u8;
    fn OverrideTemplateCoordsForObjectEvent(a0: *mut ObjectEvent);
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn ResetTrainerOpponentIds();
    fn ScriptContext_Enable();
    fn ScriptMovement_StartObjectMovementScript(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn SetMovingNpcId(a0: u16);
    fn SetTaskFuncWithFollowupFunc(
        a0: u8,
        a1: Option<unsafe extern "C" fn(u8)>,
        a2: Option<unsafe extern "C" fn(u8)>,
    );
    fn SetTrainerMovementType(a0: *mut ObjectEvent, a1: u8);
    fn SetUpTwoTrainersBattle();
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreWordInTwoHalfwords(a0: *mut u16, a1: u32);
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn TryOverrideTemplateCoordsForObjectEvent(a0: *mut ObjectEvent, a1: u8);
    fn UnfreezeObjectEvents();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForTrainersWantingBattle() -> u8 {
    let mut i: u8 = 0;
    gNoOfApproachingTrainers = 0;
    gApproachingTrainerId = 0;
    i = 0;
    'l2: while i < OBJECT_EVENTS_COUNT {
        'l1: {
            let mut numTrainers: u8 = 0;
            if gObjectEvents[i].active() == 0 {
                break 'l1;
            }
            if gObjectEvents[i].trainerType != TRAINER_TYPE_NORMAL
                && gObjectEvents[i].trainerType != TRAINER_TYPE_BURIED
            {
                break 'l1;
            }
            numTrainers = CheckTrainer(i);
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
        gTrainerApproachedPlayer = TRUE;
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
        gTrainerApproachedPlayer = TRUE;
        return TRUE;
    } else {
        gTrainerApproachedPlayer = FALSE;
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CheckTrainer(objectEventId: u8) -> u8 {
    let mut scriptPtr: *mut u8 = null_mut();
    let mut numTrainers: u8 = 1;
    let mut approachDistance: u8 = 0;
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
    approachDistance = GetTrainerApproachDistance(&raw mut gObjectEvents[objectEventId]);
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
    return 0;
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistance(trainerObj: *mut ObjectEvent) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut i: u8 = 0;
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
        i = 0;
        while i < 4 {
            approachDistance = sDirectionalApproachDistanceFuncs[i].unwrap_unchecked()(
                trainerObj,
                (*trainerObj).trainerRange_berryTreeId as i16,
                x,
                y,
            );
            if CheckPathBetweenTrainerAndPlayer(trainerObj, approachDistance, i + 1) != 0 {
                return approachDistance;
            }
            i += 1;
        }
    }
    return 0;
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceSouth(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceNorth(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceWest(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceEast(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn CheckPathBetweenTrainerAndPlayer(
    trainerObj: *mut ObjectEvent,
    approachDistance: u8,
    direction: u8,
) -> u8 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut rangeX: u8 = 0;
    let mut rangeY: u8 = 0;
    let mut i: u8 = 0;
    let mut collision: u8 = 0;
    if approachDistance == 0 {
        return 0;
    }
    x = (*trainerObj).currentCoords.x;
    y = (*trainerObj).currentCoords.y;
    MoveCoords(direction, &raw mut x, &raw mut y);
    i = 0;
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
    return 0;
}
pub(crate) unsafe extern "C" fn InitTrainerApproachTask(trainerObj: *mut ObjectEvent, range: u8) {
    let mut task: *mut Task = null_mut();
    gApproachingTrainers[gNoOfApproachingTrainers].taskId =
        CreateTask(Some(Task_RunTrainerSeeFuncList), 0x50);
    task = &raw mut gTasks[gApproachingTrainers[gNoOfApproachingTrainers].taskId];
    (*task).data[3] = range as i16;
    (*task).data[7] = gApproachingTrainers[gNoOfApproachingTrainers].objectEventId as i16;
}
pub(crate) unsafe extern "C" fn StartTrainerApproach(
    followupFunc: Option<unsafe extern "C" fn(u8)>,
) {
    let mut taskId: u8 = 0;
    let mut taskFunc: Option<unsafe extern "C" fn(u8)> = None;
    if gApproachingTrainerId == 0 {
        taskId = gApproachingTrainers[0].taskId;
    } else {
        taskId = gApproachingTrainers[1].taskId;
    }
    taskFunc = Some(Task_RunTrainerSeeFuncList);
    SetTaskFuncWithFollowupFunc(taskId, taskFunc, followupFunc);
    gTasks[taskId].data[0] = TRSEE_EXCLAMATION;
    taskFunc.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn Task_RunTrainerSeeFuncList(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut trainerObj: *mut ObjectEvent = &raw mut gObjectEvents[(*task).data[7]];
    if (*trainerObj).active() == 0 {
        SwitchTaskToFollowupFunc(taskId);
    } else {
        while sTrainerSeeFuncList[(*task).data[0]].unwrap_unchecked()(taskId, task, trainerObj) != 0
        {
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerSeeIdle(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    return FALSE;
}
pub(crate) unsafe extern "C" fn TrainerExclamationMark(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    let mut direction: u8 = 0;
    ObjectEventGetLocalIdAndMap(
        trainerObj,
        &raw mut gFieldEffectArguments[0] as *mut c_void,
        &raw mut gFieldEffectArguments[1] as *mut c_void,
        &raw mut gFieldEffectArguments[2] as *mut c_void,
    );
    FieldEffectStart(FLDEFF_EXCLAMATION_MARK_ICON);
    direction = GetFaceDirectionMovementAction((*trainerObj).facingDirection() as u32);
    ObjectEventSetHeldMovement(trainerObj, direction);
    (*task).data[0] += 1;
    return TRUE;
}
pub(crate) unsafe extern "C" fn WaitTrainerExclamationMark(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_EXCLAMATION_MARK_ICON) != 0 {
        return FALSE;
    } else {
        (*task).data[0] += 1;
        if (*trainerObj).movementType == MOVEMENT_TYPE_TREE_DISGUISE
            || (*trainerObj).movementType == MOVEMENT_TYPE_MOUNTAIN_DISGUISE
        {
            (*task).data[0] = TRSEE_REVEAL_DISGUISE;
        }
        if (*trainerObj).movementType == MOVEMENT_TYPE_BURIED {
            (*task).data[0] = TRSEE_REVEAL_BURIED;
        }
        return TRUE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn TrainerMoveToPlayer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(trainerObj) != 0
    {
        if (*task).data[3] != 0 {
            ObjectEventSetHeldMovement(
                trainerObj,
                GetWalkNormalMovementAction((*trainerObj).facingDirection() as u32),
            );
            (*task).data[3] -= 1;
        } else {
            ObjectEventSetHeldMovement(trainerObj, MOVEMENT_ACTION_FACE_PLAYER);
            (*task).data[0] += 1;
        }
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PlayerFaceApproachingTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    let mut playerObj: *mut ObjectEvent = null_mut();
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
    playerObj = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
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
    (*task).data[0] += 1;
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitPlayerFaceApproachingTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    let mut playerObj: *mut ObjectEvent = &raw mut gObjectEvents[gPlayerAvatar.objectEventId];
    if ObjectEventIsMovementOverridden(playerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(playerObj) != 0
    {
        SwitchTaskToFollowupFunc(taskId);
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn RevealDisguisedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(trainerObj) != 0
    {
        ObjectEventSetHeldMovement(trainerObj, MOVEMENT_ACTION_REVEAL_TRAINER);
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitRevealDisguisedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventClearHeldMovementIfFinished(trainerObj) != 0 {
        (*task).data[0] = TRSEE_MOVE_TO_PLAYER;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn RevealBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventIsMovementOverridden(trainerObj) == 0
        || ObjectEventClearHeldMovementIfFinished(trainerObj) != 0
    {
        ObjectEventSetHeldMovement(trainerObj, MOVEMENT_ACTION_FACE_PLAYER);
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn PopOutOfAshBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if ObjectEventCheckHeldMovementStatus(trainerObj) != 0 {
        gFieldEffectArguments[0] = (*trainerObj).currentCoords.x as i32;
        gFieldEffectArguments[1] = (*trainerObj).currentCoords.y as i32;
        gFieldEffectArguments[2] = gSprites[(*trainerObj).spriteId].subpriority as i32 - 1;
        gFieldEffectArguments[3] = 2;
        (*task).data[4] = FieldEffectStart(FLDEFF_ASH_PUFF) as i16;
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn JumpInPlaceBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    let mut sprite: *mut Sprite = null_mut();
    if gSprites[(*task).data[4]].animCmdIndex == 2 {
        (*trainerObj).set_fixedPriority(0);
        (*trainerObj).set_triggerGroundEffectsOnMove(TRUE as u32);
        sprite = &raw mut gSprites[(*trainerObj).spriteId];
        (*sprite).oam.set_priority(2);
        ObjectEventClearHeldMovementIfFinished(trainerObj);
        ObjectEventSetHeldMovement(
            trainerObj,
            GetJumpInPlaceMovementAction((*trainerObj).facingDirection() as u32),
        );
        (*task).data[0] += 1;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn WaitRevealBuriedTrainer(
    taskId: u8,
    task: *mut Task,
    trainerObj: *mut ObjectEvent,
) -> u8 {
    if FieldEffectActiveListContains(FLDEFF_ASH_PUFF) == 0 {
        (*task).data[0] = TRSEE_MOVE_TO_PLAYER;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn Task_SetBuriedTrainerMovement(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut objEvent: *mut ObjectEvent = null_mut();
    LoadWordFromTwoHalfwords(
        &raw mut (*task).data[1] as *mut u16,
        &raw mut objEvent as *mut u32,
    );
    if (*task).data[7] == 0 {
        ObjectEventClearHeldMovement(objEvent);
        (*task).data[7] += 1;
    }
    sTrainerSeeFuncList2[(*task).data[0]].unwrap_unchecked()(taskId, task, objEvent);
    if (*task).data[0] == 3 && FieldEffectActiveListContains(FLDEFF_ASH_PUFF) == 0 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBuriedTrainerMovement(objEvent: *mut ObjectEvent) {
    StoreWordInTwoHalfwords(
        &raw mut gTasks[CreateTask(Some(Task_SetBuriedTrainerMovement), 0)].data[1] as *mut u16,
        objEvent as usize as u32,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTrainerApproach() {
    StartTrainerApproach(Some(Task_EndTrainerApproach));
}
pub(crate) unsafe extern "C" fn Task_EndTrainerApproach(taskId: u8) {
    DestroyTask(taskId);
    ScriptContext_Enable();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPrepareSecondApproachingTrainer() {
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
pub unsafe extern "C" fn FldEff_ExclamationMarkIcon() -> u8 {
    let mut spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_ExclamationQuestionMark).cast_mut(),
        0,
        0,
        0x53,
    );
    if spriteId != MAX_SPRITES {
        SetIconSpriteData(&raw mut gSprites[spriteId], 0, 0);
    }
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_QuestionMarkIcon() -> u8 {
    let mut spriteId: u8 = CreateSpriteAtEnd(
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
    return 0;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_HeartIcon() -> u8 {
    let mut spriteId: u8 = CreateSpriteAtEnd(
        (&raw const *sSpriteTemplate_HeartIcon).cast_mut(),
        0,
        0,
        0x52,
    );
    if spriteId != MAX_SPRITES {
        let mut sprite: *mut Sprite = &raw mut gSprites[spriteId];
        SetIconSpriteData(sprite, FLDEFF_HEART_ICON as u16, 0);
        (*sprite).oam.set_paletteNum(2);
    }
    return 0;
}
pub(crate) unsafe extern "C" fn SetIconSpriteData(
    sprite: *mut Sprite,
    fldEffId: u16,
    spriteAnimNum: u8,
) {
    (*sprite).oam.set_priority(1);
    (*sprite).set_coordOffsetEnabled(1);
    (*sprite).data[0] = gFieldEffectArguments[0] as i16;
    (*sprite).data[1] = gFieldEffectArguments[1] as i16;
    (*sprite).data[2] = gFieldEffectArguments[2] as i16;
    (*sprite).data[3] = -5;
    (*sprite).data[7] = fldEffId as i16;
    StartSpriteAnim(sprite, spriteAnimNum);
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIcons(sprite: *mut Sprite) {
    let mut objEventId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        (*sprite).data[0] as u8,
        (*sprite).data[1] as u8,
        (*sprite).data[2] as u8,
        &raw mut objEventId,
    ) != 0
        || (*sprite).animEnded() != 0
    {
        FieldEffectStop(sprite, (*sprite).data[7] as u8);
    } else {
        let mut objEventSprite: *mut Sprite = &raw mut gSprites[gObjectEvents[objEventId].spriteId];
        (*sprite).data[4] += (*sprite).data[3];
        (*sprite).x = (*objEventSprite).x;
        (*sprite).y = (*objEventSprite).y - 16;
        (*sprite).x2 = (*objEventSprite).x2;
        (*sprite).y2 = (*objEventSprite).y2 + (*sprite).data[4];
        if (*sprite).data[4] != 0 {
            (*sprite).data[3] += 1;
        } else {
            (*sprite).data[3] = 0;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentApproachingTrainerObjectEventId() -> u8 {
    if gApproachingTrainerId == 0 {
        return gApproachingTrainers[0].objectEventId;
    } else {
        return gApproachingTrainers[1].objectEventId;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetChosenApproachingTrainerObjectEventId(arrayId: u8) -> u8 {
    if arrayId >= 2 {
        return 0;
    } else if arrayId == 0 {
        return gApproachingTrainers[0].objectEventId;
    } else {
        return gApproachingTrainers[1].objectEventId;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerFaceTrainerAfterBattle() {
    let mut objEvent: *mut ObjectEvent = null_mut();
    if gTrainerApproachedPlayer == TRUE {
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
