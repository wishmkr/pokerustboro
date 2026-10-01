//! Translated from `src/rotating_gate.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::bike::GetPlayerSpeed;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::event_data::GetVarPointer;
use crate::event_object_movement::GetMapCoordsFromSpritePos;
use crate::fieldmap::MapGridGetCollisionAt;
use crate::load_save::gSaveBlock1Ptr;
use crate::sound::PlaySE;
use crate::sprite::gSprites;
use crate::sprite::{gSpriteCoordOffsetX, gSpriteCoordOffsetY};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `LoadSpriteSheets` with this module's view of its types.
#[inline]
unsafe fn LoadSpriteSheets(a0: *mut SpriteSheet) {
    unsafe {
        crate::sprite::LoadSpriteSheets(a0 as _);
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
// Data tables (translate with cdata.py): sRotatingGate_FortreePuzzleConfig sRotatingGate_TrickHousePuzzleConfig sRotatingGateTiles_1 sRotatingGateTiles_2 sRotatingGateTiles_3 sRotatingGateTiles_4 sRotatingGateTiles_5 sRotatingGateTiles_6 sRotatingGateTiles_7 sRotatingGateTiles_8 sOamData_RotatingGateLarge sOamData_RotatingGateRegular sRotatingGatesGraphicsTable sSpriteAnim_RotatingGateLarge sSpriteAnim_RotatingGateRegular sSpriteAnimTable_RotatingGateLarge sSpriteAnimTable_RotatingGateRegular sSpriteAffineAnim_Rotated0 sSpriteAffineAnim_Rotated90 sSpriteAffineAnim_Rotated180 sSpriteAffineAnim_Rotated270 sSpriteAffineAnim_RotatingClockwise0to90 sSpriteAffineAnim_RotatingClockwise90to180 sSpriteAffineAnim_RotatingClockwise180to270 sSpriteAffineAnim_RotatingClockwise270to360 sSpriteAffineAnim_RotatingAnticlockwise360to270 sSpriteAffineAnim_RotatingAnticlockwise270to180 sSpriteAffineAnim_RotatingAnticlockwise180to90 sSpriteAffineAnim_RotatingAnticlockwise90to0 sSpriteAffineAnim_RotatingClockwise0to90Faster sSpriteAffineAnim_RotatingClockwise90to180Faster sSpriteAffineAnim_RotatingClockwise180to270Faster sSpriteAffineAnim_RotatingClockwise270to360Faster sSpriteAffineAnim_RotatingAnticlockwise360to270Faster sSpriteAffineAnim_RotatingAnticlockwise270to180Faster sSpriteAffineAnim_RotatingAnticlockwise180to90Faster sSpriteAffineAnim_RotatingAnticlockwise90to0Faster sSpriteAffineAnimTable_RotatingGate sSpriteTemplate_RotatingGateLarge sSpriteTemplate_RotatingGateRegular sRotatingGate_RotationInfoNorth sRotatingGate_RotationInfoSouth sRotatingGate_RotationInfoWest sRotatingGate_RotationInfoEast sRotatingGate_ArmPositionsClockwiseRotation sRotatingGate_ArmPositionsAntiClockwiseRotation sRotatingGate_ArmLayout

/// `struct RotatingGatePuzzle`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct RotatingGatePuzzle {
    pub x: i16,
    pub y: i16,
    pub shape: u8,
    pub orientation: u8,
}

unsafe impl Sync for RotatingGatePuzzle {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<RotatingGatePuzzle>() == 8);
    assert!(offset_of!(RotatingGatePuzzle, x) == 0);
    assert!(offset_of!(RotatingGatePuzzle, y) == 2);
    assert!(offset_of!(RotatingGatePuzzle, shape) == 4);
    assert!(offset_of!(RotatingGatePuzzle, orientation) == 5);
};

