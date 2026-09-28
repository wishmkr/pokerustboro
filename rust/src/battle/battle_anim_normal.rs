//! Translated from `src/battle_anim_normal.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gBattlersCount: u8;
    static mut gHealthboxSpriteIds: CArray<u8, 4>;
    static mut gPaletteFade: PaletteFadeControl;
    static mut gSpriteCoordOffsetX: i16;
    static mut gSpriteCoordOffsetY: i16;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimSpriteAfterTimer(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerSide(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn InvertPlttBuffer(a0: u32);
    fn IsContest() -> u8;
    fn Random2() -> u16;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut Sprite);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TintPlttBuffer(a0: u32, a1: i8, a2: i8, a3: i8);
    fn TranslateSpriteInGrowingCircle(a0: *mut Sprite);
    fn UnfadePlttBuffer(a0: u32);
    fn WaitAnimForDuration(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimConfusionDuck(sprite: *mut Sprite) {
    let mut cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
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
pub(crate) unsafe extern "C" fn AnimConfusionDuck_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Cos((*sprite).data[0], 30);
    (*sprite).y2 = Sin((*sprite).data[0], 10);
    if ((*sprite).data[0] as u16) < 128 {
        (*sprite).oam.set_priority(1);
    } else {
        (*sprite).oam.set_priority(3);
    }
    (*sprite).data[0] = (*sprite).data[0] + (*sprite).data[1] & 0xFF;
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) == (*sprite).data[3]
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSimplePaletteBlend(sprite: *mut Sprite) {
    let mut cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
    let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes((*cmd).selector);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnpackSelectedBattlePalettes(selector: i16) -> u32 {
    let mut battleBackground: u8 = selector as u8 & 1;
    let mut attacker: u8 = (selector >> 1) as u8 & 1;
    let mut target: u8 = (selector >> 2) as u8 & 1;
    let mut attackerPartner: u8 = (selector >> 3) as u8 & 1;
    let mut targetPartner: u8 = (selector >> 4) as u8 & 1;
    let mut anim1: u8 = (selector >> 5) as u8 & 1;
    let mut anim2: u8 = (selector >> 6) as u8 & 1;
    return GetBattlePalettesMask(
        battleBackground,
        attacker,
        target,
        attackerPartner,
        targetPartner,
        anim1,
        anim2,
    );
}
pub(crate) unsafe extern "C" fn AnimSimplePaletteBlend_Step(sprite: *mut Sprite) {
    if gPaletteFade.active() == 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimComplexPaletteBlend(sprite: *mut Sprite) {
    let mut cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
    let mut selectedPalettes: u32 = 0;
    (*sprite).data[0] = (*cmd).delay;
    (*sprite).data[1] = (*cmd).delay;
    (*sprite).data[2] = (*cmd).numBlends;
    (*sprite).data[3] = (*cmd).color1;
    (*sprite).data[4] = (*cmd).blendY1;
    (*sprite).data[5] = (*cmd).color2;
    (*sprite).data[6] = (*cmd).blendY2;
    (*sprite).data[7] = (*cmd).selector;
    selectedPalettes = UnpackSelectedBattlePalettes((*sprite).data[7]);
    BlendPalettes(selectedPalettes, (*cmd).blendY1 as u8, (*cmd).color1 as u16);
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(AnimComplexPaletteBlend_Step1);
}
pub(crate) unsafe extern "C" fn AnimComplexPaletteBlend_Step1(sprite: *mut Sprite) {
    let mut selectedPalettes: u32 = 0;
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if (*sprite).data[2] == 0 {
        (*sprite).callback = Some(AnimComplexPaletteBlend_Step2);
        return;
    }
    selectedPalettes = UnpackSelectedBattlePalettes((*sprite).data[7]);
    if (*sprite).data[1] as i32 & 0x100 != 0 {
        BlendPalettes(
            selectedPalettes,
            (*sprite).data[4] as u8,
            (*sprite).data[3] as u16,
        );
    } else {
        BlendPalettes(
            selectedPalettes,
            (*sprite).data[6] as u8,
            (*sprite).data[5] as u16,
        );
    }
    (*sprite).data[1] ^= 0x100;
    (*sprite).data[0] = (*sprite).data[1] & 0xFF;
    (*sprite).data[2] -= 1;
}
pub(crate) unsafe extern "C" fn AnimComplexPaletteBlend_Step2(sprite: *mut Sprite) {
    let mut selectedPalettes: u32 = 0;
    if gPaletteFade.active() == 0 {
        selectedPalettes = UnpackSelectedBattlePalettes((*sprite).data[7]);
        BlendPalettes(selectedPalettes, 0, 0);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimCirclingSparkle(sprite: *mut Sprite) {
    let mut cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
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
pub unsafe extern "C" fn AnimTask_BlendColorCycle(taskId: u8) {
    let mut cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    gTasks[taskId].data[0] = (*cmd).selector;
    gTasks[taskId].data[1] = (*cmd).delay;
    gTasks[taskId].data[2] = (*cmd).numBlends;
    gTasks[taskId].data[3] = (*cmd).initialBlendY;
    gTasks[taskId].data[4] = (*cmd).targetBlendY;
    gTasks[taskId].data[5] = (*cmd).color;
    gTasks[taskId].data[8] = FALSE as i16;
    BlendColorCycle(taskId, 0, gTasks[taskId].data[4] as u8);
    gTasks[taskId].func = Some(AnimTask_BlendColorCycleLoop);
}
pub(crate) unsafe extern "C" fn BlendColorCycle(
    taskId: u8,
    startBlendAmount: u8,
    targetBlendAmount: u8,
) {
    let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(gTasks[taskId].data[0]);
    BeginNormalPaletteFade(
        selectedPalettes,
        gTasks[taskId].data[1] as i8,
        startBlendAmount,
        targetBlendAmount,
        gTasks[taskId].data[5] as u16,
    );
    gTasks[taskId].data[2] -= 1;
    gTasks[taskId].data[8] ^= 1;
}
pub(crate) unsafe extern "C" fn AnimTask_BlendColorCycleLoop(taskId: u8) {
    let mut startBlendAmount: u8 = 0;
    let mut targetBlendAmount: u8 = 0;
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[2] > 0 {
            if gTasks[taskId].data[8] == 0 {
                startBlendAmount = gTasks[taskId].data[3] as u8;
                targetBlendAmount = gTasks[taskId].data[4] as u8;
            } else {
                startBlendAmount = gTasks[taskId].data[4] as u8;
                targetBlendAmount = gTasks[taskId].data[3] as u8;
            }
            if gTasks[taskId].data[2] == 1 {
                targetBlendAmount = 0;
            }
            BlendColorCycle(taskId, startBlendAmount, targetBlendAmount);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendColorCycleExclude(taskId: u8) {
    let mut cmd: *mut Anon6 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon6;
    let mut battler: i32 = 0;
    let mut selectedPalettes: u32 = 0;
    gTasks[taskId].data[0] = (*cmd).unk0;
    gTasks[taskId].data[1] = (*cmd).delay;
    gTasks[taskId].data[2] = (*cmd).numBlends;
    gTasks[taskId].data[3] = (*cmd).initialBlendY;
    gTasks[taskId].data[4] = (*cmd).targetBlendY;
    gTasks[taskId].data[5] = (*cmd).color;
    gTasks[taskId].data[8] = 0;
    battler = 0;
    while battler < gBattlersCount as i32 {
        if battler != gBattleAnimAttacker as i32 && battler != gBattleAnimTarget as i32 {
            selectedPalettes |= shl_i32(1, battler as u32 + 16) as u32;
        }
        battler += 1;
    }
    if (*cmd).unk0 == 1 {
        selectedPalettes |= 0xE;
    }
    gTasks[taskId].data[9] = (selectedPalettes >> 16) as i16;
    gTasks[taskId].data[10] = selectedPalettes as i16 & 0xFF;
    BlendColorCycleExclude(taskId, 0, gTasks[taskId].data[4] as u8);
    gTasks[taskId].func = Some(AnimTask_BlendColorCycleExcludeLoop);
}
pub(crate) unsafe extern "C" fn BlendColorCycleExclude(
    taskId: u8,
    startBlendAmount: u8,
    targetBlendAmount: u8,
) {
    let mut selectedPalettes: u32 =
        (gTasks[taskId].data[9] as u16 as u32) << 16 | gTasks[taskId].data[10] as u16 as u32;
    BeginNormalPaletteFade(
        selectedPalettes,
        gTasks[taskId].data[1] as i8,
        startBlendAmount,
        targetBlendAmount,
        gTasks[taskId].data[5] as u16,
    );
    gTasks[taskId].data[2] -= 1;
    gTasks[taskId].data[8] ^= 1;
}
pub(crate) unsafe extern "C" fn AnimTask_BlendColorCycleExcludeLoop(taskId: u8) {
    let mut startBlendAmount: u8 = 0;
    let mut targetBlendAmount: u8 = 0;
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[2] > 0 {
            if gTasks[taskId].data[8] == 0 {
                startBlendAmount = gTasks[taskId].data[3] as u8;
                targetBlendAmount = gTasks[taskId].data[4] as u8;
            } else {
                startBlendAmount = gTasks[taskId].data[4] as u8;
                targetBlendAmount = gTasks[taskId].data[3] as u8;
            }
            if gTasks[taskId].data[2] == 1 {
                targetBlendAmount = 0;
            }
            BlendColorCycleExclude(taskId, startBlendAmount, targetBlendAmount);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendColorCycleByTag(taskId: u8) {
    let mut cmd: *mut Anon7 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon7;
    gTasks[taskId].data[0] = (*cmd).tag;
    gTasks[taskId].data[1] = (*cmd).delay;
    gTasks[taskId].data[2] = (*cmd).numBlends;
    gTasks[taskId].data[3] = (*cmd).initialBlendY;
    gTasks[taskId].data[4] = (*cmd).targetBlendY;
    gTasks[taskId].data[5] = (*cmd).color;
    gTasks[taskId].data[8] = FALSE as i16;
    BlendColorCycleByTag(taskId, 0, gTasks[taskId].data[4] as u8);
    gTasks[taskId].func = Some(AnimTask_BlendColorCycleByTagLoop);
}
pub(crate) unsafe extern "C" fn BlendColorCycleByTag(
    taskId: u8,
    startBlendAmount: u8,
    targetBlendAmount: u8,
) {
    let mut paletteIndex: u8 = IndexOfSpritePaletteTag(gTasks[taskId].data[0] as u16);
    BeginNormalPaletteFade(
        shl_i32(1, paletteIndex as u32 + 16) as u32,
        gTasks[taskId].data[1] as i8,
        startBlendAmount,
        targetBlendAmount,
        gTasks[taskId].data[5] as u16,
    );
    gTasks[taskId].data[2] -= 1;
    gTasks[taskId].data[8] ^= 1;
}
pub(crate) unsafe extern "C" fn AnimTask_BlendColorCycleByTagLoop(taskId: u8) {
    let mut startBlendAmount: u8 = 0;
    let mut targetBlendAmount: u8 = 0;
    if gPaletteFade.active() == 0 {
        if gTasks[taskId].data[2] > 0 {
            if gTasks[taskId].data[8] == 0 {
                startBlendAmount = gTasks[taskId].data[3] as u8;
                targetBlendAmount = gTasks[taskId].data[4] as u8;
            } else {
                startBlendAmount = gTasks[taskId].data[4] as u8;
                targetBlendAmount = gTasks[taskId].data[3] as u8;
            }
            if gTasks[taskId].data[2] == 1 {
                targetBlendAmount = 0;
            }
            BlendColorCycleByTag(taskId, startBlendAmount, targetBlendAmount);
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FlashAnimTagWithColor(taskId: u8) {
    let mut cmd: *mut Anon8 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon8;
    let mut paletteIndex: u8 = 0;
    gTasks[taskId].data[0] = (*cmd).delay;
    gTasks[taskId].data[1] = (*cmd).delay;
    gTasks[taskId].data[2] = (*cmd).numBlends;
    gTasks[taskId].data[3] = (*cmd).color1;
    gTasks[taskId].data[4] = (*cmd).blendY1;
    gTasks[taskId].data[5] = (*cmd).color2;
    gTasks[taskId].data[6] = (*cmd).blendY2;
    gTasks[taskId].data[7] = (*cmd).tag;
    paletteIndex = IndexOfSpritePaletteTag((*cmd).tag as u16);
    BeginNormalPaletteFade(
        shl_i32(1, paletteIndex as u32 + 16) as u32,
        0,
        (*cmd).blendY1 as u8,
        (*cmd).blendY1 as u8,
        (*cmd).color1 as u16,
    );
    gTasks[taskId].func = Some(AnimTask_FlashAnimTagWithColor_Step1);
}
pub(crate) unsafe extern "C" fn AnimTask_FlashAnimTagWithColor_Step1(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    if gTasks[taskId].data[0] > 0 {
        gTasks[taskId].data[0] -= 1;
        return;
    }
    if gPaletteFade.active() != 0 {
        return;
    }
    if gTasks[taskId].data[2] == 0 {
        gTasks[taskId].func = Some(AnimTask_FlashAnimTagWithColor_Step2);
        return;
    }
    selectedPalettes = shl_i32(
        1,
        IndexOfSpritePaletteTag(gTasks[taskId].data[7] as u16) as u32 + 16,
    ) as u32;
    if gTasks[taskId].data[1] as i32 & 0x100 != 0 {
        BeginNormalPaletteFade(
            selectedPalettes,
            0,
            gTasks[taskId].data[4] as u8,
            gTasks[taskId].data[4] as u8,
            gTasks[taskId].data[3] as u16,
        );
    } else {
        BeginNormalPaletteFade(
            selectedPalettes,
            0,
            gTasks[taskId].data[6] as u8,
            gTasks[taskId].data[6] as u8,
            gTasks[taskId].data[5] as u16,
        );
    }
    gTasks[taskId].data[1] ^= 0x100;
    gTasks[taskId].data[0] = gTasks[taskId].data[1] & 0xFF;
    gTasks[taskId].data[2] -= 1;
}
pub(crate) unsafe extern "C" fn AnimTask_FlashAnimTagWithColor_Step2(taskId: u8) {
    let mut selectedPalettes: u32 = 0;
    if gPaletteFade.active() == 0 {
        selectedPalettes = shl_i32(
            1,
            IndexOfSpritePaletteTag(gTasks[taskId].data[7] as u16) as u32 + 16,
        ) as u32;
        BeginNormalPaletteFade(selectedPalettes, 0, 0, 0, 0);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_InvertScreenColor(taskId: u8) {
    let mut cmd: *mut Anon9 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon9;
    let mut selectedPalettes: u32 = 0;
    let mut attackerBattler: u8 = gBattleAnimAttacker;
    let mut targetBattler: u8 = gBattleAnimTarget;
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TintPalettes(taskId: u8) {
    let mut cmd: *mut Anon10 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon10;
    let mut attackerBattler: u8 = 0;
    let mut targetBattler: u8 = 0;
    let mut paletteIndex: u8 = 0;
    let mut selectedPalettes: u32 = 0;
    if gTasks[taskId].data[0] == 0 {
        gTasks[taskId].data[2] = (*cmd).flagsScenery;
        gTasks[taskId].data[3] = (*cmd).flagsAttacker;
        gTasks[taskId].data[4] = (*cmd).flagsTarget;
        gTasks[taskId].data[1] = (*cmd).duration;
        gTasks[taskId].data[5] = (*cmd).r;
        gTasks[taskId].data[6] = (*cmd).g;
        gTasks[taskId].data[7] = (*cmd).b;
    }
    gTasks[taskId].data[0] += 1;
    attackerBattler = gBattleAnimAttacker;
    targetBattler = gBattleAnimTarget;
    if gTasks[taskId].data[2] as i32 & 256 != 0 {
        selectedPalettes = PALETTES_BG;
    }
    if gTasks[taskId].data[2] as i32 & 1 != 0 {
        paletteIndex = IndexOfSpritePaletteTag(
            (*gSprites[gHealthboxSpriteIds[attackerBattler]].template).paletteTag,
        );
        selectedPalettes |= (shl_i32(1, paletteIndex as u32) as u32) << 16;
    }
    if gTasks[taskId].data[3] as i32 & 256 != 0 {
        selectedPalettes |= (shl_i32(1, attackerBattler as u32) as u32) << 16;
    }
    if gTasks[taskId].data[4] as i32 & 256 != 0 {
        selectedPalettes |= (shl_i32(1, targetBattler as u32) as u32) << 16;
    }
    TintPlttBuffer(
        selectedPalettes,
        gTasks[taskId].data[5] as i8,
        gTasks[taskId].data[6] as i8,
        gTasks[taskId].data[7] as i8,
    );
    if gTasks[taskId].data[0] == gTasks[taskId].data[1] {
        UnfadePlttBuffer(selectedPalettes);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimShakeMonOrBattlePlatforms(sprite: *mut Sprite) {
    let mut cmd: *mut Anon11 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon11;
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).data[0] = -(*cmd).velocity;
    (*sprite).data[1] = (*cmd).shakeTimer;
    (*sprite).data[2] = (*cmd).shakeTimer;
    (*sprite).data[3] = (*cmd).shakeDuration;
    match (*cmd).r#type {
        SHAKE_BG_X => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut Sprite)>>(
                    &raw mut gBattle_BG3_X as *mut c_void,
                ),
            );
        }
        SHAKE_BG_Y => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut Sprite)>>(
                    &raw mut gBattle_BG3_Y as *mut c_void,
                ),
            );
        }
        SHAKE_MON_X => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut Sprite)>>(
                    &raw mut gSpriteCoordOffsetX as *mut c_void,
                ),
            );
        }
        _ => {
            StoreSpriteCallbackInData6(
                sprite,
                core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut Sprite)>>(
                    &raw mut gSpriteCoordOffsetY as *mut c_void,
                ),
            );
        }
    }
    (*sprite).data[4] = *(((*sprite).data[6] as i32 | ((*sprite).data[7] as i32) << 16) as usize
        as *mut u16) as i16;
    (*sprite).data[5] = (*cmd).r#type;
    if (*sprite).data[5] == SHAKE_MON_X || (*sprite).data[5] == SHAKE_MON_Y {
        AnimShakeMonOrBattlePlatforms_UpdateCoordOffsetEnabled();
    }
    (*sprite).callback = Some(AnimShakeMonOrBattlePlatforms_Step);
}
pub(crate) unsafe extern "C" fn AnimShakeMonOrBattlePlatforms_Step(sprite: *mut Sprite) {
    let mut i: u8 = 0;
    if (*sprite).data[3] > 0 {
        (*sprite).data[3] -= 1;
        if (*sprite).data[1] > 0 {
            (*sprite).data[1] -= 1;
        } else {
            (*sprite).data[1] = (*sprite).data[2];
            *(((*sprite).data[6] as i32 | ((*sprite).data[7] as i32) << 16) as usize
                as *mut u16) += (*sprite).data[0] as u16;
            (*sprite).data[0] = -(*sprite).data[0];
        }
    } else {
        *(((*sprite).data[6] as i32 | ((*sprite).data[7] as i32) << 16) as usize as *mut u16) =
            (*sprite).data[4] as u16;
        if (*sprite).data[5] == SHAKE_MON_X || (*sprite).data[5] == SHAKE_MON_Y {
            i = 0;
            while i < gBattlersCount {
                gSprites[gBattlerSpriteIds[i]].set_coordOffsetEnabled(FALSE as u16);
                i += 1;
            }
        }
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimShakeMonOrBattlePlatforms_UpdateCoordOffsetEnabled() {
    let mut cmd: *mut Anon12 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon12;
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
pub unsafe extern "C" fn AnimTask_ShakeBattlePlatforms(taskId: u8) {
    let mut cmd: *mut Anon13 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon13;
    gTasks[taskId].data[0] = (*cmd).xOffset;
    gTasks[taskId].data[1] = (*cmd).yOffset;
    gTasks[taskId].data[2] = (*cmd).shakes;
    gTasks[taskId].data[3] = (*cmd).delay;
    gTasks[taskId].data[8] = (*cmd).delay;
    gBattle_BG3_X = (*cmd).xOffset as u16;
    gBattle_BG3_Y = (*cmd).yOffset as u16;
    gTasks[taskId].func = Some(AnimTask_ShakeBattlePlatforms_Step);
    gTasks[taskId].func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn AnimTask_ShakeBattlePlatforms_Step(taskId: u8) {
    if gTasks[taskId].data[3] == 0 {
        if gBattle_BG3_X as i32 == gTasks[taskId].data[0] as i32 {
            gBattle_BG3_X = (gTasks[taskId].data[0] as u16).wrapping_neg();
        } else {
            gBattle_BG3_X = gTasks[taskId].data[0] as u16;
        }
        if gBattle_BG3_Y as i32 == -(gTasks[taskId].data[1] as i32) {
            gBattle_BG3_Y = 0;
        } else {
            gBattle_BG3_Y = (gTasks[taskId].data[1] as u16).wrapping_neg();
        }
        gTasks[taskId].data[3] = gTasks[taskId].data[8];
        if ({
            gTasks[taskId].data[2] -= 1;
            gTasks[taskId].data[2]
        }) == 0
        {
            gBattle_BG3_X = 0;
            gBattle_BG3_Y = 0;
            DestroyAnimVisualTask(taskId);
        }
    } else {
        gTasks[taskId].data[3] -= 1;
    }
}
pub(crate) unsafe extern "C" fn AnimHitSplatBasic(sprite: *mut Sprite) {
    let mut cmd: *mut Anon14 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon14;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimHitSplatPersistent(sprite: *mut Sprite) {
    let mut cmd: *mut Anon15 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon15;
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
pub(crate) unsafe extern "C" fn AnimHitSplatHandleInvert(sprite: *mut Sprite) {
    let mut cmd: *mut Anon16 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER && IsContest() == 0 {
        (*cmd).y = -(*cmd).y;
    }
    AnimHitSplatBasic(sprite);
}
pub(crate) unsafe extern "C" fn AnimHitSplatRandom(sprite: *mut Sprite) {
    let mut cmd: *mut Anon17 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon17;
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
pub(crate) unsafe extern "C" fn AnimHitSplatOnMonEdge(sprite: *mut Sprite) {
    let mut cmd: *mut Anon18 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon18;
    (*sprite).data[0] = GetAnimBattlerSpriteId((*cmd).relativeTo as u8) as i16;
    (*sprite).x = gSprites[(*sprite).data[0]].x + gSprites[(*sprite).data[0]].x2;
    (*sprite).y = gSprites[(*sprite).data[0]].y + gSprites[(*sprite).data[0]].y2;
    (*sprite).x2 = (*cmd).x;
    (*sprite).y2 = (*cmd).y;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe extern "C" fn AnimCrossImpact(sprite: *mut Sprite) {
    let mut cmd: *mut Anon19 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon19;
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).data[0] = (*cmd).duration;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(WaitAnimForDuration);
}
pub(crate) unsafe extern "C" fn AnimFlashingHitSplat(sprite: *mut Sprite) {
    let mut cmd: *mut Anon20 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon20;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).callback = Some(AnimFlashingHitSplat_Step);
}
pub(crate) unsafe extern "C" fn AnimFlashingHitSplat_Step(sprite: *mut Sprite) {
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
