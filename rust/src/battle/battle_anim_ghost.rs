//! Translated from `src/battle_anim_ghost.c` by tools/rustport/c2rs.py.
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
    AnimTranslateLinear, CloneBattlerSpriteWithBlend, DestroyAnimSpriteAndDisableBlend,
    DestroySpriteAndMatrix, DestroySpriteWithActiveSheet, GetAnimBattlerSpriteId,
    GetBattlePalettesMask, GetBattlerSide, GetBattlerSpriteBGPriority,
    GetBattlerSpriteBGPriorityRank, GetBattlerSpriteCoord, GetBattlerSpriteCoordAttr,
    GetBattlerSpriteSubpriority, GetBattlerYCoordWithElevation, InitAnimLinearTranslationWithSpeed,
    InitSpritePosToAnimAttacker, InitSpritePosToAnimTarget, PrepareBattlerSpriteForRotScale,
    ResetSpriteRotScale, SetSpriteRotScale, StoreSpriteCallbackInData6,
    TranslateSpriteLinearFixedPoint, WaitAnimForDuration,
};
use crate::battle_main::gBattlerSpriteIds;
use crate::battle_main::{gBattle_WIN0H, gBattle_WIN0V, gBattleSpritesDataPtr};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::palette::{BeginNormalPaletteFade, gPaletteFade};
use crate::scanline_effect::ScanlineEffect_InitWave;
use crate::sound::{PlaySE, PlaySE12WithPanning};
use crate::sprite::gSprites;
use crate::sprite::{AllocSpritePalette, FreeSpritePaletteByTag};
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
/// `SpriteCallbackDummy` with this module's view of its types.
#[inline]
unsafe fn SpriteCallbackDummy(a0: *mut Sprite) {
    unsafe {
        crate::sprite::SpriteCallbackDummy(a0 as _);
    }
}
// Data tables (translate with cdata.py): sAffineAnim_ConfuseRayBallBounce sAffineAnims_ConfuseRayBallBounce gConfuseRayBallBounceSpriteTemplate gConfuseRayBallSpiralSpriteTemplate sAffineAnim_ShadowBall sAffineAnims_ShadowBall gShadowBallSpriteTemplate sAnim_Lick sAnims_Lick gLickSpriteTemplate sAffineAnim_Unused sAffineAnims_Unused gDestinyBondWhiteShadowSpriteTemplate gCurseNailSpriteTemplate gCurseGhostSpriteTemplate gNightmareDevilSpriteTemplate sAnim_GrudgeFlame sAnims_GrudgeFlame gGrudgeFlameSpriteTemplate sMonMoveCircularSpriteTemplate

