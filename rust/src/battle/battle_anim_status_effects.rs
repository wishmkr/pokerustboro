//! Translated from `src/battle_anim_status_effects.c` by tools/rustport/c2rs.py.
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
    dead_code,
    unused_assignments
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimVisualTask, IsContest, LaunchBattleAnimation, gAnimScriptActive,
    gAnimScriptCallback, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::GetBattlerSpriteCoord;
use crate::battle_anim_utility_funcs::InitStatsChangeAnimation;
use crate::battle_main::gBattleSpritesDataPtr;
use crate::battle_main::gBattlerSpriteIds;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::palette::gPlttBufferFaded;
use crate::sprite::gSprites;
use crate::sprite::{GetSpriteTileStartByTag, IndexOfSpritePaletteTag};
use crate::task::DestroyTask;
use crate::task::{task_func, task_get, task_set, task_set_func};
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
/// `CreateTask` with this module's view of its types.
#[inline]
unsafe fn CreateTask(a0: Option<unsafe fn(u8)>, a1: u8) -> u8 {
    unsafe { crate::task::CreateTask(core::mem::transmute(a0), a1) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `DestroySpriteAndFreeResources` with this module's view of its types.
#[inline]
unsafe fn DestroySpriteAndFreeResources(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySpriteAndFreeResources(a0 as _);
    }
}
/// `FreeSpriteOamMatrix` with this module's view of its types.
#[inline]
unsafe fn FreeSpriteOamMatrix(a0: *mut Sprite) {
    unsafe {
        crate::sprite::FreeSpriteOamMatrix(a0 as _);
    }
}
/// `LoadCompressedSpritePaletteUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpritePaletteUsingHeap(a0: *mut CompressedSpritePalette) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpritePaletteUsingHeap(a0 as _) }
}
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `SetSubspriteTables` with this module's view of its types.
#[inline]
unsafe fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable) {
    unsafe {
        crate::sprite::SetSubspriteTables(a0 as _, a1 as _);
    }
}
// Data tables (translate with cdata.py): sAnim_FlickeringOrb sAnims_FlickeringOrb sFlickeringOrbSpriteTemplate sFlickeringOrbFlippedSpriteTemplate sAnim_WeatherBallNormal sAnims_WeatherBallNormal gWeatherBallUpSpriteTemplate gWeatherBallNormalDownSpriteTemplate sAnim_SpinningSparkle sAnims_SpinningSparkle gSpinningSparkleSpriteTemplate sFlickeringFootSpriteTemplate sAnim_FlickeringImpact_0 sAnim_FlickeringImpact_1 sAnim_FlickeringImpact_2 sAnims_FlickeringImpact sFlickeringImpactSpriteTemplate sAnim_FlickeringShrinkOrb sAnims_FlickeringShrinkOrb sAffineAnim_FlickeringShrinkOrb sAffineAnims_FlickeringShrinkOrb sFlickeringShrinkOrbSpriteTemplate sFrozenIceCubeSubsprites sFrozenIceCubeSubspriteTable sFrozenIceCubeSpriteTemplate sFlashingCircleImpactSpriteTemplate

static sFlashingCircleImpactSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_anim_status_effects::sFlashingCircleImpactSpriteTemplate)
        .cast(),
);
static sFrozenIceCubeSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_anim_status_effects::sFrozenIceCubeSpriteTemplate).cast(),
);
static sFrozenIceCubeSubspriteTable: Table<CArray<SubspriteTable, 1>> = Table(
    (&raw const crate::data::battle_anim_status_effects::sFrozenIceCubeSubspriteTable).cast(),
);

