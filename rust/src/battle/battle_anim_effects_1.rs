//! Translated from `src/battle_anim_effects_1.c` by tools/rustport/c2rs.py.
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
    clippy::manual_clamp,
    dead_code,
    unused_assignments,
    unused_variables
)]

use crate::battle_anim::gBattleAnimArgs;
use crate::battle_anim::{
    BattleAnimAdjustPanning, DestroyAnimSprite, DestroyAnimVisualTask, IsBattlerSpriteVisible,
    IsContest, gBattleAnimAttacker, gBattleAnimTarget,
};
use crate::battle_anim_mons::{
    AnimFastTranslateLinear, AnimTranslateLinear, ArcTan2Neg, CloneBattlerSpriteWithBlend,
    DestroyAnimSpriteAndDisableBlend, DestroySpriteAndMatrix, DestroySpriteWithActiveSheet,
    GetAnimBattlerSpriteId, GetBattleMonSpritePalettesMask, GetBattlePalettesMask, GetBattlerSide,
    GetBattlerSpriteBGPriority, GetBattlerSpriteBGPriorityRank, GetBattlerSpriteCoord,
    GetBattlerSpriteCoord2, GetBattlerSpriteCoordAttr, GetBattlerSpriteSubpriority,
    InitAndRunAnimFastLinearTranslation, InitAnimArcTranslation,
    InitAnimFastLinearTranslationWithSpeed, InitAnimLinearTranslation, InitSpritePosToAnimAttacker,
    InitSpritePosToAnimTarget, IsDoubleBattle, LoadPointerFromVars,
    PrepareBattlerSpriteForRotScale, ResetSpriteRotScale, RunStoredCallbackWhenAffineAnimEnds,
    RunStoredCallbackWhenAnimEnds, SetAnimSpriteInitialXOffset, SetAverageBattlerPositions,
    SetBattlerSpriteYOffsetFromRotation, SetBattlerSpriteYOffsetFromYScale,
    SetSpriteCoordsToAnimAttackerCoords, SetSpriteRotScale, StartAnimLinearTranslation,
    StorePointerInVars, StoreSpriteCallbackInData6, TranslateAnimHorizontalArc,
    TranslateSpriteLinear, TranslateSpriteLinearAndFlicker, TranslateSpriteLinearById,
    TranslateSpriteLinearFixedPoint, TrySetSpriteRotScale, WaitAnimForDuration,
};
use crate::battle_anim_utility_funcs::SetAnimBgAttribute;
use crate::battle_interface::{SetHealthboxSpriteInvisible, SetHealthboxSpriteVisible};
use crate::battle_main::gBattlersCount;
use crate::battle_main::{gBattlerSpriteIds, gHealthboxSpriteIds};
#[allow(unused_imports)]
use crate::c::*;
#[allow(unused_imports)]
use crate::consts::*;
use crate::gpu_regs::{ClearGpuRegBits, SetGpuReg, SetGpuRegBits};
use crate::palette::{BeginNormalPaletteFade, BlendPalettes, LoadPalette, gPaletteFade};
use crate::palette::{gPlttBufferFaded, gPlttBufferUnfaded};
use crate::random::Random2;
use crate::sound::PlaySE12WithPanning;
use crate::sprite::gSprites;
use crate::sprite::{AllocSpritePalette, FreeSpritePaletteByTag, IndexOfSpritePaletteTag};
use crate::task::{gTasks, task_data_ptr, task_func, task_get, task_set, task_set_func};
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
const sMoveTimer: usize = 0;
const sBlendTableIdx: usize = 1;
const sBlendTimer: usize = 2;
const sBlendCycleTime: usize = 3;
const sX: usize = 4;
const sY: usize = 5;
const sVelocX: usize = 6;
const sVelocY: usize = 7;
// Data tables (translate with cdata.py): gPowderParticlesAnimCmds gPowderParticlesAnimTable gSleepPowderParticleSpriteTemplate gStunSporeParticleSpriteTemplate gPoisonPowderParticleSpriteTemplate gSolarBeamBigOrbAnimCmds1 gSolarBeamBigOrbAnimCmds2 gSolarBeamBigOrbAnimCmds3 gSolarBeamBigOrbAnimCmds4 gSolarBeamBigOrbAnimCmds5 gSolarBeamBigOrbAnimCmds6 gSolarBeamBigOrbAnimCmds7 gSolarBeamSmallOrbAnimCms gPowerAbsorptionOrbAnimCmds gSolarBeamBigOrbAnimTable gSolarBeamSmallOrbAnimTable gPowerAbsorptionOrbAnimTable gPowerAbsorptionOrbAffineAnimCmds gPowerAbsorptionOrbAffineAnimTable gPowerAbsorptionOrbSpriteTemplate gSolarBeamBigOrbSpriteTemplate gSolarBeamSmallOrbSpriteTemplate gStockpileAbsorptionOrbAffineCmds gStockpileAbsorptionOrbAffineAnimTable gStockpileAbsorptionOrbSpriteTemplate gAbsorptionOrbAffineAnimCmds gAbsorptionOrbAffineAnimTable gAbsorptionOrbSpriteTemplate gHyperBeamOrbSpriteTemplate gLeechSeedAnimCmds1 gLeechSeedAnimCmds2 gLeechSeedAnimTable gLeechSeedSpriteTemplate gSporeParticleAnimCmds1 gSporeParticleAnimCmds2 gSporeParticleAnimTable gSporeParticleSpriteTemplate gPetalDanceBigFlowerAnimCmds gPetalDanceSmallFlowerAnimCmds gPetalDanceBigFlowerAnimTable gPetalDanceSmallFlowerAnimTable gPetalDanceBigFlowerSpriteTemplate gPetalDanceSmallFlowerSpriteTemplate gRazorLeafParticleAnimCmds1 gRazorLeafParticleAnimCmds2 gRazorLeafParticleAnimTable gRazorLeafParticleSpriteTemplate gTwisterLeafSpriteTemplate gRazorLeafCutterAnimCmds gRazorLeafCutterAnimTable gRazorLeafCutterSpriteTemplate gSwiftStarAffineAnimCmds gSwiftStarAffineAnimTable gSwiftStarSpriteTemplate sAnim_ConstrictBinding sAnim_ConstrictBinding_Flipped sAnims_ConstrictBinding sAffineAnim_ConstrictBinding sAffineAnim_ConstrictBinding_Flipped sAffineAnims_ConstrictBinding gConstrictBindingSpriteTemplate gMimicOrbAffineAnimCmds1 gMimicOrbAffineAnimCmds2 gMimicOrbAffineAnimTable gMimicOrbSpriteTemplate gIngrainRootAnimCmds1 gIngrainRootAnimCmds2 gIngrainRootAnimCmds3 gIngrainRootAnimCmds4 gIngrainRootAnimTable gIngrainRootSpriteTemplate gFrenzyPlantRootSpriteTemplate gIngrainOrbAnimCmds gIngrainOrbAnimTable gIngrainOrbSpriteTemplate gFallingBagAnimCmds gFallingBagAnimTable gFallingBagAffineAnimCmds1 gFallingBagAffineAnimCmds2 gFallingBagAffineAnimTable gPresentSpriteTemplate gKnockOffItemSpriteTemplate gPresentHealParticleAnimCmds gPresentHealParticleAnimTable gPresentHealParticleSpriteTemplate gItemStealSpriteTemplate gTrickBagAffineAnimCmds1 gTrickBagAffineAnimCmds2 gTrickBagAffineAnimTable gTrickBagSpriteTemplate gTrickBagCoordinates gLeafBladeAnimCmds1 gLeafBladeAnimCmds2 gLeafBladeAnimCmds3 gLeafBladeAnimCmds4 gLeafBladeAnimCmds5 gLeafBladeAnimCmds6 gLeafBladeAnimCmds7 gLeafBladeAnimTable gLeafBladeSpriteTemplate gAromatherapyBigFlowerAffineAnimCmds gAromatherapyBigFlowerAffineAnimTable gAromatherapySmallFlowerSpriteTemplate gAromatherapyBigFlowerSpriteTemplate gSilverWindBigSparkAffineAnimCmds gSilverWindMediumSparkAffineAnimCmds gSilverWindSmallSparkAffineAnimCmds gSilverWindBigSparkAffineAnimTable gSilverWindMediumSparkAffineAnimTable gSilverWindSmallSparkAffineAnimTable gSilverWindBigSparkSpriteTemplate gSilverWindMediumSparkSpriteTemplate gSilverWindSmallSparkSpriteTemplate gMagicalLeafBlendColors gNeedleArmSpikeSpriteTemplate sAnim_Whip sAnim_Whip_Flipped sAnims_Whip gSlamHitSpriteTemplate gVineWhipSpriteTemplate sAnim_SlidingHit sAnims_SlidingHit sSlidingHit1SpriteTemplate sSlidingHit2SpriteTemplate sAffineAnim_FlickeringPunch_Normal sAffineAnim_FlickeringPunch_TurnedTopLeft sAffineAnim_FlickeringPunch_TurnedLeft sAffineAnim_FlickeringPunch_TurnedBottomLeft sAffineAnim_FlickeringPunch_UpsideDown sAffineAnim_FlickeringPunch_TurnedBottomRight sAffineAnim_FlickeringPunch_TurnedRight sAffineAnim_FlickeringPunch_TurnedTopRight sAffineAnims_FlickeringPunch sFlickeringPunchSpriteTemplate gCuttingSliceAnimCmds gCuttingSliceAnimTable gCuttingSliceSpriteTemplate gAirCutterSliceSpriteTemplate sAnim_CirclingMusicNote_Eighth sAnim_CirclingMusicNote_BeamedEighth sAnim_CirclingMusicNote_SlantedBeamedEighth sAnim_CirclingMusicNote_Quarter sAnim_CirclingMusicNote_QuarterRest sAnim_CirclingMusicNote_EighthRest sAnim_CirclingMusicNote_Eighth_Flipped sAnim_CirclingMusicNote_BeamedEighth_Flipped sAnim_CirclingMusicNote_SlantedBeamedEighth_Flipped sAnim_CirclingMusicNote_Quarter_Flipped sAnims_CirclingMusicNote sCirclingMusicNoteSpriteTemplate gProtectSpriteTemplate gMilkBottleAffineAnimCmds1 gMilkBottleAffineAnimCmds2 gMilkBottleAffineAnimTable gMilkBottleSpriteTemplate gGrantingStarsAnimCmds gGrantingStarsAnimTable gGrantingStarsSpriteTemplate gSparklingStarsSpriteTemplate sAnim_BubbleBurst sAnim_BubbleBurst_Flipped sAnims_BubbleBurst sBubbleBurstSpriteTemplate gSleepLetterZAnimCmds gSleepLetterZAnimTable gSleepLetterZAffineAnimCmds1 gSleepLetterZAffineAnimCmds1_2 gSleepLetterZAffineAnimCmds2 gSleepLetterZAffineAnimCmds2_2 gSleepLetterZAffineAnimTable gSleepLetterZSpriteTemplate gLockOnTargetSpriteTemplate gLockOnMoveTargetSpriteTemplate gInclineMonCoordTable gBowMonSpriteTemplate sTipMonSpriteTemplate gSlashSliceAnimCmds1 gSlashSliceAnimCmds2 gSlashSliceAnimTable gSlashSliceSpriteTemplate gFalseSwipeSliceSpriteTemplate gFalseSwipePositionedSliceSpriteTemplate gEndureEnergyAnimCmds gEndureEnergyAnimTable gEndureEnergySpriteTemplate gSharpenSphereAnimCmds gSharpenSphereAnimTable gSharpenSphereSpriteTemplate gOctazookaBallSpriteTemplate gOctazookaAnimCmds gOctazookaAnimTable gOctazookaSmokeSpriteTemplate gConversionAnimCmds gConversionAnimTable gConversionAffineAnimCmds gConversionAffineAnimTable gConversionSpriteTemplate gConversion2AnimCmds gConversion2AnimTable gConversion2SpriteTemplate gMoonSpriteTemplate gMoonlightSparkleAnimCmds gMoonlightSparkleAnimTable gMoonlightSparkleSpriteTemplate gHealingBlueStarAnimCmds gHealingBlueStarAnimTable gHealingBlueStarSpriteTemplate gHornHitSpriteTemplate gSuperFangAnimCmds gSuperFangAnimTable gSuperFangSpriteTemplate gWavyMusicNotesAnimCmds1 gWavyMusicNotesAnimCmds2 gWavyMusicNotesAnimCmds3 gWavyMusicNotesAnimCmds4 gWavyMusicNotesAnimCmds5 gWavyMusicNotesAnimCmds6 gWavyMusicNotesAnimCmds7 gWavyMusicNotesAnimCmds8 gMusicNotesAnimTable gWavyMusicNotesAffineAnimCmds gMusicNotesAffineAnimTable gWavyMusicNotesSpriteTemplate gParticlesColorBlendTable gFastFlyingMusicNotesSpriteTemplate gBellyDrumHandSpriteTemplate gSlowFlyingMusicNotesAffineAnimCmds gSlowFlyingMusicNotesAffineAnimTable gSlowFlyingMusicNotesSpriteTemplate gMetronomeThroughtBubbleAnimCmds1 gMetronomeThroughtBubbleAnimCmds3 gMetronomeThroughtBubbleAnimCmds2 gMetronomeThroughtBubbleAnimCmds4 gMetronomeThroughtBubbleAnimTable gThoughtBubbleSpriteTemplate gMetronomeFingerAffineAnimCmds1 gMetronomeFingerAffineAnimCmds2 gMetronomeFingerAffineAnimCmds2_2 gMetronomeFingerAffineAnimTable gMetronomeFingerSpriteTemplate gFollowMeFingerSpriteTemplate gTauntFingerAnimCmds1 gTauntFingerAnimCmds2 gTauntFingerAnimCmds3 gTauntFingerAnimCmds4 gTauntFingerAnimTable gTauntFingerSpriteTemplate

/// `__typeof__(sFrenzyPlantRootData)`
#[repr(C, align(4))]
#[derive(Clone, Copy)]
pub struct sFrenzyPlantRootData_t {
    pub startX: i16,
    pub startY: i16,
    pub targetX: i16,
    pub targetY: i16,
}

unsafe impl Sync for sFrenzyPlantRootData_t {}

/// `__anon1`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon1 {
    pub x: i16,
    pub y: i16,
    pub duration: i16,
    pub yVelocity: i16,
    pub waveAmplitude: i16,
    pub waveSpeed: i16,
}

unsafe impl Sync for Anon1 {}

/// `__anon2`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon2 {
    pub x: i16,
    pub y: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon2 {}

/// `__anon3`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon3 {
    pub x: i16,
    pub y: i16,
    pub duration: i16,
    pub animation: i16,
}

unsafe impl Sync for Anon3 {}

/// `__anon4`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon4 {
    pub x: i16,
    pub y: i16,
    pub duration: i16,
    pub waveOffset: i16,
}

unsafe impl Sync for Anon4 {}

/// `__anon5`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon5 {
    pub x: i16,
    pub y: i16,
    pub waveAmplitude: i16,
    pub wavePeriod: i16,
}

unsafe impl Sync for Anon5 {}

/// `__anon6`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon6 {
    pub initialX: i16,
    pub initialY: i16,
    pub targetX: i16,
    pub targetY: i16,
    pub duration: i16,
    pub waveAmplitude: i16,
}

unsafe impl Sync for Anon6 {}

/// `__anon7`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon7 {
    pub x: i16,
    pub y: i16,
    pub waveOffset: i16,
    pub duration: i16,
    pub blend: i16,
}

unsafe impl Sync for Anon7 {}

/// `__anon8`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon8 {
    pub initialX: i16,
    pub initialY: i16,
    pub targetY: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon8 {}

/// `__anon9`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon9 {
    pub initialX: i16,
    pub initialY: i16,
    pub targetY: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon9 {}

/// `__anon10`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon10 {
    pub upwardDeltaX: i16,
    pub upwardDeltaY: i16,
    pub upwardDuration: i16,
}

unsafe impl Sync for Anon10 {}

/// `__anon11`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon11 {
    pub initialX: i16,
    pub initialY: i16,
    pub targetX: i16,
    pub targetY: i16,
    pub duration: i16,
    pub waveAmplitude: i16,
    pub targetBoth: i16,
}

unsafe impl Sync for Anon11 {}

/// `__anon12`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon12 {
    pub duration: i16,
    pub distanceY: i16,
    pub wavePeriod: i16,
    pub waveAmplitude: i16,
    pub speedUpOnFrame: i16,
}

unsafe impl Sync for Anon12 {}

/// `__anon13`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon13 {
    pub initialX: i16,
    pub initialY: i16,
    pub affineAnimation: i16,
    pub squeezes: i16,
}

unsafe impl Sync for Anon13 {}

/// `__anon14`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon14 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon14 {}

/// `__anon15`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon15 {
    pub initialX: i16,
    pub initialY: i16,
}

unsafe impl Sync for Anon15 {}

/// `__anon16`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon16 {
    pub offsetX: i16,
    pub offsetY: i16,
    pub subpriorityM30: i16,
    pub animation: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon16 {}

/// `__anon17`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon17 {
    pub interpolatePercent: i16,
    pub offsetX: i16,
    pub offsetY: i16,
    pub subpriorityM30: i16,
    pub animation: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon17 {}

/// `__anon18`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon18 {
    pub initialX: i16,
    pub initialY: i16,
    pub velocityX: i16,
    pub waveAmplitude: i16,
    pub duration: i16,
}

unsafe impl Sync for Anon18 {}

/// `__anon19`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon19 {
    pub initialX: i16,
    pub initialY: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
}

unsafe impl Sync for Anon19 {}

/// `__anon20`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon20 {
    pub initialX: i16,
    pub initialY: i16,
    pub velocityY: i16,
    pub unused3: i16,
}

unsafe impl Sync for Anon20 {}

/// `__anon21`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon21 {
    pub initialX: i16,
    pub initialY: i16,
}

unsafe impl Sync for Anon21 {}

/// `__anon22`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon22 {
    pub initialY: i16,
    pub waveOffset: i16,
}

unsafe impl Sync for Anon22 {}

/// `__anon23`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon23 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
    pub unk5: i16,
    pub unk6: i16,
}

unsafe impl Sync for Anon23 {}

/// `__anon24`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon24 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
}

unsafe impl Sync for Anon24 {}

/// `__anon25`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon25 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon25 {}

/// `__anon26`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon26 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon26 {}

