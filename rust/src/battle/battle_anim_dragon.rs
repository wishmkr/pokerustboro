//! Translated from `src/battle_anim_dragon.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattlerAttacker: u8;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static gSineTable: CArray<i16, 0>;
    static mut gTasks: CArray<Task, 0>;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn RunStoredCallbackWhenAnimEnds(a0: *mut Sprite);
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn SetAnimSpriteInitialXOffset(a0: *mut Sprite, a1: i16);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut Sprite);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAffineAnim(a0: *mut Sprite, a1: u8);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateSpriteLinearAndFlicker(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimOutrageFlame(sprite: *mut Sprite) {
    let mut cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
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
pub(crate) unsafe extern "C" fn StartDragonFireTranslation(sprite: *mut Sprite) {
    let mut cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
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
pub(crate) unsafe extern "C" fn AnimDragonRageFirePlume(sprite: *mut Sprite) {
    let mut cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
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
pub(crate) unsafe extern "C" fn AnimDragonFireToTarget(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        StartSpriteAffineAnim(sprite, 1);
    }
    StartDragonFireTranslation(sprite);
}
pub(crate) unsafe extern "C" fn AnimDragonDanceOrb(sprite: *mut Sprite) {
    let mut cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
    let mut r5: u16 = 0;
    let mut r0: u16 = 0;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = 1;
    (*sprite).data[6] = (*cmd).angle;
    r5 = GetBattlerSpriteCoordAttr(gBattlerAttacker, BATTLER_COORD_ATTR_HEIGHT) as u16;
    r0 = GetBattlerSpriteCoordAttr(gBattlerAttacker, BATTLER_COORD_ATTR_WIDTH) as u16;
    if r5 > r0 {
        (*sprite).data[7] = (r5 as i32 / 2) as i16;
    } else {
        (*sprite).data[7] = (r0 as i32 / 2) as i16;
    }
    (*sprite).x2 = Cos((*sprite).data[6], (*sprite).data[7]);
    (*sprite).y2 = Sin((*sprite).data[6], (*sprite).data[7]);
    (*sprite).callback = Some(AnimDragonDanceOrb_Step);
}
pub(crate) unsafe extern "C" fn AnimDragonDanceOrb_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[6] = (*sprite).data[6] - (*sprite).data[5] & 0xFF;
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
            (*sprite).data[6] = (*sprite).data[6] - (*sprite).data[5] & 0xFF;
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
pub unsafe extern "C" fn AnimTask_DragonDanceWaver(taskId: u8) {
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let mut task: *mut Task = &raw mut gTasks[taskId];
    let mut i: u16 = 0;
    let mut y: u8 = 0;
    if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
        scanlineParams.dmaDest = 67108884 as usize as *mut u16 as *mut c_void;
        (*task).data[2] = gBattle_BG1_X as i16;
    } else {
        scanlineParams.dmaDest = 67108888 as usize as *mut u16 as *mut c_void;
        (*task).data[2] = gBattle_BG2_X as i16;
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    y = GetBattlerYCoordWithElevation(gBattleAnimAttacker);
    (*task).data[3] = y as i16 - 32;
    (*task).data[4] = y as i16 + 32;
    if (*task).data[3] < 0 {
        (*task).data[3] = 0;
    }
    i = (*task).data[3] as u16;
    while i as i32 <= (*task).data[4] as i32 {
        gScanlineEffectRegBuffers[0][i] = (*task).data[2] as u16;
        gScanlineEffectRegBuffers[1][i] = (*task).data[2] as u16;
        i += 1;
    }
    ScanlineEffect_SetParams(scanlineParams);
    (*task).func = Some(AnimTask_DragonDanceWaver_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_DragonDanceWaver_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
            gScanlineEffect.state = 3;
            (*task).data[0] += 1;
        }
        4 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn UpdateDragonDanceScanlineEffect(task: *mut Task) {
    let mut sineIndex: u16 = (*task).data[5] as u16;
    let mut i: u16 = 0;
    i = (*task).data[3] as u16;
    while i as i32 <= (*task).data[4] as i32 {
        gScanlineEffectRegBuffers[gScanlineEffect.srcBuffer][i] =
            (gSineTable[sineIndex] as i32 * (*task).data[6] as i32 >> 7) as u16
                + (*task).data[2] as u16;
        sineIndex = sineIndex + 8 & 0xFF;
        i += 1;
    }
    (*task).data[5] = (*task).data[5] + 9 & 0xFF;
}
pub(crate) unsafe extern "C" fn AnimOverheatFlame(sprite: *mut Sprite) {
    let mut cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    let mut i: i32 = 0;
    let mut yAmplitude: i32 = (*cmd).unk2 as i32 * 3 / 5;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).y;
    (*sprite).data[1] = Cos((*cmd).unk1, (*cmd).unk2);
    (*sprite).data[2] = Sin((*cmd).unk1, yAmplitude as i16);
    (*sprite).x += (*sprite).data[1] * (*cmd).speed;
    (*sprite).y += (*sprite).data[2] * (*cmd).speed;
    (*sprite).data[3] = (*cmd).duration;
    (*sprite).callback = Some(AnimOverheatFlame_Step);
    i = 0;
    while i < 7 {
        sUnusedOverheatData[i] = (*sprite).data[i] as u16;
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AnimOverheatFlame_Step(sprite: *mut Sprite) {
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
