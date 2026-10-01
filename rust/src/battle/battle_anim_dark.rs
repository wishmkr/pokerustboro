//! Translated from `src/battle_anim_dark.c` by tools/rustport/c2rs.py.
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
    clippy::self_assignment,
    dead_code,
    unused_assignments
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible, IsContest,
    MoveBattlerSpriteToBG, ResetBattleAnimBg, gAnimMoveTurn, gBattleAnimAttacker,
    gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimLoadCompressedBgGfx, AnimLoadCompressedBgTilemap, ClearBattleAnimBg,
    CreateInvisibleSpriteCopy, DestroySpriteAndMatrix, GetAnimBattlerSpriteId,
    GetBattleAnimBg1Data, GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide,
    GetBattlerSpriteBGPriorityRank, GetBattlerSpriteCoord, GetBattlerSpriteCoordAttr,
    InitAnimArcTranslation, InitSpriteDataForLinearTranslation, IsDoubleBattle,
    RunStoredCallbackWhenAnimEnds, SetGrayscaleOrOriginalPalette, StoreSpriteCallbackInData6,
    TranslateAnimHorizontalArc,
};
use crate::battle_anim_utility_funcs::SetAnimBgAttribute;
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattle_WIN0H, gBattle_WIN0V,
};
use crate::battle_main::{gBattlerPartyIndexes, gBattlerSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::gContestResources;
use crate::gpu_regs::{GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::palette::{FillPalette, LoadCompressedPalette};
use crate::pokemon::{GetMonData2, gEnemyParty, gPlayerParty};
use crate::sprite::gSprites;
use crate::task::{gTasks, task_get, task_set, task_set_func};
use crate::trig::Sin;
#[allow(unused_imports)]
use crate::types::*;
use crate::util::BlendPalette;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
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
// Data tables (translate with cdata.py): sUnusedBagStealSpriteTemplate sAffineAnim_Bite_0 sAffineAnim_Bite_1 sAffineAnim_Bite_2 sAffineAnim_Bite_3 sAffineAnim_Bite_4 sAffineAnim_Bite_5 sAffineAnim_Bite_6 sAffineAnim_Bite_7 gAffineAnims_Bite gSharpTeethSpriteTemplate gClampJawSpriteTemplate sAffineAnim_TearDrop_0 sAffineAnim_TearDrop_1 sAffineAnims_TearDrop gTearDropSpriteTemplate sAnim_ClawSlash_0 sAnim_ClawSlash_1 sAnims_ClawSlash gClawSlashSpriteTemplate

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub stepDelay: i16,
}

unsafe impl Sync for Anon1 {}

/// `__anon2`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon2 {
    pub stepDelay: i16,
}

unsafe impl Sync for Anon2 {}

/// `__anon3`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon3 {
    pub x: i16,
    pub y: i16,
    pub animation: i16,
    pub xVelocity: i16,
    pub yVelocity: i16,
    pub halfDuration: i16,
}

unsafe impl Sync for Anon3 {}

/// `__anon4`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon4 {
    pub relativeTo: i16,
    pub r#type: i16,
}

unsafe impl Sync for Anon4 {}

/// `__anon5`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon5 {
    pub x: i16,
    pub y: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon5 {}

/// `__anon6`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon6 {
    pub permanent: i16,
    pub useColor: i16,
    pub color: i16,
}

unsafe impl Sync for Anon6 {}

/// `__anon7`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon7 {
    pub battler: i16,
    pub mode: i16,
}

