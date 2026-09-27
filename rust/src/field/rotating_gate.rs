//! Translated from `src/rotating_gate.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sRotatingGate_FortreePuzzleConfig sRotatingGate_TrickHousePuzzleConfig sRotatingGateTiles_1 sRotatingGateTiles_2 sRotatingGateTiles_3 sRotatingGateTiles_4 sRotatingGateTiles_5 sRotatingGateTiles_6 sRotatingGateTiles_7 sRotatingGateTiles_8 sOamData_RotatingGateLarge sOamData_RotatingGateRegular sRotatingGatesGraphicsTable sSpriteAnim_RotatingGateLarge sSpriteAnim_RotatingGateRegular sSpriteAnimTable_RotatingGateLarge sSpriteAnimTable_RotatingGateRegular sSpriteAffineAnim_Rotated0 sSpriteAffineAnim_Rotated90 sSpriteAffineAnim_Rotated180 sSpriteAffineAnim_Rotated270 sSpriteAffineAnim_RotatingClockwise0to90 sSpriteAffineAnim_RotatingClockwise90to180 sSpriteAffineAnim_RotatingClockwise180to270 sSpriteAffineAnim_RotatingClockwise270to360 sSpriteAffineAnim_RotatingAnticlockwise360to270 sSpriteAffineAnim_RotatingAnticlockwise270to180 sSpriteAffineAnim_RotatingAnticlockwise180to90 sSpriteAffineAnim_RotatingAnticlockwise90to0 sSpriteAffineAnim_RotatingClockwise0to90Faster sSpriteAffineAnim_RotatingClockwise90to180Faster sSpriteAffineAnim_RotatingClockwise180to270Faster sSpriteAffineAnim_RotatingClockwise270to360Faster sSpriteAffineAnim_RotatingAnticlockwise360to270Faster sSpriteAffineAnim_RotatingAnticlockwise270to180Faster sSpriteAffineAnim_RotatingAnticlockwise180to90Faster sSpriteAffineAnim_RotatingAnticlockwise90to0Faster sSpriteAffineAnimTable_RotatingGate sSpriteTemplate_RotatingGateLarge sSpriteTemplate_RotatingGateRegular sRotatingGate_RotationInfoNorth sRotatingGate_RotationInfoSouth sRotatingGate_RotationInfoWest sRotatingGate_RotationInfoEast sRotatingGate_ArmPositionsClockwiseRotation sRotatingGate_ArmPositionsAntiClockwiseRotation sRotatingGate_ArmLayout
#[allow(unused_imports)]
use crate::data::rotating_gate::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRotatingGate_GateSpriteIds: crate::ffi::Align4<[u8; 12]> =
    crate::ffi::Align4([0; 12]);
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRotatingGate_PuzzleConfig: *mut u8 = core::ptr::null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRotatingGate_PuzzleCount: u8 = 0u8;

unsafe extern "C" {
    static mut gSaveBlock1Ptr: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn GetMapCoordsFromSpritePos(a0: i16, a1: i16, a2: *mut i16, a3: *mut i16);
    fn GetPlayerSpeed() -> i16;
    fn GetVarPointer(a0: u16) -> *mut u16;
    fn LoadSpriteSheets(a0: *mut u8);
    fn MapGridGetCollisionAt(a0: i32, a1: i32) -> u8;
    fn PlaySE(a0: u16);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
}

