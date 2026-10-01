//! Translated from `src/faraway_island.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    unused_assignments,
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::{FlagGet, VarGet, VarSet};
use crate::event_object_movement::{
    SetSpritePosToOffsetMapCoords, TryGetObjectEventIdByLocalIdAndMap,
};
use crate::ffi::{gSpecialVar_0x8004, gSpecialVar_Facing};
use crate::field_player_avatar::{gObjectEvents, gPlayerAvatar};
use crate::field_weather::UpdateSpritePaletteWithWeather;
use crate::fieldmap::MapGridGetMetatileBehaviorAt;
use crate::load_save::gSaveBlock1Ptr;
use crate::metatile_behavior::MetatileBehavior_IsPokeGrass;
use crate::sprite::IndexOfSpritePaletteTag;
use crate::sprite::gSprites;
#[allow(unused_imports)]
use crate::types::*;
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
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `LoadSpritePalette` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalette(a0: *mut SpritePalette) -> u8 {
    unsafe { crate::sprite::LoadSpritePalette(a0 as _) }
}
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// Data tables (translate with cdata.py): sFarawayIslandRockCoords

static sFarawayIslandRockCoords: Table<CArray<CArray<i16, 2>, 4>> =
    Table((&raw const crate::data::faraway_island::sFarawayIslandRockCoords).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static sGrassSpriteId: crate::global::Global<u8> = crate::global::Global::new(0);
pub(crate) static sPlayerToMewDeltaX: crate::global::Global<i16> = crate::global::Global::new(0);
pub(crate) static sPlayerToMewDeltaY: crate::global::Global<i16> = crate::global::Global::new(0);
pub(crate) static mut sMewDirectionCandidates: Aligned<CArray<u8, 4>> =
    Aligned(unsafe { zeroed() });

unsafe fn GetMewObjectEventId() -> u8 {
    let mut objectEventId: u8 = 0;
    TryGetObjectEventIdByLocalIdAndMap(
        LOCALID_FARAWAY_ISLAND_MEW,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectEventId,
    );
    objectEventId
}
pub unsafe fn GetMewMoveDirection() -> u32 {
    let mut mewSafeFromTrap: i32 = 0;
    let mew: *mut ObjectEvent = &raw mut gObjectEvents[GetMewObjectEventId()];
    sPlayerToMewDeltaX
        .set(gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x - (*mew).currentCoords.x);
    sPlayerToMewDeltaY
        .set(gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y - (*mew).currentCoords.y);
    for i in 0..4u8 {
        sMewDirectionCandidates[i] = DIR_NONE;
    }
    if gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x
        == gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x
        && gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y
            == gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y
    {
        return DIR_NONE as u32;
    }
    if VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32 % 8 == 0 {
        (*mew).set_invisible(FALSE as u32);
    } else {
        (*mew).set_invisible(TRUE as u32);
    }
    if VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32 % 9 == 0 {
        return DIR_NONE as u32;
    }
    for i in 0..4u8 {
        if gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x
            == sFarawayIslandRockCoords[i][0]
        {
            mewSafeFromTrap = FALSE as i32;
            if gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y
                < sFarawayIslandRockCoords[i][1]
            {
                if (*mew).currentCoords.y <= sFarawayIslandRockCoords[i][1] {
                    mewSafeFromTrap = TRUE as i32;
                }
            } else {
                if (*mew).currentCoords.y >= sFarawayIslandRockCoords[i][1] {
                    mewSafeFromTrap = TRUE as i32;
                }
            }
            if mewSafeFromTrap == 0 {
                if sPlayerToMewDeltaX.get() > 0 {
                    if (*mew).currentCoords.x as i32 + 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x as i32
                        && CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y)
                            != 0
                    {
                        return DIR_EAST as u32;
                    }
                } else if sPlayerToMewDeltaX.get() < 0
                    && (*mew).currentCoords.x as i32 - 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x as i32
                    && CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0
                {
                    return DIR_WEST as u32;
                }
                if (*mew).currentCoords.x
                    == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x
                {
                    if sPlayerToMewDeltaY.get() > 0 {
                        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1)
                            != 0
                        {
                            return DIR_NORTH as u32;
                        }
                    } else {
                        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1)
                            != 0
                        {
                            return DIR_SOUTH as u32;
                        }
                    }
                }
            }
        }
        if gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y
            == sFarawayIslandRockCoords[i][1]
        {
            mewSafeFromTrap = FALSE as i32;
            if gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x
                < sFarawayIslandRockCoords[i][0]
            {
                if (*mew).currentCoords.x <= sFarawayIslandRockCoords[i][0] {
                    mewSafeFromTrap = TRUE as i32;
                }
            } else {
                if (*mew).currentCoords.x >= sFarawayIslandRockCoords[i][0] {
                    mewSafeFromTrap = TRUE as i32;
                }
            }
            if mewSafeFromTrap == 0 {
                if sPlayerToMewDeltaY.get() > 0 {
                    if (*mew).currentCoords.y as i32 + 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y as i32
                        && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1)
                            != 0
                    {
                        return DIR_SOUTH as u32;
                    }
                } else if sPlayerToMewDeltaY.get() < 0
                    && (*mew).currentCoords.y as i32 - 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y as i32
                    && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0
                {
                    return DIR_NORTH as u32;
                }
                if (*mew).currentCoords.y
                    == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y
                {
                    if sPlayerToMewDeltaX.get() > 0 {
                        if CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y)
                            != 0
                        {
                            return DIR_WEST as u32;
                        }
                    } else {
                        if CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y)
                            != 0
                        {
                            return DIR_EAST as u32;
                        }
                    }
                }
            }
        }
    }
    if ShouldMewMoveNorth(mew, 0) != 0 {
        if ShouldMewMoveEast(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else if ShouldMewMoveWest(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else {
            return DIR_NORTH as u32;
        }
    }
    if ShouldMewMoveSouth(mew, 0) != 0 {
        if ShouldMewMoveEast(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else if ShouldMewMoveWest(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else {
            return DIR_SOUTH as u32;
        }
    }
    if ShouldMewMoveEast(mew, 0) != 0 {
        if ShouldMewMoveNorth(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else if ShouldMewMoveSouth(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else {
            return DIR_EAST as u32;
        }
    }
    if ShouldMewMoveWest(mew, 0) != 0 {
        if ShouldMewMoveNorth(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else if ShouldMewMoveSouth(mew, 1) != 0 {
            return GetRandomMewDirectionCandidate(2) as u32;
        } else {
            return DIR_WEST as u32;
        }
    }
    if sPlayerToMewDeltaY.get() == 0 {
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y > (*mew).currentCoords.y
            && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0
        {
            return DIR_NORTH as u32;
        }
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y < (*mew).currentCoords.y
            && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) != 0
        {
            return DIR_SOUTH as u32;
        }
        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0 {
            return DIR_NORTH as u32;
        }
        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) != 0 {
            return DIR_SOUTH as u32;
        }
    }
    if sPlayerToMewDeltaX.get() == 0 {
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x > (*mew).currentCoords.x
            && CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0
        {
            return DIR_WEST as u32;
        }
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x < (*mew).currentCoords.x
            && CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) != 0
        {
            return DIR_EAST as u32;
        }
        if CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) != 0 {
            return DIR_EAST as u32;
        }
        if CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0 {
            return DIR_WEST as u32;
        }
    }
    GetValidMewMoveDirection(DIR_NONE) as u32
}
unsafe fn CanMewMoveToCoords(x: i16, y: i16) -> u8 {
    if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x == x
        && gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y == y
    {
        return FALSE;
    }
    MetatileBehavior_IsPokeGrass(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8)
}
unsafe fn GetValidMewMoveDirection(ignoredDir: u8) -> u8 {
    let mut count: u8 = 0;
    let mew: *mut ObjectEvent = &raw mut gObjectEvents[GetMewObjectEventId()];
    for i in 0..4u8 {
        sMewDirectionCandidates[i] = DIR_NONE;
    }
    if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) == 1
        && ignoredDir != DIR_NORTH
    {
        sMewDirectionCandidates[count] = DIR_NORTH;
        count += 1;
    }
    if CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) == 1
        && ignoredDir != DIR_EAST
    {
        sMewDirectionCandidates[count] = DIR_EAST;
        count += 1;
    }
    if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) == 1
        && ignoredDir != 1
    {
        sMewDirectionCandidates[count] = DIR_SOUTH;
        count += 1;
    }
    if CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) == 1
        && ignoredDir != DIR_WEST
    {
        sMewDirectionCandidates[count] = DIR_WEST;
        count += 1;
    }
    if count > 1 {
        return sMewDirectionCandidates
            [rem_i32(VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32, count as i32)];
    } else {
        return sMewDirectionCandidates[0];
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
pub unsafe fn UpdateFarawayIslandStepCounter() {
    let mut steps: u16 = VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER);
    if (*gSaveBlock1Ptr).location.mapNum == 57 && (*gSaveBlock1Ptr).location.mapGroup == 26 {
        steps += 1;
        if steps >= 9999 {
            VarSet(VAR_FARAWAY_ISLAND_STEP_COUNTER, 0);
        } else {
            VarSet(VAR_FARAWAY_ISLAND_STEP_COUNTER, steps);
        }
    }
}
pub unsafe fn ObjectEventIsFarawayIslandMew(objectEvent: *mut ObjectEvent) -> u8 {
    if (*gSaveBlock1Ptr).location.mapNum == 57
        && (*gSaveBlock1Ptr).location.mapGroup == 26
        && (*objectEvent).graphicsId == OBJ_EVENT_GFX_MEW
    {
        return TRUE;
    }
    FALSE
}
pub unsafe fn IsMewPlayingHideAndSeek() -> u8 {
    if (*gSaveBlock1Ptr).location.mapNum == 57
        && (*gSaveBlock1Ptr).location.mapGroup == 26
        && FlagGet(FLAG_CAUGHT_MEW) != TRUE
        && FlagGet(FLAG_HIDE_MEW) != TRUE
    {
        return TRUE;
    }
    FALSE
}
pub unsafe fn ShouldMewShakeGrass(objectEvent: *mut ObjectEvent) -> u8 {
    if VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) != 0xFFFF
        && VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32 % 4 == 0
    {
        return TRUE;
    }
    FALSE
}
#[unsafe(no_mangle)]
pub unsafe fn SetMewAboveGrass() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mew: *mut ObjectEvent = &raw mut gObjectEvents[GetMewObjectEventId()];
    (*mew).set_invisible(FALSE as u32);
    if gSpecialVar_0x8004 == 1 {
        (*mew).set_fixedPriority(1);
        gSprites[(*mew).spriteId].set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
        gSprites[(*mew).spriteId].subpriority = 1;
    } else {
        VarSet(VAR_FARAWAY_ISLAND_STEP_COUNTER, 0xFFFF);
        (*mew).set_fixedPriority(1);
        gSprites[(*mew).spriteId].set_subspriteMode(SUBSPRITES_IGNORE_PRIORITY);
        if gSpecialVar_Facing != DIR_NORTH as u16 {
            gSprites[(*mew).spriteId].subpriority = 1;
        }
        LoadSpritePalette((&raw const (*(&raw const crate::data::event_object_movement::gSpritePalette_GeneralFieldEffect1).cast::<SpritePalette>())).cast_mut());
        UpdateSpritePaletteWithWeather(IndexOfSpritePaletteTag(
            (*(&raw const crate::data::event_object_movement::gSpritePalette_GeneralFieldEffect1)
                .cast::<SpritePalette>())
            .tag,
        ));
        x = (*mew).currentCoords.x;
        y = (*mew).currentCoords.y;
        SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
        sGrassSpriteId.set(CreateSpriteAtEnd(
            (*(&raw const crate::data::event_object_movement::gFieldEffectObjectTemplatePointers)
                .cast::<CArray<*mut SpriteTemplate, 0>>())[15],
            x,
            y,
            gSprites[(*mew).spriteId].subpriority - 1,
        ));
        if sGrassSpriteId.get() != MAX_SPRITES {
            let sprite: *mut Sprite = &raw mut gSprites[sGrassSpriteId.get()];
            (*sprite).set_coordOffsetEnabled(1);
            (*sprite).oam.set_priority(2);
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn DestroyMewEmergingGrassSprite() {
    if sGrassSpriteId.get() != MAX_SPRITES {
        DestroySprite(&raw mut gSprites[sGrassSpriteId.get()]);
    }
}
unsafe fn ShouldMewMoveNorth(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaY.get() > 0
        && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0
    {
        sMewDirectionCandidates[index] = DIR_NORTH;
        return TRUE;
    }
    FALSE
}
unsafe fn ShouldMewMoveEast(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaX.get() < 0
        && CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) != 0
    {
        sMewDirectionCandidates[index] = DIR_EAST;
        return TRUE;
    }
    FALSE
}
unsafe fn ShouldMewMoveSouth(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaY.get() < 0
        && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) != 0
    {
        sMewDirectionCandidates[index] = DIR_SOUTH;
        return TRUE;
    }
    FALSE
}
unsafe fn ShouldMewMoveWest(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaX.get() > 0
        && CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0
    {
        sMewDirectionCandidates[index] = DIR_WEST;
        return TRUE;
    }
    FALSE
}
unsafe fn GetRandomMewDirectionCandidate(numDirections: u8) -> u8 {
    sMewDirectionCandidates[rem_i32(
        VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32,
        numDirections as i32,
    )]
}
