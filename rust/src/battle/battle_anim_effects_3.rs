//! Translated from `src/battle_anim_effects_3.c` by tools/rustport/c2rs.py.
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
    clippy::if_same_then_else,
    clippy::manual_clamp,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    unused_assignments,
    unused_variables
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    BattleAnimAdjustPanning, DestroyAnimSprite, DestroyAnimVisualTask, IsContest, gAnimFriendship,
    gAnimMoveDmg, gAnimVisualTaskCount, gBattleAnimAttacker, gBattleAnimTarget, gWeatherMoveAnim,
};
use crate::battle_anim_flying::DestroyAnimSpriteAfterTimer;
use crate::battle_anim_mons::{
    AnimLoadCompressedBgGfx, AnimLoadCompressedBgTilemapHandleContest, AnimTranslateLinear,
    ArcTan2Neg, ClearBattleAnimBg, CloneBattlerSpriteWithBlend,
    CreateAdditionalMonSpriteForMoveAnim, DestroyAnimVisualTaskAndDisableBlend,
    DestroySpriteAndFreeResources_, DestroySpriteAndMatrix, DestroySpriteWithActiveSheet,
    GetAnimBattlerSpriteId, GetBattleAnimBg1Data, GetBattleBgPaletteNum, GetBattlerPosition,
    GetBattlerSide, GetBattlerSpriteBGPriority, GetBattlerSpriteBGPriorityRank,
    GetBattlerSpriteCoord, GetBattlerSpriteCoordAttr, GetBattlerSpriteSubpriority,
    GetBattlerYCoordWithElevation, GetBgDataForTransform, InitAndRunAnimFastLinearTranslation,
    InitAnimArcTranslation, InitAnimLinearTranslation, InitSpritePosToAnimAttacker,
    InitSpritePosToAnimTarget, IsDoubleBattle, PrepareAffineAnimInTaskData,
    PrepareBattlerSpriteForRotScale, ResetSpriteRotScale, ResetSpriteRotScale_PreserveAffine,
    RunAffineAnimFromTaskData, RunStoredCallbackWhenAnimEnds, SetAnimSpriteInitialXOffset,
    SetAverageBattlerPositions, SetBattlerSpriteYOffsetFromRotation,
    SetBattlerSpriteYOffsetFromYScale, SetSpriteCoordsToAnimAttackerCoords, SetSpriteRotScale,
    StartAnimLinearTranslation, StoreSpriteCallbackInData6, TranslateAnimHorizontalArc,
    TrySetSpriteRotScale, WaitAnimForDuration,
};
use crate::battle_anim_smokescreen::SmokescreenImpact;
use crate::battle_anim_utility_funcs::{SetAnimBgAttribute, StartMonScrollingBgMask};
use crate::battle_gfx_sfx_util::{
    HandleSpeciesGfxDataChange, LoadBattleMonGfxAndAnimate, SetBattlerShadowSpriteCallback,
};
use crate::battle_main::{
    gBattle_BG1_X, gBattle_BG1_Y, gBattle_BG2_X, gBattle_BG2_Y, gBattle_WIN0H, gBattle_WIN0V,
    gBattle_WIN1H, gBattle_WIN1V, gBattleSpritesDataPtr, gMonSpritesGfxPtr,
};
use crate::battle_main::{gBattleMonForms, gBattlerPartyIndexes, gBattlerSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::contest::{IsSpeciesNotUnown, gContestResources};
use crate::gpu_regs::{ClearGpuRegBits, GetGpuReg, SetGpuReg, SetGpuRegBits};
use crate::palette::{FillPalette, LoadCompressedPalette};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::pokemon::{GetMonData2, gEnemyParty, gPlayerParty};
use crate::random::Random2;
use crate::sound::{PlaySE1WithPanning, PlaySE12WithPanning};
use crate::sprite::FreeOamMatrix;
use crate::sprite::gSprites;
use crate::task::DestroyTask;
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
/// `ChangeSpriteAffineAnim` with this module's view of its types.
#[inline]
unsafe fn ChangeSpriteAffineAnim(a0: *mut Sprite, a1: u8) {
    unsafe {
        crate::sprite::ChangeSpriteAffineAnim(a0 as _, a1);
    }
}
/// `CreateSprite` with this module's view of its types.
#[inline]
unsafe fn CreateSprite(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSprite(a0 as _, a1, a2, a3) }
}
/// `CreateSpriteAndAnimate` with this module's view of its types.
#[inline]
unsafe fn CreateSpriteAndAnimate(a0: *mut SpriteTemplate, a1: i16, a2: i16, a3: u8) -> u8 {
    unsafe { crate::sprite::CreateSpriteAndAnimate(a0 as _, a1, a2, a3) }
}
/// `DestroySprite` with this module's view of its types.
#[inline]
unsafe fn DestroySprite(a0: *mut Sprite) {
    unsafe {
        crate::sprite::DestroySprite(a0 as _);
    }
}
/// `LoadBgTiles` with this module's view of its types.
#[inline]
unsafe fn LoadBgTiles(a0: u8, a1: *mut c_void, a2: u16, a3: u16) -> u16 {
    unsafe { crate::bg::LoadBgTiles(a0, a1 as _, a2, a3) }
}
/// `ScanlineEffect_SetParams` with this module's view of its types.
#[inline]
unsafe fn ScanlineEffect_SetParams(a0: ScanlineEffectParams) {
    unsafe {
        crate::scanline_effect::ScanlineEffect_SetParams(core::mem::transmute(a0));
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
// The C's names for task and sprite data slots.
const sTimer: usize = 0;
const tState: usize = 0;
const sVelocX: usize = 1;
const tTimer: usize = 1;
const sVelocY: usize = 2;
const tNumSquishes: usize = 3;
const tBaseX: usize = 4;
const tBaseY: usize = 5;
const tPairMax: usize = 5;
const tDotOffset: usize = 6;
const tSubpriority: usize = 6;
const tIsContest: usize = 7;
const tStartX: usize = 11;
const tStartY: usize = 12;
const tEndX: usize = 13;
const tEndY: usize = 14;
const tBattlerSpriteId: usize = 15;
// Data tables (translate with cdata.py): gScratchAnimCmds gScratchAnimTable gScratchSpriteTemplate gBlackSmokeSpriteTemplate gBlackBallSpriteTemplate gOpeningEyeAnimCmds gOpeningEyeAnimTable gOpeningEyeSpriteTemplate gWhiteHaloSpriteTemplate gTealAlertSpriteTemplate gMeanLookEyeAffineAnimCmds1 gMeanLookEyeAffineAnimCmds2 gMeanLookEyeAffineAnimTable gMeanLookEyeSpriteTemplate gSpikesSpriteTemplate gLeerAnimCmds gLeerAnimTable gLeerSpriteTemplate gLetterZAnimCmds gLetterZAnimTable gLetterZAffineAnimCmds gLetterZAffineAnimTable gLetterZSpriteTemplate gFangAnimCmds gFangAnimTable gFangAffineAnimCmds gFangAffineAnimTable gFangSpriteTemplate gSpotlightAffineAnimCmds1 gSpotlightAffineAnimCmds2 gSpotlightAffineAnimTable gSpotlightSpriteTemplate gClappingHandSpriteTemplate gClappingHand2SpriteTemplate gRapidSpinAnimCmds gRapidSpinAnimTable gRapidSpinSpriteTemplate sAffineAnims_Torment gTriAttackTriangleAnimCmds gTriAttackTriangleAnimTable gTriAttackTriangleAffineAnimCmds gTriAttackTriangleAffineAnimTable gTriAttackTriangleSpriteTemplate gEclipsingOrbAnimCmds gEclipsingOrbAnimTable gEclipsingOrbSpriteTemplate DefenseCurlDeformMonAffineAnimCmds gBatonPassPokeballSpriteTemplate gWishStarSpriteTemplate gMiniTwinklingStarSpriteTemplate gStockpileDeformMonAffineAnimCmds gSpitUpDeformMonAffineAnimCmds gSwallowBlueOrbSpriteTemplate gSwallowDeformMonAffineAnimCmds gMorningSunLightBeamCoordsTable gGreenStarAnimCmds1 gGreenStarAnimCmds2 gGreenStarAnimCmds3 gGreenStarAnimTable gGreenStarSpriteTemplate gDoomDesireLightBeamCoordTable gDoomDesireLightBeamDelayTable gStrongFrustrationAffineAnimCmds gWeakFrustrationAngerMarkSpriteTemplate gSweetScentPetalAnimCmds1 gSweetScentPetalAnimCmds2 gSweetScentPetalAnimCmds3 gSweetScentPetalAnimCmdTable gSweetScentPetalSpriteTemplate sUnusedPalette gPainSplitAnimCmds gPainSplitAnimCmdTable gPainSplitProjectileSpriteTemplate gFlatterConfettiSpriteTemplate gFlatterSpotlightSpriteTemplate gReversalOrbSpriteTemplate gDeepInhaleAffineAnimCmds gYawnCloudAffineAnimCmds1 gYawnCloudAffineAnimCmds2 gYawnCloudAffineAnimCmds3 gYawnCloudAffineAnimTable gYawnCloudSpriteTemplate gSmokeBallEscapeCloudAffineAnimCmds1 gSmokeBallEscapeCloudAffineAnimCmds2 gSmokeBallEscapeCloudAffineAnimCmds3 gSmokeBallEscapeCloudAffineAnimCmds4 gSmokeBallEscapeCloudAffineAnimTable gSmokeBallEscapeCloudSpriteTemplate gFacadeSquishAffineAnimCmds gFacadeSweatDropSpriteTemplate gFacadeBlendColors gRoarNoiseLineAnimCmds1 gRoarNoiseLineAnimCmds2 gRoarNoiseLineAnimTable gRoarNoiseLineSpriteTemplate gGlareEyeDotSpriteTemplate gAssistPawprintSpriteTemplate gBarrageBallAffineAnimCmds1 gBarrageBallAffineAnimCmds2 gBarrageBallAffineAnimTable gBarrageBallSpriteTemplate gSmellingSaltsHandSpriteTemplate gSmellingSaltsSquishAffineAnimCmds gSmellingSaltExclamationSpriteTemplate gHelpingHandClapSpriteTemplate gForesightMagnifyingGlassSpriteTemplate gMeteorMashStarSpriteTemplate sUnusedStarBurstSpriteTemplate gBlockXSpriteTemplate sUnusedItemBagStealSpriteTemplate gKnockOffStrikeAnimCmds gKnockOffStrikeAnimTable gKnockOffStrikeAffineanimCmds1 gKnockOffStrikeAffineanimCmds2 gKnockOffStrikeAffineAnimTable gKnockOffStrikeSpriteTemplate gRecycleSpriteAffineAnimCmds gRecycleSpriteAffineAnimTable gRecycleSpriteTemplate gSlackOffSquishAffineAnimCmds

const IDX_ACTIVE_SPRITES: i16 = 10;

static DefenseCurlDeformMonAffineAnimCmds: Table<CArray<AffineAnimCmd, 4>> = Table(
    (&raw const crate::data::battle_anim_effects_3::DefenseCurlDeformMonAffineAnimCmds).cast(),
);
static gBarrageBallSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_3::gBarrageBallSpriteTemplate).cast());
static gDeepInhaleAffineAnimCmds: Table<CArray<AffineAnimCmd, 6>> =
    Table((&raw const crate::data::battle_anim_effects_3::gDeepInhaleAffineAnimCmds).cast());
static gDoomDesireLightBeamCoordTable: Table<CArray<i8, 4>> =
    Table((&raw const crate::data::battle_anim_effects_3::gDoomDesireLightBeamCoordTable).cast());
static gDoomDesireLightBeamDelayTable: Table<CArray<u8, 5>> =
    Table((&raw const crate::data::battle_anim_effects_3::gDoomDesireLightBeamDelayTable).cast());
static gFacadeBlendColors: Table<CArray<u16, 24>> =
    Table((&raw const crate::data::battle_anim_effects_3::gFacadeBlendColors).cast());
static gFacadeSquishAffineAnimCmds: Table<CArray<AffineAnimCmd, 4>> =
    Table((&raw const crate::data::battle_anim_effects_3::gFacadeSquishAffineAnimCmds).cast());
static gFacadeSweatDropSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_3::gFacadeSweatDropSpriteTemplate).cast());
static gGlareEyeDotSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_3::gGlareEyeDotSpriteTemplate).cast());
static gGreenStarSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_3::gGreenStarSpriteTemplate).cast());
static gMiniTwinklingStarSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_3::gMiniTwinklingStarSpriteTemplate).cast());
static gMorningSunLightBeamCoordsTable: Table<CArray<i8, 4>> =
    Table((&raw const crate::data::battle_anim_effects_3::gMorningSunLightBeamCoordsTable).cast());
static gSlackOffSquishAffineAnimCmds: Table<CArray<AffineAnimCmd, 6>> =
    Table((&raw const crate::data::battle_anim_effects_3::gSlackOffSquishAffineAnimCmds).cast());
static gSmellingSaltsSquishAffineAnimCmds: Table<CArray<AffineAnimCmd, 3>> = Table(
    (&raw const crate::data::battle_anim_effects_3::gSmellingSaltsSquishAffineAnimCmds).cast(),
);
static gSpitUpDeformMonAffineAnimCmds: Table<CArray<AffineAnimCmd, 7>> =
    Table((&raw const crate::data::battle_anim_effects_3::gSpitUpDeformMonAffineAnimCmds).cast());
static gStockpileDeformMonAffineAnimCmds: Table<CArray<AffineAnimCmd, 5>> = Table(
    (&raw const crate::data::battle_anim_effects_3::gStockpileDeformMonAffineAnimCmds).cast(),
);
static gStrongFrustrationAffineAnimCmds: Table<CArray<AffineAnimCmd, 4>> =
    Table((&raw const crate::data::battle_anim_effects_3::gStrongFrustrationAffineAnimCmds).cast());
static gSwallowDeformMonAffineAnimCmds: Table<CArray<AffineAnimCmd, 6>> =
    Table((&raw const crate::data::battle_anim_effects_3::gSwallowDeformMonAffineAnimCmds).cast());
static sAffineAnims_Torment: Table<CArray<AffineAnimCmd, 4>> =
    Table((&raw const crate::data::battle_anim_effects_3::sAffineAnims_Torment).cast());

/// `CpuSet` with this module's view of its types.
#[inline]
unsafe fn CpuSet(a0: *mut c_void, a1: *mut c_void, a2: u32) {
    unsafe {
        crate::syscall::CpuSet(a0 as _, a1 as _, a2);
    }
}

