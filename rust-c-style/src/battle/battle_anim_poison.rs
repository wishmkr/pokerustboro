//! Translated from `src/battle_anim_poison.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAnim_ToxicBubble sAnims_ToxicBubble gToxicBubbleSpriteTemplate sAnim_PoisonProjectile sAnim_AcidPoisonDroplet sAnim_SludgeBombHit sAnims_PoisonProjectile sAffineAnim_PoisonProjectile sAffineAnim_SludgeBombHit sAffineAnims_PoisonProjectile sAffineAnims_SludgeBombHit gSludgeProjectileSpriteTemplate gAcidPoisonBubbleSpriteTemplate gSludgeBombHitParticleSpriteTemplate sAffineAnim_AcidPoisonDroplet gAffineAnims_Droplet gAcidPoisonDropletSpriteTemplate sAffineAnim_Bubble sAffineAnims_Bubble gPoisonBubbleSpriteTemplate gWaterBubbleSpriteTemplate

unsafe extern "C" {
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitSpriteDataForLinearTranslation(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
    fn TranslateSpriteLinearFixedPoint(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimSludgeProjectile(sprite: *mut Sprite) {
    if gBattleAnimArgs[3] == 0 {
        StartSpriteAnim(sprite, 2);
    }
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[5] = -30;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimSludgeProjectile_Step);
}
pub(crate) unsafe extern "C" fn AnimSludgeProjectile_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimAcidPoisonBubble(sprite: *mut Sprite) {
    let mut l1: i16 = 0;
    let mut l2: i16 = 0;
    if gBattleAnimArgs[3] == 0 {
        StartSpriteAnim(sprite, 2);
    }
    InitSpritePosToAnimAttacker(sprite, TRUE);
    SetAverageBattlerPositions(gBattleAnimTarget, TRUE, &raw mut l1, &raw mut l2);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        gBattleAnimArgs[4] = -gBattleAnimArgs[4];
    }
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[2] = l1 + gBattleAnimArgs[4];
    (*sprite).data[4] = l2 + gBattleAnimArgs[5];
    (*sprite).data[5] = -30;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimAcidPoisonBubble_Step);
}
pub(crate) unsafe extern "C" fn AnimAcidPoisonBubble_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSludgeBombHitParticle(sprite: *mut Sprite) {
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[0];
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[1];
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[5] = div_i32((*sprite).data[1] as i32, gBattleAnimArgs[2] as i32) as i16;
    (*sprite).data[6] = div_i32((*sprite).data[2] as i32, gBattleAnimArgs[2] as i32) as i16;
    (*sprite).callback = Some(AnimSludgeBombHitParticle_Step);
}
pub(crate) unsafe extern "C" fn AnimSludgeBombHitParticle_Step(sprite: *mut Sprite) {
    TranslateSpriteLinearFixedPoint(sprite);
    (*sprite).data[1] -= (*sprite).data[5];
    (*sprite).data[2] -= (*sprite).data[6];
    if (*sprite).data[0] == 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimAcidPoisonDroplet(sprite: *mut Sprite) {
    SetAverageBattlerPositions(
        gBattleAnimTarget,
        TRUE,
        &raw mut (*sprite).x,
        &raw mut (*sprite).y,
    );
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
    }
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    (*sprite).data[4] = (*sprite).y + (*sprite).data[0];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimBubbleEffect(sprite: *mut Sprite) {
    if gBattleAnimArgs[2] == 0 {
        InitSpritePosToAnimTarget(sprite, TRUE);
    } else {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            TRUE,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            gBattleAnimArgs[0] = -gBattleAnimArgs[0];
        }
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
    }
    (*sprite).callback = Some(AnimBubbleEffect_Step);
}
pub(crate) unsafe extern "C" fn AnimBubbleEffect_Step(sprite: *mut Sprite) {
    (*sprite).data[0] = (*sprite).data[0] + 0xB & 0xFF;
    (*sprite).x2 = Sin((*sprite).data[0], 4);
    (*sprite).data[1] += 0x30;
    (*sprite).y2 = -((*sprite).data[1] >> 8);
    if (*sprite).affineAnimEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
