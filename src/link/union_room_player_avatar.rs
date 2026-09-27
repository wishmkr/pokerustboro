//! Translated from `src/union_room_player_avatar.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnionRoomObjGfxIds sUnionRoomPlayerCoords sUnionRoomGroupOffsets sOppositeFacingDirection sMemberFacingDirections sUnionRoomLocalIds sHidePlayerFlags sMovement_UnionPlayerExit sMovement_UnionPlayerEnter
#[allow(unused_imports)]
use crate::data::union_room_player_avatar::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionObjWork: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionObjRefreshTimer: u32 = 0u32;

unsafe extern "C" {
    static mut gObjectEvents: u8;
    static mut gPlayerAvatar: u8;
    static mut gSaveBlock1Ptr: u8;
    static mut gSprites: u8;
    fn ArePlayerFieldControlsLocked() -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateVirtualObject(a0: u8, a1: u8, a2: i16, a3: i16, a4: u8, a5: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FreezeObjectEvent(a0: *mut u8) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn IsVirtualObjectAnimating(a0: u8) -> u32;
    fn IsVirtualObjectInvisible(a0: u8) -> u32;
    fn MapGridSetMetatileImpassabilityAt(a0: i32, a1: i32, a2: u32);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut u8) -> u8;
    fn ObjectEventIsMovementOverridden(a0: *mut u8) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut u8, a1: u8) -> u8;
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn SetVirtualObjectGraphics(a0: u8, a1: u8);
    fn SetVirtualObjectInvisibility(a0: u8, a1: u32);
    fn SetVirtualObjectSpriteAnim(a0: u8, a1: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn TrySpawnObjectEvent(a0: u8, a1: u8, a2: u8) -> u8;
    fn TurnVirtualObject(a0: u8, a1: u8);
    fn UnfreezeObjectEvent(a0: *mut u8);
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn player_get_pos_including_state_based_drift(a0: *mut i16, a1: *mut i16) -> u8;
}

