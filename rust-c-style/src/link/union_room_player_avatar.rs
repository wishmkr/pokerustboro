//! Translated from `src/union_room_player_avatar.c` by tools/rustport/c2rs.py.
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
pub(crate) static mut sUnionObjRefreshTimer: u32 = 0;

unsafe extern "C" {
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSprites: CArray<Sprite, 65>;
    fn ArePlayerFieldControlsLocked() -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn CreateVirtualObject(a0: u8, a1: u8, a2: i16, a3: i16, a4: u8, a5: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn FlagClear(a0: u16) -> u8;
    fn FlagGet(a0: u16) -> u8;
    fn FlagSet(a0: u16) -> u8;
    fn FreezeObjectEvent(a0: *mut ObjectEvent) -> u8;
    fn FuncIsActiveTask(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetPlayerFacingDirection() -> u8;
    fn GetXYCoordsOneStepInFrontOfPlayer(a0: *mut i16, a1: *mut i16);
    fn IsVirtualObjectAnimating(a0: u8) -> u32;
    fn IsVirtualObjectInvisible(a0: u8) -> u32;
    fn MapGridSetMetatileImpassabilityAt(a0: i32, a1: i32, a2: u32);
    fn ObjectEventClearHeldMovementIfFinished(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventIsMovementOverridden(a0: *mut ObjectEvent) -> u8;
    fn ObjectEventSetHeldMovement(a0: *mut ObjectEvent, a1: u8) -> u8;
    fn PlayerGetDestCoords(a0: *mut i16, a1: *mut i16);
    fn RemoveObjectEventByLocalIdAndMap(a0: u8, a1: u8, a2: u8);
    fn SetVirtualObjectGraphics(a0: u8, a1: u8);
    fn SetVirtualObjectInvisibility(a0: u8, a1: u32);
    fn SetVirtualObjectSpriteAnim(a0: u8, a1: u8);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn TrySpawnObjectEvent(a0: u8, a1: u8, a2: u8) -> u8;
    fn TurnVirtualObject(a0: u8, a1: u8);
    fn UnfreezeObjectEvent(a0: *mut ObjectEvent);
    fn VarSet(a0: u16, a1: u16) -> u8;
    fn player_get_pos_including_state_based_drift(a0: *mut i16, a1: *mut i16) -> u8;
}

pub(crate) unsafe extern "C" fn IsPlayerStandingStill() -> u32 {
    if gPlayerAvatar.tileTransitionState == T_TILE_CENTER
        || gPlayerAvatar.tileTransitionState == T_NOT_MOVING
    {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn GetUnionRoomPlayerGraphicsId(gender: u32, id: u32) -> u8 {
    return sUnionRoomObjGfxIds[gender][id % 8];
}
pub(crate) unsafe extern "C" fn GetUnionRoomPlayerCoords(
    leaderId: u32,
    memberId: u32,
    x: *mut i32,
    y: *mut i32,
) {
    *x = sUnionRoomPlayerCoords[leaderId][0] as i32
        + sUnionRoomGroupOffsets[memberId][0] as i32
        + MAP_OFFSET;
    *y = sUnionRoomPlayerCoords[leaderId][1] as i32
        + sUnionRoomGroupOffsets[memberId][1] as i32
        + MAP_OFFSET;
}
pub(crate) unsafe extern "C" fn IsUnionRoomPlayerAt(
    leaderId: u32,
    memberId: u32,
    x: i32,
    y: i32,
) -> u32 {
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsUnionRoomPlayerHidden(player_idx: u32) -> u32 {
    return FlagGet(FLAG_HIDE_UNION_ROOM_PLAYER_1 + player_idx as u16) as u32;
}
pub(crate) unsafe extern "C" fn HideUnionRoomPlayer(player_idx: u32) {
    FlagSet(FLAG_HIDE_UNION_ROOM_PLAYER_1 + player_idx as u16);
}
pub(crate) unsafe extern "C" fn ShowUnionRoomPlayer(player_idx: u32) {
    FlagClear(FLAG_HIDE_UNION_ROOM_PLAYER_1 + player_idx as u16);
}
pub(crate) unsafe extern "C" fn SetUnionRoomPlayerGfx(leaderId: u32, gfxId: u32) {
    VarSet(VAR_OBJ_GFX_ID_0 + leaderId as u16, gfxId as u16);
}
pub(crate) unsafe extern "C" fn CreateUnionRoomPlayerObjectEvent(leaderId: u32) {
    TrySpawnObjectEvent(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
pub(crate) unsafe extern "C" fn RemoveUnionRoomPlayerObjectEvent(leaderId: u32) {
    RemoveObjectEventByLocalIdAndMap(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
    );
}
pub(crate) unsafe extern "C" fn SetUnionRoomPlayerEnterExitMovement(
    leaderId: u32,
    movement: *mut u8,
) -> u32 {
    let mut objectId: u8 = 0;
    let mut object: *mut ObjectEvent = null_mut();
    if TryGetObjectEventIdByLocalIdAndMap(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectId,
    ) != 0
    {
        return FALSE as u32;
    }
    object = &raw mut gObjectEvents[objectId];
    if ObjectEventIsMovementOverridden(object) != 0 {
        return FALSE as u32;
    }
    if ObjectEventSetHeldMovement(object, *movement) != 0 {
        return FALSE as u32;
    }
    return TRUE as u32;
}
pub(crate) unsafe extern "C" fn TryReleaseUnionRoomPlayerObjectEvent(leaderId: u32) -> u32 {
    let mut objectId: u8 = 0;
    let mut object: *mut ObjectEvent = null_mut();
    if TryGetObjectEventIdByLocalIdAndMap(
        sUnionRoomLocalIds[leaderId],
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectId,
    ) != 0
    {
        return TRUE as u32;
    }
    object = &raw mut gObjectEvents[objectId];
    if ObjectEventClearHeldMovementIfFinished(object) == 0 {
        return FALSE as u32;
    }
    if ArePlayerFieldControlsLocked() == 0 {
        UnfreezeObjectEvent(object);
    } else {
        FreezeObjectEvent(object);
    }
    return TRUE as u32;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn InitUnionRoomPlayerObjects(mut players: *mut UnionRoomObject) -> u8 {
    let mut i: i32 = 0;
    sUnionObjRefreshTimer = 0;
    sUnionObjWork = players;
    i = 0;
    while i < MAX_UNION_ROOM_LEADERS {
        (*players.at(i)).state = 0;
        (*players.at(i)).gfxId = 0;
        (*players.at(i)).animState = 0;
        (*players.at(i)).schedAnim = UNION_ROOM_SPAWN_NONE;
        i += 1;
    }
    return CreateTask_AnimateUnionRoomPlayers();
}
pub(crate) unsafe extern "C" fn AnimateUnionRoomPlayerDespawn(
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
        1 => {
            if TryReleaseUnionRoomPlayerObjectEvent(leaderId) != 0 {
                RemoveUnionRoomPlayerObjectEvent(leaderId);
                HideUnionRoomPlayer(leaderId);
                *state = 0;
                return TRUE as u32;
            }
        }
        _ => {}
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn AnimateUnionRoomPlayerSpawn(
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
            fall = true;
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
            fall = true;
            if TryReleaseUnionRoomPlayerObjectEvent(leaderId) != 0 {
                *state = 0;
                return TRUE as u32;
            }
            break 'l1;
        }
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn SpawnGroupLeader(leaderId: u32, gender: u32, id: u32) -> u32 {
    let mut object: *mut UnionRoomObject = sUnionObjWork.at(leaderId);
    (*object).schedAnim = UNION_ROOM_SPAWN_IN;
    (*object).gfxId = GetUnionRoomPlayerGraphicsId(gender, id);
    if (*object).state == 0 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DespawnGroupLeader(leaderId: u32) -> u32 {
    let mut object: *mut UnionRoomObject = sUnionObjWork.at(leaderId);
    (*object).schedAnim = UNION_ROOM_SPAWN_OUT;
    if (*object).state == 1 {
        return TRUE as u32;
    } else {
        return FALSE as u32;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn AnimateUnionRoomPlayer(
    leaderId: u32,
    object: *mut UnionRoomObject,
) {
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
            fall = true;
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
            fall = true;
            if AnimateUnionRoomPlayerDespawn(&raw mut (*object).animState, leaderId, object) == 1 {
                (*object).state = 0;
            }
            break 'l1;
        }
    }
    (*object).schedAnim = UNION_ROOM_SPAWN_NONE;
}
pub(crate) unsafe extern "C" fn Task_AnimateUnionRoomPlayers(taskId: u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_UNION_ROOM_LEADERS {
        AnimateUnionRoomPlayer(i as u32, sUnionObjWork.at(i));
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn CreateTask_AnimateUnionRoomPlayers() -> u8 {
    if FuncIsActiveTask(Some(Task_AnimateUnionRoomPlayers)) == TRUE {
        return NUM_TASKS as u8;
    } else {
        return CreateTask(Some(Task_AnimateUnionRoomPlayers), 5);
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
pub(crate) unsafe extern "C" fn DestroyTask_AnimateUnionRoomPlayers() {
    let mut taskId: u8 = FindTaskIdByFunc(Some(Task_AnimateUnionRoomPlayers));
    if taskId < NUM_TASKS as u8 {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyUnionRoomPlayerObjects() {
    let mut i: i32 = 0;
    i = 0;
    while i < MAX_UNION_ROOM_LEADERS {
        if IsUnionRoomPlayerHidden(i as u32) == 0 {
            RemoveUnionRoomPlayerObjectEvent(i as u32);
            HideUnionRoomPlayer(i as u32);
        }
        i += 1;
    }
    sUnionObjWork = null_mut();
    DestroyTask_AnimateUnionRoomPlayers();
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateUnionRoomPlayerSprites(mut spriteIds: *mut u8, leaderId: i32) {
    let mut memberId: i32 = 0;
    memberId = 0;
    while memberId < MAX_RFU_PLAYERS {
        let mut id: i32 = 5 * leaderId + memberId;
        *spriteIds.at(id) = CreateVirtualObject(
            OBJ_EVENT_GFX_MAN_4,
            id as u8 - UR_SPRITE_START_ID,
            sUnionRoomPlayerCoords[leaderId][0] + sUnionRoomGroupOffsets[memberId][0] as i16,
            sUnionRoomPlayerCoords[leaderId][1] + sUnionRoomGroupOffsets[memberId][1] as i16,
            3,
            1,
        );
        SetVirtualObjectInvisibility(id as u8 - UR_SPRITE_START_ID, TRUE as u32);
        memberId += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyUnionRoomPlayerSprites(spriteIds: *mut u8) {
    let mut i: i32 = 0;
    i = 0;
    while i < NUM_UNION_ROOM_SPRITES {
        DestroySprite(&raw mut gSprites[*spriteIds.at(i)]);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetTilesAroundUnionRoomPlayersPassable() {
    let mut i: i32 = 0;
    let mut memberId: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    i = 0;
    while i < MAX_UNION_ROOM_LEADERS {
        memberId = 0;
        while memberId < MAX_RFU_PLAYERS {
            GetUnionRoomPlayerCoords(i as u32, memberId as u32, &raw mut x, &raw mut y);
            MapGridSetMetatileImpassabilityAt(x, y, FALSE as u32);
            memberId += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn GetNewFacingDirectionForUnionRoomPlayer(
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
        return 0;
    }
}
pub(crate) unsafe extern "C" fn IsUnionRoomPlayerInvisible(leaderId: u32, memberId: u32) -> u32 {
    return IsVirtualObjectInvisible(5 * leaderId as u8 + memberId as u8 - UR_SPRITE_START_ID);
}
pub(crate) unsafe extern "C" fn SpawnGroupMember(
    leaderId: u32,
    memberId: u32,
    graphicsId: u8,
    gameData: *mut RfuGameData,
) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut id: i32 = 5 * leaderId as i32 + memberId as i32;
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
pub(crate) unsafe extern "C" fn DespawnGroupMember(leaderId: u32, memberId: u32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    SetVirtualObjectSpriteAnim(
        5 * leaderId as u8 + memberId as u8 - UR_SPRITE_START_ID,
        UNION_ROOM_SPAWN_OUT,
    );
    GetUnionRoomPlayerCoords(leaderId, memberId, &raw mut x, &raw mut y);
    MapGridSetMetatileImpassabilityAt(x, y, FALSE as u32);
}
pub(crate) unsafe extern "C" fn AssembleGroup(leaderId: u32, gameData: *mut RfuGameData) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut x2: i16 = 0;
    let mut y2: i16 = 0;
    let mut i: i32 = 0;
    PlayerGetDestCoords(&raw mut x, &raw mut y);
    player_get_pos_including_state_based_drift(&raw mut x2, &raw mut y2);
    if IsVirtualObjectInvisible(5 * leaderId as u8 + 0 - UR_SPRITE_START_ID) == TRUE as u32 {
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
    i = 1;
    while i < MAX_RFU_PLAYERS {
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
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpawnGroupLeaderAndMembers(
    leaderId: u32,
    gameData: *mut RfuGameData,
) {
    let mut i: u32 = 0;
    match (*gameData).activity() {
        IN_UNION_ROOM | 84 => {
            SpawnGroupLeader(
                leaderId,
                (*gameData).playerGender() as u32,
                (*gameData).compatibility.playerTrainerId[0] as u32,
            );
            i = 0;
            while i < MAX_RFU_PLAYERS as u32 {
                DespawnGroupMember(leaderId, i);
                i += 1;
            }
        }
        65 | 68 | 69 | 72 | 81 | 82 | 83 => {
            DespawnGroupLeader(leaderId);
            AssembleGroup(leaderId, gameData);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn DespawnGroupLeaderAndMembers(
    leaderId: u32,
    gameData: *mut RfuGameData,
) {
    let mut i: i32 = 0;
    DespawnGroupLeader(leaderId);
    i = 0;
    while i < MAX_RFU_PLAYERS {
        DespawnGroupMember(leaderId, i as u32);
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn UpdateUnionRoomPlayerSprites(uroom: *mut WirelessLink_URoom) {
    let mut i: i32 = 0;
    let mut leaders: *mut RfuPlayer = null_mut();
    sUnionObjRefreshTimer = 0;
    i = 0;
    leaders = (*(*uroom).playerList).players.as_mut_ptr();
    while i < MAX_UNION_ROOM_LEADERS {
        if (*leaders.at(i)).groupScheduledAnim() == UNION_ROOM_SPAWN_IN {
            SpawnGroupLeaderAndMembers(i as u32, &raw mut (*leaders.at(i)).rfu.data);
        } else if (*leaders.at(i)).groupScheduledAnim() == UNION_ROOM_SPAWN_OUT {
            DespawnGroupLeaderAndMembers(i as u32, &raw mut (*leaders.at(i)).rfu.data);
        }
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ScheduleUnionRoomPlayerRefresh(uroom: *mut WirelessLink_URoom) {
    sUnionObjRefreshTimer = 300;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn HandleUnionRoomPlayerRefresh(uroom: *mut WirelessLink_URoom) {
    if ({
        sUnionObjRefreshTimer += 1;
        sUnionObjRefreshTimer
    }) > 300
    {
        UpdateUnionRoomPlayerSprites(uroom);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn TryInteractWithUnionRoomMember(
    list: *mut RfuPlayerList,
    memberIdPtr: *mut i16,
    leaderIdPtr: *mut i16,
    spriteIds: *mut u8,
) -> u32 {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut i: i32 = 0;
    let mut memberId: i32 = 0;
    let mut leaders: *mut RfuPlayer = null_mut();
    if IsPlayerStandingStill() == 0 {
        return FALSE as u32;
    }
    GetXYCoordsOneStepInFrontOfPlayer(&raw mut x, &raw mut y);
    i = 0;
    leaders = (*list).players.as_mut_ptr();
    while i < MAX_UNION_ROOM_LEADERS {
        memberId = 0;
        while memberId < MAX_RFU_PLAYERS {
            'l2: {
                let mut id: i32 = 5 * i + memberId;
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
            memberId += 1;
        }
        i += 1;
    }
    return FALSE as u32;
}
pub(crate) unsafe extern "C" fn SetUnionRoomObjectFacingDirection(
    memberId: i32,
    leaderId: i32,
    newDirection: u8,
) {
    TurnVirtualObject(
        MAX_RFU_PLAYERS as u8 * leaderId as u8 - UR_SPRITE_START_ID + memberId as u8,
        newDirection,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateUnionRoomMemberFacing(
    memberId: u32,
    leaderId: u32,
    list: *mut RfuPlayerList,
) {
    SetUnionRoomObjectFacingDirection(
        memberId as i32,
        leaderId as i32,
        GetNewFacingDirectionForUnionRoomPlayer(
            memberId,
            leaderId,
            &raw mut (*list).players[leaderId].rfu.data,
        ),
    );
    return;
}