pub(crate) unsafe fn AnimBlackSmoke(sprite: *mut Sprite) {
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    if gBattleAnimArgs[3] == 0 {
        (*sprite).data[0] = gBattleAnimArgs[2];
    } else {
        (*sprite).data[0] = -gBattleAnimArgs[2];
    }
    (*sprite).data[1] = gBattleAnimArgs[4];
    (*sprite).callback = Some(AnimBlackSmoke_Step);
}
pub(crate) unsafe fn AnimBlackSmoke_Step(sprite: *mut Sprite) {
    if (*sprite).data[1] > 0 {
        (*sprite).x2 = (*sprite).data[2] >> 8;
        (*sprite).data[2] += (*sprite).data[0];
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        (*sprite).data[1] -= 1;
    } else {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SmokescreenImpact(taskId: u8) {
    SmokescreenImpact(
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + 8,
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + 8,
        FALSE,
    );
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimWhiteHalo(sprite: *mut Sprite) {
    (*sprite).data[0] = 90;
    (*sprite).callback = Some(WaitAnimForDuration);
    (*sprite).data[1] = 7;
    StoreSpriteCallbackInData6(sprite, Some(AnimWhiteHalo_Step1));
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - (*sprite).data[1] as u16) << 8 | (*sprite).data[1] as u16,
    );
}
pub(crate) unsafe fn AnimWhiteHalo_Step1(sprite: *mut Sprite) {
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - (*sprite).data[1] as u16) << 8 | (*sprite).data[1] as u16,
    );
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) < 0
    {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(AnimWhiteHalo_Step2);
    }
}
pub(crate) unsafe fn AnimWhiteHalo_Step2(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 0);
    SetGpuReg(REG_OFFSET_BLDALPHA, 0);
    DestroyAnimSprite(sprite);
}
pub(crate) unsafe fn AnimTealAlert(sprite: *mut Sprite) {
    let x: u8 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2);
    let y: u8 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET);
    InitSpritePosToAnimTarget(sprite, TRUE);
    let mut rotation: u16 = ArcTan2Neg((*sprite).x - x as i16, (*sprite).y - y as i16);
    rotation += 0x6000;
    if IsContest() != 0 {
        rotation += 0x4000;
    }
    TrySetSpriteRotScale(sprite, FALSE, 0x100, 0x100, rotation);
    (*sprite).data[0] = gBattleAnimArgs[2];
    (*sprite).data[2] = x as i16;
    (*sprite).data[4] = y as i16;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimMeanLookEye(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
    (*sprite).data[0] = 4;
    (*sprite).callback = Some(AnimMeanLookEye_Step1);
}
pub(crate) unsafe fn AnimMeanLookEye_Step1(sprite: *mut Sprite) {
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - (*sprite).data[0] as u16) << 8 | (*sprite).data[0] as u16,
    );
    if (*sprite).data[1] != 0 {
        (*sprite).data[0] -= 1;
    } else {
        (*sprite).data[0] += 1;
    }
    if (*sprite).data[0] == 15 || (*sprite).data[0] == 4 {
        (*sprite).data[1] ^= 1;
    }
    if ({
        let t1 = (*sprite).data[2];
        (*sprite).data[2] += 1;
        t1
    }) > 70
    {
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        StartSpriteAffineAnim(sprite, 1);
        (*sprite).data[2] = 0;
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).set_affineAnimPaused(1);
        (*sprite).callback = Some(AnimMeanLookEye_Step2);
    }
}
pub(crate) unsafe fn AnimMeanLookEye_Step2(sprite: *mut Sprite) {
    if ({
        let t1 = (*sprite).data[2];
        (*sprite).data[2] += 1;
        t1
    }) > 9
    {
        (*sprite).set_invisible(FALSE as u16);
        (*sprite).set_affineAnimPaused(0);
        if (*sprite).affineAnimEnded() != 0 {
            (*sprite).callback = Some(AnimMeanLookEye_Step3);
        }
    }
}
pub(crate) unsafe fn AnimMeanLookEye_Step3(sprite: *mut Sprite) {
    match (*sprite).data[3] {
        0 | 1 => {
            (*sprite).x2 = 1;
            (*sprite).y2 = 0;
        }
        2 | 3 => {
            (*sprite).x2 = -1;
            (*sprite).y2 = 0;
        }
        4 | 5 => {
            (*sprite).x2 = 0;
            (*sprite).y2 = 1;
        }
        _ => {
            (*sprite).x2 = 0;
            (*sprite).y2 = -1;
        }
    }
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) > 7
    {
        (*sprite).data[3] = 0;
    }
    if ({
        let t2 = (*sprite).data[4];
        (*sprite).data[4] += 1;
        t2
    }) > 15
    {
        (*sprite).data[0] = 16;
        (*sprite).data[1] = 0;
        SetGpuReg(REG_OFFSET_BLDCNT, 16192);
        SetGpuReg(REG_OFFSET_BLDALPHA, (*sprite).data[0] as u16);
        (*sprite).callback = Some(AnimMeanLookEye_Step4);
    }
}
pub(crate) unsafe fn AnimMeanLookEye_Step4(sprite: *mut Sprite) {
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - (*sprite).data[0] as u16) << 8 | (*sprite).data[0] as u16,
    );
    if ({
        let t1 = (*sprite).data[1];
        (*sprite).data[1] += 1;
        t1
    }) > 1
    {
        (*sprite).data[0] -= 1;
        (*sprite).data[1] = 0;
    }
    if (*sprite).data[0] == 0 {
        (*sprite).set_invisible(TRUE as u16);
    }
    if (*sprite).data[0] < 0 {
        SetGpuReg(REG_OFFSET_BLDCNT, 0);
        SetGpuReg(REG_OFFSET_BLDALPHA, 0);
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SetPsychicBackground(taskId: u8) {
    task_set_func(taskId, Some(SetPsychicBackground_Step));
    gAnimVisualTaskCount -= 1;
}
pub(crate) unsafe fn SetPsychicBackground_Step(taskId: u8) {
    let mut i: i32 = 0;
    let mut lastColor: u16 = 0;
    let paletteIndex: u8 = GetBattleBgPaletteNum();
    if ({
        task_set(taskId, 5, task_get(taskId, 5) + 1);
        task_get(taskId, 5)
    }) == 4
    {
        lastColor = gPlttBufferFaded[(paletteIndex as i32 * 16) + 11];
        i = 10;
        while i > 0 {
            gPlttBufferFaded[(paletteIndex as i32 * 16) + i + 1] =
                gPlttBufferFaded[(paletteIndex as i32 * 16) + i];
            i -= 1;
        }
        gPlttBufferFaded[(paletteIndex as i32 * 16) + 1] = lastColor;
        task_set(taskId, 5, 0);
    }
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        DestroyTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FadeScreenToWhite(taskId: u8) {
    task_set_func(taskId, Some(FadeScreenToWhite_Step));
    gAnimVisualTaskCount -= 1;
}
pub(crate) unsafe fn FadeScreenToWhite_Step(taskId: u8) {
    let mut i: i32 = 0;
    let mut lastColor: u16 = 0;
    let paletteIndex: u8 = GetBattleBgPaletteNum();
    if ({
        task_set(taskId, 5, task_get(taskId, 5) + 1);
        task_get(taskId, 5)
    }) == 4
    {
        lastColor = gPlttBufferFaded[(paletteIndex as i32 * 16) + 11];
        i = 10;
        while i > 0 {
            gPlttBufferFaded[(paletteIndex as i32 * 16) + i + 1] =
                gPlttBufferFaded[(paletteIndex as i32 * 16) + i];
            i -= 1;
        }
        gPlttBufferFaded[(paletteIndex as i32 * 16) + 1] = lastColor;
        lastColor = gPlttBufferUnfaded[(paletteIndex as i32 * 16) + 11];
        i = 10;
        while i > 0 {
            gPlttBufferUnfaded[(paletteIndex as i32 * 16) + i + 1] =
                gPlttBufferUnfaded[(paletteIndex as i32 * 16) + i];
            i -= 1;
        }
        gPlttBufferUnfaded[(paletteIndex as i32 * 16) + 1] = lastColor;
        task_set(taskId, 5, 0);
    }
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        DestroyTask(taskId);
    }
}
pub(crate) unsafe fn AnimSpikes(sprite: *mut Sprite) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    SetAverageBattlerPositions(gBattleAnimTarget, FALSE, &raw mut x, &raw mut y);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        gBattleAnimArgs[2] = -gBattleAnimArgs[2];
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    (*sprite).data[2] = x + gBattleAnimArgs[2];
    (*sprite).data[4] = y + gBattleAnimArgs[3];
    (*sprite).data[5] = -50;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimSpikes_Step1);
}
pub(crate) unsafe fn AnimSpikes_Step1(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        (*sprite).data[0] = 30;
        (*sprite).data[1] = 0;
        (*sprite).callback = Some(WaitAnimForDuration);
        StoreSpriteCallbackInData6(sprite, Some(AnimSpikes_Step2));
    }
}
pub(crate) unsafe fn AnimSpikes_Step2(sprite: *mut Sprite) {
    if (*sprite).data[1] as i32 & 1 != 0 {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) == 16
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimLeer(sprite: *mut Sprite) {
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimLetterZ(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        SetAnimSpriteInitialXOffset(sprite, gBattleAnimArgs[0]);
        if IsContest() == 0 {
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                (*sprite).data[1] = gBattleAnimArgs[2];
                (*sprite).data[2] = gBattleAnimArgs[3];
            } else {
                (*sprite).data[1] = -gBattleAnimArgs[2];
                (*sprite).data[2] = -gBattleAnimArgs[3];
            }
        } else {
            (*sprite).data[1] = -gBattleAnimArgs[2];
            (*sprite).data[2] = gBattleAnimArgs[3];
        }
    }
    (*sprite).data[0] += 1;
    let var0: i32 = ((*sprite).data[0] as i32 * 20) & 0xFF;
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    (*sprite).x2 = (*sprite).data[3] / 2;
    (*sprite).y2 = Sin(var0 as i16 & 0xFF, 5) + (*sprite).data[4] / 2;
    if (*sprite).x as u16 as i32 + (*sprite).x2 as u16 as i32 > DISPLAY_WIDTH as i32 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimFang(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsTargetPlayerSide(taskId: u8) {
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_OPPONENT {
        gBattleAnimArgs[7] = FALSE as i16;
    } else {
        gBattleAnimArgs[7] = TRUE as i16;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsHealingMove(taskId: u8) {
    if gAnimMoveDmg > 0 {
        gBattleAnimArgs[7] = FALSE as i16;
    } else {
        gBattleAnimArgs[7] = TRUE as i16;
    }
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimSpotlight(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_WINOUT, 7999);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    InitSpritePosToAnimTarget(sprite, FALSE);
    (*sprite).oam.set_objMode(ST_OAM_OBJ_WINDOW);
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(AnimSpotlight_Step1);
}
pub(crate) unsafe fn AnimSpotlight_Step1(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).set_invisible(FALSE as u16);
            if (*sprite).affineAnimEnded() != 0 {
                (*sprite).data[0] += 1;
            }
        }
        1 | 3 => {
            (*sprite).data[1] += 117;
            (*sprite).x2 = (*sprite).data[1] >> 8;
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) == 21
            {
                (*sprite).data[2] = 0;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).data[1] -= 117;
            (*sprite).x2 = (*sprite).data[1] >> 8;
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) == 41
            {
                (*sprite).data[2] = 0;
                (*sprite).data[0] += 1;
            }
        }
        4 => {
            ChangeSpriteAffineAnim(sprite, 1);
            (*sprite).data[0] += 1;
        }
        5 if (*sprite).affineAnimEnded() != 0 => {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(AnimSpotlight_Step2);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimSpotlight_Step2(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_WINOUT, 16191);
    SetGpuReg(
        REG_OFFSET_DISPCNT,
        GetGpuReg(REG_OFFSET_DISPCNT) ^ DISPCNT_OBJWIN_ON,
    );
    DestroyAnimSprite(sprite);
}
pub(crate) unsafe fn AnimClappingHand(sprite: *mut Sprite) {
    if gBattleAnimArgs[3] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    }
    (*sprite).x += gBattleAnimArgs[0];
    (*sprite).y += gBattleAnimArgs[1];
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 16);
    if gBattleAnimArgs[2] == 0 {
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
        (*sprite).x2 = -12;
        (*sprite).data[1] = 2;
    } else {
        (*sprite).x2 = 12;
        (*sprite).data[1] = -2;
    }
    (*sprite).data[0] = gBattleAnimArgs[4];
    if (*sprite).data[3] != 255 {
        (*sprite).data[3] = gBattleAnimArgs[2];
    }
    (*sprite).callback = Some(AnimClappingHand_Step);
}
pub(crate) unsafe fn AnimClappingHand_Step(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).x2 += (*sprite).data[1];
        if (*sprite).x2 == 0 {
            (*sprite).data[2] += 1;
            if (*sprite).data[3] == 0 {
                PlaySE1WithPanning(SE_M_ENCORE, BattleAnimAdjustPanning(SOUND_PAN_ATTACKER));
            }
        }
    } else {
        (*sprite).x2 -= (*sprite).data[1];
        if (if (*sprite).x2 < 0 {
            -((*sprite).x2 as i32)
        } else {
            (*sprite).x2 as i32
        }) == 12
        {
            (*sprite).data[0] -= 1;
            (*sprite).data[2] -= 1;
        }
    }
    if (*sprite).data[0] == 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimClappingHand2(sprite: *mut Sprite) {
    (*sprite).oam.set_objMode(ST_OAM_OBJ_WINDOW);
    (*sprite).data[3] = 255;
    AnimClappingHand(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CreateSpotlight(taskId: u8) {
    if IsContest() != 0 {
        SetGpuReg(REG_OFFSET_WININ, 7999);
        gBattle_WIN1H = 39152;
        gBattle_WIN1V = DISPLAY_HEIGHT;
        SetGpuReg(REG_OFFSET_WIN1H, gBattle_WIN0H);
        SetGpuReg(REG_OFFSET_WIN1V, gBattle_WIN0V);
    } else {
        SetGpuReg(REG_OFFSET_WININ, 7999);
        gBattle_WIN1H = DISPLAY_WIDTH;
        gBattle_WIN1V = 30880;
        SetGpuReg(REG_OFFSET_WIN1H, gBattle_WIN1H);
        SetGpuReg(REG_OFFSET_WIN1V, gBattle_WIN1V);
        SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN1_ON);
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_RemoveSpotlight(taskId: u8) {
    SetGpuReg(REG_OFFSET_WININ, 16191);
    gBattle_WIN1H = 0;
    gBattle_WIN1V = 0;
    if IsContest() == 0 {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_WIN1_ON);
    }
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimRapidSpin(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + gBattleAnimArgs[1];
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    } else {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 + gBattleAnimArgs[1];
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    }
    (*sprite).y2 = gBattleAnimArgs[2];
    (*sprite).data[0] = ((*sprite).y2 > gBattleAnimArgs[3]) as i16;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = gBattleAnimArgs[4];
    (*sprite).data[3] = gBattleAnimArgs[5];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).callback = Some(AnimRapidSpin_Step);
}
pub(crate) unsafe fn AnimRapidSpin_Step(sprite: *mut Sprite) {
    (*sprite).data[1] = ((*sprite).data[1] + (*sprite).data[2]) & 0xFF;
    (*sprite).x2 =
        (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*sprite).data[1]] >> 4;
    (*sprite).y2 += (*sprite).data[3];
    if (*sprite).data[0] != 0 {
        if (*sprite).y2 < (*sprite).data[4] {
            DestroyAnimSprite(sprite);
        }
    } else {
        if (*sprite).y2 > (*sprite).data[4] {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_RapinSpinMonElevation(taskId: u8) {
    let mut var0: i16 = 0;
    let mut toBG2: u8 = 0;
    let mut var2: i16 = 0;
    let mut var3: i32 = 0;
    let mut var4: i32 = 0;
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[0] == 0 {
        var0 = GetBattlerYCoordWithElevation(gBattleAnimAttacker) as i16;
        toBG2 = GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker);
    } else {
        var0 = GetBattlerYCoordWithElevation(gBattleAnimTarget) as i16;
        toBG2 = GetBattlerSpriteBGPriorityRank(gBattleAnimTarget);
    }
    (*task).data[0] = var0 + 36;
    (*task).data[1] = (*task).data[0];
    (*task).data[2] = var0 - 33;
    if (*task).data[2] < 0 {
        (*task).data[2] = 0;
    }
    (*task).data[3] = (*task).data[0];
    (*task).data[4] = 8;
    (*task).data[5] = gBattleAnimArgs[1];
    (*task).data[6] = 0;
    (*task).data[7] = 0;
    if toBG2 == 1 {
        var3 = gBattle_BG1_X as i32;
        (*task).data[8] = var3 as i16;
        var4 = var3 + DISPLAY_WIDTH as i32;
    } else {
        var3 = gBattle_BG2_X as i32;
        (*task).data[8] = var3 as i16;
        var4 = var3 + DISPLAY_WIDTH as i32;
    }
    (*task).data[9] = var4 as i16;
    (*task).data[10] = gBattleAnimArgs[2];
    if gBattleAnimArgs[2] == 0 {
        (*task).data[11] = var4 as i16;
        var2 = (*task).data[8];
    } else {
        (*task).data[11] = var3 as i16;
        var2 = (*task).data[9];
    }
    (*task).data[15] = 0;
    let mut i: i16 = (*task).data[2];
    while i <= (*task).data[3] {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = var2 as u16;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = var2 as u16;
        i += 1;
    }
    if toBG2 == 1 {
        scanlineParams.dmaDest = 67108884_usize as *mut u16 as *mut c_void;
    } else {
        scanlineParams.dmaDest = 67108888_usize as *mut u16 as *mut c_void;
    }
    scanlineParams.dmaControl = 0xa2600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    ScanlineEffect_SetParams(scanlineParams);
    (*task).func = Some(RapinSpinMonElevation_Step);
}
pub(crate) unsafe fn RapinSpinMonElevation_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] -= (*task).data[5];
    if (*task).data[0] < (*task).data[2] {
        (*task).data[0] = (*task).data[2];
    }
    if (*task).data[4] == 0 {
        (*task).data[1] -= (*task).data[5];
        if (*task).data[1] < (*task).data[2] {
            (*task).data[1] = (*task).data[2];
            (*task).data[15] = 1;
        }
    } else {
        (*task).data[4] -= 1;
    }
    if ({
        (*task).data[6] += 1;
        (*task).data[6]
    }) > 1
    {
        (*task).data[6] = 0;
        (*task).data[7] = (if (*task).data[7] == 0 { 1 } else { 0 }) as i16;
        if (*task).data[7] != 0 {
            (*task).data[12] = (*task).data[8];
        } else {
            (*task).data[12] = (*task).data[9];
        }
    }
    for i in (*task).data[0]..(*task).data[1] {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = (*task).data[12] as u16;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = (*task).data[12] as u16;
    }
    let mut i: i16 = (*task).data[1];
    while i <= (*task).data[3] {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = (*task).data[11] as u16;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = (*task).data[11] as u16;
        i += 1;
    }
    if (*task).data[15] != 0 {
        if (*task).data[10] != 0 {
            (*(&raw const crate::scanline_effect::gScanlineEffect)
                .cast::<ScanlineEffect>()
                .cast_mut())
            .state = 3;
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_TormentAttacker(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*task).data[3] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*task).data[4] = 32;
    (*task).data[5] = -20;
    (*task).data[6] = 0;
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).func = Some(TormentAttacker_Step);
}
pub(crate) unsafe fn TormentAttacker_Step(taskId: u8) {
    let mut var0: i32 = 0;
    let mut var1: i32 = 0;
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let mut j: u16 = 0;
    let mut spriteId: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            var0 = (*task).data[2] as i32;
            if (*task).data[1] as i32 & 1 != 0 {
                var1 = (*task).data[4] as i32;
                x = var0 as i16 - var1 as i16;
            } else {
                var1 = (*task).data[4] as i32;
                x = var0 as i16 + var1 as i16;
            }
            y = (*task).data[3] + (*task).data[5];
            spriteId = CreateSprite(
                (&raw const (*(&raw const crate::data::battle_anim_effects_1::gThoughtBubbleSpriteTemplate).cast::<SpriteTemplate>())).cast_mut(),
                x,
                y,
                6 - (*task).data[1] as u8,
            );
            PlaySE12WithPanning(SE_M_METRONOME, BattleAnimAdjustPanning(SOUND_PAN_ATTACKER));
            if spriteId != MAX_SPRITES {
                gSprites[spriteId].set_hFlip((*task).data[1] as u16 & 1);
                gSprites[spriteId].callback = Some(SpriteCallbackDummy);
            }
            if (*task).data[1] as i32 & 1 != 0 {
                (*task).data[4] -= 6;
                (*task).data[5] -= 6;
            }
            PrepareAffineAnimInTaskData(
                task,
                (*task).data[15] as u8,
                sAffineAnims_Torment.as_ptr().cast_mut(),
            );
            (*task).data[1] += 1;
            (*task).data[0] = 1;
        }
        1 => {
            if RunAffineAnimFromTaskData(task) == 0 {
                if (*task).data[1] == 6 {
                    (*task).data[6] = 8;
                    (*task).data[0] = 3;
                } else {
                    if (*task).data[1] <= 2 {
                        (*task).data[6] = 10;
                    } else {
                        (*task).data[6] = 0;
                    }
                    (*task).data[0] = 2;
                }
            }
        }
        2 => {
            if (*task).data[6] != 0 {
                (*task).data[6] -= 1;
            } else {
                (*task).data[0] = 0;
            }
        }
        3 => {
            if (*task).data[6] != 0 {
                (*task).data[6] -= 1;
            } else {
                (*task).data[0] = 4;
            }
        }
        4 => {
            j = 0;
            for i in 0..(MAX_SPRITES as u16) {
                if gSprites[i].template == (&raw const (*(&raw const crate::data::battle_anim_effects_1::gThoughtBubbleSpriteTemplate).cast::<SpriteTemplate>())).cast_mut() {
                    gSprites[i].data[0] = taskId as i16;
                    gSprites[i].data[1] = 6;
                    StartSpriteAnim(&raw mut gSprites[i], 2);
                    gSprites[i].callback = Some(TormentAttacker_Callback);
                    if ({
                        j += 1;
                        j
                    }) == 6
                    {
                        break;
                    }
                }
            }
            (*task).data[6] = j as i16;
            (*task).data[0] = 5;
        }
        5 if (*task).data[6] == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn TormentAttacker_Callback(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        task_set(
            (*sprite).data[0],
            (*sprite).data[1],
            task_get((*sprite).data[0], (*sprite).data[1]) - 1,
        );
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn AnimTriAttackTriangle(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) < 40
    {
        let var: u16 = (*sprite).data[0] as u16;
        if var as i32 & 1 == 0 {
            (*sprite).set_invisible(TRUE as u16);
        } else {
            (*sprite).set_invisible(FALSE as u16);
        }
    }
    if (*sprite).data[0] > 30 {
        (*sprite).set_invisible(FALSE as u16);
    }
    if (*sprite).data[0] == 61 {
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).x2 = 0;
        (*sprite).y2 = 0;
        (*sprite).data[0] = 20;
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite).callback = Some(StartAnimLinearTranslation);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DefenseCurlDeformMon(taskId: u8) {
    match task_get(taskId, 0) {
        0 => {
            PrepareAffineAnimInTaskData(
                &raw mut (*gTasks.as_ptr())[taskId],
                GetAnimBattlerSpriteId(ANIM_ATTACKER),
                DefenseCurlDeformMonAffineAnimCmds.as_ptr().cast_mut(),
            );
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimBatonPassPokeball(sprite: *mut Sprite) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
            (*sprite).y =
                GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
            PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
            (*sprite).data[1] = 256;
            (*sprite).data[2] = 256;
            (*sprite).data[0] += 1;
            break 'l1;
        }
        if sw1 == 1 {
            fall = true;
            (*sprite).data[1] += 96;
            (*sprite).data[2] -= 26;
            SetSpriteRotScale(spriteId, (*sprite).data[1], (*sprite).data[2], 0);
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) == 5
            {
                (*sprite).data[0] += 1;
            }
        }
        if fall || sw1 == 2 {
            (*sprite).data[1] += 96;
            (*sprite).data[2] += 48;
            SetSpriteRotScale(spriteId, (*sprite).data[1], (*sprite).data[2], 0);
            if ({
                (*sprite).data[3] += 1;
                (*sprite).data[3]
            }) == 9
            {
                (*sprite).data[3] = 0;
                gSprites[spriteId].set_invisible(TRUE as u16);
                ResetSpriteRotScale(spriteId);
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 3 {
            (*sprite).y2 -= 6;
            if ((*sprite).y as i32 + (*sprite).y2 as i32) < -32 {
                DestroyAnimSprite(sprite);
            }
            break 'l1;
        }
    }
}
pub(crate) unsafe fn AnimWishStar(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x = -16;
    } else {
        (*sprite).x = 256;
    }
    (*sprite).y = 0;
    (*sprite).callback = Some(AnimWishStar_Step);
}
pub(crate) unsafe fn AnimWishStar_Step(sprite: *mut Sprite) {
    (*sprite).data[0] += 72;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x2 = (*sprite).data[0] >> 4;
    } else {
        (*sprite).x2 = -((*sprite).data[0] >> 4);
    }
    (*sprite).data[1] += 16;
    (*sprite).y2 += (*sprite).data[1] >> 8;
    if ({
        (*sprite).data[2] += 1;
        (*sprite).data[2]
    }) % 3
        == 0
    {
        CreateSpriteAndAnimate(
            (&raw const *gMiniTwinklingStarSpriteTemplate).cast_mut(),
            (*sprite).x + (*sprite).x2,
            (*sprite).y + (*sprite).y2,
            (*sprite).subpriority + 1,
        );
    }
    let newX: u32 = (*sprite).x as u32 + (*sprite).x2 as u32 + 32;
    if newX > 304 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimMiniTwinklingStar(sprite: *mut Sprite) {
    let rand: u8 = Random2() as u8 & 3;
    if rand == 0 {
        (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 4);
    } else {
        (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 5);
    }
    let mut y: i8 = Random2() as i8 & 7;
    if y > 3 {
        y = -y;
    }
    (*sprite).y2 = y as i16;
    (*sprite).callback = Some(AnimMiniTwinklingStar_Step);
}
pub(crate) unsafe fn AnimMiniTwinklingStar_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) < 30
    {
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) == 2
        {
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
            (*sprite).data[1] = 0;
        }
    } else {
        if (*sprite).data[1] == 2 {
            (*sprite).set_invisible(FALSE as u16);
        }
        if (*sprite).data[1] == 3 {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).data[1] = -1;
        }
        (*sprite).data[1] += 1;
    }
    if (*sprite).data[0] > 60 {
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StockpileDeformMon(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            GetAnimBattlerSpriteId(ANIM_ATTACKER),
            gStockpileDeformMonAffineAnimCmds.as_ptr().cast_mut(),
        );
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else {
        if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SpitUpDeformMon(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            GetAnimBattlerSpriteId(ANIM_ATTACKER),
            gSpitUpDeformMonAffineAnimCmds.as_ptr().cast_mut(),
        );
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else {
        if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe fn AnimSwallowBlueOrb(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            InitSpritePosToAnimAttacker(sprite, FALSE);
            (*sprite).data[1] = 0x900;
            (*sprite).data[2] =
                GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
            (*sprite).data[0] += 1;
        }
        1 => {
            (*sprite).y2 -= (*sprite).data[1] >> 8;
            (*sprite).data[1] -= 96;
            if (*sprite).y as i32 + (*sprite).y2 as i32 > (*sprite).data[2] as i32 {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SwallowDeformMon(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            GetAnimBattlerSpriteId(ANIM_ATTACKER),
            gSwallowDeformMonAffineAnimCmds.as_ptr().cast_mut(),
        );
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else {
        if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_TransformMon(taskId: u8) {
    let mut position: u8 = 0;
    let mut animBg: BattleAnimBgData = zeroed();
    let mut dest: *mut u8 = null_mut();
    let mut src: *mut u8 = null_mut();
    let mut bgTilemap: *mut u16 = null_mut();
    let mut stretch: u16 = 0;
    match task_get(taskId, 0) {
        0 => {
            SetGpuReg(REG_OFFSET_MOSAIC, 0);
            if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
                SetAnimBgAttribute(1, BG_ANIM_MOSAIC, 1);
            } else {
                SetAnimBgAttribute(2, BG_ANIM_MOSAIC, 1);
            }
            task_set(taskId, 10, gBattleAnimArgs[0]);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 => {
            if ({
                let t1 = task_get(taskId, 2);
                task_set(taskId, 2, task_get(taskId, 2) + 1);
                t1
            }) > 1
            {
                task_set(taskId, 2, 0);
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                stretch = task_get(taskId, 1) as u16;
                SetGpuReg(REG_OFFSET_MOSAIC, stretch << 4 | stretch);
                if stretch == 15 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        2 => {
            HandleSpeciesGfxDataChange(
                gBattleAnimAttacker,
                gBattleAnimTarget,
                task_get(taskId, 10) as u8,
            );
            GetBgDataForTransform(&raw mut animBg, gBattleAnimAttacker);
            if IsContest() != 0 {
                position = B_POSITION_PLAYER_LEFT;
            } else {
                position = GetBattlerPosition(gBattleAnimAttacker);
            }
            src = ((*gMonSpritesGfxPtr).sprites.ptr[position] as *mut u8)
                .at((gBattleMonForms[gBattleAnimAttacker] as i32) << 11)
                as *mut c_void as *mut u8;
            dest = animBg.bgTiles;
            CpuSet(src as *mut c_void, dest as *mut c_void, 0x4000200);
            LoadBgTiles(1, animBg.bgTiles as *mut c_void, 0x800, animBg.tilesOffset);
            if IsContest() != 0 {
                if IsSpeciesNotUnown((*(*gContestResources).moveAnim).species)
                    != IsSpeciesNotUnown((*(*gContestResources).moveAnim).targetSpecies)
                {
                    bgTilemap = animBg.bgTilemap;
                    for i in 0..8i32 {
                        for j in 0..4i32 {
                            let temp: u16 = *bgTilemap.at(j + i * 0x20);
                            *bgTilemap.at(j + i * 0x20) = *bgTilemap.at(7 - j + i * 0x20);
                            *bgTilemap.at(7 - j + i * 0x20) = temp;
                        }
                    }
                    for i in 0..8i32 {
                        for j in 0..8i32 {
                            *bgTilemap.at(j + i * 0x20) ^= 0x400;
                        }
                    }
                }
                if IsSpeciesNotUnown((*(*gContestResources).moveAnim).targetSpecies) != 0 {
                    gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].affineAnims =
                        (*(&raw const crate::data::data_tables::gAffineAnims_BattleSpriteContest)
                            .cast::<CArray<*mut AffineAnimCmd, 0>>())
                        .as_ptr()
                        .cast_mut();
                } else {
                    gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].affineAnims =
                        (*(&raw const crate::data::data_tables::gAffineAnims_BattleSpriteOpponentSide).cast::<CArray<*mut AffineAnimCmd, 0>>()).as_ptr().cast_mut();
                }
                StartSpriteAffineAnim(
                    &raw mut gSprites[gBattlerSpriteIds[gBattleAnimAttacker]],
                    BATTLER_AFFINE_NORMAL,
                );
            }
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        3 => {
            if ({
                let t2 = task_get(taskId, 2);
                task_set(taskId, 2, task_get(taskId, 2) + 1);
                t2
            }) > 1
            {
                task_set(taskId, 2, 0);
                task_set(taskId, 1, task_get(taskId, 1) - 1);
                stretch = task_get(taskId, 1) as u16;
                SetGpuReg(REG_OFFSET_MOSAIC, stretch << 4 | stretch);
                if stretch == 0 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        4 => {
            SetGpuReg(REG_OFFSET_MOSAIC, 0);
            if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
                SetAnimBgAttribute(1, BG_ANIM_MOSAIC, 0);
            } else {
                SetAnimBgAttribute(2, BG_ANIM_MOSAIC, 0);
            }
            if IsContest() == 0
                && GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT
                && task_get(taskId, 10) == 0
            {
                SetBattlerShadowSpriteCallback(
                    gBattleAnimAttacker,
                    (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker))
                        .transformSpecies,
                );
            }
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_IsMonInvisible(taskId: u8) {
    gBattleAnimArgs[7] = gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].invisible() as i16;
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CastformGfxDataChange(taskId: u8) {
    HandleSpeciesGfxDataChange(gBattleAnimAttacker, gBattleAnimTarget, TRUE);
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MorningSunLightBeam(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    match task_get(taskId, 0) {
        0 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            SetGpuReg(REG_OFFSET_BLDALPHA, 4096);
            SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
            }
            GetBattleAnimBg1Data(&raw mut animBg);
            AnimLoadCompressedBgTilemapHandleContest(
                &raw mut animBg,
                (&raw const (*(&raw const crate::data::graphics::gBattleAnimMaskTilemap_LightBeam)
                    .cast::<CArray<u32, 0>>()))
                    .cast_mut() as *mut c_void,
                FALSE as u32,
            );
            if IsContest() != 0 {
                gBattle_BG1_X = 65480;
                gBattle_BG1_Y = 0;
            } else {
                if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
                    gBattle_BG1_X = 65401;
                } else {
                    gBattle_BG1_X = 65526;
                }
                gBattle_BG1_Y = 0;
            }
            AnimLoadCompressedBgGfx(
                animBg.bgId as u32,
                (*(&raw const crate::data::graphics::gBattleAnimMaskImage_LightBeam)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBg.tilesOffset as u32,
            );
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBattleAnimMaskPalette_LightBeam)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBg.paletteId as u16 * 16,
                32,
            );
            task_set(taskId, 10, gBattle_BG1_X as i16);
            task_set(taskId, 11, gBattle_BG1_Y as i16);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
            PlaySE12WithPanning(
                SE_M_MORNING_SUN,
                BattleAnimAdjustPanning(SOUND_PAN_ATTACKER),
            );
        }
        1 => {
            if ({
                let t1 = task_get(taskId, 4);
                task_set(taskId, 4, task_get(taskId, 4) + 1);
                t1
            }) > 0
            {
                task_set(taskId, 4, 0);
                if ({
                    task_set(taskId, 1, task_get(taskId, 1) + 1);
                    task_get(taskId, 1)
                }) > 12
                {
                    task_set(taskId, 1, 12);
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    (16 - task_get(taskId, 1) as u16) << 8 | task_get(taskId, 1) as u16,
                );
                if task_get(taskId, 1) == 12 {
                    task_set(taskId, 0, task_get(taskId, 0) + 1);
                }
            }
        }
        2 => {
            if ({
                task_set(taskId, 1, task_get(taskId, 1) - 1);
                task_get(taskId, 1)
            }) < 0
            {
                task_set(taskId, 1, 0);
            }
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                (16 - task_get(taskId, 1) as u16) << 8 | task_get(taskId, 1) as u16,
            );
            if task_get(taskId, 1) == 0 {
                gBattle_BG1_X = gMorningSunLightBeamCoordsTable[task_get(taskId, 2)] as u16
                    + task_get(taskId, 10) as u16;
                if ({
                    task_set(taskId, 2, task_get(taskId, 2) + 1);
                    task_get(taskId, 2)
                }) == 4
                {
                    task_set(taskId, 0, 4);
                } else {
                    task_set(taskId, 0, 3);
                }
            }
        }
        3 => {
            if ({
                task_set(taskId, 3, task_get(taskId, 3) + 1);
                task_get(taskId, 3)
            }) == 4
            {
                task_set(taskId, 3, 0);
                task_set(taskId, 0, 1);
                PlaySE12WithPanning(
                    SE_M_MORNING_SUN,
                    BattleAnimAdjustPanning(SOUND_PAN_ATTACKER),
                );
            }
        }
        4 => {
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(animBg.bgId as u32);
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimGreenStar(sprite: *mut Sprite) {
    let mut xOffset: i16 = Random2() as i16;
    xOffset &= 0x3F;
    if xOffset > 31 {
        xOffset = 32 - xOffset;
    }
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + xOffset;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + 32;
    (*sprite).data[1] = gBattleAnimArgs[0];
    (*sprite).data[2] = gBattleAnimArgs[1];
    let spriteId1: u8 = CreateSprite(
        (&raw const *gGreenStarSpriteTemplate).cast_mut(),
        (*sprite).x,
        (*sprite).y,
        (*sprite).subpriority + 1,
    );
    let spriteId2: u8 = CreateSprite(
        (&raw const *gGreenStarSpriteTemplate).cast_mut(),
        (*sprite).x,
        (*sprite).y,
        (*sprite).subpriority + 1,
    );
    StartSpriteAnim(&raw mut gSprites[spriteId1], 1);
    StartSpriteAnim(&raw mut gSprites[spriteId2], 2);
    gSprites[spriteId1].data[1] = gBattleAnimArgs[0];
    gSprites[spriteId1].data[2] = gBattleAnimArgs[1];
    gSprites[spriteId2].data[1] = gBattleAnimArgs[0];
    gSprites[spriteId2].data[2] = gBattleAnimArgs[1];
    gSprites[spriteId1].data[7] = -1;
    gSprites[spriteId2].data[7] = -1;
    gSprites[spriteId1].set_invisible(TRUE as u16);
    gSprites[spriteId2].set_invisible(TRUE as u16);
    gSprites[spriteId1].callback = Some(AnimGreenStar_Callback);
    gSprites[spriteId2].callback = Some(AnimGreenStar_Callback);
    (*sprite).data[6] = spriteId1 as i16;
    (*sprite).data[7] = spriteId2 as i16;
    (*sprite).callback = Some(AnimGreenStar_Step1);
}
pub(crate) unsafe fn AnimGreenStar_Step1(sprite: *mut Sprite) {
    let delta: i16 = (*sprite).data[3] + (*sprite).data[2];
    (*sprite).y2 -= delta >> 8;
    (*sprite).data[3] += (*sprite).data[2];
    (*sprite).data[3] &= 0xFF;
    if (*sprite).data[4] == 0 && (*sprite).y2 < -8 {
        gSprites[(*sprite).data[6]].set_invisible(FALSE as u16);
        (*sprite).data[4] += 1;
    }
    if (*sprite).data[4] == 1 && (*sprite).y2 < -16 {
        gSprites[(*sprite).data[7]].set_invisible(FALSE as u16);
        (*sprite).data[4] += 1;
    }
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == -1
    {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).callback = Some(AnimGreenStar_Step2);
    }
}
pub(crate) unsafe fn AnimGreenStar_Step2(sprite: *mut Sprite) {
    if gSprites[(*sprite).data[6]].callback == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
        && gSprites[(*sprite).data[7]].callback
            == Some(SpriteCallbackDummy as unsafe fn(*mut Sprite))
    {
        DestroySprite(&raw mut gSprites[(*sprite).data[6]]);
        DestroySprite(&raw mut gSprites[(*sprite).data[7]]);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimGreenStar_Callback(sprite: *mut Sprite) {
    if (*sprite).invisible() == 0 {
        let delta: i16 = (*sprite).data[3] + (*sprite).data[2];
        (*sprite).y2 -= delta >> 8;
        (*sprite).data[3] += (*sprite).data[2];
        (*sprite).data[3] &= 0xFF;
        if ({
            (*sprite).data[1] -= 1;
            (*sprite).data[1]
        }) == -1
        {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(SpriteCallbackDummy);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DoomDesireLightBeam(taskId: u8) {
    let mut animBg: BattleAnimBgData = zeroed();
    match task_get(taskId, 0) {
        0 => {
            SetGpuReg(REG_OFFSET_BLDCNT, 16194);
            SetGpuReg(REG_OFFSET_BLDALPHA, 3331);
            SetAnimBgAttribute(1, BG_ANIM_SCREEN_SIZE, 0);
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 1);
            }
            GetBattleAnimBg1Data(&raw mut animBg);
            AnimLoadCompressedBgTilemapHandleContest(
                &raw mut animBg,
                (&raw const (*(&raw const crate::data::graphics::gBattleAnimMaskTilemap_LightBeam)
                    .cast::<CArray<u32, 0>>()))
                    .cast_mut() as *mut c_void,
                FALSE as u32,
            );
            if IsContest() != 0 {
                gBattle_BG1_X = 65480;
                gBattle_BG1_Y = 0;
            } else {
                let position: u8 = GetBattlerPosition(gBattleAnimTarget);
                if IsDoubleBattle() == TRUE {
                    if position == B_POSITION_OPPONENT_LEFT {
                        gBattle_BG1_X = 65381;
                    }
                    if position == B_POSITION_OPPONENT_RIGHT {
                        gBattle_BG1_X = 65421;
                    }
                    if position == B_POSITION_PLAYER_LEFT {
                        gBattle_BG1_X = 14;
                    }
                    if position == B_POSITION_PLAYER_RIGHT {
                        gBattle_BG1_X = 65516;
                    }
                } else {
                    if position == B_POSITION_OPPONENT_LEFT {
                        gBattle_BG1_X = 65401;
                    }
                    if position == B_POSITION_PLAYER_LEFT {
                        gBattle_BG1_X = 65526;
                    }
                }
                gBattle_BG1_Y = 0;
            }
            AnimLoadCompressedBgGfx(
                animBg.bgId as u32,
                (*(&raw const crate::data::graphics::gBattleAnimMaskImage_LightBeam)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBg.tilesOffset as u32,
            );
            LoadCompressedPalette(
                (*(&raw const crate::data::graphics::gBattleAnimMaskPalette_LightBeam)
                    .cast::<CArray<u32, 0>>())
                .as_ptr()
                .cast_mut(),
                animBg.paletteId as u16 * 16,
                32,
            );
            task_set(taskId, 10, gBattle_BG1_X as i16);
            task_set(taskId, 11, gBattle_BG1_Y as i16);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 => {
            task_set(taskId, 3, 0);
            if GetBattlerSide(gBattleAnimTarget) == B_SIDE_OPPONENT {
                gBattle_BG1_X = task_get(taskId, 10) as u16
                    + gDoomDesireLightBeamCoordTable[task_get(taskId, 2)] as u16;
            } else {
                gBattle_BG1_X = task_get(taskId, 10) as u16
                    - gDoomDesireLightBeamCoordTable[task_get(taskId, 2)] as u16;
            }
            if ({
                task_set(taskId, 2, task_get(taskId, 2) + 1);
                task_get(taskId, 2)
            }) == 5
            {
                task_set(taskId, 0, 5);
            } else {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        2 => {
            if ({
                task_set(taskId, 1, task_get(taskId, 1) - 1);
                task_get(taskId, 1)
            }) <= 4
            {
                task_set(taskId, 1, 5);
            }
            SetGpuReg(REG_OFFSET_BLDALPHA, (task_get(taskId, 1) as u16) << 8 | 3);
            if task_get(taskId, 1) == 5 {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        3 => {
            if ({
                task_set(taskId, 3, task_get(taskId, 3) + 1);
                task_get(taskId, 3)
            }) > gDoomDesireLightBeamDelayTable[task_get(taskId, 2)] as i16
            {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        4 => {
            if ({
                task_set(taskId, 1, task_get(taskId, 1) + 1);
                task_get(taskId, 1)
            }) > 13
            {
                task_set(taskId, 1, 13);
            }
            SetGpuReg(REG_OFFSET_BLDALPHA, (task_get(taskId, 1) as u16) << 8 | 3);
            if task_get(taskId, 1) == 13 {
                task_set(taskId, 0, 1);
            }
        }
        5 => {
            GetBattleAnimBg1Data(&raw mut animBg);
            ClearBattleAnimBg(animBg.bgId as u32);
            if IsContest() == 0 {
                SetAnimBgAttribute(1, BG_ANIM_CHAR_BASE_BLOCK, 0);
            }
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
            gBattle_BG1_X = 0;
            gBattle_BG1_Y = 0;
            SetGpuReg(REG_OFFSET_BLDCNT, 0);
            SetGpuReg(REG_OFFSET_BLDALPHA, 0);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StrongFrustrationGrowAndShrink(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            GetAnimBattlerSpriteId(ANIM_ATTACKER),
            gStrongFrustrationAffineAnimCmds.as_ptr().cast_mut(),
        );
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else {
        if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe fn AnimWeakFrustrationAngerMark(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        InitSpritePosToAnimAttacker(sprite, FALSE);
        (*sprite).data[0] += 1;
    } else if ({
        let t1 = (*sprite).data[0];
        (*sprite).data[0] += 1;
        t1
    }) > 20
    {
        (*sprite).data[1] += 160;
        (*sprite).data[2] += 128;
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            (*sprite).x2 = -((*sprite).data[1] >> 8);
        } else {
            (*sprite).x2 = (*sprite).data[1] >> 8;
        }
        (*sprite).y2 += (*sprite).data[2] >> 8;
        if (*sprite).y2 > 64 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_RockMonBackAndForth(taskId: u8) {
    let mut side: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[1] == 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    if gBattleAnimArgs[2] < 0 {
        gBattleAnimArgs[2] = 0;
    }
    if gBattleAnimArgs[2] > 2 {
        gBattleAnimArgs[2] = 2;
    }
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 8 - 2 * gBattleAnimArgs[2];
    (*task).data[4] = 0x100 + gBattleAnimArgs[2] * 128;
    (*task).data[5] = gBattleAnimArgs[2] + 2;
    (*task).data[6] = gBattleAnimArgs[1] - 1;
    (*task).data[15] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    if gBattleAnimArgs[0] == 0 {
        side = GetBattlerSide(gBattleAnimAttacker);
    } else {
        side = GetBattlerSide(gBattleAnimTarget);
    }
    if side == B_SIDE_OPPONENT {
        (*task).data[4] *= -1;
        (*task).data[5] *= -1;
    }
    PrepareBattlerSpriteForRotScale((*task).data[15] as u8, ST_OAM_OBJ_NORMAL);
    (*task).func = Some(AnimTask_RockMonBackAndForth_Step);
}
pub(crate) unsafe fn AnimTask_RockMonBackAndForth_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            gSprites[(*task).data[15]].x2 += (*task).data[5];
            (*task).data[2] -= (*task).data[4];
            SetSpriteRotScale((*task).data[15] as u8, 0x100, 0x100, (*task).data[2] as u16);
            SetBattlerSpriteYOffsetFromRotation((*task).data[15] as u8);
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) >= (*task).data[3]
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        1 => {
            gSprites[(*task).data[15]].x2 -= (*task).data[5];
            (*task).data[2] += (*task).data[4];
            SetSpriteRotScale((*task).data[15] as u8, 0x100, 0x100, (*task).data[2] as u16);
            SetBattlerSpriteYOffsetFromRotation((*task).data[15] as u8);
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) as i32
                >= (*task).data[3] as i32 * 2
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            gSprites[(*task).data[15]].x2 += (*task).data[5];
            (*task).data[2] -= (*task).data[4];
            SetSpriteRotScale((*task).data[15] as u8, 0x100, 0x100, (*task).data[2] as u16);
            SetBattlerSpriteYOffsetFromRotation((*task).data[15] as u8);
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) >= (*task).data[3]
            {
                if (*task).data[6] != 0 {
                    (*task).data[6] -= 1;
                    (*task).data[1] = 0;
                    (*task).data[0] = 0;
                } else {
                    (*task).data[0] += 1;
                }
            }
        }
        3 => {
            ResetSpriteRotScale((*task).data[15] as u8);
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimSweetScentPetal(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).x = 0;
        (*sprite).y = gBattleAnimArgs[0];
    } else {
        (*sprite).x = DISPLAY_WIDTH as i16;
        (*sprite).y = gBattleAnimArgs[0] - 30;
    }
    (*sprite).data[2] = gBattleAnimArgs[2];
    StartSpriteAnim(sprite, gBattleAnimArgs[1] as u8);
    (*sprite).callback = Some(AnimSweetScentPetal_Step);
}
pub(crate) unsafe fn AnimSweetScentPetal_Step(sprite: *mut Sprite) {
    (*sprite).data[0] += 3;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).x += 5;
        (*sprite).y -= 1;
        if (*sprite).x > DISPLAY_WIDTH as i16 {
            DestroyAnimSprite(sprite);
        }
        (*sprite).y2 = Sin((*sprite).data[0] & 0xFF, 16);
    } else {
        (*sprite).x -= 5;
        (*sprite).y += 1;
        if (*sprite).x < 0 {
            DestroyAnimSprite(sprite);
        }
        (*sprite).y2 = Cos((*sprite).data[0] & 0xFF, 16);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FlailMovement(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 0;
    (*task).data[12] = 0x20;
    (*task).data[13] = 0x40;
    (*task).data[14] = 0x800;
    (*task).data[15] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    PrepareBattlerSpriteForRotScale((*task).data[15] as u8, ST_OAM_OBJ_NORMAL);
    (*task).func = Some(AnimTask_FlailMovement_Step);
}
pub(crate) unsafe fn AnimTask_FlailMovement_Step(taskId: u8) {
    let mut temp: i32 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[2] += 0x200;
            if (*task).data[2] >= (*task).data[14] {
                let diff: i16 = (*task).data[14] - (*task).data[2];
                let div: i16 = div_i32(diff as i32, (*task).data[14] as i32 * 2) as i16;
                let r#mod: i16 = rem_i32(diff as i32, (*task).data[14] as i32 * 2) as i16;
                if div as i32 & 1 == 0 {
                    (*task).data[2] = (*task).data[14] - r#mod;
                    (*task).data[0] = 1;
                } else {
                    (*task).data[2] = r#mod - (*task).data[14];
                }
            }
        }
        1 => {
            (*task).data[2] -= 0x200;
            if (*task).data[2] as i32 <= -((*task).data[14] as i32) {
                let diff: i16 = (*task).data[14] - (*task).data[2];
                let div: i16 = div_i32(diff as i32, (*task).data[14] as i32 * 2) as i16;
                let r#mod: i16 = rem_i32(diff as i32, (*task).data[14] as i32 * 2) as i16;
                if 1 & div as i32 == 0 {
                    (*task).data[2] = r#mod - (*task).data[14];
                    (*task).data[0] = 0;
                } else {
                    (*task).data[2] = (*task).data[14] - r#mod;
                }
            }
        }
        2 => {
            ResetSpriteRotScale((*task).data[15] as u8);
            DestroyAnimVisualTask(taskId);
            return;
        }
        _ => {}
    }
    SetSpriteRotScale((*task).data[15] as u8, 0x100, 0x100, (*task).data[2] as u16);
    SetBattlerSpriteYOffsetFromRotation((*task).data[15] as u8);
    gSprites[(*task).data[15]].x2 = -(((if ({
        temp = (*task).data[2] as i32;
        temp
    }) >= 0
    {
        (*task).data[2] as i32
    } else {
        temp + 63
    }) >> 6) as i16);
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 8
    {
        if (*task).data[12] != 0 {
            (*task).data[12] -= 1;
            (*task).data[14] -= (*task).data[13];
            if (*task).data[14] < 16 {
                (*task).data[14] = 16;
            }
        } else {
            (*task).data[0] = 2;
        }
    }
}
pub(crate) unsafe fn AnimPainSplitProjectile(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        if gBattleAnimArgs[2] == ANIM_ATTACKER as i16 {
            (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
            (*sprite).y =
                GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        }
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        (*sprite).data[1] = 0x80;
        (*sprite).data[2] = 0x300;
        (*sprite).data[3] = gBattleAnimArgs[1];
        (*sprite).data[0] += 1;
    } else {
        (*sprite).x2 = (*sprite).data[1] >> 8;
        (*sprite).y2 += (*sprite).data[2] >> 8;
        if (*sprite).data[4] == 0 && (*sprite).y2 as i32 > -((*sprite).data[3] as i32) {
            (*sprite).data[4] = 1;
            (*sprite).data[2] = (-((*sprite).data[2] as i32) / 3) as i16 * 2;
        }
        (*sprite).data[1] += 192;
        (*sprite).data[2] += 128;
        if (*sprite).animEnded() != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_PainSplitMovement(taskId: u8) {
    let mut spriteId: u8 = 0;
    if task_get(taskId, 0) == 0 {
        if gBattleAnimArgs[0] == 0 {
            task_set(taskId, 11, gBattleAnimAttacker as i16);
        } else {
            task_set(taskId, 11, gBattleAnimTarget as i16);
        }
        spriteId = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
        task_set(taskId, 10, spriteId as i16);
        PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
        match gBattleAnimArgs[1] {
            0 => {
                SetSpriteRotScale(spriteId, 0xE0, 0x140, 0);
                SetBattlerSpriteYOffsetFromYScale(spriteId);
            }
            1 => {
                SetSpriteRotScale(spriteId, 0xD0, 0x130, 0xF00);
                SetBattlerSpriteYOffsetFromYScale(spriteId);
                if IsContest() != 0 || GetBattlerSide(task_get(taskId, 11) as u8) == B_SIDE_PLAYER {
                    gSprites[spriteId].y2 += 16;
                }
            }
            2 => {
                SetSpriteRotScale(spriteId, 0xD0, 0x130, 0xF100);
                SetBattlerSpriteYOffsetFromYScale(spriteId);
                if IsContest() != 0 || GetBattlerSide(task_get(taskId, 11) as u8) == B_SIDE_PLAYER {
                    gSprites[spriteId].y2 += 16;
                }
            }
            _ => {}
        }
        gSprites[spriteId].x2 = 2;
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else {
        spriteId = task_get(taskId, 10) as u8;
        if ({
            task_set(taskId, 2, task_get(taskId, 2) + 1);
            task_get(taskId, 2)
        }) == 3
        {
            task_set(taskId, 2, 0);
            gSprites[spriteId].x2 = -gSprites[spriteId].x2;
        }
        if ({
            task_set(taskId, 1, task_get(taskId, 1) + 1);
            task_get(taskId, 1)
        }) == 13
        {
            ResetSpriteRotScale(spriteId);
            gSprites[spriteId].x2 = 0;
            gSprites[spriteId].y2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe fn AnimFlatterConfetti(sprite: *mut Sprite) {
    let tileOffset: u8 = (Random2() as i32 % 12) as u8;
    (*sprite)
        .oam
        .set_tileNum((*sprite).oam.tileNum() + tileOffset as u16);
    let rand1: i32 = Random2() as i32 & 0x1FF;
    let rand2: i32 = Random2() as i32 & 0xFF;
    if rand1 & 1 != 0 {
        (*sprite).data[0] = 0x5E0 + rand1 as i16;
    } else {
        (*sprite).data[0] = 0x5E0 - rand1 as i16;
    }
    if rand2 & 1 != 0 {
        (*sprite).data[1] = 0x480 + rand2 as i16;
    } else {
        (*sprite).data[1] = 0x480 - rand2 as i16;
    }
    (*sprite).data[2] = gBattleAnimArgs[0];
    if (*sprite).data[2] == ANIM_ATTACKER as i16 {
        (*sprite).x = -8;
    } else {
        (*sprite).x = 248;
    }
    (*sprite).y = 104;
    (*sprite).callback = Some(AnimFlatterConfetti_Step);
}
pub(crate) unsafe fn AnimFlatterConfetti_Step(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        (*sprite).x2 += (*sprite).data[0] >> 8;
        (*sprite).y2 -= (*sprite).data[1] >> 8;
    } else {
        (*sprite).x2 -= (*sprite).data[0] >> 8;
        (*sprite).y2 -= (*sprite).data[1] >> 8;
    }
    (*sprite).data[0] -= 22;
    (*sprite).data[1] -= 48;
    if (*sprite).data[0] < 0 {
        (*sprite).data[0] = 0;
    }
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) == 31
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimFlatterSpotlight(sprite: *mut Sprite) {
    SetGpuReg(REG_OFFSET_WINOUT, 7999);
    SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_OBJWIN_ON);
    gBattle_WIN0H = 0;
    gBattle_WIN0V = 0;
    SetGpuReg(REG_OFFSET_WIN0H, gBattle_WIN0H);
    SetGpuReg(REG_OFFSET_WIN0V, gBattle_WIN0V);
    (*sprite).data[0] = gBattleAnimArgs[2];
    InitSpritePosToAnimTarget(sprite, FALSE);
    (*sprite).oam.set_objMode(ST_OAM_OBJ_WINDOW);
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).callback = Some(AnimFlatterSpotlight_Step);
}
pub(crate) unsafe fn AnimFlatterSpotlight_Step(sprite: *mut Sprite) {
    match (*sprite).data[1] {
        0 => {
            (*sprite).set_invisible(FALSE as u16);
            if (*sprite).affineAnimEnded() != 0 {
                (*sprite).data[1] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[0] -= 1;
                (*sprite).data[0]
            }) == 0
            {
                ChangeSpriteAffineAnim(sprite, 1);
                (*sprite).data[1] += 1;
            }
        }
        2 => {
            if (*sprite).affineAnimEnded() != 0 {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).data[1] += 1;
            }
        }
        3 => {
            SetGpuReg(REG_OFFSET_WINOUT, 16191);
            SetGpuReg(
                REG_OFFSET_DISPCNT,
                GetGpuReg(REG_OFFSET_DISPCNT) ^ DISPCNT_OBJWIN_ON,
            );
            DestroyAnimSprite(sprite);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimReversalOrb(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = gBattleAnimArgs[0];
    (*sprite).data[1] = gBattleAnimArgs[1];
    (*sprite).callback = Some(AnimReversalOrb_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimReversalOrb_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[2] >> 8);
    (*sprite).y2 = Cos((*sprite).data[1], (*sprite).data[3] >> 8);
    (*sprite).data[1] = ((*sprite).data[1] + 9) & 0xFF;
    if ((*sprite).data[1] as u16) < 64 || (*sprite).data[1] > 195 {
        (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) - 1;
    } else {
        (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) + 1;
    }
    if (*sprite).data[5] == 0 {
        (*sprite).data[2] += 0x400;
        (*sprite).data[3] += 0x100;
        (*sprite).data[4] += 1;
        if (*sprite).data[4] == (*sprite).data[0] {
            (*sprite).data[4] = 0;
            (*sprite).data[5] = 1;
        }
    } else if (*sprite).data[5] == 1 {
        (*sprite).data[2] -= 0x400;
        (*sprite).data[3] -= 0x100;
        (*sprite).data[4] += 1;
        if (*sprite).data[4] == (*sprite).data[0] {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_RolePlaySilhouette(taskId: u8) {
    let mut isBackPic: u8 = 0;
    let mut personality: u32 = 0;
    let mut otId: u32 = 0;
    let mut species: u16 = 0;
    let mut xOffset: i16 = 0;
    let mut priority: u32 = 0;
    GetAnimBattlerSpriteId(ANIM_ATTACKER);
    if IsContest() != 0 {
        isBackPic = TRUE;
        personality = (*(*gContestResources).moveAnim).targetPersonality;
        otId = (*(*gContestResources).moveAnim).otId;
        species = (*(*gContestResources).moveAnim).targetSpecies;
        xOffset = 20;
        priority = GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u32;
    } else {
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            isBackPic = FALSE;
            personality = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                MON_DATA_PERSONALITY,
            );
            otId = GetMonData2(
                &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                MON_DATA_OT_ID,
            );
            if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).transformSpecies
                == SPECIES_NONE
            {
                if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
                    species = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                        MON_DATA_SPECIES,
                    ) as u16;
                } else {
                    species = GetMonData2(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                        MON_DATA_SPECIES,
                    ) as u16;
                }
            } else {
                species =
                    (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).transformSpecies;
            }
            xOffset = 20;
            priority = GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u32;
        } else {
            isBackPic = TRUE;
            personality = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                MON_DATA_PERSONALITY,
            );
            otId = GetMonData2(
                &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                MON_DATA_OT_ID,
            );
            if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).transformSpecies
                == SPECIES_NONE
            {
                if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
                    species = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                        MON_DATA_SPECIES,
                    ) as u16;
                } else {
                    species = GetMonData2(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimTarget]],
                        MON_DATA_SPECIES,
                    ) as u16;
                }
            } else {
                species =
                    (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).transformSpecies;
            }
            xOffset = -20;
            priority = GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u32;
        }
    }
    let coord1: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
    let coord2: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    let spriteId: u8 = CreateAdditionalMonSpriteForMoveAnim(
        species,
        isBackPic,
        0,
        coord1 + xOffset,
        coord2,
        5,
        personality,
        otId,
        gBattleAnimTarget as u32,
        TRUE as u32,
    );
    gSprites[spriteId].oam.set_priority(priority as u16);
    gSprites[spriteId].oam.set_objMode(ST_OAM_OBJ_BLEND);
    FillPalette(32767, 0x100 + gSprites[spriteId].oam.paletteNum() * 16, 32);
    gSprites[spriteId].oam.set_priority(priority as u16);
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (16 - task_get(taskId, 1) as u16) << 8 | task_get(taskId, 1) as u16,
    );
    task_set(taskId, 0, spriteId as i16);
    task_set_func(taskId, Some(AnimTask_RolePlaySilhouette_Step1));
}
pub(crate) unsafe fn AnimTask_RolePlaySilhouette_Step1(taskId: u8) {
    if ({
        let t1 = task_get(taskId, 10);
        task_set(taskId, 10, task_get(taskId, 10) + 1);
        t1
    }) > 1
    {
        task_set(taskId, 10, 0);
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - task_get(taskId, 1) as u16) << 8 | task_get(taskId, 1) as u16,
        );
        if task_get(taskId, 1) == 10 {
            task_set(taskId, 10, 256);
            task_set(taskId, 11, 256);
            task_set_func(taskId, Some(AnimTask_RolePlaySilhouette_Step2));
        }
    }
}
pub(crate) unsafe fn AnimTask_RolePlaySilhouette_Step2(taskId: u8) {
    let spriteId: u8 = task_get(taskId, 0) as u8;
    task_set(taskId, 10, task_get(taskId, 10) - 16);
    task_set(taskId, 11, task_get(taskId, 11) + 128);
    gSprites[spriteId]
        .oam
        .set_affineMode(gSprites[spriteId].oam.affineMode() | ST_OAM_AFFINE_DOUBLE_MASK);
    TrySetSpriteRotScale(
        &raw mut gSprites[spriteId],
        TRUE,
        task_get(taskId, 10),
        task_get(taskId, 11),
        0,
    );
    if ({
        task_set(taskId, 12, task_get(taskId, 12) + 1);
        task_get(taskId, 12)
    }) == 9
    {
        ResetSpriteRotScale_PreserveAffine(&raw mut gSprites[spriteId]);
        DestroySpriteAndFreeResources_(&raw mut gSprites[spriteId]);
        task_set_func(taskId, Some(DestroyAnimVisualTaskAndDisableBlend));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_AcidArmor(taskId: u8) {
    let mut battler: u8 = 0;
    let mut bgX: u16 = 0;
    let mut bgY: u16 = 0;
    let mut scanlineParams: ScanlineEffectParams = zeroed();
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    (*task).data[0] = 0;
    (*task).data[1] = 0;
    (*task).data[2] = 0;
    (*task).data[3] = 16;
    (*task).data[4] = 0;
    (*task).data[5] = battler as i16;
    (*task).data[6] = 32;
    (*task).data[7] = 0;
    (*task).data[8] = 24;
    if GetBattlerSide(battler) == B_SIDE_OPPONENT {
        (*task).data[8] *= -1;
    }
    (*task).data[13] = GetBattlerYCoordWithElevation(battler) as i16 - 34;
    if (*task).data[13] < 0 {
        (*task).data[13] = 0;
    }
    (*task).data[14] = (*task).data[13] + 66;
    (*task).data[15] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    if GetBattlerSpriteBGPriorityRank(battler) == 1 {
        scanlineParams.dmaDest = 67108884_usize as *mut u16 as *mut c_void;
        SetGpuReg(REG_OFFSET_BLDCNT, 16194);
        bgX = gBattle_BG1_X;
        bgY = gBattle_BG1_Y;
    } else {
        scanlineParams.dmaDest = 67108888_usize as *mut u16 as *mut c_void;
        SetGpuReg(REG_OFFSET_BLDCNT, 16196);
        bgX = gBattle_BG2_X;
        bgY = gBattle_BG2_Y;
    }
    let mut y: i16 = 0;
    let mut i: i16 = 0;
    while y < 160 {
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i] = bgX;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i] = bgX;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[0][i as i32 + 1] = bgY;
        (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
            .cast::<CArray<CArray<u16, 960>, 2>>()
            .cast_mut())[1][i as i32 + 1] = bgY;
        y += 1;
        i += 2;
    }
    scanlineParams.dmaControl = 0xa6600001;
    scanlineParams.initState = 1;
    scanlineParams.unused9 = 0;
    ScanlineEffect_SetParams(scanlineParams);
    (*task).func = Some(AnimTask_AcidArmor_Step);
}
pub(crate) unsafe fn AnimTask_AcidArmor_Step(taskId: u8) {
    let mut var1: i16 = 0;
    let mut var2: i16 = 0;
    let mut bgX: i16 = 0;
    let mut bgY: i16 = 0;
    let mut offset: i16 = 0;
    let mut var0: i16 = 0;
    let mut i: i16 = 0;
    let mut sineIndex: i16 = 0;
    let mut var3: i16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if GetBattlerSpriteBGPriorityRank((*task).data[5] as u8) == 1 {
        bgX = gBattle_BG1_X as i16;
        bgY = gBattle_BG1_Y as i16;
    } else {
        bgX = gBattle_BG2_X as i16;
        bgY = gBattle_BG2_Y as i16;
    }
    match (*task).data[0] {
        0 => {
            offset = (*task).data[14] * 2;
            var1 = 0;
            var2 = 0;
            i = 0;
            (*task).data[1] = ((*task).data[1] + 2) & 0xFF;
            sineIndex = (*task).data[1];
            (*task).data[9] = div_i32(0x7E0, (*task).data[6] as i32) as i16;
            (*task).data[10] =
                -(div_i32((*task).data[7] as i32 * 2, (*task).data[9] as i32) as i16);
            (*task).data[11] = (*task).data[7];
            var3 = (*task).data[11] >> 5;
            (*task).data[12] = var3;
            var0 = (*task).data[14];
            while var0 > (*task).data[13] {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][offset as i32 + 1] = i as u16 - var2 as u16 + bgY as u16;
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[(*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .srcBuffer][offset] = bgX as u16
                    + var3 as u16
                    + ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[sineIndex]
                        >> 5) as u16;
                sineIndex = (sineIndex + 10) & 0xFF;
                (*task).data[11] += (*task).data[10];
                var3 = (*task).data[11] >> 5;
                (*task).data[12] = var3;
                i += 1;
                offset -= 2;
                var1 += (*task).data[6];
                var2 = var1 >> 5;
                var0 -= 1;
            }
            var0 *= 2;
            while var0 >= 0 {
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[0][var0] = bgX as u16 + DISPLAY_WIDTH;
                (*(&raw const crate::scanline_effect::gScanlineEffectRegBuffers)
                    .cast::<CArray<CArray<u16, 960>, 2>>()
                    .cast_mut())[1][var0] = bgX as u16 + DISPLAY_WIDTH;
                var0 -= 2;
            }
            if ({
                (*task).data[6] += 1;
                (*task).data[6]
            }) > 63
            {
                (*task).data[6] = 64;
                (*task).data[2] += 1;
                if (*task).data[2] as i32 & 1 != 0 {
                    (*task).data[3] -= 1;
                } else {
                    (*task).data[4] += 1;
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*task).data[4] as u16) << 8 | (*task).data[3] as u16,
                );
                if (*task).data[3] == 0 && (*task).data[4] == 16 {
                    (*task).data[2] = 0;
                    (*task).data[3] = 0;
                    (*task).data[0] += 1;
                }
            } else {
                (*task).data[7] += (*task).data[8];
            }
        }
        1 => {
            if ({
                (*task).data[2] += 1;
                (*task).data[2]
            }) > 12
            {
                (*(&raw const crate::scanline_effect::gScanlineEffect)
                    .cast::<ScanlineEffect>()
                    .cast_mut())
                .state = 3;
                (*task).data[2] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            (*task).data[2] += 1;
            if (*task).data[2] as i32 & 1 != 0 {
                (*task).data[3] += 1;
            } else {
                (*task).data[4] -= 1;
            }
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                ((*task).data[4] as u16) << 8 | (*task).data[3] as u16,
            );
            if (*task).data[3] == 16 && (*task).data[4] == 0 {
                (*task).data[2] = 0;
                (*task).data[3] = 0;
                (*task).data[0] += 1;
            }
        }
        3 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DeepInhale(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[15] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    PrepareAffineAnimInTaskData(
        &raw mut (*gTasks.as_ptr())[taskId],
        (*task).data[15] as u8,
        gDeepInhaleAffineAnimCmds.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_DeepInhale_Step);
}
pub(crate) unsafe fn AnimTask_DeepInhale_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let mut var0: u16 = (*task).data[0] as u16;
    (*task).data[0] += 1;
    var0 -= 20;
    if var0 < 23 {
        if ({
            (*task).data[1] += 1;
            (*task).data[1]
        }) > 1
        {
            (*task).data[1] = 0;
            (*task).data[2] += 1;
            if (*task).data[2] as i32 & 1 != 0 {
                gSprites[(*task).data[15]].x2 = 1;
            } else {
                gSprites[(*task).data[15]].x2 = -1;
            }
        }
    } else {
        gSprites[(*task).data[15]].x2 = 0;
    }
    if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
unsafe fn InitYawnCloudPosition(
    sprite: *mut Sprite,
    startX: i16,
    startY: i16,
    destX: i16,
    destY: i16,
    duration: u16,
) {
    (*sprite).x = startX;
    (*sprite).y = startY;
    (*sprite).data[4] = startX << 4;
    (*sprite).data[5] = startY << 4;
    (*sprite).data[6] = div_i32((destX as i32 - startX as i32) << 4, duration as i32) as i16;
    (*sprite).data[7] = div_i32((destY as i32 - startY as i32) << 4, duration as i32) as i16;
}
unsafe fn UpdateYawnCloudPosition(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[6];
    (*sprite).data[5] += (*sprite).data[7];
    (*sprite).x = (*sprite).data[4] >> 4;
    (*sprite).y = (*sprite).data[5] >> 4;
}
pub(crate) unsafe fn AnimYawnCloud(sprite: *mut Sprite) {
    let destX: i16 = (*sprite).x;
    let destY: i16 = (*sprite).y;
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    StartSpriteAffineAnim(sprite, gBattleAnimArgs[0] as u8);
    InitYawnCloudPosition(sprite, (*sprite).x, (*sprite).y, destX, destY, 64);
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(AnimYawnCloud_Step);
}
pub(crate) unsafe fn AnimYawnCloud_Step(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    let index: i32 = ((*sprite).data[0] as i32 * 8) & 0xFF;
    UpdateYawnCloudPosition(sprite);
    (*sprite).y2 = Sin(index as i16, 8);
    if (*sprite).data[0] > 58
        && ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) > 1
    {
        (*sprite).data[1] = 0;
        (*sprite).data[2] += 1;
        (*sprite).set_invisible((*sprite).data[2] as u16 & 1);
        if (*sprite).data[2] > 3 {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe fn AnimSmokeBallEscapeCloud(sprite: *mut Sprite) {
    (*sprite).data[0] = gBattleAnimArgs[3];
    StartSpriteAffineAnim(sprite, gBattleAnimArgs[0] as u8);
    if GetBattlerSide(gBattleAnimTarget) != B_SIDE_PLAYER {
        gBattleAnimArgs[1] = -gBattleAnimArgs[1];
    }
    (*sprite).x =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + gBattleAnimArgs[1];
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + gBattleAnimArgs[2];
    (*sprite).callback = Some(DestroyAnimSpriteAfterTimer);
}
pub(crate) unsafe fn AnimTask_SlideMonForFocusBand_Step2(taskId: u8) {
    task_set(taskId, 0, task_get(taskId, 0) - 1);
    if task_get(taskId, 6) as i32 & 0x8000 != 0
        && ({
            task_set(taskId, 1, task_get(taskId, 1) - 1);
            task_get(taskId, 1)
        }) == -1
    {
        if task_get(taskId, 9) == 0 {
            task_set(taskId, 9, task_get(taskId, 4));
            task_set(taskId, 4, -task_get(taskId, 4));
        } else {
            task_set(taskId, 9, 0);
        }
        if task_get(taskId, 10) == 0 {
            task_set(taskId, 10, task_get(taskId, 5));
            task_set(taskId, 5, -task_get(taskId, 5));
        } else {
            task_set(taskId, 10, 0);
        }
        task_set(taskId, 1, task_get(taskId, 13));
    }
    let var0: u16 = task_get(taskId, 7) as u16;
    let var1: u16 = task_get(taskId, 8) as u16;
    if task_get(taskId, 2) as i32 & 0x8000 != 0 {
        gSprites[task_get(taskId, 15)].x2 = task_get(taskId, 9) - (var0 >> 8) as i16;
    } else {
        gSprites[task_get(taskId, 15)].x2 = task_get(taskId, 9) + (var0 >> 8) as i16;
    }
    if task_get(taskId, 3) as i32 & 0x8000 != 0 {
        gSprites[task_get(taskId, 15)].y2 = task_get(taskId, 10) - (var1 >> 8) as i16;
    } else {
        gSprites[task_get(taskId, 15)].y2 = task_get(taskId, 10) + (var1 >> 8) as i16;
    }
    if task_get(taskId, 0) < 1 {
        DestroyTask(taskId);
        gAnimVisualTaskCount -= 1;
    }
}
pub(crate) unsafe fn AnimTask_SlideMonForFocusBand_Step1(taskId: u8) {
    let mut var0: u16 = 0;
    task_set(taskId, 0, task_get(taskId, 0) - 1);
    if task_get(taskId, 6) as i32 & 0x8000 != 0
        && ({
            task_set(taskId, 1, task_get(taskId, 1) - 1);
            task_get(taskId, 1)
        }) == -1
    {
        if task_get(taskId, 9) == 0 {
            task_set(taskId, 9, task_get(taskId, 4));
            task_set(taskId, 4, -task_get(taskId, 4));
        } else {
            task_set(taskId, 9, var0 as i16);
        }
        if task_get(taskId, 10) == 0 {
            task_set(taskId, 10, task_get(taskId, 5));
            task_set(taskId, 5, -task_get(taskId, 5));
        } else {
            task_set(taskId, 10, 0);
        }
        task_set(taskId, 1, task_get(taskId, 13));
    }
    var0 = (task_get(taskId, 2) as u16 & 0x7FFF) + task_get(taskId, 7) as u16;
    let var1: u16 = (task_get(taskId, 3) as u16 & 0x7FFF) + task_get(taskId, 8) as u16;
    if task_get(taskId, 2) as i32 & 0x8000 != 0 {
        gSprites[task_get(taskId, 15)].x2 = task_get(taskId, 9) - (var0 >> 8) as i16;
    } else {
        gSprites[task_get(taskId, 15)].x2 = task_get(taskId, 9) + (var0 >> 8) as i16;
    }
    if task_get(taskId, 3) as i32 & 0x8000 != 0 {
        gSprites[task_get(taskId, 15)].y2 = task_get(taskId, 10) - (var1 >> 8) as i16;
    } else {
        gSprites[task_get(taskId, 15)].y2 = task_get(taskId, 10) + (var1 >> 8) as i16;
    }
    task_set(taskId, 7, var0 as i16);
    task_set(taskId, 8, var1 as i16);
    if task_get(taskId, 0) < 1 {
        task_set(taskId, 0, 30);
        task_set(taskId, 13, 0);
        task_set_func(taskId, Some(AnimTask_SlideMonForFocusBand_Step2));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SlideMonForFocusBand(taskId: u8) {
    task_set(taskId, 15, gBattlerSpriteIds[gBattleAnimAttacker] as i16);
    task_set(taskId, 14, gBattleAnimArgs[0]);
    task_set(taskId, 0, gBattleAnimArgs[0]);
    task_set(taskId, 13, gBattleAnimArgs[6]);
    if gBattleAnimArgs[3] != 0 {
        task_set(taskId, 6, task_get(taskId, 6) | -32768);
    }
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        task_set(taskId, 2, gBattleAnimArgs[1]);
        task_set(taskId, 3, gBattleAnimArgs[2]);
    } else {
        if gBattleAnimArgs[1] as i32 & 0x8000 != 0 {
            task_set(taskId, 2, gBattleAnimArgs[1] & 0x7FFF);
        } else {
            task_set(taskId, 2, gBattleAnimArgs[1] | -32768);
        }
        if gBattleAnimArgs[2] as i32 & 0x8000 != 0 {
            task_set(taskId, 3, gBattleAnimArgs[2] & 0x7FFF);
        } else {
            task_set(taskId, 3, gBattleAnimArgs[2] | -32768);
        }
    }
    task_set(taskId, 8, 0);
    task_set(taskId, 7, 0);
    task_set(taskId, 4, gBattleAnimArgs[4]);
    task_set(taskId, 5, gBattleAnimArgs[5]);
    task_set_func(taskId, Some(AnimTask_SlideMonForFocusBand_Step1));
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SquishAndSweatDroplets(taskId: u8) {
    let mut battler: u8 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if gBattleAnimArgs[1] == 0 {
        DestroyAnimVisualTask(taskId);
    }
    (*task).data[tState] = 0;
    (*task).data[tTimer] = 0;
    (*task).data[2] = 0;
    (*task).data[tNumSquishes] = gBattleAnimArgs[1];
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    (*task).data[tBaseX] = GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16;
    (*task).data[tBaseY] = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16;
    (*task).data[tSubpriority] = GetBattlerSpriteSubpriority(battler) as i16;
    (*task).data[tBattlerSpriteId] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    PrepareAffineAnimInTaskData(
        task,
        (*task).data[tBattlerSpriteId] as u8,
        gFacadeSquishAffineAnimCmds.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_SquishAndSweatDroplets_Step);
}
pub(crate) unsafe fn AnimTask_SquishAndSweatDroplets_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[tState] {
        0 => {
            (*task).data[tTimer] += 1;
            if (*task).data[tTimer] == 6 {
                CreateSweatDroplets(taskId, TRUE);
            }
            if (*task).data[tTimer] == 18 {
                CreateSweatDroplets(taskId, FALSE);
            }
            if RunAffineAnimFromTaskData(task) == 0 {
                if ({
                    (*task).data[tNumSquishes] -= 1;
                    (*task).data[tNumSquishes]
                }) == 0
                {
                    (*task).data[tState] += 1;
                } else {
                    (*task).data[tTimer] = 0;
                    PrepareAffineAnimInTaskData(
                        task,
                        (*task).data[tBattlerSpriteId] as u8,
                        gFacadeSquishAffineAnimCmds.as_ptr().cast_mut(),
                    );
                }
            }
        }
        1 if (*task).data[2] == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn CreateSweatDroplets(taskId: u8, lowerDroplets: u8) {
    let mut xOffset: i8 = 0;
    let mut yOffset: i8 = 0;
    let mut xCoords: CArray<i16, 4> = zeroed();
    let mut yCoords: CArray<i16, 2> = zeroed();
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if lowerDroplets == 0 {
        xOffset = 18;
        yOffset = -20;
    } else {
        xOffset = 30;
        yOffset = 20;
    }
    xCoords[0] = (*task).data[4] - xOffset as i16;
    xCoords[1] = (*task).data[4] - xOffset as i16 - 4;
    xCoords[2] = (*task).data[4] + xOffset as i16;
    xCoords[3] = (*task).data[4] + xOffset as i16 + 4;
    yCoords[0] = (*task).data[tBaseY] + yOffset as i16;
    yCoords[1] = (*task).data[tBaseY] + yOffset as i16 + 6;
    for i in 0..4u8 {
        let spriteId: u8 = CreateSprite(
            (&raw const *gFacadeSweatDropSpriteTemplate).cast_mut(),
            xCoords[i],
            yCoords[i as i32 & 1],
            (*task).data[tSubpriority] as u8 - 5,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].data[sTimer] = 0;
            gSprites[spriteId].data[sVelocX] = (if i < 2 { -2 } else { 2 }) as i16;
            gSprites[spriteId].data[sVelocY] = -1;
            gSprites[spriteId].data[3] = taskId as i16;
            gSprites[spriteId].data[4] = 2;
            (*task).data[sVelocY] += 1;
        }
    }
}
pub(crate) unsafe fn AnimFacadeSweatDrop(sprite: *mut Sprite) {
    (*sprite).x += (*sprite).data[sVelocX];
    (*sprite).y += (*sprite).data[sVelocY];
    if ({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) > 6
    {
        task_set(
            (*sprite).data[3],
            (*sprite).data[4],
            task_get((*sprite).data[3], (*sprite).data[4]) - 1,
        );
        DestroySprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_FacadeColorBlend(taskId: u8) {
    task_set(taskId, 0, 0);
    task_set(taskId, 1, gBattleAnimArgs[1]);
    let spriteId: u8 = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8);
    task_set(
        taskId,
        2,
        0x100 + gSprites[spriteId].oam.paletteNum() as i16 * 16,
    );
    task_set_func(taskId, Some(AnimTask_FacadeColorBlend_Step));
}
pub(crate) unsafe fn AnimTask_FacadeColorBlend_Step(taskId: u8) {
    if task_get(taskId, 1) != 0 {
        BlendPalette(
            task_get(taskId, 2) as u16,
            16,
            8,
            gFacadeBlendColors[task_get(taskId, 0)],
        );
        if ({
            task_set(taskId, 0, task_get(taskId, 0) + 1);
            task_get(taskId, 0)
        }) > 23
        {
            task_set(taskId, 0, 0);
        }
        task_set(taskId, 1, task_get(taskId, 1) - 1);
    } else {
        BlendPalette(task_get(taskId, 2) as u16, 16, 0, 0);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_StatusClearedEffect(taskId: u8) {
    StartMonScrollingBgMask(
        taskId,
        0,
        0x1A0,
        gBattleAnimAttacker,
        gBattleAnimArgs[0] as u8,
        10,
        2,
        30,
        (*(&raw const crate::data::graphics::gCureBubblesGfx).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::graphics::gCureBubblesTilemap).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
        (*(&raw const crate::data::graphics::gCureBubblesPal).cast::<CArray<u32, 0>>())
            .as_ptr()
            .cast_mut(),
    );
}
pub(crate) unsafe fn AnimRoarNoiseLine(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        gBattleAnimArgs[0] = -gBattleAnimArgs[0];
    }
    (*sprite).x =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + gBattleAnimArgs[0];
    (*sprite).y =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + gBattleAnimArgs[1];
    if gBattleAnimArgs[2] == 0 {
        (*sprite).data[0] = 0x280;
        (*sprite).data[1] = -640;
    } else if gBattleAnimArgs[2] == 1 {
        (*sprite).set_vFlip(1);
        (*sprite).data[0] = 0x280;
        (*sprite).data[1] = 0x280;
    } else {
        StartSpriteAnim(sprite, 1);
        (*sprite).data[0] = 0x280;
    }
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).data[0] = -(*sprite).data[0];
        (*sprite).set_hFlip(1);
    }
    (*sprite).callback = Some(AnimRoarNoiseLine_Step);
}
pub(crate) unsafe fn AnimRoarNoiseLine_Step(sprite: *mut Sprite) {
    (*sprite).data[6] += (*sprite).data[0];
    (*sprite).data[7] += (*sprite).data[1];
    (*sprite).x2 = (*sprite).data[6] >> 8;
    (*sprite).y2 = (*sprite).data[7] >> 8;
    if ({
        (*sprite).data[5] += 1;
        (*sprite).data[5]
    }) == 14
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GlareEyeDots(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if IsContest() != 0 {
        (*task).data[tPairMax] = 8;
        (*task).data[tDotOffset] = 3;
        (*task).data[tIsContest] = TRUE as i16;
    } else {
        (*task).data[tPairMax] = 12;
        (*task).data[tDotOffset] = 3;
        (*task).data[tIsContest] = FALSE as i16;
    }
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*task).data[tStartX] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2)
            as i16
            + GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_HEIGHT) / 4;
    } else {
        (*task).data[tStartX] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2)
            as i16
            - GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_HEIGHT) / 4;
    }
    (*task).data[tStartY] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET)
        as i16
        - GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_HEIGHT) / 4;
    (*task).data[tEndX] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*task).data[tEndY] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*task).func = Some(AnimTask_GlareEyeDots_Step);
}
pub(crate) unsafe fn AnimTask_GlareEyeDots_Step(taskId: u8) {
    let mut x: i16 = 0;
    let mut y: i16 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 3
            {
                (*task).data[1] = 0;
                GetGlareEyeDotCoords(
                    (*task).data[tStartX],
                    (*task).data[tStartY],
                    (*task).data[tEndX],
                    (*task).data[tEndY],
                    (*task).data[tPairMax] as u8,
                    (*task).data[2] as u8,
                    &raw mut x,
                    &raw mut y,
                );
                for i in 0..2u8 {
                    let spriteId: u8 = CreateSprite(
                        (&raw const *gGlareEyeDotSpriteTemplate).cast_mut(),
                        x,
                        y,
                        35,
                    );
                    if spriteId != MAX_SPRITES {
                        if (*task).data[tIsContest] == 0 {
                            if i == 0 {
                                gSprites[spriteId].x2 = {
                                    gSprites[spriteId].y2 = -(*task).data[tDotOffset];
                                    gSprites[spriteId].y2
                                };
                            } else {
                                gSprites[spriteId].x2 = {
                                    gSprites[spriteId].y2 = (*task).data[tDotOffset];
                                    gSprites[spriteId].y2
                                };
                            }
                        } else {
                            if i == 0 {
                                gSprites[spriteId].x2 = -(*task).data[tDotOffset];
                                gSprites[spriteId].y2 = (*task).data[tDotOffset];
                            } else {
                                gSprites[spriteId].x2 = (*task).data[tDotOffset];
                                gSprites[spriteId].y2 = -(*task).data[tDotOffset];
                            }
                        }
                        gSprites[spriteId].data[0] = 0;
                        gSprites[spriteId].data[1] = taskId as i16;
                        gSprites[spriteId].data[2] = IDX_ACTIVE_SPRITES;
                        (*task).data[10] += 1;
                    }
                }
                if (*task).data[2] == (*task).data[tPairMax] {
                    (*task).data[0] += 1;
                }
                (*task).data[2] += 1;
            }
        }
        1 if (*task).data[10] == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
unsafe fn GetGlareEyeDotCoords(
    startX: i16,
    startY: i16,
    endX: i16,
    endY: i16,
    mut pairMax: u8,
    pairNum: u8,
    x: *mut i16,
    y: *mut i16,
) {
    if pairNum == 0 {
        *x = startX;
        *y = startY;
        return;
    }
    if pairNum >= pairMax {
        *x = endX;
        *y = endY;
        return;
    }
    pairMax -= 1;
    let x2: i32 = ((startX as i32) << 8)
        + pairNum as i32 * div_i32((endX as i32 - startX as i32) << 8, pairMax as i32);
    let y2: i32 = ((startY as i32) << 8)
        + pairNum as i32 * div_i32((endY as i32 - startY as i32) << 8, pairMax as i32);
    *x = (x2 >> 8) as i16;
    *y = (y2 >> 8) as i16;
}
pub(crate) unsafe fn AnimGlareEyeDot(sprite: *mut Sprite) {
    if ({
        (*sprite).data[sTimer] += 1;
        (*sprite).data[sTimer]
    }) > 36
    {
        task_set(
            (*sprite).data[1],
            (*sprite).data[2],
            task_get((*sprite).data[1], (*sprite).data[2]) - 1,
        );
        DestroySprite(sprite);
    }
}
pub(crate) unsafe fn AnimAssistPawprint(sprite: *mut Sprite) {
    (*sprite).x = gBattleAnimArgs[0];
    (*sprite).y = gBattleAnimArgs[1];
    (*sprite).data[2] = gBattleAnimArgs[2];
    (*sprite).data[4] = gBattleAnimArgs[3];
    (*sprite).data[0] = gBattleAnimArgs[4];
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(InitAndRunAnimFastLinearTranslation);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_BarrageBall(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[11] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*task).data[12] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*task).data[13] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*task).data[14] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
        + GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_HEIGHT) / 4;
    (*task).data[15] = CreateSprite(
        (&raw const *gBarrageBallSpriteTemplate).cast_mut(),
        (*task).data[11],
        (*task).data[12],
        GetBattlerSpriteSubpriority(gBattleAnimTarget) - 5,
    ) as i16;
    if (*task).data[15] != MAX_SPRITES as i16 {
        gSprites[(*task).data[15]].data[0] = 16;
        gSprites[(*task).data[15]].data[2] = (*task).data[13];
        gSprites[(*task).data[15]].data[4] = (*task).data[14];
        gSprites[(*task).data[15]].data[5] = -32;
        InitAnimArcTranslation(&raw mut gSprites[(*task).data[15]]);
        if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
            StartSpriteAffineAnim(&raw mut gSprites[(*task).data[15]], 1);
        }
        (*task).func = Some(AnimTask_BarrageBall_Step);
    } else {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimTask_BarrageBall_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 1
            {
                (*task).data[1] = 0;
                TranslateAnimHorizontalArc(&raw mut gSprites[(*task).data[15]]);
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) > 7
                {
                    (*task).data[0] += 1;
                }
            }
        }
        1 => {
            if TranslateAnimHorizontalArc(&raw mut gSprites[(*task).data[15]]) != 0 {
                (*task).data[1] = 0;
                (*task).data[2] = 0;
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
                gSprites[(*task).data[15]].set_invisible((*task).data[2] as u16 & 1);
                if (*task).data[2] == 16 {
                    FreeOamMatrix(gSprites[(*task).data[15]].oam.matrixNum() as u8);
                    DestroySprite(&raw mut gSprites[(*task).data[15]]);
                    (*task).data[0] += 1;
                }
            }
        }
        3 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimSmellingSaltsHand(sprite: *mut Sprite) {
    let mut battler: u8 = 0;
    if gBattleAnimArgs[0] == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 16);
    (*sprite).data[6] = gBattleAnimArgs[2];
    (*sprite).data[7] = (if gBattleAnimArgs[1] == 0 { -1 } else { 1 }) as i16;
    (*sprite).y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if gBattleAnimArgs[1] == 0 {
        (*sprite)
            .oam
            .set_matrixNum((*sprite).oam.matrixNum() | ST_OAM_HFLIP);
        (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_LEFT) - 8;
    } else {
        (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_RIGHT) + 8;
    }
    (*sprite).callback = Some(AnimSmellingSaltsHand_Step);
}
pub(crate) unsafe fn AnimSmellingSaltsHand_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 1
            {
                (*sprite).data[1] = 0;
                (*sprite).x2 += (*sprite).data[7];
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
                }) == 12
                {
                    (*sprite).data[0] += 1;
                }
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 8
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).x2 -= (*sprite).data[7] * 4;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 6
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        3 => {
            (*sprite).x2 += (*sprite).data[7] * 3;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 8
            {
                if ({
                    (*sprite).data[6] -= 1;
                    (*sprite).data[6]
                }) != 0
                {
                    (*sprite).data[1] = 0;
                    (*sprite).data[0] -= 1;
                } else {
                    DestroyAnimSprite(sprite);
                }
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SmellingSaltsSquish(taskId: u8) {
    if gBattleAnimArgs[0] == 0 {
        DestroyAnimVisualTask(taskId);
    } else {
        task_set(taskId, 0, gBattleAnimArgs[1]);
        task_set(
            taskId,
            15,
            GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16,
        );
        PrepareAffineAnimInTaskData(
            &raw mut (*gTasks.as_ptr())[taskId],
            task_get(taskId, 15) as u8,
            gSmellingSaltsSquishAffineAnimCmds.as_ptr().cast_mut(),
        );
        task_set_func(taskId, Some(AnimTask_SmellingSaltsSquish_Step));
    }
}
pub(crate) unsafe fn AnimTask_SmellingSaltsSquish_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if ({
        (*task).data[1] += 1;
        (*task).data[1]
    }) > 1
    {
        (*task).data[1] = 0;
        if (*task).data[2] as i32 & 1 == 0 {
            gSprites[(*task).data[15]].x2 = 2;
        } else {
            gSprites[(*task).data[15]].x2 = -2;
        }
    }
    if RunAffineAnimFromTaskData(task) == 0 {
        gSprites[(*task).data[15]].x2 = 0;
        if ({
            (*task).data[0] -= 1;
            (*task).data[0]
        }) != 0
        {
            PrepareAffineAnimInTaskData(
                &raw mut (*gTasks.as_ptr())[taskId],
                task_get(taskId, 15) as u8,
                gSmellingSaltsSquishAffineAnimCmds.as_ptr().cast_mut(),
            );
            (*task).data[1] = 0;
            (*task).data[2] = 0;
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe fn AnimSmellingSaltExclamation(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_TOP);
    } else {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_TOP);
    }
    if (*sprite).y < 8 {
        (*sprite).y = 8;
    }
    (*sprite).data[0] = 0;
    (*sprite).data[1] = gBattleAnimArgs[1];
    (*sprite).data[2] = 0;
    (*sprite).data[3] = gBattleAnimArgs[2];
    (*sprite).callback = Some(AnimSmellingSaltExclamation_Step);
}
pub(crate) unsafe fn AnimSmellingSaltExclamation_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) >= (*sprite).data[1]
    {
        (*sprite).data[0] = 0;
        (*sprite).data[2] = ((*sprite).data[2] + 1) & 1;
        (*sprite).set_invisible((*sprite).data[2] as u16);
        if (*sprite).data[2] != 0
            && ({
                (*sprite).data[3] -= 1;
                (*sprite).data[3]
            }) == 0
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimHelpingHandClap(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        (*sprite)
            .oam
            .set_matrixNum((*sprite).oam.matrixNum() | ST_OAM_HFLIP);
        (*sprite).x = 100;
        (*sprite).data[7] = 1;
    } else {
        (*sprite).x = 140;
        (*sprite).data[7] = -1;
    }
    (*sprite).y = 56;
    (*sprite).callback = Some(AnimHelpingHandClap_Step);
}
pub(crate) unsafe fn AnimHelpingHandClap_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).y -= (*sprite).data[7] * 2;
            if (*sprite).data[1] as i32 & 1 != 0 {
                (*sprite).x -= (*sprite).data[7] * 2;
            }
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 9
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 4
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).data[1] += 1;
            (*sprite).y += (*sprite).data[7] * 3;
            (*sprite).x2 = (*sprite).data[7]
                * ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                    [(*sprite).data[1] as i32 * 10]
                    >> 3);
            if (*sprite).data[1] == 12 {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        3 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 2
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        4 => {
            (*sprite).data[1] += 1;
            (*sprite).y -= (*sprite).data[7] * 3;
            (*sprite).x2 = (*sprite).data[7]
                * ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                    [(*sprite).data[1] as i32 * 10]
                    >> 3);
            if (*sprite).data[1] == 12 {
                (*sprite).data[0] += 1;
            }
        }
        5 => {
            (*sprite).data[1] += 1;
            (*sprite).y += (*sprite).data[7] * 3;
            (*sprite).x2 = (*sprite).data[7]
                * ((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                    [(*sprite).data[1] as i32 * 10]
                    >> 3);
            if (*sprite).data[1] == 15 {
                (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 16);
            }
            if (*sprite).data[1] == 18 {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        6 => {
            (*sprite).x += (*sprite).data[7] * 6;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 9
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        7 => {
            (*sprite).x += (*sprite).data[7] * 2;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 1
            {
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        8 => {
            (*sprite).x -= (*sprite).data[7] * 3;
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) == 5
            {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_HelpingHandAttackerMovement(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[15] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    if IsContest() == 0 {
        if IsDoubleBattle() == TRUE {
            let attackerX: i32 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i32;
            let partnerX: i32 =
                GetBattlerSpriteCoord(gBattleAnimAttacker ^ 2, BATTLER_COORD_X) as i32;
            if attackerX > partnerX {
                (*task).data[14] = 1;
            } else {
                (*task).data[14] = -1;
            }
        } else {
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                (*task).data[14] = -1;
            } else {
                (*task).data[14] = 1;
            }
        }
    } else {
        (*task).data[14] = 1;
    }
    (*task).func = Some(AnimTask_HelpingHandAttackerMovement_Step);
}
pub(crate) unsafe fn AnimTask_HelpingHandAttackerMovement_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 13
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        1 => {
            gSprites[(*task).data[15]].x2 -= (*task).data[14] * 3;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 6
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            gSprites[(*task).data[15]].x2 += (*task).data[14] * 3;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 6
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        3 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 2
            {
                (*task).data[1] = 0;
                if (*task).data[2] == 0 {
                    (*task).data[2] += 1;
                    (*task).data[0] = 1;
                } else {
                    (*task).data[0] += 1;
                }
            }
        }
        4 => {
            gSprites[(*task).data[15]].x2 += (*task).data[14];
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 3
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        5 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 6
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        6 => {
            gSprites[(*task).data[15]].x2 -= (*task).data[14] * 4;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 5
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        7 => {
            gSprites[(*task).data[15]].x2 += (*task).data[14] * 4;
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) == 5
            {
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        8 => {
            gSprites[(*task).data[15]].x2 = 0;
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimForesightMagnifyingGlass(sprite: *mut Sprite) {
    if gBattleAnimArgs[0] == 0 {
        InitSpritePosToAnimAttacker(sprite, TRUE);
        (*sprite).data[7] = gBattleAnimAttacker as i16;
    } else {
        (*sprite).data[7] = gBattleAnimTarget as i16;
    }
    if GetBattlerSide((*sprite).data[7] as u8) == B_SIDE_OPPONENT {
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
    }
    (*sprite)
        .oam
        .set_priority(GetBattlerSpriteBGPriority((*sprite).data[7] as u8) as u16);
    (*sprite).oam.set_objMode(ST_OAM_OBJ_BLEND);
    (*sprite).callback = Some(AnimForesightMagnifyingGlass_Step);
}
pub(crate) unsafe fn AnimForesightMagnifyingGlass_Step(sprite: *mut Sprite) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    match (*sprite).data[5] {
        0 => {
            'l2: {
                let sw1: i16 = (*sprite).data[6];
                let matched = sw1 == 0 || sw1 == 4 || sw1 == 1 || sw1 == 2 || sw1 == 3 || sw1 == 5;
                let mut fall = false;
                if !matched {
                    fall = true;
                    (*sprite).data[6] = 0;
                }
                if fall || sw1 == 0 || sw1 == 4 {
                    x = GetBattlerSpriteCoordAttr((*sprite).data[7] as u8, BATTLER_COORD_ATTR_RIGHT)
                        as u16
                        - 4;
                    y = GetBattlerSpriteCoordAttr(
                        (*sprite).data[7] as u8,
                        BATTLER_COORD_ATTR_BOTTOM,
                    ) as u16
                        - 4;
                    break 'l2;
                }
                if sw1 == 1 {
                    x = GetBattlerSpriteCoordAttr((*sprite).data[7] as u8, BATTLER_COORD_ATTR_RIGHT)
                        as u16
                        - 4;
                    y = GetBattlerSpriteCoordAttr((*sprite).data[7] as u8, BATTLER_COORD_ATTR_TOP)
                        as u16
                        + 4;
                    break 'l2;
                }
                if sw1 == 2 {
                    x = GetBattlerSpriteCoordAttr((*sprite).data[7] as u8, BATTLER_COORD_ATTR_LEFT)
                        as u16
                        + 4;
                    y = GetBattlerSpriteCoordAttr(
                        (*sprite).data[7] as u8,
                        BATTLER_COORD_ATTR_BOTTOM,
                    ) as u16
                        - 4;
                    break 'l2;
                }
                if sw1 == 3 {
                    x = GetBattlerSpriteCoordAttr((*sprite).data[7] as u8, BATTLER_COORD_ATTR_LEFT)
                        as u16
                        + 4;
                    y = GetBattlerSpriteCoordAttr((*sprite).data[7] as u8, BATTLER_COORD_ATTR_TOP)
                        as u16
                        - 4;
                    break 'l2;
                }
                if sw1 == 5 {
                    x = GetBattlerSpriteCoord((*sprite).data[7] as u8, BATTLER_COORD_X_2) as u16;
                    y = GetBattlerSpriteCoord((*sprite).data[7] as u8, BATTLER_COORD_Y_PIC_OFFSET)
                        as u16;
                    break 'l2;
                }
            }
            if (*sprite).data[6] == 4 {
                (*sprite).data[0] = 24;
            } else if (*sprite).data[6] == 5 {
                (*sprite).data[0] = 6;
            } else {
                (*sprite).data[0] = 12;
            }
            (*sprite).data[1] = (*sprite).x;
            (*sprite).data[2] = x as i16;
            (*sprite).data[3] = (*sprite).y;
            (*sprite).data[4] = y as i16;
            InitAnimLinearTranslation(sprite);
            (*sprite).data[5] += 1;
        }
        1 => {
            if AnimTranslateLinear(sprite) != 0 {
                match (*sprite).data[6] {
                    4 => {
                        (*sprite).x += (*sprite).x2;
                        (*sprite).y += (*sprite).y2;
                        (*sprite).y2 = 0;
                        (*sprite).x2 = 0;
                        (*sprite).data[5] = 0;
                        (*sprite).data[6] += 1;
                    }
                    5 => {
                        (*sprite).data[0] = 0;
                        (*sprite).data[1] = 16;
                        (*sprite).data[2] = 0;
                        (*sprite).data[5] = 3;
                    }
                    _ => {
                        (*sprite).x += (*sprite).x2;
                        (*sprite).y += (*sprite).y2;
                        (*sprite).y2 = 0;
                        (*sprite).x2 = 0;
                        (*sprite).data[0] = 0;
                        (*sprite).data[5] += 1;
                        (*sprite).data[6] += 1;
                    }
                }
            }
        }
        2 => {
            if ({
                (*sprite).data[0] += 1;
                (*sprite).data[0]
            }) == 4
            {
                (*sprite).data[5] = 0;
            }
        }
        3 => {
            if (*sprite).data[0] as i32 & 1 == 0 {
                (*sprite).data[1] -= 1;
            } else {
                (*sprite).data[2] += 1;
            }
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                ((*sprite).data[2] as u16) << 8 | (*sprite).data[1] as u16,
            );
            if ({
                (*sprite).data[0] += 1;
                (*sprite).data[0]
            }) == 32
            {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).data[5] += 1;
            }
        }
        4 => {
            DestroyAnimSprite(sprite);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimMeteorMashStar_Step(sprite: *mut Sprite) {
    (*sprite).x2 = div_i32(
        ((*sprite).data[2] as i32 - (*sprite).data[0] as i32) * (*sprite).data[5] as i32,
        (*sprite).data[4] as i32,
    ) as i16;
    (*sprite).y2 = div_i32(
        ((*sprite).data[3] as i32 - (*sprite).data[1] as i32) * (*sprite).data[5] as i32,
        (*sprite).data[4] as i32,
    ) as i16;
    if (*sprite).data[5] as i32 & 1 == 0 {
        CreateSprite(
            (&raw const *gMiniTwinklingStarSpriteTemplate).cast_mut(),
            (*sprite).x + (*sprite).x2,
            (*sprite).y + (*sprite).y2,
            5,
        );
    }
    if (*sprite).data[5] == (*sprite).data[4] {
        DestroyAnimSprite(sprite);
    }
    (*sprite).data[5] += 1;
}
pub(crate) unsafe fn AnimMeteorMashStar(sprite: *mut Sprite) {
    let y: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    let x: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER || IsContest() != 0 {
        (*sprite).data[0] = (*sprite).x - gBattleAnimArgs[0];
        (*sprite).data[2] = (*sprite).x - gBattleAnimArgs[2];
    } else {
        (*sprite).data[0] = (*sprite).x + gBattleAnimArgs[0];
        (*sprite).data[2] = (*sprite).x + gBattleAnimArgs[2];
    }
    (*sprite).data[1] = (*sprite).y + gBattleAnimArgs[1];
    (*sprite).data[3] = (*sprite).y + gBattleAnimArgs[3];
    (*sprite).data[4] = gBattleAnimArgs[4];
    (*sprite).x = (*sprite).data[0];
    (*sprite).y = (*sprite).data[1];
    (*sprite).callback = Some(AnimMeteorMashStar_Step);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MonToSubstitute(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    if task_get(taskId, 0) == 0 {
        PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_NORMAL);
        task_set(taskId, 1, 0x100);
        task_set(taskId, 2, 0x100);
        task_set(taskId, 0, task_get(taskId, 0) + 1);
    } else if task_get(taskId, 0) == 1 {
        task_set(taskId, 1, task_get(taskId, 1) + 0x60);
        task_set(taskId, 2, task_get(taskId, 2) - 0xD);
        SetSpriteRotScale(spriteId, task_get(taskId, 1), task_get(taskId, 2), 0);
        if ({
            task_set(taskId, 3, task_get(taskId, 3) + 1);
            task_get(taskId, 3)
        }) == 9
        {
            task_set(taskId, 3, 0);
            ResetSpriteRotScale(spriteId);
            gSprites[spriteId].set_invisible(TRUE as u16);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
    } else {
        LoadBattleMonGfxAndAnimate(gBattleAnimAttacker, FALSE, spriteId);
        if IsContest() != 0 {
            gSprites[gBattlerSpriteIds[gBattleAnimAttacker]].affineAnims =
                (*(&raw const crate::data::data_tables::gAffineAnims_BattleSpriteContest)
                    .cast::<CArray<*mut AffineAnimCmd, 0>>())
                .as_ptr()
                .cast_mut();
            StartSpriteAffineAnim(
                &raw mut gSprites[gBattlerSpriteIds[gBattleAnimAttacker]],
                BATTLER_AFFINE_NORMAL,
            );
        }
        for i in 0..(NUM_TASK_DATA as i32) {
            task_set(taskId, i, 0);
        }
        task_set_func(taskId, Some(AnimTask_MonToSubstituteDoll));
    }
}
pub(crate) unsafe fn AnimTask_MonToSubstituteDoll(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_ATTACKER);
    match task_get(taskId, 0) {
        0 => {
            gSprites[spriteId].y2 = -200;
            gSprites[spriteId].x2 = 200;
            gSprites[spriteId].set_invisible(FALSE as u16);
            task_set(taskId, 10, 0);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        1 => {
            task_set(taskId, 10, task_get(taskId, 10) + 112);
            gSprites[spriteId].y2 += task_get(taskId, 10) >> 8;
            if gSprites[spriteId].y as i32 + gSprites[spriteId].y2 as i32 >= -32 {
                gSprites[spriteId].x2 = 0;
            }
            if gSprites[spriteId].y2 > 0 {
                gSprites[spriteId].y2 = 0;
            }
            if gSprites[spriteId].y2 == 0 {
                PlaySE12WithPanning(SE_M_BUBBLE2, BattleAnimAdjustPanning(SOUND_PAN_ATTACKER));
                task_set(taskId, 10, task_get(taskId, 10) - 0x800);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        2 => {
            task_set(taskId, 10, task_get(taskId, 10) - 112);
            if task_get(taskId, 10) < 0 {
                task_set(taskId, 10, 0);
            }
            gSprites[spriteId].y2 -= task_get(taskId, 10) >> 8;
            if task_get(taskId, 10) == 0 {
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        3 => {
            task_set(taskId, 10, task_get(taskId, 10) + 112);
            gSprites[spriteId].y2 += task_get(taskId, 10) >> 8;
            if gSprites[spriteId].y2 > 0 {
                gSprites[spriteId].y2 = 0;
            }
            if gSprites[spriteId].y2 == 0 {
                PlaySE12WithPanning(SE_M_BUBBLE2, BattleAnimAdjustPanning(SOUND_PAN_ATTACKER));
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimBlockX(sprite: *mut Sprite) {
    let mut y: i16 = 0;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) - 2;
        y = -144;
    } else {
        (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) + 2;
        y = -96;
    }
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).y2 = y;
    (*sprite).callback = Some(AnimBlockX_Step);
}
pub(crate) unsafe fn AnimBlockX_Step(sprite: *mut Sprite) {
    match (*sprite).data[0] {
        0 => {
            (*sprite).y2 += 10;
            if (*sprite).y2 >= 0 {
                PlaySE12WithPanning(SE_M_SKETCH, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
                (*sprite).y2 = 0;
                (*sprite).data[0] += 1;
            }
        }
        1 => {
            (*sprite).data[1] += 4;
            (*sprite).y2 = -((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*sprite).data[1]]
                >> 3);
            if (*sprite).data[1] > 0x7F {
                PlaySE12WithPanning(SE_M_SKETCH, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
                (*sprite).data[1] = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] += 1;
            }
        }
        2 => {
            (*sprite).data[1] += 6;
            (*sprite).y2 = -((*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
                [(*sprite).data[1]]
                >> 4);
            if (*sprite).data[1] > 0x7F {
                (*sprite).data[1] = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] += 1;
            }
        }
        3 => {
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 8
            {
                PlaySE12WithPanning(SE_M_LEER, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
                (*sprite).data[1] = 0;
                (*sprite).data[0] += 1;
            }
        }
        4 if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) > 8 =>
        {
            (*sprite).data[1] = 0;
            (*sprite).data[2] += 1;
            (*sprite).set_invisible((*sprite).data[2] as u16 & 1);
            if (*sprite).data[2] == 7 {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_OdorSleuthMovement(taskId: u8) {
    if IsContest() != 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    let spriteId1: i16 = CloneBattlerSpriteWithBlend(ANIM_TARGET);
    if spriteId1 < 0 {
        DestroyAnimVisualTask(taskId);
        return;
    }
    let spriteId2: i16 = CloneBattlerSpriteWithBlend(ANIM_TARGET);
    if spriteId2 < 0 {
        DestroySpriteWithActiveSheet(&raw mut gSprites[spriteId1]);
        DestroyAnimVisualTask(taskId);
        return;
    }
    gSprites[spriteId2].x2 += 24;
    gSprites[spriteId1].x2 -= 24;
    gSprites[spriteId2].data[0] = 0;
    gSprites[spriteId1].data[0] = 0;
    gSprites[spriteId2].data[1] = 0;
    gSprites[spriteId1].data[1] = 0;
    gSprites[spriteId2].data[2] = 0;
    gSprites[spriteId1].data[2] = 0;
    gSprites[spriteId2].data[3] = 16;
    gSprites[spriteId1].data[3] = -16;
    gSprites[spriteId2].data[4] = 0;
    gSprites[spriteId1].data[4] = 128;
    gSprites[spriteId2].data[5] = 24;
    gSprites[spriteId1].data[5] = 24;
    gSprites[spriteId2].data[6] = taskId as i16;
    gSprites[spriteId1].data[6] = taskId as i16;
    gSprites[spriteId2].data[7] = 0;
    gSprites[spriteId1].data[7] = 0;
    task_set(taskId, 0, 2);
    if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).invisible() == 0 {
        gSprites[spriteId2].set_invisible(FALSE as u16);
        gSprites[spriteId1].set_invisible(TRUE as u16);
    } else {
        gSprites[spriteId2].set_invisible(TRUE as u16);
        gSprites[spriteId1].set_invisible(TRUE as u16);
    }
    gSprites[spriteId2]
        .oam
        .set_objMode(ST_OAM_OBJ_NORMAL as u32);
    gSprites[spriteId1]
        .oam
        .set_objMode(ST_OAM_OBJ_NORMAL as u32);
    gSprites[spriteId2].callback = Some(MoveOdorSleuthClone);
    gSprites[spriteId1].callback = Some(MoveOdorSleuthClone);
    task_set_func(taskId, Some(AnimTask_OdorSleuthMovementWaitFinish));
}
pub(crate) unsafe fn AnimTask_OdorSleuthMovementWaitFinish(taskId: u8) {
    if task_get(taskId, 0) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn MoveOdorSleuthClone(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        (*sprite).data[1] = 0;
        if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimTarget)).invisible() == 0 {
            (*sprite).set_invisible((*sprite).invisible() ^ 1);
        }
    }
    (*sprite).data[4] += (*sprite).data[3];
    (*sprite).data[4] &= 0xFF;
    (*sprite).x2 = Cos((*sprite).data[4], (*sprite).data[5]);
    match (*sprite).data[0] {
        0 => {
            if ({
                (*sprite).data[2] += 1;
                (*sprite).data[2]
            }) == 60
            {
                (*sprite).data[2] = 0;
                (*sprite).data[0] += 1;
            }
        }
        1 if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) > 0 =>
        {
            (*sprite).data[2] = 0;
            (*sprite).data[5] -= 2;
            if (*sprite).data[5] < 0 {
                task_set(
                    (*sprite).data[6],
                    (*sprite).data[7],
                    task_get((*sprite).data[6], (*sprite).data[7]) - 1,
                );
                DestroySpriteWithActiveSheet(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetReturnPowerLevel(taskId: u8) {
    gBattleAnimArgs[7] = 0;
    if gAnimFriendship < 60 {
        gBattleAnimArgs[7] = 0;
    }
    if gAnimFriendship > 60 && gAnimFriendship < 92 {
        gBattleAnimArgs[7] = 1;
    }
    if gAnimFriendship > 91 && gAnimFriendship < 201 {
        gBattleAnimArgs[7] = 2;
    }
    if gAnimFriendship > 200 {
        gBattleAnimArgs[7] = 3;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SnatchOpposingMonMove(taskId: u8) {
    let mut spriteId: u8 = 0;
    let mut spriteId2: u8 = 0;
    let mut personality: i32 = 0;
    let mut otId: i32 = 0;
    let mut species: u16 = 0;
    let mut subpriority: u8 = 0;
    let mut isBackPic: u8 = 0;
    let mut x: i16 = 0;
    match task_get(taskId, 0) {
        0 => {
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
            task_set(taskId, 1, task_get(taskId, 1) + 0x800);
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                gSprites[spriteId].x2 += task_get(taskId, 1) >> 8;
            } else {
                gSprites[spriteId].x2 -= task_get(taskId, 1) >> 8;
            }
            task_set(taskId, 1, task_get(taskId, 1) & 0xFF);
            x = gSprites[spriteId].x + gSprites[spriteId].x2;
            if !(-32..=272).contains(&x) {
                task_set(taskId, 1, 0);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        1 => {
            if IsContest() != 0 {
                personality = (*(*gContestResources).moveAnim).personality as i32;
                otId = (*(*gContestResources).moveAnim).otId as i32;
                species = (*(*gContestResources).moveAnim).species;
                subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker);
                isBackPic = FALSE;
                x = -32;
            } else {
                if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                    personality = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                        MON_DATA_PERSONALITY,
                    ) as i32;
                    otId = GetMonData2(
                        &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                        MON_DATA_OT_ID,
                    ) as i32;
                    if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker))
                        .transformSpecies
                        == SPECIES_NONE
                    {
                        species = GetMonData2(
                            &raw mut gPlayerParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                            MON_DATA_SPECIES,
                        ) as u16;
                    } else {
                        species = (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker))
                            .transformSpecies;
                    }
                    subpriority = gSprites[GetAnimBattlerSpriteId(1)].subpriority + 1;
                    isBackPic = FALSE;
                    x = 272;
                } else {
                    personality = GetMonData2(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                        MON_DATA_PERSONALITY,
                    ) as i32;
                    otId = GetMonData2(
                        &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                        MON_DATA_OT_ID,
                    ) as i32;
                    if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker))
                        .transformSpecies
                        == SPECIES_NONE
                    {
                        species = GetMonData2(
                            &raw mut gEnemyParty[gBattlerPartyIndexes[gBattleAnimAttacker]],
                            MON_DATA_SPECIES,
                        ) as u16;
                    } else {
                        species = (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker))
                            .transformSpecies;
                    }
                    subpriority = gSprites[GetAnimBattlerSpriteId(1)].subpriority - 1;
                    isBackPic = TRUE;
                    x = -32;
                }
            }
            spriteId2 = CreateAdditionalMonSpriteForMoveAnim(
                species,
                isBackPic,
                0,
                x,
                GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16,
                subpriority,
                personality as u32,
                otId as u32,
                gBattleAnimAttacker as u32,
                0,
            );
            if (*(*gBattleSpritesDataPtr).battlerData.at(gBattleAnimAttacker)).transformSpecies
                != SPECIES_NONE
            {
                BlendPalette(
                    0x100 + gSprites[spriteId2].oam.paletteNum() * 16,
                    16,
                    6,
                    32767,
                );
            }
            task_set(taskId, 15, spriteId2 as i16);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        2 => {
            spriteId2 = task_get(taskId, 15) as u8;
            task_set(taskId, 1, task_get(taskId, 1) + 0x800);
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                gSprites[spriteId2].x2 -= task_get(taskId, 1) >> 8;
            } else {
                gSprites[spriteId2].x2 += task_get(taskId, 1) >> 8;
            }
            task_set(taskId, 1, task_get(taskId, 1) & 0xFF);
            x = gSprites[spriteId2].x + gSprites[spriteId2].x2;
            if task_get(taskId, 14) == 0 {
                if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                    if x < GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 {
                        task_set(taskId, 14, task_get(taskId, 14) + 1);
                        gBattleAnimArgs[7] = -1;
                    }
                } else {
                    if x > GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 {
                        task_set(taskId, 14, task_get(taskId, 14) + 1);
                        gBattleAnimArgs[7] = -1;
                    }
                }
            }
            if !(-32..=272).contains(&x) {
                task_set(taskId, 1, 0);
                task_set(taskId, 0, task_get(taskId, 0) + 1);
            }
        }
        3 => {
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
            spriteId2 = task_get(taskId, 15) as u8;
            DestroySpriteAndFreeResources_(&raw mut gSprites[spriteId2]);
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                gSprites[spriteId].x2 = -gSprites[spriteId].x - 32;
            } else {
                gSprites[spriteId].x2 = 272 - gSprites[spriteId].x;
            }
            task_set(taskId, 0, task_get(taskId, 0) + 1);
        }
        4 => {
            spriteId = GetAnimBattlerSpriteId(ANIM_ATTACKER);
            task_set(taskId, 1, task_get(taskId, 1) + 0x800);
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                gSprites[spriteId].x2 += task_get(taskId, 1) >> 8;
                if gSprites[spriteId].x2 as i32 + gSprites[spriteId].x as i32
                    >= GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i32
                {
                    gSprites[spriteId].x2 = 0;
                }
            } else {
                gSprites[spriteId].x2 -= task_get(taskId, 1) >> 8;
                if gSprites[spriteId].x2 as i32 + gSprites[spriteId].x as i32
                    <= GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i32
                {
                    gSprites[spriteId].x2 = 0;
                }
            }
            task_set(taskId, 1, task_get(taskId, 1) & 0xFF);
            if gSprites[spriteId].x2 == 0 {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimUnusedItemBagSteal(sprite: *mut Sprite) {
    match (*sprite).data[7] {
        0 => {
            if gBattleAnimArgs[7] == -1 {
                PlaySE12WithPanning(SE_M_VITAL_THROW, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
                (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + 16;
                (*sprite).data[0] = -32;
                (*sprite).data[7] += 1;
                (*sprite).set_invisible(FALSE as u16);
                if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT && IsContest() == 0 {
                    (*sprite).subpriority = gSprites[GetAnimBattlerSpriteId(1)].subpriority - 1;
                }
            } else {
                (*sprite).set_invisible(TRUE as u16);
            }
        }
        1 => {
            (*sprite).y2 = Sin((*sprite).data[1], (*sprite).data[0]);
            (*sprite).data[1] += 5;
            if (*sprite).data[1] > 0x7F {
                (*sprite).data[0] /= 2;
                (*sprite).data[3] += 1;
                (*sprite).data[1] -= 0x7F;
            }
            (*sprite).data[2] += 0x100;
            if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
                (*sprite).x2 -= (*sprite).data[2] >> 8;
            } else {
                (*sprite).x2 += (*sprite).data[2] >> 8;
            }
            (*sprite).data[2] &= 0xFF;
            if (*sprite).data[3] == 2 {
                DestroyAnimSprite(sprite);
            }
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SnatchPartnerMove(taskId: u8) {
    let mut attackerX: i16 = 0;
    let mut targetX: i16 = 0;
    let mut spriteId: u8 = 0;
    match task_get(taskId, 15) {
        0 => {
            attackerX = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
            targetX = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
            task_set(taskId, 0, 6);
            if attackerX > targetX {
                task_set(taskId, 0, -task_get(taskId, 0));
            }
            task_set(taskId, 1, attackerX);
            task_set(taskId, 2, targetX);
            task_set(taskId, 15, task_get(taskId, 15) + 1);
        }
        1 => {
            spriteId = gBattlerSpriteIds[gBattleAnimAttacker];
            gSprites[spriteId].x2 += task_get(taskId, 0);
            if task_get(taskId, 0) > 0 {
                if gSprites[spriteId].x as i32 + gSprites[spriteId].x2 as i32
                    >= task_get(taskId, 2) as i32
                {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            } else {
                if gSprites[spriteId].x as i32 + gSprites[spriteId].x2 as i32
                    <= task_get(taskId, 2) as i32
                {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            }
        }
        2 => {
            task_set(taskId, 0, -task_get(taskId, 0));
            task_set(taskId, 15, task_get(taskId, 15) + 1);
        }
        3 => {
            spriteId = gBattlerSpriteIds[gBattleAnimAttacker];
            gSprites[spriteId].x2 += task_get(taskId, 0);
            if task_get(taskId, 0) < 0 {
                if gSprites[spriteId].x as i32 + gSprites[spriteId].x2 as i32
                    <= task_get(taskId, 1) as i32
                {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            } else {
                if gSprites[spriteId].x as i32 + gSprites[spriteId].x2 as i32
                    >= task_get(taskId, 1) as i32
                {
                    task_set(taskId, 15, task_get(taskId, 15) + 1);
                }
            }
        }
        _ => {
            spriteId = gBattlerSpriteIds[gBattleAnimAttacker];
            gSprites[spriteId].x2 = 0;
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_TeeterDanceMovement(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[3] = GetAnimBattlerSpriteId(ANIM_ATTACKER) as i16;
    (*task).data[4] = (if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        1
    } else {
        -1
    }) as i16;
    (*task).data[6] = gSprites[(*task).data[3]].y;
    (*task).data[5] = gSprites[(*task).data[3]].x;
    (*task).data[9] = 0;
    (*task).data[11] = 0;
    (*task).data[10] = 1;
    (*task).data[12] = 0;
    (*task).func = Some(AnimTask_TeeterDanceMovement_Step);
}
pub(crate) unsafe fn AnimTask_TeeterDanceMovement_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[11] += 8;
            (*task).data[11] &= 0xFF;
            gSprites[(*task).data[3]].x2 = (*(&raw const crate::trig::gSineTable)
                .cast::<CArray<i16, 0>>())[(*task).data[11]]
                >> 5;
            (*task).data[9] += 2;
            (*task).data[9] &= 0xFF;
            gSprites[(*task).data[3]].x = ((*(&raw const crate::trig::gSineTable)
                .cast::<CArray<i16, 0>>())[(*task).data[9]]
                >> 3)
                * (*task).data[4]
                + (*task).data[5];
            if (*task).data[9] == 0 {
                gSprites[(*task).data[3]].x = (*task).data[5];
                (*task).data[0] += 1;
            }
        }
        1 => {
            (*task).data[11] += 8;
            (*task).data[11] &= 0xFF;
            gSprites[(*task).data[3]].x2 = (*(&raw const crate::trig::gSineTable)
                .cast::<CArray<i16, 0>>())[(*task).data[11]]
                >> 5;
            if (*task).data[11] == 0 {
                gSprites[(*task).data[3]].x2 = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimKnockOffStrike_Step(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).data[1] += (*sprite).data[0];
        (*sprite).data[1] &= 0xFF;
    } else {
        (*sprite).data[1] += (*sprite).data[0];
        (*sprite).data[1] &= 0xFF;
    }
    (*sprite).x2 = Cos((*sprite).data[1], 20);
    (*sprite).y2 = Sin((*sprite).data[1], 20);
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
    (*sprite).data[2] += 1;
}
pub(crate) unsafe fn AnimKnockOffStrike(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).x -= gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
        (*sprite).data[0] = -11;
        (*sprite).data[1] = 192;
        StartSpriteAffineAnim(sprite, 1);
    } else {
        (*sprite).data[0] = 11;
        (*sprite).data[1] = 192;
        (*sprite).x += gBattleAnimArgs[0];
        (*sprite).y += gBattleAnimArgs[1];
    }
    (*sprite).callback = Some(AnimKnockOffStrike_Step);
}
pub(crate) unsafe fn AnimRecycle(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoordAttr(gBattleAnimAttacker, BATTLER_COORD_ATTR_TOP);
    if (*sprite).y < 16 {
        (*sprite).y = 16;
    }
    (*sprite).data[6] = 0;
    (*sprite).data[7] = 16;
    (*sprite).callback = Some(AnimRecycle_Step);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        ((*sprite).data[7] as u16) << 8 | (*sprite).data[6] as u16,
    );
}
pub(crate) unsafe fn AnimRecycle_Step(sprite: *mut Sprite) {
    match (*sprite).data[2] {
        0 => {
            if ({
                (*sprite).data[0] += 1;
                (*sprite).data[0]
            }) > 1
            {
                (*sprite).data[0] = 0;
                if (*sprite).data[1] as i32 & 1 == 0 {
                    if (*sprite).data[6] < 16 {
                        (*sprite).data[6] += 1;
                    }
                } else {
                    if (*sprite).data[7] != 0 {
                        (*sprite).data[7] -= 1;
                    }
                }
                (*sprite).data[1] += 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*sprite).data[7] as u16) << 8 | (*sprite).data[6] as u16,
                );
                if (*sprite).data[7] == 0 {
                    (*sprite).data[2] += 1;
                }
            }
        }
        1 => {
            if ({
                (*sprite).data[0] += 1;
                (*sprite).data[0]
            }) == 10
            {
                (*sprite).data[0] = 0;
                (*sprite).data[1] = 0;
                (*sprite).data[2] += 1;
            }
        }
        2 => {
            if ({
                (*sprite).data[0] += 1;
                (*sprite).data[0]
            }) > 1
            {
                (*sprite).data[0] = 0;
                if (*sprite).data[1] as i32 & 1 == 0 {
                    if (*sprite).data[6] != 0 {
                        (*sprite).data[6] -= 1;
                    }
                } else {
                    if (*sprite).data[7] < 16 {
                        (*sprite).data[7] += 1;
                    }
                }
                (*sprite).data[1] += 1;
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*sprite).data[7] as u16) << 8 | (*sprite).data[6] as u16,
                );
                if (*sprite).data[7] == 16 {
                    (*sprite).data[2] += 1;
                }
            }
        }
        3 => {
            DestroySpriteAndMatrix(sprite);
        }
        _ => {}
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_GetWeather(taskId: u8) {
    gBattleAnimArgs[7] = ANIM_WEATHER_NONE;
    if gWeatherMoveAnim as i32 & B_WEATHER_SUN != 0 {
        gBattleAnimArgs[7] = ANIM_WEATHER_SUN;
    } else if gWeatherMoveAnim as i32 & B_WEATHER_RAIN != 0 {
        gBattleAnimArgs[7] = ANIM_WEATHER_RAIN;
    } else if gWeatherMoveAnim as i32 & B_WEATHER_SANDSTORM != 0 {
        gBattleAnimArgs[7] = ANIM_WEATHER_SANDSTORM;
    } else if gWeatherMoveAnim as i32 & B_WEATHER_HAIL != 0 {
        gBattleAnimArgs[7] = ANIM_WEATHER_HAIL;
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SlackOffSquish(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = 0;
    (*task).data[15] = GetAnimBattlerSpriteId(gBattleAnimArgs[0] as u8) as i16;
    PrepareAffineAnimInTaskData(
        task,
        (*task).data[15] as u8,
        gSlackOffSquishAffineAnimCmds.as_ptr().cast_mut(),
    );
    (*task).func = Some(AnimTask_SlackOffSquish_Step);
}
pub(crate) unsafe fn AnimTask_SlackOffSquish_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    if task_get(taskId, 0) > 16 && task_get(taskId, 0) < 40 {
        if ({
            (*task).data[1] += 1;
            (*task).data[1]
        }) > 2
        {
            (*task).data[1] = 0;
            (*task).data[2] += 1;
            if (*task).data[2] as i32 & 1 == 0 {
                gSprites[(*task).data[15]].x2 = -1;
            } else {
                gSprites[(*task).data[15]].x2 = 1;
            }
        }
    } else {
        gSprites[(*task).data[15]].x2 = 0;
    }
    if RunAffineAnimFromTaskData(&raw mut (*gTasks.as_ptr())[taskId]) == 0 {
        DestroyAnimVisualTask(taskId);
    }
}
