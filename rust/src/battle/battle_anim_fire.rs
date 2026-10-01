//! Translated from `src/battle_anim_fire.c` by tools/rustport/c2rs.py.
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
    unused_assignments
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible, IsContest,
    gAnimCustomPanning, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimTranslateLinear, AnimTravelDiagonally, DestroySpriteAndMatrix, GetAnimBattlerSpriteId,
    GetBattleAnimBg1Data, GetBattlerAtPosition, GetBattlerSide, GetBattlerSpriteBGPriority,
    GetBattlerSpriteCoord, InitAnimLinearTranslation, InitAnimLinearTranslationWithSpeed,
    InitSpritePosToAnimAttacker, PrepareBattlerSpriteForRotScale, PrepareEruptAnimTaskData,
    ResetSpriteRotScale, SetAnimSpriteInitialXOffset, SetBattlerSpriteYOffsetFromYScale,
    SetSpriteCoordsToAnimAttackerCoords, StartAnimLinearTranslation, StoreSpriteCallbackInData6,
    TranslateSpriteInGrowingCircle, TranslateSpriteLinear, TranslateSpriteLinearFixedPoint,
    UpdateEruptAnimTask, WaitAnimForDuration,
};
use crate::battle_main::gBattlerSpriteIds;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::sound::PlaySE12WithPanning;
use crate::sprite::gSprites;
use crate::task::{gTasks, task_get, task_set};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
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
/// `StartSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn StartSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::StartSpriteAnim(a0 as _, a1);
    }
}
// The C's names for task and sprite data slots.
const sSpeedDelay: usize = 0;
const sState: usize = 0;
const tShakeNum: usize = 0;
const tState: usize = 0;
const sBounceTimer: usize = 1;
const sLaunchStage: usize = 1;
const tMaxShakes: usize = 1;
const tTimer1: usize = 1;
const sBounceDir: usize = 2;
const sX: usize = 2;
const tShakeOffset: usize = 2;
const tTimer2: usize = 2;
const sEndTimer: usize = 3;
const sY: usize = 3;
const tTimer3: usize = 3;
const tVertical: usize = 3;
const sSpeedX: usize = 4;
const tAttackerY: usize = 4;
const tPatternId: usize = 4;
const sSpeedY: usize = 5;
const tAttackerSide: usize = 5;
const sFallDelay: usize = 6;
const sTaskId: usize = 6;
const sActiveSpritesIdx: usize = 7;
const sTargetY: usize = 7;
const tAttackerSpriteId: usize = 15;
// Data tables (translate with cdata.py): sAnim_FireSpiralSpread_0 sAnim_FireSpiralSpread_1 sAnims_FireSpiralSpread gFireSpiralInwardSpriteTemplate gFireSpreadSpriteTemplate sAnim_LargeFlame sAnims_LargeFlame sAnim_FirePlume sAnims_FirePlume sAffineAnim_LargeFlame sAffineAnims_LargeFlame gLargeFlameSpriteTemplate gLargeFlameScatterSpriteTemplate gFirePlumeSpriteTemplate sUnusedEmberFirePlumeSpriteTemplate sAnim_UnusedSmallEmber sAnims_UnusedSmallEmber sUnusedSmallEmberSpriteTemplate sAffineAnim_SunlightRay sAffineAnims_SunlightRay gSunlightRaySpriteTemplate sAnim_BasicFire gAnims_BasicFire gEmberSpriteTemplate gEmberFlareSpriteTemplate gBurnFlameSpriteTemplate gFireBlastRingSpriteTemplate sAnim_FireBlastCross sAnims_FireBlastCross sAffineAnim_Unused_0 sAffineAnim_Unused_1 sAffineAnims_Unused gFireBlastCrossSpriteTemplate gFireSpiralOutwardSpriteTemplate gWeatherBallFireDownSpriteTemplate gEruptionLaunchRockSpriteTemplate sEruptionLaunchRockSpeeds gEruptionFallingRockSpriteTemplate sAnim_WillOWispOrb_0 sAnim_WillOWispOrb_1 sAnim_WillOWispOrb_2 sAnim_WillOWispOrb_3 sAnims_WillOWispOrb gWillOWispOrbSpriteTemplate sAnim_WillOWispFire sAnims_WillOWispFire gWillOWispFireSpriteTemplate sShakeDirsPattern0 sShakeDirsPattern1

