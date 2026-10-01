//! Translated from `src/battle_anim_normal.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, IsContest, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_flying::DestroyAnimSpriteAfterTimer;
use crate::battle_anim_mons::{
    DestroySpriteAndMatrix, GetAnimBattlerSpriteId, GetBattlePalettesMask, GetBattlerSide,
    InitSpritePosToAnimAttacker, InitSpritePosToAnimTarget, RunStoredCallbackWhenAffineAnimEnds,
    StoreSpriteCallbackInData6, TranslateSpriteInGrowingCircle, WaitAnimForDuration,
};
use crate::battle_main::{gBattle_BG3_X, gBattle_BG3_Y, gBattlersCount};
use crate::battle_main::{gBattlerSpriteIds, gHealthboxSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, InvertPlttBuffer, TintPlttBuffer, UnfadePlttBuffer,
    gPaletteFade,
};
use crate::random::Random2;
use crate::sprite::gSprites;
use crate::sprite::{IndexOfSpritePaletteTag, gSpriteCoordOffsetX, gSpriteCoordOffsetY};
use crate::task::{task_func, task_get, task_set, task_set_func};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
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
// The C's names for task and sprite data slots.
const sShakeVelocity: usize = 0;
const tPalSelector: usize = 0;
const tPalTag: usize = 0;
const tXOffset: usize = 0;
const sDelay: usize = 1;
const sShakeTimer: usize = 1;
const tDelay: usize = 1;
const tLength: usize = 1;
const tYOffset: usize = 1;
const sNumBlends: usize = 2;
const sShakeDuration: usize = 2;
const tFlagsScenery: usize = 2;
const tNumBlends: usize = 2;
const tNumShakes: usize = 2;
const sColor1: usize = 3;
const tColor1: usize = 3;
const tFlagsAttacker: usize = 3;
const tInitialBlendY: usize = 3;
const sBlendY1: usize = 4;
const sOriginalValue: usize = 4;
const tBlendY1: usize = 4;
const tFlagsTarget: usize = 4;
const tTargetBlendY: usize = 4;
const sColor2: usize = 5;
const sType: usize = 5;
const tBlendColor: usize = 5;
const tColor2: usize = 5;
const tColorR: usize = 5;
const sBlendY2: usize = 6;
const sShakePtrLo: usize = 6;
const tBlendY2: usize = 6;
const tColorG: usize = 6;
const sPaletteSelector: usize = 7;
const sShakePtrHi: usize = 7;
const tAnimTag: usize = 7;
const tColorB: usize = 7;
const tRestoreBlend: usize = 8;
const tShakeDelay: usize = 8;
const tPalSelectorHi: usize = 9;
const tPalSelectorLo: usize = 10;
// Data tables (translate with cdata.py): sAnim_ConfusionDuck_0 sAnim_ConfusionDuck_1 sAnims_ConfusionDuck gConfusionDuckSpriteTemplate gSimplePaletteBlendSpriteTemplate gComplexPaletteBlendSpriteTemplate sAnim_CirclingSparkle sAnims_CirclingSparkle sCirclingSparkleSpriteTemplate gShakeMonOrPlatformSpriteTemplate sAffineAnim_HitSplat_0 sAffineAnim_HitSplat_1 sAffineAnim_HitSplat_2 sAffineAnim_HitSplat_3 sAffineAnims_HitSplat gBasicHitSplatSpriteTemplate gHandleInvertHitSplatSpriteTemplate gWaterHitSplatSpriteTemplate gRandomPosHitSplatSpriteTemplate gMonEdgeHitSplatSpriteTemplate gCrossImpactSpriteTemplate gFlashingHitSplatSpriteTemplate gPersistHitSplatSpriteTemplate

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub x: i16,
    pub y: i16,
    pub waveOffset: i16,
    pub wavePeriod: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon1 {}

/// `__anon2`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon2 {
    pub selector: i16,
    pub delay: i16,
    pub initialBlendY: i16,
    pub targetBlendY: i16,
    pub color: i16,
}

unsafe impl Sync for Anon2 {}

/// `__anon3`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon3 {
    pub selector: i16,
    pub delay: i16,
    pub numBlends: i16,
    pub color1: i16,
    pub blendY1: i16,
    pub color2: i16,
    pub blendY2: i16,
}

unsafe impl Sync for Anon3 {}

/// `__anon4`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon4 {
    pub x: i16,
    pub y: i16,
}

unsafe impl Sync for Anon4 {}

/// `__anon5`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon5 {
    pub selector: i16,
    pub delay: i16,
    pub numBlends: i16,
    pub initialBlendY: i16,
    pub targetBlendY: i16,
    pub color: i16,
}

unsafe impl Sync for Anon5 {}

/// `__anon6`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon6 {
    pub unk0: i16,
    pub delay: i16,
    pub numBlends: i16,
    pub initialBlendY: i16,
    pub targetBlendY: i16,
    pub color: i16,
}

unsafe impl Sync for Anon6 {}

/// `__anon7`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon7 {
    pub tag: i16,
    pub delay: i16,
    pub numBlends: i16,
    pub initialBlendY: i16,
    pub targetBlendY: i16,
    pub color: i16,
}

