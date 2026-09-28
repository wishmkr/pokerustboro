//! Translated from `src/battle_anim_ground.c` by tools/rustport/c2rs.py.
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
// Data tables (translate with cdata.py): sAffineAnim_Bonemerang sAffineAnim_SpinningBone sAffineAnims_Bonemerang sAffineAnims_SpinningBone gBonemerangSpriteTemplate gSpinningBoneSpriteTemplate gSandAttackDirtSpriteTemplate sAnim_MudSlapMud sAnims_MudSlapMud gMudSlapMudSpriteTemplate gMudsportMudSpriteTemplate gDirtPlumeSpriteTemplate gDirtMoundSpriteTemplate

unsafe extern "C" {
    static mut gAnimMovePower: u16;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u16;
    static mut gBattle_BG1_Y: u16;
    static mut gBattle_BG2_X: u16;
    static mut gBattle_BG2_Y: u16;
    static mut gBattle_BG3_X: u16;
    static mut gBattle_BG3_Y: u16;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gScanlineEffect: ScanlineEffect;
    static mut gScanlineEffectRegBuffers: CArray<CArray<u16, 960>, 2>;
    static gSineTable: CArray<i16, 0>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoord2(a0: u8, a1: u8) -> u8;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut Sprite, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn Random2() -> u16;
    fn ScanlineEffect_SetParams(a0: ScanlineEffectParams);
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateAnimHorizontalArc(a0: *mut Sprite) -> u8;
    fn WaitAnimForDuration(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimBonemerangProjectile(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = 20;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[5] = -40;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimBonemerangProjectile_Step);
}
pub(crate) unsafe extern "C" fn AnimBonemerangProjectile_Step(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimBonemerangProjectile_End(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimBoneHitProjectile(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimDirtScatter(sprite: *mut Sprite) {
    let mut targetXPos: u8 = 0;
    let mut targetYPos: u8 = 0;
    let mut xOffset: i16 = 0;
    let mut yOffset: i16 = 0;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    targetXPos = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_X_2);
    targetYPos = GetBattlerSpriteCoord2(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET);
    xOffset = Random2() as i16 & 0x1F;
    yOffset = Random2() as i16 & 0x1F;
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
pub(crate) unsafe extern "C" fn AnimMudSportDirt(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimMudSportDirtRising(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimMudSportDirtFalling(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).y2 += 4;
            if (*sprite).y2 >= 0 {
                (*sprite).y2 = 0;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 0
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
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DigDownMovement(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if gBattleAnimArgs[0] == 0 {
        (*task).func = Some(AnimTask_DigBounceMovement);
    } else {
        (*task).func = Some(AnimTask_DigEndBounceMovementSetInvisible);
    }
    (*task).func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn AnimTask_DigBounceMovement(taskId: u8) {
    let mut y: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
            (*task).data[2] = (*task).data[2] + 6 & 0x7F;
            if ({
                (*task).data[4] += 1;
                (*task).data[4]
            }) > 2
            {
                (*task).data[4] = 0;
                (*task).data[3] += 1;
            }
            (*task).data[5] = (*task).data[3] + (gSineTable[(*task).data[2]] >> 4);
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
pub(crate) unsafe extern "C" fn AnimTask_DigEndBounceMovementSetInvisible(taskId: u8) {
    let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
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
pub unsafe extern "C" fn AnimTask_DigUpMovement(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if gBattleAnimArgs[0] == 0 {
        (*task).func = Some(AnimTask_DigSetVisibleUnderground);
    } else {
        (*task).func = Some(AnimTask_DigRiseUpFromHole);
    }
    (*task).func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe extern "C" fn AnimTask_DigSetVisibleUnderground(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
pub(crate) unsafe extern "C" fn AnimTask_DigRiseUpFromHole(taskId: u8) {
    let mut y: u8 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
pub(crate) unsafe extern "C" fn SetDigScanlineEffect(useBG1: u8, mut y: i16, endY: i16) {
    let mut bgX: i16 = 0;
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    if useBG1 == 1 {
        bgX = gBattle_BG1_X as i16;
        scanlineParams.dmaDest = 67108884 as usize as *mut u16 as *mut c_void;
    } else {
        bgX = gBattle_BG2_X as i16;
        scanlineParams.dmaDest = 67108888 as usize as *mut u16 as *mut c_void;
    }
    if y < 0 {
        y = 0;
    }
    while y < endY {
        gScanlineEffectRegBuffers[0][y] = bgX as u16;
        gScanlineEffectRegBuffers[1][y] = bgX as u16;
        y += 1;
    }
    while y < DISPLAY_HEIGHT as i16 {
        gScanlineEffectRegBuffers[0][y] = bgX as u16 + DISPLAY_WIDTH;
        gScanlineEffectRegBuffers[1][y] = bgX as u16 + DISPLAY_WIDTH;
        y += 1;
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    ScanlineEffect_SetParams(scanlineParams);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimDirtPlumeParticle(sprite: *mut Sprite) {
    let mut battler: u16 = 0;
    let mut xOffset: i16 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker as u16;
    } else {
        battler = gBattleAnimTarget as u16;
    }
    xOffset = 24;
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
pub(crate) unsafe extern "C" fn AnimDirtPlumeParticle_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimDigDirtMound(sprite: *mut Sprite) {
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
pub unsafe extern "C" fn AnimTask_HorizontalShake(taskId: u8) {
    let mut i: u16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if gBattleAnimArgs[1] != 0 {
        (*task).data[14] = {
            (*task).data[15] = gBattleAnimArgs[1] + 3;
            (*task).data[15]
        };
    } else {
        (*task).data[14] = {
            (*task).data[15] = (gAnimMovePower as i32 / 10) as i16 + 3;
            (*task).data[15]
        };
    }
    (*task).data[3] = gBattleAnimArgs[2];
    match gBattleAnimArgs[0] {
        5 => {
            (*task).data[13] = gBattle_BG3_X as i16;
            (*task).func = Some(AnimTask_ShakePlatforms);
        }
        4 => {
            (*task).data[13] = 0;
            i = 0;
            while i < MAX_BATTLERS_COUNT as u16 {
                if IsBattlerSpriteVisible(i as u8) != 0 {
                    (*task).data[9 + (*task).data[13] as i32] = gBattlerSpriteIds[i] as i16;
                    (*task).data[13] += 1;
                }
                i += 1;
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
pub(crate) unsafe extern "C" fn AnimTask_ShakePlatforms(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                if (*task).data[2] as i32 & 1 == 0 {
                    gBattle_BG3_X = (*task).data[13] as u16 + (*task).data[15] as u16;
                } else {
                    gBattle_BG3_X = (*task).data[13] as u16 - (*task).data[15] as u16;
                }
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == (*task).data[3]
                {
                    (*task).data[2] = 0;
                    (*task).data[14] -= 1;
                    (*task).data[0] += 1;
                }
            }
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                if (*task).data[2] as i32 & 1 == 0 {
                    gBattle_BG3_X = (*task).data[13] as u16 + (*task).data[14] as u16;
                } else {
                    gBattle_BG3_X = (*task).data[13] as u16 - (*task).data[14] as u16;
                }
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == 4
                {
                    (*task).data[2] = 0;
                    if ({
                        (*task).data[14] -= 1;
                        (*task).data[14]
                    }) == 0
                    {
                        (*task).data[0] += 1;
                    }
                }
            }
        }
        2 => {
            gBattle_BG3_X = (*task).data[13] as u16;
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ShakeBattlers(taskId: u8) {
    let mut i: u16 = 0;
    let mut task: *mut Task = &raw mut gTasks[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                SetBattlersXOffsetForShake(task);
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == (*task).data[3]
                {
                    (*task).data[2] = 0;
                    (*task).data[14] -= 1;
                    (*task).data[0] += 1;
                }
            }
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                SetBattlersXOffsetForShake(task);
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) == 4
                {
                    (*task).data[2] = 0;
                    if ({
                        (*task).data[14] -= 1;
                        (*task).data[14]
                    }) == 0
                    {
                        (*task).data[0] += 1;
                    }
                }
            }
        }
        2 => {
            i = 0;
            while (i as i32) < (*task).data[13] as i32 {
                gSprites[(*task).data[9 + i as i32]].x2 = 0;
                i += 1;
            }
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe extern "C" fn SetBattlersXOffsetForShake(task: *mut Task) {
    let mut i: u16 = 0;
    let mut xOffset: i16 = 0;
    if (*task).data[2] as i32 & 1 == 0 {
        xOffset = (*task).data[14] / 2 + ((*task).data[14] & 1);
    } else {
        xOffset = -((*task).data[14] / 2);
    }
    i = 0;
    while (i as i32) < (*task).data[13] as i32 {
        gSprites[(*task).data[9 + i as i32]].x2 = xOffset;
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsPowerOver99(taskId: u8) {
    gBattleAnimArgs[7] = (gAnimMovePower > 99) as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_PositionFissureBgOnBattler(taskId: u8) {
    let mut newTask: *mut Task = null_mut();
    let mut battler: u8 = if gBattleAnimArgs[0] as i32 & ANIM_TARGET as i32 != 0 {
        gBattleAnimTarget
    } else {
        gBattleAnimAttacker
    };
    if gBattleAnimArgs[0] > ANIM_TARGET as i16 {
        battler = battler ^ 2;
    }
    newTask = &raw mut gTasks[CreateTask(Some(WaitForFissureCompletion), gBattleAnimArgs[1] as u8)];
    (*newTask).data[1] = 32 - GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16 & 0x1FF;
    (*newTask).data[2] =
        64 - GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16 & 0xFF;
    gBattle_BG3_X = (*newTask).data[1] as u16;
    gBattle_BG3_Y = (*newTask).data[2] as u16;
    (*newTask).data[3] = gBattleAnimArgs[2];
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe extern "C" fn WaitForFissureCompletion(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    if gBattleAnimArgs[7] == (*task).data[3] {
        gBattle_BG3_X = 0;
        gBattle_BG3_Y = 0;
        DestroyTask(taskId);
    } else {
        gBattle_BG3_X = (*task).data[1] as u16;
        gBattle_BG3_Y = (*task).data[2] as u16;
    }
}
