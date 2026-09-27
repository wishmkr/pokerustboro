//! Translated from `src/battle_anim_effects_3.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gScratchAnimCmds gScratchAnimTable gScratchSpriteTemplate gBlackSmokeSpriteTemplate gBlackBallSpriteTemplate gOpeningEyeAnimCmds gOpeningEyeAnimTable gOpeningEyeSpriteTemplate gWhiteHaloSpriteTemplate gTealAlertSpriteTemplate gMeanLookEyeAffineAnimCmds1 gMeanLookEyeAffineAnimCmds2 gMeanLookEyeAffineAnimTable gMeanLookEyeSpriteTemplate gSpikesSpriteTemplate gLeerAnimCmds gLeerAnimTable gLeerSpriteTemplate gLetterZAnimCmds gLetterZAnimTable gLetterZAffineAnimCmds gLetterZAffineAnimTable gLetterZSpriteTemplate gFangAnimCmds gFangAnimTable gFangAffineAnimCmds gFangAffineAnimTable gFangSpriteTemplate gSpotlightAffineAnimCmds1 gSpotlightAffineAnimCmds2 gSpotlightAffineAnimTable gSpotlightSpriteTemplate gClappingHandSpriteTemplate gClappingHand2SpriteTemplate gRapidSpinAnimCmds gRapidSpinAnimTable gRapidSpinSpriteTemplate sAffineAnims_Torment gTriAttackTriangleAnimCmds gTriAttackTriangleAnimTable gTriAttackTriangleAffineAnimCmds gTriAttackTriangleAffineAnimTable gTriAttackTriangleSpriteTemplate gEclipsingOrbAnimCmds gEclipsingOrbAnimTable gEclipsingOrbSpriteTemplate DefenseCurlDeformMonAffineAnimCmds gBatonPassPokeballSpriteTemplate gWishStarSpriteTemplate gMiniTwinklingStarSpriteTemplate gStockpileDeformMonAffineAnimCmds gSpitUpDeformMonAffineAnimCmds gSwallowBlueOrbSpriteTemplate gSwallowDeformMonAffineAnimCmds gMorningSunLightBeamCoordsTable gGreenStarAnimCmds1 gGreenStarAnimCmds2 gGreenStarAnimCmds3 gGreenStarAnimTable gGreenStarSpriteTemplate gDoomDesireLightBeamCoordTable gDoomDesireLightBeamDelayTable gStrongFrustrationAffineAnimCmds gWeakFrustrationAngerMarkSpriteTemplate gSweetScentPetalAnimCmds1 gSweetScentPetalAnimCmds2 gSweetScentPetalAnimCmds3 gSweetScentPetalAnimCmdTable gSweetScentPetalSpriteTemplate sUnusedPalette gPainSplitAnimCmds gPainSplitAnimCmdTable gPainSplitProjectileSpriteTemplate gFlatterConfettiSpriteTemplate gFlatterSpotlightSpriteTemplate gReversalOrbSpriteTemplate gDeepInhaleAffineAnimCmds gYawnCloudAffineAnimCmds1 gYawnCloudAffineAnimCmds2 gYawnCloudAffineAnimCmds3 gYawnCloudAffineAnimTable gYawnCloudSpriteTemplate gSmokeBallEscapeCloudAffineAnimCmds1 gSmokeBallEscapeCloudAffineAnimCmds2 gSmokeBallEscapeCloudAffineAnimCmds3 gSmokeBallEscapeCloudAffineAnimCmds4 gSmokeBallEscapeCloudAffineAnimTable gSmokeBallEscapeCloudSpriteTemplate gFacadeSquishAffineAnimCmds gFacadeSweatDropSpriteTemplate gFacadeBlendColors gRoarNoiseLineAnimCmds1 gRoarNoiseLineAnimCmds2 gRoarNoiseLineAnimTable gRoarNoiseLineSpriteTemplate gGlareEyeDotSpriteTemplate gAssistPawprintSpriteTemplate gBarrageBallAffineAnimCmds1 gBarrageBallAffineAnimCmds2 gBarrageBallAffineAnimTable gBarrageBallSpriteTemplate gSmellingSaltsHandSpriteTemplate gSmellingSaltsSquishAffineAnimCmds gSmellingSaltExclamationSpriteTemplate gHelpingHandClapSpriteTemplate gForesightMagnifyingGlassSpriteTemplate gMeteorMashStarSpriteTemplate sUnusedStarBurstSpriteTemplate gBlockXSpriteTemplate sUnusedItemBagStealSpriteTemplate gKnockOffStrikeAnimCmds gKnockOffStrikeAnimTable gKnockOffStrikeAffineanimCmds1 gKnockOffStrikeAffineanimCmds2 gKnockOffStrikeAffineAnimTable gKnockOffStrikeSpriteTemplate gRecycleSpriteAffineAnimCmds gRecycleSpriteAffineAnimTable gRecycleSpriteTemplate gSlackOffSquishAffineAnimCmds
#[allow(unused_imports)]
use crate::data::battle_anim_effects_3::*;