unsafe impl Sync for Anon7 {}

/// `__anon8`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon8 {
    pub tag: i16,
    pub delay: i16,
    pub numBlends: i16,
    pub color1: i16,
    pub blendY1: i16,
    pub color2: i16,
    pub blendY2: i16,
}

unsafe impl Sync for Anon8 {}

/// `__anon9`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon9 {
    pub flagsScenery: i16,
    pub flagsAttacker: i16,
    pub flagsTarget: i16,
}

unsafe impl Sync for Anon9 {}

/// `__anon10`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon10 {
    pub flagsScenery: i16,
    pub flagsAttacker: i16,
    pub flagsTarget: i16,
    pub duration: i16,
    pub r: i16,
    pub g: i16,
    pub b: i16,
}

unsafe impl Sync for Anon10 {}

/// `__anon11`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon11 {
    pub velocity: i16,
    pub shakeTimer: i16,
    pub shakeDuration: i16,
    pub r#type: i16,
    pub battlerSelector: i16,
}

unsafe impl Sync for Anon11 {}

/// `__anon12`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon12 {
    pub velocity: i16,
    pub shakeDuration: i16,
    pub duration: i16,
    pub r#type: i16,
    pub battlerSelector: i16,
}

unsafe impl Sync for Anon12 {}

/// `__anon13`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon13 {
    pub xOffset: i16,
    pub yOffset: i16,
    pub shakes: i16,
    pub delay: i16,
}

unsafe impl Sync for Anon13 {}

/// `__anon14`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon14 {
    pub x: i16,
    pub y: i16,
    pub relativeTo: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon14 {}

/// `__anon15`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon15 {
    pub x: i16,
    pub y: i16,
    pub relativeTo: i16,
    pub animation: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon15 {}

/// `__anon16`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon16 {
    pub x: i16,
    pub y: i16,
    pub relativeTo: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon16 {}

/// `__anon17`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon17 {
    pub relativeTo: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon17 {}

/// `__anon18`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon18 {
    pub relativeTo: i16,
    pub x: i16,
    pub y: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon18 {}

/// `__anon19`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon19 {
    pub x: i16,
    pub y: i16,
    pub relativeTo: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon19 {}

