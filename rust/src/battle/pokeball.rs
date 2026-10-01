//! Translated from `src/pokeball.c` by tools/rustport/c2rs.py.
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
    clippy::too_many_arguments,
    dead_code,
    unused_assignments
)]

use crate::agb_main::gMain;
use crate::battle_anim_mons::{
    AnimTranslateLinear, GetBattlerAtPosition, GetBattlerPosition, GetBattlerSide,
    GetBattlerSpriteCoord, InitAnimArcTranslation, IsDoubleBattle, TranslateAnimHorizontalArc,
};
use crate::battle_anim_throw::{AnimateBallOpenParticles, ItemIdToBallId, LaunchBallFadeMonTask};
use crate::battle_gfx_sfx_util::ShouldPlayNormalMonCry;
use crate::battle_main::{
    SpriteCB_OpponentMonFromBall, SpriteCB_PlayerMonFromBall, gActiveBattler,
    gBattleSpritesDataPtr, gBattleTypeFlags, gBattlerTarget, gDoingBattleAnim,
};
use crate::battle_main::{gBattlerPartyIndexes, gBattlerSpriteIds, gHealthboxSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::m4a::{gMPlayInfo_BGM, m4aMPlayAllStop, m4aMPlayStop, m4aMPlayVolumeControl};
use crate::pokemon::{DoMonFrontSpriteAnimation, GetMonData2, gEnemyParty, gPlayerParty};
use crate::sound::{
    IsBGMPlaying, IsCryPlayingOrClearCrySongs, PlayCry_ByMode, PlayCry_ReleaseDouble, PlaySE,
    StopCryAndClearCrySongs,
};
use crate::sprite::gSprites;
use crate::sprite::{
    FreeOamMatrix, FreeSpritePaletteByTag, FreeSpriteTilesByTag, GetSpriteTileStartByTag,
};
use crate::task::{DestroyTask, TaskDummy};
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
/// `ChangeSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn ChangeSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::ChangeSpriteAffineAnim(a0 as _, a1);
    }
}
/// `CreateInvisibleSpriteWithCallback` with this module's view of its types.
#[inline]
unsafe fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe fn(*mut Sprite)>) -> u8 {
    unsafe { crate::util::CreateInvisibleSpriteWithCallback(core::mem::transmute(a0)) }
}
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
/// `LZDecompressVram` with this module's view of its types.
#[inline]
unsafe fn LZDecompressVram(a0: *mut u32, a1: *mut c_void) {
    unsafe {
        crate::decompress::LZDecompressVram(a0 as _, a1 as _);
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
// The C's names for task and sprite data slots.
const sMonSpriteId: usize = 0;
const sSpeedX: usize = 0;
const tCryTaskSpecies: usize = 0;
const sDelay: usize = 1;
const sDelayTimer: usize = 1;
const sSpeedY: usize = 1;
const tCryTaskPan: usize = 1;
const tPan: usize = 1;
const sMonPalNum: usize = 2;
const tCryTaskWantedCry: usize = 2;
const tThrowId: usize = 2;
const sFadePalsLo: usize = 3;
const tBattler: usize = 3;
const tCryTaskBattler: usize = 3;
const sFadePalsHi: usize = 4;
const tCryTaskMonSpriteId: usize = 4;
const tOpponentBattler: usize = 4;
const sFinalMonX: usize = 5;
const sTimer: usize = 5;
const tCryTaskMonPtr1: usize = 5;
const sBattler: usize = 6;
const sFinalMonY: usize = 6;
const tCryTaskMonPtr2: usize = 6;
const sSpecies: usize = 7;
const sTrigIdx: usize = 7;
const tCryTaskFrames: usize = 10;
const tCryTaskState: usize = 15;
// Data tables (translate with cdata.py): gBallSpriteSheets gBallSpritePalettes sBallOamData sBallAnimSeq3 sBallAnimSeq5 sBallAnimSeq4 sBallAnimSeq6 sBallAnimSeq0 sBallAnimSeq1 sBallAnimSeq2 sBallAnimSequences sAffineAnim_BallRotate_0 sAffineAnim_BallRotate_Right sAffineAnim_BallRotate_Left sAffineAnim_BallRotate_3 sAffineAnim_BallRotate_4 sAffineAnim_BallRotate gBallSpriteTemplates

static gBallSpritePalettes: Table<CArray<CompressedSpritePalette, 12>> =
    Table((&raw const crate::data::pokeball::gBallSpritePalettes).cast());
static gBallSpriteSheets: Table<CArray<CompressedSpriteSheet, 12>> =
    Table((&raw const crate::data::pokeball::gBallSpriteSheets).cast());
static gBallSpriteTemplates: Table<CArray<SpriteTemplate, 12>> =
    Table((&raw const crate::data::pokeball::gBallSpriteTemplates).cast());

pub unsafe fn DoPokeballSendOutAnimation(pan: i16, kindOfThrow: u8) -> u8 {
    gDoingBattleAnim = TRUE;
    (*(*gBattleSpritesDataPtr).healthBoxesData.at(gActiveBattler)).set_ballAnimActive(TRUE);
    let taskId: u8 = CreateTask(Some(Task_DoPokeballSendOutAnim), 5);
    task_set(taskId, tPan, pan);
    task_set(taskId, tThrowId, kindOfThrow as i16);
    task_set(taskId, tBattler, gActiveBattler as i16);
    0
}
pub(crate) unsafe fn Task_DoPokeballSendOutAnim(taskId: u8) {
    let mut itemId: u16 = 0;
    let mut notSendOut: u8 = FALSE;
    if task_get(taskId, 0) == 0 {
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        return;
    }
    let throwCaseId: u16 = task_get(taskId, 2) as u16;
    let battler: u8 = task_get(taskId, tBattler) as u8;
    if GetBattlerSide(battler) != B_SIDE_PLAYER {
        itemId = GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
            MON_DATA_POKEBALL,
        ) as u16;
    } else {
        itemId = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
            MON_DATA_POKEBALL,
        ) as u16;
    }
    let ballId: u16 = ItemIdToBallId(itemId) as u16;
    LoadBallGfx(ballId as u8);
    let ballSpriteId: u8 = CreateSprite(
        (&raw const gBallSpriteTemplates[ballId]).cast_mut(),
        32,
        80,
        29,
    );
    gSprites[ballSpriteId].data[0] = 0x80;
    gSprites[ballSpriteId].data[1] = 0;
    gSprites[ballSpriteId].data[7] = throwCaseId as i16;
    match throwCaseId {
        255 => {
            gBattlerTarget = battler;
            gSprites[ballSpriteId].x = 24;
            gSprites[ballSpriteId].y = 68;
            gSprites[ballSpriteId].callback = Some(SpriteCB_PlayerMonSendOut_1);
        }
        254 => {
            gSprites[ballSpriteId].x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16;
            gSprites[ballSpriteId].y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16 + 24;
            gBattlerTarget = battler;
            gSprites[ballSpriteId].data[0] = 0;
            gSprites[ballSpriteId].callback = Some(SpriteCB_OpponentMonSendOut);
        }
        _ => {
            gBattlerTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            notSendOut = TRUE;
        }
    }
    gSprites[ballSpriteId].data[sBattler] = gBattlerTarget as i16;
    if notSendOut == 0 {
        DestroyTask(taskId);
        return;
    }
    gSprites[ballSpriteId].data[0] = 34;
    gSprites[ballSpriteId].data[2] = GetBattlerSpriteCoord(gBattlerTarget, BATTLER_COORD_X) as i16;
    gSprites[ballSpriteId].data[4] =
        GetBattlerSpriteCoord(gBattlerTarget, BATTLER_COORD_Y) as i16 - 16;
    gSprites[ballSpriteId].data[5] = -40;
    InitAnimArcTranslation(&raw mut gSprites[ballSpriteId]);
    gSprites[ballSpriteId].oam.affineParam = taskId as u16;
    task_set(taskId, 4, gBattlerTarget as i16);
    task_set_func(taskId, Some(TaskDummy));
    PlaySE(SE_BALL_THROW);
}
pub(crate) unsafe fn SpriteCB_BallThrow(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        let taskId: u8 = (*sprite).oam.affineParam as u8;
        let opponentBattler: u8 = task_get(taskId, tOpponentBattler) as u8;
        let noOfShakes: u8 = task_get(taskId, tThrowId) as u8;
        StartSpriteAnim(sprite, 1);
        (*sprite).set_affineAnimPaused(TRUE);
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).data[5] = 0;
        let ballId: u16 = ItemIdToBallId(GetBattlerPokeballItemId(opponentBattler)) as u16;
        AnimateBallOpenParticles(
            (*sprite).x as u8,
            (*sprite).y as u8 - 5,
            1,
            28,
            ballId as u8,
        );
        (*sprite).data[0] = LaunchBallFadeMonTask(0, opponentBattler, 14, ballId as u8) as i16;
        (*sprite).data[sBattler] = opponentBattler as i16;
        (*sprite).data[7] = noOfShakes as i16;
        DestroyTask(taskId);
        (*sprite).callback = Some(SpriteCB_BallThrow_ReachMon);
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_ReachMon(sprite: *mut Sprite) {
    (*sprite).callback = Some(SpriteCB_BallThrow_StartShrinkMon);
}
pub(crate) unsafe fn SpriteCB_BallThrow_StartShrinkMon(sprite: *mut Sprite) {
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == 10
    {
        (*sprite).data[5] = 0;
        (*sprite).callback = Some(SpriteCB_BallThrow_ShrinkMon);
        StartSpriteAffineAnim(
            &raw mut gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]],
            BATTLER_AFFINE_RETURN,
        );
        AnimateSprite(&raw mut gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]]);
        gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]].data[1] = 0;
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_ShrinkMon(sprite: *mut Sprite) {
    (*sprite).data[5] += 1;
    if (*sprite).data[5] == 11 {
        PlaySE(SE_BALL_TRADE);
    }
    if gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]].affineAnimEnded() != 0 {
        StartSpriteAnim(sprite, 2);
        gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]].set_invisible(TRUE as u16);
        (*sprite).data[5] = 0;
        (*sprite).callback = Some(SpriteCB_BallThrow_Close);
    } else {
        gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]].data[1] += 0x60;
        gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]].y2 =
            (-(gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]].data[1] as i32) >> 8) as i16;
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_Close(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).data[5] += 1;
        if (*sprite).data[5] == 1 {
            (*sprite).data[3] = 0;
            (*sprite).data[4] = 32;
            (*sprite).data[5] = 0;
            (*sprite).y += Cos(0, 32);
            (*sprite).y2 = -Cos(0, (*sprite).data[4]);
            (*sprite).callback = Some(SpriteCB_BallThrow_FallToGround);
        }
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_FallToGround(sprite: *mut Sprite) {
    let mut r5: u8 = FALSE;
    match (*sprite).data[3] as i32 & 0xFF {
        0 => {
            (*sprite).y2 = -Cos((*sprite).data[5], (*sprite).data[4]);
            (*sprite).data[5] += 4 + ((*sprite).data[3] >> 8);
            if (*sprite).data[5] >= 64 {
                (*sprite).data[4] -= 10;
                (*sprite).data[3] += 0x101;
                if (*sprite).data[3] >> 8 == 4 {
                    r5 = TRUE;
                }
                match (*sprite).data[3] >> 8 {
                    1 => {
                        PlaySE(SE_BALL_BOUNCE_1);
                    }
                    2 => {
                        PlaySE(SE_BALL_BOUNCE_2);
                    }
                    3 => {
                        PlaySE(SE_BALL_BOUNCE_3);
                    }
                    _ => {
                        PlaySE(SE_BALL_BOUNCE_4);
                    }
                }
            }
        }
        1 => {
            (*sprite).y2 = -Cos((*sprite).data[5], (*sprite).data[4]);
            (*sprite).data[5] -= 4 + ((*sprite).data[3] >> 8);
            if (*sprite).data[5] <= 0 {
                (*sprite).data[5] = 0;
                (*sprite).data[3] &= -256;
            }
        }
        _ => {}
    }
    if r5 != 0 {
        (*sprite).data[3] = 0;
        (*sprite).y += Cos(64, 32);
        (*sprite).y2 = 0;
        if (*sprite).data[7] == 0 {
            (*sprite).callback = Some(SpriteCB_ReleaseMonFromBall);
        } else {
            (*sprite).callback = Some(SpriteCB_BallThrow_StartShakes);
            (*sprite).data[4] = 1;
            (*sprite).data[5] = 0;
        }
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_StartShakes(sprite: *mut Sprite) {
    (*sprite).data[3] += 1;
    if (*sprite).data[3] == 31 {
        (*sprite).data[3] = 0;
        (*sprite).set_affineAnimPaused(TRUE);
        StartSpriteAffineAnim(sprite, 1);
        (*sprite).callback = Some(SpriteCB_BallThrow_Shake);
        PlaySE(SE_BALL);
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_Shake(sprite: *mut Sprite) {
    match (*sprite).data[3] as i32 & 0xFF {
        0 | 2 => {
            (*sprite).x2 += (*sprite).data[4];
            (*sprite).data[5] += (*sprite).data[4];
            (*sprite).set_affineAnimPaused(FALSE);
            if (*sprite).data[5] > 3 || (*sprite).data[5] < -3 {
                (*sprite).data[3] += 1;
                (*sprite).data[5] = 0;
            }
        }
        1 => {
            (*sprite).data[5] += 1;
            if (*sprite).data[5] == 1 {
                (*sprite).data[5] = 0;
                (*sprite).data[4] = -(*sprite).data[4];
                (*sprite).data[3] += 1;
                (*sprite).set_affineAnimPaused(FALSE);
                if (*sprite).data[4] < 0 {
                    ChangeSpriteAffineAnim(sprite, 2);
                } else {
                    ChangeSpriteAffineAnim(sprite, 1);
                }
            } else {
                (*sprite).set_affineAnimPaused(TRUE);
            }
        }
        3 => {
            (*sprite).data[3] += 0x100;
            if (*sprite).data[3] >> 8 == (*sprite).data[7] {
                (*sprite).callback = Some(SpriteCB_ReleaseMonFromBall);
            } else {
                if (*sprite).data[7] == 4 && (*sprite).data[3] >> 8 == 3 {
                    (*sprite).callback = Some(SpriteCB_BallThrow_StartCaptureMon);
                    (*sprite).set_affineAnimPaused(TRUE);
                } else {
                    (*sprite).data[3] += 1;
                    (*sprite).set_affineAnimPaused(TRUE);
                }
            }
        }
        _ => {
            (*sprite).data[5] += 1;
            if (*sprite).data[5] == 31 {
                (*sprite).data[5] = 0;
                (*sprite).data[3] &= -256;
                StartSpriteAffineAnim(sprite, 3);
                if (*sprite).data[4] < 0 {
                    StartSpriteAffineAnim(sprite, 2);
                } else {
                    StartSpriteAffineAnim(sprite, 1);
                }
                PlaySE(SE_BALL);
            }
        }
    }
}
pub(crate) unsafe fn Task_PlayCryWhenReleasedFromBall(taskId: u8) {
    let wantedCry: u8 = task_get(taskId, tCryTaskWantedCry) as u8;
    let pan: i8 = task_get(taskId, tCryTaskPan) as i8;
    let species: u16 = task_get(taskId, tCryTaskSpecies) as u16;
    let battler: u8 = task_get(taskId, tCryTaskBattler) as u8;
    let monSpriteId: u8 = task_get(taskId, tCryTaskMonSpriteId) as u8;
    let mon: *mut Pokemon = ((task_get(taskId, tCryTaskMonPtr1) as u32) << 16
        | task_get(taskId, tCryTaskMonPtr2) as u16 as u32) as usize
        as *mut c_void as *mut Pokemon;
    'l1: {
        let sw1: i16 = task_get(taskId, tCryTaskState);
        let matched = sw1 == 0
            || sw1 == 1
            || sw1 == 2
            || sw1 == 20
            || sw1 == 3
            || sw1 == 30
            || sw1 == 31
            || sw1 == 32;
        let mut fall = false;
        if sw1 == 0 || !matched {
            if gSprites[monSpriteId].affineAnimEnded() != 0 {
                task_set(taskId, tCryTaskState, wantedCry as i16 + 1);
            }
            break 'l1;
        }
        if sw1 == 1 {
            if ShouldPlayNormalMonCry(mon) == TRUE as u32 {
                PlayCry_ByMode(species, pan, CRY_MODE_NORMAL);
            } else {
                PlayCry_ByMode(species, pan, CRY_MODE_WEAK);
            }
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_waitForCry(FALSE);
            DestroyTask(taskId);
            break 'l1;
        }
        if sw1 == 2 {
            StopCryAndClearCrySongs();
            task_set(taskId, tCryTaskFrames, 3);
            task_set(taskId, tCryTaskState, 20);
            break 'l1;
        }
        if sw1 == 20 {
            if task_get(taskId, tCryTaskFrames) == 0 {
                if ShouldPlayNormalMonCry(mon) == TRUE as u32 {
                    PlayCry_ReleaseDouble(species, pan, CRY_MODE_DOUBLES);
                } else {
                    PlayCry_ReleaseDouble(species, pan, CRY_MODE_WEAK_DOUBLES);
                }
                (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_waitForCry(FALSE);
                DestroyTask(taskId);
            } else {
                task_set(taskId, tCryTaskFrames, task_get(taskId, tCryTaskFrames) - 1);
            }
            break 'l1;
        }
        if sw1 == 3 {
            task_set(taskId, tCryTaskFrames, 6);
            task_set(taskId, tCryTaskState, 30);
            break 'l1;
        }
        if sw1 == 30 {
            fall = true;
            if task_get(taskId, tCryTaskFrames) != 0 {
                task_set(taskId, tCryTaskFrames, task_get(taskId, tCryTaskFrames) - 1);
                break 'l1;
            }
            task_set(taskId, tCryTaskState, task_get(taskId, tCryTaskState) + 1);
        }
        if fall || sw1 == 31 {
            if IsCryPlayingOrClearCrySongs() == 0 {
                StopCryAndClearCrySongs();
                task_set(taskId, tCryTaskFrames, 3);
                task_set(taskId, tCryTaskState, task_get(taskId, tCryTaskState) + 1);
            }
            break 'l1;
        }
        if sw1 == 32 {
            if task_get(taskId, tCryTaskFrames) != 0 {
                task_set(taskId, tCryTaskFrames, task_get(taskId, tCryTaskFrames) - 1);
                break 'l1;
            }
            if ShouldPlayNormalMonCry(mon) == TRUE as u32 {
                PlayCry_ReleaseDouble(species, pan, CRY_MODE_NORMAL);
            } else {
                PlayCry_ReleaseDouble(species, pan, CRY_MODE_WEAK);
            }
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_waitForCry(FALSE);
            DestroyTask(taskId);
            break 'l1;
        }
    }
}
pub(crate) unsafe fn SpriteCB_ReleaseMonFromBall(sprite: *mut Sprite) {
    let battler: u8 = (*sprite).data[6] as u8;
    StartSpriteAnim(sprite, 1);
    let ballId: u32 = ItemIdToBallId(GetBattlerPokeballItemId(battler)) as u32;
    AnimateBallOpenParticles(
        (*sprite).x as u8,
        (*sprite).y as u8 - 5,
        1,
        28,
        ballId as u8,
    );
    (*sprite).data[0] =
        LaunchBallFadeMonTask(TRUE, (*sprite).data[6] as u8, 14, ballId as u8) as i16;
    (*sprite).callback = Some(HandleBallAnimEnd);
    if gMain.inBattle() != 0 {
        let mut mon: *mut Pokemon = null_mut();
        let mut pan: i8 = 0;
        let mut wantedCryCase: u16 = 0;
        if GetBattlerSide(battler) != B_SIDE_PLAYER {
            mon = &raw mut gEnemyParty[gBattlerPartyIndexes[battler]];
            pan = 25;
        } else {
            mon = &raw mut gPlayerParty[gBattlerPartyIndexes[battler]];
            pan = -25;
        }
        let species: u16 = GetMonData2(mon, MON_DATA_SPECIES) as u16;
        if (battler == GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)
            || battler == GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT))
            && IsDoubleBattle() != 0
            && (*(*gBattleSpritesDataPtr).animationData).introAnimActive() != 0
        {
            if gBattleTypeFlags & BATTLE_TYPE_MULTI != 0 && gBattleTypeFlags & BATTLE_TYPE_LINK != 0
            {
                if IsBGMPlaying() != 0 {
                    m4aMPlayStop(&raw mut gMPlayInfo_BGM);
                }
            } else {
                m4aMPlayVolumeControl(&raw mut gMPlayInfo_BGM, TRACKS_ALL, 128);
            }
        }
        if IsDoubleBattle() == 0 || (*(*gBattleSpritesDataPtr).animationData).introAnimActive() == 0
        {
            wantedCryCase = 0;
        } else if battler == GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)
            || battler == GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT)
        {
            wantedCryCase = 1;
        } else {
            wantedCryCase = 2;
        }
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_waitForCry(TRUE);
        let taskId: u8 = CreateTask(Some(Task_PlayCryWhenReleasedFromBall), 3);
        task_set(taskId, 0, species as i16);
        task_set(taskId, 1, pan as i16);
        task_set(taskId, tCryTaskWantedCry, wantedCryCase as i16);
        task_set(taskId, tCryTaskBattler, battler as i16);
        task_set(
            taskId,
            tCryTaskMonSpriteId,
            gBattlerSpriteIds[(*sprite).data[6]] as i16,
        );
        task_set(taskId, tCryTaskMonPtr1, (mon as usize as u32 >> 16) as i16);
        task_set(taskId, 6, mon as usize as u32 as i16);
        task_set(taskId, tCryTaskState, 0);
    }
    StartSpriteAffineAnim(
        &raw mut gSprites[gBattlerSpriteIds[(*sprite).data[6]]],
        BATTLER_AFFINE_EMERGE,
    );
    if GetBattlerSide((*sprite).data[6] as u8) == B_SIDE_OPPONENT {
        gSprites[gBattlerSpriteIds[(*sprite).data[6]]].callback =
            Some(SpriteCB_OpponentMonFromBall);
    } else {
        gSprites[gBattlerSpriteIds[(*sprite).data[6]]].callback = Some(SpriteCB_PlayerMonFromBall);
    }
    AnimateSprite(&raw mut gSprites[gBattlerSpriteIds[(*sprite).data[6]]]);
    gSprites[gBattlerSpriteIds[(*sprite).data[6]]].data[1] = 0x1000;
}
pub(crate) unsafe fn SpriteCB_BallThrow_StartCaptureMon(sprite: *mut Sprite) {
    (*sprite).set_animPaused(TRUE);
    (*sprite).callback = Some(SpriteCB_BallThrow_CaptureMon);
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = 0;
}
pub(crate) unsafe fn HandleBallAnimEnd(sprite: *mut Sprite) {
    let mut affineAnimEnded: u8 = FALSE;
    let battler: u8 = (*sprite).data[sBattler] as u8;
    gSprites[gBattlerSpriteIds[battler]].set_invisible(FALSE as u16);
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if gSprites[gBattlerSpriteIds[battler]].affineAnimEnded() != 0 {
        StartSpriteAffineAnim(
            &raw mut gSprites[gBattlerSpriteIds[battler]],
            BATTLER_AFFINE_NORMAL,
        );
        affineAnimEnded = TRUE;
    } else {
        gSprites[gBattlerSpriteIds[battler]].data[1] -= 288;
        gSprites[gBattlerSpriteIds[battler]].y2 = gSprites[gBattlerSpriteIds[battler]].data[1] >> 8;
    }
    if (*sprite).animEnded() != 0 && affineAnimEnded != 0 {
        gSprites[gBattlerSpriteIds[battler]].y2 = 0;
        gDoingBattleAnim = FALSE;
        (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_ballAnimActive(FALSE);
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
        let mut doneBattlers: i32 = 0;
        let mut i: i32 = 0;
        while i < MAX_BATTLERS_COUNT as i32 {
            if (*(*gBattleSpritesDataPtr).healthBoxesData.at(i)).ballAnimActive() == FALSE {
                doneBattlers += 1;
            }
            i += 1;
        }
        if doneBattlers == MAX_BATTLERS_COUNT as i32 {
            for i in 0..POKEBALL_COUNT {
                FreeBallGfx(i as u8);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_BallThrow_CaptureMon(sprite: *mut Sprite) {
    let battler: u8 = (*sprite).data[sBattler] as u8;
    (*sprite).data[4] += 1;
    if (*sprite).data[4] == 40 {
    } else if (*sprite).data[4] == 95 {
        gDoingBattleAnim = FALSE;
        m4aMPlayAllStop();
        PlaySE(MUS_EVOLVED);
    } else if (*sprite).data[4] == 315 {
        FreeOamMatrix(
            gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]]
                .oam
                .matrixNum() as u8,
        );
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[(*sprite).data[sBattler]]]);
        DestroySpriteAndFreeResources(sprite);
        if gMain.inBattle() != 0 {
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_ballAnimActive(FALSE);
        }
    }
}
pub(crate) unsafe fn SpriteCB_PlayerMonSendOut_1(sprite: *mut Sprite) {
    (*sprite).data[0] = 25;
    (*sprite).data[2] =
        GetBattlerSpriteCoord((*sprite).data[sBattler] as u8, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] =
        GetBattlerSpriteCoord((*sprite).data[sBattler] as u8, BATTLER_COORD_Y_PIC_OFFSET) as i16
            + 24;
    (*sprite).data[5] = -30;
    (*sprite).oam.affineParam = (*sprite).data[sBattler] as u16;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(SpriteCB_PlayerMonSendOut_2);
}
pub(crate) unsafe fn SpriteCB_PlayerMonSendOut_2(sprite: *mut Sprite) {
    let mut r6: u32 = 0;
    let mut r7: u32 = 0;
    if ((*sprite).data[7] >> 8) as i32 & 0xFF >= 35 && ((*sprite).data[7] >> 8) as i32 & 0xFF < 80 {
        if (*sprite).oam.affineParam as i32 & 0xFF00 == 0 {
            r6 = (*sprite).data[1] as u32 & 1;
            r7 = (*sprite).data[2] as u32 & 1;
            (*sprite).data[1] = ((*sprite).data[1] / 3) & -2 | r6 as i16;
            (*sprite).data[2] = ((*sprite).data[2] / 3) & -2 | r7 as i16;
            StartSpriteAffineAnim(sprite, 4);
        }
        let r4: i16 = (*sprite).data[0];
        AnimTranslateLinear(sprite);
        (*sprite).data[7] += (*sprite).data[sBattler] / 3;
        (*sprite).y2 += Sin((*sprite).data[7] >> 8 & 0xFF, (*sprite).data[5]);
        (*sprite).oam.affineParam += 0x100;
        if ((*sprite).oam.affineParam >> 8) as i32 % 3 != 0 {
            (*sprite).data[0] = r4;
        } else {
            (*sprite).data[0] = r4 - 1;
        }
        if ((*sprite).data[7] >> 8) as i32 & 0xFF >= 80 {
            r6 = (*sprite).data[1] as u32 & 1;
            r7 = (*sprite).data[2] as u32 & 1;
            (*sprite).data[1] = ((*sprite).data[1] * 3) & -2 | r6 as i16;
            (*sprite).data[2] = ((*sprite).data[2] * 3) & -2 | r7 as i16;
        }
    } else {
        if TranslateAnimHorizontalArc(sprite) != 0 {
            (*sprite).x += (*sprite).x2;
            (*sprite).y += (*sprite).y2;
            (*sprite).y2 = 0;
            (*sprite).x2 = 0;
            (*sprite).data[sBattler] = (*sprite).oam.affineParam as i16 & 0xFF;
            (*sprite).data[0] = 0;
            if IsDoubleBattle() != 0
                && (*(*gBattleSpritesDataPtr).animationData).introAnimActive() != 0
                && (*sprite).data[sBattler] == GetBattlerAtPosition(B_POSITION_PLAYER_RIGHT) as i16
            {
                (*sprite).callback = Some(SpriteCB_ReleaseMon2FromBall);
            } else {
                (*sprite).callback = Some(SpriteCB_ReleaseMonFromBall);
            }
            StartSpriteAffineAnim(sprite, 0);
        }
    }
}
pub(crate) unsafe fn SpriteCB_ReleaseMon2FromBall(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] += 1;
        t1
    }) > 24
    {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(SpriteCB_ReleaseMonFromBall);
    }
}
pub(crate) unsafe fn SpriteCB_OpponentMonSendOut(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if (*sprite).data[0] > 15 {
        (*sprite).data[0] = 0;
        if IsDoubleBattle() != 0
            && (*(*gBattleSpritesDataPtr).animationData).introAnimActive() != 0
            && (*sprite).data[sBattler] == GetBattlerAtPosition(B_POSITION_OPPONENT_RIGHT) as i16
        {
            (*sprite).callback = Some(SpriteCB_ReleaseMon2FromBall);
        } else {
            (*sprite).callback = Some(SpriteCB_ReleaseMonFromBall);
        }
    }
}
unsafe fn AnimateBallOpenParticlesForPokeball(
    x: u8,
    y: u8,
    kindOfStars: u8,
    subpriority: u8,
) -> u8 {
    AnimateBallOpenParticles(x, y, kindOfStars, subpriority, BALL_POKE)
}
unsafe fn LaunchBallFadeMonTaskForPokeball(
    unFadeLater: u8,
    spritePalNum: u8,
    selectedPalettes: u32,
) -> u8 {
    LaunchBallFadeMonTask(unFadeLater, spritePalNum, selectedPalettes, BALL_POKE)
}
pub unsafe fn CreatePokeballSpriteToReleaseMon(
    monSpriteId: u8,
    monPalNum: u8,
    x: u8,
    y: u8,
    oamPriority: u8,
    subpriority: u8,
    delay: u8,
    fadePalettes: u32,
    species: u16,
) {
    LoadCompressedSpriteSheetUsingHeap((&raw const gBallSpriteSheets[0]).cast_mut());
    LoadCompressedSpritePaletteUsingHeap((&raw const gBallSpritePalettes[0]).cast_mut());
    let spriteId: u8 = CreateSprite(
        (&raw const gBallSpriteTemplates[0]).cast_mut(),
        x as i16,
        y as i16,
        subpriority,
    );
    gSprites[spriteId].data[sMonSpriteId] = monSpriteId as i16;
    gSprites[spriteId].data[sFinalMonX] = gSprites[monSpriteId].x;
    gSprites[spriteId].data[sFinalMonY] = gSprites[monSpriteId].y;
    gSprites[monSpriteId].x = x as i16;
    gSprites[monSpriteId].y = y as i16;
    gSprites[monSpriteId].data[sSpecies] = species as i16;
    gSprites[spriteId].data[sDelay] = delay as i16;
    gSprites[spriteId].data[sMonPalNum] = monPalNum as i16;
    gSprites[spriteId].data[sFadePalsLo] = fadePalettes as i16;
    gSprites[spriteId].data[sFadePalsHi] = (fadePalettes >> 16) as i16;
    gSprites[spriteId].oam.set_priority(oamPriority as u16);
    gSprites[spriteId].callback = Some(SpriteCB_PokeballReleaseMon);
    gSprites[monSpriteId].set_invisible(TRUE as u16);
}
pub(crate) unsafe fn SpriteCB_PokeballReleaseMon(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        let mut subpriority: u8 = 0;
        let spriteId: u8 = (*sprite).data[sMonSpriteId] as u8;
        let monPalNum: u8 = (*sprite).data[sMonPalNum] as u8;
        let selectedPalettes: u32 = (*sprite).data[sFadePalsLo] as u16 as u32
            | ((*sprite).data[sFadePalsHi] as u16 as u32) << 16;
        if (*sprite).subpriority != 0 {
            subpriority = (*sprite).subpriority - 1;
        } else {
            subpriority = 0;
        }
        StartSpriteAnim(sprite, 1);
        AnimateBallOpenParticlesForPokeball(
            (*sprite).x as u8,
            (*sprite).y as u8 - 5,
            (*sprite).oam.priority() as u8,
            subpriority,
        );
        (*sprite).data[1] =
            LaunchBallFadeMonTaskForPokeball(TRUE, monPalNum, selectedPalettes) as i16;
        (*sprite).callback = Some(SpriteCB_ReleasedMonFlyOut);
        gSprites[spriteId].set_invisible(FALSE as u16);
        StartSpriteAffineAnim(&raw mut gSprites[spriteId], BATTLER_AFFINE_EMERGE);
        AnimateSprite(&raw mut gSprites[spriteId]);
        gSprites[spriteId].data[1] = 0x1000;
        (*sprite).data[sTrigIdx] = 0;
    } else {
        (*sprite).data[1] -= 1;
    }
}
pub(crate) unsafe fn SpriteCB_ReleasedMonFlyOut(sprite: *mut Sprite) {
    let mut emergeAnimFinished: u8 = FALSE;
    let mut atFinalPosition: u8 = FALSE;
    let monSpriteId: u8 = (*sprite).data[sMonSpriteId] as u8;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if gSprites[monSpriteId].affineAnimEnded() != 0 {
        StartSpriteAffineAnim(&raw mut gSprites[monSpriteId], BATTLER_AFFINE_NORMAL);
        emergeAnimFinished = TRUE;
    }
    x = (((*sprite).data[sFinalMonX] as i32 - (*sprite).x as i32) * (*sprite).data[7] as i32 / 128)
        as u16
        + (*sprite).x as u16;
    y = (((*sprite).data[sFinalMonY] as i32 - (*sprite).y as i32) * (*sprite).data[7] as i32 / 128)
        as u16
        + (*sprite).y as u16;
    gSprites[monSpriteId].x = x as i16;
    gSprites[monSpriteId].y = y as i16;
    if (*sprite).data[7] < 128 {
        let sine: i16 = -((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [(*sprite).data[7] as u8]
            / 8);
        (*sprite).data[7] += 4;
        gSprites[monSpriteId].x2 = sine;
        gSprites[monSpriteId].y2 = sine;
    } else {
        gSprites[monSpriteId].x = (*sprite).data[sFinalMonX];
        gSprites[monSpriteId].y = (*sprite).data[sFinalMonY];
        gSprites[monSpriteId].x2 = 0;
        gSprites[monSpriteId].y2 = 0;
        atFinalPosition = TRUE;
    }
    if (*sprite).animEnded() != 0 && emergeAnimFinished != 0 && atFinalPosition != 0 {
        if gSprites[monSpriteId].data[7] == SPECIES_EGG as i16 {
            DoMonFrontSpriteAnimation(
                &raw mut gSprites[monSpriteId],
                gSprites[monSpriteId].data[7] as u16,
                TRUE,
                0,
            );
        } else {
            DoMonFrontSpriteAnimation(
                &raw mut gSprites[monSpriteId],
                gSprites[monSpriteId].data[7] as u16,
                0,
                0,
            );
        }
        DestroySpriteAndFreeResources(sprite);
    }
}
pub unsafe fn CreateTradePokeballSprite(
    monSpriteId: u8,
    monPalNum: u8,
    x: u8,
    y: u8,
    oamPriority: u8,
    subPriority: u8,
    delay: u8,
    fadePalettes: u32,
) -> u8 {
    LoadCompressedSpriteSheetUsingHeap((&raw const gBallSpriteSheets[0]).cast_mut());
    LoadCompressedSpritePaletteUsingHeap((&raw const gBallSpritePalettes[0]).cast_mut());
    let spriteId: u8 = CreateSprite(
        (&raw const gBallSpriteTemplates[0]).cast_mut(),
        x as i16,
        y as i16,
        subPriority,
    );
    gSprites[spriteId].data[sMonSpriteId] = monSpriteId as i16;
    gSprites[spriteId].data[sDelay] = delay as i16;
    gSprites[spriteId].data[sMonPalNum] = monPalNum as i16;
    gSprites[spriteId].data[sFadePalsLo] = fadePalettes as i16;
    gSprites[spriteId].data[sFadePalsHi] = (fadePalettes >> 16) as i16;
    gSprites[spriteId].oam.set_priority(oamPriority as u16);
    gSprites[spriteId].callback = Some(SpriteCB_TradePokeball);
    spriteId
}
pub(crate) unsafe fn SpriteCB_TradePokeball(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        let mut subpriority: u8 = 0;
        let monSpriteId: u8 = (*sprite).data[sMonSpriteId] as u8;
        let monPalNum: u8 = (*sprite).data[sMonPalNum] as u8;
        let selectedPalettes: u32 = (*sprite).data[sFadePalsLo] as u16 as u32
            | ((*sprite).data[sFadePalsHi] as u16 as u32) << 16;
        if (*sprite).subpriority != 0 {
            subpriority = (*sprite).subpriority - 1;
        } else {
            subpriority = 0;
        }
        StartSpriteAnim(sprite, 1);
        AnimateBallOpenParticlesForPokeball(
            (*sprite).x as u8,
            (*sprite).y as u8 - 5,
            (*sprite).oam.priority() as u8,
            subpriority,
        );
        (*sprite).data[1] =
            LaunchBallFadeMonTaskForPokeball(TRUE, monPalNum, selectedPalettes) as i16;
        (*sprite).callback = Some(SpriteCB_TradePokeballSendOff);
        StartSpriteAffineAnim(&raw mut gSprites[monSpriteId], BATTLER_AFFINE_RETURN);
        AnimateSprite(&raw mut gSprites[monSpriteId]);
        gSprites[monSpriteId].data[1] = 0;
    } else {
        (*sprite).data[1] -= 1;
    }
}
pub(crate) unsafe fn SpriteCB_TradePokeballSendOff(sprite: *mut Sprite) {
    (*sprite).data[sTimer] += 1;
    if (*sprite).data[sTimer] == 11 {
        PlaySE(SE_BALL_TRADE);
    }
    let monSpriteId: u8 = (*sprite).data[sMonSpriteId] as u8;
    if gSprites[monSpriteId].affineAnimEnded() != 0 {
        StartSpriteAnim(sprite, 2);
        gSprites[monSpriteId].set_invisible(TRUE as u16);
        (*sprite).data[sTimer] = 0;
        (*sprite).callback = Some(SpriteCB_TradePokeballEnd);
    } else {
        gSprites[monSpriteId].data[1] += 96;
        gSprites[monSpriteId].y2 = (-(gSprites[monSpriteId].data[1] as i32) >> 8) as i16;
    }
}
pub(crate) unsafe fn SpriteCB_TradePokeballEnd(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
unsafe fn DestroySpriteAndFreeResources_Ball(sprite: *mut Sprite) {
    DestroySpriteAndFreeResources(sprite);
}
pub unsafe fn StartHealthboxSlideIn(battler: u8) {
    let healthboxSprite: *mut Sprite = &raw mut gSprites[gHealthboxSpriteIds[battler]];
    (*healthboxSprite).data[sSpeedX] = 5;
    (*healthboxSprite).data[sSpeedY] = 0;
    (*healthboxSprite).x2 = 0x73;
    (*healthboxSprite).y2 = 0;
    (*healthboxSprite).callback = Some(SpriteCB_HealthboxSlideIn);
    if GetBattlerSide(battler) != B_SIDE_PLAYER {
        (*healthboxSprite).data[sSpeedX] = -(*healthboxSprite).data[sSpeedX];
        (*healthboxSprite).data[sSpeedY] = -(*healthboxSprite).data[sSpeedY];
        (*healthboxSprite).x2 = -(*healthboxSprite).x2;
        (*healthboxSprite).y2 = -(*healthboxSprite).y2;
    }
    gSprites[(*healthboxSprite).data[5]]
        .callback
        .unwrap_unchecked()(&raw mut gSprites[(*healthboxSprite).data[5]]);
    if GetBattlerPosition(battler) == B_POSITION_PLAYER_RIGHT {
        (*healthboxSprite).callback = Some(SpriteCB_HealthboxSlideInDelayed);
    }
}
pub(crate) unsafe fn SpriteCB_HealthboxSlideInDelayed(sprite: *mut Sprite) {
    (*sprite).data[sDelayTimer] += 1;
    if (*sprite).data[sDelayTimer] == 20 {
        (*sprite).data[sDelayTimer] = 0;
        (*sprite).callback = Some(SpriteCB_HealthboxSlideIn);
    }
}
pub(crate) unsafe fn SpriteCB_HealthboxSlideIn(sprite: *mut Sprite) {
    (*sprite).x2 -= (*sprite).data[sSpeedX];
    (*sprite).y2 -= (*sprite).data[sSpeedY];
    if (*sprite).x2 == 0 && (*sprite).y2 == 0 {
        (*sprite).callback = Some(SpriteCallbackDummy);
    }
}
pub unsafe fn DoHitAnimHealthboxEffect(battler: u8) {
    let spriteId: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCB_HitAnimHealthoxEffect));
    gSprites[spriteId].data[0] = 1;
    gSprites[spriteId].data[1] = gHealthboxSpriteIds[battler] as i16;
    gSprites[spriteId].callback = Some(SpriteCB_HitAnimHealthoxEffect);
}
pub(crate) unsafe fn SpriteCB_HitAnimHealthoxEffect(sprite: *mut Sprite) {
    let r1: u8 = (*sprite).data[1] as u8;
    gSprites[r1].y2 = (*sprite).data[0];
    (*sprite).data[0] = -(*sprite).data[0];
    (*sprite).data[2] += 1;
    if (*sprite).data[2] == 21 {
        gSprites[r1].x2 = 0;
        gSprites[r1].y2 = 0;
        DestroySprite(sprite);
    }
}
pub unsafe fn LoadBallGfx(ballId: u8) {
    let mut var: u16 = 0;
    if GetSpriteTileStartByTag(gBallSpriteSheets[ballId].tag) == 0xFFFF {
        LoadCompressedSpriteSheetUsingHeap((&raw const gBallSpriteSheets[ballId]).cast_mut());
        LoadCompressedSpritePaletteUsingHeap((&raw const gBallSpritePalettes[ballId]).cast_mut());
    }
    match ballId {
        BALL_DIVE | BALL_LUXURY | BALL_PREMIER => {}
        _ => {
            var = GetSpriteTileStartByTag(gBallSpriteSheets[ballId].tag);
            LZDecompressVram(
                (*(&raw const crate::data::graphics::gOpenPokeballGfx).cast::<CArray<u32, 0>>())
                    .as_ptr()
                    .cast_mut(),
                (0x6010100 + var as i32 * 32) as usize as *mut c_void,
            );
        }
    }
}
pub unsafe fn FreeBallGfx(ballId: u8) {
    FreeSpriteTilesByTag(gBallSpriteSheets[ballId].tag);
    FreeSpritePaletteByTag(gBallSpritePalettes[ballId].tag);
}
unsafe fn GetBattlerPokeballItemId(battler: u8) -> u16 {
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        return GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[battler]],
            MON_DATA_POKEBALL,
        ) as u16;
    } else {
        return GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[battler]],
            MON_DATA_POKEBALL,
        ) as u16;
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
