//! Translated from `src/trainer_see.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sEmotion_ExclamationMarkGfx sEmotion_QuestionMarkGfx sEmotion_HeartGfx sDirectionalApproachDistanceFuncs sTrainerSeeFuncList sTrainerSeeFuncList2 sOamData_Icons sSpriteImageTable_ExclamationQuestionMark sSpriteImageTable_HeartIcon sSpriteAnim_Icons1 sSpriteAnim_Icons2 sSpriteAnimTable_Icons sSpriteTemplate_ExclamationQuestionMark sSpriteTemplate_HeartIcon
#[allow(unused_imports)]
use crate::data::trainer_see::*;

#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gWhichTrainerToFaceAfterBattle: u16 = 0u16;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gPostBattleMovementScript: crate::ffi::Align4<[u8; 4]> = crate::ffi::Align4([0; 4]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gApproachingTrainers: crate::ffi::Align4<[u8; 24]> = crate::ffi::Align4([0; 24]);
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gNoOfApproachingTrainers: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "common_data")]
pub static mut gTrainerApproachedPlayer: u8 = 0u8;
#[unsafe(no_mangle)]
#[unsafe(link_section = "ewram_data")]
pub static mut gApproachingTrainerId: u8 = 0u8;

unsafe extern "C" {
    static mut gFieldEffectArguments: u8;
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSpecialVar_Result: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn CancelPlayerForcedMovement();
    fn ConfigureAndSetUpOneTrainerBattle(a0: u8, a1: *mut u8);
    fn ConfigureTwoTrainersBattle(a0: u8, a1: *mut u8);
    fn CreateSpriteAtEnd(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CurrentBattlePyramidLocation() -> u8;
    fn DestroyTask(a0: u8);
    fn FieldEffectActiveListContains(a0: u8) -> u8;
    fn FieldEffectStart(a0: u8) -> u32;
    fn FieldEffectStop(a0: *mut u8, a1: u8);
    fn FreezeObjectEventsExceptOne(a0: u8);
    fn GetBattlePyramidTrainerFlag(a0: u8) -> u8;
    fn GetCollisionAtCoords(a0: *mut u8, a1: i16, a2: i16, a3: u32) -> u8;
    fn GetCollisionFlagsAtCoords(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
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
    fn ObjectEventCheckHeldMovementStatus(a0: *mut u8) -> u8;
    fn ObjectEventClearHeldMovement(a0: *mut u8);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventGetLocalIdAndMap(a0: *mut u8, a1: *mut u8, a2: *mut u8, a3: *mut u8);
    fn ObjectEventIsMovementOverridden(a0: *mut u8) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8;
    fn OverrideTemplateCoordsForObjectEvent(a0: *mut u8);
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
    fn SetTrainerMovementType(a0: *mut u8, a1: u8);
    fn SetUpTwoTrainersBattle();
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreWordInTwoHalfwords(a0: *mut u16, a1: u32);
    fn SwitchTaskToFollowupFunc(a0: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn TryOverrideTemplateCoordsForObjectEvent(a0: *mut u8, a1: u8);
    fn UnfreezeObjectEvents();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForTrainersWantingBattle() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        ((&raw mut gNoOfApproachingTrainers)
            .cast::<u8>()
            .cast::<u8>())
        .write(0u8);
        ((&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>()).write(0u8);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    let mut numTrainers: u8 = 0u8;
                    if !((crate::c::bf_read(
                        (((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                        .wrapping_add(0),
                        0,
                        1,
                        false,
                    ) as u32)
                        != 0)
                    {
                        break 'l2;
                    }
                    if (((((((&raw mut gObjectEvents).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 36))
                    .wrapping_add(7))
                    .read()) as i32)
                        != 1i32)
                        && (((((((&raw mut gObjectEvents).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 36))
                        .wrapping_add(7))
                        .read()) as i32)
                            != 3i32)
                    {
                        break 'l2;
                    }
                    numTrainers = CheckTrainer(i);
                    if ((numTrainers) as i32) == 2i32 {
                        break 'l1;
                    }
                    if ((numTrainers) as i32) == 0i32 {
                        break 'l2;
                    }
                    if ((((&raw mut gNoOfApproachingTrainers)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32)
                        > 1i32
                    {
                        break 'l1;
                    }
                    if ((GetMonsStateToDoubles_2()) as i32) != 0i32 {
                        break 'l1;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        if ((((&raw mut gNoOfApproachingTrainers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 1i32
        {
            ResetTrainerOpponentIds();
            ConfigureAndSetUpOneTrainerBattle(
                ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gNoOfApproachingTrainers)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32)
                        .wrapping_sub(1i32)) as isize
                        * 12,
                ))
                .read(),
                (((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gNoOfApproachingTrainers)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32)
                        .wrapping_sub(1i32)) as isize
                        * 12,
                ))
                .wrapping_add(4)
                .cast::<*mut u8>())
                .read(),
            );
            ((&raw mut gTrainerApproachedPlayer)
                .cast::<u8>()
                .cast::<u8>())
            .write(1u8);
            return 1u8;
        } else {
            if ((((&raw mut gNoOfApproachingTrainers)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32)
                == 2i32
            {
                ResetTrainerOpponentIds();
                {
                    i = 0u8;
                    'l3: loop {
                        if !(((i) as i32)
                            < ((((&raw mut gNoOfApproachingTrainers)
                                .cast::<u8>()
                                .cast::<u8>())
                            .read()) as i32))
                        {
                            break 'l3;
                        }
                        'l4: {
                            ConfigureTwoTrainersBattle(
                                ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .read(),
                                (((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 12))
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                                .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                        let __p1 = (&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>();
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                SetUpTwoTrainersBattle();
                ((&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>()).write(0u8);
                ((&raw mut gTrainerApproachedPlayer)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(1u8);
                return 1u8;
            } else {
                ((&raw mut gTrainerApproachedPlayer)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(0u8);
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CheckTrainer(objectEventId: u8) -> u8 {
    unsafe {
        let mut objectEventId = objectEventId;
        let mut scriptPtr: *mut u8 = core::ptr::null_mut();
        let mut numTrainers: u8 = 1u8;
        let mut approachDistance: u8 = 0u8;
        if InTrainerHill() == 1u32 {
            scriptPtr = GetTrainerHillTrainerScript();
        } else {
            scriptPtr = GetObjectEventScriptPointerByObjectEventId(objectEventId);
        }
        if ((CurrentBattlePyramidLocation()) as i32) != 0i32 {
            if (GetBattlePyramidTrainerFlag(objectEventId)) != 0 {
                return 0u8;
            }
        } else {
            if InTrainerHill() == 1u32 {
                if (GetHillTrainerFlag(objectEventId)) != 0 {
                    return 0u8;
                }
            } else {
                if (GetTrainerFlagFromScriptPointer(scriptPtr)) != 0 {
                    return 0u8;
                }
            }
        }
        approachDistance = GetTrainerApproachDistance(
            ((&raw mut gObjectEvents).cast::<u8>())
                .wrapping_offset(((objectEventId) as i32) as isize * 36),
        );
        if ((approachDistance) as i32) != 0i32 {
            if ((((((scriptPtr).wrapping_offset(1)).read()) as i32) == 4i32)
                || (((((scriptPtr).wrapping_offset(1)).read()) as i32) == 7i32))
                || (((((scriptPtr).wrapping_offset(1)).read()) as i32) == 6i32)
            {
                if ((GetMonsStateToDoubles_2()) as i32) != 0i32 {
                    return 0u8;
                }
                numTrainers = 2u8;
            }
            ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gNoOfApproachingTrainers)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 12,
            ))
            .write(objectEventId);
            (((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gNoOfApproachingTrainers)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(4)
            .cast::<*mut u8>())
            .write(scriptPtr);
            (((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gNoOfApproachingTrainers)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(1))
            .write(approachDistance);
            InitTrainerApproachTask(
                ((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objectEventId) as i32) as isize * 36),
                ((((approachDistance) as i32).wrapping_sub(1i32)) as u8),
            );
            let __p1 = (&raw mut gNoOfApproachingTrainers)
                .cast::<u8>()
                .cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            return numTrainers;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistance(trainerObj: *mut u8) -> u8 {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut i: u8 = 0u8;
        let mut approachDistance: u8 = 0u8;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        if ((((trainerObj).wrapping_add(7)).read()) as i32) == 1i32 {
            approachDistance = (((((&raw const sDirectionalApproachDistanceFuncs)
                .cast::<u8>()
                .cast_mut()
                .cast::<Option<unsafe extern "C" fn(*mut u8, i16, i16, i16) -> u8>>())
            .cast::<Option<unsafe extern "C" fn(*mut u8, i16, i16, i16) -> u8>>())
            .wrapping_offset(
                (((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as i32)
                    .wrapping_sub(1i32)) as isize,
            ))
            .read())
            .unwrap_unchecked()(
                trainerObj,
                ((((trainerObj).wrapping_add(29)).read()) as i16),
                x,
                y,
            );
            return CheckPathBetweenTrainerAndPlayer(
                trainerObj,
                approachDistance,
                ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as u8),
            );
        } else {
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(16u32, 4u32)) {
                        break 'l1;
                    }
                    'l2: {
                        approachDistance = (((((&raw const sDirectionalApproachDistanceFuncs)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<Option<unsafe extern "C" fn(*mut u8, i16, i16, i16) -> u8>>(
                            ))
                        .cast::<Option<unsafe extern "C" fn(*mut u8, i16, i16, i16) -> u8>>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read())
                        .unwrap_unchecked()(
                            trainerObj,
                            ((((trainerObj).wrapping_add(29)).read()) as i16),
                            x,
                            y,
                        );
                        if (CheckPathBetweenTrainerAndPlayer(
                            trainerObj,
                            approachDistance,
                            ((((i) as i32).wrapping_add(1i32)) as u8),
                        )) != 0
                        {
                            return approachDistance;
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceSouth(
    trainerObj: *mut u8,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut range = range;
        let mut x = x;
        let mut y = y;
        if (((((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32) == ((x) as i32))
            && (((y) as i32)
                > (((((trainerObj).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)))
            && (((y) as i32)
                <= (((((trainerObj).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(((range) as i32)))
        {
            return ((((y) as i32).wrapping_sub(
                (((((trainerObj).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32),
            )) as u8);
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceNorth(
    trainerObj: *mut u8,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut range = range;
        let mut x = x;
        let mut y = y;
        if (((((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32) == ((x) as i32))
            && (((y) as i32)
                < (((((trainerObj).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)))
            && (((y) as i32)
                >= (((((trainerObj).wrapping_add(16))
                    .wrapping_add(2)
                    .cast::<i16>())
                .read()) as i32)
                    .wrapping_sub(((range) as i32)))
        {
            return (((((((trainerObj).wrapping_add(16))
                .wrapping_add(2)
                .cast::<i16>())
            .read()) as i32)
                .wrapping_sub(((y) as i32))) as u8);
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceWest(
    trainerObj: *mut u8,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut range = range;
        let mut x = x;
        let mut y = y;
        if (((((((trainerObj).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            == ((y) as i32))
            && (((x) as i32) < (((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32)))
            && (((x) as i32)
                >= (((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32)
                    .wrapping_sub(((range) as i32)))
        {
            return (((((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32)
                .wrapping_sub(((x) as i32))) as u8);
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn GetTrainerApproachDistanceEast(
    trainerObj: *mut u8,
    range: i16,
    x: i16,
    y: i16,
) -> u8 {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut range = range;
        let mut x = x;
        let mut y = y;
        if (((((((trainerObj).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            == ((y) as i32))
            && (((x) as i32) > (((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32)))
            && (((x) as i32)
                <= (((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32)
                    .wrapping_add(((range) as i32)))
        {
            return ((((x) as i32)
                .wrapping_sub((((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32)))
                as u8);
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn CheckPathBetweenTrainerAndPlayer(
    trainerObj: *mut u8,
    approachDistance: u8,
    direction: u8,
) -> u8 {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut approachDistance = approachDistance;
        let mut direction = direction;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut rangeX: u8 = 0u8;
        let mut rangeY: u8 = 0u8;
        let mut i: u8 = 0u8;
        let mut collision: u8 = 0u8;
        if ((approachDistance) as i32) == 0i32 {
            return 0u8;
        }
        x = (((trainerObj).wrapping_add(16)).cast::<i16>()).read();
        y = (((trainerObj).wrapping_add(16))
            .wrapping_add(2)
            .cast::<i16>())
        .read();
        MoveCoords(direction, &raw mut x, &raw mut y);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((approachDistance) as i32).wrapping_sub(1i32)) {
                    break 'l1;
                }
                'l2: {
                    collision = GetCollisionFlagsAtCoords(trainerObj, x, y, direction);
                    if (((collision) as i32) != 0i32) && ((((collision) as i32) & (-2i32)) != 0) {
                        return 0u8;
                    }
                }
                i = (i).wrapping_add(1);
                MoveCoords(direction, &raw mut x, &raw mut y);
            }
        }
        rangeX =
            (crate::c::bf_read(((trainerObj).wrapping_add(25)).wrapping_add(0), 0, 4, false) as u8);
        rangeY =
            (crate::c::bf_read(((trainerObj).wrapping_add(25)).wrapping_add(0), 4, 4, false) as u8);
        crate::c::bf_write(
            ((trainerObj).wrapping_add(25)).wrapping_add(0),
            0,
            4,
            (0u8) as i32,
        );
        crate::c::bf_write(
            ((trainerObj).wrapping_add(25)).wrapping_add(0),
            4,
            4,
            (0u8) as i32,
        );
        collision = GetCollisionAtCoords(trainerObj, x, y, ((direction) as u32));
        crate::c::bf_write(
            ((trainerObj).wrapping_add(25)).wrapping_add(0),
            0,
            4,
            (rangeX) as i32,
        );
        crate::c::bf_write(
            ((trainerObj).wrapping_add(25)).wrapping_add(0),
            4,
            4,
            (rangeY) as i32,
        );
        if ((collision) as i32) == 4i32 {
            return approachDistance;
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn InitTrainerApproachTask(trainerObj: *mut u8, range: u8) {
    unsafe {
        let mut trainerObj = trainerObj;
        let mut range = range;
        let mut task: *mut u8 = core::ptr::null_mut();
        (((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
            ((((&raw mut gNoOfApproachingTrainers)
                .cast::<u8>()
                .cast::<u8>())
            .read()) as i32) as isize
                * 12,
        ))
        .wrapping_add(8))
        .write(CreateTask(Some(Task_RunTrainerSeeFuncList), 80u8));
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gNoOfApproachingTrainers)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 12,
            ))
            .wrapping_add(8))
            .read()) as i32) as isize
                * 40,
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(((range) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                ((((&raw mut gNoOfApproachingTrainers)
                    .cast::<u8>()
                    .cast::<u8>())
                .read()) as i32) as isize
                    * 12,
            ))
            .read()) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn StartTrainerApproach(
    followupFunc: Option<unsafe extern "C" fn(u8)>,
) {
    unsafe {
        let mut followupFunc = followupFunc;
        let mut taskId: u8 = 0u8;
        let mut taskFunc: Option<unsafe extern "C" fn(u8)> = None;
        if ((((&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            taskId = ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                .wrapping_add(8))
            .read();
        } else {
            taskId = (((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                .wrapping_offset(12))
            .wrapping_add(8))
            .read();
        }
        taskFunc = Some(Task_RunTrainerSeeFuncList);
        SetTaskFuncWithFollowupFunc(taskId, taskFunc, followupFunc);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(1i16);
        (taskFunc).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn Task_RunTrainerSeeFuncList(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut trainerObj: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                as isize
                * 36,
        );
        if !((crate::c::bf_read((trainerObj).wrapping_add(0), 0, 1, false) as u32) != 0) {
            SwitchTaskToFollowupFunc(taskId);
        } else {
            'l1: loop {
                if !(((((((&raw const sTrainerSeeFuncList)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<Option<unsafe extern "C" fn(u8, *mut u8, *mut u8) -> u8>>())
                .cast::<Option<unsafe extern "C" fn(u8, *mut u8, *mut u8) -> u8>>())
                .wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize,
                ))
                .read())
                .unwrap_unchecked()(taskId, task, trainerObj))
                    != 0)
                {
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerSeeIdle(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn TrainerExclamationMark(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        let mut direction: u8 = 0u8;
        ObjectEventGetLocalIdAndMap(
            trainerObj,
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).cast::<u8>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .cast::<u8>(),
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .cast::<u8>(),
        );
        FieldEffectStart(0u8);
        direction = GetFaceDirectionMovementAction(
            ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as u32),
        );
        ObjectEventSetHeldMovement(trainerObj, direction);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 1u8;
    }
}
pub(crate) unsafe extern "C" fn WaitTrainerExclamationMark(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if (FieldEffectActiveListContains(0u8)) != 0 {
            return 0u8;
        } else {
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if (((((trainerObj).wrapping_add(6)).read()) as i32) == 57i32)
                || (((((trainerObj).wrapping_add(6)).read()) as i32) == 58i32)
            {
                (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
            }
            if ((((trainerObj).wrapping_add(6)).read()) as i32) == 63i32 {
                (((task).wrapping_add(8)).cast::<i16>()).write(8i16);
            }
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn TrainerMoveToPlayer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if (!((ObjectEventIsMovementOverridden(trainerObj)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(trainerObj)) != 0)
        {
            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
                ObjectEventSetHeldMovement(
                    trainerObj,
                    GetWalkNormalMovementAction(
                        ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16)
                            as u32),
                    ),
                );
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            } else {
                ObjectEventSetHeldMovement(trainerObj, 62u8);
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PlayerFaceApproachingTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        let mut playerObj: *mut u8 = core::ptr::null_mut();
        if ((ObjectEventIsMovementOverridden(trainerObj)) != 0)
            && (!((ObjectEventClearHeldMovementIfFinished(trainerObj)) != 0))
        {
            return 0u8;
        }
        SetTrainerMovementType(
            trainerObj,
            GetTrainerFacingDirectionMovementType(
                ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as u8),
            ),
        );
        TryOverrideTemplateCoordsForObjectEvent(
            trainerObj,
            GetTrainerFacingDirectionMovementType(
                ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as u8),
            ),
        );
        OverrideTemplateCoordsForObjectEvent(trainerObj);
        playerObj = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if ((ObjectEventIsMovementOverridden(playerObj)) != 0)
            && (!((ObjectEventClearHeldMovementIfFinished(playerObj)) != 0))
        {
            return 0u8;
        }
        CancelPlayerForcedMovement();
        ObjectEventSetHeldMovement(
            ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            ),
            GetFaceDirectionMovementAction(
                ((GetOppositeDirection(
                    ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as u8),
                )) as u32),
            ),
        );
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitPlayerFaceApproachingTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        let mut playerObj: *mut u8 = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
            (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32) as isize
                * 36,
        );
        if (!((ObjectEventIsMovementOverridden(playerObj)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(playerObj)) != 0)
        {
            SwitchTaskToFollowupFunc(taskId);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn RevealDisguisedTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if (!((ObjectEventIsMovementOverridden(trainerObj)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(trainerObj)) != 0)
        {
            ObjectEventSetHeldMovement(trainerObj, 89u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitRevealDisguisedTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if (ObjectEventClearHeldMovementIfFinished(trainerObj)) != 0 {
            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn RevealBuriedTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if (!((ObjectEventIsMovementOverridden(trainerObj)) != 0))
            || ((ObjectEventClearHeldMovementIfFinished(trainerObj)) != 0)
        {
            ObjectEventSetHeldMovement(trainerObj, 62u8);
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn PopOutOfAshBuriedTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if (ObjectEventCheckHeldMovementStatus(trainerObj)) != 0 {
            (((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>())
                .write((((((trainerObj).wrapping_add(16)).cast::<i16>()).read()) as i32));
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .write(
                    (((((trainerObj).wrapping_add(16))
                        .wrapping_add(2)
                        .cast::<i16>())
                    .read()) as i32),
                );
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .write(
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((trainerObj).wrapping_add(4)).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(67))
                    .read()) as i32)
                        .wrapping_sub(1i32),
                );
            ((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(3))
                .write(2i32);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                .write(((FieldEffectStart(49u8)) as i16));
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn JumpInPlaceBuriedTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(43))
        .read()) as i32)
            == 2i32
        {
            crate::c::bf_write((trainerObj).wrapping_add(3), 2, 1, (0u32) as i32);
            crate::c::bf_write((trainerObj).wrapping_add(0), 2, 1, (1u32) as i32);
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((((trainerObj).wrapping_add(4)).read()) as i32) as isize * 68);
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
            ObjectEventClearHeldMovementIfFinished(trainerObj);
            ObjectEventSetHeldMovement(
                trainerObj,
                GetJumpInPlaceMovementAction(
                    ((crate::c::bf_read((trainerObj).wrapping_add(24), 0, 4, false) as u16) as u32),
                ),
            );
            let __p1 = ((task).wrapping_add(8)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn WaitRevealBuriedTrainer(
    taskId: u8,
    task: *mut u8,
    trainerObj: *mut u8,
) -> u8 {
    unsafe {
        let mut taskId = taskId;
        let mut task = task;
        let mut trainerObj = trainerObj;
        if !((FieldEffectActiveListContains(49u8)) != 0) {
            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_SetBuriedTrainerMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut objEvent: *mut u8 = core::ptr::null_mut();
        LoadWordFromTwoHalfwords(
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).cast::<u16>(),
            (&raw mut objEvent).cast::<u32>(),
        );
        if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) != 0) {
            ObjectEventClearHeldMovement(objEvent);
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        (((((&raw const sTrainerSeeFuncList2)
            .cast::<u8>()
            .cast_mut()
            .cast::<Option<unsafe extern "C" fn(u8, *mut u8, *mut u8) -> u8>>())
        .cast::<Option<unsafe extern "C" fn(u8, *mut u8, *mut u8) -> u8>>())
        .wrapping_offset((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize))
        .read())
        .unwrap_unchecked()(taskId, task, objEvent);
        if ((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)
            == ((crate::c::div_u32(16u32, 4u32)) as i32).wrapping_sub(1i32))
            && (!((FieldEffectActiveListContains(49u8)) != 0))
        {
            SetTrainerMovementType(
                objEvent,
                GetTrainerFacingDirectionMovementType(
                    ((crate::c::bf_read((objEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
                ),
            );
            TryOverrideTemplateCoordsForObjectEvent(
                objEvent,
                GetTrainerFacingDirectionMovementType(
                    ((crate::c::bf_read((objEvent).wrapping_add(24), 0, 4, false) as u16) as u8),
                ),
            );
            DestroyTask(taskId);
        } else {
            crate::c::bf_write((objEvent).wrapping_add(0), 7, 1, (0u32) as i32);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetBuriedTrainerMovement(objEvent: *mut u8) {
    unsafe {
        let mut objEvent = objEvent;
        StoreWordInTwoHalfwords(
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((CreateTask(Some(Task_SetBuriedTrainerMovement), 0u8)) as i32) as isize * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .cast::<u16>(),
            ((objEvent) as usize as u32),
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoTrainerApproach() {
    unsafe {
        StartTrainerApproach(Some(Task_EndTrainerApproach));
    }
}
pub(crate) unsafe extern "C" fn Task_EndTrainerApproach(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        DestroyTask(taskId);
        ScriptContext_Enable();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryPrepareSecondApproachingTrainer() {
    unsafe {
        if ((((&raw mut gNoOfApproachingTrainers)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 2i32
        {
            if ((((&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>()).read()) as i32)
                == 0i32
            {
                let __p1 = (&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(1u16);
                UnfreezeObjectEvents();
                FreezeObjectEventsExceptOne(
                    ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(12))
                    .read(),
                );
            } else {
                ((&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>()).write(0u8);
                ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
            }
        } else {
            ((&raw mut gSpecialVar_Result).cast::<u16>()).write(0u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_ExclamationMarkIcon() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_ExclamationQuestionMark)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            83u8,
        );
        if ((spriteId) as i32) != 64i32 {
            SetIconSpriteData(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                0u16,
                0u8,
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_QuestionMarkIcon() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_ExclamationQuestionMark)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            82u8,
        );
        if ((spriteId) as i32) != 64i32 {
            SetIconSpriteData(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                33u16,
                1u8,
            );
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FldEff_HeartIcon() -> u8 {
    unsafe {
        let mut spriteId: u8 = CreateSpriteAtEnd(
            (&raw const sSpriteTemplate_HeartIcon)
                .cast::<u8>()
                .cast_mut(),
            0i16,
            0i16,
            82u8,
        );
        if ((spriteId) as i32) != 64i32 {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            SetIconSpriteData(sprite, 46u16, 0u8);
            crate::c::bf_write((sprite).wrapping_add(5), 4, 4, (2u16) as i32);
        }
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn SetIconSpriteData(
    sprite: *mut u8,
    fldEffId: u16,
    spriteAnimNum: u8,
) {
    unsafe {
        let mut sprite = sprite;
        let mut fldEffId = fldEffId;
        let mut spriteAnimNum = spriteAnimNum;
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).read()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(1))
                .read()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((&raw mut gFieldEffectArguments).cast::<i32>()).cast::<i32>()).wrapping_offset(2))
                .read()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-5i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(((fldEffId) as i16));
        StartSpriteAnim(sprite, spriteAnimNum);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TrainerIcons(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut objEventId: u8 = 0u8;
        if ((TryGetObjectEventIdByLocalIdAndMap(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            &raw mut objEventId,
        )) != 0)
            || ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
        {
            FieldEffectStop(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
            );
        } else {
            let mut objEventSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gObjectEvents).cast::<u8>())
                    .wrapping_offset(((objEventId) as i32) as isize * 36))
                .wrapping_add(4))
                .read()) as i32) as isize
                    * 68,
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((objEventSprite).wrapping_add(32).cast::<i16>()).read());
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((objEventSprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_sub(16i32)) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>())
                .write(((objEventSprite).wrapping_add(36).cast::<i16>()).read());
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((objEventSprite).wrapping_add(38).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) != 0 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p2).write(((__p2).read()).wrapping_add(1));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetCurrentApproachingTrainerObjectEventId() -> u8 {
    unsafe {
        if ((((&raw mut gApproachingTrainerId).cast::<u8>().cast::<u8>()).read()) as i32) == 0i32 {
            return (((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).read();
        } else {
            return ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                .wrapping_offset(12))
            .read();
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetChosenApproachingTrainerObjectEventId(arrayId: u8) -> u8 {
    unsafe {
        let mut arrayId = arrayId;
        if ((arrayId) as u32) >= crate::c::div_u32(24u32, 12u32) {
            return 0u8;
        } else {
            if ((arrayId) as i32) == 0i32 {
                return (((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).read();
            } else {
                return ((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(12))
                .read();
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn PlayerFaceTrainerAfterBattle() {
    unsafe {
        let mut objEvent: *mut u8 = core::ptr::null_mut();
        if ((((&raw mut gTrainerApproachedPlayer)
            .cast::<u8>()
            .cast::<u8>())
        .read()) as i32)
            == 1i32
        {
            objEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                ((((((&raw mut gApproachingTrainers).cast::<u8>()).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gWhichTrainerToFaceAfterBattle)
                        .cast::<u8>()
                        .cast::<u16>())
                    .read()) as i32) as isize
                        * 12,
                ))
                .read()) as i32) as isize
                    * 36,
            );
            (((&raw mut gPostBattleMovementScript).cast::<u8>()).cast::<u8>()).write(
                GetFaceDirectionMovementAction(
                    ((GetOppositeDirection(
                        ((crate::c::bf_read((objEvent).wrapping_add(24), 0, 4, false) as u16)
                            as u8),
                    )) as u32),
                ),
            );
            ((((&raw mut gPostBattleMovementScript).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .write(254u8);
            ScriptMovement_StartObjectMovementScript(
                255u8,
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as u8),
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as u8),
                ((&raw mut gPostBattleMovementScript).cast::<u8>()).cast::<u8>(),
            );
        } else {
            objEvent = ((&raw mut gObjectEvents).cast::<u8>()).wrapping_offset(
                (((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(5)).read()) as i32)
                    as isize
                    * 36,
            );
            (((&raw mut gPostBattleMovementScript).cast::<u8>()).cast::<u8>()).write(
                GetFaceDirectionMovementAction(
                    ((crate::c::bf_read((objEvent).wrapping_add(24), 0, 4, false) as u16) as u32),
                ),
            );
            ((((&raw mut gPostBattleMovementScript).cast::<u8>()).cast::<u8>()).wrapping_offset(1))
                .write(254u8);
            ScriptMovement_StartObjectMovementScript(
                255u8,
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .wrapping_add(1)
                    .cast::<i8>())
                .read()) as u8),
                (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                    .cast::<i8>())
                .read()) as u8),
                ((&raw mut gPostBattleMovementScript).cast::<u8>()).cast::<u8>(),
            );
        }
        SetMovingNpcId(255u16);
    }
}