static gDestinyBondWhiteShadowSpriteTemplate: Table<SpriteTemplate> = Table(
    (&raw const crate::data::battle_anim_ghost::gDestinyBondWhiteShadowSpriteTemplate).cast(),
);
static gGrudgeFlameSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_ghost::gGrudgeFlameSpriteTemplate).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub(crate) unsafe fn AnimConfuseRayBallBounce(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslationWithSpeed(sprite);
    (*sprite).callback = Some(AnimConfuseRayBallBounce_Step1);
    (*sprite).data[6] = 16;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, (*sprite).data[6] as u16);
}
pub(crate) unsafe fn AnimConfuseRayBallBounce_Step1(sprite: *mut Sprite) {
    UpdateConfuseRayBallBlend(sprite);
    if AnimTranslateLinear(sprite) != 0 {
        (*sprite).callback = Some(AnimConfuseRayBallBounce_Step2);
        return;
    }
    (*sprite).x2 += Sin((*sprite).data[5], 10);
    (*sprite).y2 += Cos((*sprite).data[5], 15);
    let r2: i16 = (*sprite).data[5];
    (*sprite).data[5] = ((*sprite).data[5] + 5) & 0xFF;
    let r0: i16 = (*sprite).data[5];
    if r2 != 0 && r2 <= 196 {
        return;
    }
    if r0 <= 0 {
        return;
    }
    PlaySE12WithPanning(SE_M_CONFUSE_RAY, gAnimCustomPanning as i8);
}
pub(crate) unsafe fn AnimConfuseRayBallBounce_Step2(sprite: *mut Sprite) {
    (*sprite).data[0] = 1;
    AnimTranslateLinear(sprite);
    (*sprite).x2 += Sin((*sprite).data[5], 10);
    (*sprite).y2 += Cos((*sprite).data[5], 15);
    let r2: i16 = (*sprite).data[5];
    (*sprite).data[5] = ((*sprite).data[5] + 5) & 0xFF;
    let r0: i16 = (*sprite).data[5];
    if (r2 == 0 || r2 > 196) && r0 > 0 {
        PlaySE(SE_M_CONFUSE_RAY);
    }
    if (*sprite).data[6] == 0 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(DestroyAnimSpriteAndDisableBlend);
    } else {
        UpdateConfuseRayBallBlend(sprite);
    }
}
unsafe fn UpdateConfuseRayBallBlend(sprite: *mut Sprite) {
    if (*sprite).data[6] > 0xFF {
        if ({
            (*sprite).data[6] += 1;
            (*sprite).data[6]
        }) == 0x10d
        {
            (*sprite).data[6] = 0;
        }
        return;
    }
    if ({
        let t2 = (*sprite).data[7];
        (*sprite).data[7] += 1;
        t2
    }) as i32
        & 0xFF
        == 0
    {
        (*sprite).data[7] &= -256;
        if (*sprite).data[7] as i32 & 0x100 != 0 {
            (*sprite).data[6] += 1;
        } else {
            (*sprite).data[6] -= 1;
        }
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - (*sprite).data[6] as u16) << 8 | (*sprite).data[6] as u16,
        );
        if (*sprite).data[6] == 0 || (*sprite).data[6] == 16 {
            (*sprite).data[7] ^= 0x100;
        }
        if (*sprite).data[6] == 0 {
            (*sprite).data[6] = 0x100;
        }
    }
}
pub(crate) unsafe fn AnimConfuseRayBallSpiral(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    (*sprite).callback = Some(AnimConfuseRayBallSpiral_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimConfuseRayBallSpiral_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[0], 32);
    (*sprite).y2 = Cos((*sprite).data[0], 8);
    let temp1: u16 = (*sprite).data[0] as u16 - 65;
    if temp1 <= 130 {
        (*sprite).oam.set_priority(2);
    } else {
        (*sprite).oam.set_priority(1);
    }
    (*sprite).data[0] = ((*sprite).data[0] + 19) & 0xFF;
    (*sprite).data[2] += 80;
    (*sprite).y2 += (*sprite).data[2] >> 8;
    (*sprite).data[7] += 1;
    if (*sprite).data[7] == 61 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_NightShadeClone(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_BLEND as u8);
    SetSpriteRotScale(spriteId, 128, 128, 0);
    gSprites[spriteId].set_invisible(FALSE as u16);
    task_set(taskId, 0, 128);
    task_set(taskId, 1, *gBattleAnimArgs.as_mut_ptr());
    task_set(taskId, 2, 0);
    task_set(taskId, 3, 16);
    task_set_func(taskId, Some(AnimTask_NightShadeClone_Step1));
}
pub(crate) unsafe fn AnimTask_NightShadeClone_Step1(taskId: u8) {
    task_set(taskId, 10, task_get(taskId, 10) + 1);
    if task_get(taskId, 10) == 3 {
        task_set(taskId, 10, 0);
        task_set(taskId, 2, task_get(taskId, 2) + 1);
        task_set(taskId, 3, task_get(taskId, 3) - 1);
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (task_get(taskId, 3) as u16) << 8 | task_get(taskId, 2) as u16,
        );
        if task_get(taskId, 2) != 9 {
            return;
        }
        task_set_func(taskId, Some(AnimTask_NightShadeClone_Step2));
    }
}
pub(crate) unsafe fn AnimTask_NightShadeClone_Step2(taskId: u8) {
    if task_get(taskId, 1) > 0 {
        task_set(taskId, 1, task_get(taskId, 1) - 1);
        return;
    }
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    task_set(taskId, 0, task_get(taskId, 0) + 8);
    if task_get(taskId, 0) <= 0xFF {
        SetSpriteRotScale(spriteId, task_get(taskId, 0), task_get(taskId, 0), 0);
    } else {
        ResetSpriteRotScale(spriteId);
        DestroyAnimVisualTask(taskId);
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    }
}
pub(crate) unsafe fn AnimShadowBall(sprite: *mut Sprite) {
    let oldPosX: i16 = (*sprite).x;
    let oldPosY: i16 = (*sprite).y;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = 0;
    (*sprite).data[1] = gBattleAnimArgs[0];
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).data[3] = gBattleAnimArgs[2];
    (*sprite).data[4] = (*sprite).x << 4;
    (*sprite).data[5] = (*sprite).y << 4;
    (*sprite).data[6] = div_i32(
        (oldPosX as i32 - (*sprite).x as i32) << 4,
        (gBattleAnimArgs[0] as i32) << 1,
    ) as i16;
    (*sprite).data[7] = div_i32(
        (oldPosY as i32 - (*sprite).y as i32) << 4,
        (gBattleAnimArgs[0] as i32) << 1,
    ) as i16;
    (*sprite).callback = Some(AnimShadowBall_Step);
}
pub(crate) unsafe fn AnimShadowBall_Step(sprite: *mut Sprite) {
    'l1: {
        match (*sprite).data[0] {
            0 => {
                (*sprite).data[4] += (*sprite).data[6];
                (*sprite).data[5] += (*sprite).data[7];
                (*sprite).x = (*sprite).data[4] >> 4;
                (*sprite).y = (*sprite).data[5] >> 4;
                (*sprite).data[1] -= 1;
                if (*sprite).data[1] > 0 {
                    break 'l1;
                }
                (*sprite).data[0] += 1;
            }
            1 => {
                (*sprite).data[2] -= 1;
                if (*sprite).data[2] > 0 {
                    break 'l1;
                }
                (*sprite).data[1] =
                    GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
                (*sprite).data[2] =
                    GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
                (*sprite).data[4] = (*sprite).x << 4;
                (*sprite).data[5] = (*sprite).y << 4;
                (*sprite).data[6] = div_i32(
                    ((*sprite).data[1] as i32 - (*sprite).x as i32) << 4,
                    (*sprite).data[3] as i32,
                ) as i16;
                (*sprite).data[7] = div_i32(
                    ((*sprite).data[2] as i32 - (*sprite).y as i32) << 4,
                    (*sprite).data[3] as i32,
                ) as i16;
                (*sprite).data[0] += 1;
            }
            2 => {
                (*sprite).data[4] += (*sprite).data[6];
                (*sprite).data[5] += (*sprite).data[7];
                (*sprite).x = (*sprite).data[4] >> 4;
                (*sprite).y = (*sprite).data[5] >> 4;
                (*sprite).data[3] -= 1;
                if (*sprite).data[3] > 0 {
                    break 'l1;
                }
                (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
                (*sprite).y =
                    GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
                (*sprite).data[0] += 1;
            }
            3 => {
                DestroySpriteAndMatrix(sprite);
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn AnimLick(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    (*sprite).callback = Some(AnimLick_Step);
}
pub(crate) unsafe fn AnimLick_Step(sprite: *mut Sprite) {
    let mut r5: u8 = FALSE;
    let mut r6: u8 = FALSE;
    if (*sprite).animEnded() != 0 {
        if (*sprite).invisible() == 0 {
            (*sprite).set_invisible(TRUE as u16);
        }
        match (*sprite).data[0] {
            0 => {
                if (*sprite).data[1] == 2 {
                    r5 = TRUE;
                }
            }
            1 => {
                if (*sprite).data[1] == 4 {
                    r5 = TRUE;
                }
            }
            _ => {
                r6 = TRUE;
            }
        }
        if r5 != 0 {
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
            (*sprite).data[2] += 1;
            (*sprite).data[1] = 0;
            if (*sprite).data[2] == 5 {
                (*sprite).data[2] = 0;
                (*sprite).data[0] += 1;
            }
        } else if r6 != 0 {
            DestroyAnimSprite(sprite);
        } else {
            (*sprite).data[1] += 1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_NightmareClone(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = CloneBattlerSpriteWithBlend(ANIM_TARGET);
    if (*task).data[0] < 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    (*task).data[1] = 0;
    (*task).data[2] = 15;
    (*task).data[3] = 2;
    (*task).data[4] = 0;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        ((*task).data[3] as u16) << 8 | (*task).data[2] as u16,
    );
    gSprites[(*task).data[0]].data[0] = 80;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        gSprites[(*task).data[0]].data[1] = -144;
        gSprites[(*task).data[0]].data[2] = 112;
    } else {
        gSprites[(*task).data[0]].data[1] = 144;
        gSprites[(*task).data[0]].data[2] = -112;
    }
    gSprites[(*task).data[0]].data[3] = 0;
    gSprites[(*task).data[0]].data[4] = 0;
    StoreSpriteCallbackInData6(
        &raw mut gSprites[(*task).data[0]],
        Some(SpriteCallbackDummy),
    );
    gSprites[(*task).data[0]].callback = Some(TranslateSpriteLinearFixedPoint);
    (*task).func = Some(AnimTask_NightmareClone_Step);
}
pub(crate) unsafe fn AnimTask_NightmareClone_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    'l1: {
        match (*task).data[4] {
            0 => {
                (*task).data[1] += 1;
                (*task).data[5] = (*task).data[1] & 3;
                if (*task).data[5] == 1 && (*task).data[2] > 0 {
                    (*task).data[2] -= 1;
                }
                if (*task).data[5] == 3 && (*task).data[3] <= 15 {
                    (*task).data[3] += 1;
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*task).data[3] as u16) << 8 | (*task).data[2] as u16,
                );
                if (*task).data[3] != 16 || (*task).data[2] != 0 {
                    break 'l1;
                }
                if (*task).data[1] <= 80 {
                    break 'l1;
                }
                DestroySpriteWithActiveSheet(&raw mut gSprites[(*task).data[0]]);
                (*task).data[4] = 1;
            }
            1 => {
                if ({
                    (*task).data[6] += 1;
                    (*task).data[6]
                }) <= 1
                {
                    break 'l1;
                }
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                (*task).data[4] += 1;
            }
            2 => {
                DestroyAnimVisualTask(taskId);
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SpiteTargetShadow(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[15] = 0;
    (*task).func = Some(AnimTask_SpiteTargetShadow_Step1);
    (*task).func.unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_SpiteTargetShadow_Step1(taskId: u8) {
    let mut startLine: i16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let position: u8 = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
    match (*task).data[15] {
        0 => {
            (*task).data[14] = AllocSpritePalette(ANIM_TAG_BENT_SPOON) as i16;
            if (*task).data[14] == 0xFF || (*task).data[14] == 0xF {
                DestroyAnimVisualTask(taskId);
            } else {
                (*task).data[0] = CloneBattlerSpriteWithBlend(ANIM_TARGET);
                if (*task).data[0] < 0 {
                    FreeSpritePaletteByTag(ANIM_TAG_BENT_SPOON);
                    DestroyAnimVisualTask(taskId);
                } else {
                    let mut mask2: i16 = 0;
                    gSprites[(*task).data[0]]
                        .oam
                        .set_paletteNum((*task).data[14] as u16);
                    gSprites[(*task).data[0]].oam.set_objMode(0);
                    gSprites[(*task).data[0]].oam.set_priority(3);
                    gSprites[(*task).data[0]].set_invisible(
                        (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).invisible(),
                    );
                    (*task).data[1] = 0;
                    (*task).data[2] = 0;
                    (*task).data[3] = 16;
                    (*task).data[13] = GetAnimBattlerSpriteId(ANIM_TARGET) as i16;
                    (*task).data[4] =
                        (gSprites[(*task).data[13]].oam.paletteNum() as i16 + 16) * 16;
                    if position == 1 {
                        let mask: u16 = DISPCNT_BG1_ON;
                        mask2 = mask as i16;
                    } else {
                        let mask: u16 = DISPCNT_BG2_ON;
                        mask2 = mask as i16;
                    }
                    ClearGpuRegBits(REG_OFFSET_DISPCNT, mask2 as u16);
                    (*task).data[15] += 1;
                }
            }
        }
        1 => {
            (*task).data[14] = ((*task).data[14] + 16) * 16;
            CpuSet(
                &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[(*task).data[4]] as *mut c_void,
                &raw mut (*(&raw const crate::palette::gPlttBufferFaded)
                    .cast::<CArray<u16, 512>>()
                    .cast_mut())[(*task).data[14]] as *mut c_void,
                0x4000008,
            );
            BlendPalette((*task).data[4] as u16, 16, 10, 15373);
            (*task).data[15] += 1;
        }
        2 => {
            startLine = gSprites[(*task).data[13]].y + gSprites[(*task).data[13]].y2 - 32;
            if startLine < 0 {
                startLine = 0;
            }
            if position == 1 {
                (*task).data[10] = ScanlineEffect_InitWave(
                    startLine as u8,
                    startLine as u8 + 64,
                    2,
                    6,
                    0,
                    SCANLINE_EFFECT_REG_BG1HOFS,
                    TRUE,
                ) as i16;
            } else {
                (*task).data[10] = ScanlineEffect_InitWave(
                    startLine as u8,
                    startLine as u8 + 64,
                    2,
                    6,
                    0,
                    SCANLINE_EFFECT_REG_BG2HOFS,
                    TRUE,
                ) as i16;
            }
            (*task).data[15] += 1;
        }
        3 => {
            if position == 1 {
                SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            } else {
                SetGpuReg(REG_OFFSET_BLDCNT, 16196);
            }
            SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
            (*task).data[15] += 1;
        }
        4 => {
            if position == 1 {
                SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG1_ON);
            } else {
                SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            }
            (*task).func = Some(AnimTask_SpiteTargetShadow_Step2);
            (*task).data[15] += 1;
        }
        _ => {
            (*task).data[15] += 1;
        }
    }
}
pub(crate) unsafe fn AnimTask_SpiteTargetShadow_Step2(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[1] += 1;
    (*task).data[5] = (*task).data[1] & 1;
    if (*task).data[5] == 0 {
        (*task).data[2] =
            (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*task).data[1]] / 18;
    }
    if (*task).data[5] == 1 {
        (*task).data[3] = 16
            - (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*task).data[1]]
                / 18;
    }
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        ((*task).data[3] as u16) << 8 | (*task).data[2] as u16,
    );
    if (*task).data[1] == 128 {
        (*task).data[15] = 0;
        (*task).func = Some(AnimTask_SpiteTargetShadow_Step3);
        (*task).func.unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe fn AnimTask_SpiteTargetShadow_Step3(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let rank: u8 = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
    match (*task).data[15] {
        0 => {
            (*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .state = 3;
            (*task).data[14] = GetAnimBattlerSpriteId(ANIM_TARGET) as i16;
            if rank == 1 {
                ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG1_ON);
            } else {
                ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            }
        }
        1 => {
            BlendPalette((*task).data[4] as u16, 16, 0, 15373);
        }
        2 => {
            gSprites[(*task).data[14]].set_invisible(TRUE as u16);
            DestroySpriteWithActiveSheet(&raw mut gSprites[(*task).data[0]]);
            FreeSpritePaletteByTag(ANIM_TAG_BENT_SPOON);
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            if rank == 1 {
                SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG1_ON);
            } else {
                SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
            }
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
    (*task).data[15] += 1;
}
pub(crate) unsafe fn AnimDestinyBondWhiteShadow(sprite: *mut Sprite) {
    let mut battler1X: i16 = 0;
    let mut battler1Y: i16 = 0;
    let mut battler2X: i16 = 0;
    let mut battler2Y: i16 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler1X = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        battler1Y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + 28;
        battler2X = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        battler2Y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + 28;
    } else {
        battler1X = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        battler1Y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + 28;
        battler2X = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        battler2Y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + 28;
    }
    let yDiff: i16 = battler2Y - battler1Y;
    (*sprite).data[0] = battler1X * 16;
    (*sprite).data[1] = battler1Y * 16;
    (*sprite).data[2] = div_i32(
        (battler2X as i32 - battler1X as i32) * 16,
        gBattleAnimArgs[1] as i32,
    ) as i16;
    (*sprite).data[3] = div_i32(yDiff as i32 * 16, gBattleAnimArgs[1] as i32) as i16;
    (*sprite).data[4] = gBattleAnimArgs[1];
    (*sprite).data[5] = battler2X;
    (*sprite).data[6] = battler2Y;
    (*sprite).data[7] = (*sprite).data[4] / 2;
    (*sprite).oam.set_priority(2);
    (*sprite).x = battler1X;
    (*sprite).y = battler1Y;
    (*sprite).callback = Some(AnimDestinyBondWhiteShadow_Step);
    (*sprite).set_invisible(TRUE as u16);
}
pub(crate) unsafe fn AnimDestinyBondWhiteShadow_Step(sprite: *mut Sprite) {
    if (*sprite).data[4] != 0 {
        (*sprite).data[0] += (*sprite).data[2];
        (*sprite).data[1] += (*sprite).data[3];
        (*sprite).x = (*sprite).data[0] >> 4;
        (*sprite).y = (*sprite).data[1] >> 4;
        if ({
            (*sprite).data[4] -= 1;
            (*sprite).data[4]
        }) == 0
        {
            (*sprite).data[0] = 0;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DestinyBondWhiteShadow(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    (*task).data[5] = 0;
    (*task).data[6] = 0;
    (*task).data[7] = 0;
    (*task).data[8] = 0;
    (*task).data[9] = 16;
    (*task).data[10] = gBattleAnimArgs[0];
    let baseX: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    let baseY: i16 = GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_BOTTOM);
    if IsContest() == 0 {
        for battler in 0..(MAX_BATTLERS_COUNT as i16) {
            if battler != gBattleAnimAttacker as i16
                && battler as i32 != gBattleAnimAttacker as i32 ^ 2
                && IsBattlerSpriteVisible(battler as u8) != 0
            {
                spriteId = CreateSprite(
                    (&raw const *gDestinyBondWhiteShadowSpriteTemplate).cast_mut(),
                    baseX,
                    baseY,
                    55,
                );
                if spriteId != MAX_SPRITES {
                    x = GetBattlerSpriteCoord(battler as u8, BATTLER_COORD_X_2) as i16;
                    y = GetBattlerSpriteCoordAttr(battler as u8, BATTLER_COORD_ATTR_BOTTOM);
                    gSprites[spriteId].data[0] = baseX << 4;
                    gSprites[spriteId].data[1] = baseY << 4;
                    gSprites[spriteId].data[2] =
                        div_i32((x as i32 - baseX as i32) << 4, gBattleAnimArgs[1] as i32) as i16;
                    gSprites[spriteId].data[3] =
                        div_i32((y as i32 - baseY as i32) << 4, gBattleAnimArgs[1] as i32) as i16;
                    gSprites[spriteId].data[4] = gBattleAnimArgs[1];
                    gSprites[spriteId].data[5] = x;
                    gSprites[spriteId].data[6] = y;
                    gSprites[spriteId].callback = Some(AnimDestinyBondWhiteShadow_Step);
                    (*task).data[(*task).data[12] as i32 + 13] = spriteId as i16;
                    (*task).data[12] += 1;
                }
            }
        }
    } else {
        spriteId = CreateSprite(
            (&raw const *gDestinyBondWhiteShadowSpriteTemplate).cast_mut(),
            baseX,
            baseY,
            55,
        );
        if spriteId != MAX_SPRITES {
            x = 48;
            y = 40;
            gSprites[spriteId].data[0] = baseX << 4;
            gSprites[spriteId].data[1] = baseY << 4;
            gSprites[spriteId].data[2] =
                div_i32((x as i32 - baseX as i32) << 4, gBattleAnimArgs[1] as i32) as i16;
            gSprites[spriteId].data[3] =
                div_i32((y as i32 - baseY as i32) << 4, gBattleAnimArgs[1] as i32) as i16;
            gSprites[spriteId].data[4] = gBattleAnimArgs[1];
            gSprites[spriteId].data[5] = x;
            gSprites[spriteId].data[6] = y;
            gSprites[spriteId].callback = Some(AnimDestinyBondWhiteShadow_Step);
            (*task).data[13] = spriteId as i16;
            (*task).data[12] = 1;
        }
    }
    (*task).func = Some(AnimTask_DestinyBondWhiteShadow_Step);
}
pub(crate) unsafe fn AnimTask_DestinyBondWhiteShadow_Step(taskId: u8) {
    let mut i: u16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if (*task).data[6] == 0
                && ({
                    (*task).data[5] += 1;
                    (*task).data[5]
                }) > 1
            {
                (*task).data[5] = 0;
                (*task).data[7] += 1;
                if (*task).data[7] as i32 & 1 != 0 {
                    if (*task).data[8] < 16 {
                        (*task).data[8] += 1;
                    }
                } else {
                    if (*task).data[9] != 0 {
                        (*task).data[9] -= 1;
                    }
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*task).data[9] as u16) << 8 | (*task).data[8] as u16,
                );
                if (*task).data[7] >= 24 {
                    (*task).data[7] = 0;
                    (*task).data[6] = 1;
                }
            }
            if (*task).data[10] != 0 {
                (*task).data[10] -= 1;
            } else if (*task).data[6] != 0 {
                (*task).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*task).data[5] += 1;
                (*task).data[5]
            }) > 1
            {
                (*task).data[5] = 0;
                (*task).data[7] += 1;
                if (*task).data[7] as i32 & 1 != 0 {
                    if (*task).data[8] != 0 {
                        (*task).data[8] -= 1;
                    }
                } else {
                    if (*task).data[9] < 16 {
                        (*task).data[9] += 1;
                    }
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*task).data[9] as u16) << 8 | (*task).data[8] as u16,
                );
                if (*task).data[8] == 0 && (*task).data[9] == 16 {
                    i = 0;
                    while (i as i32) < (*task).data[12] as i32 {
                        DestroySprite(&raw mut gSprites[(*task).data[i as i32 + 13]]);
                        i += 1;
                    }
                    (*task).data[0] += 1;
                }
            }
        }
        2 => {
            if ({
                (*task).data[5] += 1;
                (*task).data[5]
            }) > 0
            {
                (*task).data[0] += 1;
            }
        }
        3 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CurseStretchingBlackBg(taskId: u8) {
    let mut startX: i16 = 0;
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16159);
    SetGpuReg(REG_OFFSET_BLDCNT, 200);
    SetGpuReg(REG_OFFSET_BLDY, 16);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER || IsContest() != 0 {
        startX = 40;
    } else {
        startX = 200;
    }
    gBattle_WIN0H = (startX as u16) << 8 | startX as u16;
    let startY: i16 = 40;
    gBattle_WIN0V = (startY as u16) << 8 | startY as u16;
    let leftDistance: i16 = startX;
    let rightDistance: i16 = DISPLAY_WIDTH as i16 - startX;
    let topDistance: i16 = startY;
    let bottomDistance: i16 = 72;
    task_set(taskId, 1, leftDistance);
    task_set(taskId, 2, rightDistance);
    task_set(taskId, 3, topDistance);
    task_set(taskId, 4, bottomDistance);
    task_set(taskId, 5, startX);
    task_set(taskId, 6, startY);
    task_set_func(taskId, Some(AnimTask_CurseStretchingBlackBg_Step1));
}
pub(crate) unsafe fn AnimTask_CurseStretchingBlackBg_Step1(taskId: u8) {
    let mut left: u16 = 0;
    let mut right: u16 = 0;
    let mut top: u16 = 0;
    let mut bottom: u16 = 0;
    let mut selectedPalettes: u16 = 0;
    let step: i16 = task_get(taskId, 0);
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    let leftDistance: i16 = task_get(taskId, 1);
    let rightDistance: i16 = task_get(taskId, 2);
    let topDistance: i16 = task_get(taskId, 3);
    let bottomDistance: i16 = task_get(taskId, 4);
    let startX: i16 = task_get(taskId, 5);
    let startY: i16 = task_get(taskId, 6);
    if step < 16 {
        left = (startX as f32 - ((leftDistance as f32 * 0_f32) * step as f32)) as u16;
        right = (startX as f32 + ((rightDistance as f32 * 0_f32) * step as f32)) as u16;
        top = (startY as f32 - ((topDistance as f32 * 0_f32) * step as f32)) as u16;
        bottom = (startY as f32 + ((bottomDistance as f32 * 0_f32) * step as f32)) as u16;
    } else {
        left = 0;
        right = DISPLAY_WIDTH;
        top = 0;
        bottom = 112;
        selectedPalettes =
            GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE) as u16;
        BeginNormalPaletteFade(selectedPalettes as u32, 0, 16, 16, 0);
        task_set_func(taskId, Some(AnimTask_CurseStretchingBlackBg_Step2));
    }
    gBattle_WIN0H = left << 8 | right;
    gBattle_WIN0V = top << 8 | bottom;
}
pub(crate) unsafe fn AnimTask_CurseStretchingBlackBg_Step2(taskId: u8) {
    if gPaletteFade.active() == 0 {
        gBattle_WIN0H = 0;
        gBattle_WIN0V = 0;
        SetGpuReg(REG_OFFSET_WININ, 16191);
        SetGpuReg(REG_OFFSET_WINOUT, 16191);
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDY, 0);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimCurseNail(sprite: *mut Sprite) {
    let mut xDelta: i16 = 0;
    let mut xDelta2: i16 = 0;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        xDelta = 24;
        xDelta2 = -2;
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
    } else {
        xDelta = -24;
        xDelta2 = 2;
    }
    (*sprite).x += xDelta;
    (*sprite).data[1] = xDelta2;
    (*sprite).data[0] = 60;
    (*sprite).callback = Some(AnimCurseNail_Step1);
}
pub(crate) unsafe fn AnimCurseNail_Step1(sprite: *mut Sprite) {
    let mut var0: u16 = 0;
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
    } else {
        (*sprite).x2 += (*sprite).data[1];
        var0 = (*sprite).x2 as u16 + 7;
        if var0 > 14 {
            (*sprite).x += (*sprite).x2;
            (*sprite).x2 = 0;
            (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 8);
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) == 3
            {
                (*sprite).data[0] = 30;
                (*sprite).callback = Some(WaitAnimForDuration);
                StoreSpriteCallbackInData6(sprite, Some(AnimCurseNail_Step2));
            } else {
                (*sprite).data[0] = 40;
            }
        }
    }
}
pub(crate) unsafe fn AnimCurseNail_Step2(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        SetGpuReg(REG_OFFSET_BLDCNT, 16192);
        SetGpuReg(REG_OFFSET_BLDALPHA, 16);
        (*sprite).data[0] += 1;
        (*sprite).data[1] = 0;
        (*sprite).data[2] = 0;
    } else if (*sprite).data[1] < 2 {
        (*sprite).data[1] += 1;
    } else {
        (*sprite).data[1] = 0;
        (*sprite).data[2] += 1;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - (*sprite).data[2] as u16) | (((*sprite).data[2] as u16) << 8),
        );
        if (*sprite).data[2] == 16 {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(AnimCurseNail_End);
        }
    }
}
pub(crate) unsafe fn AnimCurseNail_End(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    DestroyAnimSprite(sprite);
}
pub(crate) unsafe fn AnimGhostStatusSprite(sprite: *mut Sprite) {
    let mut coeffB: u16 = 0;
    let mut coeffA: u16 = 0;
    (*sprite).x2 = Sin((*sprite).data[0], 12);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x2 = -(*sprite).x2;
    }
    (*sprite).data[0] = ((*sprite).data[0] + 6) & 0xFF;
    (*sprite).data[1] += 0x100;
    (*sprite).y2 = -((*sprite).data[1] >> 8);
    (*sprite).data[7] += 1;
    if (*sprite).data[7] == 1 {
        (*sprite).data[6] = 0x050B;
        SetGpuReg(REG_OFFSET_BLDCNT, 16192);
        SetGpuReg(REG_OFFSET_BLDALPHA, (*sprite).data[6] as u16);
    } else if (*sprite).data[7] > 30 {
        (*sprite).data[2] += 1;
        coeffB = ((*sprite).data[6] >> 8) as u16;
        coeffA = (*sprite).data[6] as u16 & 0xFF;
        if ({
            coeffB += 1;
            coeffB
        }) > 16
        {
            coeffB = 16;
        }
        coeffA -= 1;
        if (coeffA as i16) < 0 {
            coeffA = 0;
        }
        SetGpuReg(REG_OFFSET_BLDALPHA, coeffB << 8 | coeffA);
        (*sprite).data[6] = (coeffB as i16) << 8 | coeffA as i16;
        if coeffB == 16 && coeffA == 0 {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(AnimGhostStatusSprite_Step);
        }
    }
}
pub(crate) unsafe fn AnimGhostStatusSprite_Step(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    DestroyAnimSprite(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GrudgeFlames(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[1] = 16;
    (*task).data[9] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*task).data[10] = GetBattlerYCoordWithElevation(gBattleAnimAttacker) as i16;
    (*task).data[11] =
        GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_WIDTH) / 2 + 8;
    (*task).data[7] = 0;
    (*task).data[5] = GetBattlerSpriteBGPriority(gBattleAnimAttacker) as i16;
    (*task).data[6] = GetBattlerSpriteSubpriority(gBattleAnimAttacker) as i16 - 2;
    (*task).data[3] = 0;
    (*task).data[4] = 16;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    (*task).data[8] = 0;
    (*task).func = Some(AnimTask_GrudgeFlames_Step);
}
pub(crate) unsafe fn AnimTask_GrudgeFlames_Step(taskId: u8) {
    let mut spriteId: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            for i in 0..6u16 {
                spriteId = CreateSprite(
                    (&raw const *gGrudgeFlameSpriteTemplate).cast_mut(),
                    (*task).data[9],
                    (*task).data[10],
                    (*task).data[6] as u8,
                );
                if spriteId != MAX_SPRITES {
                    gSprites[spriteId].data[0] = taskId as i16;
                    gSprites[spriteId].data[1] =
                        (GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER) as i16;
                    gSprites[spriteId].data[2] = (i as i16 * 42) & 0xFF;
                    gSprites[spriteId].data[3] = (*task).data[11];
                    gSprites[spriteId].data[5] = i as i16 * 6;
                    (*task).data[7] += 1;
                }
            }
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) as i32
                & 1
                != 0
            {
                if (*task).data[3] < 14 {
                    (*task).data[3] += 1;
                }
            } else {
                if (*task).data[4] > 4 {
                    (*task).data[4] -= 1;
                }
            }
            if (*task).data[3] == 14 && (*task).data[4] == 4 {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                ((*task).data[4] as u16) << 8 | (*task).data[3] as u16,
            );
        }
        2 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 30
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        3 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) as i32
                & 1
                != 0
            {
                if (*task).data[3] > 0 {
                    (*task).data[3] -= 1;
                }
            } else {
                if (*task).data[4] < 16 {
                    (*task).data[4] += 1;
                }
            }
            if (*task).data[3] == 0 && (*task).data[4] == 16 {
                (*task).data[8] = 1;
                (*task).data[0] += 1;
            }
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                ((*task).data[4] as u16) << 8 | (*task).data[3] as u16,
            );
        }
        4 => {
            if (*task).data[7] == 0 {
                (*task).data[0] += 1;
            }
        }
        5 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimGrudgeFlame(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        (*sprite).data[2] += 2;
    } else {
        (*sprite).data[2] -= 2;
    }
    (*sprite).data[2] &= 0xFF;
    (*sprite).x2 = Sin((*sprite).data[2], (*sprite).data[3]);
    let index: u16 = (*sprite).data[2] as u16 - 65;
    if index < 127 {
        (*sprite)
            .oam
            .set_priority(task_get((*sprite).data[0], 5) as u16 + 1);
    } else {
        (*sprite)
            .oam
            .set_priority(task_get((*sprite).data[0], 5) as u16);
    }
    (*sprite).data[5] += 1;
    (*sprite).data[6] = ((*sprite).data[5] * 8) & 0xFF;
    (*sprite).y2 = Sin((*sprite).data[6], 7);
    if task_get((*sprite).data[0], 8) != 0 {
        task_set((*sprite).data[0], 7, task_get((*sprite).data[0], 7) - 1);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn AnimMonMoveCircular(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).data[5] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
    (*sprite).data[0] = 128;
    (*sprite).data[1] = 10;
    (*sprite).data[2] = gBattleAnimArgs[0];
    (*sprite).data[3] = gBattleAnimArgs[1];
    (*sprite).callback = Some(AnimMonMoveCircular_Step);
    gSprites[(*sprite).data[5]].y += 8;
}
pub(crate) unsafe fn AnimMonMoveCircular_Step(sprite: *mut Sprite) {
    if (*sprite).data[3] != 0 {
        (*sprite).data[3] -= 1;
        gSprites[(*sprite).data[5]].x2 = Sin((*sprite).data[0], (*sprite).data[1]);
        gSprites[(*sprite).data[5]].y2 = Cos((*sprite).data[0], (*sprite).data[1]);
        (*sprite).data[0] += (*sprite).data[2];
        if (*sprite).data[0] > 255 {
            (*sprite).data[0] -= 256;
        }
    } else {
        gSprites[(*sprite).data[5]].x2 = 0;
        gSprites[(*sprite).data[5]].y2 = 0;
        gSprites[(*sprite).data[5]].y -= 8;
        (*sprite).callback = Some(DestroySpriteAndMatrix);
    }
}