/// `__anon27`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon27 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
    pub unk5: i16,
    pub unk6: i16,
}

unsafe impl Sync for Anon27 {}

/// `__anon28`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon28 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon28 {}

/// `__anon29`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon29 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
}

unsafe impl Sync for Anon29 {}

/// `__anon30`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon30 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
    pub unk5: i16,
}

unsafe impl Sync for Anon30 {}

/// `__anon31`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon31 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon31 {}

/// `__anon32`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon32 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
    pub unk5: i16,
}

unsafe impl Sync for Anon32 {}

/// `__anon33`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon33 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: i16,
    pub unk5: i16,
    pub unk6: i16,
}

unsafe impl Sync for Anon33 {}

/// `__anon34`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon34 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon34 {}

/// `__anon35`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon35 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon35 {}

/// `__anon36`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon36 {
    pub unk0: i16,
}

unsafe impl Sync for Anon36 {}

/// `__anon37`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon37 {
    pub unk0: i16,
}

unsafe impl Sync for Anon37 {}

/// `__anon38`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon38 {
    pub unk0: i16,
}

unsafe impl Sync for Anon38 {}

/// `__anon39`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon39 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon39 {}

/// `__anon40`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon40 {
    pub unk0: i16,
}

unsafe impl Sync for Anon40 {}

/// `__anon41`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon41 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
}

unsafe impl Sync for Anon41 {}

/// `__anon42`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon42 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon42 {}

/// `__anon43`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon43 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon43 {}

/// `__anon44`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon44 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon44 {}

/// `__anon45`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon45 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon45 {}

/// `__anon46`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon46 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon46 {}

/// `__anon47`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon47 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon47 {}

/// `__anon48`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon48 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon48 {}

/// `__anon49`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon49 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
}

unsafe impl Sync for Anon49 {}

/// `__anon50`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon50 {
    pub unk0: i16,
}

unsafe impl Sync for Anon50 {}

/// `__anon51`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon51 {
    pub unk0: i16,
    pub unk1: i16,
    pub unk2: i16,
    pub unk3: i16,
}

unsafe impl Sync for Anon51 {}

/// `__anon52`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon52 {
    pub unk0: i16,
    pub unk1: i16,
}

unsafe impl Sync for Anon52 {}

/// `__anon53`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon53 {
    pub unk0: i16,
}

unsafe impl Sync for Anon53 {}

/// `__anon54`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon54 {
    pub unk0: i16,
}

unsafe impl Sync for Anon54 {}

/// `__anon55`
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Anon55 {
    pub unk0: i16,
}

unsafe impl Sync for Anon55 {}

#[cfg(target_arch = "arm")]
const _: () = {
    #[allow(unused_imports)]
    use core::mem::{offset_of, size_of};
    assert!(size_of::<sFrenzyPlantRootData_t>() == 8);
    assert!(offset_of!(sFrenzyPlantRootData_t, startX) == 0);
    assert!(offset_of!(sFrenzyPlantRootData_t, startY) == 2);
    assert!(offset_of!(sFrenzyPlantRootData_t, targetX) == 4);
    assert!(offset_of!(sFrenzyPlantRootData_t, targetY) == 6);
    assert!(size_of::<Anon1>() == 12);
    assert!(offset_of!(Anon1, x) == 0);
    assert!(offset_of!(Anon1, y) == 2);
    assert!(offset_of!(Anon1, duration) == 4);
    assert!(offset_of!(Anon1, yVelocity) == 6);
    assert!(offset_of!(Anon1, waveAmplitude) == 8);
    assert!(offset_of!(Anon1, waveSpeed) == 10);
    assert!(size_of::<Anon2>() == 6);
    assert!(offset_of!(Anon2, x) == 0);
    assert!(offset_of!(Anon2, y) == 2);
    assert!(offset_of!(Anon2, duration) == 4);
    assert!(size_of::<Anon3>() == 8);
    assert!(offset_of!(Anon3, x) == 0);
    assert!(offset_of!(Anon3, y) == 2);
    assert!(offset_of!(Anon3, duration) == 4);
    assert!(offset_of!(Anon3, animation) == 6);
    assert!(size_of::<Anon4>() == 8);
    assert!(offset_of!(Anon4, x) == 0);
    assert!(offset_of!(Anon4, y) == 2);
    assert!(offset_of!(Anon4, duration) == 4);
    assert!(offset_of!(Anon4, waveOffset) == 6);
    assert!(size_of::<Anon5>() == 8);
    assert!(offset_of!(Anon5, x) == 0);
    assert!(offset_of!(Anon5, y) == 2);
    assert!(offset_of!(Anon5, waveAmplitude) == 4);
    assert!(offset_of!(Anon5, wavePeriod) == 6);
    assert!(size_of::<Anon6>() == 12);
    assert!(offset_of!(Anon6, initialX) == 0);
    assert!(offset_of!(Anon6, initialY) == 2);
    assert!(offset_of!(Anon6, targetX) == 4);
    assert!(offset_of!(Anon6, targetY) == 6);
    assert!(offset_of!(Anon6, duration) == 8);
    assert!(offset_of!(Anon6, waveAmplitude) == 10);
    assert!(size_of::<Anon7>() == 10);
    assert!(offset_of!(Anon7, x) == 0);
    assert!(offset_of!(Anon7, y) == 2);
    assert!(offset_of!(Anon7, waveOffset) == 4);
    assert!(offset_of!(Anon7, duration) == 6);
    assert!(offset_of!(Anon7, blend) == 8);
    assert!(size_of::<Anon8>() == 8);
    assert!(offset_of!(Anon8, initialX) == 0);
    assert!(offset_of!(Anon8, initialY) == 2);
    assert!(offset_of!(Anon8, targetY) == 4);
    assert!(offset_of!(Anon8, duration) == 6);
    assert!(size_of::<Anon9>() == 8);
    assert!(offset_of!(Anon9, initialX) == 0);
    assert!(offset_of!(Anon9, initialY) == 2);
    assert!(offset_of!(Anon9, targetY) == 4);
    assert!(offset_of!(Anon9, duration) == 6);
    assert!(size_of::<Anon10>() == 6);
    assert!(offset_of!(Anon10, upwardDeltaX) == 0);
    assert!(offset_of!(Anon10, upwardDeltaY) == 2);
    assert!(offset_of!(Anon10, upwardDuration) == 4);
    assert!(size_of::<Anon11>() == 14);
    assert!(offset_of!(Anon11, initialX) == 0);
    assert!(offset_of!(Anon11, initialY) == 2);
    assert!(offset_of!(Anon11, targetX) == 4);
    assert!(offset_of!(Anon11, targetY) == 6);
    assert!(offset_of!(Anon11, duration) == 8);
    assert!(offset_of!(Anon11, waveAmplitude) == 10);
    assert!(offset_of!(Anon11, targetBoth) == 12);
    assert!(size_of::<Anon12>() == 10);
    assert!(offset_of!(Anon12, duration) == 0);
    assert!(offset_of!(Anon12, distanceY) == 2);
    assert!(offset_of!(Anon12, wavePeriod) == 4);
    assert!(offset_of!(Anon12, waveAmplitude) == 6);
    assert!(offset_of!(Anon12, speedUpOnFrame) == 8);
    assert!(size_of::<Anon13>() == 8);
    assert!(offset_of!(Anon13, initialX) == 0);
    assert!(offset_of!(Anon13, initialY) == 2);
    assert!(offset_of!(Anon13, affineAnimation) == 4);
    assert!(offset_of!(Anon13, squeezes) == 6);
    assert!(size_of::<Anon14>() == 4);
    assert!(offset_of!(Anon14, unk0) == 0);
    assert!(offset_of!(Anon14, unk1) == 2);
    assert!(size_of::<Anon15>() == 4);
    assert!(offset_of!(Anon15, initialX) == 0);
    assert!(offset_of!(Anon15, initialY) == 2);
    assert!(size_of::<Anon16>() == 10);
    assert!(offset_of!(Anon16, offsetX) == 0);
    assert!(offset_of!(Anon16, offsetY) == 2);
    assert!(offset_of!(Anon16, subpriorityM30) == 4);
    assert!(offset_of!(Anon16, animation) == 6);
    assert!(offset_of!(Anon16, duration) == 8);
    assert!(size_of::<Anon17>() == 12);
    assert!(offset_of!(Anon17, interpolatePercent) == 0);
    assert!(offset_of!(Anon17, offsetX) == 2);
    assert!(offset_of!(Anon17, offsetY) == 4);
    assert!(offset_of!(Anon17, subpriorityM30) == 6);
    assert!(offset_of!(Anon17, animation) == 8);
    assert!(offset_of!(Anon17, duration) == 10);
    assert!(size_of::<Anon18>() == 10);
    assert!(offset_of!(Anon18, initialX) == 0);
    assert!(offset_of!(Anon18, initialY) == 2);
    assert!(offset_of!(Anon18, velocityX) == 4);
    assert!(offset_of!(Anon18, waveAmplitude) == 6);
    assert!(offset_of!(Anon18, duration) == 8);
    assert!(size_of::<Anon19>() == 10);
    assert!(offset_of!(Anon19, initialX) == 0);
    assert!(offset_of!(Anon19, initialY) == 2);
    assert!(offset_of!(Anon19, unk2) == 4);
    assert!(offset_of!(Anon19, unk3) == 6);
    assert!(offset_of!(Anon19, unk4) == 8);
    assert!(size_of::<Anon20>() == 8);
    assert!(offset_of!(Anon20, initialX) == 0);
    assert!(offset_of!(Anon20, initialY) == 2);
    assert!(offset_of!(Anon20, velocityY) == 4);
    assert!(offset_of!(Anon20, unused3) == 6);
    assert!(size_of::<Anon21>() == 4);
    assert!(offset_of!(Anon21, initialX) == 0);
    assert!(offset_of!(Anon21, initialY) == 2);
    assert!(size_of::<Anon22>() == 4);
    assert!(offset_of!(Anon22, initialY) == 0);
    assert!(offset_of!(Anon22, waveOffset) == 2);
    assert!(size_of::<Anon23>() == 14);
    assert!(offset_of!(Anon23, unk0) == 0);
    assert!(offset_of!(Anon23, unk1) == 2);
    assert!(offset_of!(Anon23, unk2) == 4);
    assert!(offset_of!(Anon23, unk3) == 6);
    assert!(offset_of!(Anon23, unk4) == 8);
    assert!(offset_of!(Anon23, unk5) == 10);
    assert!(offset_of!(Anon23, unk6) == 12);
    assert!(size_of::<Anon24>() == 10);
    assert!(offset_of!(Anon24, unk0) == 0);
    assert!(offset_of!(Anon24, unk1) == 2);
    assert!(offset_of!(Anon24, unk2) == 4);
    assert!(offset_of!(Anon24, unk3) == 6);
    assert!(offset_of!(Anon24, unk4) == 8);
    assert!(size_of::<Anon25>() == 4);
    assert!(offset_of!(Anon25, unk0) == 0);
    assert!(offset_of!(Anon25, unk1) == 2);
    assert!(size_of::<Anon26>() == 4);
    assert!(offset_of!(Anon26, unk0) == 0);
    assert!(offset_of!(Anon26, unk1) == 2);
    assert!(size_of::<Anon27>() == 14);
    assert!(offset_of!(Anon27, unk0) == 0);
    assert!(offset_of!(Anon27, unk1) == 2);
    assert!(offset_of!(Anon27, unk2) == 4);
    assert!(offset_of!(Anon27, unk3) == 6);
    assert!(offset_of!(Anon27, unk4) == 8);
    assert!(offset_of!(Anon27, unk5) == 10);
    assert!(offset_of!(Anon27, unk6) == 12);
    assert!(size_of::<Anon28>() == 6);
    assert!(offset_of!(Anon28, unk0) == 0);
    assert!(offset_of!(Anon28, unk1) == 2);
    assert!(offset_of!(Anon28, unk2) == 4);
    assert!(size_of::<Anon29>() == 8);
    assert!(offset_of!(Anon29, unk0) == 0);
    assert!(offset_of!(Anon29, unk1) == 2);
    assert!(offset_of!(Anon29, unk2) == 4);
    assert!(offset_of!(Anon29, unk3) == 6);
    assert!(size_of::<Anon30>() == 12);
    assert!(offset_of!(Anon30, unk0) == 0);
    assert!(offset_of!(Anon30, unk1) == 2);
    assert!(offset_of!(Anon30, unk2) == 4);
    assert!(offset_of!(Anon30, unk3) == 6);
    assert!(offset_of!(Anon30, unk4) == 8);
    assert!(offset_of!(Anon30, unk5) == 10);
    assert!(size_of::<Anon31>() == 6);
    assert!(offset_of!(Anon31, unk0) == 0);
    assert!(offset_of!(Anon31, unk1) == 2);
    assert!(offset_of!(Anon31, unk2) == 4);
    assert!(size_of::<Anon32>() == 12);
    assert!(offset_of!(Anon32, unk0) == 0);
    assert!(offset_of!(Anon32, unk1) == 2);
    assert!(offset_of!(Anon32, unk2) == 4);
    assert!(offset_of!(Anon32, unk3) == 6);
    assert!(offset_of!(Anon32, unk4) == 8);
    assert!(offset_of!(Anon32, unk5) == 10);
    assert!(size_of::<Anon33>() == 14);
    assert!(offset_of!(Anon33, unk0) == 0);
    assert!(offset_of!(Anon33, unk1) == 2);
    assert!(offset_of!(Anon33, unk2) == 4);
    assert!(offset_of!(Anon33, unk3) == 6);
    assert!(offset_of!(Anon33, unk4) == 8);
    assert!(offset_of!(Anon33, unk5) == 10);
    assert!(offset_of!(Anon33, unk6) == 12);
    assert!(size_of::<Anon34>() == 4);
    assert!(offset_of!(Anon34, unk0) == 0);
    assert!(offset_of!(Anon34, unk1) == 2);
    assert!(size_of::<Anon35>() == 4);
    assert!(offset_of!(Anon35, unk0) == 0);
    assert!(offset_of!(Anon35, unk1) == 2);
    assert!(size_of::<Anon36>() == 2);
    assert!(offset_of!(Anon36, unk0) == 0);
    assert!(size_of::<Anon37>() == 2);
    assert!(offset_of!(Anon37, unk0) == 0);
    assert!(size_of::<Anon38>() == 2);
    assert!(offset_of!(Anon38, unk0) == 0);
    assert!(size_of::<Anon39>() == 6);
    assert!(offset_of!(Anon39, unk0) == 0);
    assert!(offset_of!(Anon39, unk1) == 2);
    assert!(offset_of!(Anon39, unk2) == 4);
    assert!(size_of::<Anon40>() == 2);
    assert!(offset_of!(Anon40, unk0) == 0);
    assert!(size_of::<Anon41>() == 8);
    assert!(offset_of!(Anon41, unk0) == 0);
    assert!(offset_of!(Anon41, unk1) == 2);
    assert!(offset_of!(Anon41, unk2) == 4);
    assert!(offset_of!(Anon41, unk3) == 6);
    assert!(size_of::<Anon42>() == 4);
    assert!(offset_of!(Anon42, unk0) == 0);
    assert!(offset_of!(Anon42, unk1) == 2);
    assert!(size_of::<Anon43>() == 6);
    assert!(offset_of!(Anon43, unk0) == 0);
    assert!(offset_of!(Anon43, unk1) == 2);
    assert!(offset_of!(Anon43, unk2) == 4);
    assert!(size_of::<Anon44>() == 4);
    assert!(offset_of!(Anon44, unk0) == 0);
    assert!(offset_of!(Anon44, unk1) == 2);
    assert!(size_of::<Anon45>() == 4);
    assert!(offset_of!(Anon45, unk0) == 0);
    assert!(offset_of!(Anon45, unk1) == 2);
    assert!(size_of::<Anon46>() == 4);
    assert!(offset_of!(Anon46, unk0) == 0);
    assert!(offset_of!(Anon46, unk1) == 2);
    assert!(size_of::<Anon47>() == 6);
    assert!(offset_of!(Anon47, unk0) == 0);
    assert!(offset_of!(Anon47, unk1) == 2);
    assert!(offset_of!(Anon47, unk2) == 4);
    assert!(size_of::<Anon48>() == 6);
    assert!(offset_of!(Anon48, unk0) == 0);
    assert!(offset_of!(Anon48, unk1) == 2);
    assert!(offset_of!(Anon48, unk2) == 4);
    assert!(size_of::<Anon49>() == 6);
    assert!(offset_of!(Anon49, unk0) == 0);
    assert!(offset_of!(Anon49, unk1) == 2);
    assert!(offset_of!(Anon49, unk2) == 4);
    assert!(size_of::<Anon50>() == 2);
    assert!(offset_of!(Anon50, unk0) == 0);
    assert!(size_of::<Anon51>() == 8);
    assert!(offset_of!(Anon51, unk0) == 0);
    assert!(offset_of!(Anon51, unk1) == 2);
    assert!(offset_of!(Anon51, unk2) == 4);
    assert!(offset_of!(Anon51, unk3) == 6);
    assert!(size_of::<Anon52>() == 4);
    assert!(offset_of!(Anon52, unk0) == 0);
    assert!(offset_of!(Anon52, unk1) == 2);
    assert!(size_of::<Anon53>() == 2);
    assert!(offset_of!(Anon53, unk0) == 0);
    assert!(size_of::<Anon54>() == 2);
    assert!(offset_of!(Anon54, unk0) == 0);
    assert!(size_of::<Anon55>() == 2);
    assert!(offset_of!(Anon55, unk0) == 0);
};

static gInclineMonCoordTable: Table<CArray<CArray<i8, 2>, 4>> =
    Table((&raw const crate::data::battle_anim_effects_1::gInclineMonCoordTable).cast());
static gLeafBladeSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_1::gLeafBladeSpriteTemplate).cast());
static gMagicalLeafBlendColors: Table<CArray<u16, 7>> =
    Table((&raw const crate::data::battle_anim_effects_1::gMagicalLeafBlendColors).cast());
static gMoonSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_1::gMoonSpriteTemplate).cast());
static gMoonlightSparkleSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_1::gMoonlightSparkleSpriteTemplate).cast());
static gParticlesColorBlendTable: Table<CArray<CArray<u16, 6>, 4>> =
    Table((&raw const crate::data::battle_anim_effects_1::gParticlesColorBlendTable).cast());
