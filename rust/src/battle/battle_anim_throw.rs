//! Translated from `src/battle_anim_throw.c` by tools/rustport/c2rs.py.
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
    clippy::type_complexity,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::agb_main::gMain;
use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible, IsContest, ResetBattleAnimBg,
    gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimLoadCompressedBgGfx, AnimLoadCompressedBgTilemap, GetBattleAnimBg1Data,
    GetBattlePalettesMask, GetBattlerAtPosition, GetBattlerSide, GetBattlerSpriteBGPriorityRank,
    GetBattlerSpriteCoord, GetBattlerSpriteSubpriority, InitAnimArcTranslation,
    InitSpritePosToAnimAttacker, PrepareBattlerSpriteForRotScale, ResetSpriteRotScale,
    SetBattlerSpriteYOffsetFromYScale, SetSpriteRotScale, TranslateAnimHorizontalArc,
};
use crate::battle_anim_utility_funcs::SetAnimBgAttribute;
use crate::battle_gfx_sfx_util::{ClearBehindSubstituteBit, LoadBattleMonGfxAndAnimate};
use crate::battle_interface::UpdateOamPriorityInAllHealthboxes;
use crate::battle_main::{
    SpriteCB_TrainerThrowObject, gBattle_BG1_X, gBattle_BG1_Y, gBattle_WIN0H, gBattle_WIN0V,
    gBattleSpritesDataPtr, gBattleTypeFlags, gDoingBattleAnim, gEffectBattler, gLastUsedItem,
};
use crate::battle_main::{gBattlerPartyIndexes, gBattlerSpriteIds, gHealthboxSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::m4a::m4aMPlayAllStop;
use crate::palette::{
    BeginNormalPaletteFade, BlendPalettes, LoadCompressedPalette, LoadPalette, gPaletteFade,
};
use crate::pokeball::{FreeBallGfx, LoadBallGfx};
use crate::pokemon::{GetMonData2, gEnemyParty, gPlayerParty};
use crate::sound::{PlaySE, PlaySE12WithPanning};
use crate::sprite::gSprites;
use crate::sprite::{
    AllocSpritePalette, FreeOamMatrix, FreeSpritePaletteByTag, FreeSpriteTilesByTag,
    GetSpriteTileStartByTag, IndexOfSpritePaletteTag,
};
use crate::task::{DestroyTask, TaskDummy};
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
/// `FuncIsActiveTask` with this module's view of its types.
#[inline]
unsafe fn FuncIsActiveTask(a0: Option<unsafe fn(u8)>) -> u8 {
    unsafe { crate::task::FuncIsActiveTask(core::mem::transmute(a0)) }
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
/// `RequestDma3Fill` with this module's view of its types.
#[inline]
unsafe fn RequestDma3Fill(a0: i32, a1: *mut c_void, a2: u16, a3: u8) -> i16 {
    unsafe { crate::dma3_manager::RequestDma3Fill(a0, a1 as _, a2, a3) }
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
const sDuration: usize = 0;
const sFrame: usize = 0;
const sTaskId: usize = 0;
const tBattler: usize = 0;
const tCoeff: usize = 0;
const tSpriteId: usize = 0;
const tState: usize = 0;
const sDx: usize = 1;
const tStarMove: usize = 1;
const tdCoeff: usize = 1;
const tPalOffset: usize = 3;
const sDirection: usize = 4;
const tPaletteLo: usize = 10;
const tStarTimer: usize = 10;
const tPaletteHi: usize = 11;
const tStarIdx: usize = 11;
const tNumStars: usize = 12;
const tBallId: usize = 15;
// Data tables (translate with cdata.py): sCaptureStars sBallParticleSpriteSheets sBallParticlePalettes sAnim_RegularBall sAnim_MasterBall sAnim_NetDiveBall sAnim_NestBall sAnim_LuxuryPremierBall sAnim_UltraRepeatTimerBall sAnims_BallParticles sBallParticleAnimNums sBallParticleAnimationFuncs sBallParticleSpriteTemplates gBallOpenFadeColors gPokeblockSpriteTemplate sAnim_SafariRock sAnims_SafariRock sSafariRockSpriteTemplate

/// `struct CaptureStar`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct CaptureStar {
    pub xOffset: i8,
    pub yOffset: i8,
    pub amplitude: i8,
}

unsafe impl Sync for CaptureStar {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<CaptureStar>() == 4);
    assert!(offset_of!(CaptureStar, xOffset) == 0);
    assert!(offset_of!(CaptureStar, yOffset) == 1);
    assert!(offset_of!(CaptureStar, amplitude) == 2);
};

const BALL_FALLING: i32 = 0;
const BALL_NEXT_MOVE: i32 = 5;
const BALL_PIVOT_1: i32 = 1;
const BALL_PIVOT_2: i32 = 3;
const BALL_RISING: i32 = 1;
const BALL_ROLL_1: i32 = 0;
const BALL_ROLL_2: i32 = 2;
const BALL_ROLL_3: i32 = 4;
const BALL_WAIT_NEXT_SHAKE: i32 = 6;
const MON_SHRINK: i16 = 0;
const MON_SHRINK_INVISIBLE: i16 = 2;
const MON_SHRINK_STEP: i16 = 1;
const SHINY_STAR_DIAGONAL: i16 = 1;
const SHINY_STAR_ENCIRCLE: i16 = 0;

static gBallOpenFadeColors: Table<CArray<u16, 20>> =
    Table((&raw const crate::data::battle_anim_throw::gBallOpenFadeColors).cast());
static sBallParticleAnimNums: Table<CArray<u8, 12>> =
    Table((&raw const crate::data::battle_anim_throw::sBallParticleAnimNums).cast());
static sBallParticleAnimationFuncs: Table<CArray<Option<unsafe fn(u8)>, 12>> =
    Table((&raw const crate::data::battle_anim_throw::sBallParticleAnimationFuncs).cast());
static sBallParticlePalettes: Table<CArray<CompressedSpritePalette, 12>> =
    Table((&raw const crate::data::battle_anim_throw::sBallParticlePalettes).cast());
static sBallParticleSpriteSheets: Table<CArray<CompressedSpriteSheet, 12>> =
    Table((&raw const crate::data::battle_anim_throw::sBallParticleSpriteSheets).cast());
static sBallParticleSpriteTemplates: Table<CArray<SpriteTemplate, 12>> =
    Table((&raw const crate::data::battle_anim_throw::sBallParticleSpriteTemplates).cast());
static sCaptureStars: Table<CArray<CaptureStar, 3>> =
    Table((&raw const crate::data::battle_anim_throw::sCaptureStars).cast());

#[unsafe(link_section = "common_data")]
pub static gMonShrinkDuration: crate::global::Global<u32> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gMonShrinkDelta: crate::global::Global<u16> = crate::global::Global::new(0);
#[unsafe(link_section = "common_data")]
pub static gMonShrinkDistance: crate::global::Global<u16> = crate::global::Global::new(0);

pub unsafe fn AnimTask_UnusedLevelUpHealthBox(taskId: u8) {
    let mut animBgData: BattleAnimBgData = zeroed();
    let battler: u8 = gBattleAnimAttacker;
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WININ, 16191);
    SetGpuReg(REG_OFFSET_WINOUT, 16189);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    SetGpuReg(REG_OFFSET_BLDCNT, 16194);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 0);
    SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
    SetAnimBgAttribute(1, BG_ANIM_AREA_OVERFLOW_MODE, 1);
    SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
    let healthBoxSpriteId: u8 = gHealthboxSpriteIds[battler];
    let spriteId1: u8 = gSprites[healthBoxSpriteId].oam.affineParam as u8;
    let spriteId2: u8 = gSprites[healthBoxSpriteId].data[5] as u8;
    let spriteId3: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
    let spriteId4: u8 = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
    gSprites[healthBoxSpriteId].oam.set_priority(1);
    gSprites[spriteId1].oam.set_priority(1);
    gSprites[spriteId2].oam.set_priority(1);
    gSprites[spriteId3] = gSprites[healthBoxSpriteId];
    gSprites[spriteId4] = gSprites[spriteId1];
    gSprites[spriteId3].oam.set_objMode(ST_OAM_OBJ_WINDOW);
    gSprites[spriteId4].oam.set_objMode(ST_OAM_OBJ_WINDOW);
    gSprites[spriteId3].callback = Some(SpriteCallbackDummy);
    gSprites[spriteId4].callback = Some(SpriteCallbackDummy);
    GetBattleAnimBg1Data(&raw mut animBgData);
    AnimLoadCompressedBgTilemap(
        animBgData.bgId as u32,
        (*(&raw const crate::data::graphics::UnusedLevelupAnimationTilemap)
            .cast::<CArray<u32, 0>>())
        .as_ptr()
        .cast_mut() as *mut c_void,
    );
    AnimLoadCompressedBgGfx(
        animBgData.bgId as u32,
        (*(&raw const crate::data::graphics::UnusedLevelupAnimationGfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBgData.tilesOffset as u32,
    );
    LoadCompressedPalette(
        (*(&raw const crate::data::graphics::gCureBubblesPal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        animBgData.paletteId as u16 * 16,
        32,
    );
    gBattle_BG1_X = (gSprites[spriteId3].x as u16).wrapping_neg() + 32;
    gBattle_BG1_Y = (gSprites[spriteId3].y as u16).wrapping_neg() - 32;
    task_set(taskId, 1, 640);
    task_set(taskId, 0, spriteId3 as i16);
    task_set(taskId, 2, spriteId4 as i16);
    task_set_func(taskId, Some(AnimTask_UnusedLevelUpHealthBox_Step));
}
pub(crate) unsafe fn AnimTask_UnusedLevelUpHealthBox_Step(taskId: u8) {
    let mut spriteId1: u8 = 0;
    let mut spriteId2: u8 = 0;
    let battler: u8 = gBattleAnimAttacker;
    task_set(taskId, 13, task_get(taskId, 13) + (task_get(taskId, 1)));
    gBattle_BG1_Y += task_get(taskId, 13) as u16 >> 8;
    task_set(taskId, 13, task_get(taskId, 13) & 0xFF);
    match task_get(taskId, 15) {
        0 => {
            if ({
                let t1 = task_get(taskId, 11);
                task_set(taskId, 11, task_get(taskId, 11) + 1);
                t1
            }) > 1
            {
                task_set(taskId, 11, 0);
                task_set(taskId, 12, task_get(taskId, 12) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 12) as u16) << 8 | task_get(taskId, 12) as u16,
                );
                if task_get(taskId, 12) == 8 {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            }
        }
        1 => {
            if ({
                task_set(taskId, 10, task_get(taskId, 10) + 1);
                task_get(taskId, 10)
            }) == 30
            {
                task_set(taskId, 15, task_get(taskId, 15) + 1);
            }
        }
        2 if ({
            let t3 = task_get(taskId, 11);
            task_set(taskId, 11, task_get(taskId, 11) + 1);
            t3
        }) > 1 =>
        {
            task_set(taskId, 11, 0);
            task_set(taskId, 12, task_get(taskId, 12) - 1);
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (16 - task_get(taskId, 12) as u16) << 8 | task_get(taskId, 12) as u16,
            );
            if task_get(taskId, 12) == 0 {
                ResetBattleAnimBg(FALSE);
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
                DestroySprite(&raw mut gSprites[task_get(taskId, 0)]);
                DestroySprite(&raw mut gSprites[task_get(taskId, 2)]);
                SetAnimBgAttribute(1, BG_ANIM_AREA_OVERFLOW_MODE, 0);
                spriteId1 = gSprites[gHealthboxSpriteIds[battler]].oam.affineParam as u8;
                spriteId2 = gSprites[gHealthboxSpriteIds[battler]].data[5] as u8;
                gSprites[gHealthboxSpriteIds[battler]].oam.set_priority(1);
                gSprites[spriteId1].oam.set_priority(1);
                gSprites[spriteId2].oam.set_priority(1);
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
unsafe fn LoadHealthboxPalsForLevelUp(paletteId1: *mut u8, paletteId2: *mut u8, battler: u8) {
    let healthBoxSpriteId: u8 = gHealthboxSpriteIds[battler];
    let spriteId1: u8 = gSprites[healthBoxSpriteId].oam.affineParam as u8;
    let spriteId2: u8 = gSprites[healthBoxSpriteId].data[5] as u8;
    *paletteId1 = AllocSpritePalette(TAG_HEALTHBOX_PALS_1);
    *paletteId2 = AllocSpritePalette(TAG_HEALTHBOX_PALS_2);
    let offset1: u16 = 0x100 + gSprites[healthBoxSpriteId].oam.paletteNum() * 16;
    let offset2: u16 = 0x100 + gSprites[spriteId2].oam.paletteNum() * 16;
    LoadPalette(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[offset1] as *mut c_void,
        0x100 + *paletteId1 as u16 * 16,
        32,
    );
    LoadPalette(
        &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
            .cast::<CArray<u16, 512>>()
            .cast_mut())[offset2] as *mut c_void,
        0x100 + *paletteId2 as u16 * 16,
        32,
    );
    gSprites[healthBoxSpriteId]
        .oam
        .set_paletteNum(*paletteId1 as u16);
    gSprites[spriteId1].oam.set_paletteNum(*paletteId1 as u16);
    gSprites[spriteId2].oam.set_paletteNum(*paletteId2 as u16);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_LoadHealthboxPalsForLevelUp(taskId: u8) {
    let mut paletteId1: u8 = 0;
    let mut paletteId2: u8 = 0;
    LoadHealthboxPalsForLevelUp(
        &raw mut paletteId1,
        &raw mut paletteId2,
        gBattleAnimAttacker,
    );
    DestroyAnimVisualTask(taskId);
}
unsafe fn FreeHealthboxPalsForLevelUp(battler: u8) {
    let healthBoxSpriteId: u8 = gHealthboxSpriteIds[battler];
    let spriteId1: u8 = gSprites[healthBoxSpriteId].oam.affineParam as u8;
    let spriteId2: u8 = gSprites[healthBoxSpriteId].data[5] as u8;
    FreeSpritePaletteByTag(TAG_HEALTHBOX_PALS_1);
    FreeSpritePaletteByTag(TAG_HEALTHBOX_PALS_2);
    let paletteId1: u8 = IndexOfSpritePaletteTag(TAG_HEALTHBOX_PAL);
    let paletteId2: u8 = IndexOfSpritePaletteTag(TAG_HEALTHBAR_PAL);
    gSprites[healthBoxSpriteId]
        .oam
        .set_paletteNum(paletteId1 as u16);
    gSprites[spriteId1].oam.set_paletteNum(paletteId1 as u16);
    gSprites[spriteId2].oam.set_paletteNum(paletteId2 as u16);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FreeHealthboxPalsForLevelUp(taskId: u8) {
    FreeHealthboxPalsForLevelUp(gBattleAnimAttacker);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FlashHealthboxOnLevelUp(taskId: u8) {
    task_set(taskId, 10, gBattleAnimArgs[0]);
    task_set(taskId, 11, gBattleAnimArgs[1]);
    task_set_func(taskId, Some(AnimTask_FlashHealthboxOnLevelUp_Step));
}
pub(crate) unsafe fn AnimTask_FlashHealthboxOnLevelUp_Step(taskId: u8) {
    let mut paletteNum: u8 = 0;
    let mut paletteOffset: u32 = 0;
    let mut colorOffset: u32 = 0;
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    if ({
        let t1 = task_get(taskId, 0);
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        t1
    }) >= task_get(taskId, 11)
    {
        task_set(taskId, 0, 0);
        paletteNum = IndexOfSpritePaletteTag(TAG_HEALTHBOX_PALS_1);
        colorOffset = (if task_get(taskId, 10) == 0 { 6 } else { 2 }) as u32;
        match task_get(taskId, 1) {
            0 => {
                task_set(taskId, 2, task_get(taskId, 2) + 2);
                if task_get(taskId, 2) > 16 {
                    task_set(taskId, 2, 16);
                }
                paletteOffset = 0x100 + paletteNum as u32 * 16;
                BlendPalette(
                    paletteOffset as u16 + colorOffset as u16,
                    1,
                    task_get(taskId, 2) as u8,
                    32628,
                );
                if task_get(taskId, 2) == 16 {
                    task_set(taskId, 1, task_get(taskId, 1) + 1);
                }
            }
            1 => {
                task_set(taskId, 2, task_get(taskId, 2) - 2);
                if task_get(taskId, 2) < 0 {
                    task_set(taskId, 2, 0);
                }
                paletteOffset = 0x100 + paletteNum as u32 * 16;
                BlendPalette(
                    paletteOffset as u16 + colorOffset as u16,
                    1,
                    task_get(taskId, 2) as u8,
                    32628,
                );
                if task_get(taskId, 2) == 0 {
                    DestroyAnimVisualTask(taskId);
                }
            }
            _ => {}
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SwitchOutShrinkMon(taskId: u8) {
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimAttacker];
    match task_get(taskId, 0) {
        0 => {
            PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
            task_set(taskId, 10, 0x100);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 => {
            task_set(taskId, 10, task_get(taskId, 10) + 0x30);
            SetSpriteRotScale(spriteId, task_get(taskId, 10), task_get(taskId, 10), 0);
            SetBattlerSpriteYOffsetFromYScale(spriteId);
            if task_get(taskId, 10) >= 0x2D0 {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        2 => {
            ResetSpriteRotScale(spriteId);
            gSprites[spriteId].set_invisible(TRUE as u16);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SwitchOutBallEffect(taskId: u8) {
    let mut ball: u16 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut priority: u8 = 0;
    let mut subpriority: u8 = 0;
    let mut selectedPalettes: u32 = 0;
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimAttacker];
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        ball = GetMonData2(
            &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
            MON_DATA_POKEBALL,
        ) as u16;
    } else {
        ball = GetMonData2(
            &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
            MON_DATA_POKEBALL,
        ) as u16;
    }
    let ballId: u8 = ItemIdToBallId(ball);
    match task_get(taskId, 0) {
        0 => {
            x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X);
            y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y);
            priority = gSprites[spriteId].oam.priority() as u8;
            subpriority = gSprites[spriteId].subpriority;
            task_set(
                taskId,
                10,
                AnimateBallOpenParticles(x, y + 32, priority, subpriority, ballId) as i16,
            );
            selectedPalettes =
                GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE);
            task_set(
                taskId,
                11,
                LaunchBallFadeMonTask(FALSE, gBattleAnimAttacker, selectedPalettes, ballId) as i16,
            );
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 if (*gTasks.as_ptr())[task_get(taskId, 10)].isActive == 0
            && (*gTasks.as_ptr())[task_get(taskId, 11)].isActive == 0 =>
        {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_LoadBallGfx(taskId: u8) {
    let ballId: u8 = ItemIdToBallId(gLastUsedItem);
    LoadBallGfx(ballId);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FreeBallGfx(taskId: u8) {
    let ballId: u8 = ItemIdToBallId(gLastUsedItem);
    FreeBallGfx(ballId);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsBallBlockedByTrainer(taskId: u8) {
    if (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId == BALL_TRAINER_BLOCK {
        gBattleAnimArgs[7] = -1;
    } else {
        gBattleAnimArgs[7] = 0;
    }
    DestroyAnimVisualTask(taskId);
}
pub unsafe fn ItemIdToBallId(ballItem: u16) -> u8 {
    match ballItem {
        ITEM_MASTER_BALL => {
            return BALL_MASTER;
        }
        2 => {
            return BALL_ULTRA;
        }
        ITEM_GREAT_BALL => {
            return BALL_GREAT;
        }
        ITEM_SAFARI_BALL => {
            return BALL_SAFARI;
        }
        ITEM_NET_BALL => {
            return BALL_NET;
        }
        ITEM_DIVE_BALL => {
            return BALL_DIVE;
        }
        ITEM_NEST_BALL => {
            return BALL_NEST;
        }
        ITEM_REPEAT_BALL => {
            return BALL_REPEAT;
        }
        ITEM_TIMER_BALL => {
            return BALL_TIMER;
        }
        11 => {
            return BALL_LUXURY;
        }
        ITEM_PREMIER_BALL => {
            return BALL_PREMIER;
        }
        _ => {
            return BALL_POKE;
        }
    }
    #[allow(unreachable_code)]
    {
        0
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ThrowBall(taskId: u8) {
    let ballId: u8 = ItemIdToBallId(gLastUsedItem);
    let spriteId: u8 = CreateSprite(
        (&raw const (*(&raw const crate::data::pokeball::gBallSpriteTemplates)
            .cast::<CArray<SpriteTemplate, 0>>())[ballId])
            .cast_mut(),
        32,
        80,
        29,
    );
    gSprites[spriteId].data[0] = 34;
    gSprites[spriteId].data[1] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
    gSprites[spriteId].data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 - 16;
    gSprites[spriteId].callback = Some(SpriteCB_Ball_Throw);
    (*(*gBattleSpritesDataPtr).animationData)
        .set_wildMonInvisible(gSprites[gBattlerSpriteIds[gBattleAnimTarget]].invisible() as u8);
    task_set(taskId, 0, spriteId as i16);
    task_set_func(taskId, Some(AnimTask_ThrowBall_Step));
}
pub(crate) unsafe fn AnimTask_ThrowBall_Step(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 0) as u8;
    if gSprites[spriteId].data[0] as u16 == 0xFFFF {
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ThrowBall_StandingTrainer(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    if gBattleTypeFlags & BATTLE_TYPE_WALLY_TUTORIAL != 0 {
        x = 32;
        y = 11;
    } else {
        x = 23;
        y = 5;
    }
    let ballId: u8 = ItemIdToBallId(gLastUsedItem);
    let subpriority: u8 =
        GetBattlerSpriteSubpriority(GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT)) + 1;
    let spriteId: u8 = CreateSprite(
        (&raw const (*(&raw const crate::data::pokeball::gBallSpriteTemplates)
            .cast::<CArray<SpriteTemplate, 0>>())[ballId])
            .cast_mut(),
        x + 32,
        y | 80,
        subpriority,
    );
    gSprites[spriteId].data[0] = 34;
    gSprites[spriteId].data[1] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
    gSprites[spriteId].data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 - 16;
    gSprites[spriteId].callback = Some(SpriteCallbackDummy);
    gSprites[gBattlerSpriteIds[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]].callback =
        Some(SpriteCB_TrainerThrowObject);
    task_set(taskId, 0, spriteId as i16);
    task_set_func(taskId, Some(AnimTask_ThrowBall_StandingTrainer_Step));
}
pub(crate) unsafe fn AnimTask_ThrowBall_StandingTrainer_Step(taskId: u8) {
    if gSprites[gBattlerSpriteIds[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]].animCmdIndex == 1 {
        PlaySE12WithPanning(SE_BALL_THROW, 0);
        gSprites[task_get(taskId, tSpriteId)].callback = Some(SpriteCB_Ball_Throw);
        CreateTask(Some(Task_PlayerThrow_Wait), 10);
        task_set_func(taskId, Some(AnimTask_ThrowBall_Step));
    }
}
pub(crate) unsafe fn Task_PlayerThrow_Wait(taskId: u8) {
    if gSprites[gBattlerSpriteIds[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]].animEnded() != 0 {
        StartSpriteAnim(
            &raw mut gSprites[gBattlerSpriteIds[GetBattlerAtPosition(B_POSITION_PLAYER_LEFT)]],
            0,
        );
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Throw(sprite: *mut Sprite) {
    let targetX: u16 = (*sprite).data[1] as u16;
    let targetY: u16 = (*sprite).data[2] as u16;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = targetX as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = targetY as i16;
    (*sprite).data[5] = -40;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(SpriteCB_Ball_Arc);
}
pub(crate) unsafe fn SpriteCB_Ball_Arc(sprite: *mut Sprite) {
    let mut ballId: u8 = 0;
    if TranslateAnimHorizontalArc(sprite) != 0 {
        if (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId == BALL_TRAINER_BLOCK {
            (*sprite).callback = Some(SpriteCB_Ball_Block);
        } else {
            StartSpriteAnim(sprite, 1);
            (*sprite).x += (*sprite).x2;
            (*sprite).y += (*sprite).y2;
            (*sprite).x2 = 0;
            (*sprite).y2 = 0;
            for i in 0..8i32 {
                (*sprite).data[i] = 0;
            }
            (*sprite).data[5] = 0;
            (*sprite).callback = Some(SpriteCB_Ball_MonShrink);
            ballId = ItemIdToBallId(gLastUsedItem);
            if let 0..=11 = ballId {
                AnimateBallOpenParticles((*sprite).x as u8, (*sprite).y as u8 - 5, 1, 28, ballId);
                LaunchBallFadeMonTask(FALSE, gBattleAnimTarget, 14, ballId);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_Ball_MonShrink(sprite: *mut Sprite) {
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == 10
    {
        (*sprite).data[5] = CreateTask(Some(TaskDummy), 50) as i16;
        (*sprite).callback = Some(SpriteCB_Ball_MonShrink_Step);
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]].data[1] = 0;
    }
}
pub(crate) unsafe fn SpriteCB_Ball_MonShrink_Step(sprite: *mut Sprite) {
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimTarget];
    let taskId: u8 = (*sprite).data[5] as u8;
    if ({
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        task_get(taskId, 1)
    }) == 11
    {
        PlaySE(SE_BALL_TRADE);
    }
    match task_get(taskId, tState) {
        MON_SHRINK => {
            PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
            task_set(taskId, 10, 256);
            gMonShrinkDuration.set(28);
            gMonShrinkDistance.set(
                gSprites[spriteId].y as u16 + gSprites[spriteId].y2 as u16
                    - ((*sprite).y as u16 + (*sprite).y2 as u16),
            );
            gMonShrinkDelta.set(div_u32(
                gMonShrinkDistance.get() as u32 * 256,
                gMonShrinkDuration.get(),
            ) as u16);
            task_set(taskId, 2, gMonShrinkDelta.get() as i16);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        MON_SHRINK_STEP => {
            task_set(taskId, 10, task_get(taskId, 10) + 32);
            SetSpriteRotScale(spriteId, task_get(taskId, 10), task_get(taskId, 10), 0);
            task_set(taskId, 3, task_get(taskId, 3) + (task_get(taskId, 2)));
            gSprites[spriteId].y2 = (-(task_get(taskId, 3) as i32) >> 8) as i16;
            if task_get(taskId, 10) >= 1152 {
                task_set(taskId, tState, task_get(taskId, tState) + 1);
            }
        }
        MON_SHRINK_INVISIBLE => {
            ResetSpriteRotScale(spriteId);
            gSprites[spriteId].set_invisible(TRUE as u16);
            task_set(taskId, tState, task_get(taskId, tState) + 1);
        }
        _ => {
            if task_get(taskId, 1) > 10 {
                DestroyTask(taskId);
                StartSpriteAnim(sprite, 2);
                (*sprite).data[5] = 0;
                (*sprite).callback = Some(SpriteCB_Ball_Bounce);
            }
        }
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Bounce(sprite: *mut Sprite) {
    let mut phase: i16 = 0;
    if (*sprite).animEnded() != 0 {
        (*sprite).data[3] = 0;
        (*sprite).data[4] = 40;
        (*sprite).data[5] = 0;
        phase = 0;
        (*sprite).y += Cos(phase, 40);
        (*sprite).y2 = -Cos(phase, (*sprite).data[4]);
        (*sprite).callback = Some(SpriteCB_Ball_Bounce_Step);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Bounce_Step(sprite: *mut Sprite) {
    let mut bounceCount: i16 = 0;
    let mut lastBounce: u8 = FALSE;
    match (*sprite).data[3] as i32 & 0xFF {
        BALL_FALLING => {
            (*sprite).y2 = -Cos((*sprite).data[5], (*sprite).data[4]);
            (*sprite).data[5] += ((*sprite).data[3] >> 8) + 4;
            if (*sprite).data[5] >= 64 {
                (*sprite).data[4] -= 10;
                (*sprite).data[3] += 257;
                bounceCount = (*sprite).data[3] >> 8;
                if bounceCount == 4 {
                    lastBounce = TRUE;
                }
                match bounceCount {
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
        BALL_RISING => {
            (*sprite).y2 = -Cos((*sprite).data[5], (*sprite).data[4]);
            (*sprite).data[5] -= ((*sprite).data[3] >> 8) + 4;
            if (*sprite).data[5] <= 0 {
                (*sprite).data[5] = 0;
                (*sprite).data[3] &= -256;
            }
        }
        _ => {}
    }
    if lastBounce != 0 {
        (*sprite).data[3] = 0;
        (*sprite).y += Cos(64, 40);
        (*sprite).y2 = 0;
        if (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId == BALL_NO_SHAKES {
            (*sprite).data[5] = 0;
            (*sprite).callback = Some(SpriteCB_Ball_Release);
        } else {
            (*sprite).callback = Some(SpriteCB_Ball_Wobble);
            (*sprite).data[4] = 1;
            (*sprite).data[5] = 0;
        }
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Wobble(sprite: *mut Sprite) {
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == 31
    {
        (*sprite).data[3] = 0;
        (*sprite).set_affineAnimPaused(TRUE);
        StartSpriteAffineAnim(sprite, BALL_ROTATE_RIGHT);
        (*(*gBattleSpritesDataPtr).animationData).ballSubpx = 0;
        (*sprite).callback = Some(SpriteCB_Ball_Wobble_Step);
        PlaySE(SE_BALL);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Wobble_Step(sprite: *mut Sprite) {
    let mut shakes: i8 = 0;
    let mut frame: u16 = 0;
    'l1: {
        let sw1: i32 = (*sprite).data[3] as i32 & 0xFF;
        let matched = sw1 == BALL_ROLL_1
            || sw1 == BALL_PIVOT_1
            || sw1 == BALL_ROLL_2
            || sw1 == BALL_PIVOT_2
            || sw1 == BALL_ROLL_3
            || sw1 == BALL_NEXT_MOVE
            || sw1 == BALL_WAIT_NEXT_SHAKE;
        let mut fall = false;
        if sw1 == BALL_ROLL_1 {
            if (*(*gBattleSpritesDataPtr).animationData).ballSubpx > 255 {
                (*sprite).x2 += (*sprite).data[sDirection];
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx &= 0xFF;
            } else {
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx += 176;
            }
            (*sprite).data[5] += 1;
            (*sprite).set_affineAnimPaused(FALSE);
            frame = (*sprite).data[5] as u16 + 7;
            if frame > 14 {
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx = 0;
                (*sprite).data[3] += 1;
                (*sprite).data[5] = 0;
            }
            break 'l1;
        }
        if sw1 == BALL_PIVOT_1 {
            if ({
                (*sprite).data[5] += 1;
                (*sprite).data[5]
            }) == 1
            {
                (*sprite).data[5] = 0;
                (*sprite).data[sDirection] = -(*sprite).data[sDirection];
                (*sprite).data[3] += 1;
                (*sprite).set_affineAnimPaused(FALSE);
                if (*sprite).data[sDirection] < 0 {
                    ChangeSpriteAffineAnim(sprite, BALL_ROTATE_LEFT);
                } else {
                    ChangeSpriteAffineAnim(sprite, BALL_ROTATE_RIGHT);
                }
            } else {
                (*sprite).set_affineAnimPaused(TRUE);
            }
            break 'l1;
        }
        if sw1 == BALL_ROLL_2 {
            if (*(*gBattleSpritesDataPtr).animationData).ballSubpx > 255 {
                (*sprite).x2 += (*sprite).data[sDirection];
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx &= 0xFF;
            } else {
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx += 176;
            }
            (*sprite).data[5] += 1;
            (*sprite).set_affineAnimPaused(FALSE);
            frame = (*sprite).data[5] as u16 + 12;
            if frame > 24 {
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx = 0;
                (*sprite).data[3] += 1;
                (*sprite).data[5] = 0;
            }
            break 'l1;
        }
        if sw1 == BALL_PIVOT_2 {
            fall = true;
            if ({
                let t3 = (*sprite).data[5];
                (*sprite).data[5] += 1;
                t3
            }) < 0
            {
                (*sprite).set_affineAnimPaused(TRUE);
                break 'l1;
            }
            (*sprite).data[5] = 0;
            (*sprite).data[sDirection] = -(*sprite).data[sDirection];
            (*sprite).data[3] += 1;
            (*sprite).set_affineAnimPaused(FALSE);
            if (*sprite).data[sDirection] < 0 {
                ChangeSpriteAffineAnim(sprite, BALL_ROTATE_LEFT);
            } else {
                ChangeSpriteAffineAnim(sprite, BALL_ROTATE_RIGHT);
            }
        }
        if fall || sw1 == BALL_ROLL_3 {
            if (*(*gBattleSpritesDataPtr).animationData).ballSubpx > 0xFF {
                (*sprite).x2 += (*sprite).data[sDirection];
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx &= 0xFF;
            } else {
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx += 176;
            }
            (*sprite).data[5] += 1;
            (*sprite).set_affineAnimPaused(FALSE);
            frame = (*sprite).data[5] as u16 + 4;
            if frame > 8 {
                (*(*gBattleSpritesDataPtr).animationData).ballSubpx = 0;
                (*sprite).data[3] += 1;
                (*sprite).data[5] = 0;
                (*sprite).data[sDirection] = -(*sprite).data[sDirection];
            }
            break 'l1;
        }
        if sw1 == BALL_NEXT_MOVE {
            (*sprite).data[3] += 0x100;
            shakes = ((*sprite).data[3] >> 8) as i8;
            if shakes as i32 == (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId as i32 {
                (*sprite).set_affineAnimPaused(TRUE);
                (*sprite).callback = Some(SpriteCB_Ball_Release);
            } else {
                if (*(*gBattleSpritesDataPtr).animationData).ballThrowCaseId
                    == BALL_3_SHAKES_SUCCESS
                    && shakes == 3
                {
                    (*sprite).callback = Some(SpriteCB_Ball_Capture);
                    (*sprite).set_affineAnimPaused(TRUE);
                } else {
                    (*sprite).data[3] += 1;
                    (*sprite).set_affineAnimPaused(TRUE);
                }
            }
            break 'l1;
        }
        if sw1 == BALL_WAIT_NEXT_SHAKE || !matched {
            if ({
                (*sprite).data[5] += 1;
                (*sprite).data[5]
            }) == 31
            {
                (*sprite).data[5] = 0;
                (*sprite).data[3] &= -256;
                StartSpriteAffineAnim(sprite, 3);
                if (*sprite).data[sDirection] < 0 {
                    StartSpriteAffineAnim(sprite, BALL_ROTATE_LEFT);
                } else {
                    StartSpriteAffineAnim(sprite, BALL_ROTATE_RIGHT);
                }
                PlaySE(SE_BALL);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Release(sprite: *mut Sprite) {
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == 31
    {
        (*sprite).data[5] = 0;
        (*sprite).callback = Some(SpriteCB_Ball_Release_Step);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Capture(sprite: *mut Sprite) {
    (*sprite).set_animPaused(TRUE);
    (*sprite).callback = Some(SpriteCB_Ball_Capture_Step);
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = 0;
}
pub(crate) unsafe fn SpriteCB_Ball_Capture_Step(sprite: *mut Sprite) {
    let battler: *mut u8 = &raw mut gBattleAnimTarget;
    (*sprite).data[4] += 1;
    if (*sprite).data[4] == 40 {
        PlaySE(SE_RG_BALL_CLICK);
        BlendPalettes(
            shl_i32(0x10000, (*sprite).oam.paletteNum() as u32) as u32,
            6,
            0,
        );
        MakeCaptureStars(sprite);
    } else if (*sprite).data[4] == 60 {
        BeginNormalPaletteFade(
            shl_i32(0x10000, (*sprite).oam.paletteNum() as u32) as u32,
            2,
            6,
            0,
            0,
        );
    } else if (*sprite).data[4] == 95 {
        gDoingBattleAnim = FALSE;
        UpdateOamPriorityInAllHealthboxes(1);
        m4aMPlayAllStop();
        PlaySE(MUS_RG_CAUGHT_INTRO);
    } else if (*sprite).data[4] == 315 {
        FreeOamMatrix(gSprites[gBattlerSpriteIds[*battler]].oam.matrixNum() as u8);
        DestroySprite(&raw mut gSprites[gBattlerSpriteIds[*battler]]);
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(SpriteCB_Ball_FadeOut);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_FadeOut(sprite: *mut Sprite) {
    let mut paletteIndex: u8 = 0;
    match (*sprite).data[0] {
        0 => {
            (*sprite).data[1] = 0;
            (*sprite).data[2] = 0;
            (*sprite).oam.set_objMode(ST_OAM_OBJ_BLEND);
            SetGpuReg(REG_OFFSET_BLDCNT, 16192);
            SetGpuReg(REG_OFFSET_BLDALPHA, 16);
            paletteIndex = IndexOfSpritePaletteTag((*(*sprite).template).paletteTag);
            BeginNormalPaletteFade(
                shl_i32(1, paletteIndex as u32 + 0x10) as u32,
                0,
                0,
                16,
                32767,
            );
            (*sprite).data[0] += 1;
        }
        1 => {
            if ({
                let t1 = (*sprite).data[1];
                (*sprite).data[1] += 1;
                t1
            }) > 0
            {
                (*sprite).data[1] = 0;
                (*sprite).data[2] += 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (((*sprite).data[2] as u16) << 8) | (16 - (*sprite).data[2] as u16),
                );
                if (*sprite).data[2] == 16 {
                    (*sprite).data[0] += 1;
                }
            }
        }
        2 => {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).data[0] += 1;
        }
        _ => {
            if gPaletteFade.active() == 0 {
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                (*sprite).data[0] = 0;
                (*sprite).callback = Some(DestroySpriteAfterOneFrame);
            }
        }
    }
}
pub(crate) unsafe fn DestroySpriteAfterOneFrame(sprite: *mut Sprite) {
    if (*sprite).data[sFrame] == 0 {
        (*sprite).data[sFrame] = -1;
    } else {
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
    }
}
unsafe fn MakeCaptureStars(sprite: *mut Sprite) {
    let mut subpriority: u8 = 0;
    if (*sprite).subpriority != 0 {
        subpriority = (*sprite).subpriority - 1;
    } else {
        subpriority = 0;
        (*sprite).subpriority = 1;
    }
    LoadBallParticleGfx(BALL_MASTER);
    for i in 0..3u32 {
        let spriteId: u8 = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[4]).cast_mut(),
            (*sprite).x,
            (*sprite).y,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].data[sDuration] = 24;
            gSprites[spriteId].data[2] = (*sprite).x + sCaptureStars[i].xOffset as i16;
            gSprites[spriteId].data[4] = (*sprite).y + sCaptureStars[i].yOffset as i16;
            gSprites[spriteId].data[5] = sCaptureStars[i].amplitude as i16;
            InitAnimArcTranslation(&raw mut gSprites[spriteId]);
            gSprites[spriteId].callback = Some(SpriteCB_CaptureStar_Flicker);
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[4]);
        }
    }
}
pub(crate) unsafe fn SpriteCB_CaptureStar_Flicker(sprite: *mut Sprite) {
    (*sprite).set_invisible(((*sprite).invisible() == 0) as u16);
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Release_Step(sprite: *mut Sprite) {
    StartSpriteAnim(sprite, 1);
    StartSpriteAffineAnim(sprite, 0);
    (*sprite).callback = Some(SpriteCB_Ball_Release_Wait);
    let ballId: u8 = ItemIdToBallId(gLastUsedItem);
    if let 0..=11 = ballId {
        AnimateBallOpenParticles((*sprite).x as u8, (*sprite).y as u8 - 5, 1, 28, ballId);
        LaunchBallFadeMonTask(TRUE, gBattleAnimTarget, 14, ballId);
    }
    gSprites[gBattlerSpriteIds[gBattleAnimTarget]].set_invisible(FALSE as u16);
    StartSpriteAffineAnim(
        &raw mut gSprites[gBattlerSpriteIds[gBattleAnimTarget]],
        BATTLER_AFFINE_EMERGE,
    );
    AnimateSprite(&raw mut gSprites[gBattlerSpriteIds[gBattleAnimTarget]]);
    gSprites[gBattlerSpriteIds[gBattleAnimTarget]].data[1] = 4096;
}
pub(crate) unsafe fn SpriteCB_Ball_Release_Wait(sprite: *mut Sprite) {
    let mut released: u8 = FALSE;
    if (*sprite).animEnded() != 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if gSprites[gBattlerSpriteIds[gBattleAnimTarget]].affineAnimEnded() != 0 {
        StartSpriteAffineAnim(
            &raw mut gSprites[gBattlerSpriteIds[gBattleAnimTarget]],
            BATTLER_AFFINE_NORMAL,
        );
        released = TRUE;
    } else {
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]].data[1] -= 288;
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]].y2 =
            gSprites[gBattlerSpriteIds[gBattleAnimTarget]].data[1] >> 8;
    }
    if (*sprite).animEnded() != 0 && released != 0 {
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]].y2 = 0;
        gSprites[gBattlerSpriteIds[gBattleAnimTarget]]
            .set_invisible((*(*gBattleSpritesDataPtr).animationData).wildMonInvisible() as u16);
        (*sprite).data[sFrame] = 0;
        (*sprite).callback = Some(DestroySpriteAfterOneFrame);
        gDoingBattleAnim = 0;
        UpdateOamPriorityInAllHealthboxes(1);
    }
}
pub(crate) unsafe fn SpriteCB_Ball_Block(sprite: *mut Sprite) {
    (*sprite).x += (*sprite).x2;
    (*sprite).y += (*sprite).y2;
    (*sprite).y2 = 0;
    (*sprite).x2 = 0;
    for i in 0..6i32 {
        (*sprite).data[i] = 0;
    }
    (*sprite).callback = Some(SpriteCB_Ball_Block_Step);
}
pub(crate) unsafe fn SpriteCB_Ball_Block_Step(sprite: *mut Sprite) {
    let dy: i16 = (*sprite).data[0] + 0x800;
    let dx: i16 = (*sprite).data[sDx] + 0x680;
    (*sprite).x2 -= dx >> 8;
    (*sprite).y2 += dy >> 8;
    (*sprite).data[0] = ((*sprite).data[0] + 0x800) & 0xFF;
    (*sprite).data[sDx] = ((*sprite).data[sDx] + 0x680) & 0xFF;
    if (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
        || ((*sprite).x as i32 + (*sprite).x2 as i32) < -8
    {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(DestroySpriteAfterOneFrame);
        gDoingBattleAnim = 0;
        UpdateOamPriorityInAllHealthboxes(1);
    }
}
unsafe fn LoadBallParticleGfx(ballId: u8) {
    if GetSpriteTileStartByTag(sBallParticleSpriteSheets[ballId].tag) == 0xFFFF {
        LoadCompressedSpriteSheetUsingHeap(
            (&raw const sBallParticleSpriteSheets[ballId]).cast_mut(),
        );
        LoadCompressedSpritePaletteUsingHeap((&raw const sBallParticlePalettes[ballId]).cast_mut());
    }
}
pub unsafe fn AnimateBallOpenParticles(
    x: u8,
    y: u8,
    priority: u8,
    subpriority: u8,
    ballId: u8,
) -> u8 {
    LoadBallParticleGfx(ballId);
    let taskId: u8 = CreateTask(sBallParticleAnimationFuncs[ballId], 5);
    task_set(taskId, 1, x as i16);
    task_set(taskId, 2, y as i16);
    task_set(taskId, 3, priority as i16);
    task_set(taskId, 4, subpriority as i16);
    task_set(taskId, 15, ballId as i16);
    PlaySE(SE_BALL_OPEN);
    taskId
}
unsafe fn IncrBallParticleCount() {
    if gMain.inBattle() != 0 {
        (*(*gBattleSpritesDataPtr).animationData).numBallParticles += 1;
    }
}
pub(crate) unsafe fn PokeBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut priority: u8 = 0;
    let mut subpriority: u8 = 0;
    let mut var0: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    if task_get(taskId, 0) < 16 {
        x = task_get(taskId, 1) as u8;
        y = task_get(taskId, 2) as u8;
        priority = task_get(taskId, 3) as u8;
        subpriority = task_get(taskId, 4) as u8;
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(PokeBallOpenParticleAnimation_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            var0 = task_get(taskId, 0) as u8;
            if var0 >= 8 {
                var0 -= 8;
            }
            gSprites[spriteId].data[0] = var0 as i16 * 32;
        }
        if task_get(taskId, 0) == 15 {
            if gMain.inBattle() == 0 {
                gSprites[spriteId].data[7] = 1;
            }
            DestroyTask(taskId);
            return;
        }
    }
    task_set(taskId, 0, task_get(taskId, 0) + 1);
}
pub(crate) unsafe fn PokeBallOpenParticleAnimation_Step1(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0 {
        (*sprite).callback = Some(PokeBallOpenParticleAnimation_Step2);
    } else {
        (*sprite).data[1] -= 1;
    }
}
pub(crate) unsafe fn PokeBallOpenParticleAnimation_Step2(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[0], (*sprite).data[1]);
    (*sprite).y2 = Cos((*sprite).data[0], (*sprite).data[1]);
    (*sprite).data[1] += 2;
    if (*sprite).data[1] == 50 {
        DestroyBallOpenAnimationParticle(sprite);
    }
}
pub(crate) unsafe fn TimerBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for i in 0..8u8 {
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(FanOutBallOpenParticles_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            gSprites[spriteId].data[0] = i as i16 * 32;
            gSprites[spriteId].data[4] = 10;
            gSprites[spriteId].data[5] = 2;
            gSprites[spriteId].data[6] = 1;
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn DiveBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for i in 0..8u8 {
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(FanOutBallOpenParticles_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            gSprites[spriteId].data[0] = i as i16 * 32;
            gSprites[spriteId].data[4] = 10;
            gSprites[spriteId].data[5] = 1;
            gSprites[spriteId].data[6] = 2;
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn SafariBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for i in 0..8u8 {
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(FanOutBallOpenParticles_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            gSprites[spriteId].data[0] = i as i16 * 32;
            gSprites[spriteId].data[4] = 4;
            gSprites[spriteId].data[5] = 1;
            gSprites[spriteId].data[6] = 1;
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn UltraBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for i in 0..10u8 {
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(FanOutBallOpenParticles_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            gSprites[spriteId].data[0] = i as i16 * 25;
            gSprites[spriteId].data[4] = 5;
            gSprites[spriteId].data[5] = 1;
            gSprites[spriteId].data[6] = 1;
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn GreatBallOpenParticleAnimation(taskId: u8) {
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    let mut priority: u8 = 0;
    let mut subpriority: u8 = 0;
    let mut ballId: u8 = 0;
    let mut spriteId: u8 = 0;
    if task_get(taskId, 7) != 0 {
        task_set(taskId, 7, task_get(taskId, 7) - 1);
    } else {
        ballId = task_get(taskId, 15) as u8;
        x = task_get(taskId, 1) as u8;
        y = task_get(taskId, 2) as u8;
        priority = task_get(taskId, 3) as u8;
        subpriority = task_get(taskId, 4) as u8;
        for i in 0..8u8 {
            spriteId = CreateSprite(
                (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
                x as i16,
                y as i16,
                subpriority,
            );
            if spriteId != MAX_SPRITES {
                IncrBallParticleCount();
                StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
                gSprites[spriteId].callback = Some(FanOutBallOpenParticles_Step1);
                gSprites[spriteId].oam.set_priority(priority as u16);
                gSprites[spriteId].data[0] = i as i16 * 32;
                gSprites[spriteId].data[4] = 8;
                gSprites[spriteId].data[5] = 2;
                gSprites[spriteId].data[6] = 2;
            }
        }
        task_set(taskId, 7, 8);
        if ({
            task_set(taskId, 0, task_get(taskId, 0) + 1);
            task_get(taskId, 0)
        }) == 2
        {
            if gMain.inBattle() == 0 {
                gSprites[spriteId].data[7] = 1;
            }
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe fn FanOutBallOpenParticles_Step1(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[0], (*sprite).data[1]);
    (*sprite).y2 = Cos((*sprite).data[0], (*sprite).data[2]);
    (*sprite).data[0] = ((*sprite).data[0] + (*sprite).data[4]) & 0xFF;
    (*sprite).data[1] += (*sprite).data[5];
    (*sprite).data[2] += (*sprite).data[6];
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == 51
    {
        DestroyBallOpenAnimationParticle(sprite);
    }
}
pub(crate) unsafe fn RepeatBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for i in 0..12u8 {
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(RepeatBallOpenParticleAnimation_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            gSprites[spriteId].data[0] = i as i16 * 21;
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn RepeatBallOpenParticleAnimation_Step1(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[0], (*sprite).data[1]);
    (*sprite).y2 = Cos((*sprite).data[0], Sin((*sprite).data[0], (*sprite).data[2]));
    (*sprite).data[0] = ((*sprite).data[0] + 6) & 0xFF;
    (*sprite).data[1] += 1;
    (*sprite).data[2] += 1;
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == 51
    {
        DestroyBallOpenAnimationParticle(sprite);
    }
}
pub(crate) unsafe fn MasterBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for j in 0..2u8 {
        for i in 0..8u8 {
            spriteId = CreateSprite(
                (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
                x as i16,
                y as i16,
                subpriority,
            );
            if spriteId != MAX_SPRITES {
                IncrBallParticleCount();
                StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
                gSprites[spriteId].callback = Some(FanOutBallOpenParticles_Step1);
                gSprites[spriteId].oam.set_priority(priority as u16);
                gSprites[spriteId].data[0] = i as i16 * 32;
                gSprites[spriteId].data[4] = 8;
                if j == 0 {
                    gSprites[spriteId].data[5] = 2;
                    gSprites[spriteId].data[6] = 1;
                } else {
                    gSprites[spriteId].data[5] = 1;
                    gSprites[spriteId].data[6] = 2;
                }
            }
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn PremierBallOpenParticleAnimation(taskId: u8) {
    let mut spriteId: u8 = 0;
    let ballId: u8 = task_get(taskId, 15) as u8;
    let x: u8 = task_get(taskId, 1) as u8;
    let y: u8 = task_get(taskId, 2) as u8;
    let priority: u8 = task_get(taskId, 3) as u8;
    let subpriority: u8 = task_get(taskId, 4) as u8;
    for i in 0..8u8 {
        spriteId = CreateSprite(
            (&raw const sBallParticleSpriteTemplates[ballId]).cast_mut(),
            x as i16,
            y as i16,
            subpriority,
        );
        if spriteId != MAX_SPRITES {
            IncrBallParticleCount();
            StartSpriteAnim(&raw mut gSprites[spriteId], sBallParticleAnimNums[ballId]);
            gSprites[spriteId].callback = Some(PremierBallOpenParticleAnimation_Step1);
            gSprites[spriteId].oam.set_priority(priority as u16);
            gSprites[spriteId].data[0] = i as i16 * 32;
        }
    }
    if gMain.inBattle() == 0 {
        gSprites[spriteId].data[7] = 1;
    }
    DestroyTask(taskId);
}
pub(crate) unsafe fn PremierBallOpenParticleAnimation_Step1(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[0], (*sprite).data[1]);
    (*sprite).y2 = Cos(
        (*sprite).data[0],
        Sin((*sprite).data[0] & 0x3F, (*sprite).data[2]),
    );
    (*sprite).data[0] = ((*sprite).data[0] + 10) & 0xFF;
    (*sprite).data[1] += 1;
    (*sprite).data[2] += 1;
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == 51
    {
        DestroyBallOpenAnimationParticle(sprite);
    }
}
unsafe fn DestroyBallOpenAnimationParticle(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    if gMain.inBattle() == 0 {
        if (*sprite).data[7] == 1 {
            DestroySpriteAndFreeResources(sprite);
        } else {
            DestroySprite(sprite);
        }
    } else {
        (*(*gBattleSpritesDataPtr).animationData).numBallParticles -= 1;
        if (*(*gBattleSpritesDataPtr).animationData).numBallParticles == 0 {
            i = 0;
            while i < POKEBALL_COUNT {
                if FuncIsActiveTask(sBallParticleAnimationFuncs[i]) == TRUE {
                    break;
                }
                i += 1;
            }
            if i == POKEBALL_COUNT {
                for j in 0..POKEBALL_COUNT {
                    FreeSpriteTilesByTag(sBallParticleSpriteSheets[j].tag);
                    FreeSpritePaletteByTag(sBallParticlePalettes[j].tag);
                }
            }
            DestroySprite(sprite);
        } else {
            DestroySprite(sprite);
        }
    }
}
pub unsafe fn LaunchBallFadeMonTask(
    unfadeLater: u8,
    spritePalNum: u8,
    selectedPalettes: u32,
    ballId: u8,
) -> u8 {
    let taskId: u8 = CreateTask(Some(Task_FadeMon_ToBallColor), 5);
    task_set(taskId, tBallId, ballId as i16);
    task_set(taskId, tPalOffset, spritePalNum as i16);
    task_set(taskId, tPaletteLo, selectedPalettes as i16);
    task_set(taskId, tPaletteHi, (selectedPalettes >> 16) as i16);
    if unfadeLater == 0 {
        BlendPalette(
            0x100 + spritePalNum as u16 * 16,
            16,
            0,
            gBallOpenFadeColors[ballId],
        );
        task_set(taskId, tdCoeff, 1);
    } else {
        BlendPalette(
            0x100 + spritePalNum as u16 * 16,
            16,
            16,
            gBallOpenFadeColors[ballId],
        );
        task_set(taskId, tCoeff, 16);
        task_set(taskId, tdCoeff, -1);
        task_set_func(taskId, Some(Task_FadeMon_ToNormal));
    }
    BeginNormalPaletteFade(selectedPalettes, 0, 0, 16, 32767);
    taskId
}
pub(crate) unsafe fn Task_FadeMon_ToBallColor(taskId: u8) {
    let ballId: u8 = task_get(taskId, tBallId) as u8;
    if task_get(taskId, 2) <= 16 {
        BlendPalette(
            0x100 + task_get(taskId, tPalOffset) as u16 * 16,
            16,
            task_get(taskId, tCoeff) as u8,
            gBallOpenFadeColors[ballId],
        );
        task_set(
            taskId,
            tCoeff,
            task_get(taskId, tCoeff) + (task_get(taskId, tdCoeff)),
        );
        task_set(taskId, 2, task_get(taskId, 2) + 1);
    } else if gPaletteFade.active() == 0 {
        let selectedPalettes: u32 = task_get(taskId, tPaletteLo) as u16 as u32
            | (task_get(taskId, tPaletteHi) as u16 as u32) << 16;
        BeginNormalPaletteFade(selectedPalettes, 0, 16, 0, 32767);
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn Task_FadeMon_ToNormal(taskId: u8) {
    if gPaletteFade.active() == 0 {
        let selectedPalettes: u32 = task_get(taskId, tPaletteLo) as u16 as u32
            | (task_get(taskId, tPaletteHi) as u16 as u32) << 16;
        BeginNormalPaletteFade(selectedPalettes, 0, 16, 0, 32767);
        task_set_func(taskId, Some(Task_FadeMon_ToNormal_Step));
    }
}
pub(crate) unsafe fn Task_FadeMon_ToNormal_Step(taskId: u8) {
    let ballId: u8 = task_get(taskId, tBallId) as u8;
    if task_get(taskId, 2) <= 16 {
        BlendPalette(
            0x100 + task_get(taskId, tPalOffset) as u16 * 16,
            16,
            task_get(taskId, tCoeff) as u8,
            gBallOpenFadeColors[ballId],
        );
        task_set(
            taskId,
            tCoeff,
            task_get(taskId, tCoeff) + (task_get(taskId, tdCoeff)),
        );
        task_set(taskId, 2, task_get(taskId, 2) + 1);
    } else {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SwapMonSpriteToFromSubstitute(taskId: u8) {
    let mut x: u32 = 0;
    let mut done: u32 = FALSE as u32;
    let spriteId: u8 = gBattlerSpriteIds[gBattleAnimAttacker];
    match task_get(taskId, 10) {
        0 => {
            task_set(taskId, 11, gBattleAnimArgs[0]);
            task_set(taskId, 0, task_get(taskId, 0) + 0x500);
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                gSprites[spriteId].x2 += task_get(taskId, 0) >> 8;
            } else {
                gSprites[spriteId].x2 -= task_get(taskId, 0) >> 8;
            }
            task_set(taskId, 0, task_get(taskId, 0) & 0xFF);
            x = gSprites[spriteId].x as u32 + gSprites[spriteId].x2 as u32 + 32;
            if x > 304 {
                task_set(taskId, 10, task_get(taskId, 10) + 1);
            }
        }
        1 => {
            LoadBattleMonGfxAndAnimate(gBattleAnimAttacker, task_get(taskId, 11) as u8, spriteId);
            task_set(taskId, 10, task_get(taskId, 10) + 1);
        }
        2 => {
            task_set(taskId, 0, task_get(taskId, 0) + 0x500);
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                gSprites[spriteId].x2 -= task_get(taskId, 0) >> 8;
            } else {
                gSprites[spriteId].x2 += task_get(taskId, 0) >> 8;
            }
            task_set(taskId, 0, task_get(taskId, 0) & 0xFF);
            if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                if gSprites[spriteId].x2 <= 0 {
                    gSprites[spriteId].x2 = 0;
                    done = TRUE as u32;
                }
            } else {
                if gSprites[spriteId].x2 >= 0 {
                    gSprites[spriteId].x2 = 0;
                    done = TRUE as u32;
                }
            }
            if done != 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SubstituteFadeToInvisible(taskId: u8) {
    let mut spriteId: u8 = 0;
    match task_get(taskId, 15) {
        0 => {
            if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == B_POSITION_OPPONENT_LEFT {
                SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            } else {
                SetGpuReg(REG_OFFSET_BLDCNT, 16196);
            }
            SetGpuReg(REG_OFFSET_BLDALPHA, 16);
            task_set(taskId, 15, task_get(taskId, 15) + 1);
        }
        1 => {
            if ({
                let t1 = task_get(taskId, 1);
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                t1
            }) > 1
            {
                task_set(taskId, 1, 0);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((task_get(taskId, 0) as u16) << 8) | (16 - task_get(taskId, 0) as u16),
                );
                if task_get(taskId, 0) == 16 {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            }
        }
        2 => {
            spriteId = gBattlerSpriteIds[gBattleAnimAttacker];
            RequestDma3Fill(
                0,
                (OBJ_VRAM0 as usize as *mut c_void as *mut u8)
                    .at(gSprites[spriteId].oam.tileNum() as i32 * 32)
                    as *mut c_void,
                MON_PIC_SIZE,
                1,
            );
            ClearBehindSubstituteBit(gBattleAnimAttacker);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsAttackerBehindSubstitute(taskId: u8) {
    gBattleAnimArgs[7] =
        (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker)).behindSubstitute() as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetTargetToEffectBattler(taskId: u8) {
    gBattleAnimTarget = gEffectBattler;
    DestroyAnimVisualTask(taskId);
}
pub unsafe fn TryShinyAnimation(battler: u8, mon: *mut Pokemon) {
    let mut shinyValue: u32 = 0;
    let mut taskCirc: u8 = 0;
    let mut taskDgnl: u8 = 0;
    let mut isShiny: u8 = FALSE;
    (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_triedShinyMonAnim(TRUE);
    let otId: u32 = GetMonData2(mon, MON_DATA_OT_ID);
    let personality: u32 = GetMonData2(mon, MON_DATA_PERSONALITY);
    if IsBattlerSpriteVisible(battler) != 0 {
        shinyValue = (otId & 0xFFFF0000) >> 16
            ^ otId & 0xFFFF
            ^ (personality & 0xFFFF0000) >> 16
            ^ personality & 0xFFFF;
        if shinyValue < SHINY_ODDS {
            isShiny = TRUE;
        }
        if isShiny != 0 {
            if GetSpriteTileStartByTag(ANIM_TAG_GOLD_STARS) == 0xFFFF {
                LoadCompressedSpriteSheetUsingHeap(
                    (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPicTable)
                        .cast::<CArray<CompressedSpriteSheet, 0>>())[233])
                        .cast_mut(),
                );
                LoadCompressedSpritePaletteUsingHeap(
                    (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPaletteTable)
                        .cast::<CArray<CompressedSpritePalette, 0>>())[233])
                        .cast_mut(),
                );
            }
            taskCirc = CreateTask(Some(Task_ShinyStars), 10);
            taskDgnl = CreateTask(Some(Task_ShinyStars), 10);
            task_set(taskCirc, tBattler, battler as i16);
            task_set(taskDgnl, tBattler, battler as i16);
            task_set(taskCirc, tStarMove, SHINY_STAR_ENCIRCLE);
            task_set(taskDgnl, tStarMove, SHINY_STAR_DIAGONAL);
            return;
        }
    }
    (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_finishedShinyMonAnim(TRUE);
}
pub(crate) unsafe fn Task_ShinyStars(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut pan: u8 = 0;
    if task_get(taskId, 13) < 60 {
        task_set(taskId, 13, task_get(taskId, 13) + 1);
        return;
    }
    if (*(*gBattleSpritesDataPtr).animationData).numBallParticles != 0 {
        return;
    }
    let timer: u16 = ({
        let t1 = task_get(taskId, tStarTimer);
        task_set(taskId, tStarTimer, task_get(taskId, tStarTimer) + 1);
        t1
    }) as u16;
    if timer as i32 % 4 != 0 {
        return;
    }
    let battler: u8 = task_get(taskId, 0) as u8;
    let x: u8 = GetBattlerSpriteCoord(battler, BATTLER_COORD_X);
    let y: u8 = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y);
    let starIdx: i16 = task_get(taskId, tStarIdx);
    if starIdx == 0 {
        spriteId = CreateSprite(
            (&raw const (*(&raw const crate::data::battle_anim_effects_3::gWishStarSpriteTemplate).cast::<SpriteTemplate>())).cast_mut(),
            x as i16,
            y as i16,
            5,
        );
    } else if starIdx >= 0 && task_get(taskId, tStarIdx) < 4 {
        spriteId = CreateSprite(
            (&raw const (*(&raw const crate::data::battle_anim_effects_3::gMiniTwinklingStarSpriteTemplate).cast::<SpriteTemplate>())).cast_mut(),
            x as i16,
            y as i16,
            5,
        );
        gSprites[spriteId]
            .oam
            .set_tileNum(gSprites[spriteId].oam.tileNum() + 4);
    } else {
        spriteId = CreateSprite(
            (&raw const (*(&raw const crate::data::battle_anim_effects_3::gMiniTwinklingStarSpriteTemplate).cast::<SpriteTemplate>())).cast_mut(),
            x as i16,
            y as i16,
            5,
        );
        gSprites[spriteId]
            .oam
            .set_tileNum(gSprites[spriteId].oam.tileNum() + 5);
    }
    if task_get(taskId, tStarMove) == SHINY_STAR_ENCIRCLE {
        gSprites[spriteId].callback = Some(SpriteCB_ShinyStars_Encircle);
    } else {
        gSprites[spriteId].callback = Some(SpriteCB_ShinyStars_Diagonal);
        gSprites[spriteId].x2 = -32;
        gSprites[spriteId].y2 = 32;
        gSprites[spriteId].set_invisible(TRUE as u16);
        if task_get(taskId, tStarIdx) == 0 {
            if GetBattlerSide(battler) == B_SIDE_PLAYER {
                pan = 192;
            } else {
                pan = 63;
            }
            PlaySE12WithPanning(SE_SHINY, pan as i8);
        }
    }
    gSprites[spriteId].data[0] = taskId as i16;
    task_set(taskId, tStarIdx, task_get(taskId, tStarIdx) + 1);
    if spriteId != MAX_SPRITES {
        task_set(taskId, tNumStars, task_get(taskId, tNumStars) + 1);
    }
    if task_get(taskId, tStarIdx) == 5 {
        task_set_func(taskId, Some(Task_ShinyStars_Wait));
    }
}
pub(crate) unsafe fn Task_ShinyStars_Wait(taskId: u8) {
    let mut battler: u8 = 0;
    if task_get(taskId, tNumStars) == 0 {
        if task_get(taskId, tStarMove) == SHINY_STAR_DIAGONAL {
            battler = task_get(taskId, tBattler) as u8;
            (*(*gBattleSpritesDataPtr).healthBoxesData.at(battler)).set_finishedShinyMonAnim(TRUE);
        }
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn SpriteCB_ShinyStars_Encircle(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[1], 24);
    (*sprite).y2 = Cos((*sprite).data[1], 24);
    (*sprite).data[1] += 12;
    if (*sprite).data[1] > 255 {
        task_set(
            (*sprite).data[sTaskId],
            tNumStars,
            task_get((*sprite).data[sTaskId], tNumStars) - 1,
        );
        FreeSpriteOamMatrix(sprite);
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn SpriteCB_ShinyStars_Diagonal(sprite: *mut Sprite) {
    if (*sprite).data[1] < 4 {
        (*sprite).data[1] += 1;
    } else {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).x2 += 5;
        (*sprite).y2 -= 5;
        if (*sprite).x2 > 32 {
            task_set(
                (*sprite).data[sTaskId],
                tNumStars,
                task_get((*sprite).data[sTaskId], tNumStars) - 1,
            );
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_LoadPokeblockGfx(taskId: u8) {
    LoadCompressedSpriteSheetUsingHeap(
        (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPicTable)
            .cast::<CArray<CompressedSpriteSheet, 0>>())[269])
            .cast_mut(),
    );
    LoadCompressedSpritePaletteUsingHeap(
        (&raw const (*(&raw const crate::data::battle_anim::gBattleAnimPaletteTable)
            .cast::<CArray<CompressedSpritePalette, 0>>())[269])
            .cast_mut(),
    );
    let paletteIndex: u8 = IndexOfSpritePaletteTag(ANIM_TAG_POKEBLOCK);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FreePokeblockGfx(taskId: u8) {
    FreeSpriteTilesByTag(ANIM_TAG_POKEBLOCK);
    FreeSpritePaletteByTag(ANIM_TAG_POKEBLOCK);
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn SpriteCB_PokeBlock_Throw(sprite: *mut Sprite) {
    InitSpritePosToAnimAttacker(sprite, FALSE);
    (*sprite).data[sDuration] = 30;
    (*sprite).data[2] = GetBattlerSpriteCoord(
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        BATTLER_COORD_X,
    ) as i16
        + gBattleAnimArgs[2];
    (*sprite).data[4] = GetBattlerSpriteCoord(
        GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT),
        BATTLER_COORD_Y,
    ) as i16
        + gBattleAnimArgs[3];
    (*sprite).data[5] = -32;
    InitAnimArcTranslation(sprite);
    gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].callback = Some(SpriteCB_TrainerThrowObject);
    (*sprite).callback = Some(SpriteCB_PokeBlock_LiftArm);
}
pub(crate) unsafe fn SpriteCB_PokeBlock_LiftArm(sprite: *mut Sprite) {
    if gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].animCmdIndex == 1 {
        (*sprite).callback = Some(SpriteCB_PokeBlock_Arc);
    }
}
pub(crate) unsafe fn SpriteCB_PokeBlock_Arc(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(SpriteCB_ThrowPokeBlock_Free);
    }
}
pub(crate) unsafe fn SpriteCB_ThrowPokeBlock_Free(sprite: *mut Sprite) {
    if gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].animEnded() != 0
        && ({
            (*sprite).data[0] += 1;
            (*sprite).data[0]
        }) > 0
    {
        StartSpriteAnim(&raw mut gSprites[gBattlerSpriteIds[gBattleAnimAttacker]], 0);
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetAttackerTargetLeftPos(taskId: u8) {
    match gBattleAnimArgs[0] {
        0 => {
            gBattleAnimAttacker = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
            gBattleAnimTarget = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
        }
        1 => {
            gBattleAnimAttacker = GetBattlerAtPosition(B_POSITION_OPPONENT_LEFT);
            gBattleAnimTarget = GetBattlerAtPosition(B_POSITION_PLAYER_LEFT);
        }
        _ => {}
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetTrappedMoveAnimId(taskId: u8) {
    if (*(*gBattleSpritesDataPtr).animationData).animArg == MOVE_FIRE_SPIN {
        gBattleAnimArgs[0] = TRAP_ANIM_FIRE_SPIN;
    } else if (*(*gBattleSpritesDataPtr).animationData).animArg == MOVE_WHIRLPOOL {
        gBattleAnimArgs[0] = TRAP_ANIM_WHIRLPOOL;
    } else if (*(*gBattleSpritesDataPtr).animationData).animArg == MOVE_CLAMP {
        gBattleAnimArgs[0] = TRAP_ANIM_CLAMP;
    } else if (*(*gBattleSpritesDataPtr).animationData).animArg == MOVE_SAND_TOMB {
        gBattleAnimArgs[0] = TRAP_ANIM_SAND_TOMB;
    } else {
        gBattleAnimArgs[0] = 0;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetBattlersFromArg(taskId: u8) {
    gBattleAnimAttacker = (*(*gBattleSpritesDataPtr).animationData).animArg as u8;
    gBattleAnimTarget = ((*(*gBattleSpritesDataPtr).animationData).animArg >> 8) as u8;
    DestroyAnimVisualTask(taskId);
}
