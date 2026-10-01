//! Translated from `src/battle_anim_poison.c` by tools/rustport/c2rs.py.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    static_mut_refs,
    unsafe_op_in_unsafe_fn,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons,
    dangerous_implicit_autorefs,
    overflowing_literals
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{DestroyAnimSprite, gBattleAnimAttacker, gBattleAnimTarget};
use crate::battle_anim_mons::{
    GetBattlerSide, GetBattlerSpriteCoord, InitAnimArcTranslation,
    InitSpriteDataForLinearTranslation, InitSpritePosToAnimAttacker, InitSpritePosToAnimTarget,
    SetAverageBattlerPositions, StartAnimLinearTranslation, StoreSpriteCallbackInData6,
    TranslateAnimHorizontalArc, TranslateSpriteLinearFixedPoint,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::trig::Sin;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// Data tables (translate with cdata.py): sAnim_ToxicBubble sAnims_ToxicBubble gToxicBubbleSpriteTemplate sAnim_PoisonProjectile sAnim_AcidPoisonDroplet sAnim_SludgeBombHit sAnims_PoisonProjectile sAffineAnim_PoisonProjectile sAffineAnim_SludgeBombHit sAffineAnims_PoisonProjectile sAffineAnims_SludgeBombHit gSludgeProjectileSpriteTemplate gAcidPoisonBubbleSpriteTemplate gSludgeBombHitParticleSpriteTemplate sAffineAnim_AcidPoisonDroplet gAffineAnims_Droplet gAcidPoisonDropletSpriteTemplate sAffineAnim_Bubble sAffineAnims_Bubble gPoisonBubbleSpriteTemplate gWaterBubbleSpriteTemplate

pub(crate) unsafe fn AnimSludgeProjectile(sprite: *mut Sprite) {
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
pub(crate) unsafe fn AnimSludgeProjectile_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimAcidPoisonBubble(sprite: *mut Sprite) {
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
pub(crate) unsafe fn AnimAcidPoisonBubble_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSludgeBombHitParticle(sprite: *mut Sprite) {
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
pub(crate) unsafe fn AnimSludgeBombHitParticle_Step(sprite: *mut Sprite) {
    TranslateSpriteLinearFixedPoint(sprite);
    (*sprite).data[1] -= (*sprite).data[5];
    (*sprite).data[2] -= (*sprite).data[6];
    if (*sprite).data[0] == 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimAcidPoisonDroplet(sprite: *mut Sprite) {
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
pub(crate) unsafe fn AnimBubbleEffect(sprite: *mut Sprite) {
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
pub(crate) unsafe fn AnimBubbleEffect_Step(sprite: *mut Sprite) {
    (*sprite).data[0] = ((*sprite).data[0] + 0xB) & 0xFF;
    (*sprite).x2 = Sin((*sprite).data[0], 4);
    (*sprite).data[1] += 0x30;
    (*sprite).y2 = -((*sprite).data[1] >> 8);
    if (*sprite).affineAnimEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
