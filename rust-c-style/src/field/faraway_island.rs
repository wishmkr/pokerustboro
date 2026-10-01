//! Translated from `src/faraway_island.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sFarawayIslandRockCoords

static sFarawayIslandRockCoords: Table<CArray<CArray<i16, 2>, 4>> =
    Table((&raw const crate::data::faraway_island::sFarawayIslandRockCoords).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sGrassSpriteId: u8 = 0;
pub(crate) static mut sPlayerToMewDeltaX: i16 = 0;
pub(crate) static mut sPlayerToMewDeltaY: i16 = 0;
pub(crate) static mut sMewDirectionCandidates: Aligned<CArray<u8, 4>> =
    Aligned(unsafe { zeroed() });

unsafe extern "C" {
    static gFieldEffectObjectTemplatePointers: CArray<*mut SpriteTemplate, 0>;
    static mut gObjectEvents: CArray<ObjectEvent, 16>;
    static mut gPlayerAvatar: PlayerAvatar;
    static mut gSaveBlock1Ptr: *mut SaveBlock1;
    static mut gSpecialVar_0x8004: u16;
    static mut gSpecialVar_Facing: u16;
    static gSpritePalette_GeneralFieldEffect1: SpritePalette;
    static mut gSprites: CArray<Sprite, 65>;
    fn CreateSpriteAtEnd(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn FlagGet(a0: u16) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn LoadSpritePalette(a0: *mut SpritePalette) -> u8;
    fn MapGridGetMetatileBehaviorAt(a0: i32, a1: i32) -> i32;
    fn MetatileBehavior_IsPokeGrass(a0: u8) -> u8;
    fn SetSpritePosToOffsetMapCoords(a0: *mut i16, a1: *mut i16, a2: i16, a3: i16);
    fn SpriteCallbackDummy(a0: *mut Sprite);
    fn TryGetObjectEventIdByLocalIdAndMap(a0: u8, a1: u8, a2: u8, a3: *mut u8) -> u8;
    fn UpdateSpritePaletteWithWeather(a0: u8);
    fn VarGet(a0: u16) -> u16;
    fn VarSet(a0: u16, a1: u16) -> u8;
}

pub(crate) unsafe extern "C" fn GetMewObjectEventId() -> u8 {
    let mut objectEventId: u8 = 0;
    TryGetObjectEventIdByLocalIdAndMap(
        LOCALID_FARAWAY_ISLAND_MEW,
        (*gSaveBlock1Ptr).location.mapNum as u8,
        (*gSaveBlock1Ptr).location.mapGroup as u8,
        &raw mut objectEventId,
    );
    return objectEventId;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetMewMoveDirection() -> u32 {
    let mut i: u8 = 0;
    let mut mewSafeFromTrap: i32 = 0;
    let mut mew: *mut ObjectEvent = &raw mut gObjectEvents[GetMewObjectEventId()];
    sPlayerToMewDeltaX =
        gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x - (*mew).currentCoords.x;
    sPlayerToMewDeltaY =
        gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y - (*mew).currentCoords.y;
    i = 0;
    while i < 4 {
        sMewDirectionCandidates[i] = DIR_NONE;
        i += 1;
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
    i = 0;
    while i < 4 {
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
                if sPlayerToMewDeltaX > 0 {
                    if (*mew).currentCoords.x as i32 + 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x as i32
                    {
                        if CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y)
                            != 0
                        {
                            return DIR_EAST as u32;
                        }
                    }
                } else if sPlayerToMewDeltaX < 0 {
                    if (*mew).currentCoords.x as i32 - 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x as i32
                    {
                        if CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y)
                            != 0
                        {
                            return DIR_WEST as u32;
                        }
                    }
                }
                if (*mew).currentCoords.x
                    == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.x
                {
                    if sPlayerToMewDeltaY > 0 {
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
                if sPlayerToMewDeltaY > 0 {
                    if (*mew).currentCoords.y as i32 + 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y as i32
                    {
                        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1)
                            != 0
                        {
                            return DIR_SOUTH as u32;
                        }
                    }
                } else if sPlayerToMewDeltaY < 0 {
                    if (*mew).currentCoords.y as i32 - 1
                        == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y as i32
                    {
                        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1)
                            != 0
                        {
                            return DIR_NORTH as u32;
                        }
                    }
                }
                if (*mew).currentCoords.y
                    == gObjectEvents[gPlayerAvatar.objectEventId].previousCoords.y
                {
                    if sPlayerToMewDeltaX > 0 {
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
        i += 1;
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
    if sPlayerToMewDeltaY == 0 {
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y > (*mew).currentCoords.y {
            if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0 {
                return DIR_NORTH as u32;
            }
        }
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y < (*mew).currentCoords.y {
            if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) != 0 {
                return DIR_SOUTH as u32;
            }
        }
        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0 {
            return DIR_NORTH as u32;
        }
        if CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) != 0 {
            return DIR_SOUTH as u32;
        }
    }
    if sPlayerToMewDeltaX == 0 {
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x > (*mew).currentCoords.x {
            if CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0 {
                return DIR_WEST as u32;
            }
        }
        if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x < (*mew).currentCoords.x {
            if CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) != 0 {
                return DIR_EAST as u32;
            }
        }
        if CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) != 0 {
            return DIR_EAST as u32;
        }
        if CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0 {
            return DIR_WEST as u32;
        }
    }
    return GetValidMewMoveDirection(DIR_NONE) as u32;
}
pub(crate) unsafe extern "C" fn CanMewMoveToCoords(x: i16, y: i16) -> u8 {
    if gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.x == x
        && gObjectEvents[gPlayerAvatar.objectEventId].currentCoords.y == y
    {
        return FALSE;
    }
    return MetatileBehavior_IsPokeGrass(MapGridGetMetatileBehaviorAt(x as i32, y as i32) as u8);
}
pub(crate) unsafe extern "C" fn GetValidMewMoveDirection(ignoredDir: u8) -> u8 {
    let mut i: u8 = 0;
    let mut count: u8 = 0;
    let mut mew: *mut ObjectEvent = &raw mut gObjectEvents[GetMewObjectEventId()];
    i = 0;
    while i < 4 {
        sMewDirectionCandidates[i] = DIR_NONE;
        i += 1;
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
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UpdateFarawayIslandStepCounter() {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ObjectEventIsFarawayIslandMew(objectEvent: *mut ObjectEvent) -> u8 {
    if (*gSaveBlock1Ptr).location.mapNum == 57 && (*gSaveBlock1Ptr).location.mapGroup == 26 {
        if (*objectEvent).graphicsId == OBJ_EVENT_GFX_MEW {
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn IsMewPlayingHideAndSeek() -> u8 {
    if (*gSaveBlock1Ptr).location.mapNum == 57 && (*gSaveBlock1Ptr).location.mapGroup == 26 {
        if FlagGet(FLAG_CAUGHT_MEW) != TRUE && FlagGet(FLAG_HIDE_MEW) != TRUE {
            return TRUE;
        }
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ShouldMewShakeGrass(objectEvent: *mut ObjectEvent) -> u8 {
    if VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) != 0xFFFF
        && VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32 % 4 == 0
    {
        return TRUE;
    }
    return FALSE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetMewAboveGrass() {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut mew: *mut ObjectEvent = &raw mut gObjectEvents[GetMewObjectEventId()];
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
        LoadSpritePalette((&raw const gSpritePalette_GeneralFieldEffect1).cast_mut());
        UpdateSpritePaletteWithWeather(IndexOfSpritePaletteTag(
            gSpritePalette_GeneralFieldEffect1.tag,
        ));
        x = (*mew).currentCoords.x;
        y = (*mew).currentCoords.y;
        SetSpritePosToOffsetMapCoords(&raw mut x, &raw mut y, 8, 8);
        sGrassSpriteId = CreateSpriteAtEnd(
            gFieldEffectObjectTemplatePointers[15],
            x,
            y,
            gSprites[(*mew).spriteId].subpriority - 1,
        );
        if sGrassSpriteId != MAX_SPRITES {
            let mut sprite: *mut Sprite = &raw mut gSprites[sGrassSpriteId];
            (*sprite).set_coordOffsetEnabled(1);
            (*sprite).oam.set_priority(2);
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyMewEmergingGrassSprite() {
    if sGrassSpriteId != MAX_SPRITES {
        DestroySprite(&raw mut gSprites[sGrassSpriteId]);
    }
}
pub(crate) unsafe extern "C" fn ShouldMewMoveNorth(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaY > 0
        && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y - 1) != 0
    {
        sMewDirectionCandidates[index] = DIR_NORTH;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShouldMewMoveEast(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaX < 0
        && CanMewMoveToCoords((*mew).currentCoords.x + 1, (*mew).currentCoords.y) != 0
    {
        sMewDirectionCandidates[index] = DIR_EAST;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShouldMewMoveSouth(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaY < 0
        && CanMewMoveToCoords((*mew).currentCoords.x, (*mew).currentCoords.y + 1) != 0
    {
        sMewDirectionCandidates[index] = DIR_SOUTH;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn ShouldMewMoveWest(mew: *mut ObjectEvent, index: u8) -> u8 {
    if sPlayerToMewDeltaX > 0
        && CanMewMoveToCoords((*mew).currentCoords.x - 1, (*mew).currentCoords.y) != 0
    {
        sMewDirectionCandidates[index] = DIR_WEST;
        return TRUE;
    }
    return FALSE;
}
pub(crate) unsafe extern "C" fn GetRandomMewDirectionCandidate(numDirections: u8) -> u8 {
    return sMewDirectionCandidates[rem_i32(
        VarGet(VAR_FARAWAY_ISLAND_STEP_COUNTER) as i32,
        numDirections as i32,
    )];
}
