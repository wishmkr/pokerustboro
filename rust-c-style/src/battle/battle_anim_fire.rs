//! Translated from `src/battle_anim_fire.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gAnimCustomPanning: u8;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn AnimTranslateLinear(a0: *mut Sprite) -> u8;
    fn AnimTravelDiagonally(a0: *mut Sprite);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut Sprite);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndMatrix(a0: *mut Sprite);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut BattleAnimBgData);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn InitAnimLinearTranslation(a0: *mut Sprite);
    fn InitAnimLinearTranslationWithSpeed(a0: *mut Sprite);
    fn InitSpritePosToAnimAttacker(a0: *mut Sprite, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn PrepareEruptAnimTaskData(a0: *mut Task, a1: u8, a2: i16, a3: i16, a4: i16, a5: i16, a6: u16);
    fn ResetSpriteRotScale(a0: u8);
    fn SetAnimSpriteInitialXOffset(a0: *mut Sprite, a1: i16);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut Sprite);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut Sprite);
    fn StartSpriteAnim(a0: *mut Sprite, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut Sprite, a1: Option<unsafe extern "C" fn(*mut Sprite)>);
    fn TranslateSpriteInGrowingCircle(a0: *mut Sprite);
    fn TranslateSpriteLinear(a0: *mut Sprite);
    fn TranslateSpriteLinearFixedPoint(a0: *mut Sprite);
    fn UpdateEruptAnimTask(a0: *mut Task) -> u8;
    fn WaitAnimForDuration(a0: *mut Sprite);
}