const GATE_ARM_MAX_LENGTH: i32 = 2;
const GATE_ARM_NORTH: i32 = 0;
const GATE_ARM_WEST: i32 = 3;
const GATE_ORIENTATION_270: u8 = 3;
const GATE_ORIENTATION_MAX: i32 = 4;
const GATE_ROT_NONE: u8 = 255;
const GATE_SHAPE_L1: u8 = 0;
const GATE_SHAPE_T1: u8 = 4;
const PUZZLE_FORTREE_CITY_GYM: i32 = 1;
const PUZZLE_NONE: i32 = 0;
const PUZZLE_ROUTE110_TRICK_HOUSE_PUZZLE6: i32 = 2;
const ROTATE_ANTICLOCKWISE: u32 = 1;
const ROTATE_CLOCKWISE: u8 = 2;
const ROTATE_NONE: i16 = 0;
const ROTATING_GATE_PUZZLE_MAX: i32 = 12;
const ROTATING_GATE_TILE_TAG: u16 = 4864;

static sRotatingGate_ArmLayout: Table<CArray<CArray<u8, 8>, 12>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_ArmLayout).cast());
static sRotatingGate_ArmPositionsAntiClockwiseRotation: Table<CArray<Coords8, 8>> = Table(
    (&raw const crate::data::rotating_gate::sRotatingGate_ArmPositionsAntiClockwiseRotation).cast(),
);
static sRotatingGate_ArmPositionsClockwiseRotation: Table<CArray<Coords8, 8>> = Table(
    (&raw const crate::data::rotating_gate::sRotatingGate_ArmPositionsClockwiseRotation).cast(),
);
static sRotatingGate_FortreePuzzleConfig: Table<CArray<RotatingGatePuzzle, 8>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_FortreePuzzleConfig).cast());
static sRotatingGate_RotationInfoEast: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_RotationInfoEast).cast());
static sRotatingGate_RotationInfoNorth: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_RotationInfoNorth).cast());
static sRotatingGate_RotationInfoSouth: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_RotationInfoSouth).cast());
static sRotatingGate_RotationInfoWest: Table<CArray<u8, 16>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_RotationInfoWest).cast());
static sRotatingGate_TrickHousePuzzleConfig: Table<CArray<RotatingGatePuzzle, 11>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGate_TrickHousePuzzleConfig).cast());
static sRotatingGatesGraphicsTable: Table<CArray<SpriteSheet, 9>> =
    Table((&raw const crate::data::rotating_gate::sRotatingGatesGraphicsTable).cast());
static sSpriteTemplate_RotatingGateLarge: Table<SpriteTemplate> =
    Table((&raw const crate::data::rotating_gate::sSpriteTemplate_RotatingGateLarge).cast());