unsafe fn Task_FlashingCircleImpacts(battler: u8, red: u8) -> u8 {
    let battlerSpriteId: u8 = gBattlerSpriteIds[battler];
    let taskId: u8 = CreateTask(Some(Task_UpdateFlashingCircleImpacts), 10);
    let mut spriteId: u8 = 0;
    LoadCompressedSpriteSheetUsingHeap(
        (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[136])
            .cast_mut(),
    );
    LoadCompressedSpritePaletteUsingHeap(
        (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[136])
            .cast_mut(),
    );
    task_set(taskId, 0, battler as i16);
    if red != 0 {
        task_set(taskId, 1, 31);
        for i in 0..10u8 {
            spriteId = CreateSprite(
                (&raw const *sFlashingCircleImpactSpriteTemplate).cast_mut(),
                gSprites[battlerSpriteId].x,
                gSprites[battlerSpriteId].y + 32,
                0,
            );
            gSprites[spriteId].data[0] = i as i16 * 51;
            gSprites[spriteId].data[1] = -256;
            gSprites[spriteId].set_invisible(TRUE as u16);
            if i > 4 {
                gSprites[spriteId].data[6] = 21;
            }
        }
    } else {
        task_set(taskId, 1, 31744);
        for i in 0..10u8 {
            spriteId = CreateSprite(
                (&raw const *sFlashingCircleImpactSpriteTemplate).cast_mut(),
                gSprites[battlerSpriteId].x,
                gSprites[battlerSpriteId].y - 32,
                0,
            );
            gSprites[spriteId].data[0] = i as i16 * 51;
            gSprites[spriteId].data[1] = 256;
            gSprites[spriteId].set_invisible(TRUE as u16);
            if i > 4 {
                gSprites[spriteId].data[6] = 21;
            }
        }
    }
    gSprites[spriteId].data[7] = 1;
    taskId
}
pub(crate) unsafe fn Task_UpdateFlashingCircleImpacts(taskId: u8) {
    if task_get(taskId, 2) == 2 {
        task_set(taskId, 2, 0);
        BlendPalette(
            0x100 + task_get(taskId, 0) as u16 * 16,
            16,
            task_get(taskId, 4) as u8,
            task_get(taskId, 1) as u16,
        );
        if task_get(taskId, 5) == 0 {
            task_set(taskId, 4, task_get(taskId, 4) + 1);
            if task_get(taskId, 4) > 8 {
                task_set(taskId, 5, task_get(taskId, 5) ^ 1);
            }
        } else {
            let var: u16 = task_get(taskId, 4) as u16;
            task_set(taskId, 4, task_get(taskId, 4) - 1);
            if task_get(taskId, 4) < 0 {
                task_set(taskId, 4, var as i16);
                task_set(taskId, 5, task_get(taskId, 5) ^ 1);
                task_set(taskId, 3, task_get(taskId, 3) + 1);
                if task_get(taskId, 3) == 2 {
                    DestroyTask(taskId);
                }
            }
        }
    } else {
        task_set(taskId, 2, task_get(taskId, 2) + 1);
    }
}
pub(crate) unsafe fn AnimFlashingCircleImpact(sprite: *mut Sprite) {
    if (*sprite).data[6] == 0 {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).callback = Some(AnimFlashingCircleImpact_Step);
        AnimFlashingCircleImpact_Step(sprite);
    } else {
        (*sprite).data[6] -= 1;
    }
}
pub(crate) unsafe fn AnimFlashingCircleImpact_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Cos((*sprite).data[0], 32);
    (*sprite).y2 = Sin((*sprite).data[0], 8);
    if (*sprite).data[0] < 128 {
        (*sprite).subpriority = 29;
    } else {
        (*sprite).subpriority = 31;
    }
    (*sprite).data[0] = ((*sprite).data[0] + 8) & 0xFF;
    (*sprite).data[5] += (*sprite).data[1];
    (*sprite).y2 += (*sprite).data[5] >> 8;
    (*sprite).data[2] += 1;
    if (*sprite).data[2] == 52 {
        if (*sprite).data[7] != 0 {
            DestroySpriteAndFreeResources(sprite);
        } else {
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FrozenIceCube(taskId: u8) {
    let mut x: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 - 32;
    let y: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 - 36;
    if IsContest() != 0 {
        x -= 6;
    }
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    let spriteId: u8 = CreateSprite(
        (&raw const *sFrozenIceCubeSpriteTemplate).cast_mut(),
        x,
        y,
        4,
    );
    if GetSpriteTileStartByTag(ANIM_TAG_ICE_CUBE) == 0xFFFF {
        gSprites[spriteId].set_invisible(TRUE as u16);
    }
    SetSubspriteTables(
        &raw mut gSprites[spriteId],
        sFrozenIceCubeSubspriteTable.as_ptr().cast_mut(),
    );
    task_set(taskId, 15, spriteId as i16);
    task_set_func(taskId, Some(AnimTask_FrozenIceCube_Step1));
}
pub(crate) unsafe fn AnimTask_FrozenIceCube_Step1(taskId: u8) {
    task_set(taskId, 1, task_get(taskId, 1) + 1);
    if task_get(taskId, 1) == 10 {
        task_set_func(taskId, Some(AnimTask_FrozenIceCube_Step2));
        task_set(taskId, 1, 0);
    } else {
        let var: u8 = task_get(taskId, 1) as u8;
        SetGpuReg(REG_OFFSET_BLDALPHA, (16 - var as u16) << 8 | var as u16);
    }
}
pub(crate) unsafe fn AnimTask_FrozenIceCube_Step2(taskId: u8) {
    let palIndex: u8 = IndexOfSpritePaletteTag(ANIM_TAG_ICE_CUBE);
    if ({
        let t1 = task_get(taskId, 1);
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        t1
    }) > 13
    {
        task_set(taskId, 2, task_get(taskId, 2) + 1);
        if task_get(taskId, 2) == 3 {
            let temp: u16 = gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 13];
            gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 13] =
                gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 14];
            gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 14] =
                gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 15];
            gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 15] = temp;
            task_set(taskId, 2, 0);
            task_set(taskId, 3, task_get(taskId, 3) + 1);
            if task_get(taskId, 3) == 3 {
                task_set(taskId, 3, 0);
                task_set(taskId, 1, 0);
                task_set(taskId, 4, task_get(taskId, 4) + 1);
                if task_get(taskId, 4) == 2 {
                    task_set(taskId, 1, 9);
                    task_set_func(taskId, Some(AnimTask_FrozenIceCube_Step3));
                }
            }
        }
    }
}
pub(crate) unsafe fn AnimTask_FrozenIceCube_Step3(taskId: u8) {
    task_set(taskId, 1, task_get(taskId, 1) - 1);
    if task_get(taskId, 1) == -1 {
        task_set_func(taskId, Some(AnimTask_FrozenIceCube_Step4));
        task_set(taskId, 1, 0);
    } else {
        let var: u8 = task_get(taskId, 1) as u8;
        SetGpuReg(REG_OFFSET_BLDALPHA, (16 - var as u16) << 8 | var as u16);
    }
}
pub(crate) unsafe fn AnimTask_FrozenIceCube_Step4(taskId: u8) {
    task_set(taskId, 1, task_get(taskId, 1) + 1);
    if task_get(taskId, 1) == 37 {
        let spriteId: u8 = task_get(taskId, 15) as u8;
        FreeSpriteOamMatrix(&raw mut gSprites[spriteId]);
        DestroySprite(&raw mut gSprites[spriteId]);
    } else if task_get(taskId, 1) == 39 {
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StatsChange(taskId: u8) {
    let mut goesDown: u16 = FALSE as u16;
    let mut animStatId: i16 = 0;
    let mut sharply: u16 = FALSE as u16;
    match (*(*gBattleSpritesDataPtr).animationData).animArg {
        15 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_ATK;
        }
        16 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_DEF;
        }
        17 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_SPEED;
        }
        18 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_SPATK;
        }
        19 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_SPDEF;
        }
        20 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_ACC;
        }
        21 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_EVASION;
        }
        22 => {
            goesDown = 1;
            animStatId = STAT_ANIM_PAL_ATK;
        }
        23 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_DEF;
        }
        24 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_SPEED;
        }
        25 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_SPATK;
        }
        26 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_SPDEF;
        }
        27 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_ACC;
        }
        28 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_EVASION;
        }
        39 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_ATK;
            sharply = 1;
        }
        40 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_DEF;
            sharply = TRUE as u16;
        }
        41 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_SPEED;
            sharply = TRUE as u16;
        }
        42 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_SPATK;
            sharply = TRUE as u16;
        }
        43 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_SPDEF;
            sharply = TRUE as u16;
        }
        44 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_ACC;
            sharply = TRUE as u16;
        }
        45 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_EVASION;
            sharply = TRUE as u16;
        }
        46 => {
            goesDown = 1;
            animStatId = STAT_ANIM_PAL_ATK;
            sharply = 1;
        }
        47 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_DEF;
            sharply = TRUE as u16;
        }
        48 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_SPEED;
            sharply = TRUE as u16;
        }
        49 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_SPATK;
            sharply = TRUE as u16;
        }
        50 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_SPDEF;
            sharply = TRUE as u16;
        }
        51 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_ACC;
            sharply = TRUE as u16;
        }
        52 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_EVASION;
            sharply = TRUE as u16;
        }
        STAT_ANIM_MULTIPLE_PLUS1 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_MULTIPLE;
            sharply = FALSE as u16;
        }
        STAT_ANIM_MULTIPLE_PLUS2 => {
            goesDown = FALSE as u16;
            animStatId = STAT_ANIM_PAL_MULTIPLE;
            sharply = TRUE as u16;
        }
        STAT_ANIM_MULTIPLE_MINUS1 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_MULTIPLE;
            sharply = FALSE as u16;
        }
        STAT_ANIM_MULTIPLE_MINUS2 => {
            goesDown = TRUE as u16;
            animStatId = STAT_ANIM_PAL_MULTIPLE;
            sharply = TRUE as u16;
        }
        _ => {
            DestroyAnimVisualTask(taskId);
            return;
        }
    }
    gBattleAnimArgs[0] = goesDown as i16;
    gBattleAnimArgs[1] = animStatId;
    gBattleAnimArgs[2] = FALSE as i16;
    gBattleAnimArgs[3] = FALSE as i16;
    gBattleAnimArgs[4] = sharply as i16;
    task_set_func(taskId, Some(InitStatsChangeAnimation));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub unsafe fn LaunchStatusAnimation(battler: u8, statusAnimId: u8) {
    gBattleAnimAttacker = battler;
    gBattleAnimTarget = battler;
    LaunchBattleAnimation(
        (*crate::asmdata::gBattleAnims_StatusConditions.cast::<CArray<*mut u8, 0>>())
            .as_ptr()
            .cast_mut(),
        statusAnimId as u16,
        FALSE,
    );
    let taskId: u8 = CreateTask(Some(Task_DoStatusAnimation), 10);
    task_set(taskId, 0, battler as i16);
}
pub(crate) unsafe fn Task_DoStatusAnimation(taskId: u8) {
    gAnimScriptCallback.unwrap_unchecked()();
    if gAnimScriptActive == 0 {
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(task_get(taskId, 0)))
        .set_statusAnimActive(0);
        DestroyTask(taskId);
    }
}