pub(crate) unsafe extern "C" fn AnimFireSpiralInward(sprite: *mut Sprite) {
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = 0x3C;
    (*sprite).data[2] = 0x9;
    (*sprite).data[3] = 0x1E;
    (*sprite).data[4] = -512;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteInGrowingCircle);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimFireSpread(sprite: *mut Sprite) {
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[2] = gBattleAnimArgs[3];
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimFirePlume(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimLargeFlame(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimLargeFlame_Step(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimUnusedSmallEmber(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimUnusedSmallEmber_Step(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimSunlight(sprite: *mut Sprite) {
    (*sprite).x = 0;
    (*sprite).y = 0;
    (*sprite).data[0] = 60;
    (*sprite).data[2] = 140;
    (*sprite).data[4] = 80;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe extern "C" fn AnimEmberFlare(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget)
        && (gBattleAnimAttacker == GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT)
            || gBattleAnimAttacker == GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT))
    {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).callback = Some(AnimTravelDiagonally);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimBurnFlame(sprite: *mut Sprite) {
    gBattleAnimArgs[0] = -gBattleAnimArgs[0];
    gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    (*sprite).callback = Some(AnimTravelDiagonally);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimFireRing(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[7] = gBattleAnimArgs[2];
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(AnimFireRing_Step1);
}
pub(crate) unsafe extern "C" fn AnimFireRing_Step1(sprite: *mut Sprite) {
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
pub(crate) unsafe extern "C" fn AnimFireRing_Step2(sprite: *mut Sprite) {
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
        (*sprite).data[7] = (*sprite).data[7] + 20 & 0xFF;
    }
}
pub(crate) unsafe extern "C" fn AnimFireRing_Step3(sprite: *mut Sprite) {
    UpdateFireRingCircleOffset(sprite);
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 0x1F
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn UpdateFireRingCircleOffset(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[7], 28);
    (*sprite).y2 = Cos((*sprite).data[7], 28);
    (*sprite).data[7] = (*sprite).data[7] + 20 & 0xFF;
}
pub(crate) unsafe extern "C" fn AnimFireCross(sprite: *mut Sprite) {
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = gBattleAnimArgs[4];
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteLinear);
}
pub(crate) unsafe extern "C" fn AnimFireSpiralOutward(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(AnimFireSpiralOutward_Step1));
}
pub(crate) unsafe extern "C" fn AnimFireSpiralOutward_Step1(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    (*sprite).data[0] = (*sprite).data[1];
    (*sprite).data[1] = 0;
    (*sprite).callback = Some(AnimFireSpiralOutward_Step2);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe extern "C" fn AnimFireSpiralOutward_Step2(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[2] >> 8);
    (*sprite).y2 = Cos((*sprite).data[1], (*sprite).data[2] >> 8);
    (*sprite).data[1] = (*sprite).data[1] + 10 & 0xFF;
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
pub unsafe extern "C" fn AnimTask_EruptionLaunchRocks(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[4] = gSprites[(*task).data[15]].y;
    (*task).data[5] = GetBattlerSide(gBattleAnimAttacker) as i16;
    (*task).data[6] = 0;
    PrepareBattlerSpriteForRotScale((*task).data[15] as u8, ST_OAM_OBJ_NORMAL);
    (*task).func = Some(AnimTask_EruptionLaunchRocks_Step);
}
pub(crate) unsafe extern "C" fn AnimTask_EruptionLaunchRocks_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
    'l1: {
        let sw1: i16 = (*task).data[0];
        let matched =
            sw1 == 0 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 4 || sw1 == 5 || sw1 == 6;
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            PrepareEruptAnimTaskData(task, (*task).data[15] as u8, 0x100, 0x100, 0xE0, 0x200, 32);
            (*task).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) as i32
                    & 1
                    != 0
                {
                    gSprites[(*task).data[15]].x2 = 3;
                } else {
                    gSprites[(*task).data[15]].x2 = -3;
                }
            }
            if (*task).data[5] != B_SIDE_PLAYER as i16 {
                if ({
                    (*task).data[3] += 1;
                    (*task).data[3]
                }) > 4
                {
                    (*task).data[3] = 0;
                    gSprites[(*task).data[15]].y += 1;
                }
            }
            if UpdateEruptAnimTask(task) == 0 {
                SetBattlerSpriteYOffsetFromYScale((*task).data[15] as u8);
                gSprites[(*task).data[15]].x2 = 0;
                (*task).data[1] = 0;
                (*task).data[2] = 0;
                (*task).data[3] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 4
            {
                if (*task).data[5] != B_SIDE_PLAYER as i16 {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[15] as u8,
                        0xE0,
                        0x200,
                        0x180,
                        0xF0,
                        6,
                    );
                } else {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[15] as u8,
                        0xE0,
                        0x200,
                        0x180,
                        0xC0,
                        6,
                    );
                }
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            if UpdateEruptAnimTask(task) == 0 {
                CreateEruptionLaunchRocks((*task).data[15] as u8, taskId, IDX_ACTIVE_SPRITES);
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 4 {
            fall = true;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) as i32
                    & 1
                    != 0
                {
                    gSprites[(*task).data[15]].y2 += 3;
                } else {
                    gSprites[(*task).data[15]].y2 -= 3;
                }
            }
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) > 24
            {
                if (*task).data[5] != B_SIDE_PLAYER as i16 {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[15] as u8,
                        0x180,
                        0xF0,
                        0x100,
                        0x100,
                        8,
                    );
                } else {
                    PrepareEruptAnimTaskData(
                        task,
                        (*task).data[15] as u8,
                        0x180,
                        0xC0,
                        0x100,
                        0x100,
                        8,
                    );
                }
                if (*task).data[2] as i32 & 1 != 0 {
                    gSprites[(*task).data[15]].y2 -= 3;
                }
                (*task).data[1] = 0;
                (*task).data[2] = 0;
                (*task).data[3] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 5 {
            fall = true;
            if (*task).data[5] != B_SIDE_PLAYER as i16 {
                gSprites[(*task).data[15]].y -= 1;
            }
            if UpdateEruptAnimTask(task) == 0 {
                gSprites[(*task).data[15]].y = (*task).data[4];
                ResetSpriteRotScale((*task).data[15] as u8);
                (*task).data[2] = 0;
                (*task).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 6 {
            fall = true;
            if (*task).data[6] == 0 {
                DestroyAnimVisualTask(taskId);
            }
            break 'l1;
        }
        if !matched {
            fall = true;
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn CreateEruptionLaunchRocks(
    spriteId: u8,
    taskId: u8,
    activeSpritesIdx: u8,
) {
    let mut i: u16 = 0;
    let mut j: u16 = 0;
    let mut sign: i8 = 0;
    let mut y: u16 = GetEruptionLaunchRockInitialYPos(spriteId);
    let mut x: u16 = gSprites[spriteId].x as u16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        x -= 12;
        sign = 1;
    } else {
        x += 16;
        sign = -1;
    }
    i = 0;
    j = 0;
    while i <= 6 {
        let mut spriteId: u8 = CreateSprite(
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
            gSprites[spriteId].data[6] = taskId as i16;
            gSprites[spriteId].data[7] = activeSpritesIdx as i16;
            gTasks[taskId].data[activeSpritesIdx] += 1;
        }
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn AnimEruptionLaunchRock(sprite: *mut Sprite) {
    UpdateEruptionLaunchRockPos(sprite);
    if (*sprite).invisible() != 0 {
        gTasks[(*sprite).data[6]].data[(*sprite).data[7]] -= 1;
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn GetEruptionLaunchRockInitialYPos(spriteId: u8) -> u16 {
    let mut y: i16 =
        gSprites[spriteId].y + gSprites[spriteId].y2 + gSprites[spriteId].centerToCornerVecY as i16;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        y += 74;
    } else {
        y += 44;
    }
    return y as u16;
}
pub(crate) unsafe extern "C" fn InitEruptionLaunchRockCoordData(
    sprite: *mut Sprite,
    speedX: i16,
    speedY: i16,
) {
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = (*sprite).x as u16 as i16 * 8;
    (*sprite).data[3] = (*sprite).y as u16 as i16 * 8;
    (*sprite).data[4] = speedX * 8;
    (*sprite).data[5] = speedY * 8;
}
pub(crate) unsafe extern "C" fn UpdateEruptionLaunchRockPos(sprite: *mut Sprite) {
    let mut extraLaunchSpeed: i32 = 0;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 2
    {
        (*sprite).data[0] = 0;
        (*sprite).data[1] += 1;
        extraLaunchSpeed = (*sprite).data[1] as u16 as i32 * (*sprite).data[1] as u16 as i32;
        (*sprite).data[3] += extraLaunchSpeed as i16;
    }
    (*sprite).data[2] += (*sprite).data[4];
    (*sprite).x = (*sprite).data[2] >> 3;
    (*sprite).data[3] += (*sprite).data[5];
    (*sprite).y = (*sprite).data[3] >> 3;
    if (*sprite).x < -8 || (*sprite).x > 248 || (*sprite).y < -8 || (*sprite).y > 120 {
        (*sprite).set_invisible(TRUE as u16);
    }
}
pub(crate) unsafe extern "C" fn AnimEruptionFallingRock(sprite: *mut Sprite) {
    (*sprite).x = gBattleAnimArgs[0];
    (*sprite).y = gBattleAnimArgs[1];
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 0;
    (*sprite).data[6] = gBattleAnimArgs[2];
    (*sprite).data[7] = gBattleAnimArgs[3];
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[4] as u16 * 16);
    (*sprite).callback = Some(AnimEruptionFallingRock_Step);
}
pub(crate) unsafe extern "C" fn AnimEruptionFallingRock_Step(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            if (*sprite).data[6] != 0 {
                (*sprite).data[6] -= 1;
                return;
            }
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            fall = true;
            (*sprite).y += 8;
            if (*sprite).y >= (*sprite).data[7] {
                (*sprite).y = (*sprite).data[7];
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            fall = true;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 1
            {
                (*sprite).data[1] = 0;
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
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
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) > 16
            {
                DestroyAnimSprite(sprite);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWillOWispOrb(sprite: *mut Sprite) {
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
            (*sprite).data[2] = (*sprite).data[2] + 4 & 0xFF;
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
            (*sprite).data[2] = (*sprite).data[2] + 4 & 0xFF;
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
pub(crate) unsafe extern "C" fn AnimWillOWispOrb_Step(sprite: *mut Sprite) {
    let mut initialData5: i16 = 0;
    let mut newData5: i16 = 0;
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).x2 += Sin((*sprite).data[5], 16);
        initialData5 = (*sprite).data[5];
        (*sprite).data[5] = (*sprite).data[5] + 4 & 0xFF;
        newData5 = (*sprite).data[5];
        if (initialData5 == 0 || initialData5 > 196) && newData5 > 0 && (*sprite).data[7] == 0 {
            PlaySE12WithPanning(SE_M_FLAME_WHEEL, gAnimCustomPanning as i8);
        }
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimWillOWispFire(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[1] = gBattleAnimArgs[0];
        (*sprite).data[0] += 1;
    }
    (*sprite).data[3] += 384;
    (*sprite).data[4] += 0xA0;
    (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[3] >> 8);
    (*sprite).y2 = Cos((*sprite).data[1], (*sprite).data[4] >> 8);
    (*sprite).data[1] = (*sprite).data[1] + 7 & 0xFF;
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
pub unsafe extern "C" fn AnimTask_MoveHeatWaveTargets(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
pub(crate) unsafe extern "C" fn AnimTask_MoveHeatWaveTargets_Step(taskId: u8) {
    let mut task: *mut Task = &raw mut gTasks[taskId];
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
pub unsafe extern "C" fn AnimTask_BlendBackground(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    GetBattleAnimBg1Data(&raw mut animBg);
    BlendPalette(
        0x000 + animBg.paletteId as u16 * 16,
        16,
        gBattleAnimArgs[0] as u8,
        gBattleAnimArgs[1] as u16,
    );
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShakeTargetInPattern(taskId: u8) {
    let mut dir: i8 = 0;
    let mut spriteId: u8 = 0;
    if gTasks[taskId].data[0] == 0 {
        gTasks[taskId].data[1] = gBattleAnimArgs[0];
        gTasks[taskId].data[2] = gBattleAnimArgs[1];
        gTasks[taskId].data[3] = gBattleAnimArgs[2];
        gTasks[taskId].data[4] = gBattleAnimArgs[3];
    }
    gTasks[taskId].data[0] += 1;
    spriteId = gBattlerSpriteIds[gBattleAnimTarget];
    if gTasks[taskId].data[4] == 0 {
        dir = sShakeDirsPattern0[gTasks[taskId].data[0] % 10];
    } else {
        dir = sShakeDirsPattern1[gTasks[taskId].data[0] % 10];
    }
    if gTasks[taskId].data[3] == TRUE as i16 {
        gSprites[spriteId].y2 = (if (gBattleAnimArgs[1] as i32 * dir as i32) < 0 {
            -(gBattleAnimArgs[1] as i32 * dir as i32)
        } else {
            gBattleAnimArgs[1] as i32 * dir as i32
        }) as i16;
    } else {
        gSprites[spriteId].x2 = gBattleAnimArgs[1] * dir as i16;
    }
    if gTasks[taskId].data[0] == gTasks[taskId].data[1] {
        gSprites[spriteId].x2 = 0;
        gSprites[spriteId].y2 = 0;
        DestroyAnimVisualTask(taskId);
    }
}