unsafe extern "C" {
    static mut gAffineAnims_BattleSpriteContest: u8;
    static mut gAffineAnims_BattleSpriteOpponentSide: u8;
    static mut gAnimFriendship: u8;
    static mut gAnimMoveDmg: u8;
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimMaskImage_LightBeam: u8;
    static mut gBattleAnimMaskPalette_LightBeam: u8;
    static mut gBattleAnimMaskTilemap_LightBeam: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleMonForms: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattle_WIN1H: u8;
    static mut gBattle_WIN1V: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gContestResources: u8;
    static mut gCureBubblesGfx: u8;
    static mut gCureBubblesPal: u8;
    static mut gCureBubblesTilemap: u8;
    static mut gEnemyParty: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gThoughtBubbleSpriteTemplate: u8;
    static mut gWeatherMoveAnim: u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemapHandleContest(a0: *mut u8, a1: *mut u8, a2: u32);
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn ChangeSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn ClearBattleAnimBg(a0: u32);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateAdditionalMonSpriteForMoveAnim(
        a0: u16,
        a1: u8,
        a2: u8,
        a3: i16,
        a4: i16,
        a5: u8,
        a6: u32,
        a7: u32,
        a8: u32,
        a9: u32,
    ) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateSpriteAndAnimate(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimSpriteAfterTimer(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroyAnimVisualTaskAndDisableBlend(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources_(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroySpriteWithActiveSheet(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattleBgPaletteNum() -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn GetBgDataForTransform(a0: *mut u8, a1: u8);
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn HandleSpeciesGfxDataChange(a0: u8, a1: u8, a2: u8);
    fn InitAndRunAnimFastLinearTranslation(a0: *mut u8);
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn IsSpeciesNotUnown(a0: u16) -> u8;
    fn LoadBattleMonGfxAndAnimate(a0: u8, a1: u8, a2: u8);
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PlaySE1WithPanning(a0: u16, a1: i8);
    fn PrepareAffineAnimInTaskData(a0: *mut u8, a1: u8, a2: *mut u8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn Random2() -> u16;
    fn ResetSpriteRotScale(a0: u8);
    fn ResetSpriteRotScale_PreserveAffine(a0: *mut u8);
    fn RunAffineAnimFromTaskData(a0: *mut u8) -> u8;
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAnimSpriteInitialXOffset(a0: *mut u8, a1: i16);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetBattlerShadowSpriteCallback(a0: u8, a1: u16);
    fn SetBattlerSpriteYOffsetFromRotation(a0: u8);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut u8);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SmokescreenImpact(a0: i16, a1: i16, a2: u8) -> u8;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartMonScrollingBgMask(
        a0: u8,
        a1: i32,
        a2: u16,
        a3: u8,
        a4: u8,
        a5: u8,
        a6: u8,
        a7: u8,
        a8: *mut u32,
        a9: *mut u32,
        a10: *mut u32,
    );
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn TrySetSpriteRotScale(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: u16);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimBlackSmoke(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
            .read())
            != 0)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBlackSmoke_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBlackSmoke_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32) > 0i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 8) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(((__p2).read()).wrapping_sub(1));
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SmokescreenImpact(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SmokescreenImpact(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(8i32)) as i16),
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(8i32)) as i16),
            0u8,
        );
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimWhiteHalo(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(90i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(7i16);
        StoreSpriteCallbackInData6(sprite, Some(AnimWhiteHalo_Step1));
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(
            82u8,
            ((((16i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) << 8)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimWhiteHalo_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(
            82u8,
            ((((16i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) << 8)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)) as u16),
        );
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < 0i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimWhiteHalo_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWhiteHalo_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimTealAlert(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rotation: u16 = 0u16;
        let mut x: u8 =
            GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8);
        let mut y: u8 =
            GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8);
        InitSpritePosToAnimTarget(sprite, 1u8);
        rotation = ArcTan2Neg(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_sub(((x) as i32))) as i16),
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_sub(((y) as i32))) as i16),
        );
        rotation = ((((rotation) as i32).wrapping_add(24576i32)) as u16);
        if (IsContest()) != 0 {
            rotation = ((((rotation) as i32).wrapping_add(16384i32)) as u16);
        }
        TrySetSpriteRotScale(sprite, 0u8, 256i16, 256i16, rotation);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((x) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(((y) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimMeanLookEye(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 4096u16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(4i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMeanLookEye_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimMeanLookEye_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(
            82u8,
            ((((16i32).wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                << 8)
                | (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                as u16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) != 0 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 15i32)
            || ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 4i32)
        {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write((((((__p3).read()) as i32) ^ 1i32) as i16));
        }
        if (({
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t5 = (__p4).read();
            (__p4).write(((__p4).read()).wrapping_add(1));
            __t5
        }) as i32)
            > 70i32
        {
            SetGpuReg(80u8, 0u16);
            SetGpuReg(82u8, 0u16);
            StartSpriteAffineAnim(sprite, 1u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimMeanLookEye_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMeanLookEye_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 9i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
            if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimMeanLookEye_Step3));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMeanLookEye_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            if __sw1 == 0i32 || __sw1 == 1i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(1i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write((-1i16));
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                break 'l1;
            }
            if __sw1 == 4i32 || __sw1 == 5i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 6i32 || !__matched {
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(38).cast::<i16>()).write((-1i16));
                break 'l1;
            }
        }
        if (({
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            > 7i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        if (({
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            let __t5 = (__p4).read();
            (__p4).write(((__p4).read()).wrapping_add(1));
            __t5
        }) as i32)
            > 15i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            SetGpuReg(80u8, 16192u16);
            SetGpuReg(
                82u8,
                ((0i32 | (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)) as u16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimMeanLookEye_Step4));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMeanLookEye_Step4(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(
            82u8,
            ((((16i32).wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                << 8)
                | (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                as u16),
        );
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 1i32
        {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
            SetGpuReg(80u8, 0u16);
            SetGpuReg(82u8, 0u16);
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetPsychicBackground(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(SetPsychicBackground_Step));
        let __p1 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
    }
}
pub(crate) unsafe extern "C" fn SetPsychicBackground_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut lastColor: u16 = 0u16;
        let mut paletteIndex: u8 = GetBattleBgPaletteNum();
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 4i32
        {
            lastColor = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    (((0i32).wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                        .wrapping_add(11i32)) as isize,
                ))
            .read();
            {
                i = 10i32;
                'l1: loop {
                    if !(i > 0i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((0i32)
                                    .wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                                .wrapping_add(i))
                                .wrapping_add(1i32)) as isize,
                            ))
                        .write(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((0i32)
                                        .wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                                    .wrapping_add(i)) as isize,
                                ))
                            .read(),
                        );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((0i32).wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                    .wrapping_add(1i32)) as isize,
            ))
            .write(lastColor);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            DestroyTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FadeScreenToWhite(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(FadeScreenToWhite_Step));
        let __p1 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
    }
}
pub(crate) unsafe extern "C" fn FadeScreenToWhite_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut lastColor: u16 = 0u16;
        let mut paletteIndex: u8 = GetBattleBgPaletteNum();
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 4i32
        {
            lastColor = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    (((0i32).wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                        .wrapping_add(11i32)) as isize,
                ))
            .read();
            {
                i = 10i32;
                'l1: loop {
                    if !(i > 0i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((0i32)
                                    .wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                                .wrapping_add(i))
                                .wrapping_add(1i32)) as isize,
                            ))
                        .write(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((0i32)
                                        .wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                                    .wrapping_add(i)) as isize,
                                ))
                            .read(),
                        );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((0i32).wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                    .wrapping_add(1i32)) as isize,
            ))
            .write(lastColor);
            lastColor = ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    (((0i32).wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                        .wrapping_add(11i32)) as isize,
                ))
            .read();
            {
                i = 10i32;
                'l3: loop {
                    if !(i > 0i32) {
                        break 'l3;
                    }
                    'l4: {
                        ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((0i32)
                                    .wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                                .wrapping_add(i))
                                .wrapping_add(1i32)) as isize,
                            ))
                        .write(
                            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((0i32)
                                        .wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                                    .wrapping_add(i)) as isize,
                                ))
                            .read(),
                        );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
            ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((0i32).wrapping_add(((paletteIndex) as i32).wrapping_mul(16i32)))
                    .wrapping_add(1i32)) as isize,
            ))
            .write(lastColor);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            DestroyTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpikes(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        SetAverageBattlerPositions(
            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
            0u8,
            &raw mut x,
            &raw mut y,
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((x) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((y) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-50i16));
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSpikes_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimSpikes_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(WaitAnimForDuration));
            StoreSpriteCallbackInData6(sprite, Some(AnimSpikes_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpikes_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            & 1i32)
            != 0
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        }
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 16i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLeer(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        SetAnimSpriteInitialXOffset(
            sprite,
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
        );
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimLetterZ(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var0: i32 = 0i32;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            SetSpriteCoordsToAnimAttackerCoords(sprite);
            SetAnimSpriteInitialXOffset(
                sprite,
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
            );
            if !((IsContest()) != 0) {
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(2))
                        .read(),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read(),
                    );
                } else {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                        (((-1i32).wrapping_mul(
                            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(2))
                            .read()) as i32),
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        (((-1i32).wrapping_mul(
                            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(3))
                            .read()) as i32),
                        )) as i16),
                    );
                }
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    (((-1i32).wrapping_mul(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                        .read(),
                );
            }
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        var0 = ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(20i32)
            & 255i32);
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                2i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((Sin(((var0 & 255i32) as i16), 5i16)) as i32).wrapping_add(crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                2i32,
            ))) as i16),
        );
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
            as u16) as i32)
            > 240i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFang(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsTargetPlayerSide(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 1i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        } else {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsHealingMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gAnimMoveDmg).cast::<i32>()).read() > 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        } else {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimSpotlight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(74u8, 7999u16);
        SetGpuRegBits(0u8, 32768u16);
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(64u8, ((&raw mut gBattle_WIN0H).cast::<u16>()).read());
        SetGpuReg(68u8, ((&raw mut gBattle_WIN0V).cast::<u16>()).read());
        InitSpritePosToAnimTarget(sprite, 0u8);
        crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (2u32) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSpotlight_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimSpotlight_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 || __sw1 == 3i32 {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(117i32)) as i16));
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 8) as i16),
                );
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 21i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p7).write((((((__p7).read()) as i32).wrapping_sub(117i32)) as i16));
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 8) as i16),
                );
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 41i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                ChangeSpriteAffineAnim(sprite, 1u8);
                let __p11 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p11).write(((__p11).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimSpotlight_Step2));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpotlight_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(74u8, 16191u16);
        SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) ^ 32768i32) as u16));
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimClappingHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            == 0i32
        {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(16i32)) as u16) as i32,
        );
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
            ((sprite).wrapping_add(36).cast::<i16>()).write((-12i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2i16);
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(12i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-2i16));
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            != 255i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimClappingHand_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimClappingHand_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    == 0i32
                {
                    PlaySE1WithPanning(222u16, BattleAnimAdjustPanning((-64i8)));
                }
            }
        } else {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            if (if ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) < 0i32 {
                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg()
            } else {
                ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)
            }) == 12i32
            {
                let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_sub(1));
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p5).write(((__p5).read()).wrapping_sub(1));
            }
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimClappingHand2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (2u32) as i32);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(255i16);
        AnimClappingHand(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CreateSpotlight(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (IsContest()) != 0 {
            SetGpuReg(72u8, 7999u16);
            ((&raw mut gBattle_WIN1H).cast::<u16>()).write(39152u16);
            ((&raw mut gBattle_WIN1V).cast::<u16>()).write(160u16);
            SetGpuReg(66u8, ((&raw mut gBattle_WIN0H).cast::<u16>()).read());
            SetGpuReg(70u8, ((&raw mut gBattle_WIN0V).cast::<u16>()).read());
        } else {
            SetGpuReg(72u8, 7999u16);
            ((&raw mut gBattle_WIN1H).cast::<u16>()).write(240u16);
            ((&raw mut gBattle_WIN1V).cast::<u16>()).write(30880u16);
            SetGpuReg(66u8, ((&raw mut gBattle_WIN1H).cast::<u16>()).read());
            SetGpuReg(70u8, ((&raw mut gBattle_WIN1V).cast::<u16>()).read());
            SetGpuRegBits(0u8, 16384u16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_RemoveSpotlight(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(72u8, 16191u16);
        ((&raw mut gBattle_WIN1H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN1V).cast::<u16>()).write(0u16);
        if !((IsContest()) != 0) {
            ClearGpuRegBits(0u8, 16384u16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimRapidSpin(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)
                > ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRapidSpin_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRapidSpin_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )
                & 255i32) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
                >> 4) as i16),
        );
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0 {
            if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)
                < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            {
                DestroyAnimSprite(sprite);
            }
        } else {
            if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)
                > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_RapinSpinMonElevation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: i16 = 0i16;
        let mut toBG2: u8 = 0u8;
        let mut var2: i16 = 0i16;
        let mut var3: i32 = 0i32;
        let mut var4: i32 = 0i32;
        let mut i: i16 = 0i16;
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !(((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0) {
            var0 = ((GetBattlerYCoordWithElevation(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            )) as i16);
            toBG2 = GetBattlerSpriteBGPriorityRank(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            );
        } else {
            var0 = ((GetBattlerYCoordWithElevation(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
            )) as i16);
            toBG2 =
                GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
        }
        (((task).wrapping_add(8)).cast::<i16>())
            .write(((((var0) as i32).wrapping_add(36i32)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
            .write((((task).wrapping_add(8)).cast::<i16>()).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
            .write(((((var0) as i32).wrapping_sub(33i32)) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) < 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
            .write((((task).wrapping_add(8)).cast::<i16>()).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(8i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        if ((toBG2) as i32) == 1i32 {
            var3 = ((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i32);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(((var3) as i16));
            var4 = (var3).wrapping_add(240i32);
        } else {
            var3 = ((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i32);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(((var3) as i16));
            var4 = (var3).wrapping_add(240i32);
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(((var4) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
            .read())
            != 0)
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(((var4) as i16));
            var2 = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read();
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(((var3) as i16));
            var2 = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read();
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(0i16);
        i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read();
        'l1: loop {
            if !(((i) as i32)
                <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32))
            {
                break 'l1;
            }
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                .wrapping_offset(((i) as i32) as isize))
            .write(((var2) as u16));
            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
            .write(((var2) as u16));
            i = (i).wrapping_add(1);
        }
        if ((toBG2) as i32) == 1i32 {
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108884i32) as usize as *mut u16).cast::<u8>());
        } else {
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108888i32) as usize as *mut u16).cast::<u8>());
        }
        (((&raw mut scanlineParams).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(2724200449u32);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(9)).write(0u8);
        ScanlineEffect_SetParams(
            (&raw mut scanlineParams)
                .cast::<u8>()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(RapinSpinMonElevation_Step));
    }
}
pub(crate) unsafe extern "C" fn RapinSpinMonElevation_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i16 = 0i16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )) as i16),
        );
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)
            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            (((task).wrapping_add(8)).cast::<i16>())
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) == 0i32 {
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(1i16);
            }
        } else {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
        if (({
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            > 1i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
                ((if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    == 0i32
                {
                    1i32
                } else {
                    0i32
                }) as i16),
            );
            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) != 0 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read());
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read());
            }
        }
        i = (((task).wrapping_add(8)).cast::<i16>()).read();
        'l1: loop {
            if !(((i) as i32)
                < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32))
            {
                break 'l1;
            }
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                .wrapping_offset(((i) as i32) as isize))
            .write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as u16),
            );
            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
            .write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as u16),
            );
            i = (i).wrapping_add(1);
        }
        i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read();
        'l2: loop {
            if !(((i) as i32)
                <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32))
            {
                break 'l2;
            }
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                .wrapping_offset(((i) as i32) as isize))
            .write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u16),
            );
            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .wrapping_offset(((i) as i32) as isize))
            .write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u16),
            );
            i = (i).wrapping_add(1);
        }
        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) != 0 {
            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) != 0 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
            }
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TormentAttacker(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(32i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write((-20i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(TormentAttacker_Step));
    }
}
pub(crate) unsafe extern "C" fn TormentAttacker_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: i32 = 0i32;
        let mut var1: i32 = 0i32;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                var0 =
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    & 1i32)
                    != 0
                {
                    var1 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32);
                    x = (((var0).wrapping_sub(var1)) as i16);
                } else {
                    var1 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32);
                    x = (((var0).wrapping_add(var1)) as i16);
                }
                y = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    .wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16);
                spriteId = CreateSprite(
                    (&raw mut gThoughtBubbleSpriteTemplate).cast::<u8>(),
                    x,
                    y,
                    (((6i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as u8),
                );
                PlaySE12WithPanning(186u16, BattleAnimAdjustPanning((-64i8)));
                if ((spriteId) as i32) != 64i32 {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(63),
                        0,
                        1,
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            & 1i32) as u16) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCallbackDummy));
                }
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    & 1i32)
                    != 0
                {
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p2).write((((((__p2).read()) as i32).wrapping_sub(6i32)) as i16));
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    (__p3).write((((((__p3).read()) as i32).wrapping_sub(6i32)) as i16));
                }
                PrepareAffineAnimInTaskData(
                    task,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((&raw const sAffineAnims_Torment).cast::<u8>().cast_mut()).cast::<u8>(),
                );
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p4).write(((__p4).read()).wrapping_add(1));
                (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((RunAffineAnimFromTaskData(task)) != 0) {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        == 6i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(8i16);
                        (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                    } else {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            <= 2i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                .write(10i16);
                        } else {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                .write(0i16);
                        }
                        (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    != 0i32
                {
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    (__p5).write(((__p5).read()).wrapping_sub(1));
                } else {
                    (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    != 0i32
                {
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                } else {
                    (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                {
                    i = 0u16;
                    j = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 64i32) {
                            break 'l2;
                        }
                        'l3: {
                            if ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 68))
                            .wrapping_add(20)
                            .cast::<*mut u8>())
                            .read()) as usize)
                                == (((&raw mut gThoughtBubbleSpriteTemplate).cast::<u8>()) as usize)
                            {
                                (((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .write(((taskId) as i16));
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(6i16);
                                StartSpriteAnim(
                                    ((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize * 68),
                                    2u8,
                                );
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((i) as i32) as isize * 68))
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(TormentAttacker_Callback));
                                if (({
                                    let __t7 = (j).wrapping_add(1);
                                    j = __t7;
                                    __t7
                                }) as i32)
                                    == 6i32
                                {
                                    break 'l2;
                                }
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(((j) as i16));
                (((task).wrapping_add(8)).cast::<i16>()).write(5i16);
                break 'l1;
            }
            if __sw1 == 5i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn TormentAttacker_Callback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize,
            );
            (__p1).write(((__p1).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTriAttackTriangle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 0u8);
        }
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < 40i32
        {
            let mut var: u16 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16);
            if (((var) as i32) & 1i32) == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 30i32 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 61i32 {
            StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
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
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DefenseCurlDeformMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                PrepareAffineAnimInTaskData(
                    ((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40),
                    GetAnimBattlerSpriteId(0u8),
                    ((&raw const DefenseCurlDeformMonAffineAnimCmds)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if !((RunAffineAnimFromTaskData(
                    ((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40),
                )) != 0)
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBatonPassPokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        3u8,
                    )) as i16),
                );
                PrepareBattlerSpriteForRotScale(spriteId, 0u8);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(256i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(256i16);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(96i32)) as i16));
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(26i32)) as i16));
                SetSpriteRotScale(
                    spriteId,
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    0u16,
                );
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 5i32
                {
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
            }
            if __fall || __sw1 == 2i32 {
                __fall = true;
                let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p8).write((((((__p8).read()) as i32).wrapping_add(96i32)) as i16));
                let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p9).write((((((__p9).read()) as i32).wrapping_add(48i32)) as i16));
                SetSpriteRotScale(
                    spriteId,
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    0u16,
                );
                if (({
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 9i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    ResetSpriteRotScale(spriteId);
                    let __p12 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                let __p13 = (sprite).wrapping_add(38).cast::<i16>();
                (__p13).write((((((__p13).read()) as i32).wrapping_sub(6i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                    < (-32i32)
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWishStar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write((-16i16));
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(256i16);
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWishStar_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimWishStar_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut newX: u32 = 0u32;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(72i32)) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 4) as i16),
            );
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 4)
                    .wrapping_neg()) as i16),
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(16i32)) as i16));
        let __p3 = (sprite).wrapping_add(38).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
        if crate::c::rem_i32(
            (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32),
            3i32,
        ) == 0i32
        {
            CreateSpriteAndAnimate(
                (&raw const gMiniTwinklingStarSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
                ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_add(1i32)) as u8),
            );
        }
        newX = (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
        .wrapping_add(32i32)) as u32);
        if newX > 304u32 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMiniTwinklingStar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rand: u8 = 0u8;
        let mut y: i8 = 0i8;
        rand = ((((Random2()) as i32) & 3i32) as u8);
        if ((rand) as i32) == 0i32 {
            crate::c::bf_write(
                (sprite).wrapping_add(4),
                0,
                10,
                ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                    .wrapping_add(4i32)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(4),
                0,
                10,
                ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                    .wrapping_add(5i32)) as u16) as i32,
            );
        }
        y = ((((Random2()) as i32) & 7i32) as i8);
        if ((y) as i32) > 3i32 {
            y = ((((y) as i32).wrapping_neg()) as i8);
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(((y) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMiniTwinklingStar_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimMiniTwinklingStar_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < 30i32
        {
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 2i32
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 2i32
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == 3i32
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-1i16));
            }
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 60i32 {
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StockpileDeformMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !(((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0)
        {
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                GetAnimBattlerSpriteId(0u8),
                ((&raw const gStockpileDeformMonAffineAnimCmds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((RunAffineAnimFromTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SpitUpDeformMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !(((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0)
        {
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                GetAnimBattlerSpriteId(0u8),
                ((&raw const gSpitUpDeformMonAffineAnimCmds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((RunAffineAnimFromTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSwallowBlueOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                InitSpritePosToAnimAttacker(sprite, 0u8);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(2304i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        3u8,
                    )) as i16),
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            >> 8),
                    )) as i16),
                );
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(96i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                    > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SwallowDeformMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !(((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read())
            != 0)
        {
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                GetAnimBattlerSpriteId(0u8),
                ((&raw const gSwallowDeformMonAffineAnimCmds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((RunAffineAnimFromTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TransformMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut j: i32 = 0i32;
        let mut position: u8 = 0u8;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut dest: *mut u8 = core::ptr::null_mut();
        let mut src: *mut u8 = core::ptr::null_mut();
        let mut bgTilemap: *mut u16 = core::ptr::null_mut();
        let mut stretch: u16 = 0u16;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(76u8, 0u16);
                if ((GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                )) as i32)
                    == 1i32
                {
                    SetAnimBgAttribute(1u8, 2u8, 1u8);
                } else {
                    SetAnimBgAttribute(2u8, 2u8, 1u8);
                }
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t4 = (__p3).read();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    __t4
                }) as i32)
                    > 1i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    stretch = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u16);
                    SetGpuReg(
                        76u8,
                        (((((stretch) as i32) << 4) | ((stretch) as i32)) as u16),
                    );
                    if ((stretch) as i32) == 15i32 {
                        let __p6 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                HandleSpeciesGfxDataChange(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .read()) as u8),
                );
                GetBgDataForTransform(
                    (&raw mut animBg).cast::<u8>(),
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                );
                if (IsContest()) != 0 {
                    position = 0u8;
                } else {
                    position =
                        GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
                }
                src = (((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4))
                .cast::<*mut u8>())
                .wrapping_offset(((position) as i32) as isize))
                .read())
                .wrapping_offset(
                    ((((((&raw mut gBattleMonForms).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32)
                        << 11) as isize
                        * 1,
                );
                dest = (((&raw mut animBg).cast::<u8>()).cast::<*mut u8>()).read();
                'l2: loop {
                    'l3: {
                        'l4: loop {
                            'l5: {
                                CpuSet(
                                    src,
                                    dest,
                                    ((67108864i32
                                        | (crate::c::div_i32(
                                            crate::c::div_i32(4096i32, 2i32),
                                            crate::c::div_i32(32i32, 8i32),
                                        ) & 2097151i32))
                                        as u32),
                                );
                            }
                            if !((0i32) != 0) {
                                break 'l4;
                            }
                        }
                    }
                    if !((0i32) != 0) {
                        break 'l2;
                    }
                }
                LoadBgTiles(
                    1u8,
                    (((&raw mut animBg).cast::<u8>()).cast::<*mut u8>()).read(),
                    2048u16,
                    (((&raw mut animBg).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read(),
                );
                if (IsContest()) != 0 {
                    if ((IsSpeciesNotUnown(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .cast::<u16>())
                        .read(),
                    )) as i32)
                        != ((IsSpeciesNotUnown(
                            ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                                .wrapping_add(24)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read(),
                        )) as i32)
                    {
                        bgTilemap = (((&raw mut animBg).cast::<u8>())
                            .wrapping_add(4)
                            .cast::<*mut u16>())
                        .read();
                        {
                            i = 0i32;
                            'l6: loop {
                                if !(i < 8i32) {
                                    break 'l6;
                                }
                                'l7: {
                                    {
                                        j = 0i32;
                                        'l8: loop {
                                            if !(j < 4i32) {
                                                break 'l8;
                                            }
                                            'l9: {
                                                let mut temp: u16 = ((bgTilemap).wrapping_offset(
                                                    ((j).wrapping_add((i).wrapping_mul(32i32)))
                                                        as isize,
                                                ))
                                                .read();
                                                ((bgTilemap).wrapping_offset(
                                                    ((j).wrapping_add((i).wrapping_mul(32i32)))
                                                        as isize,
                                                ))
                                                .write(
                                                    ((bgTilemap).wrapping_offset(
                                                        (((7i32).wrapping_sub(j))
                                                            .wrapping_add((i).wrapping_mul(32i32)))
                                                            as isize,
                                                    ))
                                                    .read(),
                                                );
                                                ((bgTilemap).wrapping_offset(
                                                    (((7i32).wrapping_sub(j))
                                                        .wrapping_add((i).wrapping_mul(32i32)))
                                                        as isize,
                                                ))
                                                .write(temp);
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        {
                            i = 0i32;
                            'l10: loop {
                                if !(i < 8i32) {
                                    break 'l10;
                                }
                                'l11: {
                                    {
                                        j = 0i32;
                                        'l12: loop {
                                            if !(j < 8i32) {
                                                break 'l12;
                                            }
                                            'l13: {
                                                let __p7 = (bgTilemap).wrapping_offset(
                                                    ((j).wrapping_add((i).wrapping_mul(32i32)))
                                                        as isize,
                                                );
                                                (__p7).write(
                                                    (((((__p7).read()) as i32) ^ 1024i32) as u16),
                                                );
                                            }
                                            j = (j).wrapping_add(1);
                                        }
                                    }
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                    }
                    if (IsSpeciesNotUnown(
                        ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                            .wrapping_add(24)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read(),
                    )) != 0
                    {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(16)
                        .cast::<*mut *mut u8>())
                        .write(
                            ((&raw mut gAffineAnims_BattleSpriteContest).cast::<*mut u8>())
                                .cast::<*mut u8>(),
                        );
                    } else {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(16)
                        .cast::<*mut *mut u8>())
                        .write(
                            ((&raw mut gAffineAnims_BattleSpriteOpponentSide).cast::<*mut u8>())
                                .cast::<*mut u8>(),
                        );
                    }
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                    );
                }
                let __p8 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t10 = (__p9).read();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                    __t10
                }) as i32)
                    > 1i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .write(0i16);
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                    stretch = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as u16);
                    SetGpuReg(
                        76u8,
                        (((((stretch) as i32) << 4) | ((stretch) as i32)) as u16),
                    );
                    if ((stretch) as i32) == 0i32 {
                        let __p12 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p12).write(((__p12).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                SetGpuReg(76u8, 0u16);
                if ((GetBattlerSpriteBGPriorityRank(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                )) as i32)
                    == 1i32
                {
                    SetAnimBgAttribute(1u8, 2u8, 0u8);
                } else {
                    SetAnimBgAttribute(2u8, 2u8, 0u8);
                }
                if !((IsContest()) != 0) {
                    if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                        as i32)
                        == 1i32
                    {
                        if ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32)
                            == 0i32
                        {
                            SetBattlerShadowSpriteCallback(
                                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                                (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>())
                                    .read())
                                .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(
                                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .wrapping_add(2)
                                .cast::<u16>())
                                .read(),
                            );
                        }
                    }
                }
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsMonInvisible(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                false,
            ) as u16) as i16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CastformGfxDataChange(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        HandleSpeciesGfxDataChange(
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
            1u8,
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MorningSunLightBeam(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(80u8, 16194u16);
                SetGpuReg(82u8, 4096u16);
                SetAnimBgAttribute(1u8, 0u8, 0u8);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 1u8);
                }
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                AnimLoadCompressedBgTilemapHandleContest(
                    (&raw mut animBg).cast::<u8>(),
                    (((&raw mut gBattleAnimMaskTilemap_LightBeam).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                );
                if (IsContest()) != 0 {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65480u16);
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                } else {
                    if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                        as i32)
                        != 0i32
                    {
                        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65401u16);
                    } else {
                        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65526u16);
                    }
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                }
                AnimLoadCompressedBgGfx(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                    ((&raw mut gBattleAnimMaskImage_LightBeam).cast::<u32>()).cast::<u32>(),
                    (((((&raw mut animBg).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as u32),
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleAnimMaskPalette_LightBeam).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16));
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                PlaySE12WithPanning(228u16, BattleAnimAdjustPanning((-64i8)));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    let __t4 = (__p3).read();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                    __t4
                }) as i32)
                    > 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(0i16);
                    if (({
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        > 12i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(12i16);
                    }
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
                        == 12i32
                    {
                        let __p7 = ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_sub(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    < 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                }
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
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read())
                    != 0)
                {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                        ((((((((&raw const gMorningSunLightBeamCoordsTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<i8>())
                        .cast::<i8>())
                        .wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(2))
                            .read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_add(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(10))
                                .read()) as i32),
                            )) as u16),
                    );
                    if (({
                        let __p10 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2);
                        let __t11 = ((__p10).read()).wrapping_add(1);
                        (__p10).write(__t11);
                        __t11
                    }) as i32)
                        == 4i32
                    {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(4i16);
                    } else {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .write(3i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                    PlaySE12WithPanning(228u16, BattleAnimAdjustPanning((-64i8)));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                );
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGreenStar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut xOffset: i16 = 0i16;
        let mut spriteId1: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        xOffset = ((Random2()) as i16);
        xOffset = ((((xOffset) as i32) & 63i32) as i16);
        if ((xOffset) as i32) > 31i32 {
            xOffset = (((32i32).wrapping_sub(((xOffset) as i32))) as i16);
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(((xOffset) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(32i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        spriteId1 = CreateSprite(
            (&raw const gGreenStarSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((sprite).wrapping_add(32).cast::<i16>()).read(),
            ((sprite).wrapping_add(34).cast::<i16>()).read(),
            ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_add(1i32)) as u8),
        );
        spriteId2 = CreateSprite(
            (&raw const gGreenStarSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((sprite).wrapping_add(32).cast::<i16>()).read(),
            ((sprite).wrapping_add(34).cast::<i16>()).read(),
            ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_add(1i32)) as u8),
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId1) as i32) as isize * 68),
            1u8,
        );
        StartSpriteAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68),
            2u8,
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write((-1i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write((-1i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimGreenStar_Callback));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimGreenStar_Callback));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((spriteId1) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write(((spriteId2) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimGreenStar_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimGreenStar_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut delta: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .read()) as i32)
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16);
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub((((delta) as i32) >> 8))) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 0i32)
            && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) < (-8i32))
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p4).write(((__p4).read()).wrapping_add(1));
        }
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 1i32)
            && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) < (-16i32))
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p5).write(((__p5).read()).wrapping_add(1));
        }
        if (({
            let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t7 = ((__p6).read()).wrapping_sub(1);
            (__p6).write(__t7);
            __t7
        }) as i32)
            == (-1i32)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimGreenStar_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGreenStar_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (core::mem::transmute::<_, usize>(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read(),
        ) == (SpriteCallbackDummy as *const () as usize))
            && (core::mem::transmute::<_, usize>(
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .read(),
            ) == (SpriteCallbackDummy as *const () as usize))
        {
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 68,
            ));
            DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize
                    * 68,
            ));
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGreenStar_Callback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0) {
            let mut delta: i16 = ((((((((sprite).wrapping_add(46)).cast::<i16>())
                .wrapping_offset(3))
            .read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16);
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub((((delta) as i32) >> 8))) as i16));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t5 = ((__p4).read()).wrapping_sub(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == (-1i32)
            {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCallbackDummy));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DoomDesireLightBeam(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                SetGpuReg(80u8, 16194u16);
                SetGpuReg(82u8, 3331u16);
                SetAnimBgAttribute(1u8, 0u8, 0u8);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 1u8);
                }
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                AnimLoadCompressedBgTilemapHandleContest(
                    (&raw mut animBg).cast::<u8>(),
                    (((&raw mut gBattleAnimMaskTilemap_LightBeam).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                );
                if (IsContest()) != 0 {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65480u16);
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                } else {
                    let mut position: u8 =
                        GetBattlerPosition(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
                    if ((IsDoubleBattle()) as i32) == 1i32 {
                        if ((position) as i32) == 1i32 {
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65381u16);
                        }
                        if ((position) as i32) == 3i32 {
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65421u16);
                        }
                        if ((position) as i32) == 0i32 {
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(14u16);
                        }
                        if ((position) as i32) == 2i32 {
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65516u16);
                        }
                    } else {
                        if ((position) as i32) == 1i32 {
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65401u16);
                        }
                        if ((position) as i32) == 0i32 {
                            ((&raw mut gBattle_BG1_X).cast::<u16>()).write(65526u16);
                        }
                    }
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                }
                AnimLoadCompressedBgGfx(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                    ((&raw mut gBattleAnimMaskImage_LightBeam).cast::<u32>()).cast::<u32>(),
                    (((((&raw mut animBg).cast::<u8>())
                        .wrapping_add(10)
                        .cast::<u16>())
                    .read()) as u32),
                );
                LoadCompressedPalette(
                    ((&raw mut gBattleAnimMaskPalette_LightBeam).cast::<u32>()).cast::<u32>(),
                    (((0i32).wrapping_add(
                        (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    )) as u16),
                    32u16,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16));
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(0i16);
                if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                    == 1i32
                {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32)
                            .wrapping_add(
                                ((((((&raw const gDoomDesireLightBeamCoordTable)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<i8>())
                                .cast::<i8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32),
                            )) as u16),
                    );
                } else {
                    ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
                        ((((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32)
                            .wrapping_sub(
                                ((((((&raw const gDoomDesireLightBeamCoordTable)
                                    .cast::<u8>()
                                    .cast_mut()
                                    .cast::<i8>())
                                .cast::<i8>())
                                .wrapping_offset(
                                    ((((((((&raw mut gTasks).cast::<u8>())
                                        .wrapping_offset(((taskId) as i32) as isize * 40))
                                    .wrapping_add(8))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32),
                            )) as u16),
                    );
                }
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 5i32
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(5i16);
                } else {
                    let __p5 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t7 = ((__p6).read()).wrapping_sub(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    <= 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(5i16);
                }
                SetGpuReg(
                    82u8,
                    (((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8)
                        | 3i32) as u16),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 5i32
                {
                    let __p8 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > ((((((&raw const gDoomDesireLightBeamDelayTable)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>())
                    .wrapping_offset(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32) as isize,
                    ))
                    .read()) as i32)
                {
                    let __p11 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1);
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    > 13i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(13i16);
                }
                SetGpuReg(
                    82u8,
                    (((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        << 8)
                        | 3i32) as u16),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 13i32
                {
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .write(1i16);
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                );
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StrongFrustrationGrowAndShrink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                GetAnimBattlerSpriteId(0u8),
                ((&raw const gStrongFrustrationAffineAnimCmds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((RunAffineAnimFromTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWeakFrustrationAngerMark(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 0u8);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if (({
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t3 = (__p2).read();
                (__p2).write(((__p2).read()).wrapping_add(1));
                __t3
            }) as i32)
                > 20i32
            {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(160i32)) as i16));
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p5).write((((((__p5).read()) as i32).wrapping_add(128i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .read()) as i32)
                            >> 8)
                            .wrapping_neg()) as i16),
                    );
                } else {
                    ((sprite).wrapping_add(36).cast::<i16>()).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            >> 8) as i16),
                    );
                }
                let __p6 = (sprite).wrapping_add(38).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            >> 8),
                    )) as i16),
                );
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 64i32 {
                    DestroyAnimSprite(sprite);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_RockMonBackAndForth(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut side: u8 = 0u8;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
            .read())
            != 0)
        {
            DestroyAnimVisualTask(taskId);
            return;
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            < 0i32
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .write(0i16);
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            > 2i32
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .write(2i16);
        }
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            (((8i32).wrapping_sub(
                (2i32).wrapping_mul(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                ),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            (((256i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_mul(128i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                .wrapping_add(2i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                .wrapping_sub(1i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            side = GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
        } else {
            side = GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
        }
        if ((side) as i32) == 1i32 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
            (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        PrepareBattlerSpriteForRotScale(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            0u8,
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_RockMonBackAndForth_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_RockMonBackAndForth_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    256i16,
                    256i16,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
                );
                SetBattlerSpriteYOffsetFromRotation(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                if (({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    256i16,
                    256i16,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
                );
                SetBattlerSpriteYOffsetFromRotation(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                if (({
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_mul(2i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p11 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p12 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p12).write(
                    (((((__p12).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p13).write(
                    (((((__p13).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    256i16,
                    256i16,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
                );
                SetBattlerSpriteYOffsetFromRotation(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                if (({
                    let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t15 = ((__p14).read()).wrapping_add(1);
                    (__p14).write(__t15);
                    __t15
                }) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                {
                    if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                        let __p16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                        (__p16).write(((__p16).read()).wrapping_sub(1));
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                    } else {
                        let __p17 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p17).write(((__p17).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                ResetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSweetScentPetal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(34).cast::<i16>())
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(240i16);
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_sub(30i32)) as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as u8),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSweetScentPetal_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSweetScentPetal_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(3i32)) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(5i32)) as i16));
            let __p3 = (sprite).wrapping_add(34).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(1i32)) as i16));
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 240i32 {
                DestroyAnimSprite(sprite);
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 255i32) as i16),
                16i16,
            ));
        } else {
            let __p4 = (sprite).wrapping_add(32).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_sub(5i32)) as i16));
            let __p5 = (sprite).wrapping_add(34).cast::<i16>();
            (__p5).write((((((__p5).read()) as i32).wrapping_add(1i32)) as i16));
            if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < 0i32 {
                DestroyAnimSprite(sprite);
            }
            ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 255i32) as i16),
                16i16,
            ));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FlailMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(32i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(64i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(2048i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        PrepareBattlerSpriteForRotScale(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            0u8,
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_FlailMovement_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FlailMovement_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut temp: i32 = 0i32;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(512i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                {
                    let mut diff: i16 = ((((((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(14))
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                                as i32),
                        )) as i16);
                    let mut div: i16 = ((crate::c::div_i32(
                        ((diff) as i32),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16);
                    let mut r#mod: i16 = ((crate::c::rem_i32(
                        ((diff) as i32),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16);
                    if (((div) as i32) & 1i32) == 0i32 {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                                .read()) as i32)
                                .wrapping_sub(((r#mod) as i32)))
                                as i16),
                        );
                        (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                            ((((r#mod) as i32).wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                                    .read()) as i32),
                            )) as i16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(512i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        .wrapping_neg()
                {
                    let mut diff: i16 = ((((((((task).wrapping_add(8)).cast::<i16>())
                        .wrapping_offset(14))
                    .read()) as i32)
                        .wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                                as i32),
                        )) as i16);
                    let mut div: i16 = ((crate::c::div_i32(
                        ((diff) as i32),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16);
                    let mut r#mod: i16 = ((crate::c::rem_i32(
                        ((diff) as i32),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16);
                    if (1i32 & ((div) as i32)) == 0i32 {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                            ((((r#mod) as i32).wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                                    .read()) as i32),
                            )) as i16),
                        );
                        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                                .read()) as i32)
                                .wrapping_sub(((r#mod) as i32)))
                                as i16),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ResetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                );
                DestroyAnimVisualTask(taskId);
                return;
            }
        }
        SetSpriteRotScale(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            256i16,
            256i16,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
        );
        SetBattlerSpriteYOffsetFromRotation(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>())
        .write(
            ((((if {
                let __v4 =
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
                temp = __v4;
                __v4
            } >= 0i32
            {
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            } else {
                (temp).wrapping_add(63i32)
            }) >> 6)
                .wrapping_neg()) as i16),
        );
        if (({
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t6 = ((__p5).read()).wrapping_add(1);
            (__p5).write(__t6);
            __t6
        }) as i32)
            > 8i32
        {
            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) != 0 {
                let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                (__p7).write(((__p7).read()).wrapping_sub(1));
                let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                    < 16i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(16i16);
                }
            } else {
                (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPainSplitProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                == 0i32
            {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        3u8,
                    )) as i16),
                );
            }
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(128i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(768i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    >> 8) as i16),
            );
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                == 0i32)
                && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)
                    > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_neg())
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    (((crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_neg(),
                        3i32,
                    ))
                    .wrapping_mul(2i32)) as i16),
                );
            }
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p5).write((((((__p5).read()) as i32).wrapping_add(192i32)) as i16));
            let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p6).write((((((__p6).read()) as i32).wrapping_add(128i32)) as i16));
            if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_PainSplitMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i16));
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i16));
            }
            spriteId = GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(((spriteId) as i16));
            PrepareBattlerSpriteForRotScale(spriteId, 0u8);
            'l1: {
                let __sw1 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32);
                if __sw1 == 0i32 {
                    SetSpriteRotScale(spriteId, 224i16, 320i16, 0u16);
                    SetBattlerSpriteYOffsetFromYScale(spriteId);
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    SetSpriteRotScale(spriteId, 208i16, 304i16, 3840u16);
                    SetBattlerSpriteYOffsetFromYScale(spriteId);
                    if ((IsContest()) != 0)
                        || (((GetBattlerSide(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as u8),
                        )) as i32)
                            == 0i32)
                    {
                        let __p2 = (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p2).write((((((__p2).read()) as i32).wrapping_add(16i32)) as i16));
                    }
                    break 'l1;
                }
                if __sw1 == 2i32 {
                    SetSpriteRotScale(spriteId, 208i16, 304i16, 61696u16);
                    SetBattlerSpriteYOffsetFromYScale(spriteId);
                    if ((IsContest()) != 0)
                        || (((GetBattlerSide(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as u8),
                        )) as i32)
                            == 0i32)
                    {
                        let __p3 = (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p3).write((((((__p3).read()) as i32).wrapping_add(16i32)) as i16));
                    }
                    break 'l1;
                }
            }
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(2i16);
            let __p4 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p4).write(((__p4).read()).wrapping_add(1));
        } else {
            spriteId = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as u8);
            if (({
                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2);
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as i32)
                == 3i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
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
            if (({
                let __p7 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t8 = ((__p7).read()).wrapping_add(1);
                (__p7).write(__t8);
                __t8
            }) as i32)
                == 13i32
            {
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
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlatterConfetti(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut tileOffset: u8 = 0u8;
        let mut rand1: i32 = 0i32;
        let mut rand2: i32 = 0i32;
        tileOffset = ((crate::c::rem_i32(((Random2()) as i32), 12i32)) as u8);
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(((tileOffset) as i32))) as u16) as i32,
        );
        rand1 = (((Random2()) as i32) & 511i32);
        rand2 = (((Random2()) as i32) & 255i32);
        if (rand1 & 1i32) != 0 {
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write((((1504i32).wrapping_add(rand1)) as i16));
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write((((1504i32).wrapping_sub(rand1)) as i16));
        }
        if (rand2 & 1i32) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((1152i32).wrapping_add(rand2)) as i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((1152i32).wrapping_sub(rand2)) as i16));
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((sprite).wrapping_add(32).cast::<i16>()).write((-8i16));
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(248i16);
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(104i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFlatterConfetti_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFlatterConfetti_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
        } else {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_sub(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
        }
        let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p5).write((((((__p5).read()) as i32).wrapping_sub(22i32)) as i16));
        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p6).write((((((__p6).read()) as i32).wrapping_sub(48i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if (({
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t8 = ((__p7).read()).wrapping_add(1);
            (__p7).write(__t8);
            __t8
        }) as i32)
            == 31i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlatterSpotlight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(74u8, 7999u16);
        SetGpuRegBits(0u8, 32768u16);
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(64u8, ((&raw mut gBattle_WIN0H).cast::<u16>()).read());
        SetGpuReg(68u8, ((&raw mut gBattle_WIN0V).cast::<u16>()).read());
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        InitSpritePosToAnimTarget(sprite, 0u8);
        crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (2u32) as i32);
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFlatterSpotlight_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFlatterSpotlight_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 0i32
                {
                    ChangeSpriteAffineAnim(sprite, 1u8);
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetGpuReg(74u8, 16191u16);
                SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) ^ 32768i32) as u16));
                DestroyAnimSprite(sprite);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimReversalOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimReversalOrb_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimReversalOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 8) as i16),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 8) as i16),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(9i32)
                & 255i32) as i16),
        );
        if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u16)
            as i32)
            < 64i32)
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 195i32)
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
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) != 0) {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(1024i32)) as i16));
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(256i32)) as i16));
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(((__p3).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
            }
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 1i32
            {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(1024i32)) as i16));
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p5).write((((((__p5).read()) as i32).wrapping_sub(256i32)) as i16));
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p6).write(((__p6).read()).wrapping_add(1));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                {
                    DestroyAnimSprite(sprite);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_RolePlaySilhouette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut isBackPic: u8 = 0u8;
        let mut personality: u32 = 0u32;
        let mut otId: u32 = 0u32;
        let mut species: u16 = 0u16;
        let mut xOffset: i16 = 0i16;
        let mut priority: u32 = 0u32;
        let mut spriteId: u8 = 0u8;
        let mut coord1: i16 = 0i16;
        let mut coord2: i16 = 0i16;
        GetAnimBattlerSpriteId(0u8);
        if (IsContest()) != 0 {
            isBackPic = 1u8;
            personality = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(16)
            .cast::<u32>())
            .read();
            otId = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(12)
            .cast::<u32>())
            .read();
            species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .wrapping_add(2)
            .cast::<u16>())
            .read();
            xOffset = 20i16;
            priority =
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as u32);
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                isBackPic = 0u8;
                personality = GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    0i32,
                );
                otId = GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    1i32,
                );
                if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                        == 0i32
                    {
                        species = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                        )) as u16);
                    } else {
                        species = ((GetMonData2(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                        )) as u16);
                    }
                } else {
                    species = (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize * 4,
                    ))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read();
                }
                xOffset = 20i16;
                priority = ((GetBattlerSpriteBGPriority(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                )) as u32);
            } else {
                isBackPic = 1u8;
                personality = GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    0i32,
                );
                otId = GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    1i32,
                );
                if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    == 0i32
                {
                    if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                        == 0i32
                    {
                        species = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                        )) as u16);
                    } else {
                        species = ((GetMonData2(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            11i32,
                        )) as u16);
                    }
                } else {
                    species = (((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize * 4,
                    ))
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read();
                }
                xOffset = (-20i16);
                priority = ((GetBattlerSpriteBGPriority(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                )) as u32);
            }
        }
        coord1 = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
            as i16);
        coord2 = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
            as i16);
        spriteId = CreateAdditionalMonSpriteForMoveAnim(
            species,
            isBackPic,
            0u8,
            ((((coord1) as i32).wrapping_add(((xOffset) as i32))) as i16),
            coord2,
            5u8,
            personality,
            otId,
            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as u32),
            1u32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((priority) as u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            2,
            2,
            (1u32) as i32,
        );
        FillPalette(
            32767u16,
            (((256i32).wrapping_add(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    4,
                    4,
                    false,
                ) as u16) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((priority) as u16) as i32,
        );
        SetGpuReg(80u8, 16192u16);
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
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((spriteId) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_RolePlaySilhouette_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_RolePlaySilhouette_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 1i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
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
                == 10i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(256i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(256i16);
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_RolePlaySilhouette_Step2));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_RolePlaySilhouette_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u8);
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(16i32)) as i16));
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(11);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(128i32)) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                false,
            ) as u32)
                | 2u32) as i32,
        );
        TrySetSpriteRotScale(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            1u8,
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read(),
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read(),
            0u16,
        );
        if (({
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 9i32
        {
            ResetSpriteRotScale_PreserveAffine(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            DestroySpriteAndFreeResources_(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(DestroyAnimVisualTaskAndDisableBlend));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AcidArmor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = 0u8;
        let mut bgX: u16 = 0u16;
        let mut bgY: u16 = 0u16;
        let mut y: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(16i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(((battler) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(32i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(24i16);
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((((GetBattlerYCoordWithElevation(battler)) as i32).wrapping_sub(34i32)) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32) < 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(0i16);
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
                .wrapping_add(66i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        if ((GetBattlerSpriteBGPriorityRank(battler)) as i32) == 1i32 {
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108884i32) as usize as *mut u16).cast::<u8>());
            SetGpuReg(80u8, 16194u16);
            bgX = ((&raw mut gBattle_BG1_X).cast::<u16>()).read();
            bgY = ((&raw mut gBattle_BG1_Y).cast::<u16>()).read();
        } else {
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108888i32) as usize as *mut u16).cast::<u8>());
            SetGpuReg(80u8, 16196u16);
            bgX = ((&raw mut gBattle_BG2_X).cast::<u16>()).read();
            bgY = ((&raw mut gBattle_BG2_Y).cast::<u16>()).read();
        }
        {
            y = 0i16;
            i = 0i16;
            'l1: loop {
                if !(((y) as i32) < 160i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(bgX);
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(bgX);
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                    .write(bgY);
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset((((i) as i32).wrapping_add(1i32)) as isize))
                    .write(bgY);
                }
                y = (y).wrapping_add(1);
                i = ((((i) as i32).wrapping_add(2i32)) as i16);
            }
        }
        (((&raw mut scanlineParams).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(2791309313u32);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(9)).write(0u8);
        ScanlineEffect_SetParams(
            (&raw mut scanlineParams)
                .cast::<u8>()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_AcidArmor_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AcidArmor_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        let mut var1: i16 = 0i16;
        let mut var2: i16 = 0i16;
        let mut bgX: i16 = 0i16;
        let mut bgY: i16 = 0i16;
        let mut offset: i16 = 0i16;
        let mut var0: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut sineIndex: i16 = 0i16;
        let mut var3: i16 = 0i16;
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if ((GetBattlerSpriteBGPriorityRank(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u8),
        )) as i32)
            == 1i32
        {
            bgX = ((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16);
            bgY = ((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16);
        } else {
            bgX = ((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16);
            bgY = ((((&raw mut gBattle_BG2_Y).cast::<u16>()).read()) as i16);
        }
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                offset = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                    as i32)
                    .wrapping_mul(2i32)) as i16);
                var1 = 0i16;
                var2 = 0i16;
                i = 0i16;
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_add(2i32)
                        & 255i32) as i16),
                );
                sineIndex = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read();
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(
                    ((crate::c::div_i32(
                        2016i32,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
                    (((crate::c::div_i32(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(2i32),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32),
                    ))
                    .wrapping_neg()) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read());
                var3 = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                    as i32)
                    >> 5) as i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(var3);
                var0 = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read();
                'l2: loop {
                    if !(((var0) as i32)
                        > ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i32))
                    {
                        break 'l2;
                    }
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>())
                    .wrapping_offset((((offset) as i32).wrapping_add(1i32)) as isize))
                    .write(
                        (((((i) as i32).wrapping_sub(((var2) as i32))).wrapping_add(((bgY) as i32)))
                            as u16),
                    );
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>())
                    .wrapping_offset(((offset) as i32) as isize))
                    .write(
                        (((((bgX) as i32).wrapping_add(((var3) as i32))).wrapping_add(
                            (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(((sineIndex) as i32) as isize))
                            .read()) as i32)
                                >> 5),
                        )) as u16),
                    );
                    sineIndex = ((((sineIndex) as i32).wrapping_add(10i32) & 255i32) as i16);
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                                as i32),
                        )) as i16),
                    );
                    var3 = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                        .read()) as i32)
                        >> 5) as i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(var3);
                    i = (i).wrapping_add(1);
                    offset = ((((offset) as i32).wrapping_sub(2i32)) as i16);
                    var1 = ((((var1) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16);
                    var2 = ((((var1) as i32) >> 5) as i16);
                    var0 = (var0).wrapping_sub(1);
                }
                var0 = ((((var0) as i32).wrapping_mul(2i32)) as i16);
                'l3: loop {
                    if !(((var0) as i32) >= 0i32) {
                        break 'l3;
                    }
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((var0) as i32) as isize))
                    .write(((((bgX) as i32).wrapping_add(240i32)) as u16));
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((var0) as i32) as isize))
                    .write(((((bgX) as i32).wrapping_add(240i32)) as u16));
                    var0 = ((((var0) as i32).wrapping_sub(2i32)) as i16);
                }
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 63i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(64i16);
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                    } else {
                        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                    SetGpuReg(
                        82u8,
                        (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)
                            << 8)
                            | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .read()) as i32)) as u16),
                    );
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        == 0i32)
                        && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32)
                            == 16i32)
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                        let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                } else {
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    (__p9).write(
                        (((((__p9).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                                as i32),
                        )) as i16),
                    );
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    > 12i32
                {
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p12 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p13).write(((__p13).read()).wrapping_add(1));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    & 1i32)
                    != 0
                {
                    let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p14).write(((__p14).read()).wrapping_add(1));
                } else {
                    let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p15).write(((__p15).read()).wrapping_sub(1));
                }
                SetGpuReg(
                    82u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)) as u16),
                );
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 16i32)
                    && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        == 0i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DeepInhale(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        PrepareAffineAnimInTaskData(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            ((&raw const gDeepInhaleAffineAnimCmds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_DeepInhale_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DeepInhale_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        var0 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as u16);
        let __p1 = ((task).wrapping_add(8)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        var0 = ((((var0) as i32).wrapping_sub(20i32)) as u16);
        if ((var0) as i32) < 23i32 {
            if (({
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 1i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    & 1i32)
                    != 0
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(1i16);
                } else {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write((-1i16));
                }
            }
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
        }
        if !((RunAffineAnimFromTaskData(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        )) != 0)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn InitYawnCloudPosition(
    sprite: *mut u8,
    startX: i16,
    startY: i16,
    destX: i16,
    destY: i16,
    duration: u16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut startX = startX;
        let mut startY = startY;
        let mut destX = destX;
        let mut destY = destY;
        let mut duration = duration;
        ((sprite).wrapping_add(32).cast::<i16>()).write(startX);
        ((sprite).wrapping_add(34).cast::<i16>()).write(startY);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((startX) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((startY) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((crate::c::div_i32(
                (((destX) as i32).wrapping_sub(((startX) as i32)) << 4),
                ((duration) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::div_i32(
                (((destY) as i32).wrapping_sub(((startY) as i32)) << 4),
                ((duration) as i32),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn UpdateYawnCloudPosition(sprite: *mut u8) {
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
    }
}
pub(crate) unsafe extern "C" fn AnimYawnCloud(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut destX: i16 = ((sprite).wrapping_add(32).cast::<i16>()).read();
        let mut destY: i16 = ((sprite).wrapping_add(34).cast::<i16>()).read();
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        StartSpriteAffineAnim(
            sprite,
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
        );
        InitYawnCloudPosition(
            sprite,
            ((sprite).wrapping_add(32).cast::<i16>()).read(),
            ((sprite).wrapping_add(34).cast::<i16>()).read(),
            destX,
            destY,
            64u16,
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimYawnCloud_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimYawnCloud_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index: i32 = 0i32;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        index = ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(8i32)
            & 255i32);
        UpdateYawnCloudPosition(sprite);
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(((index) as i16), 8i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 58i32 {
            if (({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 1i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_add(1));
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32) as u16) as i32,
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 3i32
                {
                    DestroySpriteAndMatrix(sprite);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSmokeBallEscapeCloud(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        StartSpriteAffineAnim(
            sprite,
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(DestroyAnimSpriteAfterTimer));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SlideMonForFocusBand_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: u16 = 0u16;
        let mut var1: u16 = 0u16;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as i32)
            & 32768i32)
            != 0)
            && ((({
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_sub(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == (-1i32))
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .read()) as i32)
                == 0i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read(),
                );
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
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .write(0i16);
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                == 0i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read(),
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
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(0i16);
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read(),
            );
        }
        var0 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .read()) as u16);
        var1 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .read()) as u16);
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            & 32768i32)
            != 0
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as i32)
                    .wrapping_sub((((var0) as i32) >> 8))) as i16),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as i32)
                    .wrapping_add((((var0) as i32) >> 8))) as i16),
            );
        }
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            & 32768i32)
            != 0
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    .wrapping_sub((((var1) as i32) >> 8))) as i16),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    .wrapping_add((((var1) as i32) >> 8))) as i16),
            );
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            < 1i32
        {
            DestroyTask(taskId);
            let __p4 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
            (__p4).write(((__p4).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SlideMonForFocusBand_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: u16 = 0u16;
        let mut var1: u16 = 0u16;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .read()) as i32)
            & 32768i32)
            != 0)
            && ((({
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_sub(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                == (-1i32))
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .read()) as i32)
                == 0i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read(),
                );
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
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .write(((var0) as i16));
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                == 0i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5))
                    .read(),
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
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(0i16);
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(13))
                .read(),
            );
        }
        var0 = (((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            & 32767i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .read()) as i32),
            )) as u16);
        var1 = (((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            & 32767i32)
            .wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .read()) as i32),
            )) as u16);
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            & 32768i32)
            != 0
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as i32)
                    .wrapping_sub((((var0) as i32) >> 8))) as i16),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read()) as i32)
                    .wrapping_add((((var0) as i32) >> 8))) as i16),
            );
        }
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            & 32768i32)
            != 0
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    .wrapping_sub((((var1) as i32) >> 8))) as i16),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    .wrapping_add((((var1) as i32) >> 8))) as i16),
            );
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((var0) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(((var1) as i16));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            < 1i32
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(30i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(13))
            .write(0i16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_SlideMonForFocusBand_Step2));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SlideMonForFocusBand(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(14))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(13))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
        );
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            != 0
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as i32)
                    | (-32768i32)) as i16),
            );
        }
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
        } else {
            if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                & 32768i32)
                != 0
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        & 32767i32) as i16),
                );
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        | (-32768i32)) as i16),
                );
            }
            if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                & 32768i32)
                != 0
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(
                    ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32)
                        & 32767i32) as i16),
                );
            } else {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(
                    ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32)
                        | (-32768i32)) as i16),
                );
            }
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_SlideMonForFocusBand_Step1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SquishAndSweatDroplets(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut battler: u8 = 0u8;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
            .read())
            != 0)
        {
            DestroyAnimVisualTask(taskId);
        }
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
            .write(((GetBattlerSpriteCoord(battler, 0u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
            .write(((GetBattlerSpriteCoord(battler, 1u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
            .write(((GetBattlerSpriteSubpriority(battler)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        PrepareAffineAnimInTaskData(
            task,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            ((&raw const gFacadeSquishAffineAnimCmds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_SquishAndSweatDroplets_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SquishAndSweatDroplets_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 6i32
                {
                    CreateSweatDroplets(taskId, 1u8);
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 18i32
                {
                    CreateSweatDroplets(taskId, 0u8);
                }
                if !((RunAffineAnimFromTaskData(task)) != 0) {
                    if (({
                        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        let __t4 = ((__p3).read()).wrapping_sub(1);
                        (__p3).write(__t4);
                        __t4
                    }) as i32)
                        == 0i32
                    {
                        let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                        PrepareAffineAnimInTaskData(
                            task,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            ((&raw const gFacadeSquishAffineAnimCmds)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateSweatDroplets(taskId: u8, lowerDroplets: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut lowerDroplets = lowerDroplets;
        let mut i: u8 = 0u8;
        let mut xOffset: i8 = 0i8;
        let mut yOffset: i8 = 0i8;
        let mut task: *mut u8 = core::ptr::null_mut();
        let mut xCoords = crate::ffi::Align4([0u8; 8]);
        let mut yCoords = crate::ffi::Align4([0u8; 4]);
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((lowerDroplets) != 0) {
            xOffset = 18i8;
            yOffset = (-20i8);
        } else {
            xOffset = 30i8;
            yOffset = 20i8;
        }
        ((&raw mut xCoords).cast::<i16>()).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_sub(((xOffset) as i32))) as i16),
        );
        (((&raw mut xCoords).cast::<i16>()).wrapping_offset(1)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_sub(((xOffset) as i32)))
            .wrapping_sub(4i32)) as i16),
        );
        (((&raw mut xCoords).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_add(((xOffset) as i32))) as i16),
        );
        (((&raw mut xCoords).cast::<i16>()).wrapping_offset(3)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_add(((xOffset) as i32)))
            .wrapping_add(4i32)) as i16),
        );
        ((&raw mut yCoords).cast::<i16>()).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(((yOffset) as i32))) as i16),
        );
        (((&raw mut yCoords).cast::<i16>()).wrapping_offset(1)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(((yOffset) as i32)))
            .wrapping_add(6i32)) as i16),
        );
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const gFacadeSweatDropSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        (((&raw mut xCoords).cast::<i16>()).wrapping_offset(((i) as i32) as isize))
                            .read(),
                        (((&raw mut yCoords).cast::<i16>())
                            .wrapping_offset((((i) as i32) & 1i32) as isize))
                        .read(),
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32)
                            .wrapping_sub(5i32)) as u8),
                    );
                    if ((spriteId) as i32) != 64i32 {
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(0i16);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(((if ((i) as i32) < 2i32 { (-2i32) } else { 2i32 }) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write((-1i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(((taskId) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(2i16);
                        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFacadeSweatDrop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 6i32
        {
            let __p5 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    as isize,
            );
            (__p5).write(((__p5).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FacadeColorBlend(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        spriteId = GetAnimBattlerSpriteId(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            (((256i32).wrapping_add(
                ((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    4,
                    4,
                    false,
                ) as u16) as i32)
                    .wrapping_mul(16i32),
            )) as i16),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_FacadeColorBlend_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FacadeColorBlend_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read())
            != 0
        {
            BlendPalette(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u16),
                16u16,
                8u8,
                ((((&raw const gFacadeBlendColors)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize,
                ))
                .read(),
            );
            if (({
                let __p1 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 23i32
            {
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(0i16);
            }
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_sub(1));
        } else {
            BlendPalette(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as u16),
                16u16,
                0u8,
                0u16,
            );
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StatusClearedEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        StartMonScrollingBgMask(
            taskId,
            0i32,
            416u16,
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            10u8,
            2u8,
            30u8,
            ((&raw mut gCureBubblesGfx).cast::<u32>()).cast::<u32>(),
            ((&raw mut gCureBubblesTilemap).cast::<u32>()).cast::<u32>(),
            ((&raw mut gCureBubblesPal).cast::<u32>()).cast::<u32>(),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimRoarNoiseLine(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
        );
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(640i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-640i16));
        } else {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                == 1i32
            {
                crate::c::bf_write((sprite).wrapping_add(63), 1, 1, (1u16) as i32);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(640i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(640i16);
            } else {
                StartSpriteAnim(sprite, 1u8);
                (((sprite).wrapping_add(46)).cast::<i16>()).write(640i16);
            }
        }
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRoarNoiseLine_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRoarNoiseLine_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                >> 8) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                >> 8) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 14i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GlareEyeDots(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (IsContest()) != 0 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(8i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(3i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(12i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(3i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        }
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_add(crate::c::div_i32(
                        ((GetBattlerSpriteCoordAttr(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            0u8,
                        )) as i32),
                        4i32,
                    ))) as i16),
            );
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_sub(crate::c::div_i32(
                        ((GetBattlerSpriteCoordAttr(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            0u8,
                        )) as i32),
                        4i32,
                    ))) as i16),
            );
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_sub(crate::c::div_i32(
                    ((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        0u8,
                    )) as i32),
                    4i32,
                ))) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_GlareEyeDots_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_GlareEyeDots_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
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
                    > 3i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    GetGlareEyeDotCoords(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as u8),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as u8),
                        &raw mut x,
                        &raw mut y,
                    );
                    {
                        i = 0u8;
                        'l2: loop {
                            if !(((i) as i32) < 2i32) {
                                break 'l2;
                            }
                            'l3: {
                                let mut spriteId: u8 = CreateSprite(
                                    (&raw const gGlareEyeDotSpriteTemplate)
                                        .cast::<u8>()
                                        .cast_mut(),
                                    x,
                                    y,
                                    35u8,
                                );
                                if ((spriteId) as i32) != 64i32 {
                                    if !((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(7))
                                    .read())
                                        != 0)
                                    {
                                        if ((i) as i32) == 0i32 {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((spriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(36)
                                            .cast::<i16>())
                                            .write({
                                                let __v4 = ((((((((task).wrapping_add(8))
                                                    .cast::<i16>())
                                                .wrapping_offset(6))
                                                .read())
                                                    as i32)
                                                    .wrapping_neg())
                                                    as i16);
                                                ((((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((spriteId) as i32) as isize * 68,
                                                    ))
                                                .wrapping_add(38)
                                                .cast::<i16>())
                                                .write(__v4);
                                                __v4
                                            });
                                        } else {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((spriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(36)
                                            .cast::<i16>())
                                            .write({
                                                let __v5 = ((((task).wrapping_add(8))
                                                    .cast::<i16>())
                                                .wrapping_offset(6))
                                                .read();
                                                ((((&raw mut gSprites).cast::<u8>())
                                                    .wrapping_offset(
                                                        ((spriteId) as i32) as isize * 68,
                                                    ))
                                                .wrapping_add(38)
                                                .cast::<i16>())
                                                .write(__v5);
                                                __v5
                                            });
                                        }
                                    } else {
                                        if ((i) as i32) == 0i32 {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((spriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(36)
                                            .cast::<i16>())
                                            .write(
                                                ((((((((task).wrapping_add(8)).cast::<i16>())
                                                    .wrapping_offset(6))
                                                .read())
                                                    as i32)
                                                    .wrapping_neg())
                                                    as i16),
                                            );
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((spriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(38)
                                            .cast::<i16>())
                                            .write(
                                                ((((task).wrapping_add(8)).cast::<i16>())
                                                    .wrapping_offset(6))
                                                .read(),
                                            );
                                        } else {
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((spriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(36)
                                            .cast::<i16>())
                                            .write(
                                                ((((task).wrapping_add(8)).cast::<i16>())
                                                    .wrapping_offset(6))
                                                .read(),
                                            );
                                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                                ((spriteId) as i32) as isize * 68,
                                            ))
                                            .wrapping_add(38)
                                            .cast::<i16>())
                                            .write(
                                                ((((((((task).wrapping_add(8)).cast::<i16>())
                                                    .wrapping_offset(6))
                                                .read())
                                                    as i32)
                                                    .wrapping_neg())
                                                    as i16),
                                            );
                                        }
                                    }
                                    (((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .write(0i16);
                                    ((((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(1))
                                    .write(((taskId) as i16));
                                    ((((((&raw mut gSprites).cast::<u8>())
                                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                                    .wrapping_add(46))
                                    .cast::<i16>())
                                    .wrapping_offset(2))
                                    .write(10i16);
                                    let __p6 = (((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(10);
                                    (__p6).write(((__p6).read()).wrapping_add(1));
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                    {
                        let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetGlareEyeDotCoords(
    startX: i16,
    startY: i16,
    endX: i16,
    endY: i16,
    pairMax: u8,
    pairNum: u8,
    x: *mut i16,
    y: *mut i16,
) {
    unsafe {
        let mut startX = startX;
        let mut startY = startY;
        let mut endX = endX;
        let mut endY = endY;
        let mut pairMax = pairMax;
        let mut pairNum = pairNum;
        let mut x = x;
        let mut y = y;
        let mut x2: i32 = 0i32;
        let mut y2: i32 = 0i32;
        if ((pairNum) as i32) == 0i32 {
            (x).write(startX);
            (y).write(startY);
            return;
        }
        if ((pairNum) as i32) >= ((pairMax) as i32) {
            (x).write(endX);
            (y).write(endY);
            return;
        }
        pairMax = (pairMax).wrapping_sub(1);
        x2 = (((startX) as i32) << 8).wrapping_add(((pairNum) as i32).wrapping_mul(
            crate::c::div_i32(
                (((endX) as i32).wrapping_sub(((startX) as i32)) << 8),
                ((pairMax) as i32),
            ),
        ));
        y2 = (((startY) as i32) << 8).wrapping_add(((pairNum) as i32).wrapping_mul(
            crate::c::div_i32(
                (((endY) as i32).wrapping_sub(((startY) as i32)) << 8),
                ((pairMax) as i32),
            ),
        ));
        (x).write(((x2 >> 8) as i16));
        (y).write(((y2 >> 8) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimGlareEyeDot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 36i32
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize,
            );
            (__p3).write(((__p3).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimAssistPawprint(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(InitAndRunAnimFastLinearTranslation));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BarrageBall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(crate::c::div_i32(
                    ((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        0u8,
                    )) as i32),
                    4i32,
                ))) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(((CreateSprite((&raw const gBarrageBallSpriteTemplate).cast::<u8>().cast_mut(), ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read(), ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read(), ((((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))).wrapping_sub(5i32)) as u8))) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32) != 64i32
        {
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .write(16i16);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read());
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read());
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write((-32i16));
            InitAnimArcTranslation(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ));
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 1i32
            {
                StartSpriteAffineAnim(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ),
                    1u8,
                );
            }
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_BarrageBall_Step));
        } else {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BarrageBall_Step(taskId: u8) {
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
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    TranslateAnimHorizontalArc(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ));
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        > 7i32
                    {
                        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (TranslateAnimHorizontalArc(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))) != 0
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 1i32) as u16) as i32,
                    );
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 16i32
                    {
                        FreeOamMatrix(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(15))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(3),
                                1,
                                5,
                                false,
                            ) as u32) as u8),
                        );
                        DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ));
                        let __p11 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSmellingSaltsHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(16i32)) as u16) as i32,
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                == 0i32
            {
                (-1i32)
            } else {
                1i32
            }) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>())
            .write(((GetBattlerSpriteCoord(battler, 3u8)) as i16));
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read())
            as i32)
            == 0i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(3),
                1,
                5,
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) | 8u32) as i32,
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoordAttr(battler, 4u8)) as i32).wrapping_sub(8i32)) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoordAttr(battler, 5u8)) as i32).wrapping_add(8i32)) as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSmellingSaltsHand_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSmellingSaltsHand_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p4 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32),
                        )) as i16),
                    );
                    if (({
                        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        == 12i32
                    {
                        let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 8i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p11 = (sprite).wrapping_add(36).cast::<i16>();
                (__p11).write(
                    (((((__p11).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(4i32),
                    )) as i16),
                );
                if (({
                    let __p12 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 6i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p14 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p15 = (sprite).wrapping_add(36).cast::<i16>();
                (__p15).write(
                    (((((__p15).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                if (({
                    let __p16 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t17 = ((__p16).read()).wrapping_add(1);
                    (__p16).write(__t17);
                    __t17
                }) as i32)
                    == 8i32
                {
                    if ({
                        let __p18 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                        let __t19 = ((__p18).read()).wrapping_sub(1);
                        (__p18).write(__t19);
                        __t19
                    }) != 0
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .write(0i16);
                        let __p20 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p20).write(((__p20).read()).wrapping_sub(1));
                    } else {
                        DestroyAnimSprite(sprite);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SmellingSaltsSquish(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            DestroyAnimVisualTask(taskId);
        } else {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write(
                ((GetAnimBattlerSpriteId(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
                )) as i16),
            );
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as u8),
                ((&raw const gSmellingSaltsSquishAffineAnimCmds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_SmellingSaltsSquish_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SmellingSaltsSquish_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            if !((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                & 1i32)
                != 0)
            {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(2i16);
            } else {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write((-2i16));
            }
        }
        if !((RunAffineAnimFromTaskData(task)) != 0) {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            if ({
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) != 0
            {
                PrepareAffineAnimInTaskData(
                    ((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40),
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as u8),
                    ((&raw const gSmellingSaltsSquishAffineAnimCmds)
                        .cast::<u8>()
                        .cast_mut())
                    .cast::<u8>(),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            } else {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSmellingSaltExclamation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(GetBattlerSpriteCoordAttr(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                2u8,
            ));
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(GetBattlerSpriteCoordAttr(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                2u8,
            ));
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 8i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(8i16);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSmellingSaltExclamation_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSmellingSaltExclamation_Step(sprite: *mut u8) {
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
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_add(1i32)
                    & 1i32) as i16),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16)
                    as i32,
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) != 0)
                && ((({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 0i32)
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHelpingHandClap(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            crate::c::bf_write(
                (sprite).wrapping_add(3),
                1,
                5,
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) | 8u32) as i32,
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(100i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(140i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write((-1i16));
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(56i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimHelpingHandClap_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimHelpingHandClap_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(34).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16),
                );
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    & 1i32)
                    != 0
                {
                    let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32)
                                .wrapping_mul(2i32),
                        )) as i16),
                    );
                }
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 9i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == 4i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p9 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p10).write(((__p10).read()).wrapping_add(1));
                let __p11 = (sprite).wrapping_add(34).cast::<i16>();
                (__p11).write(
                    (((((__p11).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(
                            (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(
                                    (((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_mul(10i32))
                                        as isize,
                                ))
                            .read()) as i32)
                                >> 3),
                        )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 12i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p12 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p13 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t14 = ((__p13).read()).wrapping_add(1);
                    (__p13).write(__t14);
                    __t14
                }) as i32)
                    == 2i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p15 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p16 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p16).write(((__p16).read()).wrapping_add(1));
                let __p17 = (sprite).wrapping_add(34).cast::<i16>();
                (__p17).write(
                    (((((__p17).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(
                            (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(
                                    (((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_mul(10i32))
                                        as isize,
                                ))
                            .read()) as i32)
                                >> 3),
                        )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 12i32
                {
                    let __p18 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                let __p19 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p19).write(((__p19).read()).wrapping_add(1));
                let __p20 = (sprite).wrapping_add(34).cast::<i16>();
                (__p20).write(
                    (((((__p20).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_mul(
                            (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(
                                    (((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(1))
                                    .read()) as i32)
                                        .wrapping_mul(10i32))
                                        as isize,
                                ))
                            .read()) as i32)
                                >> 3),
                        )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 15i32
                {
                    crate::c::bf_write(
                        (sprite).wrapping_add(4),
                        0,
                        10,
                        ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16)
                            as i32)
                            .wrapping_add(16i32)) as u16) as i32,
                    );
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    == 18i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p21 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p21).write(((__p21).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                let __p22 = (sprite).wrapping_add(32).cast::<i16>();
                (__p22).write(
                    (((((__p22).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(6i32),
                    )) as i16),
                );
                if (({
                    let __p23 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t24 = ((__p23).read()).wrapping_add(1);
                    (__p23).write(__t24);
                    __t24
                }) as i32)
                    == 9i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p25 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p25).write(((__p25).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                let __p26 = (sprite).wrapping_add(32).cast::<i16>();
                (__p26).write(
                    (((((__p26).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16),
                );
                if (({
                    let __p27 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t28 = ((__p27).read()).wrapping_add(1);
                    (__p27).write(__t28);
                    __t28
                }) as i32)
                    == 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p29 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p29).write(((__p29).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                let __p30 = (sprite).wrapping_add(32).cast::<i16>();
                (__p30).write(
                    (((((__p30).read()) as i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                if (({
                    let __p31 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t32 = ((__p31).read()).wrapping_add(1);
                    (__p31).write(__t32);
                    __t32
                }) as i32)
                    == 5i32
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HelpingHandAttackerMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        if !((IsContest()) != 0) {
            if ((IsDoubleBattle()) as i32) == 1i32 {
                let mut attackerX: i32 = ((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i32);
                let mut partnerX: i32 = ((GetBattlerSpriteCoord(
                    ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                        as u8),
                    0u8,
                )) as i32);
                if attackerX > partnerX {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(1i16);
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write((-1i16));
                }
            } else {
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write((-1i16));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(1i16);
                }
            }
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(1i16);
        }
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_HelpingHandAttackerMovement_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_HelpingHandAttackerMovement_Step(taskId: u8) {
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
                    == 13i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                if (({
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 6i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p9 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p9).write(
                    (((((__p9).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(3i32),
                    )) as i16),
                );
                if (({
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 6i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p12 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t14 = ((__p13).read()).wrapping_add(1);
                    (__p13).write(__t14);
                    __t14
                }) as i32)
                    == 2i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 0i32
                    {
                        let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                    } else {
                        let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p16).write(((__p16).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p17 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p17).write(
                    (((((__p17).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p18 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t19 = ((__p18).read()).wrapping_add(1);
                    (__p18).write(__t19);
                    __t19
                }) as i32)
                    == 3i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p20 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p20).write(((__p20).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                if (({
                    let __p21 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t22 = ((__p21).read()).wrapping_add(1);
                    (__p21).write(__t22);
                    __t22
                }) as i32)
                    == 6i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p23 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p23).write(((__p23).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                let __p24 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p24).write(
                    (((((__p24).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(4i32),
                    )) as i16),
                );
                if (({
                    let __p25 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t26 = ((__p25).read()).wrapping_add(1);
                    (__p25).write(__t26);
                    __t26
                }) as i32)
                    == 5i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p27 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p27).write(((__p27).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                let __p28 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p28).write(
                    (((((__p28).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32)
                            .wrapping_mul(4i32),
                    )) as i16),
                );
                if (({
                    let __p29 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t30 = ((__p29).read()).wrapping_add(1);
                    (__p29).write(__t30);
                    __t30
                }) as i32)
                    == 5i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p31 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p31).write(((__p31).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 8i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimForesightMagnifyingGlass(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 1u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i16));
        }
        if ((GetBattlerSide(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
        )) as i32)
            == 1i32
        {
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
        }
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            2,
            2,
            ((GetBattlerSpriteBGPriority(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as u8),
            )) as u16) as i32,
        );
        crate::c::bf_write((sprite).wrapping_add(1), 2, 2, (1u32) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimForesightMagnifyingGlass_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimForesightMagnifyingGlass_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32);
            if __sw1 == 0i32 {
                'l2: {
                    let __sw2 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                        .read()) as i32);
                    let __matched = __sw2 == 0i32
                        || __sw2 == 4i32
                        || __sw2 == 1i32
                        || __sw2 == 2i32
                        || __sw2 == 3i32
                        || __sw2 == 5i32;
                    let mut __fall = false;
                    if !__matched {
                        __fall = true;
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                            .write(0i16);
                    }
                    if __fall || __sw2 == 0i32 || __sw2 == 4i32 {
                        __fall = true;
                        x = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            5u8,
                        )) as i32)
                            .wrapping_sub(4i32)) as u16);
                        y = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            3u8,
                        )) as i32)
                            .wrapping_sub(4i32)) as u16);
                        break 'l2;
                    }
                    if __sw2 == 1i32 {
                        __fall = true;
                        x = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            5u8,
                        )) as i32)
                            .wrapping_sub(4i32)) as u16);
                        y = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            2u8,
                        )) as i32)
                            .wrapping_add(4i32)) as u16);
                        break 'l2;
                    }
                    if __sw2 == 2i32 {
                        __fall = true;
                        x = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            4u8,
                        )) as i32)
                            .wrapping_add(4i32)) as u16);
                        y = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            3u8,
                        )) as i32)
                            .wrapping_sub(4i32)) as u16);
                        break 'l2;
                    }
                    if __sw2 == 3i32 {
                        __fall = true;
                        x = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            4u8,
                        )) as i32)
                            .wrapping_add(4i32)) as u16);
                        y = ((((GetBattlerSpriteCoordAttr(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            2u8,
                        )) as i32)
                            .wrapping_sub(4i32)) as u16);
                        break 'l2;
                    }
                    if __sw2 == 5i32 {
                        __fall = true;
                        x = ((GetBattlerSpriteCoord(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            2u8,
                        )) as u16);
                        y = ((GetBattlerSpriteCoord(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as u8),
                            3u8,
                        )) as u16);
                        break 'l2;
                    }
                }
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    == 4i32
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(24i16);
                } else {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        == 5i32
                    {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
                    } else {
                        (((sprite).wrapping_add(46)).cast::<i16>()).write(12i16);
                    }
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                    .write(((x) as i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                    .write(((y) as i16));
                InitAnimLinearTranslation(sprite);
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (AnimTranslateLinear(sprite)) != 0 {
                    'l3: {
                        let __sw4 = ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset(6))
                        .read()) as i32);
                        let __matched = __sw4 == 4i32 || __sw4 == 5i32;
                        if !__matched {
                            let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                            (__p5).write(
                                (((((__p5).read()) as i32).wrapping_add(
                                    ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                                )) as i16),
                            );
                            let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                            (__p6).write(
                                (((((__p6).read()) as i32).wrapping_add(
                                    ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32),
                                )) as i16),
                            );
                            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                            let __p7 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                            (__p7).write(((__p7).read()).wrapping_add(1));
                            let __p8 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                            (__p8).write(((__p8).read()).wrapping_add(1));
                            break 'l3;
                        }
                        if __sw4 == 4i32 {
                            let __p9 = (sprite).wrapping_add(32).cast::<i16>();
                            (__p9).write(
                                (((((__p9).read()) as i32).wrapping_add(
                                    ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                                )) as i16),
                            );
                            let __p10 = (sprite).wrapping_add(34).cast::<i16>();
                            (__p10).write(
                                (((((__p10).read()) as i32).wrapping_add(
                                    ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32),
                                )) as i16),
                            );
                            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .write(0i16);
                            let __p11 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                            (__p11).write(((__p11).read()).wrapping_add(1));
                            break 'l3;
                        }
                        if __sw4 == 5i32 {
                            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .write(16i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .write(0i16);
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .write(3i16);
                            break 'l3;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p12 = ((sprite).wrapping_add(46)).cast::<i16>();
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 4i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !(((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) & 1i32) != 0) {
                    let __p14 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p14).write(((__p14).read()).wrapping_sub(1));
                } else {
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                SetGpuReg(
                    82u8,
                    (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        << 8)
                        | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)) as u16),
                );
                if (({
                    let __p16 = ((sprite).wrapping_add(46)).cast::<i16>();
                    let __t17 = ((__p16).read()).wrapping_add(1);
                    (__p16).write(__t17);
                    __t17
                }) as i32)
                    == 32i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    let __p18 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    (__p18).write(((__p18).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                DestroyAnimSprite(sprite);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMeteorMashStar_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                .wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                ),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    .wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    ))
                .wrapping_mul(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                ),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            & 1i32)
            != 0)
        {
            CreateSprite(
                (&raw const gMiniTwinklingStarSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
                5u8,
            );
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AnimMeteorMashStar(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16);
        let mut x: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16);
        if (((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32)
            || ((IsContest()) != 0)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_sub(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMeteorMashStar_Step));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MonToSubstitute(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            PrepareBattlerSpriteForRotScale(spriteId, 0u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(256i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(256i16);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 1i32
            {
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(96i32)) as i16));
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(13i32)) as i16));
                SetSpriteRotScale(
                    spriteId,
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read(),
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read(),
                    0u16,
                );
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 9i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    ResetSpriteRotScale(spriteId);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
            } else {
                LoadBattleMonGfxAndAnimate(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                    spriteId,
                );
                if (IsContest()) != 0 {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(16)
                    .cast::<*mut *mut u8>())
                    .write(
                        ((&raw mut gAffineAnims_BattleSpriteContest).cast::<*mut u8>())
                            .cast::<*mut u8>(),
                    );
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ),
                        0u8,
                    );
                }
                {
                    i = 0i32;
                    'l1: loop {
                        if !(i < 16i32) {
                            break 'l1;
                        }
                        'l2: {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset((i) as isize))
                            .write(0i16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_MonToSubstituteDoll));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MonToSubstituteDoll(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .write((-200i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(200i16);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(0i16);
                let __p2 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(112i32)) as i16));
                let __p4 = (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32)
                            >> 8),
                    )) as i16),
                );
                if ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32),
                    )
                    >= (-32i32)
                {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                }
                if ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    > 0i32
                {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(0i16);
                }
                if ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    PlaySE12WithPanning(125u16, BattleAnimAdjustPanning((-64i8)));
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p5).write((((((__p5).read()) as i32).wrapping_sub(2048i32)) as i16));
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p7).write((((((__p7).read()) as i32).wrapping_sub(112i32)) as i16));
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    < 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                }
                let __p8 = (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>();
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_sub(
                        (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32)
                            >> 8),
                    )) as i16),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    == 0i32
                {
                    let __p9 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p10 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10);
                (__p10).write((((((__p10).read()) as i32).wrapping_add(112i32)) as i16));
                let __p11 = (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>();
                (__p11).write(
                    (((((__p11).read()) as i32).wrapping_add(
                        (((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(10))
                        .read()) as i32)
                            >> 8),
                    )) as i16),
                );
                if ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    > 0i32
                {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .write(0i16);
                }
                if ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    PlaySE12WithPanning(125u16, BattleAnimAdjustPanning((-64i8)));
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBlockX(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y: i16 = 0i16;
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            ((sprite).wrapping_add(67)).write(((((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))).wrapping_sub(2i32)) as u8));
            y = (-144i16);
        } else {
            ((sprite).wrapping_add(67)).write(((((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))).wrapping_add(2i32)) as u8));
            y = (-96i16);
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(y);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBlockX_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBlockX_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(38).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(10i32)) as i16));
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
                    PlaySE12WithPanning(205u16, BattleAnimAdjustPanning(63i8));
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(4i32)) as i16));
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 3)
                        .wrapping_neg()) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 127i32
                {
                    PlaySE12WithPanning(205u16, BattleAnimAdjustPanning(63i8));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(6i32)) as i16));
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 4)
                        .wrapping_neg()) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 127i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 8i32
                {
                    PlaySE12WithPanning(192u16, BattleAnimAdjustPanning(63i8));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p10 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (({
                    let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    > 8i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p13 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p13).write(((__p13).read()).wrapping_add(1));
                    crate::c::bf_write(
                        (sprite).wrapping_add(62),
                        2,
                        1,
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 1i32) as u16) as i32,
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 7i32
                    {
                        DestroyAnimSprite(sprite);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_OdorSleuthMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId1: i16 = 0i16;
        let mut spriteId2: i16 = 0i16;
        if (IsContest()) != 0 {
            DestroyAnimVisualTask(taskId);
            return;
        }
        spriteId1 = CloneBattlerSpriteWithBlend(1u8);
        if ((spriteId1) as i32) < 0i32 {
            DestroyAnimVisualTask(taskId);
            return;
        }
        spriteId2 = CloneBattlerSpriteWithBlend(1u8);
        if ((spriteId2) as i32) < 0i32 {
            DestroySpriteWithActiveSheet(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId1) as i32) as isize * 68),
            );
            DestroyAnimVisualTask(taskId);
            return;
        }
        let __p1 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(24i32)) as i16));
        let __p2 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(36)
        .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(24i32)) as i16));
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(16i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write((-16i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(128i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(24i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(24i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((taskId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId2) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId1) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(0i16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(2i16);
        if !((crate::c::bf_read(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read()).cast::<*mut u8>())
                .read())
            .wrapping_offset(
                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize * 4,
            ))
            .wrapping_add(0),
            0,
            1,
            false,
        ) as u16)
            != 0)
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId1) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId1) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(1),
            2,
            2,
            (0u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(1),
            2,
            2,
            (0u32) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId2) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(MoveOdorSleuthClone));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId1) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(MoveOdorSleuthClone));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_OdorSleuthMovementWaitFinish));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_OdorSleuthMovementWaitFinish(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn MoveOdorSleuthClone(sprite: *mut u8) {
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
            if !((crate::c::bf_read(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(0),
                0,
                1,
                false,
            ) as u16)
                != 0)
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
        ));
        'l1: {
            let __sw4 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw4 == 0i32 {
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 60i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw4 == 1i32 {
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                    (__p10).write((((((__p10).read()) as i32).wrapping_sub(2i32)) as i16));
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        < 0i32
                    {
                        let __p11 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32) as isize
                                * 40,
                        ))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32) as isize,
                        );
                        (__p11).write(((__p11).read()).wrapping_sub(1));
                        DestroySpriteWithActiveSheet(sprite);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetReturnPowerLevel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(0i16);
        if ((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) < 60i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        }
        if (((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) > 60i32)
            && (((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) < 92i32)
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        }
        if (((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) > 91i32)
            && (((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) < 201i32)
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(2i16);
        }
        if ((((&raw mut gAnimFriendship).cast::<u8>()).read()) as i32) > 200i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(3i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SnatchOpposingMonMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        let mut spriteId2: u8 = 0u8;
        let mut personality: i32 = 0i32;
        let mut otId: i32 = 0i32;
        let mut species: u16 = 0u16;
        let mut subpriority: u8 = 0u8;
        let mut isBackPic: u8 = 0u8;
        let mut x: i16 = 0i16;
        'l1: {
            let __sw1 = (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32);
            if __sw1 == 0i32 {
                spriteId = GetAnimBattlerSpriteId(0u8);
                let __p2 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(2048i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    let __p3 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                } else {
                    let __p4 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_sub(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                }
                let __p5 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p5).write((((((__p5).read()) as i32) & 255i32) as i16));
                x = ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16);
                if (((x) as i32) < (-32i32)) || (((x) as i32) > 272i32) {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p6 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (IsContest()) != 0 {
                    personality = ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(8)
                    .cast::<u32>())
                    .read()) as i32);
                    otId = ((((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(12)
                    .cast::<u32>())
                    .read()) as i32);
                    species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                        .wrapping_add(24)
                        .cast::<*mut u8>())
                    .read())
                    .cast::<u16>())
                    .read();
                    subpriority = GetBattlerSpriteSubpriority(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    );
                    isBackPic = 0u8;
                    x = (-32i16);
                } else {
                    if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                        as i32)
                        == 0i32
                    {
                        personality = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            0i32,
                        )) as i32);
                        otId = ((GetMonData2(
                            ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            1i32,
                        )) as i32);
                        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize
                                * 4,
                        ))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            == 0i32
                        {
                            species = ((GetMonData2(
                                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                11i32,
                            )) as u16);
                        } else {
                            species = (((((((&raw mut gBattleSpritesDataPtr)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 4,
                            ))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                        }
                        subpriority = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((GetAnimBattlerSpriteId(1u8)) as i32) as isize * 68,
                        ))
                        .wrapping_add(67))
                        .read()) as i32)
                            .wrapping_add(1i32)) as u8);
                        isBackPic = 0u8;
                        x = 272i16;
                    } else {
                        personality = ((GetMonData2(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            0i32,
                        )) as i32);
                        otId = ((GetMonData2(
                            ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                .read()) as i32) as isize
                                    * 100,
                            ),
                            1i32,
                        )) as i32);
                        if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize
                                * 4,
                        ))
                        .wrapping_add(2)
                        .cast::<u16>())
                        .read()) as i32)
                            == 0i32
                        {
                            species = ((GetMonData2(
                                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>())
                                        .cast::<u16>())
                                    .wrapping_offset(
                                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                            as i32)
                                            as isize,
                                    ))
                                    .read()) as i32) as isize
                                        * 100,
                                ),
                                11i32,
                            )) as u16);
                        } else {
                            species = (((((((&raw mut gBattleSpritesDataPtr)
                                .cast::<*mut u8>())
                            .read())
                            .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize
                                    * 4,
                            ))
                            .wrapping_add(2)
                            .cast::<u16>())
                            .read();
                        }
                        subpriority = ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((GetAnimBattlerSpriteId(1u8)) as i32) as isize * 68,
                        ))
                        .wrapping_add(67))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u8);
                        isBackPic = 1u8;
                        x = (-32i16);
                    }
                }
                spriteId2 = CreateAdditionalMonSpriteForMoveAnim(
                    species,
                    isBackPic,
                    0u8,
                    x,
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        1u8,
                    )) as i16),
                    subpriority,
                    ((personality) as u32),
                    ((otId) as u32),
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as u32),
                    0u32,
                );
                if (((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize * 4,
                ))
                .wrapping_add(2)
                .cast::<u16>())
                .read()) as i32)
                    != 0i32
                {
                    BlendPalette(
                        (((256i32).wrapping_add(
                            ((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_mul(16i32),
                        )) as u16),
                        16u16,
                        6u8,
                        32767u16,
                    );
                }
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .write(((spriteId2) as i16));
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                spriteId2 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as u8);
                let __p8 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p8).write((((((__p8).read()) as i32).wrapping_add(2048i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    let __p9 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId2) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p9).write(
                        (((((__p9).read()) as i32).wrapping_sub(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                } else {
                    let __p10 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId2) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p10).write(
                        (((((__p10).read()) as i32).wrapping_add(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                }
                let __p11 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p11).write((((((__p11).read()) as i32) & 255i32) as i16));
                x = ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId2) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId2) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16);
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(14))
                .read()) as i32)
                    == 0i32
                {
                    if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                        as i32)
                        == 0i32
                    {
                        if ((x) as i32)
                            < ((GetBattlerSpriteCoord(
                                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                                0u8,
                            )) as i32)
                        {
                            let __p12 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(14);
                            (__p12).write(((__p12).read()).wrapping_add(1));
                            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(7))
                            .write((-1i16));
                        }
                    } else {
                        if ((x) as i32)
                            > ((GetBattlerSpriteCoord(
                                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                                0u8,
                            )) as i32)
                        {
                            let __p13 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(14);
                            (__p13).write(((__p13).read()).wrapping_add(1));
                            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(7))
                            .write((-1i16));
                        }
                    }
                }
                if (((x) as i32) < (-32i32)) || (((x) as i32) > 272i32) {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p14 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                spriteId = GetAnimBattlerSpriteId(0u8);
                spriteId2 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as u8);
                DestroySpriteAndFreeResources_(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId2) as i32) as isize * 68),
                );
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(
                        (((((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(32)
                        .cast::<i16>())
                        .read()) as i32)
                            .wrapping_neg())
                        .wrapping_sub(32i32)) as i16),
                    );
                } else {
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(
                        (((272i32).wrapping_sub(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read()) as i32),
                        )) as i16),
                    );
                }
                let __p15 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                spriteId = GetAnimBattlerSpriteId(0u8);
                let __p16 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p16).write((((((__p16).read()) as i32).wrapping_add(2048i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    let __p17 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p17).write(
                        (((((__p17).read()) as i32).wrapping_add(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read()) as i32),
                        )
                        >= ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            0u8,
                        )) as i32)
                    {
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(0i16);
                    }
                } else {
                    let __p18 = (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>();
                    (__p18).write(
                        (((((__p18).read()) as i32).wrapping_sub(
                            (((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read()) as i32),
                        )
                        <= ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            0u8,
                        )) as i32)
                    {
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(0i16);
                    }
                }
                let __p19 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p19).write((((((__p19).read()) as i32) & 255i32) as i16));
                if ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedItemBagSteal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32);
            if __sw1 == 0i32 {
                if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(7))
                .read()) as i32)
                    == (-1i32)
                {
                    PlaySE12WithPanning(122u16, BattleAnimAdjustPanning(63i8));
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            1u8,
                        )) as i32)
                            .wrapping_add(16i32)) as i16),
                    );
                    (((sprite).wrapping_add(46)).cast::<i16>()).write((-32i16));
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                    (__p2).write(((__p2).read()).wrapping_add(1));
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                        as i32)
                        == 1i32)
                        && (!((IsContest()) != 0))
                    {
                        ((sprite).wrapping_add(67)).write(
                            ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((GetAnimBattlerSpriteId(1u8)) as i32) as isize * 68,
                            ))
                            .wrapping_add(67))
                            .read()) as i32)
                                .wrapping_sub(1i32)) as u8),
                        );
                    }
                } else {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ));
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(5i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 127i32
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(
                        ((crate::c::div_i32(
                            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                            2i32,
                        )) as i16),
                    );
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p5).write((((((__p5).read()) as i32).wrapping_sub(127i32)) as i16));
                }
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(256i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    let __p7 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p7).write(
                        (((((__p7).read()) as i32).wrapping_sub(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                } else {
                    let __p8 = (sprite).wrapping_add(36).cast::<i16>();
                    (__p8).write(
                        (((((__p8).read()) as i32).wrapping_add(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                >> 8),
                        )) as i16),
                    );
                }
                let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p9).write((((((__p9).read()) as i32) & 255i32) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    == 2i32
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SnatchPartnerMove(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut attackerX: i16 = 0i16;
        let mut targetX: i16 = 0i16;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                attackerX = ((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i16);
                targetX = ((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    0u8,
                )) as i16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(6i16);
                if ((attackerX) as i32) > ((targetX) as i32) {
                    let __p2 = ((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
                }
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(attackerX);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(targetX);
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read();
                let __p4 = (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    > 0i32
                {
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .read()) as i32),
                        )
                        >= ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                    {
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                } else {
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .read()) as i32),
                        )
                        <= ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .read()) as i32)
                    {
                        let __p6 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p7 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_mul((-1i32))) as i16));
                let __p8 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15);
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read();
                let __p9 = (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>();
                (__p9).write(
                    (((((__p9).read()) as i32).wrapping_add(
                        (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                if (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    < 0i32
                {
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .read()) as i32),
                        )
                        <= ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        let __p10 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p10).write(((__p10).read()).wrapping_add(1));
                    }
                } else {
                    if ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_add(
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .read()) as i32),
                        )
                        >= ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32)
                    {
                        let __p11 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(15);
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 || !__matched {
                spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read();
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TeeterDanceMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32
            {
                1i32
            } else {
                (-1i32)
            }) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_TeeterDanceMovement_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_TeeterDanceMovement_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(
                    ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 5) as i16),
                );
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(2i32)) as i16));
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                (__p5).write((((((__p5).read()) as i32) & 255i32) as i16));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .write(
                    ((((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 3)
                        .wrapping_mul(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                                as i32),
                        ))
                    .wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read()) as i32)
                    == 0i32
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read());
                    let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                (__p7).write((((((__p7).read()) as i32).wrapping_add(8i32)) as i16));
                let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                (__p8).write((((((__p8).read()) as i32) & 255i32) as i16));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(
                    ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32)
                        >> 5) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                    == 0i32
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimKnockOffStrike_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write((((((__p2).read()) as i32) & 255i32) as i16));
        } else {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p4).write((((((__p4).read()) as i32) & 255i32) as i16));
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            20i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            20i16,
        ));
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p5).write(((__p5).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AnimKnockOffStrike(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write((-11i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(192i16);
            StartSpriteAffineAnim(sprite, 1u8);
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(11i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(192i16);
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimKnockOffStrike_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRecycle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(GetBattlerSpriteCoordAttr(
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            2u8,
        ));
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 16i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(16i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(16i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRecycle_Step));
        SetGpuReg(
            82u8,
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                << 8)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimRecycle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 1i32
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 1i32)
                        != 0)
                    {
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                            .read()) as i32)
                            < 16i32
                        {
                            let __p4 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                            (__p4).write(((__p4).read()).wrapping_add(1));
                        }
                    } else {
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            != 0i32
                        {
                            let __p5 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                            (__p5).write(((__p5).read()).wrapping_sub(1));
                        }
                    }
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            << 8)
                            | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32)) as u16),
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        == 0i32
                    {
                        let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 10i32
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p11 = ((sprite).wrapping_add(46)).cast::<i16>();
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    > 1i32
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                    if !((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 1i32)
                        != 0)
                    {
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                            .read()) as i32)
                            != 0i32
                        {
                            let __p13 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                            (__p13).write(((__p13).read()).wrapping_sub(1));
                        }
                    } else {
                        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            < 16i32
                        {
                            let __p14 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                            (__p14).write(((__p14).read()).wrapping_add(1));
                        }
                    }
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p15).write(((__p15).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            << 8)
                            | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32)) as u16),
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        == 16i32
                    {
                        let __p16 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        (__p16).write(((__p16).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                DestroySpriteAndMatrix(sprite);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetWeather(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(0i16);
        if (((((&raw mut gWeatherMoveAnim).cast::<u16>()).read()) as i32) & 96i32) != 0 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        } else {
            if (((((&raw mut gWeatherMoveAnim).cast::<u16>()).read()) as i32) & 7i32) != 0 {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                    .write(2i16);
            } else {
                if (((((&raw mut gWeatherMoveAnim).cast::<u16>()).read()) as i32) & 24i32) != 0 {
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                        .write(3i16);
                } else {
                    if (((((&raw mut gWeatherMoveAnim).cast::<u16>()).read()) as i32) & 128i32) != 0
                    {
                        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(7))
                        .write(4i16);
                    }
                }
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SlackOffSquish(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        PrepareAffineAnimInTaskData(
            task,
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            ((&raw const gSlackOffSquishAffineAnimCmds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_SlackOffSquish_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SlackOffSquish_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            > 16i32)
            && ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                < 40i32)
        {
            if (({
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                let __t3 = ((__p2).read()).wrapping_add(1);
                (__p2).write(__t3);
                __t3
            }) as i32)
                > 2i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if !((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    & 1i32)
                    != 0)
                {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write((-1i16));
                } else {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(1i16);
                }
            }
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
        }
        if !((RunAffineAnimFromTaskData(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        )) != 0)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
