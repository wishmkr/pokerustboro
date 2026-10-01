//! Translated from `src/union_room_player_avatar.c` by tools/rustport/c2rs.py.
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
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagClear, FlagGet, FlagSet, VarSet};
use crate::event_object_movement::{
    CreateVirtualObject, FreezeObjectEvent, IsVirtualObjectAnimating, IsVirtualObjectInvisible,
    ObjectEventClearHeldMovementIfFinished, ObjectEventIsMovementOverridden,
    ObjectEventSetHeldMovement, RemoveObjectEventByLocalIdAndMap, SetVirtualObjectGraphics,
    SetVirtualObjectInvisibility, SetVirtualObjectSpriteAnim, TryGetObjectEventIdByLocalIdAndMap,
    TrySpawnObjectEvent, TurnVirtualObject, UnfreezeObjectEvent,
};
use crate::field_player_avatar::{
    GetPlayerFacingDirection, GetXYCoordsOneStepInFrontOfPlayer, PlayerGetDestCoords,
    gObjectEvents, gPlayerAvatar, player_get_pos_including_state_based_drift,
};
use crate::fieldmap::MapGridSetMetatileImpassabilityAt;
use crate::load_save::gSaveBlock1Ptr;
use crate::script::ArePlayerFieldControlsLocked;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
// Data tables (translate with cdata.py): sUnionRoomObjGfxIds sUnionRoomPlayerCoords sUnionRoomGroupOffsets sOppositeFacingDirection sMemberFacingDirections sUnionRoomLocalIds sHidePlayerFlags sMovement_UnionPlayerExit sMovement_UnionPlayerEnter

const UR_SPRITE_START_ID: u8 = 56;

static sMemberFacingDirections: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::union_room_player_avatar::sMemberFacingDirections).cast());
static sMovement_UnionPlayerEnter: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::union_room_player_avatar::sMovement_UnionPlayerEnter).cast());
static sMovement_UnionPlayerExit: Table<CArray<u8, 2>> =
    Table((&raw const crate::data::union_room_player_avatar::sMovement_UnionPlayerExit).cast());
static sOppositeFacingDirection: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::union_room_player_avatar::sOppositeFacingDirection).cast());
static sUnionRoomGroupOffsets: Table<CArray<CArray<i8, 2>, 5>> =
    Table((&raw const crate::data::union_room_player_avatar::sUnionRoomGroupOffsets).cast());
static sUnionRoomLocalIds: Table<CArray<u8, 8>> =
    Table((&raw const crate::data::union_room_player_avatar::sUnionRoomLocalIds).cast());
static sUnionRoomObjGfxIds: Table<CArray<CArray<u8, 10>, 2>> =
    Table((&raw const crate::data::union_room_player_avatar::sUnionRoomObjGfxIds).cast());
