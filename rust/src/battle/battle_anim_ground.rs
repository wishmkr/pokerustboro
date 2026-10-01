//! Translated from `src/battle_anim_ground.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible, gAnimMovePower,
    gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    DestroySpriteAndMatrix, GetAnimBattlerSpriteId, GetBattlerSide, GetBattlerSpriteBGPriorityRank,
    GetBattlerSpriteCoord, GetBattlerSpriteCoord2, GetBattlerYCoordWithElevation,
    InitAnimArcTranslation, InitSpritePosToAnimAttacker, InitSpritePosToAnimTarget,
    StartAnimLinearTranslation, StoreSpriteCallbackInData6, TranslateAnimHorizontalArc,
    WaitAnimForDuration,
};
use crate::battle_main::gBattlerSpriteIds;
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattle_BG3_X, gBattle_BG3_Y,
};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::random::Random2;
use crate::scanline_effect::gScanlineEffect;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
use crate::task::gTasks;
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
// The C's names for task and sprite data slots.
const tState: usize = 0;
const tDelay: usize = 1;
const tTimer: usize = 2;
const tMaxTime: usize = 3;
const tInitialX: usize = 13;
const tNumBattlers: usize = 13;
const tHorizOffset: usize = 14;
const tInitHorizOffset: usize = 15;
// Data tables (translate with cdata.py): sAffineAnim_Bonemerang sAffineAnim_SpinningBone sAffineAnims_Bonemerang sAffineAnims_SpinningBone gBonemerangSpriteTemplate gSpinningBoneSpriteTemplate gSandAttackDirtSpriteTemplate sAnim_MudSlapMud sAnims_MudSlapMud gMudSlapMudSpriteTemplate gMudsportMudSpriteTemplate gDirtPlumeSpriteTemplate gDirtMoundSpriteTemplate

