//! Translated from `src/battle_anim_dragon.c` by tools/rustport/c2rs.py.
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
    clippy::missing_transmute_annotations
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    DestroySpriteAndMatrix, GetBattlerSide, GetBattlerSpriteBGPriorityRank, GetBattlerSpriteCoord,
    GetBattlerSpriteCoordAttr, GetBattlerYCoordWithElevation, RunStoredCallbackWhenAnimEnds,
    SetAnimSpriteInitialXOffset, SetSpriteCoordsToAnimAttackerCoords, StartAnimLinearTranslation,
    StoreSpriteCallbackInData6, TranslateSpriteLinearAndFlicker,
};
use crate::battle_main::{gBattle_BG1_X, gBattle_BG2_X, gBattlerAttacker};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::task::gTasks;
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
/// `StartSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAffineAnim(a0 as _, a1);
    }
}
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// Data tables (translate with cdata.py): sAnim_OutrageOverheatFire_0 sAnims_OutrageOverheatFire gOutrageFlameSpriteTemplate sAnim_DragonBreathFire_0 sAnim_DragonBreathFire_1 sAnims_DragonBreathFire sAffineAnim_DragonBreathFire_0 sAffineAnim_DragonBreathFire_1 sAffineAnims_DragonBreathFire gDragonBreathFireSpriteTemplate sAnim_DragonRageFirePlume sAnims_DragonRageFirePlume gDragonRageFirePlumeSpriteTemplate sAnim_DragonRageFire sAnims_DragonRageFire sAffineAnim_DragonRageFire_0 sAffineAnim_DragonRageFire_1 sAffineAnims_DragonRageFire gDragonRageFireSpitSpriteTemplate gDragonDanceOrbSpriteTemplate gOverheatFlameSpriteTemplate

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub x: i16,
    pub y: i16,
    pub duration: i16,
    pub xVelocity: i16,
    pub yVelocity: i16,
    pub flickerDuration: i16,
}

unsafe impl Sync for Anon1 {}

/// `__anon2`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon2 {
    pub initialX: i16,
    pub initialY: i16,
    pub targetX: i16,
    pub targetY: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon2 {}

/// `__anon3`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon3 {
    pub relativeTo: i16,
    pub x: i16,
    pub y: i16,
}

unsafe impl Sync for Anon3 {}

/// `__anon4`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon4 {
    pub angle: i16,
}

unsafe impl Sync for Anon4 {}

/// `__anon5`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon5 {
    pub speed: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub duration: i16,
    pub y: i16,
}

unsafe impl Sync for Anon5 {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Anon1>() == 12);
    assert!(offset_of!(Anon1, x) == 0);
    assert!(offset_of!(Anon1, y) == 2);
    assert!(offset_of!(Anon1, duration) == 4);
    assert!(offset_of!(Anon1, xVelocity) == 6);
    assert!(offset_of!(Anon1, yVelocity) == 8);
    assert!(offset_of!(Anon1, flickerDuration) == 10);
    assert!(size_of::<Anon2>() == 10);
    assert!(offset_of!(Anon2, initialX) == 0);
    assert!(offset_of!(Anon2, initialY) == 2);
    assert!(offset_of!(Anon2, targetX) == 4);
    assert!(offset_of!(Anon2, targetY) == 6);
    assert!(offset_of!(Anon2, duration) == 8);
    assert!(size_of::<Anon3>() == 6);
    assert!(offset_of!(Anon3, relativeTo) == 0);
    assert!(offset_of!(Anon3, x) == 2);
    assert!(offset_of!(Anon3, y) == 4);
    assert!(size_of::<Anon4>() == 2);
    assert!(offset_of!(Anon4, angle) == 0);
    assert!(size_of::<Anon5>() == 10);
    assert!(offset_of!(Anon5, speed) == 0);
    assert!(offset_of!(Anon5, unk1) == 2);
    assert!(offset_of!(Anon5, unk2) == 4);
    assert!(offset_of!(Anon5, duration) == 6);
    assert!(offset_of!(Anon5, y) == 8);
};

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedOverheatData: Aligned<CArray<u16, 7>> = Aligned(unsafe { zeroed() });

