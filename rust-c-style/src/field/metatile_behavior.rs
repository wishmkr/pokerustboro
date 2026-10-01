//! Translated from `src/metatile_behavior.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sTileBitAttributes

const TILE_FLAG_HAS_ENCOUNTERS: i32 = 1;
const TILE_FLAG_SURFABLE: i32 = 2;

static sTileBitAttributes: Table<CArray<u8, 240>> =
    Table((&raw const crate::data::metatile_behavior::sTileBitAttributes).cast());

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsATile(metatileBehavior: u8) -> u8 {
    return TRUE;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEncounterTile(metatileBehavior: u8) -> u8 {
    if sTileBitAttributes[metatileBehavior] as i32 & TILE_FLAG_HAS_ENCOUNTERS != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpEast(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_JUMP_EAST {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpWest(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_JUMP_WEST {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpNorth(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_JUMP_NORTH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpSouth(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_JUMP_SOUTH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPokeGrass(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TALL_GRASS || metatileBehavior == MB_LONG_GRASS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSandOrDeepSand(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SAND || metatileBehavior == MB_DEEP_SAND {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDeepSand(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_DEEP_SAND {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsReflective(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_POND_WATER
        || metatileBehavior == MB_PUDDLE
        || metatileBehavior == MB_UNUSED_SOOTOPOLIS_DEEP_WATER_2
        || metatileBehavior == MB_ICE
        || metatileBehavior == MB_SOOTOPOLIS_DEEP_WATER
        || metatileBehavior == MB_REFLECTION_UNDER_BRIDGE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIce(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ICE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWarpDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ANIMATED_DOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PETALBURG_GYM_DOOR || metatileBehavior == MB_ANIMATED_DOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEscalator(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_UP_ESCALATOR || metatileBehavior == MB_DOWN_ESCALATOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsUnused04(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_UNUSED_04 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLadder(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_LADDER {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNonAnimDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_NON_ANIMATED_DOOR
        || metatileBehavior == MB_WATER_DOOR
        || metatileBehavior == MB_DEEP_SOUTH_WARP
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDeepSouthWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_DEEP_SOUTH_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior: u8) -> u8 {
    if sTileBitAttributes[metatileBehavior] as i32 & TILE_FLAG_SURFABLE != 0 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEastArrowWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_EAST_ARROW_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWestArrowWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WEST_ARROW_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNorthArrowWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_NORTH_ARROW_WARP
        || metatileBehavior == MB_STAIRS_OUTSIDE_ABANDONED_SHIP
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSouthArrowWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SOUTH_ARROW_WARP
        || metatileBehavior == MB_WATER_SOUTH_ARROW_WARP
        || metatileBehavior == MB_SHOAL_CAVE_ENTRANCE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsArrowWarp(metatileBehavior: u8) -> u8 {
    let mut isArrowWarp: u8 = FALSE;
    if MetatileBehavior_IsEastArrowWarp(metatileBehavior) != 0
        || MetatileBehavior_IsWestArrowWarp(metatileBehavior) != 0
        || MetatileBehavior_IsNorthArrowWarp(metatileBehavior) != 0
        || MetatileBehavior_IsSouthArrowWarp(metatileBehavior) != 0
    {
        isArrowWarp = TRUE;
    }
    return isArrowWarp;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsForcedMovementTile(metatileBehavior: u8) -> u8 {
    if metatileBehavior >= MB_WALK_EAST && metatileBehavior <= MB_TRICK_HOUSE_PUZZLE_8_FLOOR
        || metatileBehavior >= MB_EASTWARD_CURRENT && metatileBehavior <= MB_SOUTHWARD_CURRENT
        || metatileBehavior == MB_MUDDY_SLOPE
        || metatileBehavior == MB_CRACKED_FLOOR
        || metatileBehavior == MB_WATERFALL
        || metatileBehavior == MB_ICE
        || metatileBehavior == MB_SECRET_BASE_JUMP_MAT
        || metatileBehavior == MB_SECRET_BASE_SPIN_MAT
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIce_2(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ICE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrickHouseSlipperyFloor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TRICK_HOUSE_PUZZLE_8_FLOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsUnused05(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_UNUSED_05 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkNorth(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WALK_NORTH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkSouth(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WALK_SOUTH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkWest(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WALK_WEST {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkEast(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WALK_EAST {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNorthwardCurrent(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_NORTHWARD_CURRENT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSouthwardCurrent(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SOUTHWARD_CURRENT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWestwardCurrent(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WESTWARD_CURRENT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEastwardCurrent(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_EASTWARD_CURRENT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideNorth(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SLIDE_NORTH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideSouth(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SLIDE_SOUTH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideWest(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SLIDE_WEST {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideEast(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SLIDE_EAST {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCounter(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_COUNTER {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPlayerFacingTVScreen(
    metatileBehavior: u8,
    playerDir: u8,
) -> u8 {
    if playerDir != DIR_NORTH {
        return FALSE;
    } else if metatileBehavior == MB_TELEVISION {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPC(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PC {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCableBoxResults1(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_CABLE_BOX_RESULTS_1 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsOpenSecretBaseDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SPOT_RED_CAVE_OPEN
        || metatileBehavior == MB_SECRET_BASE_SPOT_BROWN_CAVE_OPEN
        || metatileBehavior == MB_SECRET_BASE_SPOT_YELLOW_CAVE_OPEN
        || metatileBehavior == MB_SECRET_BASE_SPOT_TREE_LEFT_OPEN
        || metatileBehavior == MB_SECRET_BASE_SPOT_SHRUB_OPEN
        || metatileBehavior == MB_SECRET_BASE_SPOT_BLUE_CAVE_OPEN
        || metatileBehavior == MB_SECRET_BASE_SPOT_TREE_RIGHT_OPEN
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseCave(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SPOT_RED_CAVE
        || metatileBehavior == MB_SECRET_BASE_SPOT_BROWN_CAVE
        || metatileBehavior == MB_SECRET_BASE_SPOT_YELLOW_CAVE
        || metatileBehavior == MB_SECRET_BASE_SPOT_BLUE_CAVE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseTree(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SPOT_TREE_LEFT as u8
        || metatileBehavior == MB_SECRET_BASE_SPOT_TREE_RIGHT as u8
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseShrub(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SPOT_SHRUB {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBasePC(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_PC {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRecordMixingSecretBasePC(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_REGISTER_PC {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseScenery1(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SCENERY {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseTrainerSpot(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_TRAINER_SPOT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseImpassable(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_IMPASSABLE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseDecorationBase(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_DECORATION_BASE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBasePoster(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_POSTER {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNormal(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_NORMAL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseNorthWall(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_NORTH_WALL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseScenery2(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SCENERY {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_HoldsSmallDecoration(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_HOLDS_SMALL_DECORATION {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_HoldsLargeDecoration(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_HOLDS_LARGE_DECORATION {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseHole(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_HOLE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseBalloon(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_BALLOON {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseBreakableDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_BREAKABLE_DOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseSoundMat(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SOUND_MAT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseGlitterMat(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_GLITTER_MAT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseSandOrnament(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SAND_ORNAMENT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseShieldOrToyTV(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_TV_SHIELD {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPlayerRoomPCOn(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PLAYER_ROOM_PC_ON {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_HasRipples(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_POND_WATER
        || metatileBehavior == MB_PUDDLE
        || metatileBehavior == MB_SOOTOPOLIS_DEEP_WATER
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPuddle(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PUDDLE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTallGrass(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TALL_GRASS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLongGrass(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_LONG_GRASS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBerryTreeSoil(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BERRY_TREE_SOIL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsAshGrass(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ASHGRASS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsFootprints(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_FOOTPRINTS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBridgeOverWater(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BRIDGE_OVER_OCEAN
        || metatileBehavior == MB_BRIDGE_OVER_POND_LOW
        || metatileBehavior == MB_BRIDGE_OVER_POND_MED
        || metatileBehavior == MB_BRIDGE_OVER_POND_HIGH
        || (metatileBehavior == MB_BRIDGE_OVER_POND_HIGH_EDGE_1
            || metatileBehavior == MB_BRIDGE_OVER_POND_HIGH_EDGE_2
            || metatileBehavior == MB_UNUSED_BRIDGE
            || metatileBehavior == MB_BIKE_BRIDGE_OVER_BARRIER)
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_GetBridgeType(metatileBehavior: u8) -> u8 {
    if metatileBehavior >= MB_BRIDGE_OVER_OCEAN && metatileBehavior <= MB_BRIDGE_OVER_POND_HIGH {
        return metatileBehavior - MB_BRIDGE_OVER_OCEAN;
    }
    if metatileBehavior >= MB_BRIDGE_OVER_POND_MED_EDGE_1
        && metatileBehavior <= MB_BRIDGE_OVER_POND_MED_EDGE_2
    {
        return BRIDGE_TYPE_POND_MED;
    }
    if metatileBehavior >= MB_BRIDGE_OVER_POND_HIGH_EDGE_1
        && metatileBehavior <= MB_BRIDGE_OVER_POND_HIGH_EDGE_2
    {
        return BRIDGE_TYPE_POND_HIGH;
    }
    return BRIDGE_TYPE_OCEAN;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBridgeOverWaterNoEdge(metatileBehavior: u8) -> u8 {
    if metatileBehavior >= MB_BRIDGE_OVER_OCEAN && metatileBehavior <= MB_BRIDGE_OVER_POND_HIGH {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLandWildEncounter(metatileBehavior: u8) -> u8 {
    if MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior) == FALSE
        && MetatileBehavior_IsEncounterTile(metatileBehavior) == TRUE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWaterWildEncounter(metatileBehavior: u8) -> u8 {
    if MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior) == TRUE
        && MetatileBehavior_IsEncounterTile(metatileBehavior) == TRUE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIndoorEncounter(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_INDOOR_ENCOUNTER {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMountain(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_MOUNTAIN_TOP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDiveable(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_INTERIOR_DEEP_WATER
        || metatileBehavior == MB_DEEP_WATER
        || metatileBehavior == MB_SOOTOPOLIS_DEEP_WATER
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsUnableToEmerge(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_NO_SURFACING || metatileBehavior == MB_SEAWEED_NO_SURFACING {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsShallowFlowingWater(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SHALLOW_WATER
        || metatileBehavior == MB_STAIRS_OUTSIDE_ABANDONED_SHIP
        || metatileBehavior == MB_SHOAL_CAVE_ENTRANCE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsThinIce(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_THIN_ICE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCrackedIce(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_CRACKED_ICE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDeepOrOceanWater(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_OCEAN_WATER
        || metatileBehavior == MB_INTERIOR_DEEP_WATER
        || metatileBehavior == MB_DEEP_WATER
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsUnusedSootopolisWater(
    metatileBehavior: u8,
) -> u8 {
    if metatileBehavior == MB_UNUSED_SOOTOPOLIS_DEEP_WATER
        || metatileBehavior == MB_UNUSED_SOOTOPOLIS_DEEP_WATER_2
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSurfableAndNotWaterfall(metatileBehavior: u8) -> u8 {
    if MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior) != 0
        && MetatileBehavior_IsWaterfall(metatileBehavior) == FALSE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEastBlocked(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_IMPASSABLE_EAST
        || metatileBehavior == MB_IMPASSABLE_NORTHEAST
        || metatileBehavior == MB_IMPASSABLE_SOUTHEAST
        || metatileBehavior == MB_IMPASSABLE_WEST_AND_EAST
        || metatileBehavior == MB_SECRET_BASE_BREAKABLE_DOOR
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWestBlocked(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_IMPASSABLE_WEST
        || metatileBehavior == MB_IMPASSABLE_NORTHWEST
        || metatileBehavior == MB_IMPASSABLE_SOUTHWEST
        || metatileBehavior == MB_IMPASSABLE_WEST_AND_EAST
        || metatileBehavior == MB_SECRET_BASE_BREAKABLE_DOOR
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNorthBlocked(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_IMPASSABLE_NORTH
        || metatileBehavior == MB_IMPASSABLE_NORTHEAST
        || metatileBehavior == MB_IMPASSABLE_NORTHWEST
        || metatileBehavior == MB_IMPASSABLE_SOUTH_AND_NORTH
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSouthBlocked(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_IMPASSABLE_SOUTH
        || metatileBehavior == MB_IMPASSABLE_SOUTHEAST
        || metatileBehavior == MB_IMPASSABLE_SOUTHWEST
        || metatileBehavior == MB_IMPASSABLE_SOUTH_AND_NORTH
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsShortGrass(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SHORT_GRASS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsHotSprings(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_HOT_SPRINGS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWaterfall(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_WATERFALL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsFortreeBridge(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_FORTREE_BRIDGE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogVerticalLogTop(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PACIFIDLOG_VERTICAL_LOG_TOP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogVerticalLogBottom(
    metatileBehavior: u8,
) -> u8 {
    if metatileBehavior == MB_PACIFIDLOG_VERTICAL_LOG_BOTTOM {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogHorizontalLogLeft(
    metatileBehavior: u8,
) -> u8 {
    if metatileBehavior == MB_PACIFIDLOG_HORIZONTAL_LOG_LEFT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogHorizontalLogRight(
    metatileBehavior: u8,
) -> u8 {
    if metatileBehavior == MB_PACIFIDLOG_HORIZONTAL_LOG_RIGHT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogLog(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PACIFIDLOG_VERTICAL_LOG_TOP
        || metatileBehavior == MB_PACIFIDLOG_VERTICAL_LOG_BOTTOM
        || metatileBehavior == MB_PACIFIDLOG_HORIZONTAL_LOG_LEFT
        || metatileBehavior == MB_PACIFIDLOG_HORIZONTAL_LOG_RIGHT
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrickHousePuzzleDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TRICK_HOUSE_PUZZLE_DOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRegionMap(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_REGION_MAP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsClosedSootopolisDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_CLOSED_SOOTOPOLIS_DOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSkyPillarClosedDoor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SKY_PILLAR_CLOSED_DOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRoulette(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ROULETTE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPokeblockFeeder(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_POKEBLOCK_FEEDER {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseJumpMat(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_JUMP_MAT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseSpinMat(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SECRET_BASE_SPIN_MAT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLavaridgeB1FWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_LAVARIDGE_GYM_B1F_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLavaridge1FWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_LAVARIDGE_GYM_1F_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsAquaHideoutWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_AQUA_HIDEOUT_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsUnionRoomWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BRIDGE_OVER_OCEAN {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMossdeepGymWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_MOSSDEEP_GYM_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSurfableFishableWater(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_POND_WATER
        || metatileBehavior == MB_OCEAN_WATER
        || metatileBehavior == MB_INTERIOR_DEEP_WATER
        || metatileBehavior == MB_DEEP_WATER
        || metatileBehavior == MB_SOOTOPOLIS_DEEP_WATER
        || (metatileBehavior == MB_EASTWARD_CURRENT
            || metatileBehavior == MB_WESTWARD_CURRENT
            || metatileBehavior == MB_NORTHWARD_CURRENT
            || metatileBehavior == MB_SOUTHWARD_CURRENT)
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMtPyreHole(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_MT_PYRE_HOLE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCrackedFloorHole(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_CRACKED_FLOOR_HOLE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCrackedFloor(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_CRACKED_FLOOR {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMuddySlope(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_MUDDY_SLOPE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBumpySlope(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BUMPY_SLOPE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIsolatedVerticalRail(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ISOLATED_VERTICAL_RAIL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIsolatedHorizontalRail(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_ISOLATED_HORIZONTAL_RAIL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsVerticalRail(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_VERTICAL_RAIL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsHorizontalRail(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_HORIZONTAL_RAIL {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSeaweed(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SEAWEED || metatileBehavior == MB_SEAWEED_NO_SURFACING {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRunningDisallowed(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_NO_RUNNING
        || metatileBehavior == MB_LONG_GRASS
        || metatileBehavior == MB_HOT_SPRINGS
        || MetatileBehavior_IsPacifidlogLog(metatileBehavior) != FALSE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCuttableGrass(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TALL_GRASS
        || metatileBehavior == MB_LONG_GRASS
        || metatileBehavior == MB_ASHGRASS
        || metatileBehavior == MB_LONG_GRASS_SOUTH_EDGE
    {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRunningShoesManual(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_RUNNING_SHOES_INSTRUCTION {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPictureBookShelf(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_PICTURE_BOOK_SHELF {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBookShelf(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BOOKSHELF {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPokeCenterBookShelf(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_POKEMON_CENTER_BOOKSHELF {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsVase(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_VASE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrashCan(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TRASH_CAN {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsShopShelf(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_SHOP_SHELF {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBlueprint(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BLUEPRINT {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBattlePyramidWarp(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_BATTLE_PYRAMID_WARP {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPlayerFacingWirelessBoxResults(
    tile: u8,
    playerDir: u8,
) -> u8 {
    if playerDir != CONNECTION_NORTH {
        return FALSE;
    } else if tile == MB_WIRELESS_BOX_RESULTS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCableBoxResults2(tile: u8, playerDir: u8) -> u8 {
    if playerDir != CONNECTION_NORTH {
        return FALSE;
    } else if tile == MB_CABLE_BOX_RESULTS_2 {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsQuestionnaire(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_QUESTIONNAIRE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLongGrass_Duplicate(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_LONG_GRASS {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLongGrassSouthEdge(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_LONG_GRASS_SOUTH_EDGE {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrainerHillTimer(metatileBehavior: u8) -> u8 {
    if metatileBehavior == MB_TRAINER_HILL_TIMER {
        return TRUE;
    } else {
        return FALSE;
    }
    #[allow(unreachable_code)]
    {
        return 0;
    }
}
