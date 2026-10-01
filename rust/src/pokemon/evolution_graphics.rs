//! Translated from `src/evolution_graphics.c` by tools/rustport/c2rs.py.
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
    unused_variables
)]

#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::palette::{BeginNormalPaletteFade, gPaletteFade};
use crate::random::Random;
use crate::sound::PlaySE;
use crate::sprite::SetOamMatrix;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
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
/// `LoadCompressedSpriteSheetUsingHeap` with this module's view of its types.
#[inline]
unsafe fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8 {
    unsafe { crate::decompress::LoadCompressedSpriteSheetUsingHeap(a0 as _) }
}
/// `LoadSpritePalettes` with this module's view of its types.
#[inline]
unsafe fn LoadSpritePalettes(a0: *mut SpritePalette) {
    unsafe {
        crate::sprite::LoadSpritePalettes(a0 as _);
    }
}
// The C's names for task and sprite data slots.
const tPalNum: usize = 1;
const tPreEvoSpriteId: usize = 1;
const tPostEvoSpriteId: usize = 2;
const tSpecies: usize = 2;
const sSpeed: usize = 3;
const tPreEvoScale: usize = 3;
const tPostEvoScale: usize = 4;
const sAmplitude: usize = 5;
const tShowingPostEvo: usize = 5;
const sTrigIdx: usize = 6;
const tScaleSpeed: usize = 6;
const sTimer: usize = 7;
const tEvoStopped: usize = 8;
const tTimer: usize = 15;
// Data tables (translate with cdata.py): sEvoSparkle_Pal sEvoSparkle_Gfx sEvoSparkleSpriteSheets sEvoSparkleSpritePals sOamData_EvoSparkle sSpriteAnim_EvoSparkle sSpriteAnimTable_EvoSparkle sEvoSparkleSpriteTemplate sEvoSparkleMatrices sUnused

const MATRIX_POST_EVO: u8 = 31;
const MATRIX_PRE_EVO: u8 = 30;
const MON_MAX_SCALE: i16 = 256;
const MON_MIN_SCALE: i16 = 16;

static sEvoSparkleMatrices: Table<CArray<u16, 12>> =
    Table((&raw const crate::data::evolution_graphics::sEvoSparkleMatrices).cast());
static sEvoSparkleSpritePals: Table<CArray<SpritePalette, 2>> =
    Table((&raw const crate::data::evolution_graphics::sEvoSparkleSpritePals).cast());
static sEvoSparkleSpriteSheets: Table<CArray<CompressedSpriteSheet, 2>> =
    Table((&raw const crate::data::evolution_graphics::sEvoSparkleSpriteSheets).cast());
static sEvoSparkleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::evolution_graphics::sEvoSparkleSpriteTemplate).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub(crate) fn SpriteCB_Sparkle_Dummy(sprite: *mut Sprite) {}
unsafe fn SetEvoSparklesMatrices() {
    for i in 0..12u16 {
        SetOamMatrix(
            20 + i as u8,
            sEvoSparkleMatrices[i],
            0,
            0,
            sEvoSparkleMatrices[i],
        );
    }
}
pub(crate) unsafe fn SpriteCB_Sparkle_SpiralUpward(sprite: *mut Sprite) {
    if (*sprite).y > 8 {
        (*sprite).y =
            88 - ((*sprite).data[sTimer] as i32 * (*sprite).data[sTimer] as i32 / 80) as i16;
        (*sprite).y2 = Sin(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        ) / 4;
        (*sprite).x2 = Cos(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        );
        (*sprite).data[sTrigIdx] += 4;
        if (*sprite).data[sTimer] as i32 & 1 != 0 {
            (*sprite).data[sAmplitude] -= 1;
        }
        (*sprite).data[sTimer] += 1;
        if (*sprite).y2 > 0 {
            (*sprite).subpriority = 1;
        } else {
            (*sprite).subpriority = 20;
        }
        let mut matrixNum: u8 = ((*sprite).data[sAmplitude] / 4) as u8 + 20;
        if matrixNum > 31 {
            matrixNum = 31;
        }
        (*sprite).oam.set_matrixNum(matrixNum as u32);
    } else {
        DestroySprite(sprite);
    }
}
unsafe fn CreateSparkle_SpiralUpward(trigIdx: u8) {
    let spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        88,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[sAmplitude] = 48;
        gSprites[spriteId].data[sTrigIdx] = trigIdx as i16;
        gSprites[spriteId].data[sTimer] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(31);
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_SpiralUpward);
    }
}
pub(crate) unsafe fn SpriteCB_Sparkle_ArcDown(sprite: *mut Sprite) {
    if (*sprite).y < 88 {
        (*sprite).y =
            8 + ((*sprite).data[sTimer] as i32 * (*sprite).data[sTimer] as i32 / 5) as i16;
        (*sprite).y2 = Sin(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        ) / 4;
        (*sprite).x2 = Cos(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        );
        (*sprite).data[sAmplitude] = 8 + Sin((*sprite).data[sTimer] as u8 as i16 * 4, 40);
        (*sprite).data[sTimer] += 1;
    } else {
        DestroySprite(sprite);
    }
}
unsafe fn CreateSparkle_ArcDown(trigIdx: u8) {
    let spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        8,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[sAmplitude] = 8;
        gSprites[spriteId].data[sTrigIdx] = trigIdx as i16;
        gSprites[spriteId].data[sTimer] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(25);
        gSprites[spriteId].subpriority = 1;
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_ArcDown);
    }
}
pub(crate) unsafe fn SpriteCB_Sparkle_CircleInward(sprite: *mut Sprite) {
    if (*sprite).data[sAmplitude] > 8 {
        (*sprite).y2 = Sin(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        );
        (*sprite).x2 = Cos(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        );
        (*sprite).data[sAmplitude] -= (*sprite).data[sSpeed];
        (*sprite).data[sTrigIdx] += 4;
    } else {
        DestroySprite(sprite);
    }
}
unsafe fn CreateSparkle_CircleInward(trigIdx: u8, speed: u8) {
    let spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        56,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[sSpeed] = speed as i16;
        gSprites[spriteId].data[sAmplitude] = 120;
        gSprites[spriteId].data[sTrigIdx] = trigIdx as i16;
        gSprites[spriteId].data[sTimer] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(31);
        gSprites[spriteId].subpriority = 1;
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_CircleInward);
    }
}
pub(crate) unsafe fn SpriteCB_Sparkle_Spray(sprite: *mut Sprite) {
    if (*sprite).data[sTimer] as i32 & 3 == 0 {
        (*sprite).y += 1;
    }
    if (*sprite).data[sTrigIdx] < 128 {
        (*sprite).y2 = -Sin(
            (*sprite).data[sTrigIdx] as u8 as i16,
            (*sprite).data[sAmplitude],
        );
        (*sprite).x =
            120 + ((*sprite).data[sSpeed] as i32 * (*sprite).data[sTimer] as i32 / 3) as i16;
        (*sprite).data[sTrigIdx] += 1;
        let mut matrixNum: u8 = 31 - ((*sprite).data[sTrigIdx] as i32 * 12 / 128) as u8;
        if (*sprite).data[sTrigIdx] > 64 {
            (*sprite).subpriority = 1;
        } else {
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).subpriority = 20;
            if (*sprite).data[sTrigIdx] > 112 && (*sprite).data[sTrigIdx] as i32 & 1 != 0 {
                (*sprite).set_invisible(TRUE as u16);
            }
        }
        if matrixNum < 20 {
            matrixNum = 20;
        }
        (*sprite).oam.set_matrixNum(matrixNum as u32);
        (*sprite).data[sTimer] += 1;
    } else {
        DestroySprite(sprite);
    }
}
unsafe fn CreateSparkle_Spray(id: u8) {
    let spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        56,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[sSpeed] = 3 - (Random() as i32 % 7) as i16;
        gSprites[spriteId].data[sAmplitude] = 48 + (Random() as i16 & 0x3F);
        gSprites[spriteId].data[sTimer] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(31);
        gSprites[spriteId].subpriority = 20;
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_Spray);
    }
}
pub unsafe fn LoadEvoSparkleSpriteAndPal() {
    LoadCompressedSpriteSheetUsingHeap((&raw const sEvoSparkleSpriteSheets[0]).cast_mut());
    LoadSpritePalettes(sEvoSparkleSpritePals.as_ptr().cast_mut());
}
pub unsafe fn EvolutionSparkles_SpiralUpward(palNum: u16) -> u8 {
    let taskId: u8 = CreateTask(Some(Task_Sparkles_SpiralUpward_Init), 0);
    task_set(taskId, tPalNum, palNum as i16);
    taskId
}
pub(crate) unsafe fn Task_Sparkles_SpiralUpward_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    task_set(taskId, tTimer, 0);
    BeginNormalPaletteFade(
        shl_i32(3, task_get(taskId, tPalNum) as u32) as u32,
        0xA,
        0,
        0x10,
        32767,
    );
    task_set_func(taskId, Some(Task_Sparkles_SpiralUpward));
    PlaySE(SE_M_MEGA_KICK);
}
pub(crate) unsafe fn Task_Sparkles_SpiralUpward(taskId: u8) {
    if task_get(taskId, tTimer) < 64 {
        if task_get(taskId, tTimer) as i32 & 7 == 0 {
            for i in 0..4u8 {
                CreateSparkle_SpiralUpward((task_get(taskId, tTimer) as u8 & 120) * 2 + i * 64);
            }
        }
        task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
    } else {
        task_set(taskId, tTimer, 96);
        task_set_func(taskId, Some(Task_Sparkles_SpiralUpward_End));
    }
}
pub(crate) fn Task_Sparkles_SpiralUpward_End(taskId: u8) {
    if task_get(taskId, tTimer) != 0 {
        task_set(taskId, tTimer, task_get(taskId, tTimer) - 1);
    } else {
        DestroyTask(taskId);
    }
}
pub unsafe fn EvolutionSparkles_ArcDown() -> u8 {
    CreateTask(Some(Task_Sparkles_ArcDown_Init), 0)
}
pub(crate) unsafe fn Task_Sparkles_ArcDown_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    task_set(taskId, tTimer, 0);
    task_set_func(taskId, Some(Task_Sparkles_ArcDown));
    PlaySE(SE_M_BUBBLE_BEAM2);
}
pub(crate) unsafe fn Task_Sparkles_ArcDown(taskId: u8) {
    if task_get(taskId, tTimer) < 96 {
        if task_get(taskId, tTimer) < 6 {
            for i in 0..9u8 {
                CreateSparkle_ArcDown(i * 16);
            }
        }
        task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
    } else {
        task_set_func(taskId, Some(Task_Sparkles_ArcDown_End));
    }
}
pub(crate) fn Task_Sparkles_ArcDown_End(taskId: u8) {
    DestroyTask(taskId);
}
pub unsafe fn EvolutionSparkles_CircleInward() -> u8 {
    CreateTask(Some(Task_Sparkles_CircleInward_Init), 0)
}
pub(crate) unsafe fn Task_Sparkles_CircleInward_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    task_set(taskId, tTimer, 0);
    task_set_func(taskId, Some(Task_Sparkles_CircleInward));
    PlaySE(SE_SHINY);
}
pub(crate) unsafe fn Task_Sparkles_CircleInward(taskId: u8) {
    if task_get(taskId, tTimer) < 48 {
        if task_get(taskId, tTimer) == 0 {
            for i in 0..16u8 {
                CreateSparkle_CircleInward(i * 16, 4);
            }
        }
        if task_get(taskId, tTimer) == 32 {
            for i in 0..16u8 {
                CreateSparkle_CircleInward(i * 16, 8);
            }
        }
        task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
    } else {
        task_set_func(taskId, Some(Task_Sparkles_CircleInward_End));
    }
}
pub(crate) fn Task_Sparkles_CircleInward_End(taskId: u8) {
    DestroyTask(taskId);
}
pub unsafe fn EvolutionSparkles_SprayAndFlash(species: u16) -> u8 {
    let taskId: u8 = CreateTask(Some(Task_Sparkles_SprayAndFlash_Init), 0);
    task_set(taskId, tSpecies, species as i16);
    taskId
}
pub(crate) unsafe fn Task_Sparkles_SprayAndFlash_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    task_set(taskId, tTimer, 0);
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[32] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[32] as *mut c_void,
        48,
    );
    BeginNormalPaletteFade(0xFFF9041C, 0, 0, 0x10, 32767);
    task_set_func(taskId, Some(Task_Sparkles_SprayAndFlash));
    PlaySE(SE_M_PETAL_DANCE);
}
pub(crate) unsafe fn Task_Sparkles_SprayAndFlash(taskId: u8) {
    if task_get(taskId, tTimer) < 128 {
        match task_get(taskId, tTimer) {
            0 => {
                for i in 0..8u8 {
                    CreateSparkle_Spray(i);
                }
            }
            32 => {
                BeginNormalPaletteFade(0xFFFF041C, 0x10, 0x10, 0, 32767);
            }
            _ => {
                if task_get(taskId, tTimer) < 50 {
                    CreateSparkle_Spray(Random() as u8 & 7);
                }
            }
        }
        task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
    } else {
        task_set_func(taskId, Some(Task_Sparkles_SprayAndFlash_End));
    }
}
pub(crate) unsafe fn Task_Sparkles_SprayAndFlash_End(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
    }
}
pub unsafe fn EvolutionSparkles_SprayAndFlash_Trade(species: u16) -> u8 {
    let taskId: u8 = CreateTask(Some(Task_Sparkles_SprayAndFlashTrade_Init), 0);
    task_set(taskId, tSpecies, species as i16);
    taskId
}
pub(crate) unsafe fn Task_Sparkles_SprayAndFlashTrade_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    task_set(taskId, tTimer, 0);
    CpuSet(
        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[32] as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[32] as *mut c_void,
        48,
    );
    BeginNormalPaletteFade(0xFFF90400, 0, 0, 0x10, 32767);
    task_set_func(taskId, Some(Task_Sparkles_SprayAndFlashTrade));
    PlaySE(SE_M_PETAL_DANCE);
}
pub(crate) unsafe fn Task_Sparkles_SprayAndFlashTrade(taskId: u8) {
    if task_get(taskId, tTimer) < 128 {
        match task_get(taskId, tTimer) {
            0 => {
                for i in 0..8u8 {
                    CreateSparkle_Spray(i);
                }
            }
            32 => {
                BeginNormalPaletteFade(0xFFFF0400, 0x10, 0x10, 0, 32767);
            }
            _ => {
                if task_get(taskId, tTimer) < 50 {
                    CreateSparkle_Spray(Random() as u8 & 7);
                }
            }
        }
        task_set(taskId, tTimer, task_get(taskId, tTimer) + 1);
    } else {
        task_set_func(taskId, Some(Task_Sparkles_SprayAndFlash_End));
    }
}
pub(crate) unsafe fn SpriteCB_EvolutionMonSprite(sprite: *mut Sprite) {}
pub unsafe fn CycleEvolutionMonSprite(preEvoSpriteId: u8, postEvoSpriteId: u8) -> u8 {
    let mut monPalette: CArray<u16, 16> = zeroed();
    for i in 0..16u16 {
        monPalette[i] = 32767;
    }
    let taskId: u8 = CreateTask(Some(Task_CycleEvolutionMonSprite_Init), 0);
    task_set(taskId, tPreEvoSpriteId, preEvoSpriteId as i16);
    task_set(taskId, tPostEvoSpriteId, postEvoSpriteId as i16);
    task_set(taskId, tPreEvoScale, MON_MAX_SCALE);
    task_set(taskId, tPostEvoScale, MON_MIN_SCALE);
    let toDiv: i32 = 0x10000;
    SetOamMatrix(
        MATRIX_PRE_EVO,
        MON_MAX_SCALE as u16,
        0,
        0,
        MON_MAX_SCALE as u16,
    );
    SetOamMatrix(
        MATRIX_POST_EVO,
        div_i32(toDiv, task_get(taskId, tPostEvoScale) as i32) as u16,
        0,
        0,
        div_i32(toDiv, task_get(taskId, tPostEvoScale) as i32) as u16,
    );
    gSprites[preEvoSpriteId].callback = Some(SpriteCB_EvolutionMonSprite);
    gSprites[preEvoSpriteId]
        .oam
        .set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[preEvoSpriteId]
        .oam
        .set_matrixNum(MATRIX_PRE_EVO as u32);
    gSprites[preEvoSpriteId].set_invisible(FALSE as u16);
    CpuSet(
        monPalette.as_mut_ptr() as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[0x100 + gSprites[preEvoSpriteId].oam.paletteNum() as i32 * 16]
            as *mut c_void,
        16,
    );
    gSprites[postEvoSpriteId].callback = Some(SpriteCB_EvolutionMonSprite);
    gSprites[postEvoSpriteId]
        .oam
        .set_affineMode(ST_OAM_AFFINE_NORMAL);
    gSprites[postEvoSpriteId]
        .oam
        .set_matrixNum(MATRIX_POST_EVO as u32);
    gSprites[postEvoSpriteId].set_invisible(FALSE as u16);
    CpuSet(
        monPalette.as_mut_ptr() as *mut c_void,
        &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[0x100 + gSprites[postEvoSpriteId].oam.paletteNum() as i32 * 16]
            as *mut c_void,
        16,
    );
    task_set(taskId, tEvoStopped, FALSE as i16);
    taskId
}
pub(crate) unsafe fn Task_CycleEvolutionMonSprite_Init(taskId: u8) {
    task_set(taskId, tShowingPostEvo, FALSE as i16);
    task_set(taskId, tScaleSpeed, 8);
    task_set_func(taskId, Some(Task_CycleEvolutionMonSprite_TryEnd));
}
pub(crate) unsafe fn Task_CycleEvolutionMonSprite_TryEnd(taskId: u8) {
    if task_get(taskId, tEvoStopped) != 0 {
        EndOnPreEvoMon(taskId);
    } else if task_get(taskId, tScaleSpeed) == 128 {
        EndOnPostEvoMon(taskId);
    } else {
        task_set(taskId, tScaleSpeed, task_get(taskId, tScaleSpeed) + 2);
        task_set(
            taskId,
            tShowingPostEvo,
            task_get(taskId, tShowingPostEvo) ^ 1,
        );
        task_set_func(taskId, Some(Task_CycleEvolutionMonSprite_UpdateSize));
    }
}
pub(crate) unsafe fn Task_CycleEvolutionMonSprite_UpdateSize(taskId: u8) {
    if task_get(taskId, tEvoStopped) != 0 {
        task_set_func(taskId, Some(EndOnPreEvoMon));
    } else {
        let mut numSpritesFinished: u8 = 0;
        if task_get(taskId, tShowingPostEvo) == 0 {
            if (task_get(taskId, tPreEvoScale) as i32)
                < MON_MAX_SCALE as i32 - task_get(taskId, tScaleSpeed) as i32
            {
                task_set(
                    taskId,
                    tPreEvoScale,
                    task_get(taskId, tPreEvoScale) + (task_get(taskId, tScaleSpeed)),
                );
            } else {
                task_set(taskId, tPreEvoScale, MON_MAX_SCALE);
                numSpritesFinished += 1;
            }
            if task_get(taskId, tPostEvoScale) as i32
                > MON_MIN_SCALE as i32 + task_get(taskId, tScaleSpeed) as i32
            {
                task_set(
                    taskId,
                    tPostEvoScale,
                    task_get(taskId, tPostEvoScale) - (task_get(taskId, tScaleSpeed)),
                );
            } else {
                task_set(taskId, tPostEvoScale, MON_MIN_SCALE);
                numSpritesFinished += 1;
            }
        } else {
            if (task_get(taskId, tPostEvoScale) as i32)
                < MON_MAX_SCALE as i32 - task_get(taskId, tScaleSpeed) as i32
            {
                task_set(
                    taskId,
                    tPostEvoScale,
                    task_get(taskId, tPostEvoScale) + (task_get(taskId, tScaleSpeed)),
                );
            } else {
                task_set(taskId, tPostEvoScale, MON_MAX_SCALE);
                numSpritesFinished += 1;
            }
            if task_get(taskId, tPreEvoScale) as i32
                > MON_MIN_SCALE as i32 + task_get(taskId, tScaleSpeed) as i32
            {
                task_set(
                    taskId,
                    tPreEvoScale,
                    task_get(taskId, tPreEvoScale) - (task_get(taskId, tScaleSpeed)),
                );
            } else {
                task_set(taskId, tPreEvoScale, MON_MIN_SCALE);
                numSpritesFinished += 1;
            }
        }
        let mut oamMatrixArg: u16 = div_i32(0x10000, task_get(taskId, tPreEvoScale) as i32) as u16;
        SetOamMatrix(MATRIX_PRE_EVO, oamMatrixArg, 0, 0, oamMatrixArg);
        oamMatrixArg = div_i32(0x10000, task_get(taskId, tPostEvoScale) as i32) as u16;
        SetOamMatrix(MATRIX_POST_EVO, oamMatrixArg, 0, 0, oamMatrixArg);
        if numSpritesFinished == 2 {
            task_set_func(taskId, Some(Task_CycleEvolutionMonSprite_TryEnd));
        }
    }
}
unsafe fn EndOnPostEvoMon(taskId: u8) {
    gSprites[task_get(taskId, tPreEvoSpriteId)]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[task_get(taskId, tPreEvoSpriteId)]
        .oam
        .set_matrixNum(0);
    gSprites[task_get(taskId, tPreEvoSpriteId)].set_invisible(TRUE as u16);
    gSprites[task_get(taskId, tPostEvoSpriteId)]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[task_get(taskId, tPostEvoSpriteId)]
        .oam
        .set_matrixNum(0);
    gSprites[task_get(taskId, tPostEvoSpriteId)].set_invisible(FALSE as u16);
    DestroyTask(taskId);
}
pub(crate) unsafe fn EndOnPreEvoMon(taskId: u8) {
    gSprites[task_get(taskId, tPreEvoSpriteId)]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[task_get(taskId, tPreEvoSpriteId)]
        .oam
        .set_matrixNum(0);
    gSprites[task_get(taskId, tPreEvoSpriteId)].set_invisible(FALSE as u16);
    gSprites[task_get(taskId, tPostEvoSpriteId)]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[task_get(taskId, tPostEvoSpriteId)]
        .oam
        .set_matrixNum(0);
    gSprites[task_get(taskId, tPostEvoSpriteId)].set_invisible(TRUE as u16);
    DestroyTask(taskId);
}