pub(crate) unsafe fn AnimOutrageFlame(sprite: *mut Sprite) {
    let cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= (*cmd).x;
        (*cmd).xVelocity = -(*cmd).xVelocity;
        (*cmd).yVelocity = -(*cmd).yVelocity;
    } else {
        (*sprite).x += (*cmd).x;
    }
    (*sprite).y += (*cmd).y;
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*cmd).xVelocity;
    (*sprite).data[3] = (*cmd).yVelocity;
    (*sprite).data[5] = (*cmd).flickerDuration;
    (*sprite).set_invisible(TRUE as u16);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteLinearAndFlicker);
}
unsafe fn StartDragonFireTranslation(sprite: *mut Sprite) {
    let cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= (*cmd).initialY;
        (*sprite).y += (*cmd).initialY;
        (*sprite).data[2] -= (*cmd).targetX;
        (*sprite).data[4] += (*cmd).targetY;
    } else {
        (*sprite).x += (*cmd).initialX;
        (*sprite).y += (*cmd).initialY;
        (*sprite).data[2] += (*cmd).targetX;
        (*sprite).data[4] += (*cmd).targetY;
        StartSpriteAnim(sprite, 1);
    }
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe fn AnimDragonRageFirePlume(sprite: *mut Sprite) {
    let cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    }
    SetAnimSpriteInitialXOffset(sprite, (*cmd).x);
    (*sprite).y += (*cmd).y;
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe fn AnimDragonFireToTarget(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        StartSpriteAffineAnim(sprite, 1);
    }
    StartDragonFireTranslation(sprite);
}
pub(crate) unsafe fn AnimDragonDanceOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = 1;
    (*sprite).data[6] = (*cmd).angle;
    let r5: u16 = GetBattlerSpriteCoordAttr(gBattlerAttacker, BATTLER_COORD_ATTR_HEIGHT) as u16;
    let r0: u16 = GetBattlerSpriteCoordAttr(gBattlerAttacker, BATTLER_COORD_ATTR_WIDTH) as u16;
    if r5 > r0 {
        (*sprite).data[7] = (r5 as i32 / 2) as i16;
    } else {
        (*sprite).data[7] = (r0 as i32 / 2) as i16;
    }
    (*sprite).x2 = Cos((*sprite).data[6], (*sprite).data[7]);
    (*sprite).y2 = Sin((*sprite).data[6], (*sprite).data[7]);
    (*sprite).callback = Some(AnimDragonDanceOrb_Step);
}
pub(crate) unsafe fn AnimDragonDanceOrb_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[6] = ((*sprite).data[6] - (*sprite).data[5]) & 0xFF;
            (*sprite).x2 = Cos((*sprite).data[6], (*sprite).data[7]);
            (*sprite).y2 = Sin((*sprite).data[6], (*sprite).data[7]);
            if ({
                (*sprite).data[4] += 1;
                (*sprite).data[4]
            }) > 5
            {
                (*sprite).data[4] = 0;
                if (*sprite).data[5] <= 15
                    && ({
                        (*sprite).data[5] += 1;
                        (*sprite).data[5]
                    }) > 15
                {
                    (*sprite).data[5] = 16;
                }
            }
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) > 0x3C
            {
                (*sprite).data[3] = 0;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            (*sprite).data[6] = ((*sprite).data[6] - (*sprite).data[5]) & 0xFF;
            if (*sprite).data[7] <= 0x95
                && ({
                    (*sprite).data[7] += 8;
                    (*sprite).data[7]
                }) > 0x95
            {
                (*sprite).data[7] = 0x96;
            }
            (*sprite).x2 = Cos((*sprite).data[6], (*sprite).data[7]);
            (*sprite).y2 = Sin((*sprite).data[6], (*sprite).data[7]);
            if ({
                (*sprite).data[4] += 1;
                (*sprite).data[4]
            }) > 5
            {
                (*sprite).data[4] = 0;
                if (*sprite).data[5] <= 15
                    && ({
                        (*sprite).data[5] += 1;
                        (*sprite).data[5]
                    }) > 15
                {
                    (*sprite).data[5] = 16;
                }
            }
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) > 20
            {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DragonDanceWaver(taskId: u8) {
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
        scanlineParams.dmaDest = 67108884_usize as *mut u16 as *mut c_void;
        (*task).data[2] = gBattle_BG1_X as i16;
    } else {
        scanlineParams.dmaDest = 67108888_usize as *mut u16 as *mut c_void;
        (*task).data[2] = gBattle_BG2_X as i16;
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    let y: u8 = GetBattlerYCoordWithElevation(gBattleAnimAttacker);
    (*task).data[3] = y as i16 - 32;
    (*task).data[4] = y as i16 + 32;
    if (*task).data[3] < 0 {
        (*task).data[3] = 0;
    }
    let mut i: u16 = (*task).data[3] as u16;
    while i as i32 <= (*task).data[4] as i32 {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = (*task).data[2] as u16;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = (*task).data[2] as u16;
        i += 1;
    }
    ScanlineEffect_SetParams(scanlineParams);
    (*task).func = Some(AnimTask_DragonDanceWaver_Step);
}
pub(crate) unsafe fn AnimTask_DragonDanceWaver_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[7] += 1;
                (*task).data[7]
            }) > 1
            {
                (*task).data[7] = 0;
                if ({
                    (*task).data[6] += 1;
                    (*task).data[6]
                }) == 3
                {
                    (*task).data[0] += 1;
                }
            }
            UpdateDragonDanceScanlineEffect(task);
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 0x3C
            {
                (*task).data[0] += 1;
            }
            UpdateDragonDanceScanlineEffect(task);
        }
        2 => {
            if ({
                (*task).data[7] += 1;
                (*task).data[7]
            }) > 1
            {
                (*task).data[7] = 0;
                if ({
                    (*task).data[6] -= 1;
                    (*task).data[6]
                }) == 0
                {
                    (*task).data[0] += 1;
                }
            }
            UpdateDragonDanceScanlineEffect(task);
        }
        3 => {
            (*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .state = 3;
            (*task).data[0] += 1;
        }
        4 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn UpdateDragonDanceScanlineEffect(task: *mut Task) {
    let mut sineIndex: u16 = (*task).data[5] as u16;
    let mut i: u16 = (*task).data[3] as u16;
    while i as i32 <= (*task).data[4] as i32 {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
            .cast::<ScanlineEffect>()
            .cast_mut())
        .srcBuffer][i] = (((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [sineIndex] as i32
            * (*task).data[6] as i32)
            >> 7) as u16
            + (*task).data[2] as u16;
        sineIndex = (sineIndex + 8) & 0xFF;
        i += 1;
    }
    (*task).data[5] = ((*task).data[5] + 9) & 0xFF;
}
pub(crate) unsafe fn AnimOverheatFlame(sprite: *mut Sprite) {
    let cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    let yAmplitude: i32 = (*cmd).unk2 as i32 * 3 / 5;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).y;
    (*sprite).data[1] = Cos((*cmd).unk1, (*cmd).unk2);
    (*sprite).data[2] = Sin((*cmd).unk1, yAmplitude as i16);
    (*sprite).x += (*sprite).data[1] * (*cmd).speed;
    (*sprite).y += (*sprite).data[2] * (*cmd).speed;
    (*sprite).data[3] = (*cmd).duration;
    (*sprite).callback = Some(AnimOverheatFlame_Step);
    for i in 0..7i32 {
        sUnusedOverheatData[i] = (*sprite).data[i] as u16;
    }
}
pub(crate) unsafe fn AnimOverheatFlame_Step(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[1];
    (*sprite).data[5] += (*sprite).data[2];
    (*sprite).x2 = (*sprite).data[4] / 10;
    (*sprite).y2 = (*sprite).data[5] / 10;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > (*sprite).data[3]
    {
        DestroyAnimSprite(sprite);
    }
}
