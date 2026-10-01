//! Translated from `src/battle_anim_rock.c` by tools/rustport/c2rs.py.
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
    BattleAnimAdjustPanning, DestroyAnimSprite, DestroyAnimVisualTask, IsContest,
    gAnimDisableStructPtr, gAnimMoveDmg, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimLoadCompressedBgGfx, AnimLoadCompressedBgTilemapHandleContest, ClearBattleAnimBg,
    DestroySpriteAndMatrix, GetAnimBattlerSpriteId, GetBattleAnimBg1Data, GetBattlerSide,
    GetBattlerSpriteCoord, InitAnimArcTranslation, InitSpriteDataForLinearTranslation,
    InitSpritePosToAnimAttacker, InitSpritePosToAnimTarget, SetAverageBattlerPositions,
    StartAnimLinearTranslation, StoreSpriteCallbackInData6, TranslateAnimHorizontalArc,
    TranslateAnimSpriteToTargetMonLocation, TranslateSpriteInEllipse,
    TranslateSpriteLinearFixedPoint, UpdateAnimBg3ScreenSize,
};
use crate::battle_anim_utility_funcs::SetAnimBgAttribute;
use crate::battle_main::{gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG3_Y};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::palette::LoadCompressedPalette;
use crate::sound::PlaySE12WithPanning;
use crate::sprite::gSprites;
use crate::task::gTasks;
use crate::task::{task_get, task_set, task_set_func};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `AnimateSprite` with this module's view of its types.
#[inline]
unsafe fn AnimateSprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::AnimateSprite(a0 as _);
    }
}
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
/// `FindTaskIdByFunc` with this module's view of its types.
#[inline]
unsafe fn FindTaskIdByFunc(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FindTaskIdByFunc(core::mem::transmute(a0)) }
}
/// `SetSubspriteTables` with this module's view of its types.
#[inline]
unsafe fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable) {
    unsafe {
        crate::sprite::SetSubspriteTables(a0 as _, a1 as _);
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
// The C's names for task and sprite data slots.
const sState: usize = 0;
const sVelocityX: usize = 1;
const sVelocityY: usize = 2;
const sFractionalX: usize = 3;
const sFractionalY: usize = 4;
const tBlendTimer: usize = 10;
const tState: usize = 12;
// Data tables (translate with cdata.py): sAnim_FlyingRock_0 sAnim_FlyingRock_1 sAnim_FlyingRock_2 sAnims_FlyingRock gFallingRockSpriteTemplate gRockFragmentSpriteTemplate gSwirlingDirtSpriteTemplate sAffineAnim_Whirlpool sAffineAnims_Whirlpool gWhirlpoolSpriteTemplate gFireSpinSpriteTemplate gFlyingSandCrescentSpriteTemplate sFlyingSandSubsprites sFlyingSandSubspriteTable sAnim_Rock_Biggest sAnim_Rock_Bigger sAnim_Rock_Big sAnim_Rock_Small sAnim_Rock_Smaller sAnim_Rock_Smallest sAnims_BasicRock gAncientPowerRockSpriteTemplate gRolloutMudSpriteTemplate gRolloutRockSpriteTemplate gRockTombRockSpriteTemplate sAffineAnim_BasicRock_0 sAffineAnim_BasicRock_1 sAffineAnims_BasicRock gRockBlastRockSpriteTemplate gRockScatterSpriteTemplate gTwisterRockSpriteTemplate gWeatherBallRockDownSpriteTemplate

static gRolloutMudSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_rock::gRolloutMudSpriteTemplate).cast());
static gRolloutRockSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_rock::gRolloutRockSpriteTemplate).cast());
static sFlyingSandSubspriteTable: Table<CArray<SubspriteTable, 1>> =
    Table((&raw const crate::data::battle_anim_rock::sFlyingSandSubspriteTable).cast());