pub(crate) unsafe extern "C" fn GetCurrentMapRotatingGatePuzzleType() -> i32 {
    unsafe {
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 12i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 1i32)
        {
            return 1i32;
        }
        if ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
            .cast::<i8>())
        .read()) as i32)
            == 29i32)
            && ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).wrapping_add(4))
                .wrapping_add(1)
                .cast::<i8>())
            .read()) as i32)
                == 8i32)
        {
            return 2i32;
        }
        return 0i32;
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_ResetAllGateOrientations() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut ptr: *mut u8 = (GetVarPointer(16384u16)).cast::<u8>();
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sRotatingGate_PuzzleCount)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((ptr).wrapping_offset((i) as isize)).write(
                        (((((&raw mut sRotatingGate_PuzzleConfig)
                            .cast::<u8>()
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset((i) as isize * 8))
                        .wrapping_add(5))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_GetGateOrientation(gateId: u8) -> i32 {
    unsafe {
        let mut gateId = gateId;
        return (((((GetVarPointer(16384u16)).cast::<u8>())
            .wrapping_offset(((gateId) as i32) as isize))
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_SetGateOrientation(gateId: u8, orientation: u8) {
    unsafe {
        let mut gateId = gateId;
        let mut orientation = orientation;
        (((GetVarPointer(16384u16)).cast::<u8>()).wrapping_offset(((gateId) as i32) as isize))
            .write(orientation);
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_RotateInDirection(gateId: u8, rotationDirection: u32) {
    unsafe {
        let mut gateId = gateId;
        let mut rotationDirection = rotationDirection;
        let mut orientation: u8 = ((RotatingGate_GetGateOrientation(gateId)) as u8);
        if rotationDirection == 1u32 {
            if (orientation) != 0 {
                orientation = (orientation).wrapping_sub(1);
            } else {
                orientation = 3u8;
            }
        } else {
            orientation = (orientation).wrapping_add(1);
            orientation = ((crate::c::rem_i32(((orientation) as i32), 4i32)) as u8);
        }
        RotatingGate_SetGateOrientation(gateId, orientation);
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_LoadPuzzleConfig() {
    unsafe {
        let mut puzzleType: i32 = GetCurrentMapRotatingGatePuzzleType();
        let mut i: u32 = 0u32;
        'l1: {
            let __sw1 = puzzleType;
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                ((&raw mut sRotatingGate_PuzzleConfig)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(
                    ((&raw const sRotatingGate_FortreePuzzleConfig)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((&raw mut sRotatingGate_PuzzleCount)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(((crate::c::div_u32(64u32, 8u32)) as u8));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut sRotatingGate_PuzzleConfig)
                    .cast::<u8>()
                    .cast::<*mut u8>())
                .write(
                    ((&raw const sRotatingGate_TrickHousePuzzleConfig)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((&raw mut sRotatingGate_PuzzleCount)
                    .cast::<u8>()
                    .cast::<u8>())
                .write(((crate::c::div_u32(88u32, 8u32)) as u8));
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                return;
            }
        }
        {
            i = 0u32;
            'l2: loop {
                if !(i < 11u32) {
                    break 'l2;
                }
                'l3: {
                    ((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>()).cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(64u8);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_CreateGatesWithinViewport(deltaX: i16, deltaY: i16) {
    unsafe {
        let mut deltaX = deltaX;
        let mut deltaY = deltaY;
        let mut i: u8 = 0u8;
        let mut x: i16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
            .read()) as i32)
            .wrapping_sub(2i32)) as i16);
        let mut x2: i16 = (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add(15i32))
        .wrapping_add(2i32)) as i16);
        let mut y: i16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_sub(2i32)) as i16);
        let mut y2: i16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add(14i32)) as i16);
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32)
                    < ((((&raw mut sRotatingGate_PuzzleCount)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut x3: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    let mut y3: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((i) as i32) as isize * 8))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    if ((((((y) as i32) <= ((y3) as i32)) && (((y2) as i32) >= ((y3) as i32)))
                        && (((x) as i32) <= ((x3) as i32)))
                        && (((x2) as i32) >= ((x3) as i32)))
                        && (((((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>())
                            .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize))
                        .read()) as i32)
                            == 64i32)
                    {
                        ((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>()).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(RotatingGate_CreateGate(i, deltaX, deltaY));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_CreateGate(
    gateId: u8,
    deltaX: i16,
    deltaY: i16,
) -> u8 {
    unsafe {
        let mut gateId = gateId;
        let mut deltaX = deltaX;
        let mut deltaY = deltaY;
        let mut sprite: *mut u8 = core::ptr::null_mut();
        let mut template = crate::ffi::Align4([0u8; 24]);
        let mut spriteId: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut gate: *mut u8 = (((&raw mut sRotatingGate_PuzzleConfig)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((gateId) as i32) as isize * 8);
        if (((((gate).wrapping_add(4)).read()) as i32) == 0i32)
            || (((((gate).wrapping_add(4)).read()) as i32) == 4i32)
        {
            (&raw mut template)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (&raw const sSpriteTemplate_RotatingGateRegular)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                );
        } else {
            (&raw mut template)
                .cast::<u8>()
                .cast::<crate::c::Rec4<24>>()
                .write_unaligned(
                    (&raw const sSpriteTemplate_RotatingGateLarge)
                        .cast::<u8>()
                        .cast_mut()
                        .cast::<crate::c::Rec4<24>>()
                        .read_unaligned(),
                );
        }
        (((&raw mut template).cast::<u8>()).cast::<u16>())
            .write(((((((gate).wrapping_add(4)).read()) as i32).wrapping_add(4864i32)) as u16));
        spriteId = CreateSprite((&raw mut template).cast::<u8>(), 0i16, 0i16, 148u8);
        if ((spriteId) as i32) == 64i32 {
            return 64u8;
        }
        x = ((((((gate).cast::<i16>()).read()) as i32).wrapping_add(7i32)) as i16);
        y = ((((((gate).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_add(7i32)) as i16);
        sprite =
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((gateId) as i16));
        crate::c::bf_write((sprite).wrapping_add(62), 1, 1, (1u16) as i32);
        GetMapCoordsFromSpritePos(
            ((((x) as i32).wrapping_add(((deltaX) as i32))) as i16),
            ((((y) as i32).wrapping_add(((deltaY) as i32))) as i16),
            (sprite).wrapping_add(32).cast::<i16>(),
            (sprite).wrapping_add(34).cast::<i16>(),
        );
        RotatingGate_HideGatesOutsideViewport(sprite);
        StartSpriteAffineAnim(sprite, ((RotatingGate_GetGateOrientation(gateId)) as u8));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCallback_RotatingGate(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut affineAnimation: u8 = 0u8;
        let mut rotationDirection: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8);
        let mut orientation: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
        RotatingGate_HideGatesOutsideViewport(sprite);
        if ((rotationDirection) as i32) == 1i32 {
            affineAnimation = ((((orientation) as i32).wrapping_add(4i32)) as u8);
            if ((GetPlayerSpeed()) as i32) != 1i32 {
                affineAnimation = ((((affineAnimation) as i32).wrapping_add(8i32)) as u8);
            }
            PlaySE(48u16);
            StartSpriteAffineAnim(sprite, affineAnimation);
        } else {
            if ((rotationDirection) as i32) == 2i32 {
                affineAnimation = ((((orientation) as i32).wrapping_add(8i32)) as u8);
                if ((GetPlayerSpeed()) as i32) != 1i32 {
                    affineAnimation = ((((affineAnimation) as i32).wrapping_add(8i32)) as u8);
                }
                PlaySE(48u16);
                StartSpriteAffineAnim(sprite, affineAnimation);
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_HideGatesOutsideViewport(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut x2: i16 = 0i16;
        let mut y2: i16 = 0i16;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        x = ((((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
        .wrapping_add(((((sprite).wrapping_add(40).cast::<i8>()).read()) as i32)))
        .wrapping_add(((((&raw mut gSpriteCoordOffsetX).cast::<i16>()).read()) as i32)))
            as u16);
        y = ((((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
        .wrapping_add(((((sprite).wrapping_add(41).cast::<i8>()).read()) as i32)))
        .wrapping_add(((((&raw mut gSpriteCoordOffsetY).cast::<i16>()).read()) as i32)))
            as u16);
        x2 = ((((x) as i32).wrapping_add(64i32)) as i16);
        y2 = ((((y) as i32).wrapping_add(64i32)) as i16);
        if ((((x) as i16) as i32) > 255i32) || (((x2) as i32) < (-16i32)) {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if ((((y) as i16) as i32) > 175i32) || (((y2) as i32) < (-16i32)) {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn LoadRotatingGatePics() {
    unsafe {
        LoadSpriteSheets(
            ((&raw const sRotatingGatesGraphicsTable)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_DestroyGatesOutsideViewport() {
    unsafe {
        let mut i: i32 = 0i32;
        let mut x: i16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read()).cast::<i16>())
            .read()) as i32)
            .wrapping_sub(2i32)) as i16);
        let mut x2: i16 = (((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add(15i32))
        .wrapping_add(2i32)) as i16);
        let mut y: i16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_sub(2i32)) as i16);
        let mut y2: i16 = ((((((((&raw mut gSaveBlock1Ptr).cast::<*mut u8>()).read())
            .wrapping_add(2)
            .cast::<i16>())
        .read()) as i32)
            .wrapping_add(14i32)) as i16);
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sRotatingGate_PuzzleCount)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut xGate: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    let mut yGate: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    if ((((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>()).cast::<u8>())
                        .wrapping_offset((i) as isize))
                    .read()) as i32)
                        == 64i32
                    {
                        break 'l2;
                    }
                    if (((((xGate) as i32) < ((x) as i32)) || (((xGate) as i32) > ((x2) as i32)))
                        || (((yGate) as i32) < ((y) as i32)))
                        || (((yGate) as i32) > ((y2) as i32))
                    {
                        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(
                                ((((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>())
                                    .cast::<u8>())
                                .wrapping_offset((i) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            );
                        FreeSpriteOamMatrix(sprite);
                        DestroySprite(sprite);
                        ((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>()).cast::<u8>())
                            .wrapping_offset((i) as isize))
                        .write(64u8);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_CanRotate(gateId: u8, rotationDirection: i32) -> i32 {
    unsafe {
        let mut gateId = gateId;
        let mut rotationDirection = rotationDirection;
        let mut armPos: *mut u8 = core::ptr::null_mut();
        let mut orientation: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut shape: u8 = 0u8;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        if rotationDirection == 1i32 {
            armPos = ((&raw const sRotatingGate_ArmPositionsAntiClockwiseRotation)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>();
        } else {
            if rotationDirection == 2i32 {
                armPos = ((&raw const sRotatingGate_ArmPositionsClockwiseRotation)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>();
            } else {
                return 0i32;
            }
        }
        orientation = ((RotatingGate_GetGateOrientation(gateId)) as u8);
        shape = (((((&raw mut sRotatingGate_PuzzleConfig)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((gateId) as i32) as isize * 8))
        .wrapping_add(4))
        .read();
        x = (((((((((&raw mut sRotatingGate_PuzzleConfig)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((gateId) as i32) as isize * 8))
        .cast::<i16>())
        .read()) as i32)
            .wrapping_add(7i32)) as i16);
        y = (((((((((&raw mut sRotatingGate_PuzzleConfig)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((gateId) as i32) as isize * 8))
        .wrapping_add(2)
        .cast::<i16>())
        .read()) as i32)
            .wrapping_add(7i32)) as i16);
        {
            i = 0i32;
            'l1: loop {
                if !(i <= 3i32) {
                    break 'l1;
                }
                'l2: {
                    {
                        j = 0i32;
                        'l3: loop {
                            if !(j < 2i32) {
                                break 'l3;
                            }
                            'l4: {
                                let mut armIndex: u8 = ((((2i32).wrapping_mul(crate::c::rem_i32(
                                    ((orientation) as i32).wrapping_add(i),
                                    4i32,
                                )))
                                .wrapping_add(j))
                                    as u8);
                                if (((((((&raw const sRotatingGate_ArmLayout)
                                    .cast::<u8>()
                                    .cast_mut())
                                .cast::<u8>())
                                .wrapping_offset(((shape) as i32) as isize * 8))
                                .cast::<u8>())
                                .wrapping_offset(
                                    (((2i32).wrapping_mul(i)).wrapping_add(j)) as isize,
                                ))
                                .read())
                                    != 0
                                {
                                    if ((MapGridGetCollisionAt(
                                        ((x) as i32).wrapping_add(
                                            (((((armPos).wrapping_offset(
                                                ((armIndex) as i32) as isize * 4,
                                            ))
                                            .cast::<i8>())
                                            .read())
                                                as i32),
                                        ),
                                        ((y) as i32).wrapping_add(
                                            (((((armPos).wrapping_offset(
                                                ((armIndex) as i32) as isize * 4,
                                            ))
                                            .wrapping_add(1)
                                            .cast::<i8>())
                                            .read())
                                                as i32),
                                        ),
                                    )) as i32)
                                        == 1i32
                                    {
                                        return 0i32;
                                    }
                                }
                            }
                            j = (j).wrapping_add(1);
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 1i32;
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_HasArm(gateId: u8, armInfo: u8) -> i32 {
    unsafe {
        let mut gateId = gateId;
        let mut armInfo = armInfo;
        let mut arm: i32 = crate::c::div_i32(((armInfo) as i32), 2i32);
        let mut isLongArm: i32 = crate::c::rem_i32(((armInfo) as i32), 2i32);
        let mut armOrientation: i8 = ((crate::c::rem_i32(
            ((arm).wrapping_sub(RotatingGate_GetGateOrientation(gateId))).wrapping_add(4i32),
            4i32,
        )) as i8);
        let mut shape: i32 = (((((((&raw mut sRotatingGate_PuzzleConfig)
            .cast::<u8>()
            .cast::<*mut u8>())
        .read())
        .wrapping_offset(((gateId) as i32) as isize * 8))
        .wrapping_add(4))
        .read()) as i32);
        return ((((((((&raw const sRotatingGate_ArmLayout).cast::<u8>().cast_mut())
            .cast::<u8>())
        .wrapping_offset((shape) as isize * 8))
        .cast::<u8>())
        .wrapping_offset(
            ((((armOrientation) as i32).wrapping_mul(2i32)).wrapping_add(isLongArm)) as isize,
        ))
        .read()) as i32);
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_TriggerRotationAnimation(
    gateId: u8,
    rotationDirection: i32,
) {
    unsafe {
        let mut gateId = gateId;
        let mut rotationDirection = rotationDirection;
        if ((((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>()).cast::<u8>())
            .wrapping_offset(((gateId) as i32) as isize))
        .read()) as i32)
            != 64i32
        {
            let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((&raw mut sRotatingGate_GateSpriteIds).cast::<u8>()).cast::<u8>())
                    .wrapping_offset(((gateId) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((rotationDirection) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((RotatingGate_GetGateOrientation(gateId)) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn RotatingGate_GetRotationInfo(direction: u8, x: i16, y: i16) -> u8 {
    unsafe {
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut ptr: *mut u8 = core::ptr::null_mut();
        if ((direction) as i32) == 2i32 {
            ptr = ((&raw const sRotatingGate_RotationInfoNorth)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>();
        } else {
            if ((direction) as i32) == 1i32 {
                ptr = ((&raw const sRotatingGate_RotationInfoSouth)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>();
            } else {
                if ((direction) as i32) == 3i32 {
                    ptr = ((&raw const sRotatingGate_RotationInfoWest)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>();
                } else {
                    if ((direction) as i32) == 4i32 {
                        ptr = ((&raw const sRotatingGate_RotationInfoEast)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>();
                    } else {
                        return 255u8;
                    }
                }
            }
        }
        return ((ptr).wrapping_offset(
            ((((y) as i32).wrapping_mul(4i32)).wrapping_add(((x) as i32))) as isize,
        ))
        .read();
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RotatingGate_InitPuzzle() {
    unsafe {
        if (GetCurrentMapRotatingGatePuzzleType()) != 0 {
            RotatingGate_LoadPuzzleConfig();
            RotatingGate_ResetAllGateOrientations();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RotatingGatePuzzleCameraUpdate(deltaX: i16, deltaY: i16) {
    unsafe {
        let mut deltaX = deltaX;
        let mut deltaY = deltaY;
        if (GetCurrentMapRotatingGatePuzzleType()) != 0 {
            RotatingGate_CreateGatesWithinViewport(deltaX, deltaY);
            RotatingGate_DestroyGatesOutsideViewport();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn RotatingGate_InitPuzzleAndGraphics() {
    unsafe {
        if (GetCurrentMapRotatingGatePuzzleType()) != 0 {
            LoadRotatingGatePics();
            RotatingGate_LoadPuzzleConfig();
            RotatingGate_CreateGatesWithinViewport(0i16, 0i16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForRotatingGatePuzzleCollision(direction: u8, x: i16, y: i16) -> u32 {
    unsafe {
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut i: i32 = 0i32;
        if !((GetCurrentMapRotatingGatePuzzleType()) != 0) {
            return 0u32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sRotatingGate_PuzzleCount)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut gateX: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    let mut gateY: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    if (((((gateX) as i32).wrapping_sub(2i32) <= ((x) as i32))
                        && (((x) as i32) <= ((gateX) as i32).wrapping_add(1i32)))
                        && (((gateY) as i32).wrapping_sub(2i32) <= ((y) as i32)))
                        && (((y) as i32) <= ((gateY) as i32).wrapping_add(1i32))
                    {
                        let mut centerX: i16 = (((((x) as i32).wrapping_sub(((gateX) as i32)))
                            .wrapping_add(2i32))
                            as i16);
                        let mut centerY: i16 = (((((y) as i32).wrapping_sub(((gateY) as i32)))
                            .wrapping_add(2i32))
                            as i16);
                        let mut rotationInfo: u8 =
                            RotatingGate_GetRotationInfo(direction, centerX, centerY);
                        if ((rotationInfo) as i32) != 255i32 {
                            let mut rotationDirection: u8 =
                                (((((rotationInfo) as i32) & 240i32) >> 4) as u8);
                            let mut armInfo: u8 = ((((rotationInfo) as i32) & 15i32) as u8);
                            if (RotatingGate_HasArm(((i) as u8), armInfo)) != 0 {
                                if (RotatingGate_CanRotate(
                                    ((i) as u8),
                                    ((rotationDirection) as i32),
                                )) != 0
                                {
                                    RotatingGate_TriggerRotationAnimation(
                                        ((i) as u8),
                                        ((rotationDirection) as i32),
                                    );
                                    RotatingGate_RotateInDirection(
                                        ((i) as u8),
                                        ((rotationDirection) as u32),
                                    );
                                    return 0u32;
                                }
                                return 1u32;
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CheckForRotatingGatePuzzleCollisionWithoutAnimation(
    direction: u8,
    x: i16,
    y: i16,
) -> u32 {
    unsafe {
        let mut direction = direction;
        let mut x = x;
        let mut y = y;
        let mut i: i32 = 0i32;
        if !((GetCurrentMapRotatingGatePuzzleType()) != 0) {
            return 0u32;
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i
                    < ((((&raw mut sRotatingGate_PuzzleCount)
                        .cast::<u8>()
                        .cast::<u8>())
                    .read()) as i32))
                {
                    break 'l1;
                }
                'l2: {
                    let mut gateX: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    let mut gateY: i16 = (((((((((&raw mut sRotatingGate_PuzzleConfig)
                        .cast::<u8>()
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset((i) as isize * 8))
                    .wrapping_add(2)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(7i32)) as i16);
                    if (((((gateX) as i32).wrapping_sub(2i32) <= ((x) as i32))
                        && (((x) as i32) <= ((gateX) as i32).wrapping_add(1i32)))
                        && (((gateY) as i32).wrapping_sub(2i32) <= ((y) as i32)))
                        && (((y) as i32) <= ((gateY) as i32).wrapping_add(1i32))
                    {
                        let mut centerX: i16 = (((((x) as i32).wrapping_sub(((gateX) as i32)))
                            .wrapping_add(2i32))
                            as i16);
                        let mut centerY: i16 = (((((y) as i32).wrapping_sub(((gateY) as i32)))
                            .wrapping_add(2i32))
                            as i16);
                        let mut rotationInfo: u8 =
                            RotatingGate_GetRotationInfo(direction, centerX, centerY);
                        if ((rotationInfo) as i32) != 255i32 {
                            let mut rotationDirection: u8 =
                                (((((rotationInfo) as i32) & 240i32) >> 4) as u8);
                            let mut armInfo: u8 = ((((rotationInfo) as i32) & 15i32) as u8);
                            if (RotatingGate_HasArm(((i) as u8), armInfo)) != 0 {
                                if !((RotatingGate_CanRotate(
                                    ((i) as u8),
                                    ((rotationDirection) as i32),
                                )) != 0)
                                {
                                    return 1u32;
                                }
                            }
                        }
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 0u32;
    }
}