static sSpriteTemplate_RotatingGateRegular: Table<SpriteTemplate> =
    Table((&raw const crate::data::rotating_gate::sSpriteTemplate_RotatingGateRegular).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRotatingGate_GateSpriteIds: Aligned<CArray<u8, 12>> =
    Aligned(unsafe { zeroed() });
#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sRotatingGate_PuzzleConfig: *mut RotatingGatePuzzle = null_mut();
#[unsafe(link_section = "ewram_data")]
pub(crate) static sRotatingGate_PuzzleCount: crate::global::Global<u8> =
    crate::global::Global::new(0);

unsafe fn GetCurrentMapRotatingGatePuzzleType() -> i32 {
    if (*gSaveBlock1Ptr).location.mapGroup == 12 && (*gSaveBlock1Ptr).location.mapNum == 1 {
        return PUZZLE_FORTREE_CITY_GYM;
    }
    if (*gSaveBlock1Ptr).location.mapGroup == 29 && (*gSaveBlock1Ptr).location.mapNum == 8 {
        return PUZZLE_ROUTE110_TRICK_HOUSE_PUZZLE6;
    }
    PUZZLE_NONE
}
unsafe fn RotatingGate_ResetAllGateOrientations() {
    let ptr: *mut u8 = GetVarPointer(VAR_TEMP_0) as *mut u8;
    for i in 0..(sRotatingGate_PuzzleCount.get() as i32) {
        *ptr.at(i) = (*sRotatingGate_PuzzleConfig.at(i)).orientation;
    }
}
unsafe fn RotatingGate_GetGateOrientation(gateId: u8) -> i32 {
    *(GetVarPointer(VAR_TEMP_0) as *mut u8).at(gateId) as i32
}
unsafe fn RotatingGate_SetGateOrientation(gateId: u8, orientation: u8) {
    *(GetVarPointer(VAR_TEMP_0) as *mut u8).at(gateId) = orientation;
}
unsafe fn RotatingGate_RotateInDirection(gateId: u8, rotationDirection: u32) {
    let mut orientation: u8 = RotatingGate_GetGateOrientation(gateId) as u8;
    if rotationDirection == ROTATE_ANTICLOCKWISE {
        if orientation != 0 {
            orientation -= 1;
        } else {
            orientation = GATE_ORIENTATION_270;
        }
    } else {
        orientation += 1;
        orientation = (orientation as i32 % 4) as u8;
    }
    RotatingGate_SetGateOrientation(gateId, orientation);
}
unsafe fn RotatingGate_LoadPuzzleConfig() {
    let puzzleType: i32 = GetCurrentMapRotatingGatePuzzleType();
    match puzzleType {
        PUZZLE_FORTREE_CITY_GYM => {
            sRotatingGate_PuzzleConfig = sRotatingGate_FortreePuzzleConfig.as_ptr().cast_mut();
            sRotatingGate_PuzzleCount.set(8);
        }
        PUZZLE_ROUTE110_TRICK_HOUSE_PUZZLE6 => {
            sRotatingGate_PuzzleConfig = sRotatingGate_TrickHousePuzzleConfig.as_ptr().cast_mut();
            sRotatingGate_PuzzleCount.set(11);
        }
        _ => {
            return;
        }
    }
    for i in 0..11u32 {
        sRotatingGate_GateSpriteIds[i] = MAX_SPRITES;
    }
}
unsafe fn RotatingGate_CreateGatesWithinViewport(deltaX: i16, deltaY: i16) {
    let x: i16 = (*gSaveBlock1Ptr).pos.x - 2;
    let x2: i16 = (*gSaveBlock1Ptr).pos.x + MAP_OFFSET_W as i16 + 2;
    let y: i16 = (*gSaveBlock1Ptr).pos.y - 2;
    let y2: i16 = (*gSaveBlock1Ptr).pos.y + MAP_OFFSET_H as i16;
    let mut i: u8 = 0;
    while i < sRotatingGate_PuzzleCount.get() {
        let x3: i16 = (*sRotatingGate_PuzzleConfig.at(i)).x + MAP_OFFSET as i16;
        let y3: i16 = (*sRotatingGate_PuzzleConfig.at(i)).y + MAP_OFFSET as i16;
        if y <= y3
            && y2 >= y3
            && x <= x3
            && x2 >= x3
            && sRotatingGate_GateSpriteIds[i] == MAX_SPRITES
        {
            sRotatingGate_GateSpriteIds[i] = RotatingGate_CreateGate(i, deltaX, deltaY);
        }
        i += 1;
    }
}
unsafe fn RotatingGate_CreateGate(gateId: u8, deltaX: i16, deltaY: i16) -> u8 {
    let mut template: SpriteTemplate = zeroed();
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let gate: *mut RotatingGatePuzzle = sRotatingGate_PuzzleConfig.at(gateId);
    if (*gate).shape == GATE_SHAPE_L1 || (*gate).shape == GATE_SHAPE_T1 {
        template = *sSpriteTemplate_RotatingGateRegular;
    } else {
        template = *sSpriteTemplate_RotatingGateLarge;
    }
    template.tileTag = (*gate).shape as u16 + ROTATING_GATE_TILE_TAG;
    let spriteId: u8 = CreateSprite(&raw mut template, 0, 0, 0x94);
    if spriteId == MAX_SPRITES {
        return MAX_SPRITES;
    }
    x = (*gate).x + MAP_OFFSET as i16;
    y = (*gate).y + MAP_OFFSET as i16;
    let sprite: *mut Sprite = &raw mut gSprites[spriteId];
    (*sprite).data[0] = gateId as i16;
    (*sprite).set_coordOffsetEnabled(1);
    GetMapCoordsFromSpritePos(
        x + deltaX,
        y + deltaY,
        &raw mut (*sprite).x,
        &raw mut (*sprite).y,
    );
    RotatingGate_HideGatesOutsideViewport(sprite);
    StartSpriteAffineAnim(sprite, RotatingGate_GetGateOrientation(gateId) as u8);
    spriteId
}
pub(crate) unsafe fn SpriteCallback_RotatingGate(sprite: *mut Sprite) {
    let mut affineAnimation: u8 = 0;
    let rotationDirection: u8 = (*sprite).data[1] as u8;
    let orientation: u8 = (*sprite).data[2] as u8;
    RotatingGate_HideGatesOutsideViewport(sprite);
    if rotationDirection == ROTATE_ANTICLOCKWISE as u8 {
        affineAnimation = orientation + 4;
        if GetPlayerSpeed() != PLAYER_SPEED_NORMAL {
            affineAnimation += 8;
        }
        PlaySE(SE_ROTATING_GATE);
        StartSpriteAffineAnim(sprite, affineAnimation);
    } else if rotationDirection == ROTATE_CLOCKWISE {
        affineAnimation = orientation + 8;
        if GetPlayerSpeed() != PLAYER_SPEED_NORMAL {
            affineAnimation += 8;
        }
        PlaySE(SE_ROTATING_GATE);
        StartSpriteAffineAnim(sprite, affineAnimation);
    }
    (*sprite).data[1] = ROTATE_NONE;
}
unsafe fn RotatingGate_HideGatesOutsideViewport(sprite: *mut Sprite) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut x2: i16 = 0;
    let mut y2: i16 = 0;
    (*sprite).set_invisible(FALSE as u16);
    x = (*sprite).x as u16
        + (*sprite).x2 as u16
        + (*sprite).centerToCornerVecX as u16
        + gSpriteCoordOffsetX as u16;
    y = (*sprite).y as u16
        + (*sprite).y2 as u16
        + (*sprite).centerToCornerVecY as u16
        + gSpriteCoordOffsetY as u16;
    x2 = x as i16 + 64;
    y2 = y as i16 + 64;
    if x as i16 > 255 || x2 < -16 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if y as i16 > 175 || y2 < -16 {
        (*sprite).set_invisible(TRUE as u16);
    }
}
unsafe fn LoadRotatingGatePics() {
    LoadSpriteSheets(sRotatingGatesGraphicsTable.as_ptr().cast_mut());
}
unsafe fn RotatingGate_DestroyGatesOutsideViewport() {
    let x: i16 = (*gSaveBlock1Ptr).pos.x - 2;
    let x2: i16 = (*gSaveBlock1Ptr).pos.x + MAP_OFFSET_W as i16 + 2;
    let y: i16 = (*gSaveBlock1Ptr).pos.y - 2;
    let y2: i16 = (*gSaveBlock1Ptr).pos.y + MAP_OFFSET_H as i16;
    let mut i: i32 = 0;
    while i < sRotatingGate_PuzzleCount.get() as i32 {
        'l1: {
            let xGate: i16 = (*sRotatingGate_PuzzleConfig.at(i)).x + MAP_OFFSET as i16;
            let yGate: i16 = (*sRotatingGate_PuzzleConfig.at(i)).y + MAP_OFFSET as i16;
            if sRotatingGate_GateSpriteIds[i] == MAX_SPRITES {
                break 'l1;
            }
            if xGate < x || xGate > x2 || yGate < y || yGate > y2 {
                let sprite: *mut Sprite = &raw mut gSprites[sRotatingGate_GateSpriteIds[i]];
                FreeSpriteOamMatrix(sprite);
                DestroySprite(sprite);
                sRotatingGate_GateSpriteIds[i] = MAX_SPRITES;
            }
        }
        i += 1;
    }
}
unsafe fn RotatingGate_CanRotate(gateId: u8, rotationDirection: i32) -> i32 {
    let mut armPos: *mut Coords8 = null_mut();
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut shape: u8 = 0;
    if rotationDirection == ROTATE_ANTICLOCKWISE as i32 {
        armPos = sRotatingGate_ArmPositionsAntiClockwiseRotation
            .as_ptr()
            .cast_mut();
    } else if rotationDirection == ROTATE_CLOCKWISE as i32 {
        armPos = sRotatingGate_ArmPositionsClockwiseRotation
            .as_ptr()
            .cast_mut();
    } else {
        return FALSE as i32;
    }
    let orientation: u8 = RotatingGate_GetGateOrientation(gateId) as u8;
    shape = (*sRotatingGate_PuzzleConfig.at(gateId)).shape;
    x = (*sRotatingGate_PuzzleConfig.at(gateId)).x + MAP_OFFSET as i16;
    y = (*sRotatingGate_PuzzleConfig.at(gateId)).y + MAP_OFFSET as i16;
    let mut i: i32 = GATE_ARM_NORTH;
    while i <= GATE_ARM_WEST {
        for j in 0..GATE_ARM_MAX_LENGTH {
            let armIndex: u8 = 2 * ((orientation as i32 + i) % 4) as u8 + j as u8;
            if sRotatingGate_ArmLayout[shape][2 * i + j] != 0
                && MapGridGetCollisionAt(
                    x as i32 + (*armPos.at(armIndex)).x as i32,
                    y as i32 + (*armPos.at(armIndex)).y as i32,
                ) == 1
            {
                return FALSE as i32;
            }
        }
        i += 1;
    }
    TRUE as i32
}
unsafe fn RotatingGate_HasArm(gateId: u8, armInfo: u8) -> i32 {
    let arm: i32 = armInfo as i32 / 2;
    let isLongArm: i32 = armInfo as i32 % 2;
    let armOrientation: i8 = ((arm - RotatingGate_GetGateOrientation(gateId) + 4) % 4) as i8;
    let shape: i32 = (*sRotatingGate_PuzzleConfig.at(gateId)).shape as i32;
    sRotatingGate_ArmLayout[shape][armOrientation as i32 * 2 + isLongArm] as i32
}
unsafe fn RotatingGate_TriggerRotationAnimation(gateId: u8, rotationDirection: i32) {
    if sRotatingGate_GateSpriteIds[gateId] != MAX_SPRITES {
        let sprite: *mut Sprite = &raw mut gSprites[sRotatingGate_GateSpriteIds[gateId]];
        (*sprite).data[1] = rotationDirection as i16;
        (*sprite).data[2] = RotatingGate_GetGateOrientation(gateId) as i16;
    }
}
unsafe fn RotatingGate_GetRotationInfo(direction: u8, x: i16, y: i16) -> u8 {
    let mut ptr: *mut u8 = null_mut();
    if direction == DIR_NORTH {
        ptr = sRotatingGate_RotationInfoNorth.as_ptr().cast_mut();
    } else if direction == DIR_SOUTH {
        ptr = sRotatingGate_RotationInfoSouth.as_ptr().cast_mut();
    } else if direction == DIR_WEST {
        ptr = sRotatingGate_RotationInfoWest.as_ptr().cast_mut();
    } else if direction == DIR_EAST {
        ptr = sRotatingGate_RotationInfoEast.as_ptr().cast_mut();
    } else {
        return GATE_ROT_NONE;
    }
    *ptr.at(y as i32 * 4 + x as i32)
}
#[unsafe(no_mangle)]
pub unsafe fn RotatingGate_InitPuzzle() {
    if GetCurrentMapRotatingGatePuzzleType() != 0 {
        RotatingGate_LoadPuzzleConfig();
        RotatingGate_ResetAllGateOrientations();
    }
}
pub unsafe fn RotatingGatePuzzleCameraUpdate(deltaX: i16, deltaY: i16) {
    if GetCurrentMapRotatingGatePuzzleType() != 0 {
        RotatingGate_CreateGatesWithinViewport(deltaX, deltaY);
        RotatingGate_DestroyGatesOutsideViewport();
    }
}
#[unsafe(no_mangle)]
pub unsafe fn RotatingGate_InitPuzzleAndGraphics() {
    if GetCurrentMapRotatingGatePuzzleType() != 0 {
        LoadRotatingGatePics();
        RotatingGate_LoadPuzzleConfig();
        RotatingGate_CreateGatesWithinViewport(0, 0);
    }
}
pub unsafe fn CheckForRotatingGatePuzzleCollision(direction: u8, x: i16, y: i16) -> u32 {
    if GetCurrentMapRotatingGatePuzzleType() == 0 {
        return FALSE as u32;
    }
    let mut i: i32 = 0;
    while i < sRotatingGate_PuzzleCount.get() as i32 {
        let gateX: i16 = (*sRotatingGate_PuzzleConfig.at(i)).x + MAP_OFFSET as i16;
        let gateY: i16 = (*sRotatingGate_PuzzleConfig.at(i)).y + MAP_OFFSET as i16;
        if gateX as i32 - 2 <= x as i32
            && x as i32 <= gateX as i32 + 1
            && gateY as i32 - 2 <= y as i32
            && y as i32 <= gateY as i32 + 1
        {
            let centerX: i16 = x - gateX + 2;
            let centerY: i16 = y - gateY + 2;
            let rotationInfo: u8 = RotatingGate_GetRotationInfo(direction, centerX, centerY);
            if rotationInfo != GATE_ROT_NONE {
                let rotationDirection: u8 = ((rotationInfo as i32 & 0xF0) >> 4) as u8;
                let armInfo: u8 = rotationInfo & 0xF;
                if RotatingGate_HasArm(i as u8, armInfo) != 0 {
                    if RotatingGate_CanRotate(i as u8, rotationDirection as i32) != 0 {
                        RotatingGate_TriggerRotationAnimation(i as u8, rotationDirection as i32);
                        RotatingGate_RotateInDirection(i as u8, rotationDirection as u32);
                        return FALSE as u32;
                    }
                    return TRUE as u32;
                }
            }
        }
        i += 1;
    }
    FALSE as u32
}
pub unsafe fn CheckForRotatingGatePuzzleCollisionWithoutAnimation(
    direction: u8,
    x: i16,
    y: i16,
) -> u32 {
    if GetCurrentMapRotatingGatePuzzleType() == 0 {
        return FALSE as u32;
    }
    let mut i: i32 = 0;
    while i < sRotatingGate_PuzzleCount.get() as i32 {
        let gateX: i16 = (*sRotatingGate_PuzzleConfig.at(i)).x + MAP_OFFSET as i16;
        let gateY: i16 = (*sRotatingGate_PuzzleConfig.at(i)).y + MAP_OFFSET as i16;
        if gateX as i32 - 2 <= x as i32
            && x as i32 <= gateX as i32 + 1
            && gateY as i32 - 2 <= y as i32
            && y as i32 <= gateY as i32 + 1
        {
            let centerX: i16 = x - gateX + 2;
            let centerY: i16 = y - gateY + 2;
            let rotationInfo: u8 = RotatingGate_GetRotationInfo(direction, centerX, centerY);
            if rotationInfo != GATE_ROT_NONE {
                let rotationDirection: u8 = ((rotationInfo as i32 & 0xF0) >> 4) as u8;
                let armInfo: u8 = rotationInfo & 0xF;
                if RotatingGate_HasArm(i as u8, armInfo) != 0
                    && RotatingGate_CanRotate(i as u8, rotationDirection as i32) == 0
                {
                    return TRUE as u32;
                }
            }
        }
        i += 1;
    }
    FALSE as u32
}