pub(crate) unsafe fn AnimBonemerangProjectile(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = 20;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[5] = -40;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimBonemerangProjectile_Step);
}
pub(crate) unsafe fn AnimBonemerangProjectile_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).data[0] = 20;
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite).data[5] = 40;
        InitAnimArcTranslation(sprite);
        (*sprite).callback = Some(AnimBonemerangProjectile_End);
    }
}
pub(crate) unsafe fn AnimBonemerangProjectile_End(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimBoneHitProjectile(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimDirtScatter(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    let targetXPos: u8 = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_X_2);
    let targetYPos: u8 = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET);
    let mut xOffset: i16 = Random2() as i16 & 0x1F;
    let mut yOffset: i16 = Random2() as i16 & 0x1F;
    if xOffset > 16 {
        xOffset = 16 - xOffset;
    }
    if yOffset > 16 {
        yOffset = 16 - yOffset;
    }
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[2] = targetXPos as i16 + xOffset;
    (*sprite).data[4] = targetYPos as i16 + yOffset;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe fn AnimMudSportDirt(sprite: *mut Sprite) {
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 1);
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16
            + gBattleAnimArgs[1];
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16
            + gBattleAnimArgs[2];
        (*sprite).data[0] = (if gBattleAnimArgs[1] > 0 { 1 } else { -1 }) as i16;
        (*sprite).callback = Some(AnimMudSportDirtRising);
    } else {
        (*sprite).x = gBattleAnimArgs[1];
        (*sprite).y = gBattleAnimArgs[2];
        (*sprite).y2 = -gBattleAnimArgs[2];
        (*sprite).callback = Some(AnimMudSportDirtFalling);
    }
}
pub(crate) unsafe fn AnimMudSportDirtRising(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        (*sprite).data[1] = 0;
        (*sprite).x += (*sprite).data[0];
    }
    (*sprite).y -= 4;
    if (*sprite).y < -4 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimMudSportDirtFalling(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).y2 += 4;
            if (*sprite).y2 >= 0 {
                (*sprite).y2 = 0;
                (*sprite).data[0] += 1;
            }
        }
        1 if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) > 0 =>
        {
            (*sprite).data[1] = 0;
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) == 10
            {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DigDownMovement(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[0] == 0 {
        (*task).func = Some(AnimTask_DigBounceMovement);
    } else {
        (*task).func = Some(AnimTask_DigEndBounceMovementSetInvisible);
    }
    (*task).func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_DigBounceMovement(taskId: u8) {
    let mut y: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[10] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
            (*task).data[11] = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) as i16;
            if (*task).data[11] == 1 {
                (*task).data[12] = gBattle_BG1_X as i16;
                (*task).data[13] = gBattle_BG1_Y as i16;
            } else {
                (*task).data[12] = gBattle_BG2_X as i16;
                (*task).data[13] = gBattle_BG2_Y as i16;
            }
            y = GetBattlerYCoordWithElevation(gBattleAnimAttacker);
            (*task).data[14] = y as i16 - 32;
            (*task).data[15] = y as i16 + 32;
            if (*task).data[14] < 0 {
                (*task).data[14] = 0;
            }
            gSprites[(*task).data[10]].set_invisible(TRUE as u16);
            (*task).data[0] += 1;
        }
        1 => {
            SetDigScanlineEffect((*task).data[11] as u8, (*task).data[14], (*task).data[15]);
            (*task).data[0] += 1;
        }
        2 => {
            (*task).data[2] = ((*task).data[2] + 6) & 0x7F;
            if ({
                (*task).data[4] += 1;
                (*task).data[4]
            }) > 2
            {
                (*task).data[4] = 0;
                (*task).data[3] += 1;
            }
            (*task).data[5] = (*task).data[3]
                + ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                    [(*task).data[2]]
                    >> 4);
            if (*task).data[11] == 1 {
                gBattle_BG1_Y = (*task).data[13] as u16 - (*task).data[5] as u16;
            } else {
                gBattle_BG2_Y = (*task).data[13] as u16 - (*task).data[5] as u16;
            }
            if (*task).data[5] > 63 {
                (*task).data[5] = 120 - (*task).data[14];
                if (*task).data[11] == 1 {
                    gBattle_BG1_Y = (*task).data[13] as u16 - (*task).data[5] as u16;
                } else {
                    gBattle_BG2_Y = (*task).data[13] as u16 - (*task).data[5] as u16;
                }
                gSprites[(*task).data[10]].x2 = 272 - gSprites[(*task).data[10]].x;
                (*task).data[0] += 1;
            }
        }
        3 => {
            gScanlineEffect.state = 3;
            (*task).data[0] += 1;
        }
        4 => {
            DestroyAnimVisualTask(taskId);
            gSprites[(*task).data[10]].set_invisible(TRUE as u16);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimTask_DigEndBounceMovementSetInvisible(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    gSprites[spriteId].set_invisible(TRUE as u16);
    gSprites[spriteId].x2 = 0;
    gSprites[spriteId].y2 = 0;
    if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
        gBattle_BG1_Y = 0;
    } else {
        gBattle_BG2_Y = 0;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DigUpMovement(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[0] == 0 {
        (*task).func = Some(AnimTask_DigSetVisibleUnderground);
    } else {
        (*task).func = Some(AnimTask_DigRiseUpFromHole);
    }
    (*task).func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_DigSetVisibleUnderground(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[10] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
            gSprites[(*task).data[10]].set_invisible(FALSE as u16);
            gSprites[(*task).data[10]].x2 = 0;
            gSprites[(*task).data[10]].y2 = DISPLAY_HEIGHT as i16 - gSprites[(*task).data[10]].y;
            (*task).data[0] += 1;
        }
        1 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimTask_DigRiseUpFromHole(taskId: u8) {
    let mut y: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[10] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
            (*task).data[11] = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) as i16;
            if (*task).data[11] == 1 {
                (*task).data[12] = gBattle_BG1_X as i16;
            } else {
                (*task).data[12] = gBattle_BG2_X as i16;
            }
            y = GetBattlerYCoordWithElevation(gBattleAnimAttacker);
            (*task).data[14] = y as i16 - 32;
            (*task).data[15] = y as i16 + 32;
            (*task).data[0] += 1;
        }
        1 => {
            SetDigScanlineEffect((*task).data[11] as u8, 0, (*task).data[15]);
            (*task).data[0] += 1;
        }
        2 => {
            gSprites[(*task).data[10]].y2 = 96;
            (*task).data[0] += 1;
        }
        3 => {
            gSprites[(*task).data[10]].y2 -= 8;
            if gSprites[(*task).data[10]].y2 == 0 {
                gScanlineEffect.state = 3;
                (*task).data[0] += 1;
            }
        }
        4 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn SetDigScanlineEffect(useBG1: u8, mut y: i16, endY: i16) {
    let mut bgX: i16 = 0;
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    if useBG1 == 1 {
        bgX = gBattle_BG1_X as i16;
        scanlineParams.dmaDest = 67108884_usize as *mut u16 as *mut c_void;
    } else {
        bgX = gBattle_BG2_X as i16;
        scanlineParams.dmaDest = 67108888_usize as *mut u16 as *mut c_void;
    }
    if y < 0 {
        y = 0;
    }
    while y < endY {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][y] = bgX as u16;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][y] = bgX as u16;
        y += 1;
    }
    while y < DISPLAY_HEIGHT as i16 {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][y] = bgX as u16 + DISPLAY_WIDTH;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][y] = bgX as u16 + DISPLAY_WIDTH;
        y += 1;
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    ScanlineEffect_SetParams(scanlineParams);
}
pub unsafe fn AnimDirtPlumeParticle(sprite: *mut Sprite) {
    let mut battler: u16 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker as u16;
    } else {
        battler = gBattleAnimTarget as u16;
    }
    let mut xOffset: i16 = 24;
    if gBattleAnimArgs[1] == 1 {
        xOffset *= -1;
        gBattleAnimArgs[2] *= -1;
    }
    (*sprite).x = GetBattlerSpriteCoord(battler as u8, BATTLER_COORD_X_2) as i16 + xOffset;
    (*sprite).y = GetBattlerYCoordWithElevation(battler as u8) as i16 + 30;
    (*sprite).data[0] = gBattleAnimArgs[5];
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[3];
    (*sprite).data[5] = gBattleAnimArgs[4];
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimDirtPlumeParticle_Step);
}
pub(crate) unsafe fn AnimDirtPlumeParticle_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimDigDirtMound(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    (*sprite).x =
        GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16 - 16 + gBattleAnimArgs[1] * 32;
    (*sprite).y = GetBattlerYCoordWithElevation(battler) as i16 + 32;
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[1] as u16 * 8);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).callback = Some(WaitAnimForDuration);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_HorizontalShake(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[1] != 0 {
        (*task).data[tHorizOffset] = {
            (*task).data[tInitHorizOffset] = gBattleAnimArgs[1] + 3;
            (*task).data[tInitHorizOffset]
        };
    } else {
        (*task).data[tHorizOffset] = {
            (*task).data[tInitHorizOffset] = (gAnimMovePower as i32 / 10) as i16 + 3;
            (*task).data[tInitHorizOffset]
        };
    }
    (*task).data[tMaxTime] = gBattleAnimArgs[2];
    match gBattleAnimArgs[0] {
        5 => {
            (*task).data[13] = gBattle_BG3_X as i16;
            (*task).func = Some(AnimTask_ShakePlatforms);
        }
        4 => {
            (*task).data[13] = 0;
            for i in 0..(MAX_BATTLERS_COUNT as u16) {
                if IsBattlerSpriteVisible(i as u8) != 0 {
                    (*task).data[9 + (*task).data[13] as i32] = gBattlerSpriteIds[i] as i16;
                    (*task).data[13] += 1;
                }
            }
            (*task).func = Some(AnimTask_ShakeBattlers);
        }
        _ => {
            (*task).data[9] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
            if (*task).data[9] == SPRITE_NONE as i16 {
                DestroyAnimVisualTask(taskId);
            } else {
                (*task).data[13] = 1;
                (*task).func = Some(AnimTask_ShakeBattlers);
            }
        }
    }
}
pub(crate) unsafe fn AnimTask_ShakePlatforms(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            if ({
                (*task).data[tDelay] += 1;
                (*task).data[tDelay]
            }) > 1
            {
                (*task).data[tDelay] = 0;
                if (*task).data[tTimer] as i32 & 1 == 0 {
                    gBattle_BG3_X =
                        (*task).data[tInitialX] as u16 + (*task).data[tInitHorizOffset] as u16;
                } else {
                    gBattle_BG3_X =
                        (*task).data[tInitialX] as u16 - (*task).data[tInitHorizOffset] as u16;
                }
                if ({
                    (*task).data[tTimer] += 1;
                    (*task).data[tTimer]
                }) == (*task).data[tMaxTime]
                {
                    (*task).data[tTimer] = 0;
                    (*task).data[tHorizOffset] -= 1;
                    (*task).data[tState] += 1;
                }
            }
        }
        1 => {
            if ({
                (*task).data[tDelay] += 1;
                (*task).data[tDelay]
            }) > 1
            {
                (*task).data[tDelay] = 0;
                if (*task).data[tTimer] as i32 & 1 == 0 {
                    gBattle_BG3_X =
                        (*task).data[tInitialX] as u16 + (*task).data[tHorizOffset] as u16;
                } else {
                    gBattle_BG3_X =
                        (*task).data[tInitialX] as u16 - (*task).data[tHorizOffset] as u16;
                }
                if ({
                    (*task).data[tTimer] += 1;
                    (*task).data[tTimer]
                }) == 4
                {
                    (*task).data[tTimer] = 0;
                    if ({
                        (*task).data[tHorizOffset] -= 1;
                        (*task).data[tHorizOffset]
                    }) == 0
                    {
                        (*task).data[tState] += 1;
                    }
                }
            }
        }
        2 => {
            gBattle_BG3_X = (*task).data[tInitialX] as u16;
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimTask_ShakeBattlers(taskId: u8) {
    let mut i: u16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            if ({
                (*task).data[tDelay] += 1;
                (*task).data[tDelay]
            }) > 1
            {
                (*task).data[tDelay] = 0;
                SetBattlersXOffsetForShake(task);
                if ({
                    (*task).data[tTimer] += 1;
                    (*task).data[tTimer]
                }) == (*task).data[tMaxTime]
                {
                    (*task).data[tTimer] = 0;
                    (*task).data[tHorizOffset] -= 1;
                    (*task).data[tState] += 1;
                }
            }
        }
        1 => {
            if ({
                (*task).data[tDelay] += 1;
                (*task).data[tDelay]
            }) > 1
            {
                (*task).data[tDelay] = 0;
                SetBattlersXOffsetForShake(task);
                if ({
                    (*task).data[tTimer] += 1;
                    (*task).data[tTimer]
                }) == 4
                {
                    (*task).data[tTimer] = 0;
                    if ({
                        (*task).data[tHorizOffset] -= 1;
                        (*task).data[tHorizOffset]
                    }) == 0
                    {
                        (*task).data[tState] += 1;
                    }
                }
            }
        }
        2 => {
            i = 0;
            while (i as i32) < (*task).data[tNumBattlers] as i32 {
                gSprites[(*task).data[9 + i as i32]].x2 = 0;
                i += 1;
            }
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn SetBattlersXOffsetForShake(task: *mut Task) {
    let mut xOffset: i16 = 0;
    if (*task).data[tTimer] as i32 & 1 == 0 {
        xOffset = (*task).data[tHorizOffset] / 2 + ((*task).data[tHorizOffset] & 1);
    } else {
        xOffset = -((*task).data[tHorizOffset] / 2);
    }
    let mut i: u16 = 0;
    while (i as i32) < (*task).data[tNumBattlers] as i32 {
        gSprites[(*task).data[9 + i as i32]].x2 = xOffset;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsPowerOver99(taskId: u8) {
    gBattleAnimArgs[7] = (gAnimMovePower > 99) as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_PositionFissureBgOnBattler(taskId: u8) {
    let mut battler: u8 = if gBattleAnimArgs[0] as i32 & ANIM_TARGET as i32 != 0 {
        gBattleAnimTarget
    } else {
        gBattleAnimAttacker
    };
    if gBattleAnimArgs[0] > ANIM_TARGET as i16 {
        battler ^= 2;
    }
    let newTask: *mut Task = &raw mut (*gTasks.as_ptr())
        [CreateTask(Some(WaitForFissureCompletion), gBattleAnimArgs[1] as u8)];
    (*newTask).data[1] = (32 - GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16) & 0x1FF;
    (*newTask).data[2] =
        (64 - GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16) & 0xFF;
    gBattle_BG3_X = (*newTask).data[1] as u16;
    gBattle_BG3_Y = (*newTask).data[2] as u16;
    (*newTask).data[3] = gBattleAnimArgs[2];
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn WaitForFissureCompletion(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[7] == (*task).data[3] {
        gBattle_BG3_X = 0;
        gBattle_BG3_Y = 0;
        DestroyTask(taskId);
    } else {
        gBattle_BG3_X = (*task).data[1] as u16;
        gBattle_BG3_Y = (*task).data[2] as u16;
    }
}