pub(crate) unsafe extern "C" fn IsPlayerStandingStill() -> u32 {
    unsafe {
        if ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 2i32)
            || ((((((&raw mut gPlayerAvatar).cast::<u8>()).wrapping_add(3)).read()) as i32) == 0i32)
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
pub(crate) unsafe extern "C" fn GetUnionRoomPlayerGraphicsId(gender: u32, id: u32) -> u8 {
    unsafe {
        let mut gender = gender;
        let mut id = id;
        return ((((((&raw const sUnionRoomObjGfxIds).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((gender) as i32) as isize * 10))
        .cast::<u8>())
        .wrapping_offset(((crate::c::rem_u32(id, 8u32)) as i32) as isize))
        .read();
    }
}
pub(crate) unsafe extern "C" fn GetUnionRoomPlayerCoords(
    leaderId: u32,
    memberId: u32,
    x: *mut i32,
    y: *mut i32,
) {
    unsafe {
        let mut leaderId = leaderId;
        let mut memberId = memberId;
        let mut x = x;
        let mut y = y;
        (x).write(
            ((((((((&raw const sUnionRoomPlayerCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((leaderId) as i32) as isize * 4))
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    (((((((&raw const sUnionRoomGroupOffsets).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((memberId) as i32) as isize * 2))
                    .cast::<i8>())
                    .read()) as i32),
                ))
            .wrapping_add(7i32),
        );
        (y).write(
            (((((((((&raw const sUnionRoomPlayerCoords).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((leaderId) as i32) as isize * 4))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_add(
                    ((((((((&raw const sUnionRoomGroupOffsets).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((memberId) as i32) as isize * 2))
                    .cast::<i8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                ))
            .wrapping_add(7i32),
        );
    }
}
pub(crate) unsafe extern "C" fn IsUnionRoomPlayerAt(
    leaderId: u32,
    memberId: u32,
    x: i32,
    y: i32,
) -> u32 {
    unsafe {
        let mut leaderId = leaderId;
        let mut memberId = memberId;
        let mut x = x;
        let mut y = y;
        if (((((((((&raw const sUnionRoomPlayerCoords).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((leaderId) as i32) as isize * 4))
        .cast::<i16>())
        .read()) as i32)
            .wrapping_add(
                (((((((&raw const sUnionRoomGroupOffsets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((memberId) as i32) as isize * 2))
                .cast::<i8>())
                .read()) as i32),
            ))
        .wrapping_add(7i32)
            == x)
            && ((((((((((&raw const sUnionRoomPlayerCoords).cast::<u8>().cast_mut())
                .cast::<u8>())
            .wrapping_offset(((leaderId) as i32) as isize * 4))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                .wrapping_add(
                    ((((((((&raw const sUnionRoomGroupOffsets).cast::<u8>().cast_mut())
                        .cast::<u8>())
                    .wrapping_offset(((memberId) as i32) as isize * 2))
                    .cast::<i8>())
                    .wrapping_offset(1))
                    .read()) as i32),
                ))
            .wrapping_add(7i32)
                == y)
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
pub(crate) unsafe extern "C" fn IsUnionRoomPlayerHidden(player_idx: u32) -> u32 {
    unsafe {
        let mut player_idx = player_idx;
        return ((FlagGet((((703u32).wrapping_add(player_idx)) as u16))) as u32);
    }
}
pub(crate) unsafe extern "C" fn HideUnionRoomPlayer(player_idx: u32) {
    unsafe {
        let mut player_idx = player_idx;
        FlagSet((((703u32).wrapping_add(player_idx)) as u16));
    }
}
pub(crate) unsafe extern "C" fn ShowUnionRoomPlayer(player_idx: u32) {
    unsafe {
        let mut player_idx = player_idx;
        FlagClear((((703u32).wrapping_add(player_idx)) as u16));
    }
}
pub(crate) unsafe extern "C" fn SetUnionRoomPlayerGfx(leaderId: u32, gfxId: u32) {
    unsafe {
        let mut leaderId = leaderId;
        let mut gfxId = gfxId;
        VarSet(
            (((16400u32).wrapping_add(leaderId)) as u16),
            ((gfxId) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn CreateUnionRoomPlayerObjectEvent(leaderId: u32) {
    unsafe {
        let mut leaderId = leaderId;
        TrySpawnObjectEvent(
            ((((&raw const sUnionRoomLocalIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((leaderId) as i32) as isize))
            .read(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn RemoveUnionRoomPlayerObjectEvent(leaderId: u32) {
    unsafe {
        let mut leaderId = leaderId;
        RemoveObjectEventByLocalIdAndMap(
            ((((&raw const sUnionRoomLocalIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((leaderId) as i32) as isize))
            .read(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn SetUnionRoomPlayerEnterExitMovement(
    leaderId: u32,
    movement: *mut u8,
) -> u32 {
    unsafe {
        let mut leaderId = leaderId;
        let mut movement = movement;
        let mut objectId: u8 = 0u8;
        let mut object: *mut u8 = core::ptr::null_mut();
        if (TryGetObjectEventIdByLocalIdAndMap(
            ((((&raw const sUnionRoomLocalIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((leaderId) as i32) as isize))
            .read(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            &raw mut objectId,
        )) != 0
        {
            return 0u32;
        }
        object = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectId) as i32) as isize * 36);
        if (ObjectEventIsMovementOverridden(object)) != 0 {
            return 0u32;
        }
        if (ObjectEventSetHeldMovement(object, (movement).read())) != 0 {
            return 0u32;
        }
        return 1u32;
    }
}
pub(crate) unsafe extern "C" fn TryReleaseUnionRoomPlayerObjectEvent(leaderId: u32) -> u32 {
    unsafe {
        let mut leaderId = leaderId;
        let mut objectId: u8 = 0u8;
        let mut object: *mut u8 = core::ptr::null_mut();
        if (TryGetObjectEventIdByLocalIdAndMap(
            ((((&raw const sUnionRoomLocalIds).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((leaderId) as i32) as isize))
            .read(),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as u8),
            (((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .cast::<i8>())
            .read()) as u8),
            &raw mut objectId,
        )) != 0
        {
            return 1u32;
        }
        object = ((&raw mut gObjectEvents).cast::<u8>())
            .wrapping_offset(((objectId) as i32) as isize * 36);
        if !((ObjectEventClearHeldMovementIfFinished(object)) != 0) {
            return 0u32;
        }
        if !((ArePlayerFieldControlsLocked()) != 0) {
            UnfreezeObjectEvent(object);
        } else {
            FreezeObjectEvent(object);
        }
        return 1u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitUnionRoomPlayerObjects(players: *mut u8) -> u8 {
    unsafe {
        let mut players = players;
        let mut i: i32 = 0i32;
        ((&raw mut sUnionObjRefreshTimer).cast::<u8>().cast::<u32>()).write(0u32);
        ((&raw mut sUnionObjWork).cast::<u8>().cast::<*mut u8>()).write(players);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((players).wrapping_offset((i) as isize * 4)).write(0u8);
                    (((players).wrapping_offset((i) as isize * 4)).wrapping_add(1)).write(0u8);
                    (((players).wrapping_offset((i) as isize * 4))
                        .wrapping_add(2)
                        .cast::<i8>())
                    .write(0i8);
                    (((players).wrapping_offset((i) as isize * 4)).wrapping_add(3)).write(0u8);
                }
                i = (i).wrapping_add(1);
            }
        }
        return CreateTask_AnimateUnionRoomPlayers();
    }
}
pub(crate) unsafe extern "C" fn AnimateUnionRoomPlayerDespawn(
    state: *mut i8,
    leaderId: u32,
    object: *mut u8,
) -> u32 {
    unsafe {
        let mut state = state;
        let mut leaderId = leaderId;
        let mut object = object;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            if __sw1 == 0i32 {
                if SetUnionRoomPlayerEnterExitMovement(
                    leaderId,
                    ((&raw const sMovement_UnionPlayerExit)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                ) == 1u32
                {
                    HideUnionRoomPlayer(leaderId);
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (TryReleaseUnionRoomPlayerObjectEvent(leaderId)) != 0 {
                    RemoveUnionRoomPlayerObjectEvent(leaderId);
                    HideUnionRoomPlayer(leaderId);
                    (state).write(0i8);
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn AnimateUnionRoomPlayerSpawn(
    state: *mut i8,
    leaderId: u32,
    object: *mut u8,
) -> u32 {
    unsafe {
        let mut state = state;
        let mut leaderId = leaderId;
        let mut object = object;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        'l1: {
            let __sw1 = (((state).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if !((IsPlayerStandingStill()) != 0) {
                    break 'l1;
                }
                PlayerGetDestCoords(&raw mut x, &raw mut y);
                if IsUnionRoomPlayerAt(leaderId, 0u32, ((x) as i32), ((y) as i32)) == 1u32 {
                    break 'l1;
                }
                player_get_pos_including_state_based_drift(&raw mut x, &raw mut y);
                if IsUnionRoomPlayerAt(leaderId, 0u32, ((x) as i32), ((y) as i32)) == 1u32 {
                    break 'l1;
                }
                SetUnionRoomPlayerGfx(leaderId, ((((object).wrapping_add(1)).read()) as u32));
                CreateUnionRoomPlayerObjectEvent(leaderId);
                ShowUnionRoomPlayer(leaderId);
                (state).write(((state).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if SetUnionRoomPlayerEnterExitMovement(
                    leaderId,
                    ((&raw const sMovement_UnionPlayerEnter)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                ) == 1u32
                {
                    (state).write(((state).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (TryReleaseUnionRoomPlayerObjectEvent(leaderId)) != 0 {
                    (state).write(0i8);
                    return 1u32;
                }
                break 'l1;
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SpawnGroupLeader(leaderId: u32, gender: u32, id: u32) -> u32 {
    unsafe {
        let mut leaderId = leaderId;
        let mut gender = gender;
        let mut id = id;
        let mut object: *mut u8 = (((&raw mut sUnionObjWork).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((leaderId) as i32) as isize * 4);
        ((object).wrapping_add(3)).write(1u8);
        ((object).wrapping_add(1)).write(GetUnionRoomPlayerGraphicsId(gender, id));
        if (((object).read()) as i32) == 0i32 {
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
pub(crate) unsafe extern "C" fn DespawnGroupLeader(leaderId: u32) -> u32 {
    unsafe {
        let mut leaderId = leaderId;
        let mut object: *mut u8 = (((&raw mut sUnionObjWork).cast::<u8>().cast::<*mut u8>())
            .read())
        .wrapping_offset(((leaderId) as i32) as isize * 4);
        ((object).wrapping_add(3)).write(2u8);
        if (((object).read()) as i32) == 1i32 {
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
pub(crate) unsafe extern "C" fn AnimateUnionRoomPlayer(leaderId: u32, object: *mut u8) {
    unsafe {
        let mut leaderId = leaderId;
        let mut object = object;
        'l1: {
            let __sw1 = (((object).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if ((((object).wrapping_add(3)).read()) as i32) == 1i32 {
                    (object).write(2u8);
                    ((object).wrapping_add(2).cast::<i8>()).write(0i8);
                } else {
                    break 'l1;
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                if (!((IsUnionRoomPlayerInvisible(leaderId, 0u32)) != 0))
                    && (((((object).wrapping_add(3)).read()) as i32) == 2i32)
                {
                    (object).write(0u8);
                    ((object).wrapping_add(2).cast::<i8>()).write(0i8);
                    RemoveUnionRoomPlayerObjectEvent(leaderId);
                    HideUnionRoomPlayer(leaderId);
                } else {
                    if AnimateUnionRoomPlayerSpawn(
                        (object).wrapping_add(2).cast::<i8>(),
                        leaderId,
                        object,
                    ) == 1u32
                    {
                        (object).write(1u8);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if ((((object).wrapping_add(3)).read()) as i32) != 2i32 {
                    break 'l1;
                }
                (object).write(3u8);
                ((object).wrapping_add(2).cast::<i8>()).write(0i8);
            }
            if __fall || __sw1 == 3i32 {
                __fall = true;
                if AnimateUnionRoomPlayerDespawn(
                    (object).wrapping_add(2).cast::<i8>(),
                    leaderId,
                    object,
                ) == 1u32
                {
                    (object).write(0u8);
                }
                break 'l1;
            }
        }
        ((object).wrapping_add(3)).write(0u8);
    }
}
pub(crate) unsafe extern "C" fn Task_AnimateUnionRoomPlayers(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    AnimateUnionRoomPlayer(
                        ((i) as u32),
                        (((&raw mut sUnionObjWork).cast::<u8>().cast::<*mut u8>()).read())
                            .wrapping_offset((i) as isize * 4),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateTask_AnimateUnionRoomPlayers() -> u8 {
    unsafe {
        if ((FuncIsActiveTask(Some(Task_AnimateUnionRoomPlayers))) as i32) == 1i32 {
            return 16u8;
        } else {
            return CreateTask(Some(Task_AnimateUnionRoomPlayers), 5u8);
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn DestroyTask_AnimateUnionRoomPlayers() {
    unsafe {
        let mut taskId: u8 = FindTaskIdByFunc(Some(Task_AnimateUnionRoomPlayers));
        if ((taskId) as i32) < 16i32 {
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyUnionRoomPlayerObjects() {
    unsafe {
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if !((IsUnionRoomPlayerHidden(((i) as u32))) != 0) {
                        RemoveUnionRoomPlayerObjectEvent(((i) as u32));
                        HideUnionRoomPlayer(((i) as u32));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        ((&raw mut sUnionObjWork).cast::<u8>().cast::<*mut u8>()).write(core::ptr::null_mut());
        DestroyTask_AnimateUnionRoomPlayers();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateUnionRoomPlayerSprites(spriteIds: *mut u8, leaderId: i32) {
    unsafe {
        let mut spriteIds = spriteIds;
        let mut leaderId = leaderId;
        let mut memberId: i32 = 0i32;
        {
            memberId = 0i32;
            'l1: loop {
                if !(memberId < 5i32) {
                    break 'l1;
                }
                'l2: {
                    let mut id: i32 = ((5i32).wrapping_mul(leaderId)).wrapping_add(memberId);
                    ((spriteIds).wrapping_offset((id) as isize)).write(CreateVirtualObject(
                        65u8,
                        (((id).wrapping_sub(56i32)) as u8),
                        (((((((((&raw const sUnionRoomPlayerCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((leaderId) as isize * 4))
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_add(
                                (((((((&raw const sUnionRoomGroupOffsets)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((memberId) as isize * 2))
                                .cast::<i8>())
                                .read()) as i32),
                            )) as i16),
                        ((((((((((&raw const sUnionRoomPlayerCoords).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset((leaderId) as isize * 4))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(
                                ((((((((&raw const sUnionRoomGroupOffsets)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset((memberId) as isize * 2))
                                .cast::<i8>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            )) as i16),
                        3u8,
                        1u8,
                    ));
                    SetVirtualObjectInvisibility((((id).wrapping_sub(56i32)) as u8), 1u32);
                }
                memberId = (memberId).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyUnionRoomPlayerSprites(spriteIds: *mut u8) {
    unsafe {
        let mut spriteIds = spriteIds;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 40i32) {
                    break 'l1;
                }
                'l2: {
                    DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((spriteIds).wrapping_offset((i) as isize)).read()) as i32) as isize * 68,
                    ));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTilesAroundUnionRoomPlayersPassable() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut memberId: i32 = 0i32;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        memberId = 0i32;
                        'l3: loop {
                            if !(memberId < 5i32) {
                                break 'l3;
                            }
                            'l4: {
                                GetUnionRoomPlayerCoords(
                                    ((i) as u32),
                                    ((memberId) as u32),
                                    &raw mut x,
                                    &raw mut y,
                                );
                                MapGridSetMetatileImpassabilityAt(x, y, 0u32);
                            }
                            memberId = (memberId).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetNewFacingDirectionForUnionRoomPlayer(
    memberId: u32,
    leaderId: u32,
    gameData: *mut u8,
) -> u8 {
    unsafe {
        let mut memberId = memberId;
        let mut leaderId = leaderId;
        let mut gameData = gameData;
        if (memberId) != 0 {
            return ((((&raw const sMemberFacingDirections).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((memberId) as i32) as isize))
            .read();
        } else {
            if ((crate::c::bf_read((gameData).wrapping_add(10), 0, 7, false) as u8) as i32) == 69i32
            {
                return 1u8;
            } else {
                return 4u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn IsUnionRoomPlayerInvisible(leaderId: u32, memberId: u32) -> u32 {
    unsafe {
        let mut leaderId = leaderId;
        let mut memberId = memberId;
        return IsVirtualObjectInvisible(
            (((((5u32).wrapping_mul(leaderId)).wrapping_add(memberId)).wrapping_sub(56u32)) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn SpawnGroupMember(
    leaderId: u32,
    memberId: u32,
    graphicsId: u8,
    gameData: *mut u8,
) {
    unsafe {
        let mut leaderId = leaderId;
        let mut memberId = memberId;
        let mut graphicsId = graphicsId;
        let mut gameData = gameData;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        let mut id: i32 = ((((5u32).wrapping_mul(leaderId)).wrapping_add(memberId)) as i32);
        if IsUnionRoomPlayerInvisible(leaderId, memberId) == 1u32 {
            SetVirtualObjectInvisibility((((id).wrapping_sub(56i32)) as u8), 0u32);
            SetVirtualObjectSpriteAnim((((id).wrapping_sub(56i32)) as u8), 1u8);
        }
        SetVirtualObjectGraphics((((id).wrapping_sub(56i32)) as u8), graphicsId);
        SetUnionRoomObjectFacingDirection(
            ((memberId) as i32),
            ((leaderId) as i32),
            GetNewFacingDirectionForUnionRoomPlayer(memberId, leaderId, gameData),
        );
        GetUnionRoomPlayerCoords(leaderId, memberId, &raw mut x, &raw mut y);
        MapGridSetMetatileImpassabilityAt(x, y, 1u32);
    }
}
pub(crate) unsafe extern "C" fn DespawnGroupMember(leaderId: u32, memberId: u32) {
    unsafe {
        let mut leaderId = leaderId;
        let mut memberId = memberId;
        let mut x: i32 = 0i32;
        let mut y: i32 = 0i32;
        SetVirtualObjectSpriteAnim(
            (((((5u32).wrapping_mul(leaderId)).wrapping_add(memberId)).wrapping_sub(56u32)) as u8),
            2u8,
        );
        GetUnionRoomPlayerCoords(leaderId, memberId, &raw mut x, &raw mut y);
        MapGridSetMetatileImpassabilityAt(x, y, 0u32);
    }
}
pub(crate) unsafe extern "C" fn AssembleGroup(leaderId: u32, gameData: *mut u8) {
    unsafe {
        let mut leaderId = leaderId;
        let mut gameData = gameData;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut x2: i16 = 0i16;
        let mut y2: i16 = 0i16;
        let mut i: i32 = 0i32;
        PlayerGetDestCoords(&raw mut x, &raw mut y);
        player_get_pos_including_state_based_drift(&raw mut x2, &raw mut y2);
        if IsVirtualObjectInvisible(
            (((((5u32).wrapping_mul(leaderId)).wrapping_add(0u32)).wrapping_sub(56u32)) as u8),
        ) == 1u32
        {
            if (IsUnionRoomPlayerAt(leaderId, 0u32, ((x) as i32), ((y) as i32)) == 1u32)
                || (IsUnionRoomPlayerAt(leaderId, 0u32, ((x2) as i32), ((y2) as i32)) == 1u32)
            {
                return;
            }
            SpawnGroupMember(
                leaderId,
                0u32,
                GetUnionRoomPlayerGraphicsId(
                    ((crate::c::bf_read((gameData).wrapping_add(11), 0, 1, false) as u8) as u32),
                    (((((gameData).wrapping_add(2)).cast::<u8>()).read()) as u32),
                ),
                gameData,
            );
        }
        {
            i = 1i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    if ((((((gameData).wrapping_add(4)).cast::<u8>())
                        .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                    .read()) as i32)
                        == 0i32
                    {
                        DespawnGroupMember(leaderId, ((i) as u32));
                    } else {
                        if (IsUnionRoomPlayerAt(leaderId, ((i) as u32), ((x) as i32), ((y) as i32))
                            == 0u32)
                            && (IsUnionRoomPlayerAt(
                                leaderId,
                                ((i) as u32),
                                ((x2) as i32),
                                ((y2) as i32),
                            ) == 0u32)
                        {
                            SpawnGroupMember(
                                leaderId,
                                ((i) as u32),
                                GetUnionRoomPlayerGraphicsId(
                                    (((((((((gameData).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                                    .read()) as i32)
                                        >> 3)
                                        & 1i32) as u32),
                                    ((((((((gameData).wrapping_add(4)).cast::<u8>())
                                        .wrapping_offset(((i).wrapping_sub(1i32)) as isize))
                                    .read()) as i32)
                                        & 7i32) as u32),
                                ),
                                gameData,
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpawnGroupLeaderAndMembers(leaderId: u32, gameData: *mut u8) {
    unsafe {
        let mut leaderId = leaderId;
        let mut gameData = gameData;
        let mut i: u32 = 0u32;
        'l1: {
            let __sw1 =
                ((crate::c::bf_read((gameData).wrapping_add(10), 0, 7, false) as u8) as i32);
            if __sw1 == 64i32 || __sw1 == 84i32 {
                SpawnGroupLeader(
                    leaderId,
                    ((crate::c::bf_read((gameData).wrapping_add(11), 0, 1, false) as u8) as u32),
                    (((((gameData).wrapping_add(2)).cast::<u8>()).read()) as u32),
                );
                {
                    i = 0u32;
                    'l2: loop {
                        if !(i < 5u32) {
                            break 'l2;
                        }
                        'l3: {
                            DespawnGroupMember(leaderId, i);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 65i32
                || __sw1 == 68i32
                || __sw1 == 69i32
                || __sw1 == 72i32
                || __sw1 == 81i32
                || __sw1 == 82i32
                || __sw1 == 83i32
            {
                DespawnGroupLeader(leaderId);
                AssembleGroup(leaderId, gameData);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DespawnGroupLeaderAndMembers(leaderId: u32, gameData: *mut u8) {
    unsafe {
        let mut leaderId = leaderId;
        let mut gameData = gameData;
        let mut i: i32 = 0i32;
        DespawnGroupLeader(leaderId);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 5i32) {
                    break 'l1;
                }
                'l2: {
                    DespawnGroupMember(leaderId, ((i) as u32));
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateUnionRoomPlayerSprites(uroom: *mut u8) {
    unsafe {
        let mut uroom = uroom;
        let mut i: i32 = 0i32;
        let mut leaders: *mut u8 = core::ptr::null_mut();
        ((&raw mut sUnionObjRefreshTimer).cast::<u8>().cast::<u32>()).write(0u32);
        {
            i = 0i32;
            leaders = (((uroom).cast::<*mut u8>()).read()).cast::<u8>();
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    if ((crate::c::bf_read(
                        ((leaders).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                        0,
                        2,
                        false,
                    ) as u8) as i32)
                        == 1i32
                    {
                        SpawnGroupLeaderAndMembers(
                            ((i) as u32),
                            ((leaders).wrapping_offset((i) as isize * 32)),
                        );
                    } else {
                        if ((crate::c::bf_read(
                            ((leaders).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                            0,
                            2,
                            false,
                        ) as u8) as i32)
                            == 2i32
                        {
                            DespawnGroupLeaderAndMembers(
                                ((i) as u32),
                                ((leaders).wrapping_offset((i) as isize * 32)),
                            );
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScheduleUnionRoomPlayerRefresh(uroom: *mut u8) {
    unsafe {
        let mut uroom = uroom;
        ((&raw mut sUnionObjRefreshTimer).cast::<u8>().cast::<u32>()).write(300u32);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleUnionRoomPlayerRefresh(uroom: *mut u8) {
    unsafe {
        let mut uroom = uroom;
        if {
            let __p1 = (&raw mut sUnionObjRefreshTimer).cast::<u8>().cast::<u32>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        } > 300u32
        {
            UpdateUnionRoomPlayerSprites(uroom);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryInteractWithUnionRoomMember(
    list: *mut u8,
    memberIdPtr: *mut i16,
    leaderIdPtr: *mut i16,
    spriteIds: *mut u8,
) -> u32 {
    unsafe {
        let mut list = list;
        let mut memberIdPtr = memberIdPtr;
        let mut leaderIdPtr = leaderIdPtr;
        let mut spriteIds = spriteIds;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut i: i32 = 0i32;
        let mut memberId: i32 = 0i32;
        let mut leaders: *mut u8 = core::ptr::null_mut();
        if !((IsPlayerStandingStill()) != 0) {
            return 0u32;
        }
        GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
        {
            i = 0i32;
            leaders = (list).cast::<u8>();
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        memberId = 0i32;
                        'l3: loop {
                            if !(memberId < 5i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut id: i32 = ((5i32).wrapping_mul(i)).wrapping_add(memberId);
                                if ((x) as i32)
                                    != ((((((((&raw const sUnionRoomPlayerCoords)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .cast::<i16>())
                                    .read()) as i32)
                                        .wrapping_add(
                                            (((((((&raw const sUnionRoomGroupOffsets)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset((memberId) as isize * 2))
                                            .cast::<i8>())
                                            .read())
                                                as i32),
                                        ))
                                    .wrapping_add(7i32)
                                {
                                    break 'l4;
                                }
                                if ((y) as i32)
                                    != (((((((((&raw const sUnionRoomPlayerCoords)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset((i) as isize * 4))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_add(
                                            ((((((((&raw const sUnionRoomGroupOffsets)
                                                .cast::<u8>()
                                                .cast_mut())
                                            .cast::<u8>())
                                            .wrapping_offset((memberId) as isize * 2))
                                            .cast::<i8>())
                                            .wrapping_offset(1))
                                            .read())
                                                as i32),
                                        ))
                                    .wrapping_add(7i32)
                                {
                                    break 'l4;
                                }
                                if (IsVirtualObjectInvisible((((id).wrapping_sub(56i32)) as u8)))
                                    != 0
                                {
                                    break 'l4;
                                }
                                if (IsVirtualObjectAnimating((((id).wrapping_sub(56i32)) as u8)))
                                    != 0
                                {
                                    break 'l4;
                                }
                                if ((crate::c::bf_read(
                                    ((leaders).wrapping_offset((i) as isize * 32)).wrapping_add(26),
                                    0,
                                    2,
                                    false,
                                ) as u8) as i32)
                                    != 1i32
                                {
                                    break 'l4;
                                }
                                SetUnionRoomObjectFacingDirection(
                                    memberId,
                                    i,
                                    ((((&raw const sOppositeFacingDirection)
                                        .cast::<u8>()
                                        .cast_mut())
                                    .cast::<u8>())
                                    .wrapping_offset(
                                        ((GetPlayerFacingDirection()) as i32) as isize,
                                    ))
                                    .read(),
                                );
                                (memberIdPtr).write(((memberId) as i16));
                                (leaderIdPtr).write(((i) as i16));
                                return 1u32;
                            }
                            memberId = (memberId).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
pub(crate) unsafe extern "C" fn SetUnionRoomObjectFacingDirection(
    memberId: i32,
    leaderId: i32,
    newDirection: u8,
) {
    unsafe {
        let mut memberId = memberId;
        let mut leaderId = leaderId;
        let mut newDirection = newDirection;
        TurnVirtualObject(
            (((((5i32).wrapping_mul(leaderId)).wrapping_sub(56i32)).wrapping_add(memberId)) as u8),
            newDirection,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateUnionRoomMemberFacing(memberId: u32, leaderId: u32, list: *mut u8) {
    unsafe {
        let mut memberId = memberId;
        let mut leaderId = leaderId;
        let mut list = list;
        SetUnionRoomObjectFacingDirection(
            ((memberId) as i32),
            ((leaderId) as i32),
            GetNewFacingDirectionForUnionRoomPlayer(
                memberId,
                leaderId,
                (((list).cast::<u8>()).wrapping_offset(((leaderId) as i32) as isize * 32)),
            ),
        );
        return;
    }
}