static sUnionRoomPlayerCoords: Table<CArray<CArray<i16, 2>, 8>> =
    Table((&raw const crate::data::union_room_player_avatar::sUnionRoomPlayerCoords).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnionObjWork: *mut UnionRoomObject = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sUnionObjRefreshTimer: crate::global::Global<u32> = crate::global::Global::new(0);

pub(crate) unsafe fn IsPlayerStandingStill() -> u32 {
    if gPlayerAvatar.tileTransitionState == T_TILE_CENTER
        || gPlayerAvatar.tileTransitionState == T_NOT_MOVING
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn GetUnionRoomPlayerGraphicsId(gender: u32, id: u32) -> u8 {
    sUnionRoomObjGfxIds[gender][id % 8]
}
unsafe fn GetUnionRoomPlayerCoords(leaderId: u32, memberId: u32, x: *mut i32, y: *mut i32) {
    *x = sUnionRoomPlayerCoords[leaderId][0] as i32
        + sUnionRoomGroupOffsets[memberId][0] as i32
        + MAP_OFFSET;
    *y = sUnionRoomPlayerCoords[leaderId][1] as i32
        + sUnionRoomGroupOffsets[memberId][1] as i32
        + MAP_OFFSET;
}
fn IsUnionRoomPlayerAt(leaderId: u32, memberId: u32, x: i32, y: i32) -> u32 {
    if sUnionRoomPlayerCoords[leaderId][0] as i32
        + sUnionRoomGroupOffsets[memberId][0] as i32
        + MAP_OFFSET
        == x
        && sUnionRoomPlayerCoords[leaderId][1] as i32
            + sUnionRoomGroupOffsets[memberId][1] as i32
            + MAP_OFFSET
            == y
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsUnionRoomPlayerHidden(player_idx: u32) -> u32 {
    FlagGet(FLAG_HIDE_UNION_ROOM_PLAYER_1 + player_idx as u16) as u32
}
unsafe fn HideUnionRoomPlayer(player_idx: u32) {
    FlagSet(FLAG_HIDE_UNION_ROOM_PLAYER_1 + player_idx as u16);
}
unsafe fn ShowUnionRoomPlayer(player_idx: u32) {
    FlagClear(FLAG_HIDE_UNION_ROOM_PLAYER_1 + player_idx as u16);
}
unsafe fn SetUnionRoomPlayerGfx(leaderId: u32, gfxId: u32) {
    VarSet(VAR_OBJ_GFX_ID_0 + leaderId as u16, gfxId as u16);
}
unsafe fn CreateUnionRoomPlayerObjectEvent(leaderId: u32) {
    TrySpawnObjectEvent(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
unsafe fn RemoveUnionRoomPlayerObjectEvent(leaderId: u32) {
    RemoveObjectEventByLocalIdAndMap(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
unsafe fn SetUnionRoomPlayerEnterExitMovement(leaderId: u32, movement: *mut u8) -> u32 {
    let mut objectId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectId,
    ) != 0
    {
        return FALSE as u32;
    }
    let object: *mut ObjectEvent = &raw mut gObjectEvents[objectId];
    if ObjectEventIsMovementOverridden(object) != 0 {
        return FALSE as u32;
    }
    if ObjectEventSetHeldMovement(object, *movement) != 0 {
        return FALSE as u32;
    }
    TRUE as u32
}
unsafe fn TryReleaseUnionRoomPlayerObjectEvent(leaderId: u32) -> u32 {
    let mut objectId: u8 = 0;
    if TryGetObjectEventIdByLocalIdAndMap(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectId,
    ) != 0
    {
        return TRUE as u32;
    }
    let object: *mut ObjectEvent = &raw mut gObjectEvents[objectId];
    if ObjectEventClearHeldMovementIfFinished(object) == 0 {
        return FALSE as u32;
    }
    if ArePlayerFieldControlsLocked() == 0 {
        UnfreezeObjectEvent(object);
    } else {
        FreezeObjectEvent(object);
    }
    TRUE as u32
}
pub unsafe fn InitUnionRoomPlayerObjects(players: *mut UnionRoomObject) -> u8 {
    sUnionObjRefreshTimer.set(0);
    sUnionObjWork = players;
    for i in 0..MAX_UNION_ROOM_LEADERS {
        (*players.at(i)).state = 0;
        (*players.at(i)).gfxId = 0;
        (*players.at(i)).animState = 0;
        (*players.at(i)).schedAnim = UNION_ROOM_SPAWN_NONE;
    }
    CreateTask_AnimateUnionRoomPlayers()
}
unsafe fn AnimateUnionRoomPlayerDespawn(
    state: *mut i8,
    leaderId: u32,
    object: *mut UnionRoomObject,
) -> u32 {
    match *state {
        0 => {
            if SetUnionRoomPlayerEnterExitMovement(
                leaderId,
                sMovement_UnionPlayerExit.as_ptr().cast_mut(),
            ) == TRUE as u32
            {
                HideUnionRoomPlayer(leaderId);
                *state += 1;
            }
        }
        1 if TryReleaseUnionRoomPlayerObjectEvent(leaderId) != 0 => {
            RemoveUnionRoomPlayerObjectEvent(leaderId);
            HideUnionRoomPlayer(leaderId);
            *state = 0;
            return TRUE as u32;
        }
        _ => {}
    }
    FALSE as u32
}
unsafe fn AnimateUnionRoomPlayerSpawn(
    state: *mut i8,
    leaderId: u32,
    object: *mut UnionRoomObject,
) -> u32 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    'l1: {
        let sw1: i8 = *state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if IsPlayerStandingStill() == 0 {
                break 'l1;
            }
            PlayerGetDestCoords(&raw mut x, &raw mut y);
            if IsUnionRoomPlayerAt(leaderId, 0, x as i32, y as i32) == TRUE as u32 {
                break 'l1;
            }
            player_get_pos_including_state_based_drift(&raw mut x, &raw mut y);
            if IsUnionRoomPlayerAt(leaderId, 0, x as i32, y as i32) == TRUE as u32 {
                break 'l1;
            }
            SetUnionRoomPlayerGfx(leaderId, (*object).gfxId as u32);
            CreateUnionRoomPlayerObjectEvent(leaderId);
            ShowUnionRoomPlayer(leaderId);
            *state += 1;
        }
        if fall || sw1 == 3 {
            if SetUnionRoomPlayerEnterExitMovement(
                leaderId,
                sMovement_UnionPlayerEnter.as_ptr().cast_mut(),
            ) == TRUE as u32
            {
                *state += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if TryReleaseUnionRoomPlayerObjectEvent(leaderId) != 0 {
                *state = 0;
                return TRUE as u32;
            }
            break 'l1;
        }
    }
    FALSE as u32
}
unsafe fn SpawnGroupLeader(leaderId: u32, gender: u32, id: u32) -> u32 {
    let object: *mut UnionRoomObject = sUnionObjWork.at(leaderId);
    (*object).schedAnim = UNION_ROOM_SPAWN_IN;
    (*object).gfxId = GetUnionRoomPlayerGraphicsId(gender, id);
    if (*object).state == 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn DespawnGroupLeader(leaderId: u32) -> u32 {
    let object: *mut UnionRoomObject = sUnionObjWork.at(leaderId);
    (*object).schedAnim = UNION_ROOM_SPAWN_OUT;
    if (*object).state == 1 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn AnimateUnionRoomPlayer(leaderId: u32, object: *mut UnionRoomObject) {
    'l1: {
        let sw1: u8 = (*object).state;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if (*object).schedAnim == UNION_ROOM_SPAWN_IN {
                (*object).state = 2;
                (*object).animState = 0;
            } else {
                break 'l1;
            }
        }
        if fall || sw1 == 2 {
            if IsUnionRoomPlayerInvisible(leaderId, 0) == 0
                && (*object).schedAnim == UNION_ROOM_SPAWN_OUT
            {
                (*object).state = 0;
                (*object).animState = 0;
                RemoveUnionRoomPlayerObjectEvent(leaderId);
                HideUnionRoomPlayer(leaderId);
            } else if AnimateUnionRoomPlayerSpawn(&raw mut (*object).animState, leaderId, object)
                == TRUE as u32
            {
                (*object).state = 1;
            }
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            if (*object).schedAnim != UNION_ROOM_SPAWN_OUT {
                break 'l1;
            }
            (*object).state = 3;
            (*object).animState = 0;
        }
        if fall || sw1 == 3 {
            if AnimateUnionRoomPlayerDespawn(&raw mut (*object).animState, leaderId, object) == 1 {
                (*object).state = 0;
            }
            break 'l1;
        }
    }
    (*object).schedAnim = UNION_ROOM_SPAWN_NONE;
}
pub(crate) unsafe fn Task_AnimateUnionRoomPlayers(taskId: u8) {
    for i in 0..MAX_UNION_ROOM_LEADERS {
        AnimateUnionRoomPlayer(i as u32, sUnionObjWork.at(i));
    }
}
unsafe fn CreateTask_AnimateUnionRoomPlayers() -> u8 {
    if FuncIsActiveTask(Some(Task_AnimateUnionRoomPlayers)) == TRUE {
        return NUM_TASKS as u8;
    } else {
        return CreateTask(Some(Task_AnimateUnionRoomPlayers), 5);
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn DestroyTask_AnimateUnionRoomPlayers() {
    let taskId: u8 = FindTaskIdByFunc(Some(Task_AnimateUnionRoomPlayers));
    if taskId < NUM_TASKS as u8 {
        DestroyTask(taskId);
    }
}
pub unsafe fn DestroyUnionRoomPlayerObjects() {
    for i in 0..MAX_UNION_ROOM_LEADERS {
        if IsUnionRoomPlayerHidden(i as u32) == 0 {
            RemoveUnionRoomPlayerObjectEvent(i as u32);
            HideUnionRoomPlayer(i as u32);
        }
    }
    sUnionObjWork = null_mut();
    DestroyTask_AnimateUnionRoomPlayers();
}
pub unsafe fn CreateUnionRoomPlayerSprites(spriteIds: *mut u8, leaderId: i32) {
    for memberId in 0..MAX_RFU_PLAYERS {
        let id: i32 = 5 * leaderId + memberId;
        *spriteIds.at(id) = CreateVirtualObject(
            OBJ_EVENT_GFX_MAN_4,
            id as u8 - UR_SPRITE_START_ID,
            sUnionRoomPlayerCoords[leaderId][0] + sUnionRoomGroupOffsets[memberId][0] as i16,
            sUnionRoomPlayerCoords[leaderId][1] + sUnionRoomGroupOffsets[memberId][1] as i16,
            3,
            1,
        );
        SetVirtualObjectInvisibility(id as u8 - UR_SPRITE_START_ID, TRUE as u32);
    }
}
pub unsafe fn DestroyUnionRoomPlayerSprites(spriteIds: *mut u8) {
    for i in 0..NUM_UNION_ROOM_SPRITES {
        DestroySprite(&raw mut gSprites[*spriteIds.at(i)]);
    }
}
pub unsafe fn SetTilesAroundUnionRoomPlayersPassable() {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for i in 0..MAX_UNION_ROOM_LEADERS {
        for memberId in 0..MAX_RFU_PLAYERS {
            GetUnionRoomPlayerCoords(i as u32, memberId as u32, &raw mut x, &raw mut y);
            MapGridSetMetatileImpassabilityAt(x, y, FALSE as u32);
        }
    }
}
unsafe fn GetNewFacingDirectionForUnionRoomPlayer(
    memberId: u32,
    leaderId: u32,
    gameData: *mut RfuGameData,
) -> u8 {
    if memberId != 0 {
        return sMemberFacingDirections[memberId];
    } else if (*gameData).activity() == 69 {
        return DIR_SOUTH;
    } else {
        return DIR_EAST;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
unsafe fn IsUnionRoomPlayerInvisible(leaderId: u32, memberId: u32) -> u32 {
    IsVirtualObjectInvisible(5 * leaderId as u8 + memberId as u8 - UR_SPRITE_START_ID)
}
unsafe fn SpawnGroupMember(
    leaderId: u32,
    memberId: u32,
    graphicsId: u8,
    gameData: *mut RfuGameData,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let id: i32 = 5 * leaderId as i32 + memberId as i32;
    if IsUnionRoomPlayerInvisible(leaderId, memberId) == TRUE as u32 {
        SetVirtualObjectInvisibility(id as u8 - UR_SPRITE_START_ID, FALSE as u32);
        SetVirtualObjectSpriteAnim(id as u8 - UR_SPRITE_START_ID, UNION_ROOM_SPAWN_IN);
    }
    SetVirtualObjectGraphics(id as u8 - UR_SPRITE_START_ID, graphicsId);
    SetUnionRoomObjectFacingDirection(
        memberId as i32,
        leaderId as i32,
        GetNewFacingDirectionForUnionRoomPlayer(memberId, leaderId, gameData),
    );
    GetUnionRoomPlayerCoords(leaderId, memberId, &raw mut x, &raw mut y);
    MapGridSetMetatileImpassabilityAt(x, y, TRUE as u32);
}
unsafe fn DespawnGroupMember(leaderId: u32, memberId: u32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    SetVirtualObjectSpriteAnim(
        5 * leaderId as u8 + memberId as u8 - UR_SPRITE_START_ID,
        UNION_ROOM_SPAWN_OUT,
    );
    GetUnionRoomPlayerCoords(leaderId, memberId, &raw mut x, &raw mut y);
    MapGridSetMetatileImpassabilityAt(x, y, FALSE as u32);
}
unsafe fn AssembleGroup(leaderId: u32, gameData: *mut RfuGameData) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut x2: i16 = 0;
    let mut y2: i16 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    player_get_pos_including_state_based_drift(&raw mut x2, &raw mut y2);
    if IsVirtualObjectInvisible((5 * leaderId as u8) - UR_SPRITE_START_ID) == TRUE as u32 {
        if IsUnionRoomPlayerAt(leaderId, 0, x as i32, y as i32) == TRUE as u32
            || IsUnionRoomPlayerAt(leaderId, 0, x2 as i32, y2 as i32) == TRUE as u32
        {
            return;
        }
        SpawnGroupMember(
            leaderId,
            0,
            GetUnionRoomPlayerGraphicsId(
                (*gameData).playerGender() as u32,
                (*gameData).compatibility.playerTrainerId[0] as u32,
            ),
            gameData,
        );
    }
    for i in 1..MAX_RFU_PLAYERS {
        if (*gameData).partnerInfo[i - 1] == 0 {
            DespawnGroupMember(leaderId, i as u32);
        } else if IsUnionRoomPlayerAt(leaderId, i as u32, x as i32, y as i32) == FALSE as u32
            && IsUnionRoomPlayerAt(leaderId, i as u32, x2 as i32, y2 as i32) == FALSE as u32
        {
            SpawnGroupMember(
                leaderId,
                i as u32,
                GetUnionRoomPlayerGraphicsId(
                    ((*gameData).partnerInfo[i - 1] >> 3) as u32 & 1,
                    (*gameData).partnerInfo[i - 1] as u32 & PINFO_TID_MASK as u32,
                ),
                gameData,
            );
        }
    }
}
unsafe fn SpawnGroupLeaderAndMembers(leaderId: u32, gameData: *mut RfuGameData) {
    match (*gameData).activity() {
        IN_UNION_ROOM | 84 => {
            SpawnGroupLeader(
                leaderId,
                (*gameData).playerGender() as u32,
                (*gameData).compatibility.playerTrainerId[0] as u32,
            );
            for i in 0..(MAX_RFU_PLAYERS as u32) {
                DespawnGroupMember(leaderId, i);
            }
        }
        65 | 68 | 69 | 72 | 81 | 82 | 83 => {
            DespawnGroupLeader(leaderId);
            AssembleGroup(leaderId, gameData);
        }
        _ => {}
    }
}
unsafe fn DespawnGroupLeaderAndMembers(leaderId: u32, gameData: *mut RfuGameData) {
    DespawnGroupLeader(leaderId);
    for i in 0..MAX_RFU_PLAYERS {
        DespawnGroupMember(leaderId, i as u32);
    }
}
unsafe fn UpdateUnionRoomPlayerSprites(uroom: *mut WirelessLink_URoom) {
    sUnionObjRefreshTimer.set(0);
    let leaders: *mut RfuPlayer = (*(*uroom).playerList).players.as_mut_ptr();
    for i in 0..MAX_UNION_ROOM_LEADERS {
        if (*leaders.at(i)).groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
            SpawnGroupLeaderAndMembers(i as u32, &raw mut (*leaders.at(i)).rfu.data);
        } else if (*leaders.at(i)).groupScheduledAnim() == UNION_ROOM_SPAWN_OUT {
            DespawnGroupLeaderAndMembers(i as u32, &raw mut (*leaders.at(i)).rfu.data);
        }
    }
}
pub fn ScheduleUnionRoomPlayerRefresh(uroom: *mut WirelessLink_URoom) {
    sUnionObjRefreshTimer.set(300);
}
pub unsafe fn HandleUnionRoomPlayerRefresh(uroom: *mut WirelessLink_URoom) {
    if ({
        sUnionObjRefreshTimer.set(sUnionObjRefreshTimer.get() + 1);
        sUnionObjRefreshTimer.get()
    }) > 300
    {
        UpdateUnionRoomPlayerSprites(uroom);
    }
}
pub unsafe fn TryInteractWithUnionRoomMember(
    list: *mut RfuPlayerList,
    memberIdPtr: *mut i16,
    leaderIdPtr: *mut i16,
    spriteIds: *mut u8,
) -> u32 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if IsPlayerStandingStill() == 0 {
        return FALSE as u32;
    }
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    let leaders: *mut RfuPlayer = (*list).players.as_mut_ptr();
    for i in 0..MAX_UNION_ROOM_LEADERS {
        for memberId in 0..MAX_RFU_PLAYERS {
            'l2: {
                let id: i32 = 5 * i + memberId;
                if x as i32
                    != sUnionRoomPlayerCoords[i][0] as i32
                        + sUnionRoomGroupOffsets[memberId][0] as i32
                        + 7
                {
                    break 'l2;
                }
                if y as i32
                    != sUnionRoomPlayerCoords[i][1] as i32
                        + sUnionRoomGroupOffsets[memberId][1] as i32
                        + 7
                {
                    break 'l2;
                }
                if IsVirtualObjectInvisible(id as u8 - UR_SPRITE_START_ID) != 0 {
                    break 'l2;
                }
                if IsVirtualObjectAnimating(id as u8 - UR_SPRITE_START_ID) != 0 {
                    break 'l2;
                }
                if (*leaders.at(i)).groupScheduledAnim() != UNION_ROOM_SPAWN_IN {
                    break 'l2;
                }
                SetUnionRoomObjectFacingDirection(
                    memberId,
                    i,
                    sOppositeFacingDirection[GetPlayerFacingDirection()],
                );
                *memberIdPtr = memberId as i16;
                *leaderIdPtr = i as i16;
                return TRUE as u32;
            }
        }
    }
    FALSE as u32
}
unsafe fn SetUnionRoomObjectFacingDirection(memberId: i32, leaderId: i32, newDirection: u8) {
    TurnVirtualObject(
        MAX_RFU_PLAYERS as u8 * leaderId as u8 - UR_SPRITE_START_ID + memberId as u8,
        newDirection,
    );
}
pub unsafe fn UpdateUnionRoomMemberFacing(memberId: u32, leaderId: u32, list: *mut RfuPlayerList) {
    SetUnionRoomObjectFacingDirection(
        memberId as i32,
        leaderId as i32,
        GetNewFacingDirectionForUnionRoomPlayer(
            memberId,
            leaderId,
            &raw mut (*list).players[leaderId].rfu.data,
        ),
    );
}
