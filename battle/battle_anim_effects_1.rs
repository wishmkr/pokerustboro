//! Translated from `src/battle_anim_effects_1.c` by tools/rustport/c2rs.py, then reviewed.
#![allow(
    non_snake_case,
    non_upper_case_globals,
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
    clippy::all,
    clashing_extern_declarations,
    unpredictable_function_pointer_comparisons
)]

// Data tables (translate with cdata.py): gPowderParticlesAnimCmds gPowderParticlesAnimTable gSleepPowderParticleSpriteTemplate gStunSporeParticleSpriteTemplate gPoisonPowderParticleSpriteTemplate gSolarBeamBigOrbAnimCmds1 gSolarBeamBigOrbAnimCmds2 gSolarBeamBigOrbAnimCmds3 gSolarBeamBigOrbAnimCmds4 gSolarBeamBigOrbAnimCmds5 gSolarBeamBigOrbAnimCmds6 gSolarBeamBigOrbAnimCmds7 gSolarBeamSmallOrbAnimCms gPowerAbsorptionOrbAnimCmds gSolarBeamBigOrbAnimTable gSolarBeamSmallOrbAnimTable gPowerAbsorptionOrbAnimTable gPowerAbsorptionOrbAffineAnimCmds gPowerAbsorptionOrbAffineAnimTable gPowerAbsorptionOrbSpriteTemplate gSolarBeamBigOrbSpriteTemplate gSolarBeamSmallOrbSpriteTemplate gStockpileAbsorptionOrbAffineCmds gStockpileAbsorptionOrbAffineAnimTable gStockpileAbsorptionOrbSpriteTemplate gAbsorptionOrbAffineAnimCmds gAbsorptionOrbAffineAnimTable gAbsorptionOrbSpriteTemplate gHyperBeamOrbSpriteTemplate gLeechSeedAnimCmds1 gLeechSeedAnimCmds2 gLeechSeedAnimTable gLeechSeedSpriteTemplate gSporeParticleAnimCmds1 gSporeParticleAnimCmds2 gSporeParticleAnimTable gSporeParticleSpriteTemplate gPetalDanceBigFlowerAnimCmds gPetalDanceSmallFlowerAnimCmds gPetalDanceBigFlowerAnimTable gPetalDanceSmallFlowerAnimTable gPetalDanceBigFlowerSpriteTemplate gPetalDanceSmallFlowerSpriteTemplate gRazorLeafParticleAnimCmds1 gRazorLeafParticleAnimCmds2 gRazorLeafParticleAnimTable gRazorLeafParticleSpriteTemplate gTwisterLeafSpriteTemplate gRazorLeafCutterAnimCmds gRazorLeafCutterAnimTable gRazorLeafCutterSpriteTemplate gSwiftStarAffineAnimCmds gSwiftStarAffineAnimTable gSwiftStarSpriteTemplate sAnim_ConstrictBinding sAnim_ConstrictBinding_Flipped sAnims_ConstrictBinding sAffineAnim_ConstrictBinding sAffineAnim_ConstrictBinding_Flipped sAffineAnims_ConstrictBinding gConstrictBindingSpriteTemplate gMimicOrbAffineAnimCmds1 gMimicOrbAffineAnimCmds2 gMimicOrbAffineAnimTable gMimicOrbSpriteTemplate gIngrainRootAnimCmds1 gIngrainRootAnimCmds2 gIngrainRootAnimCmds3 gIngrainRootAnimCmds4 gIngrainRootAnimTable gIngrainRootSpriteTemplate gFrenzyPlantRootSpriteTemplate gIngrainOrbAnimCmds gIngrainOrbAnimTable gIngrainOrbSpriteTemplate gFallingBagAnimCmds gFallingBagAnimTable gFallingBagAffineAnimCmds1 gFallingBagAffineAnimCmds2 gFallingBagAffineAnimTable gPresentSpriteTemplate gKnockOffItemSpriteTemplate gPresentHealParticleAnimCmds gPresentHealParticleAnimTable gPresentHealParticleSpriteTemplate gItemStealSpriteTemplate gTrickBagAffineAnimCmds1 gTrickBagAffineAnimCmds2 gTrickBagAffineAnimTable gTrickBagSpriteTemplate gTrickBagCoordinates gLeafBladeAnimCmds1 gLeafBladeAnimCmds2 gLeafBladeAnimCmds3 gLeafBladeAnimCmds4 gLeafBladeAnimCmds5 gLeafBladeAnimCmds6 gLeafBladeAnimCmds7 gLeafBladeAnimTable gLeafBladeSpriteTemplate gAromatherapyBigFlowerAffineAnimCmds gAromatherapyBigFlowerAffineAnimTable gAromatherapySmallFlowerSpriteTemplate gAromatherapyBigFlowerSpriteTemplate gSilverWindBigSparkAffineAnimCmds gSilverWindMediumSparkAffineAnimCmds gSilverWindSmallSparkAffineAnimCmds gSilverWindBigSparkAffineAnimTable gSilverWindMediumSparkAffineAnimTable gSilverWindSmallSparkAffineAnimTable gSilverWindBigSparkSpriteTemplate gSilverWindMediumSparkSpriteTemplate gSilverWindSmallSparkSpriteTemplate gMagicalLeafBlendColors gNeedleArmSpikeSpriteTemplate sAnim_Whip sAnim_Whip_Flipped sAnims_Whip gSlamHitSpriteTemplate gVineWhipSpriteTemplate sAnim_SlidingHit sAnims_SlidingHit sSlidingHit1SpriteTemplate sSlidingHit2SpriteTemplate sAffineAnim_FlickeringPunch_Normal sAffineAnim_FlickeringPunch_TurnedTopLeft sAffineAnim_FlickeringPunch_TurnedLeft sAffineAnim_FlickeringPunch_TurnedBottomLeft sAffineAnim_FlickeringPunch_UpsideDown sAffineAnim_FlickeringPunch_TurnedBottomRight sAffineAnim_FlickeringPunch_TurnedRight sAffineAnim_FlickeringPunch_TurnedTopRight sAffineAnims_FlickeringPunch sFlickeringPunchSpriteTemplate gCuttingSliceAnimCmds gCuttingSliceAnimTable gCuttingSliceSpriteTemplate gAirCutterSliceSpriteTemplate sAnim_CirclingMusicNote_Eighth sAnim_CirclingMusicNote_BeamedEighth sAnim_CirclingMusicNote_SlantedBeamedEighth sAnim_CirclingMusicNote_Quarter sAnim_CirclingMusicNote_QuarterRest sAnim_CirclingMusicNote_EighthRest sAnim_CirclingMusicNote_Eighth_Flipped sAnim_CirclingMusicNote_BeamedEighth_Flipped sAnim_CirclingMusicNote_SlantedBeamedEighth_Flipped sAnim_CirclingMusicNote_Quarter_Flipped sAnims_CirclingMusicNote sCirclingMusicNoteSpriteTemplate gProtectSpriteTemplate gMilkBottleAffineAnimCmds1 gMilkBottleAffineAnimCmds2 gMilkBottleAffineAnimTable gMilkBottleSpriteTemplate gGrantingStarsAnimCmds gGrantingStarsAnimTable gGrantingStarsSpriteTemplate gSparklingStarsSpriteTemplate sAnim_BubbleBurst sAnim_BubbleBurst_Flipped sAnims_BubbleBurst sBubbleBurstSpriteTemplate gSleepLetterZAnimCmds gSleepLetterZAnimTable gSleepLetterZAffineAnimCmds1 gSleepLetterZAffineAnimCmds1_2 gSleepLetterZAffineAnimCmds2 gSleepLetterZAffineAnimCmds2_2 gSleepLetterZAffineAnimTable gSleepLetterZSpriteTemplate gLockOnTargetSpriteTemplate gLockOnMoveTargetSpriteTemplate gInclineMonCoordTable gBowMonSpriteTemplate sTipMonSpriteTemplate gSlashSliceAnimCmds1 gSlashSliceAnimCmds2 gSlashSliceAnimTable gSlashSliceSpriteTemplate gFalseSwipeSliceSpriteTemplate gFalseSwipePositionedSliceSpriteTemplate gEndureEnergyAnimCmds gEndureEnergyAnimTable gEndureEnergySpriteTemplate gSharpenSphereAnimCmds gSharpenSphereAnimTable gSharpenSphereSpriteTemplate gOctazookaBallSpriteTemplate gOctazookaAnimCmds gOctazookaAnimTable gOctazookaSmokeSpriteTemplate gConversionAnimCmds gConversionAnimTable gConversionAffineAnimCmds gConversionAffineAnimTable gConversionSpriteTemplate gConversion2AnimCmds gConversion2AnimTable gConversion2SpriteTemplate gMoonSpriteTemplate gMoonlightSparkleAnimCmds gMoonlightSparkleAnimTable gMoonlightSparkleSpriteTemplate gHealingBlueStarAnimCmds gHealingBlueStarAnimTable gHealingBlueStarSpriteTemplate gHornHitSpriteTemplate gSuperFangAnimCmds gSuperFangAnimTable gSuperFangSpriteTemplate gWavyMusicNotesAnimCmds1 gWavyMusicNotesAnimCmds2 gWavyMusicNotesAnimCmds3 gWavyMusicNotesAnimCmds4 gWavyMusicNotesAnimCmds5 gWavyMusicNotesAnimCmds6 gWavyMusicNotesAnimCmds7 gWavyMusicNotesAnimCmds8 gMusicNotesAnimTable gWavyMusicNotesAffineAnimCmds gMusicNotesAffineAnimTable gWavyMusicNotesSpriteTemplate gParticlesColorBlendTable gFastFlyingMusicNotesSpriteTemplate gBellyDrumHandSpriteTemplate gSlowFlyingMusicNotesAffineAnimCmds gSlowFlyingMusicNotesAffineAnimTable gSlowFlyingMusicNotesSpriteTemplate gMetronomeThroughtBubbleAnimCmds1 gMetronomeThroughtBubbleAnimCmds3 gMetronomeThroughtBubbleAnimCmds2 gMetronomeThroughtBubbleAnimCmds4 gMetronomeThroughtBubbleAnimTable gThoughtBubbleSpriteTemplate gMetronomeFingerAffineAnimCmds1 gMetronomeFingerAffineAnimCmds2 gMetronomeFingerAffineAnimCmds2_2 gMetronomeFingerAffineAnimTable gMetronomeFingerSpriteTemplate gFollowMeFingerSpriteTemplate gTauntFingerAnimCmds1 gTauntFingerAnimCmds2 gTauntFingerAnimCmds3 gTauntFingerAnimCmds4 gTauntFingerAnimTable gTauntFingerSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_effects_1::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sFrenzyPlantRootData: crate::ffi::Align4<[u8; 8]> =
    crate::ffi::Align4([0; 8]);

