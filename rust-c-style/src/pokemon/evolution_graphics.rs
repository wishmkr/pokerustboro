//! Translated from `src/evolution_graphics.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gPaletteFade: PaletteFadeControl;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gPlttBufferUnfaded: CArray<u16, 512>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32);
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8;
    fn LoadSpritePalettes(a0: *mut SpritePalette);
    fn PlaySE(a0: u16);
    fn Random() -> u16;
    fn SetOamMatrix(a0: u8, a1: u16, a2: u16, a3: u16, a4: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
}

pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_Dummy(sprite: *mut Sprite) {}
pub(crate) unsafe extern "C" fn SetEvoSparklesMatrices() {
    let mut i: u16 = 0;
    i = 0;
    while i < 12 {
        SetOamMatrix(
            20 + i as u8,
            sEvoSparkleMatrices[i],
            0,
            0,
            sEvoSparkleMatrices[i],
        );
        i += 1;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_SpiralUpward(sprite: *mut Sprite) {
    if (*sprite).y > 8 {
        let mut matrixNum: u8 = 0;
        (*sprite).y = 88 - ((*sprite).data[7] as i32 * (*sprite).data[7] as i32 / 80) as i16;
        (*sprite).y2 = Sin((*sprite).data[6] as u8 as i16, (*sprite).data[5]) / 4;
        (*sprite).x2 = Cos((*sprite).data[6] as u8 as i16, (*sprite).data[5]);
        (*sprite).data[6] += 4;
        if (*sprite).data[7] as i32 & 1 != 0 {
            (*sprite).data[5] -= 1;
        }
        (*sprite).data[7] += 1;
        if (*sprite).y2 > 0 {
            (*sprite).subpriority = 1;
        } else {
            (*sprite).subpriority = 20;
        }
        matrixNum = ((*sprite).data[5] / 4) as u8 + 20;
        if matrixNum > 31 {
            matrixNum = 31;
        }
        (*sprite).oam.set_matrixNum(matrixNum as u32);
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_SpiralUpward(trigIdx: u8) {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        88,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[5] = 48;
        gSprites[spriteId].data[6] = trigIdx as i16;
        gSprites[spriteId].data[7] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(31);
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_SpiralUpward);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_ArcDown(sprite: *mut Sprite) {
    if (*sprite).y < 88 {
        (*sprite).y = 8 + ((*sprite).data[7] as i32 * (*sprite).data[7] as i32 / 5) as i16;
        (*sprite).y2 = Sin((*sprite).data[6] as u8 as i16, (*sprite).data[5]) / 4;
        (*sprite).x2 = Cos((*sprite).data[6] as u8 as i16, (*sprite).data[5]);
        (*sprite).data[5] = 8 + Sin((*sprite).data[7] as u8 as i16 * 4, 40);
        (*sprite).data[7] += 1;
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_ArcDown(trigIdx: u8) {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        8,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[5] = 8;
        gSprites[spriteId].data[6] = trigIdx as i16;
        gSprites[spriteId].data[7] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(25);
        gSprites[spriteId].subpriority = 1;
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_ArcDown);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_CircleInward(sprite: *mut Sprite) {
    if (*sprite).data[5] > 8 {
        (*sprite).y2 = Sin((*sprite).data[6] as u8 as i16, (*sprite).data[5]);
        (*sprite).x2 = Cos((*sprite).data[6] as u8 as i16, (*sprite).data[5]);
        (*sprite).data[5] -= (*sprite).data[3];
        (*sprite).data[6] += 4;
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_CircleInward(trigIdx: u8, speed: u8) {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        56,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[3] = speed as i16;
        gSprites[spriteId].data[5] = 120;
        gSprites[spriteId].data[6] = trigIdx as i16;
        gSprites[spriteId].data[7] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(31);
        gSprites[spriteId].subpriority = 1;
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_CircleInward);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_Sparkle_Spray(sprite: *mut Sprite) {
    if (*sprite).data[7] as i32 & 3 == 0 {
        (*sprite).y += 1;
    }
    if (*sprite).data[6] < 128 {
        let mut matrixNum: u8 = 0;
        (*sprite).y2 = -Sin((*sprite).data[6] as u8 as i16, (*sprite).data[5]);
        (*sprite).x = 120 + ((*sprite).data[3] as i32 * (*sprite).data[7] as i32 / 3) as i16;
        (*sprite).data[6] += 1;
        matrixNum = 31 - ((*sprite).data[6] as i32 * 12 / 128) as u8;
        if (*sprite).data[6] > 64 {
            (*sprite).subpriority = 1;
        } else {
            (*sprite).set_invisible(FALSE as u16);
            (*sprite).subpriority = 20;
            if (*sprite).data[6] > 112 && (*sprite).data[6] as i32 & 1 != 0 {
                (*sprite).set_invisible(TRUE as u16);
            }
        }
        if matrixNum < 20 {
            matrixNum = 20;
        }
        (*sprite).oam.set_matrixNum(matrixNum as u32);
        (*sprite).data[7] += 1;
    } else {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn CreateSparkle_Spray(id: u8) {
    let mut spriteId: u8 = CreateSprite(
        (&raw const *sEvoSparkleSpriteTemplate).cast_mut(),
        120,
        56,
        0,
    );
    if spriteId != MAX_SPRITES {
        gSprites[spriteId].data[3] = 3 - (Random() as i32 % 7) as i16;
        gSprites[spriteId].data[5] = 48 + (Random() as i16 & 0x3F);
        gSprites[spriteId].data[7] = 0;
        gSprites[spriteId].oam.set_affineMode(ST_OAM_AFFINE_NORMAL);
        gSprites[spriteId].oam.set_matrixNum(31);
        gSprites[spriteId].subpriority = 20;
        gSprites[spriteId].callback = Some(SpriteCB_Sparkle_Spray);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadEvoSparkleSpriteAndPal() {
    LoadCompressedSpriteSheetUsingHeap((&raw const sEvoSparkleSpriteSheets[0]).cast_mut());
    LoadSpritePalettes(sEvoSparkleSpritePals.as_ptr().cast_mut());
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_SpiralUpward(palNum: u16) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_Sparkles_SpiralUpward_Init), 0);
    gTasks[taskId].data[1] = palNum as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SpiralUpward_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    gTasks[taskId].data[15] = 0;
    BeginNormalPaletteFade(
        shl_i32(3, gTasks[taskId].data[1] as u32) as u32,
        0xA,
        0,
        0x10,
        32767,
    );
    gTasks[taskId].func = Some(Task_Sparkles_SpiralUpward);
    PlaySE(SE_M_MEGA_KICK);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SpiralUpward(taskId: u8) {
    if gTasks[taskId].data[15] < 64 {
        if gTasks[taskId].data[15] as i32 & 7 == 0 {
            let mut i: u8 = 0;
            i = 0;
            while i < 4 {
                CreateSparkle_SpiralUpward((gTasks[taskId].data[15] as u8 & 120) * 2 + i * 64);
                i += 1;
            }
        }
        gTasks[taskId].data[15] += 1;
    } else {
        gTasks[taskId].data[15] = 96;
        gTasks[taskId].func = Some(Task_Sparkles_SpiralUpward_End);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SpiralUpward_End(taskId: u8) {
    if gTasks[taskId].data[15] != 0 {
        gTasks[taskId].data[15] -= 1;
    } else {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_ArcDown() -> u8 {
    return CreateTask(Some(Task_Sparkles_ArcDown_Init), 0);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_ArcDown_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    gTasks[taskId].data[15] = 0;
    gTasks[taskId].func = Some(Task_Sparkles_ArcDown);
    PlaySE(SE_M_BUBBLE_BEAM2);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_ArcDown(taskId: u8) {
    if gTasks[taskId].data[15] < 96 {
        if gTasks[taskId].data[15] < 6 {
            let mut i: u8 = 0;
            i = 0;
            while i < 9 {
                CreateSparkle_ArcDown(i * 16);
                i += 1;
            }
        }
        gTasks[taskId].data[15] += 1;
    } else {
        gTasks[taskId].func = Some(Task_Sparkles_ArcDown_End);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_ArcDown_End(taskId: u8) {
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_CircleInward() -> u8 {
    return CreateTask(Some(Task_Sparkles_CircleInward_Init), 0);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_CircleInward_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    gTasks[taskId].data[15] = 0;
    gTasks[taskId].func = Some(Task_Sparkles_CircleInward);
    PlaySE(SE_SHINY);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_CircleInward(taskId: u8) {
    if gTasks[taskId].data[15] < 48 {
        if gTasks[taskId].data[15] == 0 {
            let mut i: u8 = 0;
            i = 0;
            while i < 16 {
                CreateSparkle_CircleInward(i * 16, 4);
                i += 1;
            }
        }
        if gTasks[taskId].data[15] == 32 {
            let mut i: u8 = 0;
            i = 0;
            while i < 16 {
                CreateSparkle_CircleInward(i * 16, 8);
                i += 1;
            }
        }
        gTasks[taskId].data[15] += 1;
    } else {
        gTasks[taskId].func = Some(Task_Sparkles_CircleInward_End);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_CircleInward_End(taskId: u8) {
    DestroyTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_SprayAndFlash(species: u16) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_Sparkles_SprayAndFlash_Init), 0);
    gTasks[taskId].data[2] = species as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlash_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    gTasks[taskId].data[15] = 0;
    CpuSet(
        &raw mut gPlttBufferFaded[32] as *mut c_void,
        &raw mut gPlttBufferUnfaded[32] as *mut c_void,
        48,
    );
    BeginNormalPaletteFade(0xFFF9041C, 0, 0, 0x10, 32767);
    gTasks[taskId].func = Some(Task_Sparkles_SprayAndFlash);
    PlaySE(SE_M_PETAL_DANCE);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlash(taskId: u8) {
    if gTasks[taskId].data[15] < 128 {
        let mut i: u8 = 0;
        match gTasks[taskId].data[15] {
            0 => {
                i = 0;
                while i < 8 {
                    CreateSparkle_Spray(i);
                    i += 1;
                }
            }
            32 => {
                BeginNormalPaletteFade(0xFFFF041C, 0x10, 0x10, 0, 32767);
            }
            _ => {
                if gTasks[taskId].data[15] < 50 {
                    CreateSparkle_Spray(Random() as u8 & 7);
                }
            }
        }
        gTasks[taskId].data[15] += 1;
    } else {
        gTasks[taskId].func = Some(Task_Sparkles_SprayAndFlash_End);
    }
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlash_End(taskId: u8) {
    if gPaletteFade.active() == 0 {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn EvolutionSparkles_SprayAndFlash_Trade(species: u16) -> u8 {
    let mut taskId: u8 = CreateTask(Some(Task_Sparkles_SprayAndFlashTrade_Init), 0);
    gTasks[taskId].data[2] = species as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlashTrade_Init(taskId: u8) {
    SetEvoSparklesMatrices();
    gTasks[taskId].data[15] = 0;
    CpuSet(
        &raw mut gPlttBufferFaded[32] as *mut c_void,
        &raw mut gPlttBufferUnfaded[32] as *mut c_void,
        48,
    );
    BeginNormalPaletteFade(0xFFF90400, 0, 0, 0x10, 32767);
    gTasks[taskId].func = Some(Task_Sparkles_SprayAndFlashTrade);
    PlaySE(SE_M_PETAL_DANCE);
}
pub(crate) unsafe extern "C" fn Task_Sparkles_SprayAndFlashTrade(taskId: u8) {
    if gTasks[taskId].data[15] < 128 {
        let mut i: u8 = 0;
        match gTasks[taskId].data[15] {
            0 => {
                i = 0;
                while i < 8 {
                    CreateSparkle_Spray(i);
                    i += 1;
                }
            }
            32 => {
                BeginNormalPaletteFade(0xFFFF0400, 0x10, 0x10, 0, 32767);
            }
            _ => {
                if gTasks[taskId].data[15] < 50 {
                    CreateSparkle_Spray(Random() as u8 & 7);
                }
            }
        }
        gTasks[taskId].data[15] += 1;
    } else {
        gTasks[taskId].func = Some(Task_Sparkles_SprayAndFlash_End);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_EvolutionMonSprite(sprite: *mut Sprite) {}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CycleEvolutionMonSprite(preEvoSpriteId: u8, postEvoSpriteId: u8) -> u8 {
    let mut i: u16 = 0;
    let mut monPalette: CArray<u16, 16> = zeroed();
    let mut taskId: u8 = 0;
    let mut toDiv: i32 = 0;
    i = 0;
    while i < 16 {
        monPalette[i] = 32767;
        i += 1;
    }
    taskId = CreateTask(Some(Task_CycleEvolutionMonSprite_Init), 0);
    gTasks[taskId].data[1] = preEvoSpriteId as i16;
    gTasks[taskId].data[2] = postEvoSpriteId as i16;
    gTasks[taskId].data[3] = MON_MAX_SCALE;
    gTasks[taskId].data[4] = MON_MIN_SCALE;
    toDiv = 0x10000;
    SetOamMatrix(
        MATRIX_PRE_EVO,
        MON_MAX_SCALE as u16,
        0,
        0,
        MON_MAX_SCALE as u16,
    );
    SetOamMatrix(
        MATRIX_POST_EVO,
        div_i32(toDiv, gTasks[taskId].data[4] as i32) as u16,
        0,
        0,
        div_i32(toDiv, gTasks[taskId].data[4] as i32) as u16,
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
        &raw mut gPlttBufferFaded[0x100 + gSprites[preEvoSpriteId].oam.paletteNum() as i32 * 16]
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
        &raw mut gPlttBufferFaded[0x100 + gSprites[postEvoSpriteId].oam.paletteNum() as i32 * 16]
            as *mut c_void,
        16,
    );
    gTasks[taskId].data[8] = FALSE as i16;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_CycleEvolutionMonSprite_Init(taskId: u8) {
    gTasks[taskId].data[5] = FALSE as i16;
    gTasks[taskId].data[6] = 8;
    gTasks[taskId].func = Some(Task_CycleEvolutionMonSprite_TryEnd);
}
pub(crate) unsafe extern "C" fn Task_CycleEvolutionMonSprite_TryEnd(taskId: u8) {
    if gTasks[taskId].data[8] != 0 {
        EndOnPreEvoMon(taskId);
    } else if gTasks[taskId].data[6] == 128 {
        EndOnPostEvoMon(taskId);
    } else {
        gTasks[taskId].data[6] += 2;
        gTasks[taskId].data[5] ^= 1;
        gTasks[taskId].func = Some(Task_CycleEvolutionMonSprite_UpdateSize);
    }
}
pub(crate) unsafe extern "C" fn Task_CycleEvolutionMonSprite_UpdateSize(taskId: u8) {
    if gTasks[taskId].data[8] != 0 {
        gTasks[taskId].func = Some(EndOnPreEvoMon);
    } else {
        let mut oamMatrixArg: u16 = 0;
        let mut numSpritesFinished: u8 = 0;
        if gTasks[taskId].data[5] == 0 {
            if (gTasks[taskId].data[3] as i32)
                < MON_MAX_SCALE as i32 - gTasks[taskId].data[6] as i32
            {
                gTasks[taskId].data[3] += gTasks[taskId].data[6];
            } else {
                gTasks[taskId].data[3] = MON_MAX_SCALE;
                numSpritesFinished += 1;
            }
            if gTasks[taskId].data[4] as i32 > MON_MIN_SCALE as i32 + gTasks[taskId].data[6] as i32
            {
                gTasks[taskId].data[4] -= gTasks[taskId].data[6];
            } else {
                gTasks[taskId].data[4] = MON_MIN_SCALE;
                numSpritesFinished += 1;
            }
        } else {
            if (gTasks[taskId].data[4] as i32)
                < MON_MAX_SCALE as i32 - gTasks[taskId].data[6] as i32
            {
                gTasks[taskId].data[4] += gTasks[taskId].data[6];
            } else {
                gTasks[taskId].data[4] = MON_MAX_SCALE;
                numSpritesFinished += 1;
            }
            if gTasks[taskId].data[3] as i32 > MON_MIN_SCALE as i32 + gTasks[taskId].data[6] as i32
            {
                gTasks[taskId].data[3] -= gTasks[taskId].data[6];
            } else {
                gTasks[taskId].data[3] = MON_MIN_SCALE;
                numSpritesFinished += 1;
            }
        }
        oamMatrixArg = div_i32(0x10000, gTasks[taskId].data[3] as i32) as u16;
        SetOamMatrix(MATRIX_PRE_EVO, oamMatrixArg, 0, 0, oamMatrixArg);
        oamMatrixArg = div_i32(0x10000, gTasks[taskId].data[4] as i32) as u16;
        SetOamMatrix(MATRIX_POST_EVO, oamMatrixArg, 0, 0, oamMatrixArg);
        if numSpritesFinished == 2 {
            gTasks[taskId].func = Some(Task_CycleEvolutionMonSprite_TryEnd);
        }
    }
}
pub(crate) unsafe extern "C" fn EndOnPostEvoMon(taskId: u8) {
    gSprites[gTasks[taskId].data[1]]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[gTasks[taskId].data[1]].oam.set_matrixNum(0);
    gSprites[gTasks[taskId].data[1]].set_invisible(TRUE as u16);
    gSprites[gTasks[taskId].data[2]]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[gTasks[taskId].data[2]].oam.set_matrixNum(0);
    gSprites[gTasks[taskId].data[2]].set_invisible(FALSE as u16);
    DestroyTask(taskId);
}
pub(crate) unsafe extern "C" fn EndOnPreEvoMon(taskId: u8) {
    gSprites[gTasks[taskId].data[1]]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[gTasks[taskId].data[1]].oam.set_matrixNum(0);
    gSprites[gTasks[taskId].data[1]].set_invisible(FALSE as u16);
    gSprites[gTasks[taskId].data[2]]
        .oam
        .set_affineMode(ST_OAM_AFFINE_OFF);
    gSprites[gTasks[taskId].data[2]].oam.set_matrixNum(0);
    gSprites[gTasks[taskId].data[2]].set_invisible(TRUE as u16);
    DestroyTask(taskId);
}
