//! Translated from `src/battle_anim_status_effects.c` by tools/rustport/c2rs.py.
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

unsafe extern "C" {
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: Option<unsafe extern "C" fn()>;
    static mut gBattleAnimArgs: CArray<i16, 8>;
    static mut gBattleAnimAttacker: u8;
    static gBattleAnimPaletteTable: CArray<CompressedSpritePalette, 0>;
    static gBattleAnimPicTable: CArray<CompressedSpriteSheet, 0>;
    static mut gBattleAnimTarget: u8;
    static gBattleAnims_StatusConditions: CArray<*mut u8, 0>;
    static mut gBattleSpritesDataPtr: *mut BattleSpriteData;
    static mut gBattlerSpriteIds: CArray<u8, 4>;
    static mut gPlttBufferFaded: CArray<u16, 512>;
    static mut gSprites: CArray<Sprite, 65>;
    static mut gTasks: CArray<Task, 0>;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut Sprite);
    fn DestroySpriteAndFreeResources(a0: *mut Sprite);
    fn DestroyTask(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut Sprite);
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitStatsChangeAnimation(a0: u8);
    fn IsContest() -> u8;
    fn LaunchBattleAnimation(a0: *mut *mut u8, a1: u16, a2: u8);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut CompressedSpritePalette) -> u8;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut CompressedSpriteSheet) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSubspriteTables(a0: *mut Sprite, a1: *mut SubspriteTable);
    fn Sin(a0: i16, a1: i16) -> i16;
}