const IDX_ACTIVE_SPRITES: u8 = 6;

static gEruptionLaunchRockSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_fire::gEruptionLaunchRockSpriteTemplate).cast());
static sEruptionLaunchRockSpeeds: Table<CArray<CArray<i16, 2>, 7>> =
    Table((&raw const crate::data::battle_anim_fire::sEruptionLaunchRockSpeeds).cast());
static sShakeDirsPattern0: Table<CArray<i8, 16>> =
    Table((&raw const crate::data::battle_anim_fire::sShakeDirsPattern0).cast());
static sShakeDirsPattern1: Table<CArray<i8, 16>> =
    Table((&raw const crate::data::battle_anim_fire::sShakeDirsPattern1).cast());

pub(crate) unsafe fn AnimFireSpiralInward(sprite: *mut Sprite) {
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = 0x3C;
    (*sprite).data[2] = 0x9;
    (*sprite).data[3] = 0x1E;
    (*sprite).data[4] = -512;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteInGrowingCircle);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimFireSpread(sprite: *mut Sprite) {
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[2] = gBattleAnimArgs[3];
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimFirePlume(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x -= gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        (*sprite).data[2] = -gBattleAnimArgs[4];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        (*sprite).data[2] = gBattleAnimArgs[4];
    }
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).callback = Some(AnimLargeFlame_Step);
}
pub(crate) unsafe fn AnimLargeFlame(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x -= gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        (*sprite).data[2] = gBattleAnimArgs[4];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        (*sprite).data[2] = -gBattleAnimArgs[4];
    }
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).callback = Some(AnimLargeFlame_Step);
}
pub(crate) unsafe fn AnimLargeFlame_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) < (*sprite).data[4]
    {
        (*sprite).x2 += (*sprite).data[2];
        (*sprite).y2 += (*sprite).data[3];
    }
    if (*sprite).data[0] == (*sprite).data[1] {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe fn AnimUnusedSmallEmber(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).subpriority = 8;
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = gBattleAnimArgs[4];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).data[4] = gBattleAnimArgs[6];
    (*sprite).data[5] = 0;
    (*sprite).callback = Some(AnimUnusedSmallEmber_Step);
}
pub(crate) unsafe fn AnimUnusedSmallEmber_Step(sprite: *mut Sprite) {
    if (*sprite).data[3] != 0 {
        if (*sprite).data[5] > 10000 {
            (*sprite).subpriority = 1;
        }
        (*sprite).x2 = Sin(
            (*sprite).data[0],
            (*sprite).data[1] + ((*sprite).data[5] >> 8),
        );
        (*sprite).y2 = Cos(
            (*sprite).data[0],
            (*sprite).data[1] + ((*sprite).data[5] >> 8),
        );
        (*sprite).data[0] += (*sprite).data[2];
        (*sprite).data[5] += (*sprite).data[4];
        if (*sprite).data[0] > 255 {
            (*sprite).data[0] -= 256;
        } else if (*sprite).data[0] < 0 {
            (*sprite).data[0] += 256;
        }
        (*sprite).data[3] -= 1;
    } else {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe fn AnimSunlight(sprite: *mut Sprite) {
    (*sprite).x = 0;
    (*sprite).y = 0;
    (*sprite).data[0] = 60;
    (*sprite).data[2] = 140;
    (*sprite).data[4] = 80;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimEmberFlare(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
        && (gBattleAnimAttacker == GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT)
            || gBattleAnimAttacker == GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT))
    {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).callback = Some(AnimTravelDiagonally);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimBurnFlame(sprite: *mut Sprite) {
    gBattleAnimArgs[0] = -gBattleAnimArgs[0];
    gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    (*sprite).callback = Some(AnimTravelDiagonally);
}
pub unsafe fn AnimFireRing(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[7] = gBattleAnimArgs[2];
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(AnimFireRing_Step1);
}
pub(crate) unsafe fn AnimFireRing_Step1(sprite: *mut Sprite) {
    UpdateFireRingCircleOffset(sprite);
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 0x12
    {
        (*sprite).data[0] = 0x19;
        (*sprite).data[1] = (*sprite).x;
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).data[3] = (*sprite).y;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        InitAnimLinearTranslation(sprite);
        (*sprite).callback = Some(AnimFireRing_Step2);
    }
}
pub(crate) unsafe fn AnimFireRing_Step2(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        (*sprite).data[0] = 0;
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).callback = Some(AnimFireRing_Step3);
        (*sprite).callback.unwrap_unchecked()(sprite);
    } else {
        (*sprite).x2 += Sin((*sprite).data[7], 28);
        (*sprite).y2 += Cos((*sprite).data[7], 28);
        (*sprite).data[7] = ((*sprite).data[7] + 20) & 0xFF;
    }
}
pub(crate) unsafe fn AnimFireRing_Step3(sprite: *mut Sprite) {
    UpdateFireRingCircleOffset(sprite);
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 0x1F
    {
        DestroyAnimSprite(sprite);
    }
}
unsafe fn UpdateFireRingCircleOffset(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[7], 28);
    (*sprite).y2 = Cos((*sprite).data[7], 28);
    (*sprite).data[7] = ((*sprite).data[7] + 20) & 0xFF;
}
pub(crate) unsafe fn AnimFireCross(sprite: *mut Sprite) {
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = gBattleAnimArgs[4];
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteLinear);
}
pub(crate) unsafe fn AnimFireSpiralOutward(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(AnimFireSpiralOutward_Step1));
}
pub(crate) unsafe fn AnimFireSpiralOutward_Step1(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    (*sprite).data[0] = (*sprite).data[1];
    (*sprite).data[1] = 0;
    (*sprite).callback = Some(AnimFireSpiralOutward_Step2);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimFireSpiralOutward_Step2(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[2] >> 8);
    (*sprite).y2 = Cos((*sprite).data[1], (*sprite).data[2] >> 8);
    (*sprite).data[1] = ((*sprite).data[1] + 10) & 0xFF;
    (*sprite).data[2] += 0xD0;
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == -1
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_EruptionLaunchRocks(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[tAttackerSpriteId] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).data[tState] = 0;
    (*task).data[tTimer1] = 0;
    (*task).data[tTimer2] = 0;
    (*task).data[tTimer3] = 0;
    (*task).data[tAttackerY] = gSprites[(*task).data[tAttackerSpriteId]].y;
    (*task).data[tAttackerSide] = GetBattlerSide(gBattleAnimAttacker) as i16;
    (*task).data[6] = 0;
    PrepareBattlerSpriteForRotScale((*task).data[tAttackerSpriteId] as u8, ST_OAM_OBJ_NORMAL);
    (*task).func = Some(AnimTask_EruptionLaunchRocks_Step);
}
pub(crate) unsafe fn AnimTask_EruptionLaunchRocks_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    'l1: {
        let sw1: i16 = (*task).data[tState];
        let matched =
            sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4 || sw1 == 5 || sw1 == 6;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PrepareEruptAnimTaskData(
                task,
                (*task).data[tAttackerSpriteId] as u8,
                0x100,
                0x100,
                0xE0,
                0x200,
                32,
            );
            (*task).data[tState] += 1;
        }
        if fall || sw1 == 1 {
            if ({
                (*task).data[tTimer1] += 1;
                (*task).data[tTimer1]
            }) > 1
            {
                (*task).data[tTimer1] = 0;
                if ({
                    (*task).data[tTimer2] += 1;
                    (*task).data[tTimer2]
                }) as i32
                    & 1
                    != 0
                {
                    gSprites[(*task).data[tAttackerSpriteId]].x2 = 3;
                } else {
                    gSprites[(*task).data[tAttackerSpriteId]].x2 = -3;
                }
            }
            if (*task).data[tAttackerSide] != B_SIDE_PLAYER as i16
                && ({
                    (*task).data[tTimer3] += 1;
                    (*task).data[tTimer3]
                }) > 4
            {
                (*task).data[tTimer3] = 0;
                gSprites[(*task).data[tAttackerSpriteId]].y += 1;
            }
            if UpdateEruptAnimTask(task) == 0 {
                SetBattlerSpriteYOffsetFromYScale((*task).data[tAttackerSpriteId] as u8);
                gSprites[(*task).data[tAttackerSpriteId]].x2 = 0;
                (*task).data[tTimer1] = 0;
                (*task).data[tTimer2] = 0;
                (*task).data[tTimer3] = 0;
                (*task).data[tState] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if ({
                (*task).data[tTimer1] += 1;
                (*task).data[tTimer1]
            }) > 4
            {
                if (*task).data[tAttackerSide] != B_SIDE_PLAYER as i16 {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[tAttackerSpriteId] as u8,
                        0xE0,
                        0x200,
                        0x180,
                        0xF0,
                        6,
                    );
                } else {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[tAttackerSpriteId] as u8,
                        0xE0,
                        0x200,
                        0x180,
                        0xC0,
                        6,
                    );
                }
                (*task).data[tTimer1] = 0;
                (*task).data[tState] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            if UpdateEruptAnimTask(task) == 0 {
                CreateEruptionLaunchRocks(
                    (*task).data[tAttackerSpriteId] as u8,
                    taskId,
                    IDX_ACTIVE_SPRITES,
                );
                (*task).data[tState] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            if ({
                (*task).data[tTimer1] += 1;
                (*task).data[tTimer1]
            }) > 1
            {
                (*task).data[tTimer1] = 0;
                if ({
                    (*task).data[tTimer2] += 1;
                    (*task).data[tTimer2]
                }) as i32
                    & 1
                    != 0
                {
                    gSprites[(*task).data[tAttackerSpriteId]].y2 += 3;
                } else {
                    gSprites[(*task).data[tAttackerSpriteId]].y2 -= 3;
                }
            }
            if ({
                (*task).data[tTimer3] += 1;
                (*task).data[tTimer3]
            }) > 24
            {
                if (*task).data[tAttackerSide] != B_SIDE_PLAYER as i16 {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[tAttackerSpriteId] as u8,
                        0x180,
                        0xF0,
                        0x100,
                        0x100,
                        8,
                    );
                } else {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[tAttackerSpriteId] as u8,
                        0x180,
                        0xC0,
                        0x100,
                        0x100,
                        8,
                    );
                }
                if (*task).data[tTimer2] as i32 & 1 != 0 {
                    gSprites[(*task).data[tAttackerSpriteId]].y2 -= 3;
                }
                (*task).data[tTimer1] = 0;
                (*task).data[tTimer2] = 0;
                (*task).data[tTimer3] = 0;
                (*task).data[tState] += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            if (*task).data[tAttackerSide] != B_SIDE_PLAYER as i16 {
                gSprites[(*task).data[tAttackerSpriteId]].y -= 1;
            }
            if UpdateEruptAnimTask(task) == 0 {
                gSprites[(*task).data[tAttackerSpriteId]].y = (*task).data[tAttackerY];
                ResetSpriteRotScale((*task).data[tAttackerSpriteId] as u8);
                (*task).data[tTimer2] = 0;
                (*task).data[tState] += 1;
            }
            break 'l1;
        }
        if sw1 == 6 {
            if (*task).data[6] == 0 {
                DestroyAnimVisualTask(taskId);
            }
            break 'l1;
        }
        if !matched {
            break 'l1;
        }
    }
}
unsafe fn CreateEruptionLaunchRocks(spriteId: u8, taskId: u8, activeSpritesIdx: u8) {
    let mut sign: i8 = 0;
    let y: u16 = GetEruptionLaunchRockInitialYPos(spriteId);
    let mut x: u16 = gSprites[spriteId].x as u16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        x -= 12;
        sign = 1;
    } else {
        x += 16;
        sign = -1;
    }
    let mut j: u16 = 0;
    for i in 0..=6u16 {
        let spriteId: u8 = CreateSprite(
            (&raw const *gEruptionLaunchRockSpriteTemplate).cast_mut(),
            x as i16,
            y as i16,
            2,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId]
                .oam
                .set_tileNum(gSprites[spriteId].oam.tileNum() + (j * 4 + 0x40));
            if ({
                j += 1;
                j
            }) >= 5
            {
                j = 0;
            }
            InitEruptionLaunchRockCoordData(
                &raw mut gSprites[spriteId],
                sEruptionLaunchRockSpeeds[i][0] * sign as i16,
                sEruptionLaunchRockSpeeds[i][1],
            );
            gSprites[spriteId].data[sTaskId] = taskId as i16;
            gSprites[spriteId].data[sActiveSpritesIdx] = activeSpritesIdx as i16;
            task_set(
                taskId,
                activeSpritesIdx,
                task_get(taskId, activeSpritesIdx) + 1,
            );
        }
    }
}
pub(crate) unsafe fn AnimEruptionLaunchRock(sprite: *mut Sprite) {
    UpdateEruptionLaunchRockPos(sprite);
    if (*sprite).invisible() != 0 {
        task_set(
            (*sprite).data[sTaskId],
            (*sprite).data[sActiveSpritesIdx],
            task_get((*sprite).data[sTaskId], (*sprite).data[sActiveSpritesIdx]) - 1,
        );
        DestroySprite(sprite);
    }
}
unsafe fn GetEruptionLaunchRockInitialYPos(spriteId: u8) -> u16 {
    let mut y: i16 =
        gSprites[spriteId].y + gSprites[spriteId].y2 + gSprites[spriteId].centerToCornerVecY as i16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        y += 74;
    } else {
        y += 44;
    }
    y as u16
}
unsafe fn InitEruptionLaunchRockCoordData(sprite: *mut Sprite, speedX: i16, speedY: i16) {
    (*sprite).data[sSpeedDelay] = 0;
    (*sprite).data[sLaunchStage] = 0;
    (*sprite).data[sX] = (*sprite).x as u16 as i16 * 8;
    (*sprite).data[sY] = (*sprite).y as u16 as i16 * 8;
    (*sprite).data[sSpeedX] = speedX * 8;
    (*sprite).data[sSpeedY] = speedY * 8;
}
unsafe fn UpdateEruptionLaunchRockPos(sprite: *mut Sprite) {
    let mut extraLaunchSpeed: i32 = 0;
    if ({
        (*sprite).data[sSpeedDelay] += 1;
        (*sprite).data[sSpeedDelay]
    }) > 2
    {
        (*sprite).data[sSpeedDelay] = 0;
        (*sprite).data[sLaunchStage] += 1;
        extraLaunchSpeed =
            (*sprite).data[sLaunchStage] as u16 as i32 * (*sprite).data[sLaunchStage] as u16 as i32;
        (*sprite).data[sY] += extraLaunchSpeed as i16;
    }
    (*sprite).data[sX] += (*sprite).data[sSpeedX];
    (*sprite).x = (*sprite).data[sX] >> 3;
    (*sprite).data[sY] += (*sprite).data[sSpeedY];
    (*sprite).y = (*sprite).data[sY] >> 3;
    if (*sprite).x < -8 || (*sprite).x > 248 || (*sprite).y < -8 || (*sprite).y > 120 {
        (*sprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe fn AnimEruptionFallingRock(sprite: *mut Sprite) {
    (*sprite).x = gBattleAnimArgs[0];
    (*sprite).y = gBattleAnimArgs[1];
    (*sprite).data[sState] = 0;
    (*sprite).data[sBounceTimer] = 0;
    (*sprite).data[sBounceDir] = 0;
    (*sprite).data[sFallDelay] = gBattleAnimArgs[2];
    (*sprite).data[sTargetY] = gBattleAnimArgs[3];
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[4] as u16 * 16);
    (*sprite).callback = Some(AnimEruptionFallingRock_Step);
}
pub(crate) unsafe fn AnimEruptionFallingRock_Step(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[sState];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if (*sprite).data[sFallDelay] != 0 {
                (*sprite).data[sFallDelay] -= 1;
                return;
            }
            (*sprite).data[sState] += 1;
        }
        if fall || sw1 == 1 {
            (*sprite).y += 8;
            if (*sprite).y >= (*sprite).data[sTargetY] {
                (*sprite).y = (*sprite).data[sTargetY];
                (*sprite).data[sState] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            if ({
                (*sprite).data[sBounceTimer] += 1;
                (*sprite).data[sBounceTimer]
            }) > 1
            {
                (*sprite).data[sBounceTimer] = 0;
                if ({
                    (*sprite).data[sBounceDir] += 1;
                    (*sprite).data[sBounceDir]
                }) as i32
                    & 1
                    != 0
                {
                    (*sprite).y2 = -3;
                } else {
                    (*sprite).y2 = 3;
                }
            }
            if ({
                (*sprite).data[sEndTimer] += 1;
                (*sprite).data[sEndTimer]
            }) > 16
            {
                DestroyAnimSprite(sprite);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn AnimWillOWispOrb(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            InitSpritePosToAnimAttacker(sprite, FALSE);
            StartSpriteAnim(sprite, gBattleAnimArgs[2] as u8);
            (*sprite).data[7] = gBattleAnimArgs[2];
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                (*sprite).data[4] = 4;
            } else {
                (*sprite).data[4] = -4;
            }
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
            (*sprite).data[0] += 1;
        }
        1 => {
            (*sprite).data[1] += 192;
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                (*sprite).y2 = -((*sprite).data[1] >> 8);
            } else {
                (*sprite).y2 = (*sprite).data[1] >> 8;
            }
            (*sprite).x2 = Sin((*sprite).data[2], (*sprite).data[4]);
            (*sprite).data[2] = ((*sprite).data[2] + 4) & 0xFF;
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) == 1
            {
                (*sprite).data[3] = 0;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).x2 = Sin((*sprite).data[2], (*sprite).data[4]);
            (*sprite).data[2] = ((*sprite).data[2] + 4) & 0xFF;
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) == 31
            {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).y2 = 0;
                (*sprite).x2 = 0;
                (*sprite).data[0] = 256;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] =
                    GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] =
                    GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
                InitAnimLinearTranslationWithSpeed(sprite);
                (*sprite).callback = Some(AnimWillOWispOrb_Step);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimWillOWispOrb_Step(sprite: *mut Sprite) {
    let mut initialData5: i16 = 0;
    let mut newData5: i16 = 0;
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).x2 += Sin((*sprite).data[5], 16);
        initialData5 = (*sprite).data[5];
        (*sprite).data[5] = ((*sprite).data[5] + 4) & 0xFF;
        newData5 = (*sprite).data[5];
        if (initialData5 == 0 || initialData5 > 196) && newData5 > 0 && (*sprite).data[7] == 0 {
            PlaySE12WithPanning(SE_M_FLAME_WHEEL, gAnimCustomPanning as i8);
        }
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimWillOWispFire(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[1] = gBattleAnimArgs[0];
        (*sprite).data[0] += 1;
    }
    (*sprite).data[3] += 384;
    (*sprite).data[4] += 0xA0;
    (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[3] >> 8);
    (*sprite).y2 = Cos((*sprite).data[1], (*sprite).data[4] >> 8);
    (*sprite).data[1] = ((*sprite).data[1] + 7) & 0xFF;
    if IsContest() == 0 {
        if (*sprite).data[1] < 64 || (*sprite).data[1] > 195 {
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
        } else {
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16 + 1);
        }
    } else {
        if (*sprite).data[1] < 64 || (*sprite).data[1] > 195 {
            (*sprite).subpriority = 0x1D;
        } else {
            (*sprite).subpriority = 0x1F;
        }
    }
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) > 0x14
    {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
    if (*sprite).data[2] == 0x1E {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MoveHeatWaveTargets(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[12] = (if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        1
    } else {
        -1
    }) as i16;
    (*task).data[13] = IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) as i16 + 1;
    (*task).data[14] = GetAnimBattlerSpriteId(ANIM_TARGET) as i16;
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_DEF_PARTNER) as i16;
    (*task).func = Some(AnimTask_MoveHeatWaveTargets_Step);
}
pub(crate) unsafe fn AnimTask_MoveHeatWaveTargets_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[10] += (*task).data[12] * 2;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) >= 2
            {
                (*task).data[1] = 0;
                (*task).data[2] += 1;
                if (*task).data[2] as i32 & 1 != 0 {
                    (*task).data[11] = 2;
                } else {
                    (*task).data[11] = -2;
                }
            }
            (*task).data[3] = 0;
            while (*task).data[3] < (*task).data[13] {
                gSprites[(*task).data[(*task).data[3] as i32 + 14]].x2 =
                    (*task).data[10] + (*task).data[11];
                (*task).data[3] += 1;
            }
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) == 16
            {
                (*task).data[9] = 0;
                (*task).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) >= 5
            {
                (*task).data[1] = 0;
                (*task).data[2] += 1;
                if (*task).data[2] as i32 & 1 != 0 {
                    (*task).data[11] = 2;
                } else {
                    (*task).data[11] = -2;
                }
            }
            (*task).data[3] = 0;
            while (*task).data[3] < (*task).data[13] {
                gSprites[(*task).data[(*task).data[3] as i32 + 14]].x2 =
                    (*task).data[10] + (*task).data[11];
                (*task).data[3] += 1;
            }
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) == 96
            {
                (*task).data[9] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            (*task).data[10] -= (*task).data[12] * 2;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) >= 2
            {
                (*task).data[1] = 0;
                (*task).data[2] += 1;
                if (*task).data[2] as i32 & 1 != 0 {
                    (*task).data[11] = 2;
                } else {
                    (*task).data[11] = -2;
                }
            }
            (*task).data[3] = 0;
            while (*task).data[3] < (*task).data[13] {
                gSprites[(*task).data[(*task).data[3] as i32 + 14]].x2 =
                    (*task).data[10] + (*task).data[11];
                (*task).data[3] += 1;
            }
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) == 16
            {
                (*task).data[0] += 1;
            }
        }
        3 => {
            (*task).data[3] = 0;
            while (*task).data[3] < (*task).data[13] {
                gSprites[(*task).data[(*task).data[3] as i32 + 14]].x2 = 0;
                (*task).data[3] += 1;
            }
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BlendBackground(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    GetBattleAnimBg1Data(&raw mut animBg);
    BlendPalette(
        animBg.paletteId as u16 * 16,
        16,
        gBattleAnimArgs[0] as u8,
        gBattleAnimArgs[1] as u16,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeTargetInPattern(taskId: u8) {
    let mut dir: i8 = 0;
    if task_get(taskId, tShakeNum) == 0 {
        task_set(taskId, tMaxShakes, gBattleAnimArgs[0]);
        task_set(taskId, tShakeOffset, gBattleAnimArgs[1]);
        task_set(taskId, tVertical, gBattleAnimArgs[2]);
        task_set(taskId, tPatternId, gBattleAnimArgs[3]);
    }
    task_set(taskId, tShakeNum, task_get(taskId, tShakeNum) + 1);
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimTarget];
    if task_get(taskId, tPatternId) == 0 {
        dir = sShakeDirsPattern0[task_get(taskId, tShakeNum) % 10];
    } else {
        dir = sShakeDirsPattern1[task_get(taskId, tShakeNum) % 10];
    }
    if task_get(taskId, tVertical) == TRUE as i16 {
        gSprites[spriteId].y2 = (if (gBattleAnimArgs[1] as i32 * dir as i32) < 0 {
            -(gBattleAnimArgs[1] as i32 * dir as i32)
        } else {
            gBattleAnimArgs[1] as i32 * dir as i32
        }) as i16;
    } else {
        gSprites[spriteId].x2 = gBattleAnimArgs[1] * dir as i16;
    }
    if task_get(taskId, tShakeNum) == task_get(taskId, tMaxShakes) {
        gSprites[spriteId].x2 = 0;
        gSprites[spriteId].y2 = 0;
        DestroyAnimVisualTask(taskId);
    }
}