/// `__anon20`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon20 {
    pub x: i16,
    pub y: i16,
    pub relativeTo: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon20 {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Anon1>() == 10);
    assert!(offset_of!(Anon1, x) == 0);
    assert!(offset_of!(Anon1, y) == 2);
    assert!(offset_of!(Anon1, waveOffset) == 4);
    assert!(offset_of!(Anon1, wavePeriod) == 6);
    assert!(offset_of!(Anon1, duration) == 8);
    assert!(size_of::<Anon2>() == 10);
    assert!(offset_of!(Anon2, selector) == 0);
    assert!(offset_of!(Anon2, delay) == 2);
    assert!(offset_of!(Anon2, initialBlendY) == 4);
    assert!(offset_of!(Anon2, targetBlendY) == 6);
    assert!(offset_of!(Anon2, color) == 8);
    assert!(size_of::<Anon3>() == 14);
    assert!(offset_of!(Anon3, selector) == 0);
    assert!(offset_of!(Anon3, delay) == 2);
    assert!(offset_of!(Anon3, numBlends) == 4);
    assert!(offset_of!(Anon3, color1) == 6);
    assert!(offset_of!(Anon3, blendY1) == 8);
    assert!(offset_of!(Anon3, color2) == 10);
    assert!(offset_of!(Anon3, blendY2) == 12);
    assert!(size_of::<Anon4>() == 4);
    assert!(offset_of!(Anon4, x) == 0);
    assert!(offset_of!(Anon4, y) == 2);
    assert!(size_of::<Anon5>() == 12);
    assert!(offset_of!(Anon5, selector) == 0);
    assert!(offset_of!(Anon5, delay) == 2);
    assert!(offset_of!(Anon5, numBlends) == 4);
    assert!(offset_of!(Anon5, initialBlendY) == 6);
    assert!(offset_of!(Anon5, targetBlendY) == 8);
    assert!(offset_of!(Anon5, color) == 10);
    assert!(size_of::<Anon6>() == 12);
    assert!(offset_of!(Anon6, unk0) == 0);
    assert!(offset_of!(Anon6, delay) == 2);
    assert!(offset_of!(Anon6, numBlends) == 4);
    assert!(offset_of!(Anon6, initialBlendY) == 6);
    assert!(offset_of!(Anon6, targetBlendY) == 8);
    assert!(offset_of!(Anon6, color) == 10);
    assert!(size_of::<Anon7>() == 12);
    assert!(offset_of!(Anon7, tag) == 0);
    assert!(offset_of!(Anon7, delay) == 2);
    assert!(offset_of!(Anon7, numBlends) == 4);
    assert!(offset_of!(Anon7, initialBlendY) == 6);
    assert!(offset_of!(Anon7, targetBlendY) == 8);
    assert!(offset_of!(Anon7, color) == 10);
    assert!(size_of::<Anon8>() == 14);
    assert!(offset_of!(Anon8, tag) == 0);
    assert!(offset_of!(Anon8, delay) == 2);
    assert!(offset_of!(Anon8, numBlends) == 4);
    assert!(offset_of!(Anon8, color1) == 6);
    assert!(offset_of!(Anon8, blendY1) == 8);
    assert!(offset_of!(Anon8, color2) == 10);
    assert!(offset_of!(Anon8, blendY2) == 12);
    assert!(size_of::<Anon9>() == 6);
    assert!(offset_of!(Anon9, flagsScenery) == 0);
    assert!(offset_of!(Anon9, flagsAttacker) == 2);
    assert!(offset_of!(Anon9, flagsTarget) == 4);
    assert!(size_of::<Anon10>() == 14);
    assert!(offset_of!(Anon10, flagsScenery) == 0);
    assert!(offset_of!(Anon10, flagsAttacker) == 2);
    assert!(offset_of!(Anon10, flagsTarget) == 4);
    assert!(offset_of!(Anon10, duration) == 6);
    assert!(offset_of!(Anon10, r) == 8);
    assert!(offset_of!(Anon10, g) == 10);
    assert!(offset_of!(Anon10, b) == 12);
    assert!(size_of::<Anon11>() == 10);
    assert!(offset_of!(Anon11, velocity) == 0);
    assert!(offset_of!(Anon11, shakeTimer) == 2);
    assert!(offset_of!(Anon11, shakeDuration) == 4);
    assert!(offset_of!(Anon11, r#type) == 6);
    assert!(offset_of!(Anon11, battlerSelector) == 8);
    assert!(size_of::<Anon12>() == 10);
    assert!(offset_of!(Anon12, velocity) == 0);
    assert!(offset_of!(Anon12, shakeDuration) == 2);
    assert!(offset_of!(Anon12, duration) == 4);
    assert!(offset_of!(Anon12, r#type) == 6);
    assert!(offset_of!(Anon12, battlerSelector) == 8);
    assert!(size_of::<Anon13>() == 8);
    assert!(offset_of!(Anon13, xOffset) == 0);
    assert!(offset_of!(Anon13, yOffset) == 2);
    assert!(offset_of!(Anon13, shakes) == 4);
    assert!(offset_of!(Anon13, delay) == 6);
    assert!(size_of::<Anon14>() == 8);
    assert!(offset_of!(Anon14, x) == 0);
    assert!(offset_of!(Anon14, y) == 2);
    assert!(offset_of!(Anon14, relativeTo) == 4);
    assert!(offset_of!(Anon14, animation) == 6);
    assert!(size_of::<Anon15>() == 10);
    assert!(offset_of!(Anon15, x) == 0);
    assert!(offset_of!(Anon15, y) == 2);
    assert!(offset_of!(Anon15, relativeTo) == 4);
    assert!(offset_of!(Anon15, animation) == 6);
    assert!(offset_of!(Anon15, duration) == 8);
    assert!(size_of::<Anon16>() == 8);
    assert!(offset_of!(Anon16, x) == 0);
    assert!(offset_of!(Anon16, y) == 2);
    assert!(offset_of!(Anon16, relativeTo) == 4);
    assert!(offset_of!(Anon16, animation) == 6);
    assert!(size_of::<Anon17>() == 4);
    assert!(offset_of!(Anon17, relativeTo) == 0);
    assert!(offset_of!(Anon17, animation) == 2);
    assert!(size_of::<Anon18>() == 8);
    assert!(offset_of!(Anon18, relativeTo) == 0);
    assert!(offset_of!(Anon18, x) == 2);
    assert!(offset_of!(Anon18, y) == 4);
    assert!(offset_of!(Anon18, animation) == 6);
    assert!(size_of::<Anon19>() == 8);
    assert!(offset_of!(Anon19, x) == 0);
    assert!(offset_of!(Anon19, y) == 2);
    assert!(offset_of!(Anon19, relativeTo) == 4);
    assert!(offset_of!(Anon19, duration) == 6);
    assert!(size_of::<Anon20>() == 8);
    assert!(offset_of!(Anon20, x) == 0);
    assert!(offset_of!(Anon20, y) == 2);
    assert!(offset_of!(Anon20, relativeTo) == 4);
    assert!(offset_of!(Anon20, animation) == 6);
};

pub(crate) unsafe fn AnimConfusionDuck(sprite: *mut Sprite) {
    let cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
    (*sprite).x += (*cmd).x;
    (*sprite).y += (*cmd).y;
    (*sprite).data[0] = (*cmd).waveOffset;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).data[1] = -(*cmd).wavePeriod;
        (*sprite).data[4] = 1;
    } else {
        (*sprite).data[1] = (*cmd).wavePeriod;
        (*sprite).data[4] = 0;
        StartSpriteAnim(sprite, 1);
    }
    (*sprite).data[3] = (*cmd).duration;
    (*sprite).callback = Some(AnimConfusionDuck_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimConfusionDuck_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Cos((*sprite).data[0], 30);
    (*sprite).y2 = Sin((*sprite).data[0], 10);
    if ((*sprite).data[0] as u16) < 128 {
        (*sprite).oam.set_priority(1);
    } else {
        (*sprite).oam.set_priority(3);
    }
    (*sprite).data[0] = ((*sprite).data[0] + (*sprite).data[1]) & 0xFF;
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) == (*sprite).data[3]
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSimplePaletteBlend(sprite: *mut Sprite) {
    let cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
    let selectedPalettes: u32 = UnpackSelectedBattlePalettes((*cmd).selector);
    BeginNormalPaletteFade(
        selectedPalettes,
        (*cmd).delay as i8,
        (*cmd).initialBlendY as u8,
        (*cmd).targetBlendY as u8,
        (*cmd).color as u16,
    );
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(AnimSimplePaletteBlend_Step);
}
pub unsafe fn UnpackSelectedBattlePalettes(selector: i16) -> u32 {
    let battleBackground: u8 = selector as u8 & 1;
    let attacker: u8 = (selector >> 1) as u8 & 1;
    let target: u8 = (selector >> 2) as u8 & 1;
    let attackerPartner: u8 = (selector >> 3) as u8 & 1;
    let targetPartner: u8 = (selector >> 4) as u8 & 1;
    let anim1: u8 = (selector >> 5) as u8 & 1;
    let anim2: u8 = (selector >> 6) as u8 & 1;
    GetBattlePalettesMask(
        battleBackground,
        attacker,
        target,
        attackerPartner,
        targetPartner,
        anim1,
        anim2,
    )
}
pub(crate) unsafe fn AnimSimplePaletteBlend_Step(sprite: *mut Sprite) {
    if gPaletteFade.active() == 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimComplexPaletteBlend(sprite: *mut Sprite) {
    let cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
    (*sprite).data[0] = (*cmd).delay;
    (*sprite).data[sDelay] = (*cmd).delay;
    (*sprite).data[sNumBlends] = (*cmd).numBlends;
    (*sprite).data[sColor1] = (*cmd).color1;
    (*sprite).data[sBlendY1] = (*cmd).blendY1;
    (*sprite).data[sColor2] = (*cmd).color2;
    (*sprite).data[sBlendY2] = (*cmd).blendY2;
    (*sprite).data[sPaletteSelector] = (*cmd).selector;
    let selectedPalettes: u32 = UnpackSelectedBattlePalettes((*sprite).data[sPaletteSelector]);
    BlendPalettes(selectedPalettes, (*cmd).blendY1 as u8, (*cmd).color1 as u16);
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(AnimComplexPaletteBlend_Step1);
}
pub(crate) unsafe fn AnimComplexPaletteBlend_Step1(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if (*sprite).data[sNumBlends] == 0 {
        (*sprite).callback = Some(AnimComplexPaletteBlend_Step2);
        return;
    }
    let selectedPalettes: u32 = UnpackSelectedBattlePalettes((*sprite).data[sPaletteSelector]);
    if (*sprite).data[sDelay] as i32 & 0x100 != 0 {
        BlendPalettes(
            selectedPalettes,
            (*sprite).data[sBlendY1] as u8,
            (*sprite).data[sColor1] as u16,
        );
    } else {
        BlendPalettes(
            selectedPalettes,
            (*sprite).data[sBlendY2] as u8,
            (*sprite).data[sColor2] as u16,
        );
    }
    (*sprite).data[sDelay] ^= 0x100;
    (*sprite).data[0] = (*sprite).data[sDelay] & 0xFF;
    (*sprite).data[sNumBlends] -= 1;
}
pub(crate) unsafe fn AnimComplexPaletteBlend_Step2(sprite: *mut Sprite) {
    let mut selectedPalettes: u32 = 0;
    if gPaletteFade.active() == 0 {
        selectedPalettes = UnpackSelectedBattlePalettes((*sprite).data[sPaletteSelector]);
        BlendPalettes(selectedPalettes, 0, 0);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimCirclingSparkle(sprite: *mut Sprite) {
    let cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
    (*sprite).x += (*cmd).x;
    (*sprite).y += (*cmd).y;
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 10;
    (*sprite).data[2] = 8;
    (*sprite).data[3] = 40;
    (*sprite).data[4] = 112;
    (*sprite).data[5] = 0;
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteInGrowingCircle);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendColorCycle(taskId: u8) {
    let cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    task_set(taskId, tPalSelector, (*cmd).selector);
    task_set(taskId, tDelay, (*cmd).delay);
    task_set(taskId, tNumBlends, (*cmd).numBlends);
    task_set(taskId, tInitialBlendY, (*cmd).initialBlendY);
    task_set(taskId, tTargetBlendY, (*cmd).targetBlendY);
    task_set(taskId, tBlendColor, (*cmd).color);
    task_set(taskId, tRestoreBlend, FALSE as i16);
    BlendColorCycle(taskId, 0, task_get(taskId, tTargetBlendY) as u8);
    task_set_func(taskId, Some(AnimTask_BlendColorCycleLoop));
}
unsafe fn BlendColorCycle(taskId: u8, startBlendAmount: u8, targetBlendAmount: u8) {
    let selectedPalettes: u32 = UnpackSelectedBattlePalettes(task_get(taskId, tPalSelector));
    BeginNormalPaletteFade(
        selectedPalettes,
        task_get(taskId, tDelay) as i8,
        startBlendAmount,
        targetBlendAmount,
        task_get(taskId, tBlendColor) as u16,
    );
    task_set(taskId, tNumBlends, task_get(taskId, tNumBlends) - 1);
    task_set(taskId, tRestoreBlend, task_get(taskId, tRestoreBlend) ^ 1);
}
pub(crate) unsafe fn AnimTask_BlendColorCycleLoop(taskId: u8) {
    let mut startBlendAmount: u8 = 0;
    let mut targetBlendAmount: u8 = 0;
    if gPaletteFade.active() == 0 {
        if task_get(taskId, tNumBlends) > 0 {
            if task_get(taskId, tRestoreBlend) == 0 {
                startBlendAmount = task_get(taskId, tInitialBlendY) as u8;
                targetBlendAmount = task_get(taskId, tTargetBlendY) as u8;
            } else {
                startBlendAmount = task_get(taskId, tTargetBlendY) as u8;
                targetBlendAmount = task_get(taskId, tInitialBlendY) as u8;
            }
            if task_get(taskId, tNumBlends) == 1 {
                targetBlendAmount = 0;
            }
            BlendColorCycle(taskId, startBlendAmount, targetBlendAmount);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendColorCycleExclude(taskId: u8) {
    let cmd: *mut Anon6 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon6;
    let mut selectedPalettes: u32 = 0;
    task_set(taskId, 0, (*cmd).unk0);
    task_set(taskId, tDelay, (*cmd).delay);
    task_set(taskId, tNumBlends, (*cmd).numBlends);
    task_set(taskId, tInitialBlendY, (*cmd).initialBlendY);
    task_set(taskId, tTargetBlendY, (*cmd).targetBlendY);
    task_set(taskId, tBlendColor, (*cmd).color);
    task_set(taskId, tRestoreBlend, 0);
    let mut battler: i32 = 0;
    while battler < gBattlersCount as i32 {
        if battler != gBattleAnimAttacker as i32 && battler != gBattleAnimTarget as i32 {
            selectedPalettes |= shl_i32(1, battler as u32 + 16) as u32;
        }
        battler += 1;
    }
    if (*cmd).unk0 == 1 {
        selectedPalettes |= 0xE;
    }
    task_set(taskId, tPalSelectorHi, (selectedPalettes >> 16) as i16);
    task_set(taskId, tPalSelectorLo, selectedPalettes as i16 & 0xFF);
    BlendColorCycleExclude(taskId, 0, task_get(taskId, tTargetBlendY) as u8);
    task_set_func(taskId, Some(AnimTask_BlendColorCycleExcludeLoop));
}
unsafe fn BlendColorCycleExclude(taskId: u8, startBlendAmount: u8, targetBlendAmount: u8) {
    let selectedPalettes: u32 = (task_get(taskId, tPalSelectorHi) as u16 as u32) << 16
        | task_get(taskId, tPalSelectorLo) as u16 as u32;
    BeginNormalPaletteFade(
        selectedPalettes,
        task_get(taskId, tDelay) as i8,
        startBlendAmount,
        targetBlendAmount,
        task_get(taskId, tBlendColor) as u16,
    );
    task_set(taskId, tNumBlends, task_get(taskId, tNumBlends) - 1);
    task_set(taskId, tRestoreBlend, task_get(taskId, tRestoreBlend) ^ 1);
}
pub(crate) unsafe fn AnimTask_BlendColorCycleExcludeLoop(taskId: u8) {
    let mut startBlendAmount: u8 = 0;
    let mut targetBlendAmount: u8 = 0;
    if gPaletteFade.active() == 0 {
        if task_get(taskId, tNumBlends) > 0 {
            if task_get(taskId, tRestoreBlend) == 0 {
                startBlendAmount = task_get(taskId, tInitialBlendY) as u8;
                targetBlendAmount = task_get(taskId, tTargetBlendY) as u8;
            } else {
                startBlendAmount = task_get(taskId, tTargetBlendY) as u8;
                targetBlendAmount = task_get(taskId, tInitialBlendY) as u8;
            }
            if task_get(taskId, tNumBlends) == 1 {
                targetBlendAmount = 0;
            }
            BlendColorCycleExclude(taskId, startBlendAmount, targetBlendAmount);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendColorCycleByTag(taskId: u8) {
    let cmd: *mut Anon7 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon7;
    task_set(taskId, tPalTag, (*cmd).tag);
    task_set(taskId, tDelay, (*cmd).delay);
    task_set(taskId, tNumBlends, (*cmd).numBlends);
    task_set(taskId, tInitialBlendY, (*cmd).initialBlendY);
    task_set(taskId, tTargetBlendY, (*cmd).targetBlendY);
    task_set(taskId, tBlendColor, (*cmd).color);
    task_set(taskId, tRestoreBlend, FALSE as i16);
    BlendColorCycleByTag(taskId, 0, task_get(taskId, tTargetBlendY) as u8);
    task_set_func(taskId, Some(AnimTask_BlendColorCycleByTagLoop));
}
unsafe fn BlendColorCycleByTag(taskId: u8, startBlendAmount: u8, targetBlendAmount: u8) {
    let paletteIndex: u8 = IndexOfSpritePaletteTag(task_get(taskId, tPalTag) as u16);
    BeginNormalPaletteFade(
        shl_i32(1, paletteIndex as u32 + 16) as u32,
        task_get(taskId, tDelay) as i8,
        startBlendAmount,
        targetBlendAmount,
        task_get(taskId, tBlendColor) as u16,
    );
    task_set(taskId, tNumBlends, task_get(taskId, tNumBlends) - 1);
    task_set(taskId, tRestoreBlend, task_get(taskId, tRestoreBlend) ^ 1);
}
pub(crate) unsafe fn AnimTask_BlendColorCycleByTagLoop(taskId: u8) {
    let mut startBlendAmount: u8 = 0;
    let mut targetBlendAmount: u8 = 0;
    if gPaletteFade.active() == 0 {
        if task_get(taskId, tNumBlends) > 0 {
            if task_get(taskId, tRestoreBlend) == 0 {
                startBlendAmount = task_get(taskId, tInitialBlendY) as u8;
                targetBlendAmount = task_get(taskId, tTargetBlendY) as u8;
            } else {
                startBlendAmount = task_get(taskId, tTargetBlendY) as u8;
                targetBlendAmount = task_get(taskId, tInitialBlendY) as u8;
            }
            if task_get(taskId, tNumBlends) == 1 {
                targetBlendAmount = 0;
            }
            BlendColorCycleByTag(taskId, startBlendAmount, targetBlendAmount);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FlashAnimTagWithColor(taskId: u8) {
    let cmd: *mut Anon8 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon8;
    task_set(taskId, 0, (*cmd).delay);
    task_set(taskId, tDelay, (*cmd).delay);
    task_set(taskId, tNumBlends, (*cmd).numBlends);
    task_set(taskId, tColor1, (*cmd).color1);
    task_set(taskId, tBlendY1, (*cmd).blendY1);
    task_set(taskId, tColor2, (*cmd).color2);
    task_set(taskId, tBlendY2, (*cmd).blendY2);
    task_set(taskId, tAnimTag, (*cmd).tag);
    let paletteIndex: u8 = IndexOfSpritePaletteTag((*cmd).tag as u16);
    BeginNormalPaletteFade(
        shl_i32(1, paletteIndex as u32 + 16) as u32,
        0,
        (*cmd).blendY1 as u8,
        (*cmd).blendY1 as u8,
        (*cmd).color1 as u16,
    );
    task_set_func(taskId, Some(AnimTask_FlashAnimTagWithColor_Step1));
}
pub(crate) unsafe fn AnimTask_FlashAnimTagWithColor_Step1(taskId: u8) {
    if task_get(taskId, 0) > 0 {
        task_set(taskId, 0, task_get(taskId, 0) - 1);
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if task_get(taskId, tNumBlends) == 0 {
        task_set_func(taskId, Some(AnimTask_FlashAnimTagWithColor_Step2));
        return;
    }
    let selectedPalettes: u32 = shl_i32(
        1,
        IndexOfSpritePaletteTag(task_get(taskId, tAnimTag) as u16) as u32 + 16,
    ) as u32;
    if task_get(taskId, tDelay) as i32 & 0x100 != 0 {
        BeginNormalPaletteFade(
            selectedPalettes,
            0,
            task_get(taskId, tBlendY1) as u8,
            task_get(taskId, tBlendY1) as u8,
            task_get(taskId, tColor1) as u16,
        );
    } else {
        BeginNormalPaletteFade(
            selectedPalettes,
            0,
            task_get(taskId, tBlendY2) as u8,
            task_get(taskId, tBlendY2) as u8,
            task_get(taskId, tColor2) as u16,
        );
    }
    task_set(taskId, tDelay, task_get(taskId, tDelay) ^ 0x100);
    task_set(taskId, 0, task_get(taskId, tDelay) & 0xFF);
    task_set(taskId, tNumBlends, task_get(taskId, tNumBlends) - 1);
}
pub(crate) unsafe fn AnimTask_FlashAnimTagWithColor_Step2(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    if gPaletteFade.active() == 0 {
        selectedPalettes = shl_i32(
            1,
            IndexOfSpritePaletteTag(task_get(taskId, tAnimTag) as u16) as u32 + 16,
        ) as u32;
        BeginNormalPaletteFade(selectedPalettes, 0, 0, 0, 0);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_InvertScreenColor(taskId: u8) {
    let cmd: *mut Anon9 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon9;
    let mut selectedPalettes: u32 = 0;
    let attackerBattler: u8 = gBattleAnimAttacker;
    let targetBattler: u8 = gBattleAnimTarget;
    if (*cmd).flagsScenery as i32 & 256 != 0 {
        selectedPalettes = GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
    }
    if (*cmd).flagsAttacker as i32 & 256 != 0 {
        selectedPalettes |= shl_i32(0x10000, attackerBattler as u32) as u32;
    }
    if (*cmd).flagsTarget as i32 & 256 != 0 {
        selectedPalettes |= shl_i32(0x10000, targetBattler as u32) as u32;
    }
    InvertPlttBuffer(selectedPalettes);
    DestroyAnimVisualTask(taskId);
}
pub unsafe fn AnimTask_TintPalettes(taskId: u8) {
    let cmd: *mut Anon10 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon10;
    let mut paletteIndex: u8 = 0;
    let mut selectedPalettes: u32 = 0;
    if task_get(taskId, 0) == 0 {
        task_set(taskId, tFlagsScenery, (*cmd).flagsScenery);
        task_set(taskId, tFlagsAttacker, (*cmd).flagsAttacker);
        task_set(taskId, tFlagsTarget, (*cmd).flagsTarget);
        task_set(taskId, tLength, (*cmd).duration);
        task_set(taskId, tColorR, (*cmd).r);
        task_set(taskId, tColorG, (*cmd).g);
        task_set(taskId, tColorB, (*cmd).b);
    }
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    let attackerBattler: u8 = gBattleAnimAttacker;
    let targetBattler: u8 = gBattleAnimTarget;
    if task_get(taskId, tFlagsScenery) as i32 & 256 != 0 {
        selectedPalettes = PALETTES_BG;
    }
    if task_get(taskId, tFlagsScenery) as i32 & 1 != 0 {
        paletteIndex = IndexOfSpritePaletteTag(
            (*gSprites[gHealthboxSpriteIds[attackerBattler]].template).paletteTag,
        );
        selectedPalettes |= (shl_i32(1, paletteIndex as u32) as u32) << 16;
    }
    if task_get(taskId, tFlagsAttacker) as i32 & 256 != 0 {
        selectedPalettes |= (shl_i32(1, attackerBattler as u32) as u32) << 16;
    }
    if task_get(taskId, tFlagsTarget) as i32 & 256 != 0 {
        selectedPalettes |= (shl_i32(1, targetBattler as u32) as u32) << 16;
    }
    TintPlttBuffer(
        selectedPalettes,
        task_get(taskId, tColorR) as i8,
        task_get(taskId, tColorG) as i8,
        task_get(taskId, tColorB) as i8,
    );
    if task_get(taskId, 0) == task_get(taskId, tLength) {
        UnfadePlttBuffer(selectedPalettes);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimShakeMonOrBattlePlatforms(sprite: *mut Sprite) {
    let cmd: *mut Anon11 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon11;
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).data[sShakeVelocity] = -(*cmd).velocity;
    (*sprite).data[sShakeTimer] = (*cmd).shakeTimer;
    (*sprite).data[sShakeDuration] = (*cmd).shakeTimer;
    (*sprite).data[3] = (*cmd).shakeDuration;
    match (*cmd).r#type {
        SHAKE_BG_X => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe fn(*mut Sprite)>>(
                    &raw mut gBattle_BG3_X as *mut c_void,
                ),
            );
        }
        SHAKE_BG_Y => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe fn(*mut Sprite)>>(
                    &raw mut gBattle_BG3_Y as *mut c_void,
                ),
            );
        }
        SHAKE_MON_X => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe fn(*mut Sprite)>>(
                    &raw mut gSpriteCoordOffsetX as *mut c_void,
                ),
            );
        }
        _ => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe fn(*mut Sprite)>>(
                    &raw mut gSpriteCoordOffsetY as *mut c_void,
                ),
            );
        }
    }
    (*sprite).data[sOriginalValue] = *(((*sprite).data[sShakePtrLo] as i32
        | ((*sprite).data[sShakePtrHi] as i32) << 16)
        as usize as *mut u16) as i16;
    (*sprite).data[sType] = (*cmd).r#type;
    if (*sprite).data[sType] == SHAKE_MON_X || (*sprite).data[sType] == SHAKE_MON_Y {
        AnimShakeMonOrBattlePlatforms_UpdateCoordOffsetEnabled();
    }
    (*sprite).callback = Some(AnimShakeMonOrBattlePlatforms_Step);
}
pub(crate) unsafe fn AnimShakeMonOrBattlePlatforms_Step(sprite: *mut Sprite) {
    let mut i: u8 = 0;
    if (*sprite).data[3] > 0 {
        (*sprite).data[3] -= 1;
        if (*sprite).data[sShakeTimer] > 0 {
            (*sprite).data[sShakeTimer] -= 1;
        } else {
            (*sprite).data[sShakeTimer] = (*sprite).data[sShakeDuration];
            *(((*sprite).data[sShakePtrLo] as i32 | ((*sprite).data[sShakePtrHi] as i32) << 16)
                as usize as *mut u16) += (*sprite).data[sShakeVelocity] as u16;
            (*sprite).data[sShakeVelocity] = -(*sprite).data[sShakeVelocity];
        }
    } else {
        *(((*sprite).data[sShakePtrLo] as i32 | ((*sprite).data[sShakePtrHi] as i32) << 16) as usize
            as *mut u16) = (*sprite).data[sOriginalValue] as u16;
        if (*sprite).data[sType] == SHAKE_MON_X || (*sprite).data[sType] == SHAKE_MON_Y {
            i = 0;
            while i < gBattlersCount {
                gSprites[gBattlerSpriteIds[i]].set_coordOffsetEnabled(FALSE as u16);
                i += 1;
            }
        }
        DestroyAnimSprite(sprite);
    }
}
unsafe fn AnimShakeMonOrBattlePlatforms_UpdateCoordOffsetEnabled() {
    let cmd: *mut Anon12 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon12;
    gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].set_coordOffsetEnabled(FALSE as u16);
    gSprites[gBattlerSpriteIds[gBattleAnimTarget]].set_coordOffsetEnabled(FALSE as u16);
    if (*cmd).battlerSelector == 2 {
        gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].set_coordOffsetEnabled(TRUE as u16);
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]].set_coordOffsetEnabled(TRUE as u16);
    } else {
        if (*cmd).battlerSelector == 0 {
            gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].set_coordOffsetEnabled(TRUE as u16);
        } else {
            gSprites[gBattlerSpriteIds[gBattleAnimTarget]].set_coordOffsetEnabled(TRUE as u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeBattlePlatforms(taskId: u8) {
    let cmd: *mut Anon13 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon13;
    task_set(taskId, tXOffset, (*cmd).xOffset);
    task_set(taskId, tYOffset, (*cmd).yOffset);
    task_set(taskId, tNumShakes, (*cmd).shakes);
    task_set(taskId, 3, (*cmd).delay);
    task_set(taskId, tShakeDelay, (*cmd).delay);
    gBattle_BG3_X = (*cmd).xOffset as u16;
    gBattle_BG3_Y = (*cmd).yOffset as u16;
    task_set_func(taskId, Some(AnimTask_ShakeBattlePlatforms_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_ShakeBattlePlatforms_Step(taskId: u8) {
    if task_get(taskId, 3) == 0 {
        if gBattle_BG3_X as i32 == task_get(taskId, tXOffset) as i32 {
            gBattle_BG3_X = (task_get(taskId, tXOffset) as u16).wrapping_neg();
        } else {
            gBattle_BG3_X = task_get(taskId, tXOffset) as u16;
        }
        if gBattle_BG3_Y as i32 == -(task_get(taskId, tYOffset) as i32) {
            gBattle_BG3_Y = 0;
        } else {
            gBattle_BG3_Y = (task_get(taskId, tYOffset) as u16).wrapping_neg();
        }
        task_set(taskId, 3, task_get(taskId, tShakeDelay));
        if ({
            task_set(taskId, tNumShakes, task_get(taskId, tNumShakes) - 1);
            task_get(taskId, tNumShakes)
        }) == 0
        {
            gBattle_BG3_X = 0;
            gBattle_BG3_Y = 0;
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 3, task_get(taskId, 3) - 1);
    }
}
pub(crate) unsafe fn AnimHitSplatBasic(sprite: *mut Sprite) {
    let cmd: *mut Anon14 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon14;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimHitSplatPersistent(sprite: *mut Sprite) {
    let cmd: *mut Anon15 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon15;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSpriteAfterTimer));
}
pub(crate) unsafe fn AnimHitSplatHandleInvert(sprite: *mut Sprite) {
    let cmd: *mut Anon16 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER && IsContest() == 0 {
        (*cmd).y = -(*cmd).y;
    }
    AnimHitSplatBasic(sprite);
}
pub(crate) unsafe fn AnimHitSplatRandom(sprite: *mut Sprite) {
    let cmd: *mut Anon17 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon17;
    if (*cmd).animation == -1 {
        (*cmd).animation = Random2() as i16 & 3;
    }
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    } else {
        InitSpritePosToAnimTarget(sprite, FALSE);
    }
    (*sprite).x2 += (Random2() as i32 % 48) as i16 - 24;
    (*sprite).y2 += (Random2() as i32 % 24) as i16 - 12;
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe fn AnimHitSplatOnMonEdge(sprite: *mut Sprite) {
    let cmd: *mut Anon18 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon18;
    (*sprite).data[0] = GetAnimBattlerSpriteId((*cmd).relativeTo as u8) as i16;
    (*sprite).x = gSprites[(*sprite).data[0]].x + gSprites[(*sprite).data[0]].x2;
    (*sprite).y = gSprites[(*sprite).data[0]].y + gSprites[(*sprite).data[0]].y2;
    (*sprite).x2 = (*cmd).x;
    (*sprite).y2 = (*cmd).y;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe fn AnimCrossImpact(sprite: *mut Sprite) {
    let cmd: *mut Anon19 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon19;
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).data[0] = (*cmd).duration;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(WaitAnimForDuration);
}
pub(crate) unsafe fn AnimFlashingHitSplat(sprite: *mut Sprite) {
    let cmd: *mut Anon20 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon20;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).callback = Some(AnimFlashingHitSplat_Step);
}
pub(crate) unsafe fn AnimFlashingHitSplat_Step(sprite: *mut Sprite) {
    (*sprite).set_invisible((*sprite).invisible() ^ 1);
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] += 1;
        t1
    }) > 12
    {
        DestroyAnimSprite(sprite);
    }
}
