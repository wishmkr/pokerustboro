//! Translated from `src/battle_anim_mon_movement.c` by tools/rustport/c2rs.py.
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
    DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible, IsContest, gAnimMoveDmg,
    gAnimMovePower, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    GetAnimBattlerSpriteId, GetBattlerAtPosition, GetBattlerSide,
    InitSpriteDataForLinearTranslation, PrepareBattlerSpriteForRotScale, ResetSpriteRotScale,
    SetBattlerSpriteYOffsetFromRotation, SetSpriteRotScale, StoreSpriteCallbackInData6,
    TranslateSpriteLinearById, TranslateSpriteLinearByIdFixedPoint,
};
use crate::battle_main::gBattlerSpriteIds;
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::sprite::gSprites;
use crate::task::gTasks;
use crate::task::{task_func, task_get, task_set, task_set_func};
use crate::trig::{Cos, Sin};
#[allow(unused_imports)]
use crate::types::*;
#[allow(unused_imports)]
use core::ffi::c_void;
#[allow(unused_imports)]
use core::mem::zeroed;
#[allow(unused_imports)]
use core::ptr::null_mut;
// Data tables (translate with cdata.py): gHorizontalLungeSpriteTemplate gVerticalDipSpriteTemplate gSlideMonToOriginalPosSpriteTemplate gSlideMonToOffsetSpriteTemplate gSlideMonToOffsetAndBackSpriteTemplate

