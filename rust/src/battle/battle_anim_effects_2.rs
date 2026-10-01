//! Translated from `src/battle_anim_effects_2.c` by tools/rustport/c2rs.py.
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
    BattleAnimAdjustPanning, DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible,
    IsContest, gAnimDisableStructPtr, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_effects_1::SetSpriteNextToMonHead;
use crate::battle_anim_mons::{
    AnimLoadCompressedBgGfx, AnimLoadCompressedBgTilemapHandleContest, AnimSpriteOnMonPos,
    AnimTranslateLinear, ArcTan2Neg, ClearBattleAnimBg, CloneBattlerSpriteWithBlend,
    DestroySpriteAndMatrix, DestroySpriteWithActiveSheet, GetAnimBattlerSpriteId,
    GetBattleAnimBg1Data, GetBattlePalettesMask, GetBattlerSide, GetBattlerSpriteBGPriorityRank,
    GetBattlerSpriteCoord, GetBattlerSpriteCoordAttr, GetBattlerSpriteSubpriority,
    GetBattlerYCoordWithElevation, InitAnimLinearTranslation,
    InitAnimLinearTranslationWithSpeedAndPos, InitSpritePosToAnimAttacker, IsDoubleBattle,
    PrepareAffineAnimInTaskData, PrepareBattlerSpriteForRotScale, ResetSpriteRotScale,
    RunAffineAnimFromTaskData, RunStoredCallbackWhenAffineAnimEnds, RunStoredCallbackWhenAnimEnds,
    SetAnimSpriteInitialXOffset, SetAverageBattlerPositions, SetBattlerSpriteYOffsetFromRotation,
    SetBattlerSpriteYOffsetFromYScale, SetGrayscaleOrOriginalPalette,
    SetSpriteCoordsToAnimAttackerCoords, SetSpritePrimaryCoordsFromSecondaryCoords,
    SetSpriteRotScale, StartAnimLinearTranslation, StoreSpriteCallbackInData6,
    TranslateSpriteInCircle, TranslateSpriteInEllipse, TranslateSpriteLinearFixedPoint,
    TrySetSpriteRotScale, WaitAnimForDuration,
};
use crate::battle_anim_utility_funcs::SetAnimBgAttribute;
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_WIN0H, gBattle_WIN0V, gBattleTypeFlags,
    gMonSpritesGfxPtr,
};
use crate::battle_main::{gBattlerPositions, gBattlerSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::math_util::{MathUtil_Inv16, MathUtil_Mul16};
use crate::palette::gPlttBufferFaded;
use crate::palette::{BlendPalettes, LoadCompressedPalette, LoadPalette};
use crate::random::Random2;
use crate::sound::PlaySE12WithPanning;
use crate::sprite::gSprites;
use crate::sprite::{
    AllocOamMatrix, AllocSpritePalette, FreeOamMatrix, FreeSpritePaletteByTag,
    IndexOfSpritePaletteTag,
};
use crate::task::{gTasks, task_get, task_set, task_set_func};
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
/// `CalcCenterToCornerVec` with this module's view of its types.
#[inline]
unsafe fn CalcCenterToCornerVec(a0: *mut Sprite, a1: u8, a2: u8, a3: u8) {
    unsafe {
        crate::sprite::CalcCenterToCornerVec(a0 as _, a1, a2, a3);
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
/// `Free` with this module's view of its types.
#[inline]
unsafe fn Free(a0: *mut c_void) {
    unsafe {
        crate::malloc::Free(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `LZDecompressWram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressWram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressWram(a0 as _, a1 as _);
    }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
    }
}
/// `SeekSpriteAnim` with this module's view of its types.
#[inline]
unsafe fn SeekSpriteAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::SeekSpriteAnim(a0 as _, a1);
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
const sAmplitudeX: usize = 1;
const sCircleSpeed: usize = 2;
const sMoveSteps: usize = 3;
const sAmplitudeY: usize = 4;
// Data tables (translate with cdata.py): sCirclingFingerSpriteTemplate sAnim_BouncingMusicNote sAnims_BouncingMusicNote sBouncingMusicNoteSpriteTemplate sVibrateBattlerBackSpriteTemplate sMovingClampSpriteTemplate sAnim_SmallExplosion sAnims_SmallExplosion sAffineAnim_SmallExplosion sAffineAnims_SmallExplosion sSmallExplosionSpriteTemplate gKinesisZapEnergyAnimCmds gKinesisZapEnergyAnimTable gKinesisZapEnergySpriteTemplate gSwordsDanceBladeAffineAnimCmds gSwordsDanceBladeAffineAnimTable gSwordsDanceBladeSpriteTemplate gSonicBoomSpriteTemplate gAirWaveProjectileSpriteTemplate gGrowingRingAffineAnimCmds gWaterPulseRingAffineAnimCmds gGrowingRingAffineAnimTable gWaterPulseRingAffineAnimTable gSupersonicRingSpriteTemplate gScreechRingSpriteTemplate gMetalSoundSpriteTemplate gWaterPulseRingSpriteTemplate gEggThrowSpriteTemplate sVoidLinesSpriteTemplate gCoinAnimCmds gCoinAnimTable gFallingCoinAffineAnimCmds gFallingCoinAffineAnimTable gCoinThrowSpriteTemplate gFallingCoinSpriteTemplate gBulletSeedAffineAnimCmds gBulletSeedAffineAnimTable gBulletSeedSpriteTemplate gRazorWindTornadoAffineAnimCmds gRazorWindTornadoAffineAnimTable gRazorWindTornadoSpriteTemplate gViceGripAnimCmds1 gViceGripAnimCmds2 gViceGripAnimTable gViceGripSpriteTemplate gGuillotineAnimCmds1 gGuillotineAnimCmds2 gGuillotineAnimTable gGuillotineSpriteTemplate gSplashEffectAffineAnimCmds gGrowAndShrinkAffineAnimCmds gBreathPuffAnimCmds1 gBreathPuffAnimCmds2 gBreathPuffAnimTable gBreathPuffSpriteTemplate gAngerMarkAffineAnimCmds gAngerMarkAffineAnimTable gAngerMarkSpriteTemplate gThrashMoveMonAffineAnimCmds gPencilSpriteTemplate gSnoreZSpriteTemplate gExplosionAnimCmds gExplosionAnimTable gExplosionSpriteTemplate gSoftBoiledEggAffineAnimCmds1 gSoftBoiledEggAffineAnimCmds2 gSoftBoiledEggAffineAnimCmds3 gSoftBoiledEggAffineAnimTable gSoftBoiledEggSpriteTemplate gThinRingExpandingAffineAnimCmds1 gThinRingExpandingAffineAnimCmds2 gHyperVoiceRingAffineAnimCmds gThinRingExpandingAffineAnimTable gHyperVoiceRingAffineAnimTable gThinRingExpandingSpriteTemplate gThinRingShrinkingAffineAnimCmds gThinRingShrinkingAffineAnimTable gThinRingShrinkingSpriteTemplate gBlendThinRingExpandingSpriteTemplate gHyperVoiceRingSpriteTemplate gUproarRingSpriteTemplate gStretchAttackerAffineAnimCmds gSpeedDustAnimCmds gSpeedDustAnimTable gSpeedDustSpriteTemplate gSpeedDustPosTable gBellAnimCmds gBellAnimTable gBellSpriteTemplate sMusicNotePaletteTagsTable gHealBellMusicNoteSpriteTemplate gMagentaHeartSpriteTemplate sAffineAnims_StretchBattlerUp gRedHeartProjectileSpriteTemplate gRedHeartBurstSpriteTemplate gRedHeartRisingSpriteTemplate gHiddenPowerOrbAffineAnimCmds gHiddenPowerOrbAffineAnimTable gHiddenPowerOrbSpriteTemplate gHiddenPowerOrbScatterSpriteTemplate gSpitUpOrbAffineAnimCmds gSpitUpOrbAffineAnimTable gSpitUpOrbSpriteTemplate gEyeSparkleAnimCmds gEyeSparkleAnimTable gEyeSparkleSpriteTemplate gAngelSpriteAnimCmds gAngelSpriteAnimTable gAngelSpriteTemplate gPinkHeartSpriteTemplate gDevilAnimCmds1 gDevilAnimCmds2 gDevilAnimTable gDevilSpriteTemplate sAnim_FurySwipes sAnim_FurySwipes_Flipped sAnims_FurySwipes gFurySwipesSpriteTemplate gMovementWavesAnimCmds1 gMovementWavesAnimCmds2 gMovementWavesAnimTable gMovementWavesSpriteTemplate sAffineAnims_UproarDistortion gJaggedMusicNoteSpriteTemplate gPerishSongMusicNoteAffineAnimCmds1 gPerishSongMusicNoteAffineAnimCmds2 gPerishSongMusicNoteAffineAnimCmds3 gPerishSongMusicNoteAffineAnimTable gPerishSongMusicNoteSpriteTemplate gPerishSongMusicNote2SpriteTemplate gGuardRingAffineAnimCmds1 gGuardRingAffineAnimCmds2 gGuardRingAffineAnimTable gGuardRingSpriteTemplate

const NUM_MUSIC_NOTE_PAL_TAGS: i32 = 3;

static gAirWaveProjectileSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_2::gAirWaveProjectileSpriteTemplate).cast());
static gGrowAndShrinkAffineAnimCmds: Table<CArray<AffineAnimCmd, 4>> =
    Table((&raw const crate::data::battle_anim_effects_2::gGrowAndShrinkAffineAnimCmds).cast());
static gSpeedDustPosTable: Table<CArray<CArray<i8, 2>, 4>> =
    Table((&raw const crate::data::battle_anim_effects_2::gSpeedDustPosTable).cast());
static gSpeedDustSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_2::gSpeedDustSpriteTemplate).cast());
static gSplashEffectAffineAnimCmds: Table<CArray<AffineAnimCmd, 4>> =
    Table((&raw const crate::data::battle_anim_effects_2::gSplashEffectAffineAnimCmds).cast());
static gStretchAttackerAffineAnimCmds: Table<CArray<AffineAnimCmd, 2>> =
    Table((&raw const crate::data::battle_anim_effects_2::gStretchAttackerAffineAnimCmds).cast());
static gThrashMoveMonAffineAnimCmds: Table<CArray<AffineAnimCmd, 6>> =
    Table((&raw const crate::data::battle_anim_effects_2::gThrashMoveMonAffineAnimCmds).cast());
static sAffineAnims_StretchBattlerUp: Table<CArray<AffineAnimCmd, 3>> =
    Table((&raw const crate::data::battle_anim_effects_2::sAffineAnims_StretchBattlerUp).cast());
static sAffineAnims_UproarDistortion: Table<CArray<AffineAnimCmd, 4>> =
    Table((&raw const crate::data::battle_anim_effects_2::sAffineAnims_UproarDistortion).cast());
static sMusicNotePaletteTagsTable: Table<CArray<u16, 3>> =
    Table((&raw const crate::data::battle_anim_effects_2::sMusicNotePaletteTagsTable).cast());
static sVoidLinesSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_2::sVoidLinesSpriteTemplate).cast());

/// `AllocZeroed` with this module's view of its types.
#[inline]
unsafe fn AllocZeroed(a0: u32) -> *mut c_void {
    unsafe { crate::malloc::AllocZeroed(a0) as *mut c_void }
}

pub(crate) unsafe fn AnimCirclingFinger(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[sAmplitudeX] = gBattleAnimArgs[2];
    (*sprite).data[sCircleSpeed] = gBattleAnimArgs[4];
    (*sprite).data[sMoveSteps] = gBattleAnimArgs[5];
    (*sprite).data[sAmplitudeY] = gBattleAnimArgs[3];
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteInEllipse);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimBouncingMusicNote(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    SetSpriteNextToMonHead(battler, sprite);
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).callback = Some(AnimBouncingMusicNote_Step);
}
pub(crate) unsafe fn AnimBouncingMusicNote_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).y2 -= 3;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 6
            {
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            (*sprite).y2 += 3;
            if ({
                (*sprite).data[1] -= 1;
                (*sprite).data[1]
            }) == 0
            {
                (*sprite).data[0] += 1;
            }
        }
        2 if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 64 =>
        {
            DestroyAnimSprite(sprite);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimVibrateBattlerBack_Step(sprite: *mut Sprite) {
    gSprites[(*sprite).data[2]].x2 += (*sprite).data[1];
    let temp: i16 = (*sprite).data[1];
    (*sprite).data[1] = -temp;
    if (*sprite).data[0] == 0 {
        gSprites[(*sprite).data[2]].x2 = 0;
        DestroySpriteAndMatrix(sprite);
    }
    (*sprite).data[0] -= 1;
}
pub(crate) unsafe fn AnimVibrateBattlerBack(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimTarget];
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = spriteId as i16;
    (*sprite).callback = Some(AnimVibrateBattlerBack_Step);
    (*sprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe fn AnimMovingClamp(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[5] = gBattleAnimArgs[4];
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(AnimMovingClamp_Step));
}
pub(crate) unsafe fn AnimMovingClamp_Step(sprite: *mut Sprite) {
    (*sprite).data[0] = (*sprite).data[1];
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[4] = (*sprite).y + 15;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(AnimMovingClamp_End));
}
pub(crate) unsafe fn AnimMovingClamp_End(sprite: *mut Sprite) {
    if (*sprite).data[5] == 0 {
        DestroyAnimSprite(sprite);
    } else {
        (*sprite).data[5] -= 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_Withdraw(taskId: u8) {
    PrepareBattlerSpriteForRotScale(gBattlerSpriteIds[gBattleAnimAttacker], ST_OAM_OBJ_NORMAL);
    task_set_func(taskId, Some(AnimTask_Withdraw_Step));
}
pub(crate) unsafe fn AnimTask_Withdraw_Step(taskId: u8) {
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimAttacker];
    let mut rotation: i16 = 0;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        rotation = -task_get(taskId, 0);
    } else {
        rotation = task_get(taskId, 0);
    }
    SetSpriteRotScale(spriteId, 0x100, 0x100, rotation as u16);
    if task_get(taskId, 1) == 0 {
        task_set(taskId, 0, task_get(taskId, 0) + 0xB0);
        gSprites[spriteId].y2 += 1;
    } else if task_get(taskId, 1) == 1 {
        if ({
            task_set(taskId, 3, task_get(taskId, 3) + 1);
            task_get(taskId, 3)
        }) == 30
        {
            task_set(taskId, 1, 2);
        }
        return;
    } else {
        task_set(taskId, 0, task_get(taskId, 0) - 0xB0);
        gSprites[spriteId].y2 -= 1;
    }
    SetBattlerSpriteYOffsetFromRotation(spriteId);
    if task_get(taskId, 0) == 0xF20 || task_get(taskId, 0) == 0 {
        if task_get(taskId, 1) == 2 {
            ResetSpriteRotScale(spriteId);
            DestroyAnimVisualTask(taskId);
        } else {
            task_set(taskId, 1, task_get(taskId, 1) + 1);
        }
    }
}
pub(crate) unsafe fn AnimKinesisZapEnergy(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
    } else {
        (*sprite).x += gBattleAnimArgs[0];
    }
    (*sprite).y += gBattleAnimArgs[1];
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).set_hFlip(1);
        if gBattleAnimArgs[2] != 0 {
            (*sprite).set_vFlip(1);
        }
    } else {
        if gBattleAnimArgs[2] != 0 {
            (*sprite).set_vFlip(1);
        }
    }
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimSwordsDanceBlade(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, FALSE);
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(AnimSwordsDanceBlade_Step));
}
pub(crate) unsafe fn AnimSwordsDanceBlade_Step(sprite: *mut Sprite) {
    (*sprite).data[0] = 6;
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[4] = (*sprite).y - 32;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimSonicBoomProjectile(sprite: *mut Sprite) {
    if IsContest() != 0 {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    } else if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
    }
    InitSpritePosToAnimAttacker(sprite, TRUE);
    let targetXPos: i16 =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[2];
    let targetYPos: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET)
        as i16
        + gBattleAnimArgs[3];
    let mut rotation: u16 = ArcTan2Neg(targetXPos - (*sprite).x, targetYPos - (*sprite).y);
    rotation -= 0x1000;
    if IsContest() != 0 {
        rotation -= 0x6000;
    }
    TrySetSpriteRotScale(sprite, FALSE, 0x100, 0x100, rotation);
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] = targetXPos;
    (*sprite).data[4] = targetYPos;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimAirWaveProjectile_Step2(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] -= 1;
        t1
    }) <= 0
    {
        task_set((*sprite).data[7], 1, task_get((*sprite).data[7], 1) - 1);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn AnimAirWaveProjectile_Step1(sprite: *mut Sprite) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[(*sprite).data[7]];
    if (*sprite).data[0] > (*task).data[5] {
        (*sprite).data[5] += (*sprite).data[3];
        (*sprite).data[6] += (*sprite).data[4];
    } else {
        (*sprite).data[5] -= (*sprite).data[3];
        (*sprite).data[6] -= (*sprite).data[4];
    }
    (*sprite).data[1] += (*sprite).data[5];
    (*sprite).data[2] += (*sprite).data[6];
    if 1 & (*task).data[7] as i32 != 0 {
        (*sprite).x2 = -(((*sprite).data[1] as u16 >> 8) as i16);
    } else {
        (*sprite).x2 = ((*sprite).data[1] as u16 >> 8) as i16;
    }
    if 1 & (*task).data[8] as i32 != 0 {
        (*sprite).y2 = -(((*sprite).data[2] as u16 >> 8) as i16);
    } else {
        (*sprite).y2 = ((*sprite).data[2] as u16 >> 8) as i16;
    }
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] -= 1;
        t1
    }) <= 0
    {
        (*sprite).data[0] = 30;
        (*sprite).callback = Some(AnimAirWaveProjectile_Step2);
    }
}
pub(crate) unsafe fn AnimAirWaveProjectile(sprite: *mut Sprite) {
    let mut a: i16 = 0;
    let mut b: i16 = 0;
    let mut c: i16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[(*sprite).data[7]];
    (*sprite).data[1] += -2 & (*task).data[7];
    (*sprite).data[2] += -2 & (*task).data[8];
    if (*task).data[7] as i32 & 1 != 0 {
        (*sprite).x2 = -(((*sprite).data[1] as u16 >> 8) as i16);
    } else {
        (*sprite).x2 = ((*sprite).data[1] as u16 >> 8) as i16;
    }
    if (*task).data[8] as i32 & 1 != 0 {
        (*sprite).y2 = -(((*sprite).data[2] as u16 >> 8) as i16);
    } else {
        (*sprite).y2 = ((*sprite).data[2] as u16 >> 8) as i16;
    }
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] -= 1;
        t1
    }) <= 0
    {
        (*sprite).data[0] = 8;
        (*task).data[5] = 4;
        a = MathUtil_Inv16(4096);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        if (*task).data[11] >= (*sprite).x {
            b = ((*task).data[11] - (*sprite).x) << 8;
        } else {
            b = ((*sprite).x - (*task).data[11]) << 8;
        }
        if (*task).data[12] >= (*sprite).y {
            c = ((*task).data[12] - (*sprite).y) << 8;
        } else {
            c = ((*sprite).y - (*task).data[12]) << 8;
        }
        (*sprite).data[2] = 0;
        (*sprite).data[1] = 0;
        (*sprite).data[6] = 0;
        (*sprite).data[5] = 0;
        (*sprite).data[3] = MathUtil_Mul16(
            MathUtil_Mul16(b, a),
            MathUtil_Inv16((1_f32 * 256_f32) as i16),
        );
        (*sprite).data[4] = MathUtil_Mul16(
            MathUtil_Mul16(c, a),
            MathUtil_Inv16((1_f32 * 256_f32) as i16),
        );
        (*sprite).callback = Some(AnimAirWaveProjectile_Step1);
    }
}
pub(crate) unsafe fn AirCutterProjectileStep2(taskId: u8) {
    if task_get(taskId, 1) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AirCutterProjectileStep1(taskId: u8) {
    if ({
        let t1 = task_get(taskId, 0);
        task_set(taskId, 0, task_get(taskId, 0) - 1);
        t1
    }) <= 0
    {
        let spriteId: u8 = CreateSprite(
            (&raw const *gAirWaveProjectileSpriteTemplate).cast_mut(),
            task_get(taskId, 9),
            task_get(taskId, 10),
            task_get(taskId, 2) as u8 - task_get(taskId, 1) as u8,
        );
        let sprite: *mut Sprite = &raw mut gSprites[spriteId];
        match task_get(taskId, 4) {
            1 => {
                (*sprite).oam.set_matrixNum((*sprite).oam.matrixNum() | 24);
            }
            2 => {
                (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
            }
            _ => {}
        }
        (*sprite).data[0] = task_get(taskId, 5) - task_get(taskId, 6);
        (*sprite).data[7] = taskId as i16;
        (*gTasks.as_ptr())[taskId].data[task_get(taskId, 1) as i32 + 13] = spriteId as i16;
        task_set(taskId, 0, task_get(taskId, 3));
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        PlaySE12WithPanning(SE_M_BLIZZARD2, BattleAnimAdjustPanning(-63));
        if task_get(taskId, 1) > 2 {
            task_set_func(taskId, Some(AirCutterProjectileStep2));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AirCutterProjectile(taskId: u8) {
    let mut targetX: i16 = 0;
    let mut targetY: i16 = 0;
    let mut xDiff: i16 = 0;
    if IsContest() != 0 {
        task_set(taskId, 4, 2);
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
        if gBattleAnimArgs[2] as i32 & 1 != 0 {
            gBattleAnimArgs[2] &= -2;
        } else {
            gBattleAnimArgs[2] |= 1;
        }
    } else {
        if gBattlerPositions[gBattleAnimTarget] as i32 & 1 == B_SIDE_PLAYER as i32 {
            task_set(taskId, 4, 1);
            gBattleAnimArgs[0] = -gBattleAnimArgs[0];
            gBattleAnimArgs[1] = -gBattleAnimArgs[1];
            if gBattleAnimArgs[2] as i32 & 1 != 0 {
                gBattleAnimArgs[2] &= -2;
            } else {
                gBattleAnimArgs[2] |= 1;
            }
        }
    }
    let attackerX: i16 = {
        task_set(
            taskId,
            9,
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16,
        );
        task_get(taskId, 9)
    };
    let attackerY: i16 = {
        task_set(
            taskId,
            10,
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16,
        );
        task_get(taskId, 10)
    };
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) != 0
    {
        SetAverageBattlerPositions(gBattleAnimTarget, FALSE, &raw mut targetX, &raw mut targetY);
    } else {
        targetX = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        targetY = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    }
    targetX = {
        task_set(taskId, 11, targetX + gBattleAnimArgs[0]);
        task_get(taskId, 11)
    };
    targetY = {
        task_set(taskId, 12, targetY + gBattleAnimArgs[1]);
        task_get(taskId, 12)
    };
    if targetX >= attackerX {
        xDiff = targetX - attackerX;
    } else {
        xDiff = attackerX - targetX;
    }
    task_set(
        taskId,
        5,
        MathUtil_Mul16(xDiff, MathUtil_Inv16(gBattleAnimArgs[2] & -2)),
    );
    task_set(
        taskId,
        6,
        MathUtil_Mul16(task_get(taskId, 5), (0_f32 * 256_f32) as i16),
    );
    task_set(taskId, 7, gBattleAnimArgs[2]);
    if targetY >= attackerY {
        task_set(
            taskId,
            8,
            MathUtil_Mul16(targetY - attackerY, MathUtil_Inv16(task_get(taskId, 5))) & -2,
        );
    } else {
        task_set(
            taskId,
            8,
            MathUtil_Mul16(attackerY - targetY, MathUtil_Inv16(task_get(taskId, 5))) | 1,
        );
    }
    task_set(taskId, 3, gBattleAnimArgs[3]);
    if gBattleAnimArgs[4] as i32 & 0x80 != 0 {
        gBattleAnimArgs[4] ^= 0x80;
        if gBattleAnimArgs[4] >= 64 {
            let var: u16 = GetBattlerSpriteSubpriority(gBattleAnimTarget) as u16
                + (gBattleAnimArgs[4] as u16 - 64);
            task_set(taskId, 2, var as i16);
        } else {
            let var: u16 =
                GetBattlerSpriteSubpriority(gBattleAnimTarget) as u16 - gBattleAnimArgs[4] as u16;
            task_set(taskId, 2, var as i16);
        }
    } else {
        if gBattleAnimArgs[4] >= 64 {
            let var: u16 = GetBattlerSpriteSubpriority(gBattleAnimTarget) as u16
                + (gBattleAnimArgs[4] as u16 - 64);
            task_set(taskId, 2, var as i16);
        } else {
            let var: u16 =
                GetBattlerSpriteSubpriority(gBattleAnimTarget) as u16 - gBattleAnimArgs[4] as u16;
            task_set(taskId, 2, var as i16);
        }
    }
    if task_get(taskId, 2) < 3 {
        task_set(taskId, 2, 3);
    }
    task_set_func(taskId, Some(AirCutterProjectileStep1));
}
pub(crate) unsafe fn AnimVoidLines(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, FALSE);
    (*sprite).data[0] =
        0x100 + IndexOfSpritePaletteTag(sVoidLinesSpriteTemplate.paletteTag) as i16 * 16;
    (*sprite).callback = Some(AnimVoidLines_Step);
}
pub(crate) unsafe fn AnimVoidLines_Step(sprite: *mut Sprite) {
    let mut id: u16 = 0;
    let mut val: u16 = 0;
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) == 2
    {
        (*sprite).data[1] = 0;
        id = (*sprite).data[0] as u16;
        val = gPlttBufferFaded[8 + id as i32];
        for i in 8..16i32 {
            gPlttBufferFaded[i + id as i32] = gPlttBufferFaded[i + id as i32 + 1];
        }
        gPlttBufferFaded[id as i32 + 15] = val;
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) == 24
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimCoinThrow(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    let mut r6: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    let r7: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[3];
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    r6 += gBattleAnimArgs[2];
    let mut var: u16 = ArcTan2Neg(r6 - (*sprite).x, r7 - (*sprite).y);
    var -= 0x4000;
    TrySetSpriteRotScale(sprite, FALSE, 0x100, 0x100, var);
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] = r6;
    (*sprite).data[4] = r7;
    (*sprite).callback = Some(InitAnimLinearTranslationWithSpeedAndPos);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimFallingCoin(sprite: *mut Sprite) {
    (*sprite).data[2] = -16;
    (*sprite).y += 8;
    (*sprite).callback = Some(AnimFallingCoin_Step);
}
pub(crate) unsafe fn AnimFallingCoin_Step(sprite: *mut Sprite) {
    (*sprite).data[0] += 0x80;
    (*sprite).x2 = (*sprite).data[0] >> 8;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).x2 = -(*sprite).x2;
    }
    (*sprite).y2 = Sin((*sprite).data[1], (*sprite).data[2]);
    (*sprite).data[1] += 5;
    if (*sprite).data[1] > 126 {
        (*sprite).data[1] = 0;
        (*sprite).data[2] /= 2;
        if ({
            (*sprite).data[3] += 1;
            (*sprite).data[3]
        }) == 2
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimBulletSeed(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = 20;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    (*sprite).set_affineAnimPaused(1);
    StoreSpriteCallbackInData6(sprite, Some(AnimBulletSeed_Step1));
}
pub(crate) unsafe fn AnimBulletSeed_Step1(sprite: *mut Sprite) {
    PlaySE12WithPanning(SE_M_HORN_ATTACK, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    let ptr: *mut i16 = &raw mut (*sprite).data[7];
    for i in 0..8i32 {
        *ptr.at(i - 7) = 0;
    }
    let mut rand: u16 = Random2();
    (*sprite).data[6] = -12 - (rand as i16 & 7);
    rand = Random2();
    (*sprite).data[7] = (rand as i32 % 160) as i16 + 0xA0;
    (*sprite).callback = Some(AnimBulletSeed_Step2);
    (*sprite).set_affineAnimPaused(0);
}
pub(crate) unsafe fn AnimBulletSeed_Step2(sprite: *mut Sprite) {
    (*sprite).data[0] += (*sprite).data[7];
    (*sprite).x2 = (*sprite).data[0] >> 8;
    if (*sprite).data[7] as i32 & 1 != 0 {
        (*sprite).x2 = -(*sprite).x2;
    }
    (*sprite).y2 = Sin((*sprite).data[1], (*sprite).data[6]);
    (*sprite).data[1] += 8;
    if (*sprite).data[1] > 126 {
        (*sprite).data[1] = 0;
        (*sprite).data[2] /= 2;
        if ({
            (*sprite).data[3] += 1;
            (*sprite).data[3]
        }) == 1
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimRazorWindTornado(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, FALSE);
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).y += 16;
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = gBattleAnimArgs[2];
    (*sprite).data[2] = gBattleAnimArgs[5];
    (*sprite).data[3] = gBattleAnimArgs[6];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).callback = Some(TranslateSpriteInCircle);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimViceGripPincer(sprite: *mut Sprite) {
    let mut startXOffset: i16 = 32;
    let mut startYOffset: i16 = -32;
    let mut endXOffset: i16 = 16;
    let mut endYOffset: i16 = -16;
    if gBattleAnimArgs[0] != 0 {
        startXOffset = -32;
        startYOffset = 32;
        endXOffset = -16;
        endYOffset = 16;
        StartSpriteAnim(sprite, 1);
    }
    (*sprite).x += startXOffset;
    (*sprite).y += startYOffset;
    (*sprite).data[0] = 6;
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + endXOffset;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + endYOffset;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(AnimViceGripPincer_Step));
}
pub(crate) unsafe fn AnimViceGripPincer_Step(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimGuillotinePincer(sprite: *mut Sprite) {
    let mut startXOffset: i16 = 32;
    let mut startYOffset: i16 = -32;
    let mut endXOffset: i16 = 16;
    let mut endYOffset: i16 = -16;
    if gBattleAnimArgs[0] != 0 {
        startXOffset = -32;
        startYOffset = 32;
        endXOffset = -16;
        endYOffset = 16;
        StartSpriteAnim(sprite, gBattleAnimArgs[0] as u8);
    }
    (*sprite).x += startXOffset;
    (*sprite).y += startYOffset;
    (*sprite).data[0] = 6;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + endXOffset;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + endYOffset;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = gBattleAnimArgs[0];
    (*sprite).data[6] = (*sprite).data[0];
    (*sprite).callback = Some(AnimGuillotinePincer_Step1);
}
pub(crate) unsafe fn AnimGuillotinePincer_Step1(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 && (*sprite).animEnded() != 0 {
        SeekSpriteAnim(sprite, 0);
        (*sprite).set_animPaused(1);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).x2 = 2;
        (*sprite).y2 = -2;
        (*sprite).data[0] = (*sprite).data[6];
        (*sprite).data[1] ^= 1;
        (*sprite).data[2] ^= 1;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 0;
        (*sprite).callback = Some(AnimGuillotinePincer_Step2);
    }
}
pub(crate) unsafe fn AnimGuillotinePincer_Step2(sprite: *mut Sprite) {
    if (*sprite).data[3] != 0 {
        (*sprite).x2 = -(*sprite).x2;
        (*sprite).y2 = -(*sprite).y2;
    }
    (*sprite).data[3] ^= 1;
    if ({
        (*sprite).data[4] += 1;
        (*sprite).data[4]
    }) == 51
    {
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 0;
        (*sprite).set_animPaused(0);
        StartSpriteAnim(sprite, (*sprite).data[5] as u8 ^ 1);
        (*sprite).callback = Some(AnimGuillotinePincer_Step3);
    }
}
pub(crate) unsafe fn AnimGuillotinePincer_Step3(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GrowAndGrayscale(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
    PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_BLEND as u8);
    SetSpriteRotScale(spriteId, 0xD0, 0xD0, 0);
    SetGrayscaleOrOriginalPalette(gSprites[spriteId].oam.paletteNum() + 16, FALSE);
    task_set(taskId, 0, 80);
    task_set_func(taskId, Some(AnimTask_GrowAndGrayscale_Step));
}
pub(crate) unsafe fn AnimTask_GrowAndGrayscale_Step(taskId: u8) {
    if ({
        task_set(taskId, 0, task_get(taskId, 0) - 1);
        task_get(taskId, 0)
    }) == -1
    {
        let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
        ResetSpriteRotScale(spriteId);
        SetGrayscaleOrOriginalPalette(gSprites[spriteId].oam.paletteNum() + 16, TRUE);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_Minimize(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    (*task).data[0] = spriteId as i16;
    PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[4] = 0x100;
    (*task).data[5] = 0;
    (*task).data[6] = 0;
    (*task).data[7] = GetBattlerSpriteSubpriority(gBattleAnimAttacker) as i16;
    (*task).func = Some(AnimTask_Minimize_Step);
}
pub(crate) unsafe fn AnimTask_Minimize_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[1] {
        0 => {
            if (*task).data[2] == 0 || (*task).data[2] == 3 || (*task).data[2] == 6 {
                CreateMinimizeSprite(task, taskId);
            }
            (*task).data[2] += 1;
            (*task).data[4] += 0x28;
            SetSpriteRotScale((*task).data[0] as u8, (*task).data[4], (*task).data[4], 0);
            SetBattlerSpriteYOffsetFromYScale((*task).data[0] as u8);
            if (*task).data[2] == 32 {
                (*task).data[5] += 1;
                (*task).data[1] += 1;
            }
        }
        1 => {
            if (*task).data[6] == 0 {
                if (*task).data[5] == 3 {
                    (*task).data[2] = 0;
                    (*task).data[1] = 3;
                } else {
                    (*task).data[2] = 0;
                    (*task).data[3] = 0;
                    (*task).data[4] = 0x100;
                    SetSpriteRotScale((*task).data[0] as u8, (*task).data[4], (*task).data[4], 0);
                    SetBattlerSpriteYOffsetFromYScale((*task).data[0] as u8);
                    (*task).data[1] = 2;
                }
            }
        }
        2 => {
            (*task).data[1] = 0;
        }
        3 => {
            if ({
                (*task).data[2] += 1;
                (*task).data[2]
            }) > 32
            {
                (*task).data[2] = 0;
                (*task).data[1] += 1;
            }
        }
        4 => {
            (*task).data[2] += 2;
            (*task).data[4] -= 0x50;
            SetSpriteRotScale((*task).data[0] as u8, (*task).data[4], (*task).data[4], 0);
            SetBattlerSpriteYOffsetFromYScale((*task).data[0] as u8);
            if (*task).data[2] == 32 {
                (*task).data[2] = 0;
                (*task).data[1] += 1;
            }
        }
        5 => {
            ResetSpriteRotScale((*task).data[0] as u8);
            gSprites[(*task).data[15]].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn CreateMinimizeSprite(task: *mut Task, taskId: u8) {
    let mut matrixNum: u16 = 0;
    let spriteId: i16 = CloneBattlerSpriteWithBlend(ANIM_ATTACKER);
    if spriteId >= 0 {
        if ({
            matrixNum = AllocOamMatrix() as u16;
            matrixNum
        }) == 0xFF
        {
            DestroySpriteWithActiveSheet(&raw mut gSprites[spriteId]);
        } else {
            gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_DOUBLE);
            gSprites[spriteId].set_affineAnimPaused(TRUE);
            gSprites[spriteId].oam.set_matrixNum(matrixNum as u32);
            gSprites[spriteId].subpriority = (*task).data[7] as u8 - (*task).data[3] as u8;
            (*task).data[3] += 1;
            (*task).data[6] += 1;
            gSprites[spriteId].data[0] = 16;
            gSprites[spriteId].data[1] = taskId as i16;
            gSprites[spriteId].data[2] = 6;
            gSprites[spriteId].callback = Some(ClonedMinizeSprite_Step);
            SetSpriteRotScale(spriteId as u8, (*task).data[4], (*task).data[4], 0);
            gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
            CalcCenterToCornerVec(
                &raw mut gSprites[spriteId],
                gSprites[spriteId].oam.shape() as u8,
                gSprites[spriteId].oam.size() as u8,
                gSprites[spriteId].oam.affineMode() as u8,
            );
        }
    }
}
pub(crate) unsafe fn ClonedMinizeSprite_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == 0
    {
        task_set(
            (*sprite).data[1],
            (*sprite).data[2],
            task_get((*sprite).data[1], (*sprite).data[2]) - 1,
        );
        FreeOamMatrix((*sprite).oam.matrixNum() as u8);
        DestroySpriteWithActiveSheet(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_Splash(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[1] == 0 {
        DestroyAnimVisualTask(taskId);
    } else {
        let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
        (*task).data[0] = spriteId as i16;
        (*task).data[1] = 0;
        (*task).data[2] = gBattleAnimArgs[1];
        (*task).data[3] = 0;
        (*task).data[4] = 0;
        PrepareAffineAnimInTaskData(
            task,
            spriteId,
            gSplashEffectAffineAnimCmds.as_ptr().cast_mut(),
        );
        (*task).func = Some(AnimTask_Splash_Step);
    }
}
pub(crate) unsafe fn AnimTask_Splash_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[1] {
        0 => {
            RunAffineAnimFromTaskData(task);
            (*task).data[4] += 3;
            gSprites[(*task).data[0]].y2 += (*task).data[4];
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) > 7
            {
                (*task).data[3] = 0;
                (*task).data[1] += 1;
            }
        }
        1 => {
            RunAffineAnimFromTaskData(task);
            gSprites[(*task).data[0]].y2 += (*task).data[4];
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) > 7
            {
                (*task).data[3] = 0;
                (*task).data[1] += 1;
            }
        }
        2 => {
            if (*task).data[4] != 0 {
                gSprites[(*task).data[0]].y2 -= 2;
                (*task).data[4] -= 2;
            } else {
                (*task).data[1] += 1;
            }
        }
        3 if RunAffineAnimFromTaskData(task) == 0 => {
            if ({
                (*task).data[2] -= 1;
                (*task).data[2]
            }) == 0
            {
                gSprites[(*task).data[0]].y2 = 0;
                DestroyAnimVisualTask(taskId);
            } else {
                PrepareAffineAnimInTaskData(
                    task,
                    (*task).data[0] as u8,
                    gSplashEffectAffineAnimCmds.as_ptr().cast_mut(),
                );
                (*task).data[1] = 0;
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GrowAndShrink(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    PrepareAffineAnimInTaskData(
        task,
        spriteId,
        gGrowAndShrinkAffineAnimCmds.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_GrowAndShrink_Step);
}
pub(crate) unsafe fn AnimTask_GrowAndShrink_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if RunAffineAnimFromTaskData(task) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimBreathPuff(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        StartSpriteAnim(sprite, 0);
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + 32;
        (*sprite).data[1] = 64;
    } else {
        StartSpriteAnim(sprite, 1);
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 - 32;
        (*sprite).data[1] = -64;
    }
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = 52;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
}
pub(crate) unsafe fn AnimAngerMark(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        gBattleAnimArgs[1] *= -1;
    }
    (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[1];
    (*sprite).y =
        GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16 + gBattleAnimArgs[2];
    if (*sprite).y < 8 {
        (*sprite).y = 8;
    }
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ThrashMoveMonHorizontal(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    (*task).data[0] = spriteId as i16;
    (*task).data[1] = 0;
    PrepareAffineAnimInTaskData(
        task,
        spriteId,
        gThrashMoveMonAffineAnimCmds.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_ThrashMoveMonHorizontal_Step);
}
pub(crate) unsafe fn AnimTask_ThrashMoveMonHorizontal_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if RunAffineAnimFromTaskData(task) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ThrashMoveMonVertical(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = GetAnimBattlerSpriteId(0) as i16;
    (*task).data[1] = 0;
    (*task).data[2] = 4;
    (*task).data[3] = 7;
    (*task).data[4] = 3;
    (*task).data[5] = gSprites[(*task).data[0]].x;
    (*task).data[6] = gSprites[(*task).data[0]].y;
    (*task).data[7] = 0;
    (*task).data[8] = 0;
    (*task).data[9] = 2;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        (*task).data[2] *= -1;
    }
    (*task).func = Some(AnimTask_ThrashMoveMonVertical_Step);
}
pub(crate) unsafe fn AnimTask_ThrashMoveMonVertical_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if ({
        (*task).data[7] += 1;
        (*task).data[7]
    }) > 2
    {
        (*task).data[7] = 0;
        (*task).data[8] += 1;
        if (*task).data[8] as i32 & 1 != 0 {
            gSprites[(*task).data[0]].y += (*task).data[9];
        } else {
            gSprites[(*task).data[0]].y -= (*task).data[9];
        }
    }
    match (*task).data[1] {
        0 => {
            gSprites[(*task).data[0]].x += (*task).data[2];
            if ({
                (*task).data[3] -= 1;
                (*task).data[3]
            }) == 0
            {
                (*task).data[3] = 14;
                (*task).data[1] = 1;
            }
        }
        1 => {
            gSprites[(*task).data[0]].x -= (*task).data[2];
            if ({
                (*task).data[3] -= 1;
                (*task).data[3]
            }) == 0
            {
                (*task).data[3] = 7;
                (*task).data[1] = 2;
            }
        }
        2 => {
            gSprites[(*task).data[0]].x += (*task).data[2];
            if ({
                (*task).data[3] -= 1;
                (*task).data[3]
            }) == 0
            {
                if ({
                    (*task).data[4] -= 1;
                    (*task).data[4]
                }) != 0
                {
                    (*task).data[3] = 7;
                    (*task).data[1] = 0;
                } else {
                    if (*task).data[8] as i32 & 1 != 0 {
                        gSprites[(*task).data[0]].y -= (*task).data[9];
                    }
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SketchDrawMon(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let mut params: ScanlineEffectParams = zeroed();
    (*task).data[0] = GetBattlerYCoordWithElevation(gBattleAnimTarget) as i16 + 32;
    (*task).data[1] = 4;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[4] = 0;
    (*task).data[5] = 0;
    (*task).data[15] = GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_HEIGHT);
    if GetBattlerSpriteBGPriorityRank(gBattleAnimTarget) == 1 {
        (*task).data[6] = gBattle_BG1_X as i16;
        params.dmaDest = 67108884_usize as *mut u16 as *mut c_void;
    } else {
        (*task).data[6] = gBattle_BG2_X as i16;
        params.dmaDest = 67108888_usize as *mut u16 as *mut c_void;
    }
    let mut i: i16 = (*task).data[0] - 0x40;
    while i <= (*task).data[0] {
        if i >= 0 {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][i] = (*task).data[6] as u16 + 0xF0;
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1][i] = (*task).data[6] as u16 + 0xF0;
        }
        i += 1;
    }
    params.dmaControl = 0xa2600001;
    params.initState = 1;
    params.unused9 = 0;
    ScanlineEffect_SetParams(params);
    (*task).func = Some(AnimTask_SketchDrawMon_Step);
}
pub(crate) unsafe fn AnimTask_SketchDrawMon_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[4] {
        0 => {
            if ({
                (*task).data[5] += 1;
                (*task).data[5]
            }) > 20
            {
                (*task).data[4] += 1;
            }
        }
        1 if ({
            (*task).data[1] += 1;
            (*task).data[1]
        }) > 3 =>
        {
            (*task).data[1] = 0;
            (*task).data[2] = (*task).data[3] & 3;
            (*task).data[5] = (*task).data[0] - (*task).data[3];
            match (*task).data[2] {
                0 => {}
                1 => {
                    (*task).data[5] -= 2;
                }
                2 => {
                    (*task).data[5] += 1;
                }
                3 => {
                    (*task).data[5] += 1;
                }
                _ => {}
            }
            if (*task).data[5] >= 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0][(*task).data[5]] = (*task).data[6] as u16;
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[1][(*task).data[5]] = (*task).data[6] as u16;
            }
            if ({
                (*task).data[3] += 1;
                (*task).data[3]
            }) >= (*task).data[15]
            {
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimPencil(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 - 16;
    (*sprite).y = GetBattlerYCoordWithElevation(gBattleAnimTarget) as i16 + 16;
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 16;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_HEIGHT) + 2;
    (*sprite).data[6] = BattleAnimAdjustPanning(SOUND_PAN_TARGET) as i16;
    (*sprite).callback = Some(AnimPencil_Step);
}
pub(crate) unsafe fn AnimPencil_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) > 1
            {
                (*sprite).data[2] = 0;
                (*sprite).set_invisible(((*sprite).invisible() == 0) as u16);
            }
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 16
            {
                (*sprite).set_invisible(FALSE as u16);
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 3
                && (*sprite).data[2] < (*sprite).data[5]
            {
                (*sprite).data[1] = 0;
                (*sprite).y -= 1;
                (*sprite).data[2] += 1;
                if (*sprite).data[2] % 10 == 0 {
                    PlaySE12WithPanning(SE_M_SKETCH, (*sprite).data[6] as i8);
                }
            }
            (*sprite).data[4] += (*sprite).data[3];
            if (*sprite).data[4] > 31 {
                (*sprite).data[4] = 0x40 - (*sprite).data[4];
                (*sprite).data[3] *= -1;
            } else if (*sprite).data[4] <= -32 {
                (*sprite).data[4] = -64 - (*sprite).data[4];
                (*sprite).data[3] *= -1;
            }
            (*sprite).x2 = (*sprite).data[4];
            if (*sprite).data[5] == (*sprite).data[2] {
                (*sprite).data[1] = 0;
                (*sprite).data[2] = 0;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) > 1
            {
                (*sprite).data[2] = 0;
                (*sprite).set_invisible(((*sprite).invisible() == 0) as u16);
            }
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 16
            {
                (*sprite).set_invisible(FALSE as u16);
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimBlendThinRing(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if gBattleAnimArgs[2] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    let mut r4: u8 = gBattleAnimArgs[3] as u8 ^ 1;
    if IsDoubleBattle() != 0 && IsBattlerSpriteVisible(battler ^ 2) != 0 {
        SetAverageBattlerPositions(battler, r4, &raw mut x, &raw mut y);
        if r4 == 0 {
            r4 = GetBattlerSpriteCoord(battler, BATTLER_COORD_X);
        } else {
            r4 = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2);
        }
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            gBattleAnimArgs[0] -= x - r4 as i16 - gBattleAnimArgs[0];
        } else {
            gBattleAnimArgs[0] = x - r4 as i16;
        }
    }
    (*sprite).callback = Some(AnimSpriteOnMonPos);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimHyperVoiceRing_WaitEnd(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        FreeSpriteOamMatrix(sprite);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimHyperVoiceRing(sprite: *mut Sprite) {
    let mut startX: i16 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut yCoordType: u8 = 0;
    let mut battler1: u8 = 0;
    let mut battler2: u8 = 0;
    let mut xCoordType: u8 = 0;
    if gBattleAnimArgs[5] == 0 {
        battler1 = gBattleAnimAttacker;
        battler2 = gBattleAnimTarget;
    } else {
        battler1 = gBattleAnimTarget;
        battler2 = gBattleAnimAttacker;
    }
    if gBattleAnimArgs[6] == 0 {
        xCoordType = BATTLER_COORD_X;
        yCoordType = BATTLER_COORD_Y;
    } else {
        xCoordType = BATTLER_COORD_X_2;
        yCoordType = BATTLER_COORD_Y_PIC_OFFSET;
    }
    if GetBattlerSide(battler1) != B_SIDE_PLAYER {
        startX = GetBattlerSpriteCoord(battler1, xCoordType) as i16 + gBattleAnimArgs[0];
        if IsBattlerSpriteVisible(battler2 ^ 2) != 0 {
            (*sprite).subpriority =
                gSprites[gBattlerSpriteIds[battler2 as i32 ^ 2]].subpriority - 1;
        } else {
            (*sprite).subpriority = gSprites[gBattlerSpriteIds[battler2]].subpriority - 1;
        }
    } else {
        startX = GetBattlerSpriteCoord(battler1, xCoordType) as i16 - gBattleAnimArgs[0];
        if IsContest() == 0 && IsBattlerSpriteVisible(battler1 ^ 2) != 0 {
            if gSprites[gBattlerSpriteIds[battler1]].x
                < gSprites[gBattlerSpriteIds[battler1 as i32 ^ 2]].x
            {
                (*sprite).subpriority =
                    gSprites[gBattlerSpriteIds[battler1 as i32 ^ 2]].subpriority + 1;
            } else {
                (*sprite).subpriority = gSprites[gBattlerSpriteIds[battler1]].subpriority - 1;
            }
        } else {
            (*sprite).subpriority = gSprites[gBattlerSpriteIds[battler1]].subpriority - 1;
        }
    }
    let startY: i16 = GetBattlerSpriteCoord(battler1, yCoordType) as i16 + gBattleAnimArgs[1];
    if IsContest() == 0 && IsBattlerSpriteVisible(battler2 ^ 2) != 0 {
        SetAverageBattlerPositions(battler2, gBattleAnimArgs[6] as u8, &raw mut x, &raw mut y);
    } else {
        x = GetBattlerSpriteCoord(battler2, xCoordType) as i16;
        y = GetBattlerSpriteCoord(battler2, yCoordType) as i16;
    }
    if GetBattlerSide(battler2) != 0 {
        x += gBattleAnimArgs[3];
    } else {
        x -= gBattleAnimArgs[3];
    }
    y += gBattleAnimArgs[4];
    (*sprite).x = {
        (*sprite).data[1] = startX;
        (*sprite).data[1]
    };
    (*sprite).y = {
        (*sprite).data[3] = startY;
        (*sprite).data[3]
    };
    (*sprite).data[2] = x;
    (*sprite).data[4] = y;
    (*sprite).data[0] = gBattleAnimArgs[0];
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(AnimHyperVoiceRing_WaitEnd);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimUproarRing(sprite: *mut Sprite) {
    let index: u8 = IndexOfSpritePaletteTag(ANIM_TAG_THIN_RING);
    if index != 0xFF {
        BlendPalette(
            0x100 + index as u16 * 16 + 1,
            15,
            gBattleAnimArgs[5] as u8,
            gBattleAnimArgs[4] as u16,
        );
    }
    StartSpriteAffineAnim(sprite, 1);
    (*sprite).callback = Some(AnimSpriteOnMonPos);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimSoftBoiledEgg(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, FALSE);
    let r1: i16 = (if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        -160
    } else {
        160
    }) as i16;
    (*sprite).data[0] = 0x380;
    (*sprite).data[1] = r1;
    (*sprite).data[7] = gBattleAnimArgs[2];
    (*sprite).callback = Some(AnimSoftBoiledEgg_Step1);
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step1(sprite: *mut Sprite) {
    (*sprite).y2 -= (*sprite).data[0] >> 8;
    (*sprite).x2 = (*sprite).data[1] >> 8;
    (*sprite).data[0] -= 32;
    let add: i16 = (if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        -160
    } else {
        160
    }) as i16;
    (*sprite).data[1] += add;
    if (*sprite).y2 > 0 {
        (*sprite).y += (*sprite).y2;
        (*sprite).x += (*sprite).x2;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).data[0] = 0;
        StartSpriteAffineAnim(sprite, 1);
        (*sprite).callback = Some(AnimSoftBoiledEgg_Step2);
    }
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step2(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] += 1;
        t1
    }) > 19
    {
        StartSpriteAffineAnim(sprite, 2);
        (*sprite).callback = Some(AnimSoftBoiledEgg_Step3);
    }
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step3(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        StartSpriteAffineAnim(sprite, 1);
        (*sprite).data[0] = 0;
        if (*sprite).data[7] == 0 {
            (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 16);
            (*sprite).callback = Some(AnimSoftBoiledEgg_Step3_Callback1);
        } else {
            (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 32);
            (*sprite).callback = Some(AnimSoftBoiledEgg_Step4);
        }
    }
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step3_Callback1(sprite: *mut Sprite) {
    (*sprite).y2 -= 2;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 9
    {
        (*sprite).data[0] = 16;
        (*sprite).data[1] = 0;
        SetGpuReg(REG_OFFSET_BLDCNT, 16192);
        SetGpuReg(REG_OFFSET_BLDALPHA, (*sprite).data[0] as u16);
        (*sprite).callback = Some(AnimSoftBoiledEgg_Step3_Callback2);
    }
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step3_Callback2(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) % 3
        == 0
    {
        (*sprite).data[0] -= 1;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - (*sprite).data[0] as u16) << 8 | (*sprite).data[0] as u16,
        );
        if (*sprite).data[0] == 0 {
            (*sprite).callback = Some(AnimSoftBoiledEgg_Step4);
        }
    }
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step4(sprite: *mut Sprite) {
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        (*sprite).set_invisible(TRUE as u16);
        if (*sprite).data[7] == 0 {
            (*sprite).callback = Some(AnimSoftBoiledEgg_Step4_Callback);
        } else {
            (*sprite).callback = Some(DestroyAnimSprite);
        }
    }
}
pub(crate) unsafe fn AnimSoftBoiledEgg_Step4_Callback(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    DestroyAnimSprite(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AttackerStretchAndDisappear(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    (*task).data[0] = spriteId as i16;
    PrepareAffineAnimInTaskData(
        task,
        spriteId,
        gStretchAttackerAffineAnimCmds.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_AttackerStretchAndDisappear_Step);
}
pub(crate) unsafe fn AnimTask_AttackerStretchAndDisappear_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if RunAffineAnimFromTaskData(task) == 0 {
        gSprites[(*task).data[0]].y2 = 0;
        gSprites[(*task).data[0]].set_invisible(TRUE as u16);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ExtremeSpeedImpact(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[12] = 3;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*task).data[13] = -1;
        (*task).data[14] = 8;
    } else {
        (*task).data[13] = 1;
        (*task).data[14] = -8;
    }
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_TARGET) as i16;
    (*task).func = Some(AnimTask_ExtremeSpeedImpact_Step);
}
pub(crate) unsafe fn AnimTask_ExtremeSpeedImpact_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            gSprites[(*task).data[15]].x2 += (*task).data[14];
            (*task).data[1] = 0;
            (*task).data[2] = 0;
            (*task).data[3] = 0;
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                (*task).data[2] += 1;
                if (*task).data[2] as i32 & 1 != 0 {
                    gSprites[(*task).data[15]].x2 += 6;
                } else {
                    gSprites[(*task).data[15]].x2 -= 6;
                }
                if ({
                    (*task).data[3] += 1;
                    (*task).data[3]
                }) > 4
                {
                    if (*task).data[2] as i32 & 1 != 0 {
                        gSprites[(*task).data[15]].x2 -= 6;
                    }
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            if ({
                (*task).data[12] -= 1;
                (*task).data[12]
            }) != 0
            {
                (*task).data[0] = 0;
            } else {
                (*task).data[0] += 1;
            }
        }
        3 => {
            gSprites[(*task).data[15]].x2 += (*task).data[13];
            if gSprites[(*task).data[15]].x2 == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ExtremeSpeedMonReappear(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[4] = 1;
    (*task).data[13] = 14;
    (*task).data[14] = 2;
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).func = Some(AnimTask_ExtremeSpeedMonReappear_Step);
}
pub(crate) unsafe fn AnimTask_ExtremeSpeedMonReappear_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[0] == 0
        && ({
            (*task).data[1] += 1;
            (*task).data[1]
        }) > (*task).data[4]
    {
        (*task).data[1] = 0;
        if ({
            (*task).data[2] += 1;
            (*task).data[2]
        }) as i32
            & 1
            != 0
        {
            gSprites[(*task).data[15]].set_invisible(FALSE as u16);
        } else {
            gSprites[(*task).data[15]].set_invisible(TRUE as u16);
        }
        if ({
            (*task).data[3] += 1;
            (*task).data[3]
        }) >= (*task).data[13]
        {
            if ({
                (*task).data[4] += 1;
                (*task).data[4]
            }) < (*task).data[14]
            {
                (*task).data[1] = 0;
                (*task).data[2] = 0;
                (*task).data[3] = 0;
            } else {
                gSprites[(*task).data[15]].set_invisible(FALSE as u16);
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SpeedDust(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[1] = 4;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[4] = 0;
    (*task).data[5] = 0;
    (*task).data[6] = 0;
    (*task).data[7] = 0;
    (*task).data[8] = 0;
    (*task).data[13] = 0;
    (*task).data[14] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
    (*task).data[15] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    (*task).func = Some(AnimTask_SpeedDust_Step);
}
pub(crate) unsafe fn AnimTask_SpeedDust_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[8] {
        0 => {
            if ({
                (*task).data[4] += 1;
                (*task).data[4]
            }) > 1
            {
                (*task).data[4] = 0;
                (*task).data[5] = ((*task).data[5] + 1) & 1;
                if ({
                    (*task).data[6] += 1;
                    (*task).data[6]
                }) > 20
                {
                    if (*task).data[7] == 0 {
                        (*task).data[6] = 0;
                        (*task).data[8] = 1;
                    } else {
                        (*task).data[8] = 2;
                    }
                }
            }
        }
        1 => {
            (*task).data[5] = 0;
            if ({
                (*task).data[4] += 1;
                (*task).data[4]
            }) > 20
            {
                (*task).data[7] = 1;
                (*task).data[8] = 0;
            }
        }
        2 => {
            (*task).data[5] = 1;
        }
        _ => {}
    }
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 4
            {
                (*task).data[1] = 0;
                let spriteId: u8 = CreateSprite(
                    (&raw const *gSpeedDustSpriteTemplate).cast_mut(),
                    (*task).data[14],
                    (*task).data[15],
                    0,
                );
                if spriteId != MAX_SPRITES {
                    gSprites[spriteId].data[0] = taskId as i16;
                    gSprites[spriteId].data[1] = 13;
                    gSprites[spriteId].x2 = gSpeedDustPosTable[(*task).data[2]][0] as i16;
                    gSprites[spriteId].y2 = gSpeedDustPosTable[(*task).data[2]][1] as i16;
                    (*task).data[13] += 1;
                    if ({
                        (*task).data[2] += 1;
                        (*task).data[2]
                    }) > 3
                    {
                        (*task).data[2] = 0;
                        if ({
                            (*task).data[3] += 1;
                            (*task).data[3]
                        }) > 5
                        {
                            (*task).data[0] += 1;
                        }
                    }
                }
            }
        }
        1 if (*task).data[13] == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimSpeedDust(sprite: *mut Sprite) {
    (*sprite).set_invisible(task_get((*sprite).data[0], 5) as u16);
    if (*sprite).animEnded() != 0 {
        task_set(
            (*sprite).data[0],
            (*sprite).data[1],
            task_get((*sprite).data[0], (*sprite).data[1]) - 1,
        );
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_LoadMusicNotesPals(taskId: u8) {
    let mut paletteNums: CArray<u8, 3> = zeroed();
    paletteNums[0] = IndexOfSpritePaletteTag(ANIM_TAG_MUSIC_NOTES_2);
    let mut i: i32 = 1;
    while i < NUM_MUSIC_NOTE_PAL_TAGS {
        paletteNums[i] = AllocSpritePalette(ANIM_SPRITES_START - i as u16);
        i += 1;
    }
    (*gMonSpritesGfxPtr).buffer = AllocZeroed(8192) as *mut u16;
    LZDecompressWram(
        (*(&raw const crate::data::graphics::gBattleAnimSpritePal_MusicNotes2)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        (*gMonSpritesGfxPtr).buffer as *mut c_void,
    );
    for i in 0..NUM_MUSIC_NOTE_PAL_TAGS {
        LoadPalette(
            (*gMonSpritesGfxPtr).buffer.at(i * 32) as *mut c_void,
            0x100 + paletteNums[i] as u16 * 16,
            32,
        );
    }
    Free((*gMonSpritesGfxPtr).buffer as *mut c_void);
    (*gMonSpritesGfxPtr).buffer = null_mut();
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FreeMusicNotesPals(taskId: u8) {
    for i in 0..NUM_MUSIC_NOTE_PAL_TAGS {
        FreeSpritePaletteByTag(sMusicNotePaletteTagsTable[i]);
    }
    DestroyAnimVisualTask(taskId);
}
unsafe fn SetMusicNotePalette(sprite: *mut Sprite, a: u8, b: u8) {
    let tile: u8 = (if b as i32 & 1 != 0 { 32 } else { 0 }) as u8;
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + (tile as u16 + a as u16 * 4));
    (*sprite)
        .oam
        .set_paletteNum(IndexOfSpritePaletteTag(sMusicNotePaletteTagsTable[b >> 1]) as u16);
}
pub(crate) unsafe fn AnimHealBellMusicNote(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, FALSE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + gBattleAnimArgs[2];
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[3];
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    SetMusicNotePalette(sprite, gBattleAnimArgs[5] as u8, gBattleAnimArgs[6] as u8);
}
pub(crate) unsafe fn AnimMagentaHeart(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 1
    {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    }
    (*sprite).x2 = Sin((*sprite).data[1], 8);
    (*sprite).y2 = (*sprite).data[2] >> 8;
    (*sprite).data[1] = ((*sprite).data[1] + 7) & 0xFF;
    (*sprite).data[2] -= 0x80;
    if (*sprite).data[0] == 60 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FakeOut(taskId: u8) {
    let win0h: u16 = (if IsContest() != 0 {
        152
    } else {
        DISPLAY_WIDTH as i32
    }) as u16;
    let win0v: u16 = 0;
    gBattle_WIN0H = win0h;
    gBattle_WIN0V = DISPLAY_HEIGHT;
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    SetGpuReg(REG_OFFSET_WININ, 16159);
    SetGpuReg(REG_OFFSET_WINOUT, 16191);
    SetGpuReg(REG_OFFSET_BLDCNT, 200);
    SetGpuReg(REG_OFFSET_BLDY, 16);
    task_set(taskId, 0, win0v as i16);
    task_set(taskId, 1, win0h as i16);
    task_set_func(taskId, Some(AnimTask_FakeOut_Step1));
}
pub(crate) unsafe fn AnimTask_FakeOut_Step1(taskId: u8) {
    task_set(taskId, 0, task_get(taskId, 0) + 13);
    task_set(taskId, 1, task_get(taskId, 1) - 13);
    if task_get(taskId, 0) >= task_get(taskId, 1) {
        gBattle_WIN0H = 0;
        task_set_func(taskId, Some(AnimTask_FakeOut_Step2));
    } else {
        gBattle_WIN0H = (task_get(taskId, 0) as u16) << 8 | task_get(taskId, 1) as u16;
    }
}
pub(crate) unsafe fn AnimTask_FakeOut_Step2(taskId: u8) {
    if ({
        task_set(taskId, 10, task_get(taskId, 10) + 1);
        task_get(taskId, 10)
    }) == 5
    {
        task_set(taskId, 11, 0x88);
        SetGpuReg(REG_OFFSET_BLDCNT, 136);
        BlendPalettes(
            GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE),
            16,
            32767,
        );
    } else if task_get(taskId, 10) > 4 {
        gBattle_WIN0H = 0;
        gBattle_WIN0V = 0;
        SetGpuReg(REG_OFFSET_WININ, 16191);
        SetGpuReg(REG_OFFSET_WINOUT, 16191);
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDY, 0);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StretchTargetUp(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
    if ({
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_get(taskId, 0)
    }) == 1
    {
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            GetAnimBattlerSpriteId(ANIM_TARGET),
            sAffineAnims_StretchBattlerUp.as_ptr().cast_mut(),
        );
        gSprites[spriteId].x2 = 4;
    } else {
        gSprites[spriteId].x2 = -gSprites[spriteId].x2;
        if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
            gSprites[spriteId].x2 = 0;
            gSprites[spriteId].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StretchAttackerUp(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    if ({
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_get(taskId, 0)
    }) == 1
    {
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            GetAnimBattlerSpriteId(ANIM_ATTACKER),
            sAffineAnims_StretchBattlerUp.as_ptr().cast_mut(),
        );
        gSprites[spriteId].x2 = 4;
    } else {
        gSprites[spriteId].x2 = -gSprites[spriteId].x2;
        if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
            gSprites[spriteId].x2 = 0;
            gSprites[spriteId].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe fn AnimRedHeartProjectile(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = 95;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    (*sprite).callback = Some(AnimRedHeartProjectile_Step);
}
pub(crate) unsafe fn AnimRedHeartProjectile_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).y2 += Sin((*sprite).data[5], 14);
        (*sprite).data[5] = ((*sprite).data[5] + 4) & 0xFF;
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub unsafe fn AnimParticleBurst(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[1] = gBattleAnimArgs[0];
        (*sprite).data[2] = gBattleAnimArgs[1];
        (*sprite).data[0] += 1;
    } else {
        (*sprite).data[4] += (*sprite).data[1];
        (*sprite).x2 = (*sprite).data[4] >> 8;
        (*sprite).y2 = Sin((*sprite).data[3], (*sprite).data[2]);
        (*sprite).data[3] = ((*sprite).data[3] + 3) & 0xFF;
        if (*sprite).data[3] > 100 {
            (*sprite).set_invisible(((*sprite).data[3] % 2) as u16);
        }
        if (*sprite).data[3] > 120 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimRedHeartRising(sprite: *mut Sprite) {
    (*sprite).x = gBattleAnimArgs[0];
    (*sprite).y = DISPLAY_HEIGHT as i16;
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gBattleAnimArgs[1];
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(AnimRedHeartRising_Step));
}
pub(crate) unsafe fn AnimRedHeartRising_Step(sprite: *mut Sprite) {
    let mut y: i16 = 0;
    (*sprite).data[2] += (*sprite).data[1];
    (*sprite).y2 = -(((*sprite).data[2] as u16 >> 8) as i16);
    (*sprite).x2 = Sin((*sprite).data[3], 4);
    (*sprite).data[3] = ((*sprite).data[3] + 3) & 0xFF;
    y = (*sprite).y + (*sprite).y2;
    if y <= 72 {
        (*sprite).set_invisible(((*sprite).data[3] % 2) as u16);
        if y <= 64 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_HeartsBackground(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 3);
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
        (*(&raw const crate::data::graphics::gBattleAnimBgImage_Attract).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBg.tilesOffset as u32,
    );
    AnimLoadCompressedBgTilemapHandleContest(
        &raw mut animBg,
        (*(&raw const crate::data::graphics::gBattleAnimBgTilemap_Attract).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
        FALSE as u32,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleAnimBgPalette_Attract)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        animBg.paletteId as u16 * 16,
        32,
    );
    task_set_func(taskId, Some(AnimTask_HeartsBackground_Step));
}
pub(crate) unsafe fn AnimTask_HeartsBackground_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    match task_get(taskId, 12) {
        0 => {
            if ({
                task_set(taskId, 10, task_get(taskId, 10) + 1);
                task_get(taskId, 10)
            }) == 4
            {
                task_set(taskId, 10, 0);
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 11) as u16) << 8 | task_get(taskId, 11) as u16,
                );
                if task_get(taskId, 11) == 16 {
                    task_set(taskId, 12, task_get(taskId, 12) + 1);
                    task_set(taskId, 11, 0);
                }
            }
        }
        1 => {
            if ({
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                task_get(taskId, 11)
            }) == 141
            {
                task_set(taskId, 11, 16);
                task_set(taskId, 12, task_get(taskId, 12) + 1);
            }
        }
        2 => {
            if ({
                task_set(taskId, 10, task_get(taskId, 10) + 1);
                task_get(taskId, 10)
            }) == 4
            {
                task_set(taskId, 10, 0);
                task_set(taskId, 11, task_get(taskId, 11) - 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 11) as u16) << 8 | task_get(taskId, 11) as u16,
                );
                if task_get(taskId, 11) == 0 {
                    task_set(taskId, 12, task_get(taskId, 12) + 1);
                    task_set(taskId, 11, 0);
                }
            }
        }
        3 => {
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(animBg.bgId as u32);
            task_set(taskId, 12, task_get(taskId, 12) + 1);
        }
        4 => {
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ScaryFace(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
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
    if IsContest() != 0 {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBg,
            (&raw const (*(&raw const crate::data::graphics::gBattleAnimBgTilemap_ScaryFaceContest).cast::<CArray<u32, 0>>())).cast_mut() as *mut c_void,
            FALSE as u32,
        );
    } else if GetBattlerSide(gBattleAnimTarget) == B_SIDE_OPPONENT {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBg,
            (&raw const (*(&raw const crate::data::graphics::gBattleAnimBgTilemap_ScaryFacePlayer)
                .cast::<CArray<u32, 0>>()))
                .cast_mut() as *mut c_void,
            FALSE as u32,
        );
    } else {
        AnimLoadCompressedBgTilemapHandleContest(
            &raw mut animBg,
            (&raw const (*(&raw const crate::data::graphics::gBattleAnimBgTilemap_ScaryFaceOpponent).cast::<CArray<u32, 0>>())).cast_mut() as *mut c_void,
            FALSE as u32,
        );
    }
    AnimLoadCompressedBgGfx(
        animBg.bgId as u32,
        (*(&raw const crate::data::graphics::gBattleAnimBgImage_ScaryFace)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        animBg.tilesOffset as u32,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gBattleAnimBgPalette_ScaryFace)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut(),
        animBg.paletteId as u16 * 16,
        32,
    );
    task_set_func(taskId, Some(AnimTask_ScaryFace_Step));
}
pub(crate) unsafe fn AnimTask_ScaryFace_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    'l1: {
        let sw1: i16 = task_get(taskId, 12);
        let mut fall = false;
        if sw1 == 0 {
            if ({
                task_set(taskId, 10, task_get(taskId, 10) + 1);
                task_get(taskId, 10)
            }) == 2
            {
                task_set(taskId, 10, 0);
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 11) as u16) << 8 | task_get(taskId, 11) as u16,
                );
                if task_get(taskId, 11) == 14 {
                    task_set(taskId, 12, task_get(taskId, 12) + 1);
                    task_set(taskId, 11, 0);
                }
            }
            break 'l1;
        }
        if sw1 == 1 {
            if ({
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                task_get(taskId, 11)
            }) == 21
            {
                task_set(taskId, 11, 14);
                task_set(taskId, 12, task_get(taskId, 12) + 1);
            }
            break 'l1;
        }
        if sw1 == 2 {
            if ({
                task_set(taskId, 10, task_get(taskId, 10) + 1);
                task_get(taskId, 10)
            }) == 2
            {
                task_set(taskId, 10, 0);
                task_set(taskId, 11, task_get(taskId, 11) - 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 11) as u16) << 8 | task_get(taskId, 11) as u16,
                );
                if task_get(taskId, 11) == 0 {
                    task_set(taskId, 12, task_get(taskId, 12) + 1);
                    task_set(taskId, 11, 0);
                }
            }
            break 'l1;
        }
        if sw1 == 3 {
            fall = true;
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(1);
            ClearBattleAnimBg(2);
            task_set(taskId, 12, task_get(taskId, 12) + 1);
        }
        if fall || sw1 == 4 {
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            DestroyAnimVisualTask(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe fn AnimOrbitFast(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).set_affineAnimPaused(1);
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = gBattleAnimArgs[1];
    (*sprite).data[7] = GetBattlerSpriteSubpriority(gBattleAnimAttacker) as i16;
    (*sprite).callback = Some(AnimOrbitFast_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimOrbitFast_Step(sprite: *mut Sprite) {
    if (*sprite).data[1] >= 64 && (*sprite).data[1] <= 191 {
        (*sprite).subpriority = (*sprite).data[7] as u8 + 1;
    } else {
        (*sprite).subpriority = (*sprite).data[7] as u8 - 1;
    }
    (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[2] >> 8);
    (*sprite).y2 = Cos((*sprite).data[1], (*sprite).data[3] >> 8);
    (*sprite).data[1] = ((*sprite).data[1] + 9) & 0xFF;
    match (*sprite).data[5] {
        1 => {
            (*sprite).data[2] -= 0x400;
            (*sprite).data[3] -= 0x100;
            if ({
                (*sprite).data[4] += 1;
                (*sprite).data[4]
            }) == (*sprite).data[0]
            {
                (*sprite).data[5] = 2;
                return;
            }
        }
        0 => {
            (*sprite).data[2] += 0x400;
            (*sprite).data[3] += 0x100;
            if ({
                (*sprite).data[4] += 1;
                (*sprite).data[4]
            }) == (*sprite).data[0]
            {
                (*sprite).data[4] = 0;
                (*sprite).data[5] = 1;
            }
        }
        _ => {}
    }
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimOrbitScatter(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = Sin(gBattleAnimArgs[0], 10);
    (*sprite).data[1] = Cos(gBattleAnimArgs[0], 7);
    (*sprite).callback = Some(AnimOrbitScatter_Step);
}
pub(crate) unsafe fn AnimOrbitScatter_Step(sprite: *mut Sprite) {
    (*sprite).x2 += (*sprite).data[0];
    (*sprite).y2 += (*sprite).data[1];
    if (*sprite).x as i32 + (*sprite).x2 as i32 > 256
        || ((*sprite).x as i32 + (*sprite).x2 as i32) < -16
        || (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
        || ((*sprite).y as i32 + (*sprite).y2 as i32) < -16
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSpitUpOrb_Step(sprite: *mut Sprite) {
    (*sprite).x2 += (*sprite).data[0];
    (*sprite).y2 += (*sprite).data[1];
    if ({
        let t1 = (*sprite).data[3];
        (*sprite).data[3] += 1;
        t1
    }) >= (*sprite).data[2]
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSpitUpOrb(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = Sin(gBattleAnimArgs[0], 10);
    (*sprite).data[1] = Cos(gBattleAnimArgs[0], 7);
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).callback = Some(AnimSpitUpOrb_Step);
}
pub(crate) unsafe fn AnimEyeSparkle_Step(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimEyeSparkle(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).callback = Some(AnimEyeSparkle_Step);
}
pub(crate) unsafe fn AnimAngel(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
    }
    (*sprite).data[0] += 1;
    let var0: i16 = ((*sprite).data[0] * 10) & 0xFF;
    (*sprite).x2 = Sin(var0, 80) >> 8;
    if (*sprite).data[0] < 80 {
        (*sprite).y2 = (*sprite).data[0] / 2 + (Cos(var0, 80) >> 8);
    }
    if (*sprite).data[0] > 90 {
        (*sprite).data[2] += 1;
        (*sprite).x2 -= (*sprite).data[2] / 2;
    }
    if (*sprite).data[0] > 100 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimPinkHeart_Step(sprite: *mut Sprite) {
    (*sprite).data[5] += 1;
    (*sprite).x2 = Sin((*sprite).data[3], 5);
    (*sprite).y2 = (*sprite).data[5] / 2;
    (*sprite).data[3] = ((*sprite).data[3] + 3) & 0xFF;
    if (*sprite).data[5] > 20 {
        (*sprite).set_invisible(((*sprite).data[5] % 2) as u16);
    }
    if (*sprite).data[5] > 30 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimPinkHeart(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[1] = gBattleAnimArgs[0];
        (*sprite).data[2] = gBattleAnimArgs[1];
        (*sprite).data[0] += 1;
    } else {
        (*sprite).data[4] += (*sprite).data[1];
        (*sprite).x2 = (*sprite).data[4] >> 8;
        (*sprite).y2 = Sin((*sprite).data[3], (*sprite).data[2]);
        (*sprite).data[3] = ((*sprite).data[3] + 3) & 0xFF;
        if (*sprite).data[3] > 70 {
            (*sprite).callback = Some(AnimPinkHeart_Step);
            (*sprite).x += (*sprite).x2;
            (*sprite).y += (*sprite).y2;
            (*sprite).x2 = 0;
            (*sprite).y2 = 0;
            (*sprite).data[3] = (Random2() as i32 % 180) as i16;
        }
    }
}
pub(crate) unsafe fn AnimDevil(sprite: *mut Sprite) {
    if (*sprite).data[3] == 0 {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        StartSpriteAnim(sprite, 0);
        (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) - 1;
        (*sprite).data[2] = 1;
    }
    (*sprite).data[0] += (*sprite).data[2];
    (*sprite).data[1] = ((*sprite).data[0] as i32 * 4 % 256) as i16;
    if (*sprite).data[1] < 0 {
        (*sprite).data[1] = 0;
    }
    (*sprite).x2 = Cos((*sprite).data[1], 30 - (*sprite).data[0] / 4);
    (*sprite).y2 = Sin((*sprite).data[1], 10 - (*sprite).data[0] / 8);
    if (*sprite).data[1] > 128 && (*sprite).data[2] > 0 {
        (*sprite).data[2] = -1;
    }
    if (*sprite).data[1] == 0 && (*sprite).data[2] < 0 {
        (*sprite).data[2] = 1;
    }
    (*sprite).data[3] += 1;
    if (*sprite).data[3] < 10 || (*sprite).data[3] > 80 {
        (*sprite).set_invisible(((*sprite).data[0] % 2) as u16);
    } else {
        (*sprite).set_invisible(FALSE as u16);
    }
    if (*sprite).data[3] > 90 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimFurySwipes(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        StartSpriteAnim(sprite, gBattleAnimArgs[2] as u8);
        (*sprite).data[0] += 1;
    } else if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimMovementWaves(sprite: *mut Sprite) {
    if gBattleAnimArgs[2] == 0 {
        DestroyAnimSprite(sprite);
    } else {
        if gBattleAnimArgs[0] == 0 {
            (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
            (*sprite).y =
                GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        } else {
            (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
            (*sprite).y =
                GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        }
        if gBattleAnimArgs[1] == 0 {
            (*sprite).x += 32;
        } else {
            (*sprite).x -= 32;
        }
        (*sprite).data[0] = gBattleAnimArgs[2];
        (*sprite).data[1] = gBattleAnimArgs[1];
        StartSpriteAnim(sprite, (*sprite).data[1] as u8);
        (*sprite).callback = Some(AnimMovementWaves_Step);
    }
}
pub(crate) unsafe fn AnimMovementWaves_Step(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        if ({
            (*sprite).data[0] -= 1;
            (*sprite).data[0]
        }) != 0
        {
            StartSpriteAnim(sprite, (*sprite).data[1] as u8);
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_UproarDistortion(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    PrepareAffineAnimInTaskData(
        &raw mut (*gTasks.as_ptr())[taskId],
        spriteId,
        sAffineAnims_UproarDistortion.as_ptr().cast_mut(),
    );
    task_set_func(taskId, Some(AnimTask_UproarDistortion_Step));
}
pub(crate) unsafe fn AnimTask_UproarDistortion_Step(taskId: u8) {
    if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimJaggedMusicNote(sprite: *mut Sprite) {
    let battler: u8 = if gBattleAnimArgs[0] == 0 {
        gBattleAnimAttacker
    } else {
        gBattleAnimTarget
    };
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        gBattleAnimArgs[1] *= -1;
    }
    (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[1];
    (*sprite).y =
        GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16 + gBattleAnimArgs[2];
    (*sprite).data[0] = 0;
    (*sprite).data[1] = (*sprite).x << 3;
    (*sprite).data[2] = (*sprite).y << 3;
    (*sprite).data[3] = (((gBattleAnimArgs[1] as i32) << 3) / 8) as i16;
    (*sprite).data[4] = (((gBattleAnimArgs[2] as i32) << 3) / 8) as i16;
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[3] as u16 * 16);
    (*sprite).callback = Some(AnimJaggedMusicNote_Step);
}
pub(crate) unsafe fn AnimJaggedMusicNote_Step(sprite: *mut Sprite) {
    (*sprite).data[1] += (*sprite).data[3];
    (*sprite).data[2] += (*sprite).data[4];
    (*sprite).x = (*sprite).data[1] >> 3;
    (*sprite).y = (*sprite).data[2] >> 3;
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 16
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimPerishSongMusicNote2(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[1] = 120 - gBattleAnimArgs[0];
        (*sprite).set_invisible(TRUE as u16);
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == (*sprite).data[1]
    {
        SetGrayscaleOrOriginalPalette((*sprite).oam.paletteNum() + 16, FALSE);
    }
    if (*sprite).data[0] as i32 == (*sprite).data[1] as i32 + 80 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimPerishSongMusicNote(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).x = 120;
        (*sprite).y = gBattleAnimArgs[0] / 2 - 15;
        StartSpriteAnim(sprite, gBattleAnimArgs[1] as u8);
        (*sprite).data[5] = 120;
        (*sprite).data[3] = gBattleAnimArgs[2];
    }
    (*sprite).data[0] += 1;
    (*sprite).data[1] = (*sprite).data[0] / 2;
    let index: u16 = ((*sprite).data[0] as u16 * 3 + (*sprite).data[3] as u16) & 0xFF;
    (*sprite).data[6] += 10;
    (*sprite).data[6] &= 0xFF;
    (*sprite).x2 = Cos(index as i16, 100);
    (*sprite).y2 = (*sprite).data[1] + Sin(index as i16, 10) + Cos((*sprite).data[6], 4);
    if (*sprite).data[0] > (*sprite).data[5] {
        (*sprite).callback = Some(AnimPerishSongMusicNote_Step1);
        (*sprite).data[0] = 0;
        SetSpritePrimaryCoordsFromSecondaryCoords(sprite);
        (*sprite).data[2] = 5;
        (*sprite).data[4] = 0;
        (*sprite).data[3] = 0;
        StartSpriteAffineAnim(sprite, 1);
    }
}
pub(crate) unsafe fn AnimPerishSongMusicNote_Step1(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 10
    {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(AnimPerishSongMusicNote_Step2);
    }
}
pub(crate) unsafe fn AnimPerishSongMusicNote_Step2(sprite: *mut Sprite) {
    (*sprite).data[3] += (*sprite).data[2];
    (*sprite).y2 = (*sprite).data[3];
    (*sprite).data[2] += 1;
    if (*sprite).data[3] > 48 && (*sprite).data[2] > 0 {
        (*sprite).data[2] = (*sprite).data[4] - 5;
        (*sprite).data[4] += 1;
    }
    if (*sprite).data[4] > 3 {
        (*sprite).set_invisible(((*sprite).data[2] % 2) as u16);
        DestroyAnimSprite(sprite);
    }
    if (*sprite).data[4] == 4 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimGuardRing(sprite: *mut Sprite) {
    if gBattleTypeFlags & BATTLE_TYPE_DOUBLE != 0
        && IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) != 0
    {
        SetAverageBattlerPositions(
            gBattleAnimAttacker,
            FALSE,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
        (*sprite).y += 40;
        StartSpriteAffineAnim(sprite, 1);
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + 40;
    }
    (*sprite).data[0] = 13;
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[4] = (*sprite).y - 72;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsFuryCutterHitRight(taskId: u8) {
    gBattleAnimArgs[7] = (*gAnimDisableStructPtr).furyCutterCounter as i16 & 1;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetFuryCutterHitCount(taskId: u8) {
    gBattleAnimArgs[7] = (*gAnimDisableStructPtr).furyCutterCounter as i16;
    DestroyAnimVisualTask(taskId);
}