pub(crate) unsafe extern "C" fn Task_FlashingCircleImpacts(battler: u8, red: u8) -> u8 {
    let mut battlerSpriteId: u8 = gBattlerSpriteIds[battler];
    let mut taskId: u8 = CreateTask(Some(Task_UpdateFlashingCircleImpacts), 10);
    let mut spriteId: u8 = 0;
    let mut i: u8 = 0;
    LoadCompressedSpriteSheetUsingHeap((&raw const gBattleAnimPicTable[136]).cast_mut());
    LoadCompressedSpritePaletteUsingHeap((&raw const gBattleAnimPaletteTable[136]).cast_mut());
    gTasks[taskId].data[0] = battler as i16;
    if red != 0 {
        gTasks[taskId].data[1] = 31;
        i = 0;
        while i < 10 {
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
            i += 1;
        }
    } else {
        gTasks[taskId].data[1] = 31744;
        i = 0;
        while i < 10 {
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
            i += 1;
        }
    }
    gSprites[spriteId].data[7] = 1;
    return taskId;
}
pub(crate) unsafe extern "C" fn Task_UpdateFlashingCircleImpacts(taskId: u8) {
    if gTasks[taskId].data[2] == 2 {
        gTasks[taskId].data[2] = 0;
        BlendPalette(
            0x100 + gTasks[taskId].data[0] as u16 * 16,
            16,
            gTasks[taskId].data[4] as u8,
            gTasks[taskId].data[1] as u16,
        );
        if gTasks[taskId].data[5] == 0 {
            gTasks[taskId].data[4] += 1;
            if gTasks[taskId].data[4] > 8 {
                gTasks[taskId].data[5] ^= 1;
            }
        } else {
            let mut var: u16 = gTasks[taskId].data[4] as u16;
            gTasks[taskId].data[4] -= 1;
            if gTasks[taskId].data[4] < 0 {
                gTasks[taskId].data[4] = var as i16;
                gTasks[taskId].data[5] ^= 1;
                gTasks[taskId].data[3] += 1;
                if gTasks[taskId].data[3] == 2 {
                    DestroyTask(taskId);
                }
            }
        }
    } else {
        gTasks[taskId].data[2] += 1;
    }
}
pub(crate) unsafe extern "C" fn AnimFlashingCircleImpact(sprite: *mut Sprite) {
    if (*sprite).data[6] == 0 {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).callback = Some(AnimFlashingCircleImpact_Step);
        AnimFlashingCircleImpact_Step(sprite);
    } else {
        (*sprite).data[6] -= 1;
    }
}
pub(crate) unsafe extern "C" fn AnimFlashingCircleImpact_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Cos((*sprite).data[0], 32);
    (*sprite).y2 = Sin((*sprite).data[0], 8);
    if (*sprite).data[0] < 128 {
        (*sprite).subpriority = 29;
    } else {
        (*sprite).subpriority = 31;
    }
    (*sprite).data[0] = (*sprite).data[0] + 8 & 0xFF;
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
pub unsafe extern "C" fn AnimTask_FrozenIceCube(taskId: u8) {
    let mut x: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 - 32;
    let mut y: i16 =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 - 36;
    let mut spriteId: u8 = 0;
    if IsContest() != 0 {
        x -= 6;
    }
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    spriteId = CreateSprite(
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
    gTasks[taskId].data[15] = spriteId as i16;
    gTasks[taskId].func = Some(AnimTask_FrozenIceCube_Step1);
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step1(taskId: u8) {
    gTasks[taskId].data[1] += 1;
    if gTasks[taskId].data[1] == 10 {
        gTasks[taskId].func = Some(AnimTask_FrozenIceCube_Step2);
        gTasks[taskId].data[1] = 0;
    } else {
        let mut var: u8 = gTasks[taskId].data[1] as u8;
        SetGpuReg(REG_OFFSET_BLDALPHA, (16 - var as u16) << 8 | var as u16);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step2(taskId: u8) {
    let mut palIndex: u8 = IndexOfSpritePaletteTag(ANIM_TAG_ICE_CUBE);
    if ({
        let t1 = gTasks[taskId].data[1];
        gTasks[taskId].data[1] += 1;
        t1
    }) > 13
    {
        gTasks[taskId].data[2] += 1;
        if gTasks[taskId].data[2] == 3 {
            let mut temp: u16 = 0;
            temp = gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 13];
            gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 13] =
                gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 14];
            gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 14] =
                gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 15];
            gPlttBufferFaded[0x100 + palIndex as i32 * 16 + 15] = temp;
            gTasks[taskId].data[2] = 0;
            gTasks[taskId].data[3] += 1;
            if gTasks[taskId].data[3] == 3 {
                gTasks[taskId].data[3] = 0;
                gTasks[taskId].data[1] = 0;
                gTasks[taskId].data[4] += 1;
                if gTasks[taskId].data[4] == 2 {
                    gTasks[taskId].data[1] = 9;
                    gTasks[taskId].func = Some(AnimTask_FrozenIceCube_Step3);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step3(taskId: u8) {
    gTasks[taskId].data[1] -= 1;
    if gTasks[taskId].data[1] == -1 {
        gTasks[taskId].func = Some(AnimTask_FrozenIceCube_Step4);
        gTasks[taskId].data[1] = 0;
    } else {
        let mut var: u8 = gTasks[taskId].data[1] as u8;
        SetGpuReg(REG_OFFSET_BLDALPHA, (16 - var as u16) << 8 | var as u16);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step4(taskId: u8) {
    gTasks[taskId].data[1] += 1;
    if gTasks[taskId].data[1] == 37 {
        let mut spriteId: u8 = gTasks[taskId].data[15] as u8;
        FreeSpriteOamMatrix(&raw mut gSprites[spriteId]);
        DestroySprite(&raw mut gSprites[spriteId]);
    } else if gTasks[taskId].data[1] == 39 {
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StatsChange(taskId: u8) {
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
    gTasks[taskId].func = Some(InitStatsChangeAnimation);
    gTasks[taskId].func.unwrap_unchecked()(taskId);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchStatusAnimation(battler: u8, statusAnimId: u8) {
    let mut taskId: u8 = 0;
    gBattleAnimAttacker = battler;
    gBattleAnimTarget = battler;
    LaunchBattleAnimation(
        gBattleAnims_StatusConditions.as_ptr().cast_mut(),
        statusAnimId as u16,
        FALSE,
    );
    taskId = CreateTask(Some(Task_DoStatusAnimation), 10);
    gTasks[taskId].data[0] = battler as i16;
}
pub(crate) unsafe extern "C" fn Task_DoStatusAnimation(taskId: u8) {
    gAnimScriptCallback.unwrap_unchecked()();
    if gAnimScriptActive == 0 {
        (*(*gBattleSpritesDataPtr)
            .healthBoxesData
            .at(gTasks[taskId].data[0]))
        .set_statusAnimActive(0);
        DestroyTask(taskId);
    }
}
