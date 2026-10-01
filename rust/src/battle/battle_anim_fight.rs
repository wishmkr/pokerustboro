//! Translated from `src/battle_anim_fight.c` by tools/rustport/c2rs.py.
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
    DestroyAnimSprite, DestroyAnimVisualTask, IsContest, gAnimMoveTurn, gBattleAnimAttacker,
    gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimTranslateLinear, AnimTranslateLinear_WithFollowup, AnimTravelDiagonally,
    DestroySpriteAndMatrix, GetBattlerPosition, GetBattlerSide, GetBattlerSpriteBGPriority,
    GetBattlerSpriteCoord, GetBattlerSpriteCoordAttr, InitAnimLinearTranslation,
    InitSpritePosToAnimAttacker, InitSpritePosToAnimTarget, LoadPointerFromVars,
    RunStoredCallbackWhenAnimEnds, SetAnimSpriteInitialXOffset, StartAnimLinearTranslation,
    StorePointerInVars, StoreSpriteCallbackInData6, UpdateAnimBg3ScreenSize, WaitAnimForDuration,
};
use crate::battle_main::gBattlerPositions;
use crate::battle_main::{gBattle_BG3_X, gBattle_BG3_Y, gBattlerAttacker};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::SetGpuReg;
use crate::random::Random2;
use crate::sprite::FreeOamMatrix;
use crate::sprite::gSprites;
use crate::task::gTasks;
use crate::trig::Sin;
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
// Data tables (translate with cdata.py): sUnusedHumanoidFootSpriteTemplate sAnim_Fist sAnim_FootWide sAnim_FootTall sAnim_HandLeft sAnim_HandRight sAnims_HandsAndFeet gKarateChopSpriteTemplate gJumpKickSpriteTemplate gFistFootSpriteTemplate gFistFootRandomPosSpriteTemplate gCrossChopHandSpriteTemplate gSlidingKickSpriteTemplate sAffineAnim_SpinningHandOrFoot sAffineAnims_SpinningHandOrFoot gSpinningHandOrFootSpriteTemplate sAffineAnim_MegaPunchKick sAffineAnims_MegaPunchKick gMegaPunchKickSpriteTemplate gStompFootSpriteTemplate gDizzyPunchDuckSpriteTemplate gBrickBreakWallSpriteTemplate gBrickBreakWallShardSpriteTemplate sAffineAnim_SuperpowerOrb sAffineAnims_SuperpowerOrb gSuperpowerOrbSpriteTemplate gSuperpowerRockSpriteTemplate gSuperpowerFireballSpriteTemplate gArmThrustHandSpriteTemplate sAnim_RevengeSmallScratch_0 sAnim_RevengeSmallScratch_1 sAnim_RevengeSmallScratch_2 sAnims_RevengeSmallScratch gRevengeSmallScratchSpriteTemplate sAnim_RevengeBigScratch_0 sAnim_RevengeBigScratch_1 sAnim_RevengeBigScratch_2 sAnims_RevengeBigScratch gRevengeBigScratchSpriteTemplate sAffineAnim_FocusPunchFist sAffineAnims_FocusPunchFist gFocusPunchFistSpriteTemplate

