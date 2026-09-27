//! Translated from `src/metatile_behavior.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sTileBitAttributes
#[allow(unused_imports)]
use crate::data::metatile_behavior::*;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsATile(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        return 1u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEncounterTile(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((((&raw const sTileBitAttributes).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((metatileBehavior) as i32) as isize))
        .read()) as i32)
            & 1i32)
            != 0
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpEast(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 56i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpWest(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 57i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpNorth(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 58i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsJumpSouth(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 59i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPokeGrass(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 2i32) || (((metatileBehavior) as i32) == 3i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSandOrDeepSand(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 33i32) || (((metatileBehavior) as i32) == 6i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDeepSand(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 6i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsReflective(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((((metatileBehavior) as i32) == 16i32) || (((metatileBehavior) as i32) == 22i32))
            || (((metatileBehavior) as i32) == 26i32))
            || (((metatileBehavior) as i32) == 32i32))
            || (((metatileBehavior) as i32) == 20i32))
            || (((metatileBehavior) as i32) == 43i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIce(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 32i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWarpDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 105i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 141i32) || (((metatileBehavior) as i32) == 105i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEscalator(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 106i32) || (((metatileBehavior) as i32) == 107i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsUnused04(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 4i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLadder(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 97i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNonAnimDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((metatileBehavior) as i32) == 96i32) || (((metatileBehavior) as i32) == 108i32))
            || (((metatileBehavior) as i32) == 110i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDeepSouthWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 110i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((((&raw const sTileBitAttributes).cast::<u8>().cast_mut()).cast::<u8>())
            .wrapping_offset(((metatileBehavior) as i32) as isize))
        .read()) as i32)
            & 2i32)
            != 0
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEastArrowWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 98i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWestArrowWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 99i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNorthArrowWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 100i32) || (((metatileBehavior) as i32) == 27i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSouthArrowWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((metatileBehavior) as i32) == 101i32) || (((metatileBehavior) as i32) == 109i32))
            || (((metatileBehavior) as i32) == 28i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsArrowWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        let mut isArrowWarp: u8 = 0u8;
        if ((((MetatileBehavior_IsEastArrowWarp(metatileBehavior)) != 0)
            || ((MetatileBehavior_IsWestArrowWarp(metatileBehavior)) != 0))
            || ((MetatileBehavior_IsNorthArrowWarp(metatileBehavior)) != 0))
            || ((MetatileBehavior_IsSouthArrowWarp(metatileBehavior)) != 0)
        {
            isArrowWarp = 1u8;
        }
        return isArrowWarp;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsForcedMovementTile(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((((((((metatileBehavior) as i32) >= 64i32)
            && (((metatileBehavior) as i32) <= 72i32))
            || ((((metatileBehavior) as i32) >= 80i32)
                && (((metatileBehavior) as i32) <= 83i32)))
            || (((metatileBehavior) as i32) == 208i32))
            || (((metatileBehavior) as i32) == 210i32))
            || (((metatileBehavior) as i32) == 19i32))
            || (((metatileBehavior) as i32) == 32i32))
            || (((metatileBehavior) as i32) == 187i32))
            || (((metatileBehavior) as i32) == 188i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIce_2(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 32i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrickHouseSlipperyFloor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 72i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsUnused05(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 5i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkNorth(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 66i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkSouth(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 67i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkWest(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 65i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWalkEast(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 64i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNorthwardCurrent(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 82i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSouthwardCurrent(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 83i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWestwardCurrent(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 81i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEastwardCurrent(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 80i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideNorth(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 70i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideSouth(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 71i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideWest(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 69i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSlideEast(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 68i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCounter(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 128i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPlayerFacingTVScreen(
    metatileBehavior: u8,
    playerDir: u8,
) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        let mut playerDir = playerDir;
        if ((playerDir) as i32) != 2i32 {
            return 0u8;
        } else {
            if ((metatileBehavior) as i32) == 134i32 {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPC(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 131i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCableBoxResults1(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 132i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsOpenSecretBaseDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((((((metatileBehavior) as i32) == 145i32)
            || (((metatileBehavior) as i32) == 147i32))
            || (((metatileBehavior) as i32) == 149i32))
            || (((metatileBehavior) as i32) == 151i32))
            || (((metatileBehavior) as i32) == 153i32))
            || (((metatileBehavior) as i32) == 155i32))
            || (((metatileBehavior) as i32) == 157i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseCave(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((metatileBehavior) as i32) == 144i32) || (((metatileBehavior) as i32) == 146i32))
            || (((metatileBehavior) as i32) == 148i32))
            || (((metatileBehavior) as i32) == 154i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseTree(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 150i32) || (((metatileBehavior) as i32) == 156i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseShrub(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 152i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBasePC(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 176i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRecordMixingSecretBasePC(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 177i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseScenery1(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 178i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseTrainerSpot(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 179i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseImpassable(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 185i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseDecorationBase(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 198i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBasePoster(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 199i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNormal(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 0i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseNorthWall(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 183i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseScenery2(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 178i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_HoldsSmallDecoration(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 181i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_HoldsLargeDecoration(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 195i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseHole(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 194i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseBalloon(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 184i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseBreakableDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 190i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseSoundMat(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 189i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseGlitterMat(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 186i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseSandOrnament(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 191i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseShieldOrToyTV(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 196i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPlayerRoomPCOn(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 197i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_HasRipples(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((metatileBehavior) as i32) == 16i32) || (((metatileBehavior) as i32) == 22i32))
            || (((metatileBehavior) as i32) == 20i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPuddle(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 22i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTallGrass(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 2i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLongGrass(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 3i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBerryTreeSoil(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 160i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsAshGrass(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 36i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsFootprints(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 37i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBridgeOverWater(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((((metatileBehavior) as i32) == 112i32) || (((metatileBehavior) as i32) == 113i32))
            || (((metatileBehavior) as i32) == 114i32))
            || (((metatileBehavior) as i32) == 115i32))
            || ((((((metatileBehavior) as i32) == 124i32)
                || (((metatileBehavior) as i32) == 125i32))
                || (((metatileBehavior) as i32) == 126i32))
                || (((metatileBehavior) as i32) == 127i32))
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_GetBridgeType(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) >= 112i32) && (((metatileBehavior) as i32) <= 115i32) {
            return ((((metatileBehavior) as i32).wrapping_sub(112i32)) as u8);
        }
        if (((metatileBehavior) as i32) >= 122i32) && (((metatileBehavior) as i32) <= 123i32) {
            return 2u8;
        }
        if (((metatileBehavior) as i32) >= 124i32) && (((metatileBehavior) as i32) <= 125i32) {
            return 3u8;
        }
        return 0u8;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBridgeOverWaterNoEdge(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) >= 112i32) && (((metatileBehavior) as i32) <= 115i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLandWildEncounter(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior)) as i32) == 0i32)
            && (((MetatileBehavior_IsEncounterTile(metatileBehavior)) as i32) == 1i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWaterWildEncounter(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior)) as i32) == 1i32)
            && (((MetatileBehavior_IsEncounterTile(metatileBehavior)) as i32) == 1i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIndoorEncounter(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 11i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMountain(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 12i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDiveable(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((metatileBehavior) as i32) == 17i32) || (((metatileBehavior) as i32) == 18i32))
            || (((metatileBehavior) as i32) == 20i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsUnableToEmerge(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 25i32) || (((metatileBehavior) as i32) == 42i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsShallowFlowingWater(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((metatileBehavior) as i32) == 23i32) || (((metatileBehavior) as i32) == 27i32))
            || (((metatileBehavior) as i32) == 28i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsThinIce(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 38i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCrackedIce(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 39i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsDeepOrOceanWater(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((metatileBehavior) as i32) == 21i32) || (((metatileBehavior) as i32) == 17i32))
            || (((metatileBehavior) as i32) == 18i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Unref_MetatileBehavior_IsUnusedSootopolisWater(
    metatileBehavior: u8,
) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 24i32) || (((metatileBehavior) as i32) == 26i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSurfableAndNotWaterfall(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((MetatileBehavior_IsSurfableWaterOrUnderwater(metatileBehavior)) != 0)
            && (((MetatileBehavior_IsWaterfall(metatileBehavior)) as i32) == 0i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsEastBlocked(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((((metatileBehavior) as i32) == 48i32) || (((metatileBehavior) as i32) == 52i32))
            || (((metatileBehavior) as i32) == 54i32))
            || (((metatileBehavior) as i32) == 193i32))
            || (((metatileBehavior) as i32) == 190i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWestBlocked(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((((((metatileBehavior) as i32) == 49i32) || (((metatileBehavior) as i32) == 53i32))
            || (((metatileBehavior) as i32) == 55i32))
            || (((metatileBehavior) as i32) == 193i32))
            || (((metatileBehavior) as i32) == 190i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsNorthBlocked(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((metatileBehavior) as i32) == 50i32) || (((metatileBehavior) as i32) == 52i32))
            || (((metatileBehavior) as i32) == 53i32))
            || (((metatileBehavior) as i32) == 192i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSouthBlocked(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((metatileBehavior) as i32) == 51i32) || (((metatileBehavior) as i32) == 54i32))
            || (((metatileBehavior) as i32) == 55i32))
            || (((metatileBehavior) as i32) == 192i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsShortGrass(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 7i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsHotSprings(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 40i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsWaterfall(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 19i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsFortreeBridge(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 120i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogVerticalLogTop(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 116i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogVerticalLogBottom(
    metatileBehavior: u8,
) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 117i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogHorizontalLogLeft(
    metatileBehavior: u8,
) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 118i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogHorizontalLogRight(
    metatileBehavior: u8,
) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 119i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPacifidlogLog(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((metatileBehavior) as i32) == 116i32) || (((metatileBehavior) as i32) == 117i32))
            || (((metatileBehavior) as i32) == 118i32))
            || (((metatileBehavior) as i32) == 119i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrickHousePuzzleDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 140i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRegionMap(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 133i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsClosedSootopolisDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 139i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSkyPillarClosedDoor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 234i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRoulette(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 138i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPokeblockFeeder(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 135i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseJumpMat(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 187i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSecretBaseSpinMat(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 188i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLavaridgeB1FWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 41i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLavaridge1FWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 104i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsAquaHideoutWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 103i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsUnionRoomWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 112i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMossdeepGymWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 14i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSurfableFishableWater(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((((metatileBehavior) as i32) == 16i32) || (((metatileBehavior) as i32) == 21i32))
            || (((metatileBehavior) as i32) == 17i32))
            || (((metatileBehavior) as i32) == 18i32))
            || (((metatileBehavior) as i32) == 20i32))
            || ((((((metatileBehavior) as i32) == 80i32)
                || (((metatileBehavior) as i32) == 81i32))
                || (((metatileBehavior) as i32) == 82i32))
                || (((metatileBehavior) as i32) == 83i32))
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMtPyreHole(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 15i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCrackedFloorHole(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 102i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCrackedFloor(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 210i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsMuddySlope(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 208i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBumpySlope(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 209i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIsolatedVerticalRail(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 211i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsIsolatedHorizontalRail(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 212i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsVerticalRail(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 213i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsHorizontalRail(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 214i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsSeaweed(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((metatileBehavior) as i32) == 34i32) || (((metatileBehavior) as i32) == 42i32) {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRunningDisallowed(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((metatileBehavior) as i32) == 10i32) || (((metatileBehavior) as i32) == 3i32))
            || (((metatileBehavior) as i32) == 40i32))
            || (((MetatileBehavior_IsPacifidlogLog(metatileBehavior)) as i32) != 0i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCuttableGrass(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if (((((metatileBehavior) as i32) == 2i32) || (((metatileBehavior) as i32) == 3i32))
            || (((metatileBehavior) as i32) == 36i32))
            || (((metatileBehavior) as i32) == 9i32)
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsRunningShoesManual(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 142i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPictureBookShelf(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 224i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBookShelf(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 225i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPokeCenterBookShelf(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 226i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsVase(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 227i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrashCan(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 228i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsShopShelf(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 229i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBlueprint(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 230i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsBattlePyramidWarp(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 13i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsPlayerFacingWirelessBoxResults(
    tile: u8,
    playerDir: u8,
) -> u8 {
    unsafe {
        let mut tile = tile;
        let mut playerDir = playerDir;
        if ((playerDir) as i32) != 2i32 {
            return 0u8;
        } else {
            if ((tile) as i32) == 232i32 {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsCableBoxResults2(tile: u8, playerDir: u8) -> u8 {
    unsafe {
        let mut tile = tile;
        let mut playerDir = playerDir;
        if ((playerDir) as i32) != 2i32 {
            return 0u8;
        } else {
            if ((tile) as i32) == 231i32 {
                return 1u8;
            } else {
                return 0u8;
            }
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsQuestionnaire(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 143i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLongGrass_Duplicate(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 3i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsLongGrassSouthEdge(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 9i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn MetatileBehavior_IsTrainerHillTimer(metatileBehavior: u8) -> u8 {
    unsafe {
        let mut metatileBehavior = metatileBehavior;
        if ((metatileBehavior) as i32) == 233i32 {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