static gSolarBeamSmallOrbSpriteTemplate: Table<SpriteTemplate> =
    Table((&raw const crate::data::battle_anim_effects_1::gSolarBeamSmallOrbSpriteTemplate).cast());
static gTrickBagCoordinates: Table<CArray<CArray<i8, 3>, 11>> =
    Table((&raw const crate::data::battle_anim_effects_1::gTrickBagCoordinates).cast());

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrenzyPlantRootData: sFrenzyPlantRootData_t = unsafe { zeroed() };

pub(crate) unsafe fn AnimMovePowderParticle(sprite: *mut Sprite) {
    let cmd: *mut Anon1 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon1;
    (*sprite).x += (*cmd).x;
    (*sprite).y += (*cmd).y;
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*cmd).yVelocity;
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).data[3] = -(*cmd).waveAmplitude;
    } else {
        (*sprite).data[3] = (*cmd).waveAmplitude;
    }
    (*sprite).data[4] = (*cmd).waveSpeed;
    (*sprite).callback = Some(AnimMovePowderParticle_Step);
}
pub(crate) unsafe fn AnimMovePowderParticle_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
        (*sprite).y2 = (*sprite).data[2] >> 8;
        (*sprite).data[2] += (*sprite).data[1];
        (*sprite).x2 = Sin((*sprite).data[5], (*sprite).data[3]);
        (*sprite).data[5] = ((*sprite).data[5] + (*sprite).data[4]) & 0xFF;
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimPowerAbsorptionOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon2 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon2;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
}
pub(crate) unsafe fn AnimSolarBeamBigOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon3 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon3;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    StartSpriteAnim(sprite, (*cmd).animation as u8);
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).callback = Some(StartAnimLinearTranslation);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimSolarBeamSmallOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon4 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon4;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = (*cmd).waveOffset;
    (*sprite).callback = Some(AnimSolarBeamSmallOrb_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimSolarBeamSmallOrb_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) != 0 {
        DestroySprite(sprite);
    } else {
        if (*sprite).data[5] > 0x7F {
            (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) + 1;
        } else {
            (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimTarget) + 6;
        }
        (*sprite).x2 += Sin((*sprite).data[5], 5);
        (*sprite).y2 += Cos((*sprite).data[5], 14);
        (*sprite).data[5] = ((*sprite).data[5] + 15) & 0xFF;
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CreateSmallSolarBeamOrbs(taskId: u8) {
    if ({
        task_set(taskId, 0, task_get(taskId, 0) - 1);
        task_get(taskId, 0)
    }) == -1
    {
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        task_set(taskId, 0, 6);
        gBattleAnimArgs[0] = 15;
        gBattleAnimArgs[1] = 0;
        gBattleAnimArgs[2] = 80;
        gBattleAnimArgs[3] = 0;
        CreateSpriteAndAnimate(
            (&raw const *gSolarBeamSmallOrbSpriteTemplate).cast_mut(),
            0,
            0,
            GetBattlerSpriteSubpriority(gBattleAnimTarget) + 1,
        );
    }
    if task_get(taskId, 1) == 15 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimAbsorptionOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon5 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon5;
    InitSpritePosToAnimTarget(sprite, TRUE);
    (*sprite).data[0] = (*cmd).wavePeriod;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[5] = (*cmd).waveAmplitude;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimAbsorptionOrb_Step);
}
pub(crate) unsafe fn AnimAbsorptionOrb_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimHyperBeamOrb(sprite: *mut Sprite) {
    let animNum: u16 = Random2();
    StartSpriteAnim(sprite, (animNum as i32 % 8) as u8);
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= 20;
    } else {
        (*sprite).x += 20;
    }
    let speed: u16 = Random2();
    (*sprite).data[0] = (speed as i16 & 31) + 64;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    InitAnimFastLinearTranslationWithSpeed(sprite);
    (*sprite).data[5] = Random2() as i16 & 0xFF;
    (*sprite).data[6] = (*sprite).subpriority as i16;
    (*sprite).callback = Some(AnimHyperBeamOrb_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimHyperBeamOrb_Step(sprite: *mut Sprite) {
    if AnimFastTranslateLinear(sprite) != 0 {
        DestroyAnimSprite(sprite);
    } else {
        (*sprite).y2 += Cos((*sprite).data[5], 12);
        if (*sprite).data[5] < 0x7F {
            (*sprite).subpriority = (*sprite).data[6] as u8;
        } else {
            (*sprite).subpriority = (*sprite).data[6] as u8 + 1;
        }
        (*sprite).data[5] += 24;
        (*sprite).data[5] &= 0xFF;
    }
}
pub(crate) unsafe fn AnimLeechSeed(sprite: *mut Sprite) {
    let cmd: *mut Anon6 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon6;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*cmd).targetX = -(*cmd).targetX;
    }
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[2] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 + (*cmd).targetX;
    (*sprite).data[4] =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + (*cmd).targetY;
    (*sprite).data[5] = (*cmd).waveAmplitude;
    InitAnimArcTranslation(sprite);
    (*sprite).callback = Some(AnimLeechSeed_Step);
}
pub(crate) unsafe fn AnimLeechSeed_Step(sprite: *mut Sprite) {
    if TranslateAnimHorizontalArc(sprite) != 0 {
        (*sprite).set_invisible(TRUE as u16);
        (*sprite).data[0] = 10;
        (*sprite).callback = Some(WaitAnimForDuration);
        StoreSpriteCallbackInData6(sprite, Some(AnimLeechSeedSprouts));
    }
}
pub(crate) unsafe fn AnimLeechSeedSprouts(sprite: *mut Sprite) {
    (*sprite).set_invisible(FALSE as u16);
    StartSpriteAnim(sprite, 1);
    (*sprite).data[0] = 60;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimSporeParticle(sprite: *mut Sprite) {
    let cmd: *mut Anon7 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon7;
    InitSpritePosToAnimTarget(sprite, TRUE);
    StartSpriteAnim(sprite, (*cmd).blend as u8);
    if (*cmd).blend == TRUE as i16 {
        (*sprite).oam.set_objMode(ST_OAM_OBJ_BLEND);
    }
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*cmd).waveOffset;
    (*sprite).callback = Some(AnimSporeParticle_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimSporeParticle_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Sin((*sprite).data[1], 32);
    (*sprite).y2 = Cos((*sprite).data[1], -3)
        + (({
            (*sprite).data[2] += 24;
            (*sprite).data[2]
        }) >> 8);
    if (*sprite).data[1] as u16 as i32 - 0x40 < 0x80 {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
    } else {
        let mut priority: u8 = GetBattlerSpriteBGPriority(gBattleAnimTarget) + 1;
        if priority > 3 {
            priority = 3;
        }
        (*sprite).oam.set_priority(priority as u16);
    }
    (*sprite).data[1] += 2;
    (*sprite).data[1] &= 0xFF;
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == -1
    {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SporeDoubleBattle(taskId: u8) {
    if IsContest() != 0 || IsDoubleBattle() == 0 {
        DestroyAnimVisualTask(taskId);
    } else {
        if GetBattlerSpriteBGPriorityRank(gBattleAnimTarget) == 1 {
            SetAnimBgAttribute(2, BG_ANIM_PRIORITY, 3);
        } else {
            SetAnimBgAttribute(1, BG_ANIM_PRIORITY, 1);
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimPetalDanceBigFlower(sprite: *mut Sprite) {
    let cmd: *mut Anon8 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon8;
    InitSpritePosToAnimAttacker(sprite, FALSE);
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET)
        as i16
        + (*cmd).targetY;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = 0x40;
    (*sprite).callback = Some(AnimPetalDanceBigFlower_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimPetalDanceBigFlower_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).x2 += Sin((*sprite).data[5], 32);
        (*sprite).y2 += Cos((*sprite).data[5], -5);
        if (*sprite).data[5] as u16 as i32 - 0x40 < 0x80 {
            (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) - 1;
        } else {
            (*sprite).subpriority = GetBattlerSpriteSubpriority(gBattleAnimAttacker) + 1;
        }
        (*sprite).data[5] = ((*sprite).data[5] + 5) & 0xFF;
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimPetalDanceSmallFlower(sprite: *mut Sprite) {
    let cmd: *mut Anon9 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon9;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = (*sprite).x;
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET)
        as i16
        + (*cmd).targetY;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = 0x40;
    (*sprite).callback = Some(AnimPetalDanceSmallFlower_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimPetalDanceSmallFlower_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        (*sprite).x2 += Sin((*sprite).data[5], 8);
        if (*sprite).data[5] as u16 as i32 - 59 < 5 || (*sprite).data[5] as u16 as i32 - 187 < 5 {
            (*sprite)
                .oam
                .set_matrixNum((*sprite).oam.matrixNum() ^ ST_OAM_HFLIP);
        }
        (*sprite).data[5] += 5;
        (*sprite).data[5] &= 0xFF;
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimRazorLeafParticle(sprite: *mut Sprite) {
    let cmd: *mut Anon10 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon10;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*sprite).data[0] = (*cmd).upwardDeltaX;
    (*sprite).data[1] = (*cmd).upwardDeltaY;
    (*sprite).data[2] = (*cmd).upwardDuration;
    (*sprite).callback = Some(AnimRazorLeafParticle_Step1);
}
pub(crate) unsafe fn AnimRazorLeafParticle_Step1(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        if (*sprite).data[1] as i32 & 1 != 0 {
            (*sprite).data[0] = 0x80;
            (*sprite).data[1] = 0;
            (*sprite).data[2] = 0;
        } else {
            (*sprite).data[0] = 0;
            (*sprite).data[1] = 0;
            (*sprite).data[2] = 0;
        }
        (*sprite).callback = Some(AnimRazorLeafParticle_Step2);
    } else {
        (*sprite).data[2] -= 1;
        (*sprite).x += (*sprite).data[0];
        (*sprite).y += (*sprite).data[1];
    }
}
pub(crate) unsafe fn AnimRazorLeafParticle_Step2(sprite: *mut Sprite) {
    if GetBattlerSide(gBattleAnimAttacker) != 0 {
        (*sprite).x2 = -Sin((*sprite).data[0], 25);
    } else {
        (*sprite).x2 = Sin((*sprite).data[0], 25);
    }
    (*sprite).data[0] += 2;
    (*sprite).data[0] &= 0xFF;
    (*sprite).data[1] += 1;
    if (*sprite).data[1] as i32 & 1 == 0 {
        (*sprite).y2 += 1;
    }
    if (*sprite).data[1] > 80 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimTranslateLinearSingleSineWave(sprite: *mut Sprite) {
    let cmd: *mut Anon11 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon11;
    InitSpritePosToAnimAttacker(sprite, TRUE);
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*cmd).targetX = -(*cmd).targetX;
    }
    (*sprite).data[0] = (*cmd).duration;
    if (*cmd).targetBoth == 0 {
        (*sprite).data[2] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).targetX;
        (*sprite).data[4] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET)
            as i16
            + (*cmd).targetY;
    } else {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            TRUE,
            &raw mut (*sprite).data[2],
            &raw mut (*sprite).data[4],
        );
        (*sprite).data[2] += (*cmd).targetX;
        (*sprite).data[4] += (*cmd).targetY;
    }
    (*sprite).data[5] = (*cmd).waveAmplitude;
    InitAnimArcTranslation(sprite);
    if GetBattlerSide(gBattleAnimAttacker) == GetBattlerSide(gBattleAnimTarget) {
        (*sprite).data[0] = 1;
    } else {
        (*sprite).data[0] = 0;
    }
    (*sprite).callback = Some(AnimTranslateLinearSingleSineWave_Step);
}
pub(crate) unsafe fn AnimTranslateLinearSingleSineWave_Step(sprite: *mut Sprite) {
    let mut destroy: u8 = FALSE;
    let a: i16 = (*sprite).data[0];
    let b: i16 = (*sprite).data[7];
    (*sprite).data[0] = 1;
    TranslateAnimHorizontalArc(sprite);
    let r0: i16 = (*sprite).data[7];
    (*sprite).data[0] = a;
    if b > 200 && r0 < 56 && (*sprite).oam.affineParam == 0 {
        (*sprite).oam.affineParam += 1;
    }
    if (*sprite).oam.affineParam != 0 && (*sprite).data[0] != 0 {
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        (*sprite).oam.affineParam += 1;
        if (*sprite).oam.affineParam == 30 {
            destroy = TRUE;
        }
    }
    if (*sprite).x as i32 + (*sprite).x2 as i32 > 256
        || ((*sprite).x as i32 + (*sprite).x2 as i32) < -16
        || (*sprite).y as i32 + (*sprite).y2 as i32 > DISPLAY_HEIGHT as i32
        || ((*sprite).y as i32 + (*sprite).y2 as i32) < -16
    {
        destroy = TRUE;
    }
    if destroy != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub unsafe fn AnimMoveTwisterParticle(sprite: *mut Sprite) {
    let cmd: *mut Anon12 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon12;
    if IsDoubleBattle() == TRUE {
        SetAverageBattlerPositions(
            gBattleAnimTarget,
            TRUE,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
    }
    (*sprite).y += 32;
    (*sprite).data[0] = (*cmd).duration;
    (*sprite).data[1] = (*cmd).distanceY;
    (*sprite).data[2] = (*cmd).wavePeriod;
    (*sprite).data[3] = (*cmd).waveAmplitude;
    (*sprite).data[4] = (*cmd).speedUpOnFrame;
    (*sprite).callback = Some(AnimMoveTwisterParticle_Step);
}
pub(crate) unsafe fn AnimMoveTwisterParticle_Step(sprite: *mut Sprite) {
    if (*sprite).data[1] == 0xFF {
        (*sprite).y -= 2;
    } else if (*sprite).data[1] > 0 {
        (*sprite).y -= 2;
        (*sprite).data[1] -= 2;
    }
    (*sprite).data[5] += (*sprite).data[2];
    if (*sprite).data[0] < (*sprite).data[4] {
        (*sprite).data[5] += (*sprite).data[2];
    }
    (*sprite).data[5] &= 0xFF;
    (*sprite).x2 = Cos((*sprite).data[5], (*sprite).data[3]);
    (*sprite).y2 = Sin((*sprite).data[5], 5);
    if (*sprite).data[5] < 0x80 {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16 - 1);
    } else {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16 + 1);
    }
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == 0
    {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimConstrictBinding(sprite: *mut Sprite) {
    let cmd: *mut Anon13 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon13;
    InitSpritePosToAnimTarget(sprite, FALSE);
    (*sprite).set_affineAnimPaused(1);
    StartSpriteAffineAnim(sprite, (*cmd).affineAnimation as u8);
    (*sprite).data[6] = (*cmd).affineAnimation;
    (*sprite).data[7] = (*cmd).squeezes;
    (*sprite).callback = Some(AnimConstrictBinding_Step1);
}
pub(crate) unsafe fn AnimConstrictBinding_Step1(sprite: *mut Sprite) {
    let mut spriteId: u8 = 0;
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        (*sprite).set_affineAnimPaused(0);
        spriteId = GetAnimBattlerSpriteId(ANIM_TARGET);
        (*sprite).data[0] = 0x100;
        (*sprite).callback = Some(AnimConstrictBinding_Step2);
    }
}
pub(crate) unsafe fn AnimConstrictBinding_Step2(sprite: *mut Sprite) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
    if (*sprite).data[2] == 0 {
        (*sprite).data[0] += 11;
    } else {
        (*sprite).data[0] -= 11;
    }
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) == 6
    {
        (*sprite).data[1] = 0;
        (*sprite).data[2] ^= 1;
    }
    if (*sprite).affineAnimEnded() != 0 {
        if ({
            (*sprite).data[7] -= 1;
            (*sprite).data[7]
        }) > 0
        {
            StartSpriteAffineAnim(sprite, (*sprite).data[6] as u8);
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ShrinkTargetCopy(taskId: u8) {
    let cmd: *mut Anon14 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon14;
    let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
    if gSprites[spriteId].invisible() != 0 {
        DestroyAnimVisualTask(taskId);
    } else {
        PrepareBattlerSpriteForRotScale(spriteId, ST_OAM_OBJ_BLEND as u8);
        task_set(taskId, 14, gSprites[spriteId].oam.priority() as i16);
        gSprites[spriteId]
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget) as u16);
        spriteId = GetAnimBattlerSpriteId(ANIM_DEF_PARTNER);
        task_set(taskId, 15, gSprites[spriteId].oam.priority() as i16);
        gSprites[spriteId]
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimTarget ^ 2) as u16);
        task_set(taskId, 0, (*cmd).unk0);
        task_set(taskId, 1, (*cmd).unk1);
        task_set(taskId, 11, 0x100);
        task_set_func(taskId, Some(AnimTask_DuplicateAndShrinkToPos_Step1));
    }
}
pub(crate) unsafe fn AnimTask_DuplicateAndShrinkToPos_Step1(taskId: u8) {
    let spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
    task_set(taskId, 10, task_get(taskId, 10) + (task_get(taskId, 0)));
    gSprites[spriteId].x2 = task_get(taskId, 10) >> 8;
    if GetBattlerSide(gBattleAnimTarget) != B_SIDE_PLAYER {
        gSprites[spriteId].x2 = -gSprites[spriteId].x2;
    }
    task_set(taskId, 11, task_get(taskId, 11) + 16);
    SetSpriteRotScale(spriteId, task_get(taskId, 11), task_get(taskId, 11), 0);
    SetBattlerSpriteYOffsetFromYScale(spriteId);
    if ({
        task_set(taskId, 1, task_get(taskId, 1) - 1);
        task_get(taskId, 1)
    }) == 0
    {
        task_set(taskId, 0, 0);
        task_set_func(taskId, Some(AnimTask_DuplicateAndShrinkToPos_Step2));
    }
}
pub(crate) unsafe fn AnimTask_DuplicateAndShrinkToPos_Step2(taskId: u8) {
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        if task_get(taskId, 0) == 0 {
            let mut spriteId: u8 = GetAnimBattlerSpriteId(ANIM_TARGET);
            ResetSpriteRotScale(spriteId);
            gSprites[spriteId].x2 = 0;
            gSprites[spriteId].y2 = 0;
            gSprites[spriteId]
                .oam
                .set_priority(task_get(taskId, 14) as u16);
            spriteId = GetAnimBattlerSpriteId(ANIM_DEF_PARTNER);
            gSprites[spriteId]
                .oam
                .set_priority(task_get(taskId, 15) as u16);
            task_set(taskId, 0, task_get(taskId, 0) + 1);
            return;
        }
    } else {
        if task_get(taskId, 0) == 0 {
            return;
        }
    }
    task_set(taskId, 0, task_get(taskId, 0) + 1);
    if task_get(taskId, 0) == 3 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimMimicOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon15 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon15;
    'l1: {
        match (*sprite).data[0] {
            0 => {
                if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
                    (*cmd).initialX *= -1;
                }
                (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16
                    + (*cmd).initialX;
                (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16
                    + (*cmd).initialY;
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).data[0] += 1;
            }
            1 => {
                (*sprite).set_invisible(FALSE as u16);
                if (*sprite).affineAnimEnded() != 0 {
                    ChangeSpriteAffineAnim(sprite, 1);
                    (*sprite).data[0] = 25;
                    (*sprite).data[2] =
                        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
                    (*sprite).data[4] =
                        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET)
                            as i16;
                    (*sprite).callback = Some(InitAndRunAnimFastLinearTranslation);
                    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
                    break 'l1;
                }
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn AnimIngrainRoot(sprite: *mut Sprite) {
    let cmd: *mut Anon16 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon16;
    if (*sprite).data[0] == 0 {
        (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
        (*sprite).x2 = (*cmd).offsetX;
        (*sprite).y2 = (*cmd).offsetY;
        (*sprite).subpriority = (*cmd).subpriorityM30 as u8 + 30;
        StartSpriteAnim(sprite, (*cmd).animation as u8);
        (*sprite).data[2] = (*cmd).duration;
        (*sprite).data[0] += 1;
        if (*sprite).y as i32 + (*sprite).y2 as i32 > 120 {
            (*sprite).y += (*sprite).y2 + (*sprite).y - 120;
        }
    }
    (*sprite).callback = Some(AnimRootFlickerOut);
}
pub(crate) unsafe fn AnimFrenzyPlantRoot(sprite: *mut Sprite) {
    let cmd: *mut Anon17 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon17;
    let attackerX: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    let attackerY: i16 =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    let mut targetX: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    let mut targetY: i16 =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    targetX -= attackerX;
    targetY -= attackerY;
    (*sprite).x = attackerX + (targetX as i32 * (*cmd).interpolatePercent as i32 / 100) as i16;
    (*sprite).y = attackerY + (targetY as i32 * (*cmd).interpolatePercent as i32 / 100) as i16;
    (*sprite).x2 = (*cmd).offsetX;
    (*sprite).y2 = (*cmd).offsetY;
    (*sprite).subpriority = (*cmd).subpriorityM30 as u8 + 30;
    StartSpriteAnim(sprite, (*cmd).animation as u8);
    (*sprite).data[2] = (*cmd).duration;
    (*sprite).callback = Some(AnimRootFlickerOut);
    sFrenzyPlantRootData.startX = (*sprite).x;
    sFrenzyPlantRootData.startY = (*sprite).y;
    sFrenzyPlantRootData.targetX = targetX;
    sFrenzyPlantRootData.targetY = targetY;
}
pub(crate) unsafe fn AnimRootFlickerOut(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) as i32
        > (*sprite).data[2] as i32 - 10
    {
        (*sprite).set_invisible(((*sprite).data[0] % 2) as u16);
    }
    if (*sprite).data[0] > (*sprite).data[2] {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimIngrainOrb(sprite: *mut Sprite) {
    let cmd: *mut Anon18 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon18;
    if (*sprite).data[0] == 0 {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + (*cmd).initialX;
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + (*cmd).initialY;
        (*sprite).data[1] = (*cmd).velocityX;
        (*sprite).data[2] = (*cmd).waveAmplitude;
        (*sprite).data[3] = (*cmd).duration;
    }
    (*sprite).data[0] += 1;
    (*sprite).x2 = (*sprite).data[1] * (*sprite).data[0];
    (*sprite).y2 = Sin(((*sprite).data[0] * 20) & 0xFF, (*sprite).data[2]);
    if (*sprite).data[0] > (*sprite).data[3] {
        DestroyAnimSprite(sprite);
    }
}
unsafe fn InitItemBagData(sprite: *mut Sprite, mut c: i16) {
    let a: i32 = ((*sprite).x as i32) << 8 | (*sprite).y as i32;
    let b: i32 = ((*sprite).data[6] as i32) << 8 | (*sprite).data[7] as i32;
    c <<= 8;
    (*sprite).data[5] = a as i16;
    (*sprite).data[6] = b as i16;
    (*sprite).data[7] = c;
}
pub unsafe fn moveAlongLinearPath(sprite: *mut Sprite) -> u8 {
    let xStartPos: u16 = ((*sprite).data[5] >> 8) as u8 as u16;
    let yStartPos: u16 = (*sprite).data[5] as u8 as u16;
    let mut xEndPos: i32 = ((*sprite).data[6] >> 8) as u8 as i32;
    let yEndPos: i32 = (*sprite).data[6] as u8 as i32;
    let totalTime: i16 = (*sprite).data[7] >> 8;
    let mut currentTime: i16 = (*sprite).data[7] & 0xFF;
    if xEndPos == 0 {
        xEndPos = -32;
    } else if xEndPos == 255 {
        xEndPos = 272;
    }
    let yEndPos_2: i16 = yEndPos as i16 - yStartPos as i16;
    let r0: i16 = xEndPos as i16 - xStartPos as i16;
    let var1: i32 = div_i32(r0 as i32 * currentTime as i32, totalTime as i32);
    let vaxEndPos: i32 = div_i32(yEndPos_2 as i32 * currentTime as i32, totalTime as i32);
    (*sprite).x = var1 as i16 + xStartPos as i16;
    (*sprite).y = vaxEndPos as i16 + yStartPos as i16;
    if ({
        currentTime += 1;
        currentTime
    }) == totalTime
    {
        return TRUE;
    }
    (*sprite).data[7] = totalTime << 8 | currentTime;
    FALSE
}
pub(crate) unsafe fn AnimItemSteal_Step2(sprite: *mut Sprite) {
    if (*sprite).data[0] == 10 {
        StartSpriteAffineAnim(sprite, 1);
    }
    (*sprite).data[0] += 1;
    if (*sprite).data[0] > 50 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimItemSteal_Step1(sprite: *mut Sprite) {
    (*sprite).data[0] += div_i32((*sprite).data[3] as i32 * 128, (*sprite).data[4] as i32) as i16;
    if (*sprite).data[0] >= 128 {
        (*sprite).data[1] += 1;
        (*sprite).data[0] = 0;
    }
    (*sprite).y2 = Sin((*sprite).data[0] + 128, 30 - (*sprite).data[1] * 8);
    if moveAlongLinearPath(sprite) != 0 {
        (*sprite).y2 = 0;
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(AnimItemSteal_Step2);
    }
}
pub(crate) unsafe fn AnimPresent(sprite: *mut Sprite) {
    let cmd: *mut Anon19 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon19;
    InitSpritePosToAnimAttacker(sprite, FALSE);
    let targetX: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
    let targetY: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    if gBattleAnimAttacker as i32 ^ 2 == gBattleAnimTarget as i32 {
        (*sprite).data[6] = targetX;
        (*sprite).data[7] = targetY + 10;
        InitItemBagData(sprite, 60);
        (*sprite).data[3] = 1;
    } else {
        (*sprite).data[6] = targetX;
        (*sprite).data[7] = targetY + 10;
        InitItemBagData(sprite, 60);
        (*sprite).data[3] = 3;
    }
    (*sprite).data[4] = 60;
    (*sprite).callback = Some(AnimItemSteal_Step1);
}
pub(crate) unsafe fn AnimKnockOffOpponentsItem(sprite: *mut Sprite) {
    (*sprite).data[0] += div_i32((*sprite).data[3] as i32 * 128, (*sprite).data[4] as i32) as i16;
    if (*sprite).data[0] > 0x7F {
        (*sprite).data[1] += 1;
        (*sprite).data[0] = 0;
    }
    (*sprite).y2 = Sin((*sprite).data[0] + 0x80, 30 - (*sprite).data[1] * 8);
    if moveAlongLinearPath(sprite) != 0 {
        (*sprite).y2 = 0;
        (*sprite).data[0] = 0;
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimKnockOffItem(sprite: *mut Sprite) {
    let targetY: i16 = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).data[6] = 0;
        (*sprite).data[7] = targetY + 10;
        InitItemBagData(sprite, 40);
        (*sprite).data[3] = 3;
        (*sprite).data[4] = 60;
        (*sprite).callback = Some(AnimItemSteal_Step1);
    } else {
        (*sprite).data[6] = 255;
        (*sprite).data[7] = targetY + 10;
        if IsContest() != 0 {
            (*sprite).data[6] = 0;
        }
        InitItemBagData(sprite, 40);
        (*sprite).data[3] = 3;
        (*sprite).data[4] = 60;
        (*sprite).callback = Some(AnimKnockOffOpponentsItem);
    }
}
pub(crate) unsafe fn AnimPresentHealParticle(sprite: *mut Sprite) {
    let cmd: *mut Anon20 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon20;
    if (*sprite).data[0] == 0 {
        InitSpritePosToAnimTarget(sprite, FALSE);
        (*sprite).data[1] = (*cmd).velocityY;
    }
    (*sprite).data[0] += 1;
    (*sprite).y2 = (*sprite).data[1] * (*sprite).data[0];
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimItemSteal(sprite: *mut Sprite) {
    let cmd: *mut Anon21 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon21;
    InitSpritePosToAnimTarget(sprite, FALSE);
    let attackerX: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16;
    let attackerY: i16 = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16;
    if gBattleAnimTarget as i32 ^ 2 == gBattleAnimAttacker as i32 {
        (*sprite).data[6] = attackerX;
        (*sprite).data[7] = attackerY + 10;
        InitItemBagData(sprite, 60);
        (*sprite).data[3] = 1;
    } else {
        (*sprite).data[6] = attackerX;
        (*sprite).data[7] = attackerY + 10;
        InitItemBagData(sprite, 60);
        (*sprite).data[3] = 3;
    }
    (*sprite).data[4] = 60;
    (*sprite).callback = Some(AnimItemSteal_Step3);
}
pub(crate) unsafe fn AnimItemSteal_Step3(sprite: *mut Sprite) {
    (*sprite).data[0] += div_i32((*sprite).data[3] as i32 * 128, (*sprite).data[4] as i32) as i16;
    if (*sprite).data[0] > 127 {
        (*sprite).data[1] += 1;
        (*sprite).data[0] = 0;
    }
    (*sprite).y2 = Sin((*sprite).data[0] + 0x80, 30 - (*sprite).data[1] * 8);
    if (*sprite).y2 == 0 {
        PlaySE12WithPanning(SE_M_BUBBLE2, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
    }
    if moveAlongLinearPath(sprite) != 0 {
        (*sprite).y2 = 0;
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(AnimItemSteal_Step2);
        PlaySE12WithPanning(SE_M_BUBBLE2, BattleAnimAdjustPanning(SOUND_PAN_ATTACKER));
    }
}
pub(crate) unsafe fn AnimTrickBag(sprite: *mut Sprite) {
    let cmd: *mut Anon22 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon22;
    if (*sprite).data[0] == 0 {
        if IsContest() == 0 {
            (*sprite).data[1] = (*cmd).waveOffset;
            (*sprite).x = 120;
        } else {
            (*sprite).data[1] = (((*cmd).waveOffset as i32 - 32) % 256) as i16;
            (*sprite).x = 70;
        }
        (*sprite).y = (*cmd).initialY;
        (*sprite).data[2] = (*cmd).initialY;
        (*sprite).data[4] = 20;
        (*sprite).x2 = Cos((*sprite).data[1], 60);
        (*sprite).y2 = Sin((*sprite).data[1], 20);
        (*sprite).callback = Some(AnimTrickBag_Step1);
        if (*sprite).data[1] > 0 && (*sprite).data[1] < 192 {
            (*sprite).subpriority = 31;
        } else {
            (*sprite).subpriority = 29;
        }
    }
}
pub(crate) unsafe fn AnimTrickBag_Step1(sprite: *mut Sprite) {
    'l1: {
        match (*sprite).data[3] {
            0 => {
                if (*sprite).data[2] > 78 {
                    (*sprite).data[3] = 1;
                    StartSpriteAffineAnim(sprite, 1);
                    break 'l1;
                } else {
                    (*sprite).data[2] += (*sprite).data[4] / 10;
                    (*sprite).data[4] += 3;
                    (*sprite).y = (*sprite).data[2];
                    break 'l1;
                }
            }
            1 if (*sprite).data[3] != 0 && (*sprite).affineAnimEnded() != 0 => {
                (*sprite).data[0] = 0;
                (*sprite).data[2] = 0;
                (*sprite).callback = Some(AnimTrickBag_Step2);
            }
            _ => {}
        }
    }
}
pub(crate) unsafe fn AnimTrickBag_Step2(sprite: *mut Sprite) {
    if (*sprite).data[2] == gTrickBagCoordinates[(*sprite).data[0]][1] as i16 {
        if gTrickBagCoordinates[(*sprite).data[0]][2] == 127 {
            (*sprite).data[0] = 0;
            (*sprite).callback = Some(AnimTrickBag_Step3);
        }
        (*sprite).data[2] = 0;
        (*sprite).data[0] += 1;
    } else {
        (*sprite).data[2] += 1;
        (*sprite).data[1] = (gTrickBagCoordinates[(*sprite).data[0]][0] as i16
            * gTrickBagCoordinates[(*sprite).data[0]][2] as i16
            + (*sprite).data[1])
            & 0xFF;
        if IsContest() == 0 {
            if (*sprite).data[1] as u16 as i32 - 1 < 191 {
                (*sprite).subpriority = 31;
            } else {
                (*sprite).subpriority = 29;
            }
        }
        (*sprite).x2 = Cos((*sprite).data[1], 60);
        (*sprite).y2 = Sin((*sprite).data[1], 20);
    }
}
pub(crate) unsafe fn AnimTrickBag_Step3(sprite: *mut Sprite) {
    if (*sprite).data[0] > 20 {
        DestroyAnimSprite(sprite);
    }
    (*sprite).set_invisible(((*sprite).data[0] % 2) as u16);
    (*sprite).data[0] += 1;
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_LeafBlade(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[4] = GetBattlerSpriteSubpriority(gBattleAnimTarget) as i16 - 1;
    (*task).data[6] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*task).data[7] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    (*task).data[10] = GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_WIDTH);
    (*task).data[11] = GetBattlerSpriteCoordAttr(gBattleAnimTarget, BATTLER_COORD_ATTR_HEIGHT);
    (*task).data[5] = (if GetBattlerSide(gBattleAnimTarget) == 1 {
        1
    } else {
        -1
    }) as i16;
    (*task).data[9] = 56 - (*task).data[5] * 64;
    (*task).data[8] = (*task).data[7] - (*task).data[9] + (*task).data[6];
    (*task).data[2] = CreateSprite(
        (&raw const *gLeafBladeSpriteTemplate).cast_mut(),
        (*task).data[8],
        (*task).data[9],
        (*task).data[4] as u8,
    ) as i16;
    if (*task).data[2] == MAX_SPRITES as i16 {
        DestroyAnimVisualTask(taskId);
    }
    gSprites[(*task).data[2]].data[0] = 10;
    gSprites[(*task).data[2]].data[1] = (*task).data[8];
    gSprites[(*task).data[2]].data[2] =
        (*task).data[6] - ((*task).data[10] / 2 + 10) * (*task).data[5];
    gSprites[(*task).data[2]].data[3] = (*task).data[9];
    gSprites[(*task).data[2]].data[4] =
        (*task).data[7] + ((*task).data[11] / 2 + 10) * (*task).data[5];
    gSprites[(*task).data[2]].data[5] = LeafBladeGetPosFactor(&raw mut gSprites[(*task).data[2]]);
    InitAnimArcTranslation(&raw mut gSprites[(*task).data[2]]);
    (*task).func = Some(AnimTask_LeafBlade_Step);
}
pub(crate) unsafe fn AnimTask_LeafBlade_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    let sprite: *mut Sprite = &raw mut gSprites[(*task).data[2]];
    let a: i32 = (*task).data[0] as i32;
    'l1: {
        match a {
            4 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    (*task).data[15] = 5;
                    (*task).data[0] = 0xFF;
                }
            }
            8 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    (*task).data[15] = 9;
                    (*task).data[0] = 0xFF;
                }
            }
            0 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    (*task).data[15] = 1;
                    (*task).data[0] = 0xFF;
                }
            }
            1 => {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).x2 = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] = 10;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[6];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[7];
                (*sprite).data[5] = LeafBladeGetPosFactor(sprite);
                (*task).data[4] += 2;
                (*task).data[3] = a as i16;
                (*sprite).subpriority = (*task).data[4] as u8;
                StartSpriteAnim(sprite, (*task).data[3] as u8);
                InitAnimArcTranslation(sprite);
                (*task).data[0] += 1;
            }
            2 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    (*task).data[15] = 3;
                    (*task).data[0] = 0xFF;
                }
            }
            3 => {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).x2 = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] = 10;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[6] - ((*task).data[10] / 2 + 10) * (*task).data[5];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[7] - ((*task).data[11] / 2 + 10) * (*task).data[5];
                (*sprite).data[5] = LeafBladeGetPosFactor(sprite);
                (*task).data[3] = 2;
                (*sprite).subpriority = (*task).data[4] as u8;
                StartSpriteAnim(sprite, (*task).data[3] as u8);
                InitAnimArcTranslation(sprite);
                (*task).data[0] += 1;
            }
            5 => {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).x2 = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] = 10;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[6] + ((*task).data[10] / 2 + 10) * (*task).data[5];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[7] + ((*task).data[11] / 2 + 10) * (*task).data[5];
                (*sprite).data[5] = LeafBladeGetPosFactor(sprite);
                (*task).data[4] -= 2;
                (*task).data[3] = 3;
                (*sprite).subpriority = (*task).data[4] as u8;
                StartSpriteAnim(sprite, (*task).data[3] as u8);
                InitAnimArcTranslation(sprite);
                (*task).data[0] += 1;
            }
            6 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    (*task).data[15] = 7;
                    (*task).data[0] = 0xFF;
                }
            }
            7 => {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).x2 = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] = 10;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[6];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[7];
                (*sprite).data[5] = LeafBladeGetPosFactor(sprite);
                (*task).data[4] += 2;
                (*task).data[3] = 4;
                (*sprite).subpriority = (*task).data[4] as u8;
                StartSpriteAnim(sprite, (*task).data[3] as u8);
                InitAnimArcTranslation(sprite);
                (*task).data[0] += 1;
            }
            9 => {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).x2 = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] = 10;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[6] - ((*task).data[10] / 2 + 10) * (*task).data[5];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[7] + ((*task).data[11] / 2 + 10) * (*task).data[5];
                (*sprite).data[5] = LeafBladeGetPosFactor(sprite);
                (*task).data[3] = 5;
                (*sprite).subpriority = (*task).data[4] as u8;
                StartSpriteAnim(sprite, (*task).data[3] as u8);
                InitAnimArcTranslation(sprite);
                (*task).data[0] += 1;
            }
            10 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    (*task).data[15] = 11;
                    (*task).data[0] = 0xFF;
                }
            }
            11 => {
                (*sprite).x += (*sprite).x2;
                (*sprite).y += (*sprite).y2;
                (*sprite).x2 = 0;
                (*sprite).y2 = 0;
                (*sprite).data[0] = 10;
                (*sprite).data[1] = (*sprite).x;
                (*sprite).data[2] = (*task).data[8];
                (*sprite).data[3] = (*sprite).y;
                (*sprite).data[4] = (*task).data[9];
                (*sprite).data[5] = LeafBladeGetPosFactor(sprite);
                (*task).data[4] -= 2;
                (*task).data[3] = 6;
                (*sprite).subpriority = (*task).data[4] as u8;
                StartSpriteAnim(sprite, (*task).data[3] as u8);
                InitAnimArcTranslation(sprite);
                (*task).data[0] += 1;
                break 'l1;
            }
            12 => {
                AnimTask_LeafBlade_Step2(task, taskId);
                if TranslateAnimHorizontalArc(sprite) != 0 {
                    DestroySprite(sprite);
                    (*task).data[0] += 1;
                }
            }
            13 => {
                if (*task).data[12] == 0 {
                    DestroyAnimVisualTask(taskId);
                }
            }
            255 if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 5 =>
            {
                (*task).data[1] = 0;
                (*task).data[0] = (*task).data[15];
            }
            _ => {}
        }
    }
}
unsafe fn LeafBladeGetPosFactor(sprite: *mut Sprite) -> i16 {
    let mut var: i16 = 8;
    if (*sprite).data[4] < (*sprite).y {
        var = -var;
    }
    var
}
unsafe fn AnimTask_LeafBlade_Step2(task: *mut Task, taskId: u8) {
    (*task).data[14] += 1;
    if (*task).data[14] > 0 {
        (*task).data[14] = 0;
        let spriteX: i16 = gSprites[(*task).data[2]].x + gSprites[(*task).data[2]].x2;
        let spriteY: i16 = gSprites[(*task).data[2]].y + gSprites[(*task).data[2]].y2;
        let spriteId: u8 = CreateSprite(
            (&raw const *gLeafBladeSpriteTemplate).cast_mut(),
            spriteX,
            spriteY,
            (*task).data[4] as u8,
        );
        if spriteId != MAX_SPRITES {
            gSprites[spriteId].data[6] = taskId as i16;
            gSprites[spriteId].data[7] = 12;
            task_set(taskId, 12, task_get(taskId, 12) + 1);
            gSprites[spriteId].data[0] = (*task).data[13] & 1;
            task_set(taskId, 13, task_get(taskId, 13) + 1);
            StartSpriteAnim(&raw mut gSprites[spriteId], (*task).data[3] as u8);
            gSprites[spriteId].subpriority = (*task).data[4] as u8;
            gSprites[spriteId].callback = Some(AnimTask_LeafBlade_Step2_Callback);
        }
    }
}
pub(crate) unsafe fn AnimTask_LeafBlade_Step2_Callback(sprite: *mut Sprite) {
    (*sprite).data[0] += 1;
    if (*sprite).data[0] > 1 {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
        (*sprite).data[1] += 1;
        if (*sprite).data[1] > 8 {
            task_set(
                (*sprite).data[6],
                (*sprite).data[7],
                task_get((*sprite).data[6], (*sprite).data[7]) - 1,
            );
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimFlyingParticle(sprite: *mut Sprite) {
    let cmd: *mut Anon23 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon23;
    let mut battler: u8 = 0;
    if (*cmd).unk6 == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if GetBattlerSide(battler) != B_SIDE_PLAYER {
        (*sprite).data[4] = 0;
        (*sprite).data[2] = (*cmd).unk3;
        (*sprite).x = -16;
    } else {
        (*sprite).data[4] = 1;
        (*sprite).data[2] = -(*cmd).unk3;
        (*sprite).x = 256;
    }
    (*sprite).data[1] = (*cmd).unk1;
    (*sprite).data[0] = (*cmd).unk2;
    (*sprite).data[3] = (*cmd).unk4;
    match (*cmd).unk5 {
        0 => {
            (*sprite).y = (*cmd).unk0;
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(battler) as u16);
        }
        1 => {
            (*sprite).y = (*cmd).unk0;
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(battler) as u16 + 1);
        }
        2 => {
            (*sprite).y =
                GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).unk0;
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(battler) as u16);
        }
        3 => {
            (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET)
                as i16
                + (*cmd).unk0;
            GetAnimBattlerSpriteId(ANIM_TARGET);
            (*sprite)
                .oam
                .set_priority(GetBattlerSpriteBGPriority(battler) as u16 + 1);
        }
        _ => {}
    }
    (*sprite).callback = Some(AnimFlyingParticle_Step);
}
pub(crate) unsafe fn AnimFlyingParticle_Step(sprite: *mut Sprite) {
    let a: i32 = (*sprite).data[7] as i32;
    (*sprite).data[7] += 1;
    (*sprite).y2 = (((*sprite).data[1] as i32
        * (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*sprite).data[0]]
            as i32)
        >> 8) as i16;
    (*sprite).x2 = (*sprite).data[2] * a as i16;
    (*sprite).data[0] = ((*sprite).data[3] * a as i16) & 0xFF;
    if (*sprite).data[4] == 0 {
        if ((*sprite).x2 as i32 + (*sprite).x as i32) < 248 {
            return;
        }
    } else {
        if (*sprite).x2 as i32 + (*sprite).x as i32 > -16 {
            return;
        }
    }
    DestroySpriteAndMatrix(sprite);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_CycleMagicalLeafPal(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            (*task).data[8] = 0x100 + IndexOfSpritePaletteTag(ANIM_TAG_LEAF) as i16 * 16;
            (*task).data[12] = 0x100 + IndexOfSpritePaletteTag(ANIM_TAG_RAZOR_LEAF) as i16 * 16;
            (*task).data[0] += 1;
        }
        1 if ({
            (*task).data[9] += 1;
            (*task).data[9]
        }) >= 0 =>
        {
            (*task).data[9] = 0;
            BlendPalette(
                (*task).data[8] as u16,
                16,
                (*task).data[10] as u8,
                gMagicalLeafBlendColors[(*task).data[11]],
            );
            BlendPalette(
                (*task).data[12] as u16,
                16,
                (*task).data[10] as u8,
                gMagicalLeafBlendColors[(*task).data[11]],
            );
            if ({
                (*task).data[10] += 1;
                (*task).data[10]
            }) == 17
            {
                (*task).data[10] = 0;
                if ({
                    (*task).data[11] += 1;
                    (*task).data[11]
                }) == 7
                {
                    (*task).data[11] = 0;
                }
            }
        }
        _ => {}
    }
    if gBattleAnimArgs[7] == -1 {
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimNeedleArmSpike(sprite: *mut Sprite) {
    let cmd: *mut Anon24 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon24;
    let mut a: u8 = 0;
    let mut b: u8 = 0;
    let mut c: u16 = 0;
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    if (*cmd).unk4 == 0 {
        DestroyAnimSprite(sprite);
    } else {
        if (*cmd).unk0 == 0 {
            a = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2);
            b = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET);
        } else {
            a = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2);
            b = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET);
        }
        (*sprite).data[0] = (*cmd).unk4;
        if (*cmd).unk1 == 0 {
            (*sprite).x = (*cmd).unk2 + a as i16;
            (*sprite).y = (*cmd).unk3 + b as i16;
            (*sprite).data[5] = a as i16;
            (*sprite).data[6] = b as i16;
        } else {
            (*sprite).x = a as i16;
            (*sprite).y = b as i16;
            (*sprite).data[5] = (*cmd).unk2 + a as i16;
            (*sprite).data[6] = (*cmd).unk3 + b as i16;
        }
        x = (*sprite).x as u16;
        (*sprite).data[1] = x as i16 * 16;
        y = (*sprite).y as u16;
        (*sprite).data[2] = y as i16 * 16;
        (*sprite).data[3] = div_i32(
            ((*sprite).data[5] as i32 - (*sprite).x as i32) * 16,
            (*cmd).unk4 as i32,
        ) as i16;
        (*sprite).data[4] = div_i32(
            ((*sprite).data[6] as i32 - (*sprite).y as i32) * 16,
            (*cmd).unk4 as i32,
        ) as i16;
        c = ArcTan2Neg((*sprite).data[5] - x as i16, (*sprite).data[6] - y as i16);
        if IsContest() != 0 {
            c -= 0x8000;
        }
        TrySetSpriteRotScale(sprite, FALSE, 0x100, 0x100, c);
        (*sprite).callback = Some(AnimNeedleArmSpike_Step);
    }
}
pub(crate) unsafe fn AnimNeedleArmSpike_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[1] += (*sprite).data[3];
        (*sprite).data[2] += (*sprite).data[4];
        (*sprite).x = (*sprite).data[1] >> 4;
        (*sprite).y = (*sprite).data[2] >> 4;
        (*sprite).data[0] -= 1;
    } else {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe fn AnimWhipHit_WaitEnd(sprite: *mut Sprite) {
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSlidingHit(sprite: *mut Sprite) {
    let cmd: *mut Anon25 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon25;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= (*cmd).unk0;
        (*sprite).y += (*cmd).unk1;
    } else {
        (*sprite).x += (*cmd).unk0;
        (*sprite).y += (*cmd).unk1;
    }
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub(crate) unsafe fn AnimWhipHit(sprite: *mut Sprite) {
    let cmd: *mut Anon26 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon26;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        StartSpriteAnim(sprite, 1);
    }
    (*sprite).callback = Some(AnimWhipHit_WaitEnd);
    SetAnimSpriteInitialXOffset(sprite, (*cmd).unk0);
    (*sprite).y += (*cmd).unk1;
}
pub(crate) unsafe fn AnimFlickeringPunch(sprite: *mut Sprite) {
    let cmd: *mut Anon27 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon27;
    (*sprite).x += (*cmd).unk0;
    (*sprite).y += (*cmd).unk1;
    (*sprite).data[0] = (*cmd).unk2;
    (*sprite).data[1] = (*cmd).unk3;
    (*sprite).data[3] = (*cmd).unk4;
    (*sprite).data[5] = (*cmd).unk5;
    StartSpriteAffineAnim(sprite, (*cmd).unk6 as u8);
    StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    (*sprite).callback = Some(TranslateSpriteLinearAndFlicker);
}
pub(crate) unsafe fn AnimCuttingSlice(sprite: *mut Sprite) {
    let cmd: *mut Anon28 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon28;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).y += 8;
    }
    (*sprite).callback = Some(AnimSlice_Step);
    if (*cmd).unk2 == 0 {
        (*sprite).x += (*cmd).unk0;
    } else {
        (*sprite).x -= (*cmd).unk0;
        (*sprite).set_hFlip(1);
    }
    (*sprite).y += (*cmd).unk1;
    (*sprite).data[1] -= 0x400;
    (*sprite).data[2] += 0x400;
    (*sprite).data[5] = (*cmd).unk2;
    if (*sprite).data[5] == 1 {
        (*sprite).data[1] = -(*sprite).data[1];
    }
}
pub(crate) unsafe fn AnimAirCutterSlice(sprite: *mut Sprite) {
    let cmd: *mut Anon29 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon29;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    match (*cmd).unk3 {
        1 => {
            x = GetBattlerSpriteCoord(gBattleAnimTarget ^ 2, BATTLER_COORD_X);
            y = GetBattlerSpriteCoord(gBattleAnimTarget ^ 2, BATTLER_COORD_Y);
        }
        2 => {
            x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X);
            y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y);
            if IsBattlerSpriteVisible(gBattleAnimTarget ^ 2) != 0 {
                x = ((GetBattlerSpriteCoord(gBattleAnimTarget ^ 2, BATTLER_COORD_X) as i32
                    + x as i32)
                    / 2) as u8;
                y = ((GetBattlerSpriteCoord(gBattleAnimTarget ^ 2, BATTLER_COORD_Y) as i32
                    + y as i32)
                    / 2) as u8;
            }
        }
        _ => {
            x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X);
            y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y);
        }
    }
    (*sprite).x = x as i16;
    (*sprite).y = y as i16;
    if GetBattlerSide(gBattleAnimTarget) == B_SIDE_PLAYER {
        (*sprite).y += 8;
    }
    (*sprite).callback = Some(AnimSlice_Step);
    if (*cmd).unk2 == 0 {
        (*sprite).x += (*cmd).unk0;
    } else {
        (*sprite).x -= (*cmd).unk0;
        (*sprite).set_hFlip(1);
    }
    (*sprite).y += (*cmd).unk1;
    (*sprite).data[1] -= 0x400;
    (*sprite).data[2] += 0x400;
    (*sprite).data[5] = (*cmd).unk2;
    if (*sprite).data[5] == 1 {
        (*sprite).data[1] = -(*sprite).data[1];
    }
}
pub(crate) unsafe fn AnimSlice_Step(sprite: *mut Sprite) {
    (*sprite).data[3] += (*sprite).data[1];
    (*sprite).data[4] += (*sprite).data[2];
    if (*sprite).data[5] == 0 {
        (*sprite).data[1] += 0x18;
    } else {
        (*sprite).data[1] -= 0x18;
    }
    (*sprite).data[2] -= 0x18;
    (*sprite).x2 = (*sprite).data[3] >> 8;
    (*sprite).y2 = (*sprite).data[4] >> 8;
    (*sprite).data[0] += 1;
    if (*sprite).data[0] == 20 {
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        (*sprite).data[0] = 3;
        (*sprite).callback = Some(WaitAnimForDuration);
    }
}
unsafe fn UnusedFlickerAnim(sprite: *mut Sprite) {
    if (*sprite).data[2] > 1 {
        if (*sprite).data[3] as i32 & 1 != 0 {
            (*sprite).set_invisible(FALSE as u16);
            gSprites[(*sprite).data[0]].set_invisible(0);
            gSprites[(*sprite).data[1]].set_invisible(FALSE as u16);
        } else {
            (*sprite).set_invisible(TRUE as u16);
            gSprites[(*sprite).data[0]].set_invisible(TRUE as u16);
            gSprites[(*sprite).data[1]].set_invisible(1);
        }
        (*sprite).data[2] = 0;
        (*sprite).data[3] += 1;
    } else {
        (*sprite).data[2] += 1;
    }
    if (*sprite).data[3] == 10 {
        DestroySprite(&raw mut gSprites[(*sprite).data[0]]);
        DestroySprite(&raw mut gSprites[(*sprite).data[1]]);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimCirclingMusicNote(sprite: *mut Sprite) {
    let cmd: *mut Anon30 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon30;
    (*sprite).data[0] = (*cmd).unk2;
    if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
        (*sprite).x -= (*cmd).unk0;
    } else {
        (*sprite).x += (*cmd).unk0;
    }
    StartSpriteAnim(sprite, (*cmd).unk5 as u8);
    (*sprite).data[1] = -(*cmd).unk3;
    (*sprite).y += (*cmd).unk1;
    (*sprite).data[3] = (*cmd).unk4;
    (*sprite).callback = Some(AnimCirclingMusicNote_Step);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimCirclingMusicNote_Step(sprite: *mut Sprite) {
    (*sprite).x2 = Cos((*sprite).data[0], 100);
    (*sprite).y2 = Sin((*sprite).data[0], 20);
    if (*sprite).data[0] < 128 {
        (*sprite).subpriority = 0;
    } else {
        (*sprite).subpriority = 14;
    }
    (*sprite).data[0] = ((*sprite).data[0] + (*sprite).data[1]) & 0xFF;
    (*sprite).data[5] += 130;
    (*sprite).y2 += (*sprite).data[5] >> 8;
    (*sprite).data[2] += 1;
    if (*sprite).data[2] == (*sprite).data[3] {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimProtect(sprite: *mut Sprite) {
    let cmd: *mut Anon31 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon31;
    if IsContest() != 0 {
        (*cmd).unk1 += 8;
    }
    (*sprite).x = GetBattlerSpriteCoord2(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + (*cmd).unk0;
    (*sprite).y = GetBattlerSpriteCoord2(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + (*cmd).unk1;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER || IsContest() != 0 {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u16 + 1);
    } else {
        (*sprite)
            .oam
            .set_priority(GetBattlerSpriteBGPriority(gBattleAnimAttacker) as u16);
    }
    (*sprite).data[0] = (*cmd).unk2;
    (*sprite).data[2] = 0x100 + IndexOfSpritePaletteTag(ANIM_TAG_PROTECT) as i16 * 16;
    (*sprite).data[7] = 16;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        (((*sprite).data[7] as u16) << 8) | (16 - (*sprite).data[7] as u16),
    );
    (*sprite).callback = Some(AnimProtect_Step);
}
pub(crate) unsafe fn AnimProtect_Step(sprite: *mut Sprite) {
    let mut i: i32 = 0;
    let mut id: i32 = 0;
    let mut savedPal: i32 = 0;
    (*sprite).data[5] += 96;
    (*sprite).x2 = -((*sprite).data[5] >> 8);
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        (*sprite).data[1] = 0;
        savedPal = gPlttBufferFaded[(*sprite).data[2] as i32 + 1] as i32;
        i = 0;
        while i < 6 {
            id = (*sprite).data[2] as i32
                + ({
                    i += 1;
                    i
                });
            gPlttBufferFaded[id] = gPlttBufferFaded[id + 1];
        }
        gPlttBufferFaded[(*sprite).data[2] as i32 + 7] = savedPal as u16;
    }
    if (*sprite).data[7] > 6
        && (*sprite).data[0] > 0
        && ({
            (*sprite).data[6] += 1;
            (*sprite).data[6]
        }) > 1
    {
        (*sprite).data[6] = 0;
        (*sprite).data[7] -= 1;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (((*sprite).data[7] as u16) << 8) | (16 - (*sprite).data[7] as u16),
        );
    }
    if (*sprite).data[0] > 0 {
        (*sprite).data[0] -= 1;
    } else if ({
        (*sprite).data[6] += 1;
        (*sprite).data[6]
    }) > 1
    {
        (*sprite).data[6] = 0;
        (*sprite).data[7] += 1;
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (((*sprite).data[7] as u16) << 8) | (16 - (*sprite).data[7] as u16),
        );
        if (*sprite).data[7] == 16 {
            (*sprite).set_invisible(TRUE as u16);
            (*sprite).callback = Some(DestroyAnimSpriteAndDisableBlend);
        }
    }
}
pub(crate) unsafe fn AnimMilkBottle(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + -24;
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[6] = 0;
    (*sprite).data[7] = 16;
    SetGpuReg(REG_OFFSET_BLDCNT, 16192);
    SetGpuReg(
        REG_OFFSET_BLDALPHA,
        ((*sprite).data[7] as u16) << 8 | (*sprite).data[6] as u16,
    );
    (*sprite).callback = Some(AnimMilkBottle_Step1);
}
pub(crate) unsafe fn AnimMilkBottle_Step1(sprite: *mut Sprite) {
    'l1: {
        match (*sprite).data[0] {
            0 => {
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
                }) > 0
                {
                    (*sprite).data[2] = 0;
                    if ({
                        (*sprite).data[1] += 1;
                        (*sprite).data[1]
                    }) as i32
                        & 1
                        != 0
                    {
                        if (*sprite).data[6] <= 15 {
                            (*sprite).data[6] += 1;
                        }
                    } else if (*sprite).data[7] > 0 {
                        (*sprite).data[7] -= 1;
                    }
                    SetGpuReg(
                        REG_OFFSET_BLDALPHA,
                        ((*sprite).data[7] as u16) << 8 | (*sprite).data[6] as u16,
                    );
                    if (*sprite).data[6] == 16 && (*sprite).data[7] == 0 {
                        (*sprite).data[1] = 0;
                        (*sprite).data[0] += 1;
                    }
                }
            }
            1 => {
                if ({
                    (*sprite).data[1] += 1;
                    (*sprite).data[1]
                }) > 8
                {
                    (*sprite).data[1] = 0;
                    StartSpriteAffineAnim(sprite, 1);
                    (*sprite).data[0] += 1;
                }
            }
            2 => {
                AnimMilkBottle_Step2(sprite, 16, 4);
                if ({
                    (*sprite).data[1] += 1;
                    (*sprite).data[1]
                }) > 2
                {
                    (*sprite).data[1] = 0;
                    (*sprite).y += 1;
                }
                if ({
                    (*sprite).data[2] += 1;
                    (*sprite).data[2]
                }) <= 29
                {
                    break 'l1;
                }
                if (*sprite).data[2] as i32 & 1 != 0 {
                    if (*sprite).data[6] > 0 {
                        (*sprite).data[6] -= 1;
                    }
                } else if (*sprite).data[7] <= 15 {
                    (*sprite).data[7] += 1;
                }
                SetGpuReg(
                    REG_OFFSET_BLDALPHA,
                    ((*sprite).data[7] as u16) << 8 | (*sprite).data[6] as u16,
                );
                if (*sprite).data[6] == 0 && (*sprite).data[7] == 16 {
                    (*sprite).data[1] = 0;
                    (*sprite).data[2] = 0;
                    (*sprite).data[0] += 1;
                }
            }
            3 => {
                (*sprite).set_invisible(TRUE as u16);
                (*sprite).data[0] += 1;
            }
            4 => {
                SetGpuReg(REG_OFFSET_BLDCNT, 0);
                SetGpuReg(REG_OFFSET_BLDALPHA, 0);
                DestroyAnimSprite(sprite);
            }
            _ => {}
        }
    }
}
unsafe fn AnimMilkBottle_Step2(sprite: *mut Sprite, unk1: i32, unk2: i32) {
    if (*sprite).data[3] <= 11 {
        (*sprite).data[4] += 2;
    }
    if (*sprite).data[3] as u16 as i32 - 0x12 <= 0x17 {
        (*sprite).data[4] -= 2;
    }
    if (*sprite).data[3] > 0x2F {
        (*sprite).data[4] += 2;
    }
    (*sprite).x2 = (*sprite).data[4] / 9;
    (*sprite).y2 = (*sprite).data[4] / 14;
    if (*sprite).y2 < 0 {
        (*sprite).y2 *= -1;
    }
    (*sprite).data[3] += 1;
    if (*sprite).data[3] > 0x3B {
        (*sprite).data[3] = 0;
    }
}
pub(crate) unsafe fn AnimGrantingStars(sprite: *mut Sprite) {
    let cmd: *mut Anon32 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon32;
    if (*cmd).unk2 == 0 {
        SetSpriteCoordsToAnimAttackerCoords(sprite);
    }
    SetAnimSpriteInitialXOffset(sprite, (*cmd).unk0);
    (*sprite).y += (*cmd).unk1;
    (*sprite).data[0] = (*cmd).unk5;
    (*sprite).data[1] = (*cmd).unk3;
    (*sprite).data[2] = (*cmd).unk4;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
}
pub(crate) unsafe fn AnimSparklingStars(sprite: *mut Sprite) {
    let cmd: *mut Anon33 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon33;
    let mut battler: u8 = 0;
    if (*cmd).unk2 == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    if IsDoubleBattle() != 0 && IsBattlerSpriteVisible(battler ^ 2) != 0 {
        SetAverageBattlerPositions(
            battler,
            (*cmd).unk6 as u8,
            &raw mut (*sprite).x,
            &raw mut (*sprite).y,
        );
        SetAnimSpriteInitialXOffset(sprite, (*cmd).unk0);
        (*sprite).y += (*cmd).unk1;
    } else {
        if (*cmd).unk6 == 0 {
            (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16;
            (*sprite).y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y) as i16 + (*cmd).unk1;
        } else {
            (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X_2) as i16;
            (*sprite).y =
                GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).unk1;
        }
        SetAnimSpriteInitialXOffset(sprite, (*cmd).unk0);
    }
    (*sprite).data[0] = (*cmd).unk5;
    (*sprite).data[1] = (*cmd).unk3;
    (*sprite).data[2] = (*cmd).unk4;
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(TranslateSpriteLinearFixedPoint);
}
pub(crate) unsafe fn AnimBubbleBurst(sprite: *mut Sprite) {
    let cmd: *mut Anon34 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon34;
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).x += (*cmd).unk0;
        (*sprite).y += (*cmd).unk1;
    } else {
        (*sprite).x -= (*cmd).unk0;
        (*sprite).y += (*cmd).unk1;
        StartSpriteAnim(sprite, 1);
    }
    (*sprite).callback = Some(AnimBubbleBurst_Step);
}
pub(crate) unsafe fn AnimBubbleBurst_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 30
    {
        (*sprite).y2 = ((30 - (*sprite).data[0] as i32) / 3) as i16;
        (*sprite).x2 = Sin((*sprite).data[1] * 4, 3);
        (*sprite).data[1] += 1;
    }
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSleepLetterZ(sprite: *mut Sprite) {
    let cmd: *mut Anon35 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon35;
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).x += (*cmd).unk0;
        (*sprite).y += (*cmd).unk1;
        (*sprite).data[3] = 1;
    } else {
        (*sprite).x -= (*cmd).unk0;
        (*sprite).y += (*cmd).unk1;
        (*sprite).data[3] = -1;
        StartSpriteAffineAnim(sprite, 1);
    }
    (*sprite).callback = Some(AnimSleepLetterZ_Step);
}
pub(crate) unsafe fn AnimSleepLetterZ_Step(sprite: *mut Sprite) {
    (*sprite).y2 = -((*sprite).data[0] / 40);
    (*sprite).x2 = (*sprite).data[4] / 10;
    (*sprite).data[4] += (*sprite).data[3] * 2;
    (*sprite).data[0] += (*sprite).data[1];
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 60
    {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe fn AnimLockOnTarget(sprite: *mut Sprite) {
    (*sprite).x -= 32;
    (*sprite).y -= 32;
    (*sprite).data[0] = 20;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step1));
}
pub(crate) unsafe fn AnimLockOnTarget_Step1(sprite: *mut Sprite) {
    match (*sprite).data[5] as i32 & 1 {
        0 => {
            (*sprite).data[0] = 1;
            (*sprite).callback = Some(WaitAnimForDuration);
            StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step1));
        }
        1 => {
            (*sprite).x += (*sprite).x2;
            (*sprite).y += (*sprite).y2;
            (*sprite).y2 = 0;
            (*sprite).x2 = 0;
            (*sprite).data[0] = 8;
            (*sprite).data[2] =
                (*sprite).x + gInclineMonCoordTable[(*sprite).data[5] >> 8][0] as i16;
            (*sprite).data[4] =
                (*sprite).y + gInclineMonCoordTable[(*sprite).data[5] >> 8][1] as i16;
            (*sprite).callback = Some(StartAnimLinearTranslation);
            StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step2));
            (*sprite).data[5] += 0x100;
            PlaySE12WithPanning(SE_M_LOCK_ON, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
        }
        _ => {}
    }
    (*sprite).data[5] ^= 1;
}
pub(crate) unsafe fn AnimLockOnTarget_Step2(sprite: *mut Sprite) {
    if (*sprite).data[5] >> 8 == 4 {
        (*sprite).data[0] = 10;
        (*sprite).callback = Some(WaitAnimForDuration);
        StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step3));
    } else {
        (*sprite).callback = Some(AnimLockOnTarget_Step1);
    }
}
pub(crate) unsafe fn AnimLockOnTarget_Step3(sprite: *mut Sprite) {
    let mut a: i16 = 0;
    let mut b: i16 = 0;
    if (*sprite).oam.affineParam == 0 {
        (*sprite).data[0] = 3;
        (*sprite).data[1] = 0;
        (*sprite).data[2] = 0;
        (*sprite).callback = Some(WaitAnimForDuration);
        StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step4));
    } else {
        match (*sprite).oam.affineParam {
            1 => {
                a = -8;
                b = -8;
            }
            2 => {
                a = -8;
                b = 8;
            }
            3 => {
                a = 8;
                b = -8;
            }
            _ => {
                a = 8;
                b = 8;
            }
        }
        (*sprite).x += (*sprite).x2;
        (*sprite).y += (*sprite).y2;
        (*sprite).y2 = 0;
        (*sprite).x2 = 0;
        (*sprite).data[0] = 6;
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + a;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + b;
        (*sprite).callback = Some(StartAnimLinearTranslation);
        StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step5));
    }
}
pub(crate) unsafe fn AnimLockOnTarget_Step4(sprite: *mut Sprite) {
    if (*sprite).data[2] == 0 {
        if ({
            (*sprite).data[1] += 3;
            (*sprite).data[1]
        }) > 16
        {
            (*sprite).data[1] = 16;
        }
    } else if ({
        (*sprite).data[1] -= 3;
        (*sprite).data[1]
    }) < 0
    {
        (*sprite).data[1] = 0;
    }
    BlendPalettes(
        GetBattlePalettesMask(1, 1, 1, 1, 1, FALSE, FALSE),
        (*sprite).data[1] as u8,
        32767,
    );
    if (*sprite).data[1] == 16 {
        (*sprite).data[2] += 1;
        let pal: i32 = (*sprite).oam.paletteNum() as i32;
        LoadPalette(
            &raw mut (*(&raw const crate::palette::gPlttBufferUnfaded)
                .cast::<CArray<u16, 512>>()
                .cast_mut())[0x100 + pal * 16 + 8] as *mut c_void,
            0x100 + pal as u16 * 16 + 1,
            4,
        );
        PlaySE12WithPanning(SE_M_LEER, BattleAnimAdjustPanning(SOUND_PAN_TARGET));
    } else if (*sprite).data[1] == 0 {
        (*sprite).callback = Some(AnimLockOnTarget_Step5);
    }
}
pub(crate) unsafe fn AnimLockOnTarget_Step5(sprite: *mut Sprite) {
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        (*sprite).data[1] = 0;
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(AnimLockOnTarget_Step6);
    }
}
pub(crate) unsafe fn AnimLockOnTarget_Step6(sprite: *mut Sprite) {
    if (*sprite).data[0] % 3 == 0 {
        (*sprite).data[1] += 1;
        (*sprite).set_invisible((*sprite).invisible() ^ 1);
    }
    (*sprite).data[0] += 1;
    if (*sprite).data[1] == 8 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimLockOnMoveTarget(sprite: *mut Sprite) {
    let cmd: *mut Anon36 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon36;
    (*sprite).oam.affineParam = (*cmd).unk0 as u16;
    if (*cmd).unk0 == 1 {
        (*sprite).x -= 0x18;
        (*sprite).y -= 0x18;
    } else if (*cmd).unk0 == 2 {
        (*sprite).x -= 0x18;
        (*sprite).y += 0x18;
        (*sprite).oam.set_matrixNum(ST_OAM_VFLIP);
    } else if (*cmd).unk0 == 3 {
        (*sprite).x += 0x18;
        (*sprite).y -= 0x18;
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
    } else {
        (*sprite).x += 0x18;
        (*sprite).y += 0x18;
        (*sprite).oam.set_matrixNum(24);
    }
    (*sprite).oam.set_tileNum((*sprite).oam.tileNum() + 16);
    (*sprite).callback = Some(AnimLockOnTarget);
    (*sprite).callback.unwrap_unchecked()(sprite);
}
pub(crate) unsafe fn AnimBowMon(sprite: *mut Sprite) {
    let cmd: *mut Anon37 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon37;
    (*sprite).set_invisible(TRUE as u16);
    (*sprite).data[0] = 0;
    match (*cmd).unk0 {
        0 => {
            (*sprite).callback = Some(AnimBowMon_Step1);
        }
        1 => {
            (*sprite).callback = Some(AnimBowMon_Step2);
        }
        2 => {
            (*sprite).callback = Some(AnimBowMon_Step3);
        }
        _ => {
            (*sprite).callback = Some(AnimBowMon_Step4);
        }
    }
}
pub(crate) unsafe fn AnimBowMon_Step1(sprite: *mut Sprite) {
    (*sprite).data[0] = 6;
    (*sprite).data[1] = (if GetBattlerSide(gBattleAnimAttacker) != 0 {
        2
    } else {
        -2
    }) as i16;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
    StoreSpriteCallbackInData6(sprite, Some(AnimBowMon_Step1_Callback));
    (*sprite).callback = Some(TranslateSpriteLinearById);
}
pub(crate) unsafe fn AnimBowMon_Step1_Callback(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[3] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
        PrepareBattlerSpriteForRotScale((*sprite).data[3] as u8, ST_OAM_OBJ_NORMAL);
        (*sprite).data[4] = (if ({
            (*sprite).data[6] = GetBattlerSide(gBattleAnimAttacker) as i16;
            (*sprite).data[6]
        }) != 0
        {
            0x300
        } else {
            -768
        }) as i16;
        (*sprite).data[5] = 0;
    }
    (*sprite).data[5] += (*sprite).data[4];
    SetSpriteRotScale(
        (*sprite).data[3] as u8,
        0x100,
        0x100,
        (*sprite).data[5] as u16,
    );
    SetBattlerSpriteYOffsetFromRotation((*sprite).data[3] as u8);
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 3
    {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(AnimBowMon_Step4);
    }
}
pub(crate) unsafe fn AnimBowMon_Step2(sprite: *mut Sprite) {
    (*sprite).data[0] = 4;
    (*sprite).data[1] = (if GetBattlerSide(gBattleAnimAttacker) != 0 {
        -3
    } else {
        3
    }) as i16;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
    StoreSpriteCallbackInData6(sprite, Some(AnimBowMon_Step4));
    (*sprite).callback = Some(TranslateSpriteLinearById);
}
pub(crate) unsafe fn AnimBowMon_Step3(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 8
    {
        (*sprite).data[0] = 0;
        (*sprite).callback = Some(AnimBowMon_Step3_Callback);
    }
}
pub(crate) unsafe fn AnimBowMon_Step3_Callback(sprite: *mut Sprite) {
    if (*sprite).data[0] == 0 {
        (*sprite).data[3] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
        (*sprite).data[6] = GetBattlerSide(gBattleAnimAttacker) as i16;
        if GetBattlerSide(gBattleAnimAttacker) != B_SIDE_PLAYER {
            (*sprite).data[4] = -1024;
            (*sprite).data[5] = 0xC00;
        } else {
            (*sprite).data[4] = 0x400;
            (*sprite).data[5] = -3072;
        }
    }
    (*sprite).data[5] += (*sprite).data[4];
    SetSpriteRotScale(
        (*sprite).data[3] as u8,
        0x100,
        0x100,
        (*sprite).data[5] as u16,
    );
    SetBattlerSpriteYOffsetFromRotation((*sprite).data[3] as u8);
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 2
    {
        ResetSpriteRotScale((*sprite).data[3] as u8);
        (*sprite).callback = Some(AnimBowMon_Step4);
    }
}
pub(crate) unsafe fn AnimBowMon_Step4(sprite: *mut Sprite) {
    DestroyAnimSprite(sprite);
}
pub(crate) unsafe fn AnimTipMon(sprite: *mut Sprite) {
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(AnimTipMon_Step);
}
pub(crate) unsafe fn AnimTipMon_Step(sprite: *mut Sprite) {
    'l1: {
        let sw1: i16 = (*sprite).data[0];
        let mut fall = false;
        if sw1 == 0 {
            fall = true;
            (*sprite).data[1] = 0;
            (*sprite).data[2] = gBattlerSpriteIds[gBattleAnimAttacker] as i16;
            (*sprite).data[3] = GetBattlerSide(gBattleAnimAttacker) as i16;
            (*sprite).data[4] = (if (*sprite).data[3] != B_SIDE_PLAYER as i16 {
                0x200
            } else {
                -512
            }) as i16;
            (*sprite).data[5] = 0;
            PrepareBattlerSpriteForRotScale((*sprite).data[2] as u8, ST_OAM_OBJ_NORMAL);
            (*sprite).data[0] += 1;
        }
        if fall || sw1 == 1 {
            (*sprite).data[5] += (*sprite).data[4];
            SetSpriteRotScale(
                (*sprite).data[2] as u8,
                0x100,
                0x100,
                (*sprite).data[5] as u16,
            );
            SetBattlerSpriteYOffsetFromRotation((*sprite).data[2] as u8);
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 3
            {
                (*sprite).data[1] = 0;
                (*sprite).data[4] *= -1;
                (*sprite).data[0] += 1;
            }
            break 'l1;
        }
        if sw1 == 2 {
            (*sprite).data[5] += (*sprite).data[4];
            SetSpriteRotScale(
                (*sprite).data[2] as u8,
                0x100,
                0x100,
                (*sprite).data[5] as u16,
            );
            SetBattlerSpriteYOffsetFromRotation((*sprite).data[2] as u8);
            if ({
                (*sprite).data[1] += 1;
                (*sprite).data[1]
            }) > 3
            {
                ResetSpriteRotScale((*sprite).data[2] as u8);
                DestroyAnimSprite(sprite);
            }
            break 'l1;
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_SkullBashPosition(taskId: u8) {
    let cmd: *mut Anon38 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon38;
    task_set(taskId, 0, gBattlerSpriteIds[gBattleAnimAttacker] as i16);
    let side: u8 = GetBattlerSide(gBattleAnimAttacker);
    task_set(taskId, 1, side as i16);
    task_set(taskId, 2, 0);
    match (*cmd).unk0 {
        0 => {
            task_set(taskId, 2, 0);
            task_set(taskId, 3, 8);
            task_set(taskId, 4, 0);
            task_set(taskId, 5, 3);
            if side == B_SIDE_PLAYER {
                task_set(taskId, 5, -task_get(taskId, 5));
            }
            task_set_func(taskId, Some(AnimTask_SkullBashPositionSet));
        }
        1 => {
            task_set(taskId, 3, 8);
            task_set(taskId, 4, 0x600);
            task_set(taskId, 5, 0xC0);
            if side == B_SIDE_PLAYER {
                task_set(taskId, 4, -task_get(taskId, 4));
                task_set(taskId, 5, -task_get(taskId, 5));
            }
            task_set_func(taskId, Some(AnimTask_SkullBashPositionReset));
        }
        _ => {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe fn AnimTask_SkullBashPositionSet(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[2] {
        0 => {
            if (*task).data[3] != 0 {
                (*task).data[4] += (*task).data[5];
                gSprites[(*task).data[0]].x2 = (*task).data[4];
                (*task).data[3] -= 1;
            } else {
                (*task).data[3] = 8;
                (*task).data[4] = 0;
                (*task).data[5] = (if (*task).data[1] == 0 { -192 } else { 0xC0 }) as i16;
                PrepareBattlerSpriteForRotScale((*task).data[0] as u8, 0);
                (*task).data[2] += 1;
            }
        }
        1 => {
            if (*task).data[3] != 0 {
                (*task).data[4] += (*task).data[5];
                SetSpriteRotScale((*task).data[0] as u8, 0x100, 0x100, (*task).data[4] as u16);
                SetBattlerSpriteYOffsetFromRotation((*task).data[0] as u8);
                (*task).data[3] -= 1;
            } else {
                (*task).data[3] = 8;
                (*task).data[4] = gSprites[(*task).data[0]].x2;
                (*task).data[5] = (if (*task).data[1] == 0 { 0x2 } else { -2 }) as i16;
                (*task).data[6] = 1;
                (*task).data[2] += 1;
            }
        }
        2 => {
            if (*task).data[3] != 0 {
                if (*task).data[6] != 0 {
                    (*task).data[6] -= 1;
                } else {
                    if (*task).data[3] as i32 & 1 != 0 {
                        gSprites[(*task).data[0]].x2 = (*task).data[4] + (*task).data[5];
                    } else {
                        gSprites[(*task).data[0]].x2 = (*task).data[4] - (*task).data[5];
                    }
                    (*task).data[6] = 1;
                    (*task).data[3] -= 1;
                }
            } else {
                gSprites[(*task).data[0]].x2 = (*task).data[4];
                (*task).data[3] = 12;
                (*task).data[2] += 1;
            }
        }
        3 => {
            if (*task).data[3] != 0 {
                (*task).data[3] -= 1;
            } else {
                (*task).data[3] = 3;
                (*task).data[4] = gSprites[(*task).data[0]].x2;
                (*task).data[5] = (if (*task).data[1] == 0 { 8 } else { -8 }) as i16;
                (*task).data[2] += 1;
            }
        }
        4 => {
            if (*task).data[3] != 0 {
                (*task).data[4] += (*task).data[5];
                gSprites[(*task).data[0]].x2 = (*task).data[4];
                (*task).data[3] -= 1;
            } else {
                DestroyAnimVisualTask(taskId);
            }
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimTask_SkullBashPositionReset(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[3] != 0 {
        (*task).data[4] -= (*task).data[5];
        SetSpriteRotScale((*task).data[0] as u8, 0x100, 0x100, (*task).data[4] as u16);
        SetBattlerSpriteYOffsetFromRotation((*task).data[0] as u8);
        (*task).data[3] -= 1;
    } else {
        ResetSpriteRotScale((*task).data[0] as u8);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimSlashSlice(sprite: *mut Sprite) {
    let cmd: *mut Anon39 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon39;
    if (*cmd).unk0 == 0 {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + (*cmd).unk1;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16
            + (*cmd).unk2;
    } else {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).unk1;
        (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16
            + (*cmd).unk2;
    }
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    StoreSpriteCallbackInData6(sprite, Some(AnimFalseSwipeSlice_Step3));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
pub(crate) unsafe fn AnimFalseSwipeSlice(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + -48;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    StoreSpriteCallbackInData6(sprite, Some(AnimFalseSwipeSlice_Step1));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
pub(crate) unsafe fn AnimFalseSwipePositionedSlice(sprite: *mut Sprite) {
    let cmd: *mut Anon40 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon40;
    (*sprite).x =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 - 48 + (*cmd).unk0;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16;
    StartSpriteAnim(sprite, 1);
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).callback = Some(AnimFalseSwipeSlice_Step3);
}
pub(crate) unsafe fn AnimFalseSwipeSlice_Step1(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 8
    {
        (*sprite).data[0] = 12;
        (*sprite).data[1] = 8;
        (*sprite).data[2] = 0;
        StoreSpriteCallbackInData6(sprite, Some(AnimFalseSwipeSlice_Step2));
        (*sprite).callback = Some(TranslateSpriteLinear);
    }
}
pub(crate) unsafe fn AnimFalseSwipeSlice_Step2(sprite: *mut Sprite) {
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).callback = Some(AnimFalseSwipeSlice_Step3);
}
pub(crate) unsafe fn AnimFalseSwipeSlice_Step3(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 1
    {
        (*sprite).data[0] = 0;
        (*sprite).set_invisible(((*sprite).invisible() == 0) as u16);
        if ({
            (*sprite).data[1] += 1;
            (*sprite).data[1]
        }) > 8
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe fn AnimEndureEnergy(sprite: *mut Sprite) {
    let cmd: *mut Anon41 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon41;
    if (*cmd).unk0 == 0 {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + (*cmd).unk1;
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + (*cmd).unk2;
    } else {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X) as i16 + (*cmd).unk1;
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y) as i16 + (*cmd).unk2;
    }
    (*sprite).data[0] = 0;
    (*sprite).data[1] = (*cmd).unk3;
    (*sprite).callback = Some(AnimEndureEnergy_Step);
}
pub(crate) unsafe fn AnimEndureEnergy_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > (*sprite).data[1]
    {
        (*sprite).data[0] = 0;
        (*sprite).y -= 1;
    }
    (*sprite).y -= (*sprite).data[0];
    if (*sprite).animEnded() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimSharpenSphere(sprite: *mut Sprite) {
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
    (*sprite).y =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 - 12;
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 2;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 0;
    (*sprite).data[5] = BattleAnimAdjustPanning(SOUND_PAN_ATTACKER) as i16;
    (*sprite).callback = Some(AnimSharpenSphere_Step);
}
pub(crate) unsafe fn AnimSharpenSphere_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) >= (*sprite).data[1]
    {
        (*sprite).set_invisible(((*sprite).invisible() == 0) as u16);
        if (*sprite).invisible() == 0 {
            (*sprite).data[4] += 1;
            if (*sprite).data[4] as i32 & 1 == 0 {
                PlaySE12WithPanning(SE_M_SWAGGER2, (*sprite).data[5] as i8);
            }
        }
        (*sprite).data[0] = 0;
        if ({
            (*sprite).data[2] += 1;
            (*sprite).data[2]
        }) > 1
        {
            (*sprite).data[2] = 0;
            (*sprite).data[1] += 1;
        }
    }
    if (*sprite).animEnded() != 0 && (*sprite).data[1] > 16 && (*sprite).invisible() != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimConversion(sprite: *mut Sprite) {
    let cmd: *mut Anon42 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon42;
    if (*sprite).data[0] == 0 {
        (*sprite).x =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X) as i16 + (*cmd).unk0;
        (*sprite).y =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y) as i16 + (*cmd).unk1;
        if IsContest() != 0 {
            (*sprite).y += 10;
        }
        (*sprite).data[0] += 1;
    }
    if gBattleAnimArgs[7] as u16 == 0xFFFF {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_ConversionAlphaBlend(taskId: u8) {
    if task_get(taskId, 2) == 1 {
        gBattleAnimArgs[7] = -1;
        task_set(taskId, 2, task_get(taskId, 2) + 1);
    } else if task_get(taskId, 2) == 2 {
        DestroyAnimVisualTask(taskId);
    } else {
        if ({
            task_set(taskId, 0, task_get(taskId, 0) + 1);
            task_get(taskId, 0)
        }) == 4
        {
            task_set(taskId, 0, 0);
            task_set(taskId, 1, task_get(taskId, 1) + 1);
            SetGpuReg(
                REG_OFFSET_BLDALPHA,
                ((task_get(taskId, 1) as u16) << 8) | (16 - task_get(taskId, 1) as u16),
            );
            if task_get(taskId, 1) == 16 {
                task_set(taskId, 2, task_get(taskId, 2) + 1);
            }
        }
    }
}
pub(crate) unsafe fn AnimConversion2(sprite: *mut Sprite) {
    let cmd: *mut Anon43 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon43;
    InitSpritePosToAnimTarget(sprite, FALSE);
    (*sprite).set_animPaused(1);
    (*sprite).data[0] = (*cmd).unk2;
    (*sprite).callback = Some(AnimConversion2_Step);
}
pub(crate) unsafe fn AnimConversion2_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        (*sprite).data[0] -= 1;
    } else {
        (*sprite).set_animPaused(0);
        (*sprite).data[0] = 30;
        (*sprite).data[2] = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16;
        (*sprite).data[4] =
            GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16;
        (*sprite).callback = Some(StartAnimLinearTranslation);
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_Conversion2AlphaBlend(taskId: u8) {
    if ({
        task_set(taskId, 0, task_get(taskId, 0) + 1);
        task_get(taskId, 0)
    }) == 4
    {
        task_set(taskId, 0, 0);
        task_set(taskId, 1, task_get(taskId, 1) + 1);
        SetGpuReg(
            REG_OFFSET_BLDALPHA,
            (16 - task_get(taskId, 1) as u16) << 8 | task_get(taskId, 1) as u16,
        );
        if task_get(taskId, 1) == 16 {
            DestroyAnimVisualTask(taskId);
        }
    }
}
unsafe fn AnimTask_HideBattlersHealthbox(taskId: u8) {
    let cmd: *mut Anon44 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon44;
    let mut i: u8 = 0;
    while i < gBattlersCount {
        if (*cmd).unk0 == TRUE as i16 && GetBattlerSide(i) == B_SIDE_PLAYER {
            SetHealthboxSpriteInvisible(gHealthboxSpriteIds[i]);
        }
        if (*cmd).unk1 == 1 && GetBattlerSide(i) == 1 {
            SetHealthboxSpriteInvisible(gHealthboxSpriteIds[i]);
        }
        i += 1;
    }
    DestroyAnimVisualTask(taskId);
}
unsafe fn AnimTask_ShowBattlersHealthbox(taskId: u8) {
    let mut i: u8 = 0;
    while i < gBattlersCount {
        SetHealthboxSpriteVisible(gHealthboxSpriteIds[i]);
        i += 1;
    }
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimMoon(sprite: *mut Sprite) {
    let cmd: *mut Anon45 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon45;
    if IsContest() != 0 {
        (*sprite).x = 48;
        (*sprite).y = 40;
    } else {
        (*sprite).x = (*cmd).unk0;
        (*sprite).y = (*cmd).unk1;
    }
    (*sprite).oam.set_shape(0);
    (*sprite).oam.set_size(3);
    (*sprite).data[0] = 0;
    (*sprite).callback = Some(AnimMoon_Step);
}
pub(crate) unsafe fn AnimMoon_Step(sprite: *mut Sprite) {
    if (*sprite).data[0] != 0 {
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe fn AnimMoonlightSparkle(sprite: *mut Sprite) {
    let cmd: *mut Anon46 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon46;
    (*sprite).x =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + (*cmd).unk0;
    (*sprite).y = (*cmd).unk1;
    (*sprite).data[0] = 0;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = 1;
    (*sprite).callback = Some(AnimMoonlightSparkle_Step);
}
pub(crate) unsafe fn AnimMoonlightSparkle_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 1
    {
        (*sprite).data[1] = 0;
        if (*sprite).data[2] < 120 {
            (*sprite).y += 1;
            (*sprite).data[2] += 1;
        }
    }
    if (*sprite).data[0] != 0 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MoonlightEndFade(taskId: u8) {
    let a: i32 =
        GetBattlePalettesMask(TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE) as i32 & 0xFFFF;
    task_set(taskId, 0, 0);
    task_set(taskId, 1, 0);
    task_set(taskId, 2, 0);
    task_set(taskId, 3, a as i16);
    task_set(taskId, 4, 0);
    task_set(taskId, 5, 0);
    task_set(taskId, 6, 0);
    task_set(taskId, 7, 13);
    task_set(taskId, 8, 14);
    task_set(taskId, 9, 15);
    let mut b: i32 = GetBattleMonSpritePalettesMask(1, 1, 1, 1) as i32;
    let c: i32 = a | b;
    StorePointerInVars(
        task_data_ptr(taskId, 14),
        task_data_ptr(taskId, 15),
        c as usize as *mut c_void,
    );
    b |= shl_i32(0x10000, IndexOfSpritePaletteTag(ANIM_TAG_MOON) as u32);
    let d: i32 = IndexOfSpritePaletteTag(ANIM_TAG_GREEN_SPARKLE) as i32;
    BeginNormalPaletteFade(
        shl_i32(0x10000, d as u32) as u32 | b as u32,
        0,
        0,
        16,
        32699,
    );
    task_set_func(taskId, Some(AnimTask_MoonlightEndFade_Step));
    task_func(taskId).unwrap_unchecked()(taskId);
}
pub(crate) unsafe fn AnimTask_MoonlightEndFade_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    match (*task).data[0] {
        0 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 0
            {
                let mut color: u16 = 0;
                (*task).data[1] = 0;
                if ({
                    (*task).data[2] += 1;
                    (*task).data[2]
                }) <= 15
                {
                    (*task).data[4] += (*task).data[7];
                    (*task).data[5] += (*task).data[8];
                    (*task).data[6] += (*task).data[9];
                    let red: u16 = ((*task).data[4] >> 3) as u16;
                    let green: u16 = ((*task).data[5] >> 3) as u16;
                    let blue: u16 = ((*task).data[6] >> 3) as u16;
                    color = red | green << 5 | blue << 10;
                } else {
                    color = 32699;
                    (*task).data[0] += 1;
                }
                let mut bitmask: u16 = 1;
                let mut r3: u16 = 0;
                for i in 0..=15u16 {
                    if (*task).data[3] as i32 & bitmask as i32 != 0 {
                        for j in 1..=15u16 {
                            gPlttBufferFaded[r3 as i32 + j as i32] = color;
                        }
                    }
                    bitmask <<= 1;
                    r3 += 16;
                }
            }
        }
        1 => {
            if gPaletteFade.active() == 0 {
                for spriteId in 0..MAX_SPRITES {
                    if gSprites[spriteId].template == (&raw const *gMoonSpriteTemplate).cast_mut()
                        || gSprites[spriteId].template
                            == (&raw const *gMoonlightSparkleSpriteTemplate).cast_mut()
                    {
                        gSprites[spriteId].data[0] = 1;
                    }
                }
                (*task).data[1] = 0;
                (*task).data[0] += 1;
            }
        }
        2 => {
            if ({
                (*task).data[1] += 1;
                (*task).data[1]
            }) > 30
            {
                BeginNormalPaletteFade(
                    LoadPointerFromVars((*task).data[14], (*task).data[15]) as usize as u32,
                    0,
                    16,
                    0,
                    32699,
                );
                (*task).data[0] += 1;
            }
        }
        3 if gPaletteFade.active() == 0 => {
            DestroyAnimVisualTask(taskId);
        }
        _ => {}
    }
}
pub(crate) unsafe fn AnimHornHit(sprite: *mut Sprite) {
    let cmd: *mut Anon47 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon47;
    if (*cmd).unk2 < 2 {
        (*cmd).unk2 = 2;
    }
    if (*cmd).unk2 > 0x7F {
        (*cmd).unk2 = 0x7F;
    }
    (*sprite).data[0] = 0;
    (*sprite).data[1] = (*cmd).unk2;
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2) as i16 + (*cmd).unk0;
    (*sprite).y =
        GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).unk1;
    (*sprite).data[6] = (*sprite).x;
    (*sprite).data[7] = (*sprite).y;
    if IsContest() != 0 {
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
        (*sprite).x += 40;
        (*sprite).y += 20;
        (*sprite).data[2] = (*sprite).x << 7;
        (*sprite).data[3] = div_i32(-5120, (*sprite).data[1] as i32) as i16;
        (*sprite).data[4] = (*sprite).y << 7;
        (*sprite).data[5] = div_i32(-2560, (*sprite).data[1] as i32) as i16;
    } else if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_PLAYER {
        (*sprite).x -= 40;
        (*sprite).y += 20;
        (*sprite).data[2] = (*sprite).x << 7;
        (*sprite).data[3] = div_i32(0x1400, (*sprite).data[1] as i32) as i16;
        (*sprite).data[4] = (*sprite).y << 7;
        (*sprite).data[5] = div_i32(-2560, (*sprite).data[1] as i32) as i16;
    } else {
        (*sprite).x += 40;
        (*sprite).y -= 20;
        (*sprite).data[2] = (*sprite).x << 7;
        (*sprite).data[3] = div_i32(-5120, (*sprite).data[1] as i32) as i16;
        (*sprite).data[4] = (*sprite).y << 7;
        (*sprite).data[5] = div_i32(0xA00, (*sprite).data[1] as i32) as i16;
        (*sprite).oam.set_matrixNum(24);
    }
    (*sprite).callback = Some(AnimHornHit_Step);
}
pub(crate) unsafe fn AnimHornHit_Step(sprite: *mut Sprite) {
    (*sprite).data[2] += (*sprite).data[3];
    (*sprite).data[4] += (*sprite).data[5];
    (*sprite).x = (*sprite).data[2] >> 7;
    (*sprite).y = (*sprite).data[4] >> 7;
    if ({
        (*sprite).data[1] -= 1;
        (*sprite).data[1]
    }) == 1
    {
        (*sprite).x = (*sprite).data[6];
        (*sprite).y = (*sprite).data[7];
    }
    if (*sprite).data[1] == 0 {
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_DoubleTeam(taskId: u8) {
    let mut obj: i32 = 0;
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    (*task).data[0] = GetAnimBattlerSpriteId(0) as i16;
    (*task).data[1] = AllocSpritePalette(ANIM_TAG_BENT_SPOON) as i16;
    let r3: u16 = 0x100 + (*task).data[1] as u16 * 16;
    let r4: u16 = (gSprites[(*task).data[0]].oam.paletteNum() + 16) * 16;
    for i in 1..16u16 {
        gPlttBufferUnfaded[r3 as i32 + i as i32] = gPlttBufferUnfaded[r4 as i32 + i as i32];
    }
    BlendPalette(r3, 16, 11, 0);
    (*task).data[3] = 0;
    let mut i: u16 = 0;
    while i < 2
        && ({
            obj = CloneBattlerSpriteWithBlend(0) as i32;
            obj
        }) >= 0
    {
        gSprites[obj].oam.set_paletteNum((*task).data[1] as u16);
        gSprites[obj].data[0] = 0;
        gSprites[obj].data[1] = (i as i16) << 7;
        gSprites[obj].data[2] = taskId as i16;
        gSprites[obj].callback = Some(AnimDoubleTeam);
        (*task).data[3] += 1;
        i += 1;
    }
    (*task).func = Some(AnimTask_DoubleTeam_Step);
    if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG1_ON);
    } else {
        ClearGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
    }
}
pub(crate) unsafe fn AnimTask_DoubleTeam_Step(taskId: u8) {
    let task: *mut Task = &raw mut (*gTasks.as_ptr())[taskId];
    if (*task).data[3] == 0 {
        if GetBattlerSpriteBGPriorityRank(gBattleAnimAttacker) == 1 {
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG1_ON);
        } else {
            SetGpuRegBits(REG_OFFSET_DISPCNT, DISPCNT_BG2_ON);
        }
        FreeSpritePaletteByTag(ANIM_TAG_BENT_SPOON);
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe fn AnimDoubleTeam(sprite: *mut Sprite) {
    if ({
        (*sprite).data[3] += 1;
        (*sprite).data[3]
    }) > 1
    {
        (*sprite).data[3] = 0;
        (*sprite).data[0] += 1;
    }
    if (*sprite).data[0] > 64 {
        task_set((*sprite).data[2], 3, task_get((*sprite).data[2], 3) - 1);
        DestroySpriteWithActiveSheet(sprite);
    } else {
        (*sprite).data[4] =
            (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*sprite).data[0]] / 6;
        (*sprite).data[5] = (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())
            [(*sprite).data[0]]
            / 13;
        (*sprite).data[1] = ((*sprite).data[1] + (*sprite).data[5]) & 0xFF;
        (*sprite).x2 = Sin((*sprite).data[1], (*sprite).data[4]);
    }
}
pub(crate) unsafe fn AnimSuperFang(sprite: *mut Sprite) {
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MusicNotesRainbowBlend(taskId: u8) {
    let mut index: u16 = IndexOfSpritePaletteTag(gParticlesColorBlendTable[0][0]) as u16;
    if index != 0xFF {
        index = 0x100 + index * 16;
        for i in 1..6u16 {
            gPlttBufferFaded[index as i32 + i as i32] = gParticlesColorBlendTable[0][i];
        }
    }
    for j in 1..4u16 {
        index = AllocSpritePalette(gParticlesColorBlendTable[j][0]) as u16;
        if index != 0xFF {
            index = 0x100 + index * 16;
            for i in 1..6u16 {
                gPlttBufferFaded[index as i32 + i as i32] = gParticlesColorBlendTable[j][i];
            }
        }
    }
    DestroyAnimVisualTask(taskId);
}
#[unsafe(no_mangle)]
pub unsafe fn AnimTask_MusicNotesClearRainbowBlend(taskId: u8) {
    for i in 1..4u16 {
        FreeSpritePaletteByTag(gParticlesColorBlendTable[i][0]);
    }
    DestroyAnimVisualTask(taskId);
}
pub(crate) unsafe fn AnimWavyMusicNotes(sprite: *mut Sprite) {
    let cmd: *mut Anon48 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon48;
    let mut index: u8 = 0;
    let mut x: u8 = 0;
    let mut y: u8 = 0;
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    StartSpriteAnim(sprite, (*cmd).unk0 as u8);
    if ({
        index = IndexOfSpritePaletteTag(gParticlesColorBlendTable[(*cmd).unk1][0]);
        index
    }) != 0xFF
    {
        (*sprite).oam.set_paletteNum(index as u16);
    }
    (*sprite).data[sBlendTableIdx] = (*cmd).unk1;
    (*sprite).data[sBlendTimer] = 0;
    (*sprite).data[sBlendCycleTime] = (*cmd).unk2;
    if IsContest() != 0 {
        x = 48;
        y = 40;
    } else {
        x = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_X_2);
        y = GetBattlerSpriteCoord(gBattleAnimTarget, BATTLER_COORD_Y_PIC_OFFSET);
    }
    (*sprite).data[sX] = (*sprite).x << 4;
    (*sprite).data[sY] = (*sprite).y << 4;
    AnimWavyMusicNotes_CalcVelocity(
        x as i16 - (*sprite).x,
        y as i16 - (*sprite).y,
        &raw mut (*sprite).data[sVelocX],
        &raw mut (*sprite).data[sVelocY],
        40,
    );
    (*sprite).callback = Some(AnimWavyMusicNotes_Step);
}
unsafe fn AnimWavyMusicNotes_CalcVelocity(
    x: i16,
    y: i16,
    velocX: *mut i16,
    velocY: *mut i16,
    mut xSpeedFactor: i8,
) {
    if x < 0 {
        xSpeedFactor = -xSpeedFactor;
    }
    let x2: i32 = x as i32 * 256;
    let mut time: i32 = div_i32(x2, xSpeedFactor as i32);
    if time == 0 {
        time = 1;
    }
    *velocX = div_i32(x2, time) as i16;
    *velocY = div_i32(y as i32 * 256, time) as i16;
}
pub(crate) unsafe fn AnimWavyMusicNotes_Step(sprite: *mut Sprite) {
    let mut y: i16 = 0;
    let mut index: u8 = 0;
    (*sprite).data[sMoveTimer] += 1;
    let trigIdx: i16 = ((*sprite).data[sMoveTimer] as i32 * 5 % 256) as i16;
    (*sprite).data[sX] += (*sprite).data[sVelocX];
    (*sprite).data[sY] += (*sprite).data[sVelocY];
    (*sprite).x = (*sprite).data[sX] >> 4;
    (*sprite).y = (*sprite).data[sY] >> 4;
    (*sprite).y2 = Sin(trigIdx, 15);
    y = (*sprite).y;
    if (*sprite).x < -16 || (*sprite).x > 256 || !(-16..=128).contains(&y) {
        DestroySpriteAndMatrix(sprite);
    } else {
        if (*sprite).data[sBlendCycleTime] != 0
            && ({
                (*sprite).data[sBlendTimer] += 1;
                (*sprite).data[sBlendTimer]
            }) > (*sprite).data[sBlendCycleTime]
        {
            (*sprite).data[sBlendTimer] = 0;
            if ({
                (*sprite).data[sBlendTableIdx] += 1;
                (*sprite).data[sBlendTableIdx]
            }) > 3
            {
                (*sprite).data[sBlendTableIdx] = 0;
            }
            index = IndexOfSpritePaletteTag(
                gParticlesColorBlendTable[(*sprite).data[sBlendTableIdx]][0],
            );
            if index != 0xFF {
                (*sprite).oam.set_paletteNum(index as u16);
            }
        }
    }
}
pub(crate) unsafe fn AnimFlyingMusicNotes(sprite: *mut Sprite) {
    let cmd: *mut Anon49 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon49;
    if GetBattlerSide(gBattleAnimAttacker) == B_SIDE_OPPONENT {
        (*cmd).unk1 *= -1;
    }
    (*sprite).x =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + (*cmd).unk1;
    (*sprite).y =
        GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 + (*cmd).unk2;
    StartSpriteAnim(sprite, (*cmd).unk0 as u8);
    (*sprite).data[2] = 0;
    (*sprite).data[3] = 0;
    (*sprite).data[4] = (*sprite).x << 4;
    (*sprite).data[5] = (*sprite).y << 4;
    (*sprite).data[6] = ((((*cmd).unk1 as i32) << 4) / 5) as i16;
    (*sprite).data[7] = ((((*cmd).unk2 as i32) << 7) / 5) as i16;
    (*sprite).callback = Some(AnimFlyingMusicNotes_Step);
}
pub(crate) unsafe fn AnimFlyingMusicNotes_Step(sprite: *mut Sprite) {
    (*sprite).data[4] += (*sprite).data[6];
    (*sprite).data[5] += (*sprite).data[7];
    (*sprite).x = (*sprite).data[4] >> 4;
    (*sprite).y = (*sprite).data[5] >> 4;
    if (*sprite).data[0] > 5 && (*sprite).data[3] == 0 {
        (*sprite).data[2] = ((*sprite).data[2] + 16) & 0xFF;
        (*sprite).x2 = Cos((*sprite).data[2], 18);
        (*sprite).y2 = Sin((*sprite).data[2], 18);
        if (*sprite).data[2] == 0 {
            (*sprite).data[3] = 1;
        }
    }
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) == 48
    {
        DestroySpriteAndMatrix(sprite);
    }
}
pub(crate) unsafe fn AnimBellyDrumHand(sprite: *mut Sprite) {
    let cmd: *mut Anon50 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon50;
    let mut a: i16 = 0;
    if (*cmd).unk0 == 1 {
        (*sprite).oam.set_matrixNum(ST_OAM_HFLIP);
        a = 16;
    } else {
        a = -16;
    }
    (*sprite).x = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_X_2) as i16 + a;
    (*sprite).y = GetBattlerSpriteCoord(gBattleAnimAttacker, BATTLER_COORD_Y_PIC_OFFSET) as i16 + 8;
    (*sprite).data[0] = 8;
    (*sprite).callback = Some(WaitAnimForDuration);
    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
}
pub unsafe fn AnimSlowFlyingMusicNotes(sprite: *mut Sprite) {
    let cmd: *mut Anon51 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon51;
    SetSpriteCoordsToAnimAttackerCoords(sprite);
    (*sprite).y += 8;
    StartSpriteAnim(sprite, (*cmd).unk1 as u8);
    let index: u8 = IndexOfSpritePaletteTag(gParticlesColorBlendTable[(*cmd).unk2][0]);
    if index != 0xFF {
        (*sprite).oam.set_paletteNum(index as u16);
    }
    let xDiff: i16 = (if (*cmd).unk0 == 0 { -32 } else { 32 }) as i16;
    (*sprite).data[0] = 40;
    (*sprite).data[1] = (*sprite).x;
    (*sprite).data[2] = xDiff + (*sprite).data[1];
    (*sprite).data[3] = (*sprite).y;
    (*sprite).data[4] = (*sprite).data[3] - 40;
    InitAnimLinearTranslation(sprite);
    (*sprite).data[5] = (*cmd).unk3;
    (*sprite).callback = Some(AnimSlowFlyingMusicNotes_Step);
}
pub(crate) unsafe fn AnimSlowFlyingMusicNotes_Step(sprite: *mut Sprite) {
    if AnimTranslateLinear(sprite) == 0 {
        let mut xDiff: i16 = Sin((*sprite).data[5], 8);
        if (*sprite).x2 < 0 {
            xDiff = -xDiff;
        }
        (*sprite).x2 += xDiff;
        (*sprite).y2 += Sin((*sprite).data[5], 4);
        (*sprite).data[5] = ((*sprite).data[5] + 8) & 0xFF;
    } else {
        DestroyAnimSprite(sprite);
    }
}
pub unsafe fn SetSpriteNextToMonHead(battler: u8, sprite: *mut Sprite) {
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_RIGHT) + 8;
    } else {
        (*sprite).x = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_LEFT) - 8;
    }
    (*sprite).y = GetBattlerSpriteCoord(battler, BATTLER_COORD_Y_PIC_OFFSET) as i16
        - GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_HEIGHT) / 4;
}
pub(crate) unsafe fn AnimThoughtBubble(sprite: *mut Sprite) {
    let cmd: *mut Anon52 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon52;
    let mut battler: u8 = 0;
    if (*cmd).unk0 == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    SetSpriteNextToMonHead(battler, sprite);
    let animNum: u8 = (if GetBattlerSide(battler) == 0 { 0 } else { 1 }) as u8;
    (*sprite).data[0] = (*cmd).unk1;
    (*sprite).data[1] = animNum as i16 + 2;
    StartSpriteAnim(sprite, animNum);
    StoreSpriteCallbackInData6(sprite, Some(AnimThoughtBubble_Step));
    (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
}
pub(crate) unsafe fn AnimThoughtBubble_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] -= 1;
        (*sprite).data[0]
    }) == 0
    {
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        StartSpriteAnim(sprite, (*sprite).data[1] as u8);
        (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    }
}
pub(crate) unsafe fn AnimMetronomeFinger(sprite: *mut Sprite) {
    let cmd: *mut Anon53 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon53;
    let mut battler: u8 = 0;
    if (*cmd).unk0 == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    SetSpriteNextToMonHead(battler, sprite);
    (*sprite).data[0] = 0;
    StoreSpriteCallbackInData6(sprite, Some(AnimMetronomeFinger_Step));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe fn AnimMetronomeFinger_Step(sprite: *mut Sprite) {
    if ({
        (*sprite).data[0] += 1;
        (*sprite).data[0]
    }) > 16
    {
        StartSpriteAffineAnim(sprite, 1);
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
    }
}
pub(crate) unsafe fn AnimFollowMeFinger(sprite: *mut Sprite) {
    let cmd: *mut Anon54 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon54;
    let mut battler: u8 = 0;
    if (*cmd).unk0 == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    (*sprite).x = GetBattlerSpriteCoord(battler, BATTLER_COORD_X) as i16;
    (*sprite).y = GetBattlerSpriteCoordAttr(battler, BATTLER_COORD_ATTR_TOP);
    if (*sprite).y <= 9 {
        (*sprite).y = 10;
    }
    (*sprite).data[0] = 1;
    (*sprite).data[1] = 0;
    (*sprite).data[2] = (*sprite).subpriority as i16;
    (*sprite).data[3] = (*sprite).subpriority as i16 + 4;
    (*sprite).data[4] = 0;
    StoreSpriteCallbackInData6(sprite, Some(AnimFollowMeFinger_Step1));
    (*sprite).callback = Some(RunStoredCallbackWhenAffineAnimEnds);
}
pub(crate) unsafe fn AnimFollowMeFinger_Step1(sprite: *mut Sprite) {
    if ({
        (*sprite).data[4] += 1;
        (*sprite).data[4]
    }) > 12
    {
        (*sprite).callback = Some(AnimFollowMeFinger_Step2);
    }
}
pub(crate) unsafe fn AnimFollowMeFinger_Step2(sprite: *mut Sprite) {
    let mut x2: i16 = 0;
    (*sprite).data[1] += 4;
    if (*sprite).data[1] > 254 {
        if ({
            (*sprite).data[0] -= 1;
            (*sprite).data[0]
        }) == 0
        {
            (*sprite).x2 = 0;
            (*sprite).callback = Some(AnimMetronomeFinger_Step);
            return;
        } else {
            (*sprite).data[1] &= 0xFF;
        }
    }
    if (*sprite).data[1] > 0x4F {
        (*sprite).subpriority = (*sprite).data[3] as u8;
    }
    if (*sprite).data[1] > 0x9F {
        (*sprite).subpriority = (*sprite).data[2] as u8;
    }
    let x1: i16 =
        (*(&raw const crate::trig::gSineTable).cast::<CArray<i16, 0>>())[(*sprite).data[1]];
    x2 = x1 >> 3;
    (*sprite).x2 = (x1 >> 3) + (x2 >> 1);
}
pub(crate) unsafe fn AnimTauntFinger(sprite: *mut Sprite) {
    let cmd: *mut Anon55 = gBattleAnimArgs.as_mut_ptr() as *mut c_void as *mut Anon55;
    let mut battler: u8 = 0;
    if (*cmd).unk0 == 0 {
        battler = gBattleAnimAttacker;
    } else {
        battler = gBattleAnimTarget;
    }
    SetSpriteNextToMonHead(battler, sprite);
    if GetBattlerSide(battler) == B_SIDE_PLAYER {
        StartSpriteAnim(sprite, 0);
        (*sprite).data[0] = 2;
    } else {
        StartSpriteAnim(sprite, 1);
        (*sprite).data[0] = 3;
    }
    (*sprite).callback = Some(AnimTauntFinger_Step1);
}
pub(crate) unsafe fn AnimTauntFinger_Step1(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 10
    {
        (*sprite).data[1] = 0;
        StartSpriteAnim(sprite, (*sprite).data[0] as u8);
        StoreSpriteCallbackInData6(sprite, Some(AnimTauntFinger_Step2));
        (*sprite).callback = Some(RunStoredCallbackWhenAnimEnds);
    }
}
pub(crate) unsafe fn AnimTauntFinger_Step2(sprite: *mut Sprite) {
    if ({
        (*sprite).data[1] += 1;
        (*sprite).data[1]
    }) > 5
    {
        DestroyAnimSprite(sprite);
    }
}