pub(crate) unsafe fn AnimFallingRock(sprite: *mut Sprite) {
    if gBattleAnimArgs[3] != 0 {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            FALSE,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
    }
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += 14;
    StartSpriteAnim(sprite, gBattleAnimArgs[1] as u8);
    AnimateSprite(sprite);
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 4;
    (*sprite).data[3] = 16;
    (*sprite).data[4] = -70;
    (*sprite).data[5] = gBattleAnimArgs[2];
    StoreSpriteCallbackInData6(sprite, Some(AnimFallingRock_Step));
    (*sprite).callback = Some(TranslateSpriteInEllipse);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimFallingRock_Step(sprite: *mut Sprite) {
    (*sprite).x += (*sprite).data[5];
    (*sprite).data[0] = 192;
    (*sprite).data[1] = (*sprite).data[5];
    (*sprite).data[2] = 4;
    (*sprite).data[3] = 32;
    (*sprite).data[4] = -24;
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteInEllipse);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimRockFragment(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[5] as u8);
    AnimateSprite(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[3];
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe fn AnimParticleInVortex(sprite: *mut Sprite) {
    if gBattleAnimArgs[6] == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    } else {
        InitSpritePosToAnimTarget(sprite, FALSE);
    }
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[2] = gBattleAnimArgs[4];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).callback = Some(AnimParticleInVortex_Step);
}
pub(crate) unsafe fn AnimParticleInVortex_Step(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[1];
    (*sprite).y2 = -((*sprite).data[4] >> 8);
    (*sprite).x2 = Sin((*sprite).data[5], (*sprite).data[3]);
    (*sprite).data[5] = ((*sprite).data[5] + (*sprite).data[2]) & 0xFF;
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == -1
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_LoadSandstormBackground(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut var0: i32 = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
    SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
    }
    gBattle_BG1_X = 0;
    gBattle_BG1_Y = 0;
    SetGpuReg(REG_OFFSET_BG1HOFS, gBattle_BG1_X);
    SetGpuReg(REG_OFFSET_BG1VOFS, gBattle_BG1_Y);
    GetBattleAnimBg1Data(&raw mut animBg);
    AnimLoadCompressedBgGfx(
        animBg.bgId as u32,
        (*(&raw const crate::data::graphics::gBattleAnimBgImage_Sandstorm)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        animBg.tilesOffset as u32,
    );
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBg,
        (*(&raw const crate::data::graphics::gBattleAnimBgTilemap_Sandstorm)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
        FALSE as u32,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleAnimSpritePal_FlyingDirt)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        animBg.paletteId as u16 * 16,
        32,
    );
    if gBattleAnimArgs[0] != 0 && GetBattlerSide(gBattleAnimAttacker) != 0 {
        var0 = 1;
    }
    task_set(taskId, 0, var0 as i16);
    task_set_func(taskId, Some(AnimTask_LoadSandstormBackground_Step));
}
pub(crate) unsafe fn AnimTask_LoadSandstormBackground_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    if task_get(taskId, 0) == 0 {
        gBattle_BG1_X += 65530;
    } else {
        gBattle_BG1_X += 6;
    }
    gBattle_BG1_Y += 65535;
    match task_get(taskId, tState) {
        0 => {
            if ({
                task_set(taskId, tBlendTimer, task_get(taskId, tBlendTimer) + 1);
                task_get(taskId, tBlendTimer)
            }) == 4
            {
                task_set(taskId, tBlendTimer, 0);
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 11) as u16) << 8 | task_get(taskId, 11) as u16,
                );
                if task_get(taskId, 11) == 7 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    task_set(taskId, 11, 0);
                }
            }
        }
        1 => {
            if ({
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                task_get(taskId, 11)
            }) == 101
            {
                task_set(taskId, 11, 7);
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        2 => {
            if ({
                task_set(taskId, tBlendTimer, task_get(taskId, tBlendTimer) + 1);
                task_get(taskId, tBlendTimer)
            }) == 4
            {
                task_set(taskId, tBlendTimer, 0);
                task_set(taskId, 11, task_get(taskId, 11) - 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 11) as u16) << 8 | task_get(taskId, 11) as u16,
                );
                if task_get(taskId, 11) == 0 {
                    task_set(taskId, tState, task_get(taskId, tState) + 1);
                    task_set(taskId, 11, 0);
                }
            }
        }
        3 => {
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(animBg.bgId as u32);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        4 => {
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimFlyingSandCrescent(sprite: *mut Sprite) {
    if (*sprite).data[sState] == 0 {
        if gBattleAnimArgs[3] != 0 && GetBattlerSide(gBattleAnimAttacker) != 0 {
            (*sprite).x = 304;
            gBattleAnimArgs[1] = -gBattleAnimArgs[1];
            (*sprite).data[5] = 1;
            (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
        } else {
            (*sprite).x = -64;
        }
        (*sprite).y = gBattleAnimArgs[0];
        SetSubspriteTables(sprite, sFlyingSandSubspriteTable.as_ptr().cast_mut());
        (*sprite).data[sVelocityX] = gBattleAnimArgs[1];
        (*sprite).data[sVelocityY] = gBattleAnimArgs[2];
        (*sprite).data[sState] += 1;
    } else {
        (*sprite).data[sFractionalX] += (*sprite).data[sVelocityX];
        (*sprite).data[sFractionalY] += (*sprite).data[sVelocityY];
        (*sprite).x2 += (*sprite).data[sFractionalX] >> 8;
        (*sprite).y2 += (*sprite).data[sFractionalY] >> 8;
        (*sprite).data[sFractionalX] &= 0xFF;
        (*sprite).data[sFractionalY] &= 0xFF;
        if (*sprite).data[5] == 0 {
            if (*sprite).x as i32 + (*sprite).x2 as i32 > 272 {
                (*sprite).callback = Some(DestroyAnimSprite);
            }
        } else if ((*sprite).x as i32 + (*sprite).x2 as i32) < -32 {
            (*sprite).callback = Some(DestroyAnimSprite);
        }
    }
}
pub(crate) unsafe fn AnimRaiseSprite(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[4] as u8);
    InitSpritePosToAnimAttacker(sprite, FALSE);
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[4] = (*sprite).y + gBattleAnimArgs[2];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_Rollout(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let var0: u16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as u16;
    let var1: u16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as u16 + 24;
    let var2: u16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as u16;
    let mut var3: u16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as u16 + 24;
    if gBattleAnimAttacker as i32 ^ 2 == gBattleAnimTarget as i32 {
        var3 = var1;
    }
    let rolloutCounter: u8 = GetRolloutCounter();
    if rolloutCounter == 1 {
        (*task).data[8] = 32;
    } else {
        (*task).data[8] = 48 - rolloutCounter as i16 * 8;
    }
    (*task).data[0] = 0;
    (*task).data[11] = 0;
    (*task).data[9] = 0;
    (*task).data[12] = 1;
    (*task).data[10] = (*task).data[8] / 8 - 1;
    (*task).data[2] = var0 as i16 * 8;
    (*task).data[3] = var1 as i16 * 8;
    (*task).data[4] = div_i32((var2 as i32 - var0 as i32) * 8, (*task).data[8] as i32) as i16;
    (*task).data[5] = div_i32((var3 as i32 - var1 as i32) * 8, (*task).data[8] as i32) as i16;
    (*task).data[6] = 0;
    (*task).data[7] = 0;
    let pan1: i16 = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER) as i16;
    let pan2: i16 = BattleAnimAdjustPanning(SOUND_PAN_TARGET) as i16;
    (*task).data[13] = pan1;
    (*task).data[14] = div_i32(pan2 as i32 - pan1 as i32, (*task).data[8] as i32) as i16;
    (*task).data[1] = rolloutCounter as i16;
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).func = Some(AnimTask_Rollout_Step);
}
pub(crate) unsafe fn AnimTask_Rollout_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[6] -= (*task).data[4];
            (*task).data[7] -= (*task).data[5];
            gSprites[(*task).data[15]].x2 = (*task).data[6] >> 3;
            gSprites[(*task).data[15]].y2 = (*task).data[7] >> 3;
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) == 10
            {
                (*task).data[11] = 20;
                (*task).data[0] += 1;
            }
            PlaySE12WithPanning(SE_M_HEADBUTT, (*task).data[13] as i8);
        }
        1 => {
            if ({
                (*task).data[11] -= 1;
                (*task).data[11]
            }) == 0
            {
                (*task).data[0] += 1;
            }
        }
        2 => {
            if ({
                (*task).data[9] -= 1;
                (*task).data[9]
            }) != 0
            {
                (*task).data[6] += (*task).data[4];
                (*task).data[7] += (*task).data[5];
            } else {
                (*task).data[6] = 0;
                (*task).data[7] = 0;
                (*task).data[0] += 1;
            }
            gSprites[(*task).data[15]].x2 = (*task).data[6] >> 3;
            gSprites[(*task).data[15]].y2 = (*task).data[7] >> 3;
        }
        3 => {
            (*task).data[2] += (*task).data[4];
            (*task).data[3] += (*task).data[5];
            if ({
                (*task).data[9] += 1;
                (*task).data[9]
            }) >= (*task).data[10]
            {
                (*task).data[9] = 0;
                CreateRolloutDirtSprite(task);
                (*task).data[13] += (*task).data[14];
                PlaySE12WithPanning(SE_M_DIG, (*task).data[13] as i8);
            }
            if ({
                (*task).data[8] -= 1;
                (*task).data[8]
            }) == 0
            {
                (*task).data[0] += 1;
            }
        }
        4 if (*task).data[11] == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn CreateRolloutDirtSprite(task: *mut Task) {
    let mut spriteTemplate: *mut SpriteTemplate = null_mut();
    let mut tileOffset: i32 = 0;
    match (*task).data[1] {
        1 => {
            spriteTemplate = (&raw const *gRolloutMudSpriteTemplate).cast_mut();
            tileOffset = 0;
        }
        2 | 3 => {
            spriteTemplate = (&raw const *gRolloutRockSpriteTemplate).cast_mut();
            tileOffset = 80;
        }
        4 => {
            spriteTemplate = (&raw const *gRolloutRockSpriteTemplate).cast_mut();
            tileOffset = 64;
        }
        5 => {
            spriteTemplate = (&raw const *gRolloutRockSpriteTemplate).cast_mut();
            tileOffset = 48;
        }
        _ => {
            return;
        }
    }
    let mut x: u16 = ((*task).data[2] >> 3) as u16;
    let y: u16 = ((*task).data[3] >> 3) as u16;
    x += (*task).data[12] as u16 * 4;
    let spriteId: u8 = CreateSprite(spriteTemplate, x as i16, y as i16, 35);
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[0] = 18;
        gSprites[spriteId].data[2] = (*task).data[12] * 20 + x as i16 + (*task).data[1] * 3;
        gSprites[spriteId].data[4] = y as i16;
        gSprites[spriteId].data[5] = -16 - (*task).data[1] * 2;
        gSprites[spriteId]
            .oam
            .set_tileNum(gSprites[spriteId].oam.tileNum() + tileOffset as u16);
        InitAnimArcTranslation(&raw mut gSprites[spriteId]);
        (*task).data[11] += 1;
    }
    (*task).data[12] *= -1;
}
pub(crate) unsafe fn AnimRolloutParticle(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        let taskId: u8 = FindTaskIdByFunc(Some(AnimTask_Rollout_Step));
        if taskId != TASK_NONE {
            task_set(taskId, 11, task_get(taskId, 11) - 1);
        }
        DestroySprite(sprite);
    }
}
unsafe fn GetRolloutCounter() -> u8 {
    let mut retVal: u8 =
        (*gAnimDisableStructPtr).rolloutTimerStartValue() - (*gAnimDisableStructPtr).rolloutTimer();
    let var0: u8 = retVal - 1;
    if var0 > 4 {
        retVal = 1;
    }
    retVal
}
pub(crate) unsafe fn AnimRockTomb(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[4] as u8);
    (*sprite).x2 = gBattleAnimArgs[0];
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).data[3] -= gBattleAnimArgs[2];
    (*sprite).data[0] = 3;
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimRockTomb_Step);
    (*sprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe fn AnimRockTomb_Step(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    if (*sprite).data[3] != 0 {
        (*sprite).y2 = (*sprite).data[2] + (*sprite).data[3];
        (*sprite).data[3] += (*sprite).data[0];
        (*sprite).data[0] += 1;
        if (*sprite).data[3] > 0 {
            (*sprite).data[3] = 0;
        }
    } else {
        if ({
            (*sprite).data[1] -= 1;
            (*sprite).data[1]
        }) == 0
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimRockBlastRock(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        StartSpriteAffineAnim(sprite, 1);
    }
    TranslateAnimSpriteToTargetMonLocation(sprite);
}
pub(crate) unsafe fn AnimRockScatter(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[1] = gBattleAnimArgs[0];
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).data[5] = gBattleAnimArgs[2];
    StartSpriteAnim(sprite, gBattleAnimArgs[3] as u8);
    (*sprite).callback = Some(AnimRockScatter_Step);
}
pub(crate) unsafe fn AnimRockScatter_Step(sprite: *mut Sprite) {
    (*sprite).data[0] += 8;
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).x2 += (*sprite).data[3] / 40;
    (*sprite).y2 -= Sin((*sprite).data[0], (*sprite).data[5]);
    if (*sprite).data[0] > 140 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetSeismicTossDamageLevel(taskId: u8) {
    if gAnimMoveDmg < 33 {
        gBattleAnimArgs[7] = 0;
    }
    if gAnimMoveDmg as u32 - 33 < 33 {
        gBattleAnimArgs[7] = 1;
    }
    if gAnimMoveDmg > 65 {
        gBattleAnimArgs[7] = 2;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MoveSeismicTossBg(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        UpdateAnimBg3ScreenSize(FALSE);
        task_set(taskId, 1, 200);
    }
    gBattle_BG3_Y += (task_get(taskId, 1) / 10) as u16;
    task_set(taskId, 1, task_get(taskId, 1) - 3);
    if task_get(taskId, 0) == 120 {
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyAnimVisualTask(taskId);
    }
    task_set(taskId, 0, task_get(taskId, 0) + 1);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SeismicTossBgAccelerateDownAtEnd(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        UpdateAnimBg3ScreenSize(FALSE);
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_set(taskId, 2, gBattle_BG3_Y as i16);
    }
    task_set(taskId, 1, task_get(taskId, 1) + 80);
    task_set(taskId, 1, task_get(taskId, 1) & 0xFF);
    gBattle_BG3_Y = task_get(taskId, 2) as u16 + Cos(4, task_get(taskId, 1)) as u16;
    if gBattleAnimArgs[7] == 0xFFF {
        gBattle_BG3_Y = 0;
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyAnimVisualTask(taskId);
    }
}