unsafe extern "C" {
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlersCount: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimFastTranslateLinear(a0: *mut u8) -> u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn ChangeSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAndAnimate(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimSpriteAndDisableBlend(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroySpriteWithActiveSheet(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleMonSpritePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8) -> u32;
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoord2(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAndRunAnimFastLinearTranslation(a0: *mut u8);
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitAnimFastLinearTranslationWithSpeed(a0: *mut u8);
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn LoadPointerFromVars(a0: i16, a1: i16) -> *mut u8;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn Random2() -> u16;
    fn ResetSpriteRotScale(a0: u8);
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAnimSpriteInitialXOffset(a0: *mut u8, a1: i16);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetBattlerSpriteYOffsetFromRotation(a0: u8);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetHealthboxSpriteInvisible(a0: u8);
    fn SetHealthboxSpriteVisible(a0: u8);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut u8);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StorePointerInVars(a0: *mut i16, a1: *mut i16, a2: *mut u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn TranslateSpriteLinear(a0: *mut u8);
    fn TranslateSpriteLinearAndFlicker(a0: *mut u8);
    fn TranslateSpriteLinearById(a0: *mut u8);
    fn TranslateSpriteLinearFixedPoint(a0: *mut u8);
    fn TrySetSpriteRotScale(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: u16);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimMovePowderParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMovePowderParticle_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimMovePowderParticle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 8) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
            ));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )
                    & 255i32) as i16),
            );
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPowerAbsorptionOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    }
}
pub(crate) unsafe extern "C" fn AnimSolarBeamBigOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        StartSpriteAnim(
            sprite,
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimSolarBeamSmallOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSolarBeamSmallOrb_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSolarBeamSmallOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            DestroySprite(sprite);
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > 127i32
            {
                ((sprite).wrapping_add(67)).write(
                    ((((GetBattlerSpriteSubpriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as i32)
                        .wrapping_add(1i32)) as u8),
                );
            } else {
                ((sprite).wrapping_add(67)).write(
                    ((((GetBattlerSpriteSubpriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as i32)
                        .wrapping_add(6i32)) as u8),
                );
            }
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        5i16,
                    )) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        14i16,
                    )) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(15i32)
                    & 255i32) as i16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CreateSmallSolarBeamOrbs(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == (-1i32)
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(6i16);
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(15i16);
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .write(0i16);
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .write(80i16);
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .write(0i16);
            CreateSpriteAndAnimate((&raw const gSolarBeamSmallOrbSpriteTemplate).cast::<u8>().cast_mut(), 0i16, 0i16, ((((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))).wrapping_add(1i32)) as u8));
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 15i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimAbsorptionOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimTarget(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimAbsorptionOrb_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimAbsorptionOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHyperBeamOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut speed: u16 = 0u16;
        let mut animNum: u16 = Random2();
        StartSpriteAnim(
            sprite,
            ((crate::c::rem_i32(((animNum) as i32), 8i32)) as u8),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(20i32)) as i16));
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(20i32)) as i16));
        }
        speed = Random2();
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((((speed) as i32) & 31i32).wrapping_add(64i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        InitAnimFastLinearTranslationWithSpeed(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((Random2()) as i32) & 255i32) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((((sprite).wrapping_add(67)).read()) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimHyperBeamOrb_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimHyperBeamOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimFastTranslateLinear(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        } else {
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        12i16,
                    )) as i32),
                )) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                < 127i32
            {
                ((sprite).wrapping_add(67)).write(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                );
            } else {
                ((sprite).wrapping_add(67)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_add(1i32)) as u8),
                );
            }
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(24i32)) as i16));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLeechSeed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((cmd).wrapping_add(4).cast::<i16>()).write(
                ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimLeechSeed_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimLeechSeed_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimForDuration));
            StoreSpriteCallbackInData6(sprite, Some(AnimLeechSeedSprouts));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLeechSeedSprouts(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        StartSpriteAnim(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(60i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimSporeParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimTarget(sprite, 1u8);
        StartSpriteAnim(
            sprite,
            ((((cmd).wrapping_add(8).cast::<i16>()).read()) as u8),
        );
        if ((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32) == 1i32 {
            crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (1u32) as i32);
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSporeParticle_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSporeParticle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            32i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                (-3i16),
            )) as i32)
                .wrapping_add(
                    ((({
                        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __v2 = (((((__p1).read()) as i32).wrapping_add(24i32)) as i16);
                        (__p1).write(__v2);
                        __v2
                    }) as i32)
                        >> 8),
                )) as i16),
        );
        if (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            .wrapping_sub(64i32)) as u16) as i32)
            < 128i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as u16) as i32,
            );
        } else {
            let mut priority: u8 =
                ((((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as i32)
                    .wrapping_add(1i32)) as u8);
            if ((priority) as i32) > 3i32 {
                priority = 3u8;
            }
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, ((priority) as u16) as i32);
        }
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p4).write((((((__p4).read()) as i32) & 255i32) as i16));
        if (({
            let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t6 = ((__p5).read()).wrapping_sub(1);
            (__p5).write(__t6);
            __t6
        }) as i32)
            == (-1i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SporeDoubleBattle(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((IsContest()) != 0) || (!((IsDoubleBattle()) != 0)) {
            DestroyAnimVisualTask(taskId);
        } else {
            if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                as i32)
                == 1i32
            {
                SetAnimBgAttribute(2u8, 4u8, 3u8);
            } else {
                SetAnimBgAttribute(1u8, 4u8, 1u8);
            }
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPetalDanceBigFlower(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 0u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(64i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimPetalDanceBigFlower_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimPetalDanceBigFlower_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        32i16,
                    )) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        (-5i16),
                    )) as i32),
                )) as i16),
            );
            if (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32)
                .wrapping_sub(64i32)) as u16) as i32)
                < 128i32
            {
                ((sprite).wrapping_add(67)).write(
                    ((((GetBattlerSpriteSubpriority(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    )) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
            } else {
                ((sprite).wrapping_add(67)).write(
                    ((((GetBattlerSpriteSubpriority(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    )) as i32)
                        .wrapping_add(1i32)) as u8),
                );
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(5i32)
                    & 255i32) as i16),
            );
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPetalDanceSmallFlower(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(64i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimPetalDanceSmallFlower_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimPetalDanceSmallFlower_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        8i16,
                    )) as i32),
                )) as i16),
            );
            if ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32)
                .wrapping_sub(59i32)) as u16) as i32)
                < 5i32)
                || ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_sub(187i32)) as u16) as i32)
                    < 5i32)
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(3),
                    1,
                    5,
                    ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) ^ 8u32)
                        as i32,
                );
            }
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(5i32)) as i16));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRazorLeafParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((cmd).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRazorLeafParticle_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimRazorLeafParticle_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) != 0) {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 1i32)
                != 0
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(128i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            } else {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            }
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimRazorLeafParticle_Step2));
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRazorLeafParticle_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin((((sprite).wrapping_add(46)).cast::<i16>()).read(), 25i16)) as i32)
                    .wrapping_neg()) as i16),
            );
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                25i16,
            ));
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32) & 255i32) as i16));
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p3).write(((__p3).read()).wrapping_add(1));
        if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            & 1i32)
            != 0)
        {
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 80i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTranslateLinearSingleSineWave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((cmd).wrapping_add(4).cast::<i16>()).write(
                ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        if !((((cmd).wrapping_add(12).cast::<i16>()).read()) != 0) {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i32)
                    .wrapping_add(((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                1u8,
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2),
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        InitAnimArcTranslation(sprite);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
            == ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTranslateLinearSingleSineWave_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTranslateLinearSingleSineWave_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut destroy: u8 = 0u8;
        let mut a: i16 = (((sprite).wrapping_add(46)).cast::<i16>()).read();
        let mut b: i16 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read();
        let mut r0: i16 = 0i16;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        TranslateAnimHorizontalArc(sprite);
        r0 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read();
        (((sprite).wrapping_add(46)).cast::<i16>()).write(a);
        if ((((b) as i32) > 200i32) && (((r0) as i32) < 56i32))
            && (((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32) == 0i32)
        {
            let __p1 = (sprite).wrapping_add(6).cast::<u16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((sprite).wrapping_add(6).cast::<u16>()).read()) != 0)
            && (((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            let __p2 = (sprite).wrapping_add(6).cast::<u16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            if ((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32) == 30i32 {
                destroy = 1u8;
            }
        }
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
            > 256i32)
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                < (-16i32)))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                > 160i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                < (-16i32))
        {
            destroy = 1u8;
        }
        if (destroy) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimMoveTwisterParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((IsDoubleBattle()) as i32) == 1i32 {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                1u8,
                (sprite).wrapping_add(32).cast::<i16>(),
                (sprite).wrapping_add(34).cast::<i16>(),
            );
        }
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(32i32)) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((cmd).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMoveTwisterParticle_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimMoveTwisterParticle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 255i32
        {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 0i32
            {
                let __p2 = (sprite).wrapping_add(34).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(2i32)) as i16));
            }
        }
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
        }
        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p6).write((((((__p6).read()) as i32) & 255i32) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            5i16,
        ));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            < 128i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as i32)
                    .wrapping_sub(1i32)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as i32)
                    .wrapping_add(1i32)) as u16) as i32,
            );
        }
        if (({
            let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t8 = ((__p7).read()).wrapping_sub(1);
            (__p7).write(__t8);
            __t8
        }) as i32)
            == 0i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimConstrictBinding(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimTarget(sprite, 0u8);
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(4).cast::<i16>()).read()) as u8),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimConstrictBinding_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimConstrictBinding_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
            spriteId = GetAnimBattlerSpriteId(1u8);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(256i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimConstrictBinding_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimConstrictBinding_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) != 0) {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(11i32)) as i16));
        } else {
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(11i32)) as i16));
        }
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 6i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p5).write((((((__p5).read()) as i32) ^ 1i32) as i16));
        }
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            if (({
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                let __t7 = ((__p6).read()).wrapping_sub(1);
                (__p6).write(__t7);
                __t7
            }) as i32)
                > 0i32
            {
                StartSpriteAffineAnim(
                    sprite,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as u8),
                );
            } else {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShrinkTargetCopy(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            false,
        ) as u16)
            != 0
        {
            DestroyAnimVisualTask(taskId);
        } else {
            PrepareBattlerSpriteForRotScale(spriteId, 1u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14))
            .write(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as u16) as i32,
            );
            spriteId = GetAnimBattlerSpriteId(3u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    false,
                ) as u16) as i16),
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(
                    ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                )) as u16) as i32,
            );
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((cmd).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(256i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_DuplicateAndShrinkToPos_Step1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DuplicateAndShrinkToPos_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32),
            )) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
        .write(
            ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                >> 8) as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(16i32)) as i16));
        SetSpriteRotScale(
            spriteId,
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read(),
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read(),
            0u16,
        );
        SetBattlerSpriteYOffsetFromYScale(spriteId);
        if (({
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            let __t4 = ((__p3).read()).wrapping_sub(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 0i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_DuplicateAndShrinkToPos_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DuplicateAndShrinkToPos_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 0i32
            {
                let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
                ResetSpriteRotScale(spriteId);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(14))
                    .read()) as u16) as i32,
                );
                spriteId = GetAnimBattlerSpriteId(3u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    2,
                    2,
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as u16) as i32,
                );
                let __p1 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_add(1));
                return;
            }
        } else {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 0i32
            {
                return;
            }
        }
        let __p2 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 3i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMimicOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    let __p2 = (cmd).cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
                }
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        0u8,
                    )) as i32)
                        .wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        1u8,
                    )) as i32)
                        .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    ChangeSpriteAffineAnim(sprite, 1u8);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(25i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            2u8,
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            3u8,
                        )) as i16),
                    );
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(InitAndRunAnimFastLinearTranslation));
                    StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
                    break 'l1;
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimIngrainRoot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(((cmd).cast::<i16>()).read());
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(((cmd).wrapping_add(2).cast::<i16>()).read());
            ((sprite).wrapping_add(67)).write(
                ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_add(30i32))
                    as u8),
            );
            StartSpriteAnim(
                sprite,
                ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((cmd).wrapping_add(8).cast::<i16>()).read());
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                > 120i32
            {
                let __p2 = (sprite).wrapping_add(34).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32),
                        ))
                        .wrapping_sub(120i32),
                    )) as i16),
                );
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRootFlickerOut));
    }
}
pub(crate) unsafe extern "C" fn AnimFrenzyPlantRoot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut attackerX: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16);
        let mut attackerY: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16);
        let mut targetX: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16);
        let mut targetY: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16);
        targetX = ((((targetX) as i32).wrapping_sub(((attackerX) as i32))) as i16);
        targetY = ((((targetY) as i32).wrapping_sub(((attackerY) as i32))) as i16);
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((attackerX) as i32).wrapping_add(crate::c::div_i32(
                ((targetX) as i32).wrapping_mul(((((cmd).cast::<i16>()).read()) as i32)),
                100i32,
            ))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((attackerY) as i32).wrapping_add(crate::c::div_i32(
                ((targetY) as i32).wrapping_mul(((((cmd).cast::<i16>()).read()) as i32)),
                100i32,
            ))) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((sprite).wrapping_add(67)).write(
            ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_add(30i32)) as u8),
        );
        StartSpriteAnim(
            sprite,
            ((((cmd).wrapping_add(8).cast::<i16>()).read()) as u8),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRootFlickerOut));
        (((&raw mut sFrenzyPlantRootData).cast::<u8>()).cast::<i16>())
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        (((&raw mut sFrenzyPlantRootData).cast::<u8>())
            .wrapping_add(2)
            .cast::<i16>())
        .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        (((&raw mut sFrenzyPlantRootData).cast::<u8>())
            .wrapping_add(4)
            .cast::<i16>())
        .write(targetX);
        (((&raw mut sFrenzyPlantRootData).cast::<u8>())
            .wrapping_add(6)
            .cast::<i16>())
        .write(targetY);
    }
}
pub(crate) unsafe extern "C" fn AnimRootFlickerOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_sub(10i32)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((crate::c::rem_i32(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    2i32,
                )) as u16) as i32,
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimIngrainOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((cmd).wrapping_add(4).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((cmd).wrapping_add(6).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_mul((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(20i32)
                & 255i32) as i16),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn InitItemBagData(sprite: *mut u8, c: i16) {
    unsafe {
        let mut sprite = sprite;
        let mut c = c;
        let mut a: i32 = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 8)
            | ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32));
        let mut b: i32 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .read()) as i32)
            << 8)
            | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32));
        c = ((((c) as i32) << 8) as i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(((a) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(((b) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(c);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn moveAlongLinearPath(sprite: *mut u8) -> u8 {
    unsafe {
        let mut sprite = sprite;
        let mut xStartPos: u16 =
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >> 8) as u8) as u16);
        let mut yStartPos: u16 = (((((((sprite).wrapping_add(46)).cast::<i16>())
            .wrapping_offset(5))
        .read()) as u8) as u16);
        let mut xEndPos: i32 =
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                >> 8) as u8) as i32);
        let mut yEndPos: i32 = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .read()) as u8) as i32);
        let mut totalTime: i16 =
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                >> 8) as i16);
        let mut currentTime: i16 =
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                & 255i32) as i16);
        let mut yEndPos_2: i16 = 0i16;
        let mut r0: i16 = 0i16;
        let mut var1: i32 = 0i32;
        let mut vaxEndPos: i32 = 0i32;
        if xEndPos == 0i32 {
            xEndPos = (-32i32);
        } else {
            if xEndPos == 255i32 {
                xEndPos = 272i32;
            }
        }
        yEndPos_2 = (((yEndPos).wrapping_sub(((yStartPos) as i32))) as i16);
        r0 = (((xEndPos).wrapping_sub(((xStartPos) as i32))) as i16);
        var1 = crate::c::div_i32(
            ((r0) as i32).wrapping_mul(((currentTime) as i32)),
            ((totalTime) as i32),
        );
        vaxEndPos = crate::c::div_i32(
            ((yEndPos_2) as i32).wrapping_mul(((currentTime) as i32)),
            ((totalTime) as i32),
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((var1).wrapping_add(((xStartPos) as i32))) as i16));
        ((sprite).wrapping_add(34).cast::<i16>())
            .write((((vaxEndPos).wrapping_add(((yStartPos) as i32))) as i16));
        if (({
            let __t1 = (currentTime).wrapping_add(1);
            currentTime = __t1;
            __t1
        }) as i32)
            == ((totalTime) as i32)
        {
            return 1u8;
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write((((((totalTime) as i32) << 8) | ((currentTime) as i32)) as i16));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AnimItemSteal_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 10i32 {
            StartSpriteAffineAnim(sprite, 1u8);
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 50i32 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimItemSteal_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ))) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >= 128i32 {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_add(1));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(128i32))
                as i16),
            (((30i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_mul(8i32),
            )) as i16),
        ));
        if (moveAlongLinearPath(sprite)) != 0 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimItemSteal_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPresent(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut targetX: i16 = 0i16;
        let mut targetY: i16 = 0i16;
        InitSpritePosToAnimAttacker(sprite, 0u8);
        targetX = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
            as i16);
        targetY = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
            as i16);
        if (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
            == ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(targetX);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((targetY) as i32).wrapping_add(10i32)) as i16));
            InitItemBagData(sprite, 60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(targetX);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((targetY) as i32).wrapping_add(10i32)) as i16));
            InitItemBagData(sprite, 60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(3i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(60i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimItemSteal_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimKnockOffOpponentsItem(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ))) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 127i32 {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_add(1));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(128i32))
                as i16),
            (((30i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_mul(8i32),
            )) as i16),
        ));
        if (moveAlongLinearPath(sprite)) != 0 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimKnockOffItem(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut targetY: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                as i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((targetY) as i32).wrapping_add(10i32)) as i16));
            InitItemBagData(sprite, 40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(3i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(60i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimItemSteal_Step1));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(255i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((targetY) as i32).wrapping_add(10i32)) as i16));
            if (IsContest()) != 0 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            }
            InitItemBagData(sprite, 40i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(3i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(60i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimKnockOffOpponentsItem));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPresentHealParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            InitSpritePosToAnimTarget(sprite, 0u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_mul((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimItemSteal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut attackerX: i16 = 0i16;
        let mut attackerY: i16 = 0i16;
        InitSpritePosToAnimTarget(sprite, 0u8);
        attackerX =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i16);
        attackerY =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i16);
        if (((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
            == ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(attackerX);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((attackerY) as i32).wrapping_add(10i32)) as i16));
            InitItemBagData(sprite, 60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(attackerX);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((attackerY) as i32).wrapping_add(10i32)) as i16));
            InitItemBagData(sprite, 60i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(3i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(60i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimItemSteal_Step3));
    }
}
pub(crate) unsafe extern "C" fn AnimItemSteal_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    .wrapping_mul(128i32),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            ))) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 127i32 {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_add(1));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(128i32))
                as i16),
            (((30i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_mul(8i32),
            )) as i16),
        ));
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) == 0i32 {
            PlaySE12WithPanning(125u16, BattleAnimAdjustPanning(63i8));
        }
        if (moveAlongLinearPath(sprite)) != 0 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimItemSteal_Step2));
            PlaySE12WithPanning(125u16, BattleAnimAdjustPanning((-64i8)));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTrickBag(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            if !((IsContest()) != 0) {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((cmd).wrapping_add(2).cast::<i16>()).read());
                ((sprite).wrapping_add(32).cast::<i16>()).write(120i16);
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((crate::c::rem_i32(
                        ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_sub(32i32),
                        256i32,
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(70i16);
            }
            ((sprite).wrapping_add(34).cast::<i16>()).write(((cmd).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((cmd).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(20i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                60i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                20i16,
            ));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimTrickBag_Step1));
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 0i32)
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    < 192i32)
            {
                ((sprite).wrapping_add(67)).write(31u8);
            } else {
                ((sprite).wrapping_add(67)).write(29u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTrickBag_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32);
            if __sw1 == 0i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 78i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
                    StartSpriteAffineAnim(sprite, 1u8);
                    break 'l1;
                } else {
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(crate::c::div_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                            10i32,
                        ))) as i16),
                    );
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p3).write((((((__p3).read()) as i32).wrapping_add(3i32)) as i16));
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    );
                    break 'l1;
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0)
                    && ((crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0)
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimTrickBag_Step2));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTrickBag_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == ((((((((&raw const gTrickBagCoordinates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 3,
                ))
            .cast::<i8>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            if ((((((((&raw const gTrickBagCoordinates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 3,
                ))
            .cast::<i8>())
            .wrapping_offset(2))
            .read()) as i32)
                == 127i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimTrickBag_Step3));
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((((&raw const gTrickBagCoordinates).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 3,
                    ))
                .cast::<i8>())
                .read()) as i32)
                    .wrapping_mul(
                        ((((((((&raw const gTrickBagCoordinates).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize
                                * 3,
                        ))
                        .cast::<i8>())
                        .wrapping_offset(2))
                        .read()) as i32),
                    ))
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                ) & 255i32) as i16),
            );
            if !((IsContest()) != 0) {
                if (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_sub(1i32)) as u16) as i32)
                    < 191i32
                {
                    ((sprite).wrapping_add(67)).write(31u8);
                } else {
                    ((sprite).wrapping_add(67)).write(29u8);
                }
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                60i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                20i16,
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTrickBag_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 20i32 {
            DestroyAnimSprite(sprite);
        }
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((crate::c::rem_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                2i32,
            )) as u16) as i32,
        );
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_LeafBlade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                as i32)
                .wrapping_sub(1i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
            GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
            GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                == 1i32
            {
                1i32
            } else {
                (-1i32)
            }) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(
            (((56i32).wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    .wrapping_mul(64i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read()) as i32),
                ))
            .wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            ((CreateSprite(
                (&raw const gLeafBladeSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read(),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
            )) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) == 64i32
        {
            DestroyAnimVisualTask(taskId);
        }
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(10i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read());
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_sub(
                    ((crate::c::div_i32(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32),
                        2i32,
                    ))
                    .wrapping_add(10i32))
                    .wrapping_mul(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    ),
                )) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read());
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_add(
                    ((crate::c::div_i32(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            as i32),
                        2i32,
                    ))
                    .wrapping_add(10i32))
                    .wrapping_mul(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    ),
                )) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(LeafBladeGetPosFactor(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize
                    * 68,
            ),
        ));
        InitAnimArcTranslation(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_LeafBlade_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_LeafBlade_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        );
        let mut a: i32 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
        'l1: {
            let __sw1 = a;
            if __sw1 == 4i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(5i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(255i16);
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(9i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(255i16);
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(1i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(255i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(LeafBladeGetPosFactor(sprite));
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(((a) as i16));
                ((sprite).wrapping_add(67)).write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                );
                StartSpriteAnim(
                    sprite,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                );
                InitAnimArcTranslation(sprite);
                let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(3i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(255i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p6 = (sprite).wrapping_add(32).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p7 = (sprite).wrapping_add(34).cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_sub(
                            ((crate::c::div_i32(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32),
                                2i32,
                            ))
                            .wrapping_add(10i32))
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            ),
                        )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_sub(
                            ((crate::c::div_i32(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                    .read()) as i32),
                                2i32,
                            ))
                            .wrapping_add(10i32))
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            ),
                        )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(LeafBladeGetPosFactor(sprite));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(2i16);
                ((sprite).wrapping_add(67)).write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                );
                StartSpriteAnim(
                    sprite,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                );
                InitAnimArcTranslation(sprite);
                let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                let __p9 = (sprite).wrapping_add(32).cast::<i16>();
                (__p9).write(
                    (((((__p9).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p10 = (sprite).wrapping_add(34).cast::<i16>();
                (__p10).write(
                    (((((__p10).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_add(
                            ((crate::c::div_i32(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32),
                                2i32,
                            ))
                            .wrapping_add(10i32))
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            ),
                        )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_add(
                            ((crate::c::div_i32(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                    .read()) as i32),
                                2i32,
                            ))
                            .wrapping_add(10i32))
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            ),
                        )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(LeafBladeGetPosFactor(sprite));
                let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p11).write((((((__p11).read()) as i32).wrapping_sub(2i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(3i16);
                ((sprite).wrapping_add(67)).write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                );
                StartSpriteAnim(
                    sprite,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                );
                InitAnimArcTranslation(sprite);
                let __p12 = ((task).wrapping_add(8)).cast::<i16>();
                (__p12).write(((__p12).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 6i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(7i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(255i16);
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                let __p13 = (sprite).wrapping_add(32).cast::<i16>();
                (__p13).write(
                    (((((__p13).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p14 = (sprite).wrapping_add(34).cast::<i16>();
                (__p14).write(
                    (((((__p14).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(LeafBladeGetPosFactor(sprite));
                let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p15).write((((((__p15).read()) as i32).wrapping_add(2i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(4i16);
                ((sprite).wrapping_add(67)).write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                );
                StartSpriteAnim(
                    sprite,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                );
                InitAnimArcTranslation(sprite);
                let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 9i32 {
                let __p17 = (sprite).wrapping_add(32).cast::<i16>();
                (__p17).write(
                    (((((__p17).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p18 = (sprite).wrapping_add(34).cast::<i16>();
                (__p18).write(
                    (((((__p18).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_sub(
                            ((crate::c::div_i32(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32),
                                2i32,
                            ))
                            .wrapping_add(10i32))
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            ),
                        )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_add(
                            ((crate::c::div_i32(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                    .read()) as i32),
                                2i32,
                            ))
                            .wrapping_add(10i32))
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            ),
                        )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(LeafBladeGetPosFactor(sprite));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(5i16);
                ((sprite).wrapping_add(67)).write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                );
                StartSpriteAnim(
                    sprite,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                );
                InitAnimArcTranslation(sprite);
                let __p19 = ((task).wrapping_add(8)).cast::<i16>();
                (__p19).write(((__p19).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 10i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(11i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(255i16);
                }
                break 'l1;
            }
            if __sw1 == 11i32 {
                {
                    let __p20 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p20).write(
                        (((((__p20).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                        )) as i16),
                    );
                    let __p21 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p21).write(
                        (((((__p21).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32),
                        )) as i16),
                    );
                    ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read(),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                        .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read(),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                        .write(LeafBladeGetPosFactor(sprite));
                    let __p22 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p22).write((((((__p22).read()) as i32).wrapping_sub(2i32)) as i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(6i16);
                    ((sprite).wrapping_add(67)).write(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as u8),
                    );
                    StartSpriteAnim(
                        sprite,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as u8),
                    );
                    InitAnimArcTranslation(sprite);
                    let __p23 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p23).write(((__p23).read()).wrapping_add(1));
                    break 'l1;
                }
            }
            if __sw1 == 12i32 {
                AnimTask_LeafBlade_Step2(task, taskId);
                if (TranslateAnimHorizontalArc(sprite)) != 0 {
                    DestroySprite(sprite);
                    let __p24 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p24).write(((__p24).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 13i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
            if __sw1 == 255i32 {
                if (({
                    let __p25 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t26 = ((__p25).read()).wrapping_add(1);
                    (__p25).write(__t26);
                    __t26
                }) as i32)
                    > 5i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    (((task).wrapping_add(8)).cast::<i16>()).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                    );
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn LeafBladeGetPosFactor(sprite: *mut u8) -> i16 {
    unsafe {
        let mut sprite = sprite;
        let mut var: i16 = 8i16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            < ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
        {
            var = ((((var) as i32).wrapping_neg()) as i16);
        }
        return var;
    }
}
pub(crate) unsafe extern "C" fn AnimTask_LeafBlade_Step2(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32) > 0i32 {
            let mut spriteId: u8 = 0u8;
            let mut spriteX: i16 = 0i16;
            let mut spriteY: i16 = 0i16;
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(0i16);
            spriteX = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16);
            spriteY = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16);
            spriteId = CreateSprite(
                (&raw const gLeafBladeSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                spriteX,
                spriteY,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
            );
            if ((spriteId) as i32) != 64i32 {
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(((taskId) as i16));
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(12i16);
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p2).write(((__p2).read()).wrapping_add(1));
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                        as i32)
                        & 1i32) as i16),
                );
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13);
                (__p3).write(((__p3).read()).wrapping_add(1));
                StartSpriteAnim(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(67))
                .write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimTask_LeafBlade_Step2_Callback));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_LeafBlade_Step2_Callback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 1i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 8i32
            {
                let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize,
                );
                (__p3).write(((__p3).read()).wrapping_sub(1));
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlyingParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: u8 = 0u8;
        if !((((cmd).wrapping_add(12).cast::<i16>()).read()) != 0) {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        if ((GetBattlerSide(battler)) as i32) != 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((cmd).wrapping_add(6).cast::<i16>()).read());
            ((sprite).wrapping_add(32).cast::<i16>()).write((-16i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(256i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        'l1: {
            let __sw1 = ((((cmd).wrapping_add(10).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(34).cast::<i16>()).write(((cmd).cast::<i16>()).read());
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((GetBattlerSpriteBGPriority(battler)) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite).wrapping_add(34).cast::<i16>()).write(((cmd).cast::<i16>()).read());
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((((GetBattlerSpriteBGPriority(battler)) as i32).wrapping_add(1i32)) as u16)
                        as i32,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoord(battler, 3u8)) as i32)
                        .wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((GetBattlerSpriteBGPriority(battler)) as u16) as i32,
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        3u8,
                    )) as i32)
                        .wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                GetAnimBattlerSpriteId(1u8);
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((((GetBattlerSpriteBGPriority(battler)) as i32).wrapping_add(1i32)) as u16)
                        as i32,
                );
                break 'l1;
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFlyingParticle_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFlyingParticle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut a: i32 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_mul(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32),
                )
                >> 8) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_mul(a)) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                .wrapping_mul(a)
                & 255i32) as i16),
        );
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) != 0) {
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                < 248i32
            {
                return;
            }
        } else {
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                > (-16i32)
            {
                return;
            }
        }
        DestroySpriteAndMatrix(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CycleMagicalLeafPal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(
                    (((256i32).wrapping_add(
                        ((IndexOfSpritePaletteTag(10063u16)) as i32).wrapping_mul(16i32),
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                    (((256i32).wrapping_add(
                        ((IndexOfSpritePaletteTag(10160u16)) as i32).wrapping_mul(16i32),
                    )) as i16),
                );
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    >= 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
                    BlendPalette(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as u16),
                        16u16,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as u8),
                        ((((&raw const gMagicalLeafBlendColors)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                                as i32) as isize,
                        ))
                        .read(),
                    );
                    BlendPalette(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as u16),
                        16u16,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as u8),
                        ((((&raw const gMagicalLeafBlendColors)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                                as i32) as isize,
                        ))
                        .read(),
                    );
                    if (({
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        == 17i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(0i16);
                        if (({
                            let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                            let __t8 = ((__p7).read()).wrapping_add(1);
                            (__p7).write(__t8);
                            __t8
                        }) as i32)
                            == 7i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                .write(0i16);
                        }
                    }
                }
                break 'l1;
            }
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            == (-1i32)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimNeedleArmSpike(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut a: u8 = 0u8;
        let mut b: u8 = 0u8;
        let mut c: u16 = 0u16;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        if ((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32) == 0i32 {
            DestroyAnimSprite(sprite);
        } else {
            if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
                a = GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                );
                b = GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    3u8,
                );
            } else {
                a = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8);
                b = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8);
            }
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write(((cmd).wrapping_add(8).cast::<i16>()).read());
            if ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) == 0i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)
                        .wrapping_add(((a) as i32))) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)
                        .wrapping_add(((b) as i32))) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                    .write(((a) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                    .write(((b) as i16));
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write(((a) as i16));
                ((sprite).wrapping_add(34).cast::<i16>()).write(((b) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                    ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)
                        .wrapping_add(((a) as i32))) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)
                        .wrapping_add(((b) as i32))) as i16),
                );
            }
            x = ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((((x) as i32).wrapping_mul(16i32)) as i16));
            y = ((((sprite).wrapping_add(34).cast::<i16>()).read()) as u16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((((y) as i32).wrapping_mul(16i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((crate::c::div_i32(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                    .wrapping_mul(16i32),
                    ((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((crate::c::div_i32(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                    .wrapping_mul(16i32),
                    ((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            c = ArcTan2Neg(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_sub(((x) as i32))) as i16),
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    .wrapping_sub(((y) as i32))) as i16),
            );
            if (IsContest()) != 0 {
                c = ((((c) as i32).wrapping_sub(32768i32)) as u16);
            }
            TrySetSpriteRotScale(sprite, 0u8, 256i16, 256i16, c);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimNeedleArmSpike_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimNeedleArmSpike_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    >> 4) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 4) as i16),
            );
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
        } else {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWhipHit_WaitEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSlidingHit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimWhipHit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            StartSpriteAnim(sprite, 1u8);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWhipHit_WaitEnd));
        SetAnimSpriteInitialXOffset(sprite, ((cmd).cast::<i16>()).read());
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimFlickeringPunch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(12).cast::<i16>()).read()) as u8),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearAndFlicker));
    }
}
pub(crate) unsafe extern "C" fn AnimCuttingSlice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSlice_Step));
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) == 0i32 {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        }
        let __p4 = (sprite).wrapping_add(34).cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p5).write((((((__p5).read()) as i32).wrapping_sub(1024i32)) as i16));
        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p6).write((((((__p6).read()) as i32).wrapping_add(1024i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimAirCutterSlice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        'l1: {
            let __sw1 = ((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 0i32;
            if __sw1 == 1i32 {
                x = GetBattlerSpriteCoord(
                    ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                    0u8,
                );
                y = GetBattlerSpriteCoord(
                    ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                    1u8,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                x = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8);
                y = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8);
                if (IsBattlerSpriteVisible(
                    ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                )) != 0
                {
                    x = ((crate::c::div_i32(
                        ((GetBattlerSpriteCoord(
                            ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
                                as u8),
                            0u8,
                        )) as i32)
                            .wrapping_add(((x) as i32)),
                        2i32,
                    )) as u8);
                    y = ((crate::c::div_i32(
                        ((GetBattlerSpriteCoord(
                            ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32)
                                as u8),
                            1u8,
                        )) as i32)
                            .wrapping_add(((y) as i32)),
                        2i32,
                    )) as u8);
                }
                break 'l1;
            }
            if __sw1 == 0i32 || !__matched {
                x = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8);
                y = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8);
                break 'l1;
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(((x) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(((y) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSlice_Step));
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) == 0i32 {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            let __p4 = (sprite).wrapping_add(32).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        }
        let __p5 = (sprite).wrapping_add(34).cast::<i16>();
        (__p5).write(
            (((((__p5).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p6).write((((((__p6).read()) as i32).wrapping_sub(1024i32)) as i16));
        let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p7).write((((((__p7).read()) as i32).wrapping_add(1024i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSlice_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 0i32
        {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(24i32)) as i16));
        } else {
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p4).write((((((__p4).read()) as i32).wrapping_sub(24i32)) as i16));
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p5).write((((((__p5).read()) as i32).wrapping_sub(24i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 8) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8) as i16),
        );
        let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p6).write(((__p6).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 20i32 {
            StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimForDuration));
        }
    }
}
pub(crate) unsafe extern "C" fn UnusedFlickerAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32) > 1i32
        {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                & 1i32)
                != 0
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 10i32
        {
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ));
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 68,
            ));
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimCirclingMusicNote(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
        StartSpriteAnim(
            sprite,
            ((((cmd).wrapping_add(10).cast::<i16>()).read()) as u8),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
        );
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimCirclingMusicNote_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimCirclingMusicNote_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            100i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            20i16,
        ));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 128i32 {
            ((sprite).wrapping_add(67)).write(0u8);
        } else {
            ((sprite).wrapping_add(67)).write(14u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(130i32)) as i16));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write(((__p3).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimProtect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (IsContest()) != 0 {
            let __p1 = (cmd).wrapping_add(2).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord2(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(((((cmd).cast::<i16>()).read()) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord2(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32)
            || ((IsContest()) != 0)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((GetBattlerSpriteBGPriority(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                )) as i32)
                    .wrapping_add(1i32)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as u16) as i32,
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            (((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(10280u16)) as i32).wrapping_mul(16i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(16i16);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(
            82u8,
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                << 8)
                | (16i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32),
                )) as u16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimProtect_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimProtect_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut id: i32 = 0i32;
        let mut savedPal: i32 = 0i32;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(96i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >> 8)
                .wrapping_neg()) as i16),
        );
        if (({
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            > 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            savedPal = ((((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(1i32)) as isize,
                ))
            .read()) as i32);
            i = 0i32;
            'l1: loop {
                if !(i < 6i32) {
                    break 'l1;
                }
                id = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_add({
                        let __t4 = (i).wrapping_add(1);
                        i = __t4;
                        __t4
                    });
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                    .wrapping_offset((id) as isize))
                .write(
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((id).wrapping_add(1i32)) as isize))
                    .read(),
                );
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_add(7i32)) as isize,
            ))
            .write(((savedPal) as u16));
        }
        if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            > 6i32)
            && ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32))
            && ((({
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as i32)
                > 1i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p7).write((((((__p7).read()) as i32).wrapping_sub(1i32)) as i16));
            SetGpuReg(
                82u8,
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    << 8)
                    | (16i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as u16),
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p8).write((((((__p8).read()) as i32).wrapping_sub(1i32)) as i16));
        } else {
            if (({
                let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                let __t10 = ((__p9).read()).wrapping_add(1);
                (__p9).write(__t10);
                __t10
            }) as i32)
                > 1i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p11).write(((__p11).read()).wrapping_add(1));
                SetGpuReg(
                    82u8,
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        << 8)
                        | (16i32).wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                        )) as u16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    == 16i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(DestroyAnimSpriteAndDisableBlend));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMilkBottle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(65512i32)) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(16i16);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(
            82u8,
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                << 8)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)) as u16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMilkBottle_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimMilkBottle_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    if ((({
                        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        & 1i32)
                        != 0i32
                    {
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                            .read()) as i32)
                            <= 15i32
                        {
                            let __p6 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                            (__p6).write(((__p6).read()).wrapping_add(1));
                        }
                    } else {
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            > 0i32
                        {
                            let __p7 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                            (__p7).write(((__p7).read()).wrapping_sub(1));
                        }
                    }
                    SetGpuReg(
                        82u8,
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            << 8)
                            | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32)) as u16),
                    );
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        == 16i32)
                        && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            == 0i32)
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .write(0i16);
                        let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > 8i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    StartSpriteAffineAnim(sprite, 1u8);
                    let __p11 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                AnimMilkBottle_Step2(sprite, 16i32, 4i32);
                if (({
                    let __p12 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    > 2i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p14 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                if (({
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t16 = ((__p15).read()).wrapping_add(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    <= 29i32
                {
                    break 'l1;
                }
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    & 1i32)
                    != 0
                {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        > 0i32
                    {
                        let __p17 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                        (__p17).write(((__p17).read()).wrapping_sub(1));
                    }
                } else {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        <= 15i32
                    {
                        let __p18 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                        (__p18).write(((__p18).read()).wrapping_add(1));
                    }
                }
                SetGpuReg(
                    82u8,
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        << 8)
                        | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32)) as u16),
                );
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    == 0i32)
                    && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        == 16i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p19 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p19).write(((__p19).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                let __p20 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p20).write(((__p20).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimSprite(sprite);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMilkBottle_Step2(sprite: *mut u8, unk1: i32, unk2: i32) {
    unsafe {
        let mut sprite = sprite;
        let mut unk1 = unk1;
        let mut unk2 = unk2;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            <= 11i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        }
        if (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            .wrapping_sub(18i32)) as u16) as i32)
            <= 23i32
        {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            > 47i32
        {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write((((((__p3).read()) as i32).wrapping_add(2i32)) as i16));
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                9i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                14i32,
            )) as i16),
        );
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) < 0i32 {
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p5).write(((__p5).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            > 59i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGrantingStars(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if !((((cmd).wrapping_add(4).cast::<i16>()).read()) != 0) {
            SetSpriteCoordsToAnimAttackerCoords(sprite);
        }
        SetAnimSpriteInitialXOffset(sprite, ((cmd).cast::<i16>()).read());
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPoint));
    }
}
pub(crate) unsafe extern "C" fn AnimSparklingStars(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: u8 = 0u8;
        if !((((cmd).wrapping_add(4).cast::<i16>()).read()) != 0) {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        if ((IsDoubleBattle()) != 0)
            && ((IsBattlerSpriteVisible(((((battler) as i32) ^ 2i32) as u8))) != 0)
        {
            SetAverageBattlerPositions(
                battler,
                ((((cmd).wrapping_add(12).cast::<i16>()).read()) as u8),
                (sprite).wrapping_add(32).cast::<i16>(),
                (sprite).wrapping_add(34).cast::<i16>(),
            );
            SetAnimSpriteInitialXOffset(sprite, ((cmd).cast::<i16>()).read());
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            if !((((cmd).wrapping_add(12).cast::<i16>()).read()) != 0) {
                ((sprite).wrapping_add(32).cast::<i16>())
                    .write(((GetBattlerSpriteCoord(battler, 0u8)) as i16));
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoord(battler, 1u8)) as i32)
                        .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                        as i16),
                );
            } else {
                ((sprite).wrapping_add(32).cast::<i16>())
                    .write(((GetBattlerSpriteCoord(battler, 2u8)) as i16));
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoord(battler, 3u8)) as i32)
                        .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                        as i16),
                );
            }
            SetAnimSpriteInitialXOffset(sprite, ((cmd).cast::<i16>()).read());
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPoint));
    }
}
pub(crate) unsafe extern "C" fn AnimBubbleBurst(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            StartSpriteAnim(sprite, 1u8);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBubbleBurst_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBubbleBurst_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 30i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((crate::c::div_i32(
                    (30i32).wrapping_sub(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ),
                    3i32,
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_mul(4i32)) as i16),
                3i16,
            ));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSleepLetterZ(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
        } else {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write((-1i16));
            StartSpriteAffineAnim(sprite, 1u8);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSleepLetterZ_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSleepLetterZ_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                40i32,
            ))
            .wrapping_neg()) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                10i32,
            )) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    .wrapping_mul(2i32),
            )) as i16),
        );
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 60i32
        {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(32i32)) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(32i32)) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32)
                & 1i32);
            if __sw1 == 0i32 {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(WaitAnimForDuration));
                StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                        (((((((&raw const gInclineMonCoordTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32)
                                >> 8) as isize
                                * 2,
                        ))
                        .cast::<i8>())
                        .read()) as i32),
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                        ((((((((&raw const gInclineMonCoordTable).cast::<u8>().cast_mut())
                            .cast::<u8>())
                        .wrapping_offset(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32)
                                >> 8) as isize
                                * 2,
                        ))
                        .cast::<i8>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(StartAnimLinearTranslation));
                StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step2));
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(256i32)) as i16));
                PlaySE12WithPanning(210u16, BattleAnimAdjustPanning(63i8));
                break 'l1;
            }
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p5).write((((((__p5).read()) as i32) ^ 1i32) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32) >> 8)
            == 4i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimForDuration));
            StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step3));
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimLockOnTarget_Step1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut a: i16 = 0i16;
        let mut b: i16 = 0i16;
        if ((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32) == 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimForDuration));
            StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step4));
        } else {
            'l1: {
                let __sw1 = ((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32);
                let __matched = __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
                if __sw1 == 1i32 {
                    a = (-8i16);
                    b = (-8i16);
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    a = (-8i16);
                    b = 8i16;
                    break 'l1;
                }
                if __sw1 == 3i32 {
                    a = 8i16;
                    b = (-8i16);
                    break 'l1;
                }
                if !__matched {
                    a = 8i16;
                    b = 8i16;
                    break 'l1;
                }
            }
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i32)
                    .wrapping_add(((a) as i32))) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i32)
                    .wrapping_add(((b) as i32))) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(StartAnimLinearTranslation));
            StoreSpriteCallbackInData6(sprite, Some(AnimLockOnTarget_Step5));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget_Step4(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __v2 = (((((__p1).read()) as i32).wrapping_add(3i32)) as i16);
                (__p1).write(__v2);
                __v2
            }) as i32)
                > 16i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(16i16);
            }
        } else {
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __v4 = (((((__p3).read()) as i32).wrapping_sub(3i32)) as i16);
                (__p3).write(__v4);
                __v4
            }) as i32)
                < 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            }
        }
        BlendPalettes(
            GetBattlePalettesMask(1u8, 1u8, 1u8, 1u8, 1u8, 0u8, 0u8),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            32767u16,
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 16i32
        {
            let mut pal: i32 = 0i32;
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p5).write(((__p5).read()).wrapping_add(1));
            pal = ((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32);
            LoadPalette(
                ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    (((256i32).wrapping_add((pal).wrapping_mul(16i32))).wrapping_add(8i32))
                        as isize,
                ))
                .cast::<u8>(),
                ((((256i32).wrapping_add((pal).wrapping_mul(16i32))).wrapping_add(1i32)) as u16),
                4u16,
            );
            PlaySE12WithPanning(192u16, BattleAnimAdjustPanning(63i8));
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 0i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimLockOnTarget_Step5));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget_Step5(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimLockOnTarget_Step6));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnTarget_Step6(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if crate::c::rem_i32(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            3i32,
        ) == 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        }
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 8i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLockOnMoveTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        ((sprite).wrapping_add(6).cast::<u16>()).write(((((cmd).cast::<i16>()).read()) as u16));
        if ((((cmd).cast::<i16>()).read()) as i32) == 1i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(24i32)) as i16));
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(24i32)) as i16));
        } else {
            if ((((cmd).cast::<i16>()).read()) as i32) == 2i32 {
                let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(24i32)) as i16));
                let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_add(24i32)) as i16));
                crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (16u32) as i32);
            } else {
                if ((((cmd).cast::<i16>()).read()) as i32) == 3i32 {
                    let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p5).write((((((__p5).read()) as i32).wrapping_add(24i32)) as i16));
                    let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p6).write((((((__p6).read()) as i32).wrapping_sub(24i32)) as i16));
                    crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
                } else {
                    let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p7).write((((((__p7).read()) as i32).wrapping_add(24i32)) as i16));
                    let __p8 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p8).write((((((__p8).read()) as i32).wrapping_add(24i32)) as i16));
                    crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (24u32) as i32);
                }
            }
        }
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(16i32)) as u16) as i32,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimLockOnTarget));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        'l1: {
            let __sw1 = ((((cmd).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimBowMon_Step1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimBowMon_Step2));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimBowMon_Step3));
                break 'l1;
            }
            if !__matched {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimBowMon_Step4));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
                2i32
            } else {
                (-2i32)
            }) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        StoreSpriteCallbackInData6(sprite, Some(AnimBowMon_Step1_Callback));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearById));
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon_Step1_Callback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i16),
            );
            PrepareBattlerSpriteForRotScale(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
                0u8,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((if ({
                    let __v1 =
                        ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                            as i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(__v1);
                    __v1
                }) != 0
                {
                    768i32
                } else {
                    (-768i32)
                }) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        SetSpriteRotScale(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            256i16,
            256i16,
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u16),
        );
        SetBattlerSpriteYOffsetFromRotation(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 3i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimBowMon_Step4));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(4i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
                (-3i32)
            } else {
                3i32
            }) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        StoreSpriteCallbackInData6(sprite, Some(AnimBowMon_Step4));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearById));
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 8i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimBowMon_Step3_Callback));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon_Step3_Callback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i16),
            );
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-1024i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(3072i16);
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1024i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-3072i16));
            }
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        SetSpriteRotScale(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            256i16,
            256i16,
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u16),
        );
        SetBattlerSpriteYOffsetFromRotation(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
        );
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            > 2i32
        {
            ResetSpriteRotScale(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimBowMon_Step4));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBowMon_Step4(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimTipMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTipMon_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTipMon_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        != 0i32
                    {
                        512i32
                    } else {
                        (-512i32)
                    }) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                PrepareBattlerSpriteForRotScale(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                    0u8,
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                    256i16,
                    256i16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as u16),
                );
                SetBattlerSpriteYOffsetFromRotation(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                );
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 3i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p6).write((((((__p6).read()) as i32).wrapping_mul((-1i32))) as i16));
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                    256i16,
                    256i16,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as u16),
                );
                SetBattlerSpriteYOffsetFromRotation(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as u8),
                );
                if (({
                    let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > 3i32
                {
                    ResetSpriteRotScale(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as u8),
                    );
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SkullBashPosition(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut side: u8 = 0u8;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        side = GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((side) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        'l1: {
            let __sw1 = ((((cmd).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32;
            if !__matched {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
            if __sw1 == 0i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(8i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(0i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(3i16);
                if ((side) as i32) == 0i32 {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_SkullBashPositionSet));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(8i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1536i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(192i16);
                if ((side) as i32) == 0i32 {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .write(
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_SkullBashPositionReset));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SkullBashPositionSet(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                                as i32),
                        )) as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read());
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(8i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        ((if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            == 0i32
                        {
                            (-192i32)
                        } else {
                            192i32
                        }) as i16),
                    );
                    PrepareBattlerSpriteForRotScale(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                        0u8,
                    );
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p5).write(
                        (((((__p5).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                                as i32),
                        )) as i16),
                    );
                    SetSpriteRotScale(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                        256i16,
                        256i16,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as u16),
                    );
                    SetBattlerSpriteYOffsetFromRotation(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                    );
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(8i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        ((if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            == 0i32
                        {
                            2i32
                        } else {
                            (-2i32)
                        }) as i16),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(1i16);
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
                    if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                        let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                        (__p8).write(((__p8).read()).wrapping_sub(1));
                    } else {
                        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            & 1i32)
                            != 0
                        {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                    .read()) as i32)
                                    .wrapping_add(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(5))
                                        .read()) as i32),
                                    )) as i16),
                            );
                        } else {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                    .read()) as i32)
                                    .wrapping_sub(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(5))
                                        .read()) as i32),
                                    )) as i16),
                            );
                        }
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(1i16);
                        let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p9).write(((__p9).read()).wrapping_sub(1));
                    }
                } else {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read());
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(12i16);
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(3i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read(),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        ((if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            == 0i32
                        {
                            8i32
                        } else {
                            (-8i32)
                        }) as i16),
                    );
                    let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
                    let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p13).write(
                        (((((__p13).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                                as i32),
                        )) as i16),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read());
                    let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p14).write(((__p14).read()).wrapping_sub(1));
                } else {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SkullBashPositionReset(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                )) as i16),
            );
            SetSpriteRotScale(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                256i16,
                256i16,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16),
            );
            SetBattlerSpriteYOffsetFromRotation(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
            );
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(((__p2).read()).wrapping_sub(1));
        } else {
            ResetSpriteRotScale((((((task).wrapping_add(8)).cast::<i16>()).read()) as u8));
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSlashSlice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    3u8,
                )) as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        StoreSpriteCallbackInData6(sprite, Some(AnimFalseSwipeSlice_Step3));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimFalseSwipeSlice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(65488i32)) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        StoreSpriteCallbackInData6(sprite, Some(AnimFalseSwipeSlice_Step1));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimFalseSwipePositionedSlice(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_sub(48i32))
            .wrapping_add(((((cmd).cast::<i16>()).read()) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        StartSpriteAnim(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFalseSwipeSlice_Step3));
    }
}
pub(crate) unsafe extern "C" fn AnimFalseSwipeSlice_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 8i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(8i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            StoreSpriteCallbackInData6(sprite, Some(AnimFalseSwipeSlice_Step2));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(TranslateSpriteLinear));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFalseSwipeSlice_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFalseSwipeSlice_Step3));
    }
}
pub(crate) unsafe extern "C" fn AnimFalseSwipeSlice_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((!((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0))
                    as u16) as i32,
            );
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                > 8i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimEndureEnergy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimEndureEnergy_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimEndureEnergy_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
        let __p4 = (sprite).wrapping_add(34).cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32)
                .wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSharpenSphere(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_sub(12i32)) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((BattleAnimAdjustPanning((-64i8))) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSharpenSphere_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSharpenSphere_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((!((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0))
                    as u16) as i32,
            );
            if !((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0) {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p3).write(((__p3).read()).wrapping_add(1));
                if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    & 1i32)
                    != 0)
                {
                    PlaySE12WithPanning(
                        194u16,
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i8),
                    );
                }
            }
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                > 1i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p6).write(((__p6).read()).wrapping_add(1));
            }
        }
        if (((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 16i32))
            && ((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimConversion(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i32)
                    .wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            if (IsContest()) != 0 {
                let __p1 = (sprite).wrapping_add(34).cast::<i16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_add(10i32)) as i16));
            }
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ConversionAlphaBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == 1i32
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write((-1i16));
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                == 2i32
            {
                DestroyAnimVisualTask(taskId);
            } else {
                if (({
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 4i32
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(0i16);
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        (((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                            << 8)
                            | (16i32).wrapping_sub(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .read()) as i32),
                            )) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        == 16i32
                    {
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimConversion2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimTarget(sprite, 0u8);
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimConversion2_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimConversion2_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(StartAnimLinearTranslation));
            StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Conversion2AlphaBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 4i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(0i16);
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32),
                ) << 8)
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)) as u16),
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 16i32
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_HideBattlersHealthbox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (((((cmd).cast::<i16>()).read()) as i32) == 1i32)
                        && (((GetBattlerSide(i)) as i32) == 0i32)
                    {
                        SetHealthboxSpriteInvisible(
                            (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    if (((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) == 1i32)
                        && (((GetBattlerSide(i)) as i32) == 1i32)
                    {
                        SetHealthboxSpriteInvisible(
                            (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ShowBattlersHealthbox(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    SetHealthboxSpriteVisible(
                        (((&raw mut gHealthboxSpriteIds).cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimMoon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (IsContest()) != 0 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(48i16);
            ((sprite).wrapping_add(34).cast::<i16>()).write(40i16);
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(((cmd).cast::<i16>()).read());
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        }
        crate::c::bf_write((sprite).wrapping_add(1), 6, 2, (0u32) as i32);
        crate::c::bf_write((sprite).wrapping_add(3), 6, 2, (3u32) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMoon_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimMoon_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMoonlightSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((((cmd).cast::<i16>()).read()) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>())
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMoonlightSparkle_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimMoonlightSparkle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 120i32
            {
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
        }
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoonlightEndFade(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut a: i32 =
            ((GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8) & 65535u32) as i32);
        let mut b: i32 = 0i32;
        let mut c: i32 = 0i32;
        let mut d: i32 = 0i32;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((a) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(13i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(14i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(15i16);
        b = ((GetBattleMonSpritePalettesMask(1u8, 1u8, 1u8, 1u8)) as i32);
        c = (a | b);
        StorePointerInVars(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(14),
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15),
            ((c) as usize as *mut u8),
        );
        b = (b | crate::c::shl_i32(65536i32, ((IndexOfSpritePaletteTag(10194u16)) as u32)));
        d = ((IndexOfSpritePaletteTag(10195u16)) as i32);
        BeginNormalPaletteFade(
            ((crate::c::shl_i32(65536i32, ((d) as u32)) | b) as u32),
            0i8,
            0u8,
            16u8,
            32699u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_MoonlightEndFade_Step));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MoonlightEndFade_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 0i32
                {
                    let mut color: u16 = 0u16;
                    let mut bitmask: u16 = 0u16;
                    let mut r3: u16 = 0u16;
                    let mut i: u16 = 0u16;
                    let mut j: u16 = 0u16;
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        <= 15i32
                    {
                        let mut red: u16 = 0u16;
                        let mut green: u16 = 0u16;
                        let mut blue: u16 = 0u16;
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        (__p6).write(
                            (((((__p6).read()) as i32).wrapping_add(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7))
                                    .read()) as i32),
                            )) as i16),
                        );
                        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                        (__p7).write(
                            (((((__p7).read()) as i32).wrapping_add(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                                    .read()) as i32),
                            )) as i16),
                        );
                        let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                        (__p8).write(
                            (((((__p8).read()) as i32).wrapping_add(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                    .read()) as i32),
                            )) as i16),
                        );
                        red = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                            .read()) as i32)
                            >> 3) as u16);
                        green = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                            .read()) as i32)
                            >> 3) as u16);
                        blue = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                            .read()) as i32)
                            >> 3) as u16);
                        color = (((((red) as i32) | (((green) as i32) << 5))
                            | (((blue) as i32) << 10)) as u16);
                    } else {
                        color = 32699u16;
                        let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                    bitmask = 1u16;
                    r3 = 0u16;
                    {
                        i = 0u16;
                        'l2: loop {
                            if !(((i) as i32) <= 15i32) {
                                break 'l2;
                            }
                            'l3: {
                                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                    .read()) as i32)
                                    & ((bitmask) as i32))
                                    != 0
                                {
                                    {
                                        j = 1u16;
                                        'l4: loop {
                                            if !(((j) as i32) <= 15i32) {
                                                break 'l4;
                                            }
                                            'l5: {
                                                ((((&raw mut gPlttBufferFaded).cast::<u16>())
                                                    .cast::<u16>())
                                                .wrapping_offset(
                                                    (((r3) as i32).wrapping_add(((j) as i32)))
                                                        as isize,
                                                ))
                                                .write(color);
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                }
                                bitmask = ((((bitmask) as i32) << 1) as u16);
                                r3 = ((((r3) as i32).wrapping_add(16i32)) as u16);
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    let mut spriteId: u8 = 0u8;
                    {
                        spriteId = 0u8;
                        'l6: loop {
                            if !(((spriteId) as i32) < 64i32) {
                                break 'l6;
                            }
                            'l7: {
                                if (((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(20)
                                .cast::<*mut u8>())
                                .read()) as usize)
                                    == (((&raw const gMoonSpriteTemplate).cast::<u8>().cast_mut())
                                        as usize))
                                    || (((((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(20)
                                    .cast::<*mut u8>())
                                    .read()) as usize)
                                        == (((&raw const gMoonlightSparkleSpriteTemplate)
                                            .cast::<u8>()
                                            .cast_mut())
                                            as usize))
                                {
                                    (((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .write(1i16);
                                }
                            }
                            spriteId = (spriteId).wrapping_add(1);
                        }
                    }
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p10 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    > 30i32
                {
                    BeginNormalPaletteFade(
                        ((LoadPointerFromVars(
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                        )) as usize as u32),
                        0i8,
                        16u8,
                        0u8,
                        32699u16,
                    );
                    let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((crate::c::bf_read(
                    ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
                    7,
                    1,
                    false,
                ) as u16)
                    != 0)
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHornHit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) < 2i32 {
            ((cmd).wrapping_add(4).cast::<i16>()).write(2i16);
        }
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) > 127i32 {
            ((cmd).wrapping_add(4).cast::<i16>()).write(127i16);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((((cmd).cast::<i16>()).read()) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        if (IsContest()) != 0 {
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(40i32)) as i16));
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(20i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 7) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((crate::c::div_i32(
                    (-5120i32),
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 7) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((crate::c::div_i32(
                    (-2560i32),
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32
            {
                let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(40i32)) as i16));
                let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_add(20i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 7) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    ((crate::c::div_i32(
                        5120i32,
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 7) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                    ((crate::c::div_i32(
                        (-2560i32),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
            } else {
                let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add(40i32)) as i16));
                let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_sub(20i32)) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 7) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                    ((crate::c::div_i32(
                        (-5120i32),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 7) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                    ((crate::c::div_i32(
                        2560i32,
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (24u32) as i32);
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimHornHit_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimHornHit_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 7) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 7) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t4 = ((__p3).read()).wrapping_sub(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 1i32
        {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DoubleTeam(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut obj: i32 = 0i32;
        let mut r3: u16 = 0u16;
        let mut r4: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
            .write(((AllocSpritePalette(10097u16)) as i16));
        r3 = (((256i32).wrapping_add(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_mul(16i32),
        )) as u16);
        r4 = (((((crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(5),
            4,
            4,
            false,
        ) as u16) as i32)
            .wrapping_add(16i32))
        .wrapping_mul(16i32)) as u16);
        {
            i = 1u16;
            'l1: loop {
                if !(((i) as i32) < 16i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                        .wrapping_offset((((r3) as i32).wrapping_add(((i) as i32))) as isize))
                    .write(
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset((((r4) as i32).wrapping_add(((i) as i32))) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        BlendPalette(r3, 16u16, 11u8, 0u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        i = 0u16;
        'l3: loop {
            if !((((i) as i32) < 2i32)
                && ({
                    let __v1 = ((CloneBattlerSpriteWithBlend(0u8)) as i32);
                    obj = __v1;
                    __v1
                } >= 0i32))
            {
                break 'l3;
            }
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset((obj) as isize * 68))
                    .wrapping_add(5),
                4,
                4,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16)
                    as i32,
            );
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset((obj) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .write(0i16);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((obj) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((((i) as i32) << 7) as i16));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset((obj) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((taskId) as i16));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset((obj) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimDoubleTeam));
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(((__p2).read()).wrapping_add(1));
            i = (i).wrapping_add(1);
        }
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_DoubleTeam_Step));
        if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
            as i32)
            == 1i32
        {
            ClearGpuRegBits(0u8, 512u16);
        } else {
            ClearGpuRegBits(0u8, 1024u16);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DoubleTeam_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) != 0) {
            if ((GetBattlerSpriteBGPriorityRank(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            )) as i32)
                == 1i32
            {
                SetGpuRegBits(0u8, 512u16);
            } else {
                SetGpuRegBits(0u8, 1024u16);
            }
            FreeSpritePaletteByTag(10097u16);
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDoubleTeam(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 64i32 {
            let __p4 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p4).write(((__p4).read()).wrapping_sub(1));
            DestroySpriteWithActiveSheet(sprite);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32),
                    6i32,
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize,
                    ))
                    .read()) as i32),
                    13i32,
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )
                    & 255i32) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            ));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSuperFang(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MusicNotesRainbowBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut index: u16 = 0u16;
        index = ((IndexOfSpritePaletteTag(
            ((((&raw const gParticlesColorBlendTable)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .cast::<u16>())
            .read(),
        )) as u16);
        if ((index) as i32) != 255i32 {
            index = (((256i32).wrapping_add(((index) as i32).wrapping_mul(16i32))) as u16);
            {
                i = 1u16;
                'l1: loop {
                    if !(((i) as u32) < crate::c::div_u32(12u32, 2u32)) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((index) as i32).wrapping_add(((i) as i32))) as isize,
                            ))
                        .write(
                            (((((&raw const gParticlesColorBlendTable)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        {
            j = 1u16;
            'l3: loop {
                if !(((j) as u32) < crate::c::div_u32(48u32, 12u32)) {
                    break 'l3;
                }
                'l4: {
                    index = ((AllocSpritePalette(
                        (((((&raw const gParticlesColorBlendTable)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((j) as i32) as isize * 12))
                        .cast::<u16>())
                        .read(),
                    )) as u16);
                    if ((index) as i32) != 255i32 {
                        index =
                            (((256i32).wrapping_add(((index) as i32).wrapping_mul(16i32))) as u16);
                        {
                            i = 1u16;
                            'l5: loop {
                                if !(((i) as u32) < crate::c::div_u32(12u32, 2u32)) {
                                    break 'l5;
                                }
                                'l6: {
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            (((index) as i32).wrapping_add(((i) as i32))) as isize,
                                        ))
                                    .write(
                                        ((((((&raw const gParticlesColorBlendTable)
                                            .cast::<u8>()
                                            .cast_mut())
                                        .cast::<u8>())
                                        .wrapping_offset(((j) as i32) as isize * 12))
                                        .cast::<u16>())
                                        .wrapping_offset(((i) as i32) as isize))
                                        .read(),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                }
                j = (j).wrapping_add(1);
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MusicNotesClearRainbowBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        {
            i = 1u16;
            'l1: loop {
                if !(((i) as u32) < crate::c::div_u32(48u32, 12u32)) {
                    break 'l1;
                }
                'l2: {
                    FreeSpritePaletteByTag(
                        (((((&raw const gParticlesColorBlendTable)
                            .cast::<u8>()
                            .cast_mut())
                        .cast::<u8>())
                        .wrapping_offset(((i) as i32) as isize * 12))
                        .cast::<u16>())
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimWavyMusicNotes(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut index: u8 = 0u8;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        StartSpriteAnim(sprite, ((((cmd).cast::<i16>()).read()) as u8));
        if (({
            let __v1 = IndexOfSpritePaletteTag(
                (((((&raw const gParticlesColorBlendTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>())
                .wrapping_offset(
                    ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) as isize * 12,
                ))
                .cast::<u16>())
                .read(),
            );
            index = __v1;
            __v1
        }) as i32)
            != 255i32
        {
            crate::c::bf_write((sprite).wrapping_add(5), 4, 4, ((index) as u16) as i32);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        if (IsContest()) != 0 {
            x = 48u8;
            y = 40u8;
        } else {
            x = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8);
            y = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16));
        AnimWavyMusicNotes_CalcVelocity(
            ((((x) as i32)
                .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                as i16),
            ((((y) as i32)
                .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                as i16),
            (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6),
            (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7),
            40i8,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWavyMusicNotes_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimWavyMusicNotes_CalcVelocity(
    x: i16,
    y: i16,
    velocX: *mut i16,
    velocY: *mut i16,
    xSpeedFactor: i8,
) {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut velocX = velocX;
        let mut velocY = velocY;
        let mut xSpeedFactor = xSpeedFactor;
        let mut x2: i32 = 0i32;
        let mut time: i32 = 0i32;
        if ((x) as i32) < 0i32 {
            xSpeedFactor = ((((xSpeedFactor) as i32).wrapping_neg()) as i8);
        }
        x2 = ((x) as i32).wrapping_mul(256i32);
        time = crate::c::div_i32(x2, ((xSpeedFactor) as i32));
        if time == 0i32 {
            time = 1i32;
        }
        (velocX).write(((crate::c::div_i32(x2, time)) as i16));
        (velocY).write(((crate::c::div_i32(((y) as i32).wrapping_mul(256i32), time)) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimWavyMusicNotes_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y: i16 = 0i16;
        let mut trigIdx: i16 = 0i16;
        let mut index: u8 = 0u8;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        trigIdx = ((crate::c::rem_i32(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(5i32),
            256i32,
        )) as i16);
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 4) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >> 4) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(trigIdx, 15i16));
        y = ((sprite).wrapping_add(34).cast::<i16>()).read();
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-16i32))
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 256i32))
            || (((y) as i32) < (-16i32)))
            || (((y) as i32) > 128i32)
        {
            DestroySpriteAndMatrix(sprite);
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0)
                && ((({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32))
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                if (({
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > ((crate::c::div_u32(48u32, 12u32)) as i32).wrapping_sub(1i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                }
                index = IndexOfSpritePaletteTag(
                    (((((&raw const gParticlesColorBlendTable)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 12,
                    ))
                    .cast::<u16>())
                    .read(),
                );
                if ((index) as i32) != 255i32 {
                    crate::c::bf_write((sprite).wrapping_add(5), 4, 4, ((index) as u16) as i32);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlyingMusicNotes(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            let __p1 = (cmd).wrapping_add(2).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                as i16),
        );
        StartSpriteAnim(sprite, ((((cmd).cast::<i16>()).read()) as u8));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((crate::c::div_i32(
                (((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) << 4),
                5i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::div_i32(
                (((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) << 7),
                5i32,
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFlyingMusicNotes_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFlyingMusicNotes_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 4) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >> 4) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 5i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                == 0i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_add(16i32)
                    & 255i32) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                18i16,
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                18i16,
            ));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(1i16);
            }
        }
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 48i32
        {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBellyDrumHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut a: i16 = 0i16;
        if ((((cmd).cast::<i16>()).read()) as i32) == 1i32 {
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
            a = 16i16;
        } else {
            a = (-16i16);
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((a) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(8i32)) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimSlowFlyingMusicNotes(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut xDiff: i16 = 0i16;
        let mut index: u8 = 0u8;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        StartSpriteAnim(
            sprite,
            ((((cmd).wrapping_add(2).cast::<i16>()).read()) as u8),
        );
        index = IndexOfSpritePaletteTag(
            (((((&raw const gParticlesColorBlendTable)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>())
            .wrapping_offset(
                ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) as isize * 12,
            ))
            .cast::<u16>())
            .read(),
        );
        if ((index) as i32) != 255i32 {
            crate::c::bf_write((sprite).wrapping_add(5), 4, 4, ((index) as u16) as i32);
        }
        xDiff = ((if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            (-32i32)
        } else {
            32i32
        }) as i16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(40i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((xDiff) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                .wrapping_sub(40i32)) as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSlowFlyingMusicNotes_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSlowFlyingMusicNotes_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((AnimTranslateLinear(sprite)) as i32) == 0i32 {
            let mut xDiff: i16 = 0i16;
            xDiff = Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                8i16,
            );
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) < 0i32 {
                xDiff = ((((xDiff) as i32).wrapping_neg()) as i16);
            }
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(((xDiff) as i32))) as i16));
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        4i16,
                    )) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(8i32)
                    & 255i32) as i16),
            );
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn SetSpriteNextToMonHead(battler: u8, sprite: *mut u8) {
    unsafe {
        let mut battler = battler;
        let mut sprite = sprite;
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoordAttr(battler, 5u8)) as i32).wrapping_add(8i32)) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoordAttr(battler, 4u8)) as i32).wrapping_sub(8i32)) as i16),
            );
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 3u8)) as i32).wrapping_sub(crate::c::div_i32(
                ((GetBattlerSpriteCoordAttr(battler, 0u8)) as i32),
                4i32,
            ))) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimThoughtBubble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut animNum: u8 = 0u8;
        let mut battler: u8 = 0u8;
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        SetSpriteNextToMonHead(battler, sprite);
        animNum = ((if ((GetBattlerSide(battler)) as i32) == 0i32 {
            0i32
        } else {
            1i32
        }) as u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((animNum) as i32).wrapping_add(2i32)) as i16));
        StartSpriteAnim(sprite, animNum);
        StoreSpriteCallbackInData6(sprite, Some(AnimThoughtBubble_Step));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimThoughtBubble_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
            StartSpriteAnim(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(RunStoredCallbackWhenAnimEnds));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMetronomeFinger(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: u8 = 0u8;
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        SetSpriteNextToMonHead(battler, sprite);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        StoreSpriteCallbackInData6(sprite, Some(AnimMetronomeFinger_Step));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimMetronomeFinger_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 16i32
        {
            StartSpriteAffineAnim(sprite, 1u8);
            StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(RunStoredCallbackWhenAffineAnimEnds));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFollowMeFinger(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: u8 = 0u8;
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        ((sprite).wrapping_add(32).cast::<i16>())
            .write(((GetBattlerSpriteCoord(battler, 0u8)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>()).write(GetBattlerSpriteCoordAttr(battler, 2u8));
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) <= 9i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(10i16);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((((sprite).wrapping_add(67)).read()) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((((((sprite).wrapping_add(67)).read()) as i32).wrapping_add(4i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        StoreSpriteCallbackInData6(sprite, Some(AnimFollowMeFinger_Step1));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimFollowMeFinger_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 12i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimFollowMeFinger_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFollowMeFinger_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x1: i16 = 0i16;
        let mut x2: i16 = 0i16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 254i32
        {
            if (({
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t3 = ((__p2).read()).wrapping_sub(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == 0i32
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimMetronomeFinger_Step));
                return;
            } else {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32) & 255i32) as i16));
            }
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 79i32
        {
            ((sprite).wrapping_add(67)).write(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u8),
            );
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 159i32
        {
            ((sprite).wrapping_add(67)).write(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8),
            );
        }
        x1 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                as isize,
        ))
        .read();
        x2 = ((((x1) as i32) >> 3) as i16);
        ((sprite).wrapping_add(36).cast::<i16>())
            .write((((((x1) as i32) >> 3).wrapping_add((((x2) as i32) >> 1))) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimTauntFinger(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: u8 = 0u8;
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        SetSpriteNextToMonHead(battler, sprite);
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            StartSpriteAnim(sprite, 0u8);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(2i16);
        } else {
            StartSpriteAnim(sprite, 1u8);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTauntFinger_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTauntFinger_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 10i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            StartSpriteAnim(
                sprite,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8),
            );
            StoreSpriteCallbackInData6(sprite, Some(AnimTauntFinger_Step2));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(RunStoredCallbackWhenAnimEnds));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTauntFinger_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 5i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