#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeMon(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    if spriteId == SPRITE_NONE {
        DestroyAnimVisualTask(taskId);
        return;
    }
    gSprites[spriteId].x2 = gBattleAnimArgs[1];
    gSprites[spriteId].y2 = gBattleAnimArgs[2];
    task_set(taskId, 0, spriteId as i16);
    task_set(taskId, 1, gBattleAnimArgs[3]);
    task_set(taskId, 2, gBattleAnimArgs[4]);
    task_set(taskId, 3, gBattleAnimArgs[4]);
    task_set(taskId, 4, gBattleAnimArgs[1]);
    task_set(taskId, 5, gBattleAnimArgs[2]);
    task_set_func(taskId, Some(AnimTask_ShakeMon_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_ShakeMon_Step(taskId: u8) {
    if task_get(taskId, 3) == 0 {
        if gSprites[task_get(taskId, 0)].x2 == 0 {
            gSprites[task_get(taskId, 0)].x2 = task_get(taskId, 4);
        } else {
            gSprites[task_get(taskId, 0)].x2 = 0;
        }
        if gSprites[task_get(taskId, 0)].y2 == 0 {
            gSprites[task_get(taskId, 0)].y2 = task_get(taskId, 5);
        } else {
            gSprites[task_get(taskId, 0)].y2 = 0;
        }
        task_set(taskId, 3, task_get(taskId, 2));
        if ({
            task_set(taskId, 1, task_get(taskId, 1) - 1);
            task_get(taskId, 1)
        }) == 0
        {
            gSprites[task_get(taskId, 0)].x2 = 0;
            gSprites[task_get(taskId, 0)].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 3, task_get(taskId, 3) - 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeMon2(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut abort: u8 = FALSE;
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] < MAX_BATTLERS_COUNT as i16 {
        spriteId = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
        if spriteId == SPRITE_NONE {
            abort = TRUE;
        }
    } else if gBattleAnimArgs[0] != 8 {
        match gBattleAnimArgs[0] {
            4 => {
                battler = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            }
            5 => {
                battler = GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT);
            }
            6 => {
                battler = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            }
            _ => {
                battler = GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT);
            }
        }
        if IsBattlerSpriteVisible(battler) == FALSE {
            abort = TRUE;
        }
        spriteId = gBattlerSpriteIds[battler];
    } else {
        spriteId = gBattlerSpriteIds[gBattleAnimAttacker];
    }
    if abort != 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    gSprites[spriteId].x2 = gBattleAnimArgs[1];
    gSprites[spriteId].y2 = gBattleAnimArgs[2];
    task_set(taskId, 0, spriteId as i16);
    task_set(taskId, 1, gBattleAnimArgs[3]);
    task_set(taskId, 2, gBattleAnimArgs[4]);
    task_set(taskId, 3, gBattleAnimArgs[4]);
    task_set(taskId, 4, gBattleAnimArgs[1]);
    task_set(taskId, 5, gBattleAnimArgs[2]);
    task_set_func(taskId, Some(AnimTask_ShakeMon2_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_ShakeMon2_Step(taskId: u8) {
    if task_get(taskId, 3) == 0 {
        if gSprites[task_get(taskId, 0)].x2 == task_get(taskId, 4) {
            gSprites[task_get(taskId, 0)].x2 = -task_get(taskId, 4);
        } else {
            gSprites[task_get(taskId, 0)].x2 = task_get(taskId, 4);
        }
        if gSprites[task_get(taskId, 0)].y2 == task_get(taskId, 5) {
            gSprites[task_get(taskId, 0)].y2 = -task_get(taskId, 5);
        } else {
            gSprites[task_get(taskId, 0)].y2 = task_get(taskId, 5);
        }
        task_set(taskId, 3, task_get(taskId, 2));
        if ({
            task_set(taskId, 1, task_get(taskId, 1) - 1);
            task_get(taskId, 1)
        }) == 0
        {
            gSprites[task_get(taskId, 0)].x2 = 0;
            gSprites[task_get(taskId, 0)].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 3, task_get(taskId, 3) - 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeMonInPlace(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    if spriteId == SPRITE_NONE {
        DestroyAnimVisualTask(taskId);
        return;
    }
    gSprites[spriteId].x2 += gBattleAnimArgs[1];
    gSprites[spriteId].y2 += gBattleAnimArgs[2];
    task_set(taskId, 0, spriteId as i16);
    task_set(taskId, 1, 0);
    task_set(taskId, 2, gBattleAnimArgs[3]);
    task_set(taskId, 3, 0);
    task_set(taskId, 4, gBattleAnimArgs[4]);
    task_set(taskId, 5, gBattleAnimArgs[1] * 2);
    task_set(taskId, 6, gBattleAnimArgs[2] * 2);
    task_set_func(taskId, Some(AnimTask_ShakeMonInPlace_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_ShakeMonInPlace_Step(taskId: u8) {
    if task_get(taskId, 3) == 0 {
        if task_get(taskId, 1) as i32 & 1 != 0 {
            gSprites[task_get(taskId, 0)].x2 += task_get(taskId, 5);
            gSprites[task_get(taskId, 0)].y2 += task_get(taskId, 6);
        } else {
            gSprites[task_get(taskId, 0)].x2 -= task_get(taskId, 5);
            gSprites[task_get(taskId, 0)].y2 -= task_get(taskId, 6);
        }
        task_set(taskId, 3, task_get(taskId, 4));
        if ({
            task_set(taskId, 1, task_get(taskId, 1) + 1);
            task_get(taskId, 1)
        }) >= task_get(taskId, 2)
        {
            if task_get(taskId, 1) as i32 & 1 != 0 {
                gSprites[task_get(taskId, 0)].x2 += task_get(taskId, 5) / 2;
                gSprites[task_get(taskId, 0)].y2 += task_get(taskId, 6) / 2;
            } else {
                gSprites[task_get(taskId, 0)].x2 -= task_get(taskId, 5) / 2;
                gSprites[task_get(taskId, 0)].y2 -= task_get(taskId, 6) / 2;
            }
            DestroyAnimVisualTask(taskId);
        }
    } else {
        task_set(taskId, 3, task_get(taskId, 3) - 1);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeAndSinkMon(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    gSprites[spriteId].x2 = gBattleAnimArgs[1];
    task_set(taskId, 0, spriteId as i16);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    task_set(taskId, 2, gBattleAnimArgs[2]);
    task_set(taskId, 3, gBattleAnimArgs[3]);
    task_set(taskId, 4, gBattleAnimArgs[4]);
    task_set_func(taskId, Some(AnimTask_ShakeAndSinkMon_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_ShakeAndSinkMon_Step(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 0) as u8;
    let mut x: i16 = task_get(taskId, 1);
    if task_get(taskId, 2)
        == ({
            let t1 = task_get(taskId, 8);
            task_set(taskId, 8, task_get(taskId, 8) + 1);
            t1
        })
    {
        task_set(taskId, 8, 0);
        if gSprites[spriteId].x2 == x {
            x = -x;
        }
        gSprites[spriteId].x2 += x;
    }
    task_set(taskId, 1, x);
    task_set(taskId, 9, task_get(taskId, 9) + (task_get(taskId, 3)));
    gSprites[spriteId].y2 = task_get(taskId, 9) >> 8;
    if ({
        task_set(taskId, 4, task_get(taskId, 4) - 1);
        task_get(taskId, 4)
    }) == 0
    {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_TranslateMonElliptical(taskId: u8) {
    let mut wavePeriod: u8 = 1;
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    if gBattleAnimArgs[4] > 5 {
        gBattleAnimArgs[4] = 5;
    }
    let mut i: u8 = 0;
    while (i as i16) < gBattleAnimArgs[4] {
        wavePeriod <<= 1;
        i += 1;
    }
    task_set(taskId, 0, spriteId as i16);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    task_set(taskId, 2, gBattleAnimArgs[2]);
    task_set(taskId, 3, gBattleAnimArgs[3]);
    task_set(taskId, 4, wavePeriod as i16);
    task_set_func(taskId, Some(AnimTask_TranslateMonElliptical_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_TranslateMonElliptical_Step(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 0) as u8;
    gSprites[spriteId].x2 = Sin(task_get(taskId, 5), task_get(taskId, 1));
    gSprites[spriteId].y2 = -Cos(task_get(taskId, 5), task_get(taskId, 2));
    gSprites[spriteId].y2 += task_get(taskId, 2);
    task_set(taskId, 5, task_get(taskId, 5) + (task_get(taskId, 4)));
    task_set(taskId, 5, task_get(taskId, 5) & 0xff);
    if task_get(taskId, 5) == 0 {
        task_set(taskId, 3, task_get(taskId, 3) - 1);
    }
    if task_get(taskId, 3) == 0 {
        gSprites[spriteId].x2 = 0;
        gSprites[spriteId].y2 = 0;
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_TranslateMonEllipticalRespectSide(taskId: u8) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
    }
    AnimTask_TranslateMonElliptical(taskId);
}
pub(crate) unsafe fn DoHorizontalLunge(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).data[1] = -gBattleAnimArgs[1];
    } else {
        (*sprite).data[1] = gBattleAnimArgs[1];
    }
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[2] = 0;
    (*sprite).data[3] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
    (*sprite).data[4] = gBattleAnimArgs[0];
    StoreSpriteCallbackInData6(sprite, Some(ReverseHorizontalLungeDirection));
    (*sprite).callback = Some(TranslateSpriteLinearById);
}
pub(crate) unsafe fn ReverseHorizontalLungeDirection(sprite: *mut Sprite) {
    (*sprite).data[0] = (*sprite).data[4];
    (*sprite).data[1] = -(*sprite).data[1];
    (*sprite).callback = Some(TranslateSpriteLinearById);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn DoVerticalDip(sprite: *mut Sprite) {
    (*sprite).set_invisible(TRUE as u16);
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[2] as u8);
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = 0;
    (*sprite).data[2] = gBattleAnimArgs[1];
    (*sprite).data[3] = spriteId as i16;
    (*sprite).data[4] = gBattleAnimArgs[0];
    StoreSpriteCallbackInData6(sprite, Some(ReverseVerticalDipDirection));
    (*sprite).callback = Some(TranslateSpriteLinearById);
}
pub(crate) unsafe fn ReverseVerticalDipDirection(sprite: *mut Sprite) {
    (*sprite).data[0] = (*sprite).data[4];
    (*sprite).data[2] = -(*sprite).data[2];
    (*sprite).callback = Some(TranslateSpriteLinearById);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn SlideMonToOriginalPos(sprite: *mut Sprite) {
    let mut monSpriteId: u32 = 0;
    if gBattleAnimArgs[0] == 0 {
        monSpriteId = gBattlerSpriteIds[gBattleAnimAttacker] as u32;
    } else {
        monSpriteId = gBattlerSpriteIds[gBattleAnimTarget] as u32;
    }
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[1] = gSprites[monSpriteId].x + gSprites[monSpriteId].x2;
    (*sprite).data[2] = gSprites[monSpriteId].x;
    (*sprite).data[3] = gSprites[monSpriteId].y + gSprites[monSpriteId].y2;
    (*sprite).data[4] = gSprites[monSpriteId].y;
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = gSprites[monSpriteId].x2;
    (*sprite).data[6] = gSprites[monSpriteId].y2;
    (*sprite).set_invisible(TRUE as u16);
    if gBattleAnimArgs[1] == 1 {
        (*sprite).data[2] = 0;
    } else if gBattleAnimArgs[1] == 2 {
        (*sprite).data[1] = 0;
    }
    (*sprite).data[7] = gBattleAnimArgs[1];
    (*sprite).data[7] |= (monSpriteId as i16) << 8;
    (*sprite).callback = Some(SlideMonToOriginalPos_Step);
}
pub(crate) unsafe fn SlideMonToOriginalPos_Step(sprite: *mut Sprite) {
    let lo: u8 = (*sprite).data[7] as u8;
    let monSpriteId: i8 = ((*sprite).data[7] >> 8) as i8;
    let monSprite: *mut Sprite = &raw mut gSprites[monSpriteId];
    if (*sprite).data[0] == 0 {
        if lo < 2 {
            (*monSprite).x2 = 0;
        }
        if lo == 2 || lo == 0 {
            (*monSprite).y2 = 0;
        }
        DestroyAnimSprite(sprite);
    } else {
        (*sprite).data[0] -= 1;
        (*sprite).data[3] += (*sprite).data[1];
        (*sprite).data[4] += (*sprite).data[2];
        (*monSprite).x2 = ((*sprite).data[3] >> 8) as i8 as i16 + (*sprite).data[5];
        (*monSprite).y2 = ((*sprite).data[4] >> 8) as i8 as i16 + (*sprite).data[6];
    }
}
pub(crate) unsafe fn SlideMonToOffset(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    let monSpriteId: u8 = gBattlerSpriteIds[battler];
    if GetBattlerSide(battler) != B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        if gBattleAnimArgs[3] == 1 {
            gBattleAnimArgs[2] = -gBattleAnimArgs[2];
        }
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = gSprites[monSpriteId].x;
    (*sprite).data[2] = gSprites[monSpriteId].x + gBattleAnimArgs[1];
    (*sprite).data[3] = gSprites[monSpriteId].y;
    (*sprite).data[4] = gSprites[monSpriteId].y + gBattleAnimArgs[2];
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = monSpriteId as i16;
    (*sprite).set_invisible(TRUE as u16);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteLinearByIdFixedPoint);
}
pub(crate) unsafe fn SlideMonToOffsetAndBack(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    (*sprite).set_invisible(TRUE as u16);
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    let spriteId: u8 = gBattlerSpriteIds[battler];
    if GetBattlerSide(battler) != 0 {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        if gBattleAnimArgs[3] == 1 {
            gBattleAnimArgs[2] = -gBattleAnimArgs[2];
        }
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[1] = gSprites[spriteId].x + gSprites[spriteId].x2;
    (*sprite).data[2] = (*sprite).data[1] + gBattleAnimArgs[1];
    (*sprite).data[3] = gSprites[spriteId].y + gSprites[spriteId].y2;
    (*sprite).data[4] = (*sprite).data[3] + gBattleAnimArgs[2];
    InitSpriteDataForLinearTranslation(sprite);
    (*sprite).data[3] = gSprites[spriteId].x2 << 8;
    (*sprite).data[4] = gSprites[spriteId].y2 << 8;
    (*sprite).data[5] = spriteId as i16;
    (*sprite).data[6] = gBattleAnimArgs[5];
    if gBattleAnimArgs[5] == 0 {
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    } else {
        StoreSpriteCallbackInData6(sprite, Some(SlideMonToOffsetAndBack_End));
    }
    (*sprite).callback = Some(TranslateSpriteLinearByIdFixedPoint);
}
pub(crate) unsafe fn SlideMonToOffsetAndBack_End(sprite: *mut Sprite) {
    gSprites[(*sprite).data[5]].x2 = 0;
    gSprites[(*sprite).data[5]].y2 = 0;
    DestroyAnimSprite(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_WindUpLunge(taskId: u8) {
    let wavePeriod: i16 = div_i32(0x8000, gBattleAnimArgs[3] as i32) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[5] = -gBattleAnimArgs[5];
    }
    task_set(
        taskId,
        0,
        GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16,
    );
    task_set(
        taskId,
        1,
        div_i32((gBattleAnimArgs[1] as i32) << 8, gBattleAnimArgs[3] as i32) as i16,
    );
    task_set(taskId, 2, gBattleAnimArgs[2]);
    task_set(taskId, 3, gBattleAnimArgs[3]);
    task_set(taskId, 4, gBattleAnimArgs[4]);
    task_set(
        taskId,
        5,
        div_i32((gBattleAnimArgs[5] as i32) << 8, gBattleAnimArgs[6] as i32) as i16,
    );
    task_set(taskId, 6, gBattleAnimArgs[6]);
    task_set(taskId, 7, wavePeriod);
    task_set_func(taskId, Some(AnimTask_WindUpLunge_Step1));
}
pub(crate) unsafe fn AnimTask_WindUpLunge_Step1(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 0) as u8;
    task_set(taskId, 11, task_get(taskId, 11) + (task_get(taskId, 1)));
    gSprites[spriteId].x2 = task_get(taskId, 11) >> 8;
    gSprites[spriteId].y2 = Sin(
        (task_get(taskId, 10) >> 8) as u8 as i16,
        task_get(taskId, 2),
    );
    task_set(taskId, 10, task_get(taskId, 10) + (task_get(taskId, 7)));
    if ({
        task_set(taskId, 3, task_get(taskId, 3) - 1);
        task_get(taskId, 3)
    }) == 0
    {
        task_set_func(taskId, Some(AnimTask_WindUpLunge_Step2));
    }
}
pub(crate) unsafe fn AnimTask_WindUpLunge_Step2(taskId: u8) {
    let mut spriteId: u8 = 0;
    if task_get(taskId, 4) > 0 {
        task_set(taskId, 4, task_get(taskId, 4) - 1);
    } else {
        spriteId = task_get(taskId, 0) as u8;
        task_set(taskId, 12, task_get(taskId, 12) + (task_get(taskId, 5)));
        gSprites[spriteId].x2 = (task_get(taskId, 12) >> 8) + (task_get(taskId, 11) >> 8);
        if ({
            task_set(taskId, 6, task_get(taskId, 6) - 1);
            task_get(taskId, 6)
        }) == 0
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SlideOffScreen(taskId: u8) {
    let mut spriteId: u8 = 0;
    match gBattleAnimArgs[0] {
        0 | 1 => {
            spriteId = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
        }
        2 => {
            if IsBattlerSpriteVisible(gBattleAnimAttacker ^ 2) == 0 {
                DestroyAnimVisualTask(taskId);
                return;
            }
            spriteId = gBattlerSpriteIds[gBattleAnimAttacker as i32 ^ 2];
        }
        3 => {
            if IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) == 0 {
                DestroyAnimVisualTask(taskId);
                return;
            }
            spriteId = gBattlerSpriteIds[gBattleAnimTarget as i32 ^ 2];
        }
        _ => {
            DestroyAnimVisualTask(taskId);
            return;
        }
    }
    task_set(taskId, 0, spriteId as i16);
    if GetBattlerSide(gBattleAnimTarget) != B_SIDE_PLAYER {
        task_set(taskId, 1, gBattleAnimArgs[1]);
    } else {
        task_set(taskId, 1, -gBattleAnimArgs[1]);
    }
    task_set_func(taskId, Some(AnimTask_SlideOffScreen_Step));
}
pub(crate) unsafe fn AnimTask_SlideOffScreen_Step(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 0) as u8;
    gSprites[spriteId].x2 += task_get(taskId, 1);
    if (gSprites[spriteId].x2 as i32 + gSprites[spriteId].x as i32) < -32
        || gSprites[spriteId].x2 as i32 + gSprites[spriteId].x as i32 > 272
    {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SwayMon(taskId: u8) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
    }
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[4] as u8);
    task_set(taskId, 0, gBattleAnimArgs[0]);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    task_set(taskId, 2, gBattleAnimArgs[2]);
    task_set(taskId, 3, gBattleAnimArgs[3]);
    task_set(taskId, 4, spriteId as i16);
    if gBattleAnimArgs[4] == 0 {
        task_set(taskId, 5, gBattleAnimAttacker as i16);
    } else {
        task_set(taskId, 5, gBattleAnimTarget as i16);
    }
    task_set(taskId, 12, 1);
    task_set_func(taskId, Some(AnimTask_SwayMonStep));
}
pub(crate) unsafe fn AnimTask_SwayMonStep(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 4) as u8;
    let sineIndex: u16 = task_get(taskId, 10) as u16 + task_get(taskId, 2) as u16;
    task_set(taskId, 10, sineIndex as i16);
    let waveIndex: i32 = (sineIndex >> 8) as i32;
    let sineValue: i16 = Sin(waveIndex as i16, task_get(taskId, 1));
    if task_get(taskId, 0) == 0 {
        gSprites[spriteId].x2 = sineValue;
    } else {
        if GetBattlerSide(task_get(taskId, 5) as u8) == B_SIDE_PLAYER {
            gSprites[spriteId].y2 = (if sineValue >= 0 {
                sineValue as i32
            } else {
                -(sineValue as i32)
            }) as i16;
        } else {
            gSprites[spriteId].y2 = (if sineValue >= 0 {
                -(sineValue as i32)
            } else {
                sineValue as i32
            }) as i16;
        }
    }
    if waveIndex >= 0x80 && task_get(taskId, 11) == 0 && task_get(taskId, 12) == 1
        || waveIndex < 0x7f && task_get(taskId, 11) == 1 && task_get(taskId, 12) == 0
    {
        task_set(taskId, 11, task_get(taskId, 11) ^ 1);
        task_set(taskId, 12, task_get(taskId, 12) ^ 1);
        if ({
            task_set(taskId, 3, task_get(taskId, 3) - 1);
            task_get(taskId, 3)
        }) == 0
        {
            gSprites[spriteId].x2 = 0;
            gSprites[spriteId].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ScaleMonAndRestore(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[3] as u8);
    PrepareBattlerSpriteForRotScale(spriteId, gBattleAnimArgs[4] as u8);
    task_set(taskId, 0, gBattleAnimArgs[0]);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    task_set(taskId, 2, gBattleAnimArgs[2]);
    task_set(taskId, 3, gBattleAnimArgs[2]);
    task_set(taskId, 4, spriteId as i16);
    task_set(taskId, 10, 0x100);
    task_set(taskId, 11, 0x100);
    task_set_func(taskId, Some(AnimTask_ScaleMonAndRestore_Step));
}
pub(crate) unsafe fn AnimTask_ScaleMonAndRestore_Step(taskId: u8) {
    task_set(taskId, 10, task_get(taskId, 10) + (task_get(taskId, 0)));
    task_set(taskId, 11, task_get(taskId, 11) + (task_get(taskId, 1)));
    let spriteId: u8 = task_get(taskId, 4) as u8;
    SetSpriteRotScale(spriteId, task_get(taskId, 10), task_get(taskId, 11), 0);
    if ({
        task_set(taskId, 2, task_get(taskId, 2) - 1);
        task_get(taskId, 2)
    }) == 0
    {
        if task_get(taskId, 3) > 0 {
            task_set(taskId, 0, -task_get(taskId, 0));
            task_set(taskId, 1, -task_get(taskId, 1));
            task_set(taskId, 2, task_get(taskId, 3));
            task_set(taskId, 3, 0);
        } else {
            ResetSpriteRotScale(spriteId);
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_RotateMonSpriteToSide(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[2] as u8);
    PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
    task_set(taskId, 1, 0);
    task_set(taskId, 2, gBattleAnimArgs[0]);
    if gBattleAnimArgs[3] != 1 {
        task_set(taskId, 3, 0);
    } else {
        task_set(taskId, 3, gBattleAnimArgs[0] * gBattleAnimArgs[1]);
    }
    task_set(taskId, 4, gBattleAnimArgs[1]);
    task_set(taskId, 5, spriteId as i16);
    task_set(taskId, 6, gBattleAnimArgs[3]);
    if IsContest() != 0 {
        task_set(taskId, 7, 1);
    } else {
        if gBattleAnimArgs[2] == ANIM_ATTACKER as i16 {
            task_set(
                taskId,
                7,
                (GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER) as i16,
            );
        } else {
            task_set(
                taskId,
                7,
                (GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER) as i16,
            );
        }
    }
    if task_get(taskId, 7) != 0 && IsContest() == 0 {
        task_set(taskId, 3, -task_get(taskId, 3));
        task_set(taskId, 4, -task_get(taskId, 4));
    }
    task_set_func(taskId, Some(AnimTask_RotateMonSpriteToSide_Step));
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_RotateMonToSideAndRestore(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[2] as u8);
    PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
    task_set(taskId, 1, 0);
    task_set(taskId, 2, gBattleAnimArgs[0]);
    if gBattleAnimArgs[2] == ANIM_ATTACKER as i16 {
        if GetBattlerSide(gBattleAnimAttacker) != 0 {
            gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        }
    } else {
        if GetBattlerSide(gBattleAnimTarget) != 0 {
            gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        }
    }
    if gBattleAnimArgs[3] != 1 {
        task_set(taskId, 3, 0);
    } else {
        task_set(taskId, 3, gBattleAnimArgs[0] * gBattleAnimArgs[1]);
    }
    task_set(taskId, 4, gBattleAnimArgs[1]);
    task_set(taskId, 5, spriteId as i16);
    task_set(taskId, 6, gBattleAnimArgs[3]);
    task_set(taskId, 7, 1);
    task_set(taskId, 3, -task_get(taskId, 3));
    task_set(taskId, 4, -task_get(taskId, 4));
    task_set_func(taskId, Some(AnimTask_RotateMonSpriteToSide_Step));
}
pub(crate) unsafe fn AnimTask_RotateMonSpriteToSide_Step(taskId: u8) {
    task_set(taskId, 3, task_get(taskId, 3) + (task_get(taskId, 4)));
    SetSpriteRotScale(
        task_get(taskId, 5) as u8,
        0x100,
        0x100,
        task_get(taskId, 3) as u16,
    );
    if task_get(taskId, 7) != 0 {
        SetBattlerSpriteYOffsetFromRotation(task_get(taskId, 5) as u8);
    }
    if ({
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        task_get(taskId, 1)
    }) >= task_get(taskId, 2)
    {
        'l1: {
            let sw2: i16 = task_get(taskId, 6);
            let matched = sw2 == 1 || sw2 == 0 || sw2 == 2;
            let mut fall = false;
            if sw2 == 1 {
                fall = true;
                ResetSpriteRotScale(task_get(taskId, 5) as u8);
            }
            if fall || sw2 == 0 || !matched {
                DestroyAnimVisualTask(taskId);
                return;
            }
            if sw2 == 2 {
                task_set(taskId, 1, 0);
                task_set(taskId, 4, -task_get(taskId, 4));
                task_set(taskId, 6, 1);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShakeTargetBasedOnMovePowerOrDmg(taskId: u8) {
    if gBattleAnimArgs[0] == 0 {
        task_set(taskId, 15, (gAnimMovePower as i32 / 12) as i16);
        if task_get(taskId, 15) < 1 {
            task_set(taskId, 15, 1);
        }
        if task_get(taskId, 15) > 16 {
            task_set(taskId, 15, 16);
        }
    } else {
        task_set(taskId, 15, (gAnimMoveDmg / 12) as i16);
        if task_get(taskId, 15) < 1 {
            task_set(taskId, 15, 1);
        }
        if task_get(taskId, 15) > 16 {
            task_set(taskId, 15, 16);
        }
    }
    task_set(taskId, 14, task_get(taskId, 15) / 2);
    task_set(
        taskId,
        13,
        task_get(taskId, 14) + (task_get(taskId, 15) & 1),
    );
    task_set(taskId, 12, 0);
    task_set(taskId, 10, gBattleAnimArgs[3]);
    task_set(taskId, 11, gBattleAnimArgs[4]);
    task_set(taskId, 7, GetAnimBattlerSpriteId(ANIM_TARGET) as i16);
    task_set(taskId, 8, gSprites[task_get(taskId, 7)].x2);
    task_set(taskId, 9, gSprites[task_get(taskId, 7)].y2);
    task_set(taskId, 0, 0);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    task_set(taskId, 2, gBattleAnimArgs[2]);
    task_set_func(taskId, Some(AnimTask_ShakeTargetBasedOnMovePowerOrDmg_Step));
}
pub(crate) unsafe fn AnimTask_ShakeTargetBasedOnMovePowerOrDmg_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if ({
        (*task).data[0] += 1;
        (*task).data[0]
    }) > (*task).data[1]
    {
        (*task).data[0] = 0;
        (*task).data[12] = ((*task).data[12] + 1) & 1;
        if (*task).data[10] != 0 {
            if (*task).data[12] != 0 {
                gSprites[(*task).data[7]].x2 = (*task).data[8] + (*task).data[13];
            } else {
                gSprites[(*task).data[7]].x2 = (*task).data[8] - (*task).data[14];
            }
        }
        if (*task).data[11] != 0 {
            if (*task).data[12] != 0 {
                gSprites[(*task).data[7]].y2 = (*task).data[15];
            } else {
                gSprites[(*task).data[7]].y2 = 0;
            }
        }
        if ({
            (*task).data[2] -= 1;
            (*task).data[2]
        }) == 0
        {
            gSprites[(*task).data[7]].x2 = 0;
            gSprites[(*task).data[7]].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