unsafe impl Sync for Anon7 {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<Anon1>() == 2);
    assert!(offset_of!(Anon1, stepDelay) == 0);
    assert!(size_of::<Anon2>() == 2);
    assert!(offset_of!(Anon2, stepDelay) == 0);
    assert!(size_of::<Anon3>() == 12);
    assert!(offset_of!(Anon3, x) == 0);
    assert!(offset_of!(Anon3, y) == 2);
    assert!(offset_of!(Anon3, animation) == 4);
    assert!(offset_of!(Anon3, xVelocity) == 6);
    assert!(offset_of!(Anon3, yVelocity) == 8);
    assert!(offset_of!(Anon3, halfDuration) == 10);
    assert!(size_of::<Anon4>() == 4);
    assert!(offset_of!(Anon4, relativeTo) == 0);
    assert!(offset_of!(Anon4, r#type) == 2);
    assert!(size_of::<Anon5>() == 6);
    assert!(offset_of!(Anon5, x) == 0);
    assert!(offset_of!(Anon5, y) == 2);
    assert!(offset_of!(Anon5, animation) == 4);
    assert!(size_of::<Anon6>() == 6);
    assert!(offset_of!(Anon6, permanent) == 0);
    assert!(offset_of!(Anon6, useColor) == 2);
    assert!(offset_of!(Anon6, color) == 4);
    assert!(size_of::<Anon7>() == 4);
    assert!(offset_of!(Anon7, battler) == 0);
    assert!(offset_of!(Anon7, mode) == 2);
};

#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AttackerFadeToInvisible(taskId: u8) {
    let cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
    task_set(taskId, 0, (*cmd).stepDelay);
    let battler: i32 = gBattleAnimAttacker as i32;
    task_set(taskId, 1, 16);
    SetGpuReg(REG_OFFSET_BLDALPHA, 16);
    if GetBattlerSpriteBGPriorityRank(battler as u8) == 1 {
        SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    } else {
        SetGpuReg(REG_OFFSET_BLDCNT, 16196);
    }
    task_set_func(taskId, Some(AnimTask_AttackerFadeToInvisible_Step));
}
pub(crate) unsafe fn AnimTask_AttackerFadeToInvisible_Step(taskId: u8) {
    let mut blendA: u8 = (task_get(taskId, 1) >> 8) as u8;
    let mut blendB: u8 = task_get(taskId, 1) as u8;
    if task_get(taskId, 2) == task_get(taskId, 0) as u8 as i16 {
        blendA += 1;
        blendB -= 1;
        task_set(taskId, 1, (blendA as i16) << 8 | blendB as i16);
        SetGpuReg(REG_OFFSET_BLDALPHA, task_get(taskId, 1) as u16);
        task_set(taskId, 2, 0);
        if blendA == 16 {
            gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].set_invisible(TRUE as u16);
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 2, task_get(taskId, 2) + 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AttackerFadeFromInvisible(taskId: u8) {
    let cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
    task_set(taskId, 0, (*cmd).stepDelay);
    task_set(taskId, 1, 4096);
    task_set_func(taskId, Some(AnimTask_AttackerFadeFromInvisible_Step));
    SetGpuReg(REG_OFFSET_BLDALPHA, task_get(taskId, 1) as u16);
}
pub(crate) unsafe fn AnimTask_AttackerFadeFromInvisible_Step(taskId: u8) {
    let mut blendA: u8 = (task_get(taskId, 1) >> 8) as u8;
    let mut blendB: u8 = task_get(taskId, 1) as u8;
    if task_get(taskId, 2) == task_get(taskId, 0) as u8 as i16 {
        blendA -= 1;
        blendB += 1;
        task_set(taskId, 1, (blendA as i16) << 8 | blendB as i16);
        SetGpuReg(REG_OFFSET_BLDALPHA, task_get(taskId, 1) as u16);
        task_set(taskId, 2, 0);
        if blendA == 0 {
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 2, task_get(taskId, 2) + 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_InitAttackerFadeFromInvisible(taskId: u8) {
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
        SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    } else {
        SetGpuReg(REG_OFFSET_BLDCNT, 16196);
    }
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimUnusedBagSteal(sprite: *mut Sprite) {
    (*sprite).data[1] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = 0x7E;
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[3] = -(*sprite).data[1];
    (*sprite).data[4] = -(*sprite).data[2];
    (*sprite).data[6] = -40;
    (*sprite).callback = Some(AnimUnusedBagSteal_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimUnusedBagSteal_Step(sprite: *mut Sprite) {
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).x2 = (*sprite).data[3] >> 8;
    (*sprite).y2 = (*sprite).data[4] >> 8;
    if (*sprite).data[7] == 0 {
        (*sprite).data[3] += (*sprite).data[1];
        (*sprite).data[4] += (*sprite).data[2];
        (*sprite).x2 = (*sprite).data[3] >> 8;
        (*sprite).y2 = (*sprite).data[4] >> 8;
        (*sprite).data[0] -= 1;
    }
    (*sprite).y2 += Sin((*sprite).data[5], (*sprite).data[6]);
    (*sprite).data[5] = ((*sprite).data[5] + 3) & 0xFF;
    if (*sprite).data[5] > 0x7F {
        (*sprite).data[5] = 0;
        (*sprite).data[6] += 20;
        (*sprite).data[7] += 1;
    }
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == 0
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimBite(sprite: *mut Sprite) {
    let cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
    (*sprite).x += (*cmd).x;
    (*sprite).y += (*cmd).y;
    StartSpriteAffineAnim(sprite, (*cmd).animation as u8);
    (*sprite).data[0] = (*cmd).xVelocity;
    (*sprite).data[1] = (*cmd).yVelocity;
    (*sprite).data[2] = (*cmd).halfDuration;
    (*sprite).callback = Some(AnimBite_Step1);
}
pub(crate) unsafe fn AnimBite_Step1(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[0];
    (*sprite).data[5] += (*sprite).data[1];
    (*sprite).x2 = (*sprite).data[4] >> 8;
    (*sprite).y2 = (*sprite).data[5] >> 8;
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == (*sprite).data[2]
    {
        (*sprite).callback = Some(AnimBite_Step2);
    }
}
pub(crate) unsafe fn AnimBite_Step2(sprite: *mut Sprite) {
    (*sprite).data[4] -= (*sprite).data[0];
    (*sprite).data[5] -= (*sprite).data[1];
    (*sprite).x2 = (*sprite).data[4] >> 8;
    (*sprite).y2 = (*sprite).data[5] >> 8;
    if ({
        (*sprite).data[3] -= 1;
        (*sprite).data[3]
    }) == 0
    {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe fn AnimTearDrop(sprite: *mut Sprite) {
    let cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
    let mut battler: u8 = 0;
    if (*cmd).relativeTo == ANIM_ATTACKER as i16 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    let mut xOffset: i8 = 20;
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 4);
    match (*cmd).r#type {
        0 => {
            (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_RIGHT) - 8;
            (*sprite).y = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_TOP) + 8;
        }
        1 => {
            (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_RIGHT) - 14;
            (*sprite).y = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_TOP) + 16;
        }
        2 => {
            (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_LEFT) + 8;
            (*sprite).y = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_TOP) + 8;
            StartSpriteAffineAnim(sprite, 1);
            xOffset = -20;
        }
        3 => {
            (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_LEFT) + 14;
            (*sprite).y = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_TOP) + 16;
            StartSpriteAffineAnim(sprite, 1);
            xOffset = -20;
        }
        _ => {}
    }
    (*sprite).data[0] = 32;
    (*sprite).data[2] = (*sprite).x + xOffset as i16;
    (*sprite).data[4] = (*sprite).y + 12;
    (*sprite).data[5] = -12;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimTearDrop_Step);
}
pub(crate) unsafe fn AnimTearDrop_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroySpriteAndMatrix(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MoveAttackerMementoShadow(taskId: u8) {
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let mut animBg: BattleAnimBgData = zeroed();
    let mut var0: i32 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[7] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + 31;
    (*task).data[6] = GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_TOP) - 7;
    (*task).data[5] = (*task).data[7];
    (*task).data[4] = (*task).data[6];
    (*task).data[13] = ((*task).data[7] - (*task).data[6]) << 8;
    let pos: u8 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X);
    (*task).data[14] = pos as i16 - 32;
    (*task).data[15] = pos as i16 + 32;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*task).data[8] = -12;
    } else {
        (*task).data[8] = -64;
    }
    (*task).data[3] = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) as i16;
    if (*task).data[3] == 1 {
        GetBattleAnimBg1Data(&raw mut animBg);
        (*task).data[10] = gBattle_BG1_Y as i16;
        SetGpuReg(REG_OFFSET_BLDCNT, 16194);
        FillPalette(0, animBg.paletteId as u16 * 16, 32);
        scanlineParams.dmaDest = 67108886_usize as *mut u16 as *mut c_void;
        var0 = WINOUT_WIN01_BG1;
        if IsContest() == 0 {
            gBattle_BG2_X += DISPLAY_WIDTH;
        }
    } else {
        (*task).data[10] = gBattle_BG2_Y as i16;
        SetGpuReg(REG_OFFSET_BLDCNT, 16196);
        FillPalette(0, 144, 32);
        scanlineParams.dmaDest = 67108890_usize as *mut u16 as *mut c_void;
        var0 = WINOUT_WIN01_BG2;
        if IsContest() == 0 {
            gBattle_BG1_X += DISPLAY_WIDTH;
        }
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    (*task).data[11] = 0;
    (*task).data[12] = 16;
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    SetAllBattlersSpritePriority(3);
    for i in 0..112u16 {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = (*task).data[10] as u16;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = (*task).data[10] as u16;
    }
    ScanlineEffect_SetParams(scanlineParams);
    SetGpuReg(REG_OFFSET_WINOUT, 16128 | var0 as u16 ^ 63);
    SetGpuReg(REG_OFFSET_WININ, 16191);
    gBattle_WIN0H = ((*task).data[14] as u16) << 8 | (*task).data[15] as u16;
    gBattle_WIN0V = DISPLAY_HEIGHT;
    (*task).func = Some(AnimTask_MoveAttackerMementoShadow_Step);
}
pub(crate) unsafe fn AnimTask_MoveAttackerMementoShadow_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
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
                    if (*task).data[11] != 12 {
                        (*task).data[11] += 1;
                    }
                } else {
                    if (*task).data[12] != 8 {
                        (*task).data[12] -= 1;
                    }
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*task).data[12] as u16) << 8 | (*task).data[11] as u16,
                );
                if (*task).data[11] == 12 && (*task).data[12] == 8 {
                    (*task).data[0] += 1;
                }
            }
        }
        1 => {
            (*task).data[4] -= 8;
            DoMementoShadowEffect(task);
            if (*task).data[4] < (*task).data[8] {
                (*task).data[0] += 1;
            }
        }
        2 => {
            (*task).data[4] -= 8;
            DoMementoShadowEffect(task);
            (*task).data[14] += 4;
            (*task).data[15] -= 4;
            if (*task).data[14] >= (*task).data[15] {
                (*task).data[14] = (*task).data[15];
            }
            gBattle_WIN0H = ((*task).data[14] as u16) << 8 | (*task).data[15] as u16;
            if (*task).data[14] == (*task).data[15] {
                (*task).data[0] += 1;
            }
        }
        3 => {
            (*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .state = 3;
            (*task).data[0] += 1;
        }
        4 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MoveTargetMementoShadow(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let mut x: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if IsContest() == TRUE {
                gBattle_WIN0H = 0;
                gBattle_WIN0V = 0;
                SetGpuReg(REG_OFFSET_WININ, 16191);
                SetGpuReg(REG_OFFSET_WINOUT, 16191);
                DestroyAnimVisualTask(taskId);
            } else {
                (*task).data[3] = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget) as i16;
                if (*task).data[3] == 1 {
                    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
                    gBattle_BG2_X += DISPLAY_WIDTH;
                } else {
                    SetGpuReg(REG_OFFSET_BLDCNT, 16196);
                    gBattle_BG1_X += DISPLAY_WIDTH;
                }
                (*task).data[0] += 1;
            }
        }
        1 => {
            if (*task).data[3] == 1 {
                GetBattleAnimBg1Data(&raw mut animBg);
                (*task).data[10] = gBattle_BG1_Y as i16;
                FillPalette(0, animBg.paletteId as u16 * 16, 32);
            } else {
                (*task).data[10] = gBattle_BG2_Y as i16;
                FillPalette(0, 144, 32);
            }
            SetAllBattlersSpritePriority(3);
            (*task).data[0] += 1;
        }
        2 => {
            (*task).data[7] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + 31;
            (*task).data[6] =
                GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_TOP) - 7;
            (*task).data[13] = ((*task).data[7] - (*task).data[6]) << 8;
            x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X);
            (*task).data[14] = x as i16 - 4;
            (*task).data[15] = x as i16 + 4;
            if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
                (*task).data[8] = -12;
            } else {
                (*task).data[8] = -64;
            }
            (*task).data[4] = (*task).data[8];
            (*task).data[5] = (*task).data[8];
            (*task).data[11] = 12;
            (*task).data[12] = 8;
            (*task).data[0] += 1;
        }
        3 => {
            if (*task).data[3] == 1 {
                scanlineParams.dmaDest = 67108886_usize as *mut u16 as *mut c_void;
            } else {
                scanlineParams.dmaDest = 67108890_usize as *mut u16 as *mut c_void;
            }
            for i in 0..112u16 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0][i] = (*task).data[10] as u16 + (159 - i);
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[1][i] = (*task).data[10] as u16 + (159 - i);
            }
            scanlineParams.dmaControl = 0xa2600001;
            scanlineParams.initState = 1;
            scanlineParams.unused9 = 0;
            ScanlineEffect_SetParams(scanlineParams);
            (*task).data[0] += 1;
        }
        4 => {
            if (*task).data[3] == 1 {
                SetGpuReg(REG_OFFSET_WINOUT, 16189);
            } else {
                SetGpuReg(REG_OFFSET_WINOUT, 16187);
            }
            SetGpuReg(REG_OFFSET_WININ, 16191);
            gBattle_WIN0H = ((*task).data[14] as u16) << 8 | (*task).data[15] as u16;
            gBattle_WIN0V = DISPLAY_HEIGHT;
            (*task).data[0] = 0;
            (*task).data[1] = 0;
            (*task).data[2] = 0;
            SetGpuReg(REG_OFFSET_BLDALPHA, 2060);
            (*task).func = Some(AnimTask_MoveTargetMementoShadow_Step);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimTask_MoveTargetMementoShadow_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[5] += 8;
            if (*task).data[5] >= (*task).data[7] {
                (*task).data[5] = (*task).data[7];
            }
            DoMementoShadowEffect(task);
            if (*task).data[5] == (*task).data[7] {
                (*task).data[0] += 1;
            }
        }
        1 => {
            if ((*task).data[15] as i32 - (*task).data[14] as i32) < 0x40 {
                (*task).data[14] -= 4;
                (*task).data[15] += 4;
            } else {
                (*task).data[1] = 1;
            }
            gBattle_WIN0H = ((*task).data[14] as u16) << 8 | (*task).data[15] as u16;
            (*task).data[4] += 8;
            if (*task).data[4] >= (*task).data[6] {
                (*task).data[4] = (*task).data[6];
            }
            DoMementoShadowEffect(task);
            if (*task).data[4] == (*task).data[6] && (*task).data[1] != 0 {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                (*task).data[2] += 1;
                if (*task).data[2] as i32 & 1 != 0 {
                    if (*task).data[11] != 0 {
                        (*task).data[11] -= 1;
                    }
                } else {
                    if (*task).data[12] < 16 {
                        (*task).data[12] += 1;
                    }
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*task).data[12] as u16) << 8 | (*task).data[11] as u16,
                );
                if (*task).data[11] == 0 && (*task).data[12] == 16 {
                    (*task).data[0] += 1;
                }
            }
        }
        3 => {
            (*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .state = 3;
            (*task).data[0] += 1;
        }
        4 => {
            gBattle_WIN0H = 0;
            gBattle_WIN0V = 0;
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_WINOUT, 16191);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn DoMementoShadowEffect(task: *mut Task) {
    let mut var0: i32 = 0;
    let mut var1: i32 = 0;
    let mut i: i16 = 0;
    let mut var4: i32 = 0;
    let var2: i16 = (*task).data[5] - (*task).data[4];
    if var2 != 0 {
        var0 = div_i32((*task).data[13] as i32, var2 as i32);
        var1 = ((*task).data[6] as i32) << 8;
        for i in 0..(*task).data[4] {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .srcBuffer][i] = (*task).data[10] as u16 - (i as u16 - 159);
        }
        i = (*task).data[4];
        while i <= (*task).data[5] {
            if i >= 0 {
                let var3: i16 = (var1 >> 8) as i16 - i;
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = var3 as u16 + (*task).data[10] as u16;
            }
            var1 += var0;
            i += 1;
        }
        var4 = (*task).data[10] as i32 - (i as i32 - 159);
        i = i;
        while i < (*task).data[7] {
            if i >= 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][i] = var4 as u16;
                var4 -= 1;
            }
            i += 1;
        }
    } else {
        var4 = (*task).data[10] as i32 + 159;
        for i in 0..112i16 {
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[0][i] = var4 as u16;
            (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                .cast::<CArray<CArray<u16, 960>, 2>>()
                .cast_mut())[1][i] = var4 as u16;
            var4 -= 1;
        }
    }
}
unsafe fn SetAllBattlersSpritePriority(priority: u8) {
    for i in 0..(MAX_BATTLERS_COUNT as u16) {
        let spriteId: u8 = GetAnimBattlerSpriteId(i as u8);
        if spriteId != SPRITE_NONE {
            gSprites[spriteId].oam.set_priority(priority as u16);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_InitMementoShadow(taskId: u8) {
    let toBG2: u8 = (if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) as i32 ^ 1 != 0 {
        1
    } else {
        0
    }) as u8;
    MoveBattlerSpriteToBG(gBattleAnimAttacker, toBG2, TRUE);
    gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].set_invisible(FALSE as u16);
    if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) != 0 {
        MoveBattlerSpriteToBG(gBattleAnimAttacker ^ 2, toBG2 ^ 1, 1);
        gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]].set_invisible(FALSE as u16);
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MementoHandleBg(taskId: u8) {
    let toBG2: u8 = (if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) as i32 ^ 1 != 0 {
        1
    } else {
        FALSE as i32
    }) as u8;
    ResetBattleAnimBg(toBG2);
    if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) != 0 {
        ResetBattleAnimBg(toBG2 ^ 1);
    }
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimClawSlash(sprite: *mut Sprite) {
    let cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    (*sprite).x += (*cmd).x;
    (*sprite).y += (*cmd).y;
    StartSpriteAnim(sprite, (*cmd).animation as u8);
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MetallicShine(taskId: u8) {
    let cmd: *mut Anon6 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon6;
    let mut species: u16 = 0;
    let mut paletteNum: u16 = 0;
    let mut animBg: BattleAnimBgData = zeroed();
    let mut priorityChanged: u32 = FALSE as u32;
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 3080);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 0);
    SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
    if IsContest() == 0 {
        SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
    }
    if IsDoubleBattle() != 0
        && IsContest() == 0
        && (GetBattlerPosition(gBattleAnimAttacker) == B_POSITION_OPPONENT_RIGHT
            || GetBattlerPosition(gBattleAnimAttacker) == B_POSITION_PLAYER_LEFT)
        && IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) == TRUE
    {
        gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
            .oam
            .set_priority(
                gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                    .oam
                    .priority()
                    - 1,
            );
        SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        priorityChanged = TRUE as u32;
    }
    if IsContest() != 0 {
        species = (*(*gContestResources).moveAnim).species;
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            species = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                MON_DATA_SPECIES,
            ) as u16;
        } else {
            species = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                MON_DATA_SPECIES,
            ) as u16;
        }
    }
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    let newSpriteId: u8 =
        CreateInvisibleSpriteCopy(gBattleAnimAttacker as i32, spriteId, species as i32);
    GetBattleAnimBg1Data(&raw mut animBg);
    AnimLoadCompressedBgTilemap(
        animBg.bgId as u32,
        (*(&raw const crate::data::graphics::gMetalShineTilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut() as *mut c_void,
    );
    AnimLoadCompressedBgGfx(
        animBg.bgId as u32,
        (*(&raw const crate::data::graphics::gMetalShineGfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBg.tilesOffset as u32,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gMetalShinePalette).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBg.paletteId as u16 * 16,
        32,
    );
    gBattle_BG1_X = (gSprites[spriteId].x as u16).wrapping_neg() + 96;
    gBattle_BG1_Y = (gSprites[spriteId].y as u16).wrapping_neg() + 32;
    paletteNum = 16 + gSprites[spriteId].oam.paletteNum();
    if (*cmd).useColor == 0 {
        SetGrayscaleOrOriginalPalette(paletteNum, FALSE);
    } else {
        BlendPalette(paletteNum * 16, 16, 11, (*cmd).color as u16);
    }
    task_set(taskId, 0, newSpriteId as i16);
    task_set(taskId, 1, (*cmd).permanent);
    task_set(taskId, 2, (*cmd).useColor);
    task_set(taskId, 3, (*cmd).color);
    task_set(taskId, 6, priorityChanged as i16);
    task_set_func(taskId, Some(AnimTask_MetallicShine_Step));
}
pub(crate) unsafe fn AnimTask_MetallicShine_Step(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    let mut paletteNum: u16 = 0;
    let mut spriteId: u8 = 0;
    task_set(taskId, 10, task_get(taskId, 10) + 4);
    gBattle_BG1_X -= 4;
    if task_get(taskId, 10) == 128 {
        task_set(taskId, 10, 0);
        gBattle_BG1_X += 128;
        task_set(taskId, 11, task_get(taskId, 11) + 1);
        if task_get(taskId, 11) == 2 {
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
            paletteNum = 16 + gSprites[spriteId].oam.paletteNum();
            if task_get(taskId, 1) == 0 {
                SetGrayscaleOrOriginalPalette(paletteNum, TRUE);
            }
            DestroySprite(&raw mut gSprites[task_get(taskId, 0)]);
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(animBg.bgId as u32);
            if task_get(taskId, 6) == 1 {
                gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                    .oam
                    .set_priority(
                        gSprites[gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2]]
                            .oam
                            .priority()
                            + 1,
                    );
            }
        } else if task_get(taskId, 11) == 3 {
            gBattle_WIN0H = 0;
            gBattle_WIN0V = 0;
            SetGpuReg(REG_OFFSET_WININ, 16191);
            SetGpuReg(REG_OFFSET_WINOUT, 16191);
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            SetGpuReg(
                REG_OFFSET_DISPCNT,
                GetGpuReg(REG_OFFSET_DISPCNT) ^ DISPCNT_OBJWIN_ON,
            );
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetGrayscaleOrOriginalPal(taskId: u8) {
    let cmd: *mut Anon7 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon7;
    let mut spriteId: u8 = 0;
    let mut battler: u8 = 0;
    let mut calcSpriteId: u8 = FALSE;
    let mut position: u8 = B_POSITION_PLAYER_LEFT;
    match (*cmd).battler {
        0..=3 => {
            spriteId = GetAnimBattlerSpriteId((*cmd).battler as u8);
        }
        4 => {
            position = B_POSITION_PLAYER_LEFT;
            calcSpriteId = TRUE;
        }
        5 => {
            position = B_POSITION_PLAYER_RIGHT;
            calcSpriteId = TRUE;
        }
        6 => {
            position = B_POSITION_OPPONENT_LEFT;
            calcSpriteId = TRUE;
        }
        7 => {
            position = B_POSITION_OPPONENT_RIGHT;
            calcSpriteId = TRUE;
        }
        _ => {
            spriteId = SPRITE_NONE;
        }
    }
    if calcSpriteId != 0 {
        battler = GetBattlerAtPosition(position);
        if IsBattlerSpriteVisible(battler) != 0 {
            spriteId = gBattlerSpriteIds[battler];
        } else {
            spriteId = SPRITE_NONE;
        }
    }
    if spriteId != SPRITE_NONE {
        SetGrayscaleOrOriginalPalette(gSprites[spriteId].oam.paletteNum() + 16, (*cmd).mode as u8);
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn GetIsDoomDesireHitTurn(taskId: u8) {
    if gAnimMoveTurn < 2 {
        gBattleAnimArgs[7] = FALSE as i16;
    }
    if gAnimMoveTurn == 2 {
        gBattleAnimArgs[7] = TRUE as i16;
    }
    DestroyAnimVisualTask(taskId);
}