pub(crate) unsafe fn AnimUnusedHumanoidFoot(sprite: *mut Sprite) {
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).data[0] = 15;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimSlideHandOrFootToTarget(sprite: *mut Sprite) {
    if gBattleAnimArgs[7] == 1 && GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
    }
    StartSpriteAnim(sprite, gBattleAnimArgs[6] as u8);
    gBattleAnimArgs[6] = 0;
    AnimTravelDiagonally(sprite);
}
pub(crate) unsafe fn AnimJumpKick(sprite: *mut Sprite) {
    if IsContest() != 0 {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
        gBattleAnimArgs[3] = -gBattleAnimArgs[3];
    }
    AnimSlideHandOrFootToTarget(sprite);
}
pub(crate) unsafe fn AnimBasicFistOrFoot(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, gBattleAnimArgs[4] as u8);
    if gBattleAnimArgs[3] == 0 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
    } else {
        InitSpritePosToAnimTarget(sprite, TRUE);
    }
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimFistOrFootRandomPos(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if gBattleAnimArgs[2] < 0 {
        gBattleAnimArgs[2] = (Random2() as i32 % 5) as i16;
    }
    StartSpriteAnim(sprite, gBattleAnimArgs[2] as u8);
    (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    let xMod: i16 = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_WIDTH) / 2;
    let yMod: i16 = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_HEIGHT) / 4;
    x = rem_i32(Random2() as i32, xMod as i32) as i16;
    y = rem_i32(Random2() as i32, yMod as i32) as i16;
    if Random2() as i32 & 1 != 0 {
        x *= -1;
    }
    if Random2() as i32 & 1 != 0 {
        y *= -1;
    }
    if gBattlerPositions[battler] as i32 & 1 == B_SIDE_PLAYER as i32 {
        y -= 16;
    }
    (*sprite).x += x;
    (*sprite).y += y;
    (*sprite).data[0] = gBattleAnimArgs[1];
    (*sprite).data[7] = CreateSprite(
        (&raw const (*(&raw const crate::data::battle_anim_normal::gBasicHitSplatSpriteTemplate)
            .cast::<SpriteTemplate>()))
            .cast_mut(),
        (*sprite).x,
        (*sprite).y,
        (*sprite).subpriority + 1,
    ) as i16;
    if (*sprite).data[7] != MAX_SPRITES as i16 {
        StartSpriteAffineAnim(&raw mut gSprites[(*sprite).data[7]], 0);
        gSprites[(*sprite).data[7]].callback = Some(SpriteCallbackDummy);
    }
    (*sprite).callback = Some(AnimFistOrFootRandomPos_Step);
}
pub(crate) unsafe fn AnimFistOrFootRandomPos_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        if (*sprite).data[7] != MAX_SPRITES as i16 {
            FreeOamMatrix(gSprites[(*sprite).data[7]].oam.matrixNum() as u8);
            DestroySprite(&raw mut gSprites[(*sprite).data[7]]);
        }
        DestroyAnimSprite(sprite);
    } else {
        (*sprite).data[0] -= 1;
    }
}
pub(crate) unsafe fn AnimCrossChopHand(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    (*sprite).data[0] = 30;
    if gBattleAnimArgs[2] == 0 {
        (*sprite).data[2] = (*sprite).x - 20;
    } else {
        (*sprite).data[2] = (*sprite).x + 20;
        (*sprite).set_hFlip(1);
    }
    (*sprite).data[4] = (*sprite).y - 20;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(AnimCrossChopHand_Step));
}
pub(crate) unsafe fn AnimCrossChopHand_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == 11
    {
        (*sprite).data[2] = (*sprite).x - (*sprite).x2;
        (*sprite).data[4] = (*sprite).y - (*sprite).y2;
        (*sprite).data[0] = 8;
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).callback = Some(StartAnimLinearTranslation);
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe fn AnimSlidingKick(sprite: *mut Sprite) {
    if gBattleAnimAttacker as i32 ^ 2 == gBattleAnimTarget as i32
        && GetBattlerPosition(gBattleAnimTarget) < B_POSITION_PLAYER_RIGHT
    {
        gBattleAnimArgs[0] *= -1;
    }
    InitSpritePosToAnimTarget(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = (*sprite).y;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = gBattleAnimArgs[5];
    (*sprite).data[6] = gBattleAnimArgs[4];
    (*sprite).data[7] = 0;
    (*sprite).callback = Some(AnimSlidingKick_Step);
}
pub(crate) unsafe fn AnimSlidingKick_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).y2 += Sin((*sprite).data[7] >> 8, (*sprite).data[5]);
        (*sprite).data[7] += (*sprite).data[6];
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSpinningKickOrPunch(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    StartSpriteAnim(sprite, gBattleAnimArgs[2] as u8);
    (*sprite).data[0] = gBattleAnimArgs[3];
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(AnimSpinningKickOrPunchFinish));
}
pub(crate) unsafe fn AnimSpinningKickOrPunchFinish(sprite: *mut Sprite) {
    StartSpriteAffineAnim(sprite, 0);
    (*sprite).set_affineAnimPaused(1);
    (*sprite).data[0] = 20;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimStompFoot(sprite: *mut Sprite) {
    InitSpritePosToAnimTarget(sprite, TRUE);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).callback = Some(AnimStompFoot_Step);
}
pub(crate) unsafe fn AnimStompFoot_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == -1
    {
        (*sprite).data[0] = 6;
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite).callback = Some(StartAnimLinearTranslation);
        StoreSpriteCallbackInData6(sprite, Some(AnimStompFoot_End));
    }
}
pub(crate) unsafe fn AnimStompFoot_End(sprite: *mut Sprite) {
    (*sprite).data[0] = 15;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimDizzyPunchDuck(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        InitSpritePosToAnimTarget(sprite, TRUE);
        (*sprite).data[1] = gBattleAnimArgs[2];
        (*sprite).data[2] = gBattleAnimArgs[3];
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
pub(crate) unsafe fn AnimBrickBreakWall(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    }
    (*sprite).x += gBattleAnimArgs[1];
    (*sprite).y += gBattleAnimArgs[2];
    (*sprite).data[0] = 0;
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = gBattleAnimArgs[4];
    (*sprite).data[3] = 0;
    (*sprite).callback = Some(AnimBrickBreakWall_Step);
}
pub(crate) unsafe fn AnimBrickBreakWall_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[1] -= 1;
                (*sprite).data[1]
            }) == 0
            {
                if (*sprite).data[2] == 0 {
                    DestroyAnimSprite(sprite);
                } else {
                    (*sprite).data[0] += 1;
                }
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 1
            {
                (*sprite).data[1] = 0;
                (*sprite).data[3] += 1;
                if (*sprite).data[3] as i32 & 1 != 0 {
                    (*sprite).x2 = 2;
                } else {
                    (*sprite).x2 = -2;
                }
            }
            if ({
                (*sprite).data[2] -= 1;
                (*sprite).data[2]
            }) == 0
            {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimBrickBreakWallShard(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + gBattleAnimArgs[2];
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[3];
    } else {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 + gBattleAnimArgs[2];
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[3];
    }
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[1] as u16 * 16);
    (*sprite).data[0] = 0;
    match gBattleAnimArgs[1] {
        0 => {
            (*sprite).data[6] = -3;
            (*sprite).data[7] = -3;
        }
        1 => {
            (*sprite).data[6] = 3;
            (*sprite).data[7] = -3;
        }
        2 => {
            (*sprite).data[6] = -3;
            (*sprite).data[7] = 3;
        }
        3 => {
            (*sprite).data[6] = 3;
            (*sprite).data[7] = 3;
        }
        _ => {
            DestroyAnimSprite(sprite);
            return;
        }
    }
    (*sprite).callback = Some(AnimBrickBreakWallShard_Step);
}
pub(crate) unsafe fn AnimBrickBreakWallShard_Step(sprite: *mut Sprite) {
    (*sprite).x += (*sprite).data[6];
    (*sprite).y += (*sprite).data[7];
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 40
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSuperpowerOrb(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattlerAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattlerAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u16);
        (*sprite).data[7] = gBattleAnimTarget as i16;
    } else {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
        (*sprite).data[7] = gBattleAnimAttacker as i16;
    }
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 12;
    (*sprite).data[2] = 8;
    (*sprite).callback = Some(AnimSuperpowerOrb_Step);
}
pub(crate) unsafe fn AnimSuperpowerOrb_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 180
    {
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        (*sprite).data[0] = 16;
        (*sprite).data[1] = (*sprite).x;
        (*sprite).data[2] =
            GetBattlerSpriteCoord((*sprite).data[7] as u8, BATTLER_COORD_X_2) as i16;
        (*sprite).data[3] = (*sprite).y;
        (*sprite).data[4] =
            GetBattlerSpriteCoord((*sprite).data[7] as u8, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        InitAnimLinearTranslation(sprite);
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        (*sprite).callback = Some(AnimTranslateLinear_WithFollowup);
    }
}
pub(crate) unsafe fn AnimSuperpowerRock(sprite: *mut Sprite) {
    (*sprite).x = gBattleAnimArgs[0];
    (*sprite).y = 120;
    (*sprite).data[0] = gBattleAnimArgs[3];
    StorePointerInVars(
        &raw mut (*sprite).data[4],
        &raw mut (*sprite).data[5],
        (((*sprite).y as i32) << 8) as usize as *mut c_void,
    );
    (*sprite).data[6] = gBattleAnimArgs[1];
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + gBattleAnimArgs[2] as u16 * 4);
    (*sprite).callback = Some(AnimSuperpowerRock_Step1);
}
pub(crate) unsafe fn AnimSuperpowerRock_Step1(sprite: *mut Sprite) {
    let mut var0: *mut c_void = null_mut();
    if (*sprite).data[0] != 0 {
        var0 = LoadPointerFromVars((*sprite).data[4], (*sprite).data[5]);
        var0 = (var0 as *mut u8).at(-((*sprite).data[6] as i32)) as *mut c_void;
        StorePointerInVars(&raw mut (*sprite).data[4], &raw mut (*sprite).data[5], var0);
        var0 = (var0 as usize as i32 >> 8) as usize as *mut c_void;
        (*sprite).y = var0 as usize as i32 as i16;
        if (*sprite).y < -8 {
            DestroyAnimSprite(sprite);
        } else {
            (*sprite).data[0] -= 1;
        }
    } else {
        let pos0: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        let pos1: i16 =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        let pos2: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        let pos3: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite).data[0] = pos2 - pos0;
        (*sprite).data[1] = pos3 - pos1;
        (*sprite).data[2] = (*sprite).x << 4;
        (*sprite).data[3] = (*sprite).y << 4;
        (*sprite).callback = Some(AnimSuperpowerRock_Step2);
    }
}
pub(crate) unsafe fn AnimSuperpowerRock_Step2(sprite: *mut Sprite) {
    (*sprite).data[2] += (*sprite).data[0];
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).x = (*sprite).data[2] >> 4;
    (*sprite).y = (*sprite).data[3] >> 4;
    let edgeX: u16 = (*sprite).x as u16 + 8;
    if edgeX > 256 || (*sprite).y < -8 || (*sprite).y > 120 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSuperpowerFireball(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattlerAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattlerAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        battler = gBattleAnimTarget;
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u16);
    } else {
        battler = gBattleAnimAttacker;
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
    }
    if IsContest() != 0 {
        (*sprite)
            .oam
            .set_matrixNum((*sprite).oam.matrixNum() | ST_OAM_HFLIP);
    } else if GetBattlerSide(battler) == B_SIDE_PLAYER {
        (*sprite).oam.set_matrixNum((*sprite).oam.matrixNum() | 24);
    }
    (*sprite).data[0] = 16;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(AnimTranslateLinear_WithFollowup);
}
pub(crate) unsafe fn AnimArmThrustHit_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] == (*sprite).data[4] {
        DestroyAnimSprite(sprite);
    }
    (*sprite).data[0] += 1;
}
pub(crate) unsafe fn AnimArmThrustHit(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[1] = gBattleAnimArgs[3];
    (*sprite).data[2] = gBattleAnimArgs[0];
    (*sprite).data[3] = gBattleAnimArgs[1];
    (*sprite).data[4] = gBattleAnimArgs[2];
    let mut turn: u8 = gAnimMoveTurn;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        turn += 1;
    }
    if turn as i32 & 1 != 0 {
        (*sprite).data[2] = -(*sprite).data[2];
        (*sprite).data[1] += 1;
    }
    StartSpriteAnim(sprite, (*sprite).data[1] as u8);
    (*sprite).x2 = (*sprite).data[2];
    (*sprite).y2 = (*sprite).data[3];
    (*sprite).callback = Some(AnimArmThrustHit_Step);
}
pub(crate) unsafe fn AnimRevengeScratch(sprite: *mut Sprite) {
    if gBattleAnimArgs[2] == ANIM_ATTACKER as i16 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    } else {
        InitSpritePosToAnimTarget(sprite, FALSE);
    }
    if IsContest() != 0 {
        StartSpriteAnim(sprite, 2);
    } else if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        StartSpriteAnim(sprite, 1);
    }
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimFocusPunchFist(sprite: *mut Sprite) {
    if (*sprite).affineAnimEnded() != 0 {
        (*sprite).data[1] = ((*sprite).data[1] + 40) & 0xFF;
        (*sprite).x2 = Sin((*sprite).data[1], 2);
        if ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) > 40
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MoveSkyUppercutBg(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            UpdateAnimBg3ScreenSize(FALSE);
            (*task).data[8] = gBattleAnimArgs[0];
            (*task).data[0] += 1;
        }
        1 => {
            if ({
                (*task).data[8] -= 1;
                (*task).data[8]
            }) == -1
            {
                (*task).data[0] += 1;
            }
        }
        _ => {
            (*task).data[9] += 1280;
        }
    }
    (*task).data[10] += 2816;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        gBattle_BG3_X += ((*task).data[9] >> 8) as u16;
    } else {
        gBattle_BG3_X -= ((*task).data[9] >> 8) as u16;
    }
    gBattle_BG3_Y += ((*task).data[10] >> 8) as u16;
    (*task).data[9] &= 0xFF;
    (*task).data[10] &= 0xFF;
    if gBattleAnimArgs[7] == -1 {
        gBattle_BG3_X = 0;
        gBattle_BG3_Y = 0;
        UpdateAnimBg3ScreenSize(TRUE);
        DestroyAnimVisualTask(taskId);
    }
}
