//! Translated from `src/battle_anim_effects_2.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sCirclingFingerSpriteTemplate sAnim_BouncingMusicNote sAnims_BouncingMusicNote sBouncingMusicNoteSpriteTemplate sVibrateBattlerBackSpriteTemplate sMovingClampSpriteTemplate sAnim_SmallExplosion sAnims_SmallExplosion sAffineAnim_SmallExplosion sAffineAnims_SmallExplosion sSmallExplosionSpriteTemplate gKinesisZapEnergyAnimCmds gKinesisZapEnergyAnimTable gKinesisZapEnergySpriteTemplate gSwordsDanceBladeAffineAnimCmds gSwordsDanceBladeAffineAnimTable gSwordsDanceBladeSpriteTemplate gSonicBoomSpriteTemplate gAirWaveProjectileSpriteTemplate gGrowingRingAffineAnimCmds gWaterPulseRingAffineAnimCmds gGrowingRingAffineAnimTable gWaterPulseRingAffineAnimTable gSupersonicRingSpriteTemplate gScreechRingSpriteTemplate gMetalSoundSpriteTemplate gWaterPulseRingSpriteTemplate gEggThrowSpriteTemplate sVoidLinesSpriteTemplate gCoinAnimCmds gCoinAnimTable gFallingCoinAffineAnimCmds gFallingCoinAffineAnimTable gCoinThrowSpriteTemplate gFallingCoinSpriteTemplate gBulletSeedAffineAnimCmds gBulletSeedAffineAnimTable gBulletSeedSpriteTemplate gRazorWindTornadoAffineAnimCmds gRazorWindTornadoAffineAnimTable gRazorWindTornadoSpriteTemplate gViceGripAnimCmds1 gViceGripAnimCmds2 gViceGripAnimTable gViceGripSpriteTemplate gGuillotineAnimCmds1 gGuillotineAnimCmds2 gGuillotineAnimTable gGuillotineSpriteTemplate gSplashEffectAffineAnimCmds gGrowAndShrinkAffineAnimCmds gBreathPuffAnimCmds1 gBreathPuffAnimCmds2 gBreathPuffAnimTable gBreathPuffSpriteTemplate gAngerMarkAffineAnimCmds gAngerMarkAffineAnimTable gAngerMarkSpriteTemplate gThrashMoveMonAffineAnimCmds gPencilSpriteTemplate gSnoreZSpriteTemplate gExplosionAnimCmds gExplosionAnimTable gExplosionSpriteTemplate gSoftBoiledEggAffineAnimCmds1 gSoftBoiledEggAffineAnimCmds2 gSoftBoiledEggAffineAnimCmds3 gSoftBoiledEggAffineAnimTable gSoftBoiledEggSpriteTemplate gThinRingExpandingAffineAnimCmds1 gThinRingExpandingAffineAnimCmds2 gHyperVoiceRingAffineAnimCmds gThinRingExpandingAffineAnimTable gHyperVoiceRingAffineAnimTable gThinRingExpandingSpriteTemplate gThinRingShrinkingAffineAnimCmds gThinRingShrinkingAffineAnimTable gThinRingShrinkingSpriteTemplate gBlendThinRingExpandingSpriteTemplate gHyperVoiceRingSpriteTemplate gUproarRingSpriteTemplate gStretchAttackerAffineAnimCmds gSpeedDustAnimCmds gSpeedDustAnimTable gSpeedDustSpriteTemplate gSpeedDustPosTable gBellAnimCmds gBellAnimTable gBellSpriteTemplate sMusicNotePaletteTagsTable gHealBellMusicNoteSpriteTemplate gMagentaHeartSpriteTemplate sAffineAnims_StretchBattlerUp gRedHeartProjectileSpriteTemplate gRedHeartBurstSpriteTemplate gRedHeartRisingSpriteTemplate gHiddenPowerOrbAffineAnimCmds gHiddenPowerOrbAffineAnimTable gHiddenPowerOrbSpriteTemplate gHiddenPowerOrbScatterSpriteTemplate gSpitUpOrbAffineAnimCmds gSpitUpOrbAffineAnimTable gSpitUpOrbSpriteTemplate gEyeSparkleAnimCmds gEyeSparkleAnimTable gEyeSparkleSpriteTemplate gAngelSpriteAnimCmds gAngelSpriteAnimTable gAngelSpriteTemplate gPinkHeartSpriteTemplate gDevilAnimCmds1 gDevilAnimCmds2 gDevilAnimTable gDevilSpriteTemplate sAnim_FurySwipes sAnim_FurySwipes_Flipped sAnims_FurySwipes gFurySwipesSpriteTemplate gMovementWavesAnimCmds1 gMovementWavesAnimCmds2 gMovementWavesAnimTable gMovementWavesSpriteTemplate sAffineAnims_UproarDistortion gJaggedMusicNoteSpriteTemplate gPerishSongMusicNoteAffineAnimCmds1 gPerishSongMusicNoteAffineAnimCmds2 gPerishSongMusicNoteAffineAnimCmds3 gPerishSongMusicNoteAffineAnimTable gPerishSongMusicNoteSpriteTemplate gPerishSongMusicNote2SpriteTemplate gGuardRingAffineAnimCmds1 gGuardRingAffineAnimCmds2 gGuardRingAffineAnimTable gGuardRingSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_effects_2::*;

unsafe extern "C" {
    static mut gAnimDisableStructPtr: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimBgImage_Attract: u8;
    static mut gBattleAnimBgImage_ScaryFace: u8;
    static mut gBattleAnimBgPalette_Attract: u8;
    static mut gBattleAnimBgPalette_ScaryFace: u8;
    static mut gBattleAnimBgTilemap_Attract: u8;
    static mut gBattleAnimBgTilemap_ScaryFaceContest: u8;
    static mut gBattleAnimBgTilemap_ScaryFaceOpponent: u8;
    static mut gBattleAnimBgTilemap_ScaryFacePlayer: u8;
    static mut gBattleAnimSpritePal_MusicNotes2: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattlerPositions: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gMonSpritesGfxPtr: u8;
    static mut gPlttBufferFaded: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocOamMatrix() -> u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AllocZeroed(a0: u32) -> *mut u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemapHandleContest(a0: *mut u8, a1: *mut u8, a2: u32);
    fn AnimSpriteOnMonPos(a0: *mut u8);
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn ClearBattleAnimBg(a0: u32);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroySpriteWithActiveSheet(a0: *mut u8);
    fn Free(a0: *mut u8);
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitAnimLinearTranslationWithSpeedAndPos(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LZDecompressWram(a0: *mut u32, a1: *mut u8);
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn MathUtil_Inv16(a0: i16) -> i16;
    fn MathUtil_Mul16(a0: i16, a1: i16) -> i16;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareAffineAnimInTaskData(a0: *mut u8, a1: u8, a2: *mut u8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn Random2() -> u16;
    fn ResetSpriteRotScale(a0: u8);
    fn RunAffineAnimFromTaskData(a0: *mut u8) -> u8;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn SeekSpriteAnim(a0: *mut u8, a1: u8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAnimSpriteInitialXOffset(a0: *mut u8, a1: i16);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetBattlerSpriteYOffsetFromRotation(a0: u8);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGrayscaleOrOriginalPalette(a0: u16, a1: u8);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut u8);
    fn SetSpriteNextToMonHead(a0: u8, a1: *mut u8);
    fn SetSpritePrimaryCoordsFromSecondaryCoords(a0: *mut u8);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateSpriteInCircle(a0: *mut u8);
    fn TranslateSpriteInEllipse(a0: *mut u8);
    fn TranslateSpriteLinearFixedPoint(a0: *mut u8);
    fn TrySetSpriteRotScale(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: u16);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimCirclingFinger(sprite: *mut u8) {
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteInEllipse));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimBouncingMusicNote(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        SetSpriteNextToMonHead(battler, sprite);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBouncingMusicNote_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBouncingMusicNote_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(38).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(3i32)) as i16));
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 6i32
                {
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p6 = (sprite).wrapping_add(38).cast::<i16>();
                (__p6).write((((((__p6).read()) as i32).wrapping_add(3i32)) as i16));
                if (({
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t8 = ((__p7).read()).wrapping_sub(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == 0i32
                {
                    let __p9 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 64i32
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimVibrateBattlerBack_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut temp: i16 = 0i16;
        let __p1 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(36)
        .cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        temp = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read();
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((temp) as i32).wrapping_neg()) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            DestroySpriteAndMatrix(sprite);
        }
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write(((__p2).read()).wrapping_sub(1));
    }
}
pub(crate) unsafe extern "C" fn AnimVibrateBattlerBack(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        }
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(((spriteId) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimVibrateBattlerBack_Step));
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn AnimMovingClamp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(AnimMovingClamp_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimMovingClamp_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(15i32))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(AnimMovingClamp_End));
    }
}
pub(crate) unsafe extern "C" fn AnimMovingClamp_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 0i32
        {
            DestroyAnimSprite(sprite);
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Withdraw(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        PrepareBattlerSpriteForRotScale(
            (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read(),
            0u8,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_Withdraw_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Withdraw_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        let mut rotation: i16 = 0i16;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            rotation = (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                .wrapping_neg()) as i16);
        } else {
            rotation = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read();
        }
        SetSpriteRotScale(spriteId, 256i16, 256i16, ((rotation) as u16));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(176i32)) as i16));
            let __p2 = (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 1i32
            {
                if (({
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == 30i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(2i16);
                }
                return;
            } else {
                let __p5 = ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_sub(176i32)) as i16));
                let __p6 = (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_sub(1));
            }
        }
        SetBattlerSpriteYOffsetFromRotation(spriteId);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 3872i32)
            || ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
                == 0i32)
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 2i32
            {
                ResetSpriteRotScale(spriteId);
                DestroyAnimVisualTask(taskId);
            } else {
                let __p7 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1);
                (__p7).write(((__p7).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimKinesisZapEnergy(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        } else {
            let __p2 = (sprite).wrapping_add(32).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
        }
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read())
                != 0
            {
                crate::c::bf_write((sprite).wrapping_add(63), 1, 1, (1u16) as i32);
            }
        } else {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read())
                != 0
            {
                crate::c::bf_write((sprite).wrapping_add(63), 1, 1, (1u16) as i32);
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimSwordsDanceBlade(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 0u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(AnimSwordsDanceBlade_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSwordsDanceBlade_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(32i32))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimSonicBoomProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut targetXPos: i16 = 0i16;
        let mut targetYPos: i16 = 0i16;
        let mut rotation: u16 = 0u16;
        if (IsContest()) != 0 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
            }
        }
        InitSpritePosToAnimAttacker(sprite, 1u8);
        targetXPos =
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16);
        targetYPos =
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16);
        rotation = ArcTan2Neg(
            ((((targetXPos) as i32)
                .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                as i16),
            ((((targetYPos) as i32)
                .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                as i16),
        );
        rotation = ((((rotation) as i32).wrapping_sub(4096i32)) as u16);
        if (IsContest()) != 0 {
            rotation = ((((rotation) as i32).wrapping_sub(24576i32)) as u16);
        }
        TrySetSpriteRotScale(sprite, 0u8, 256i16, 256i16, rotation);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(targetXPos);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(targetYPos);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimAirWaveProjectile_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            <= 0i32
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimAirWaveProjectile_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                as isize
                * 40,
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            > ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
        } else {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p5).write(
            (((((__p5).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )) as i16),
        );
        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p6).write(
            (((((__p6).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        if (1i32 & ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32))
            != 0
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as u16) as i32)
                    >> 8)
                    .wrapping_mul((-1i32))) as i16),
            );
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as u16) as i32)
                    >> 8) as i16),
            );
        }
        if (1i32 & ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32))
            != 0
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as u16) as i32)
                    >> 8)
                    .wrapping_mul((-1i32))) as i16),
            );
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as u16) as i32)
                    >> 8) as i16),
            );
        }
        if (({
            let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t8 = (__p7).read();
            (__p7).write(((__p7).read()).wrapping_sub(1));
            __t8
        }) as i32)
            <= 0i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimAirWaveProjectile_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimAirWaveProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut a: i16 = 0i16;
        let mut b: i16 = 0i16;
        let mut c: i16 = 0i16;
        let mut task: *mut u8 = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                as isize
                * 40,
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((-2i32)
                    & ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((-2i32)
                    & ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)),
            )) as i16),
        );
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32) & 1i32)
            != 0
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as u16) as i32)
                    >> 8)
                    .wrapping_mul((-1i32))) as i16),
            );
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as u16) as i32)
                    >> 8) as i16),
            );
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32) & 1i32)
            != 0
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as u16) as i32)
                    >> 8)
                    .wrapping_mul((-1i32))) as i16),
            );
        } else {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as u16) as i32)
                    >> 8) as i16),
            );
        }
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            __t4
        }) as i32)
            <= 0i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(4i16);
            a = MathUtil_Inv16(4096i16);
            let __p5 = (sprite).wrapping_add(32).cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p6 = (sprite).wrapping_add(34).cast::<i16>();
            (__p6).write(
                (((((__p6).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                >= ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            {
                b = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                    as i32)
                    .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                    << 8) as i16);
            } else {
                b = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                        as i32),
                ) << 8) as i16);
            }
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
                >= ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            {
                c = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                    as i32)
                    .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32))
                    << 8) as i16);
            } else {
                c = ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                        as i32),
                ) << 8) as i16);
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(MathUtil_Mul16(
                MathUtil_Mul16(b, a),
                MathUtil_Inv16(((((1.75f32) as f32) * ((256i32) as f32)) as i16)),
            ));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(MathUtil_Mul16(
                MathUtil_Mul16(c, a),
                MathUtil_Inv16(((((1.75f32) as f32) * ((256i32) as f32)) as i16)),
            ));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimAirWaveProjectile_Step1));
        }
    }
}
pub(crate) unsafe extern "C" fn AirCutterProjectileStep2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 0i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AirCutterProjectileStep1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            <= 0i32
        {
            let mut spriteId: u8 = 0u8;
            let mut sprite: *mut u8 = core::ptr::null_mut();
            spriteId = CreateSprite(
                (&raw const gAirWaveProjectileSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9))
                .read(),
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read(),
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read()) as i32),
                    )) as u8),
            );
            sprite = ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68);
            'l1: {
                let __sw3 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32);
                if __sw3 == 1i32 {
                    crate::c::bf_write(
                        (sprite).wrapping_add(3),
                        1,
                        5,
                        ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) | 24u32)
                            as i32,
                    );
                    break 'l1;
                }
                if __sw3 == 2i32 {
                    crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
                    break 'l1;
                }
            }
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32)
                    .wrapping_sub(
                        ((((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(6))
                        .read()) as i32),
                    )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((taskId) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                (((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_add(13i32)) as isize,
            ))
            .write(((spriteId) as i16));
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read(),
            );
            let __p4 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p4).write(((__p4).read()).wrapping_add(1));
            PlaySE12WithPanning(154u16, BattleAnimAdjustPanning((-63i8)));
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                > 2i32
            {
                ((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AirCutterProjectileStep2));
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AirCutterProjectile(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut attackerY: i16 = 0i16;
        let mut attackerX: i16 = 0i16;
        let mut targetX: i16 = 0i16;
        let mut targetY: i16 = 0i16;
        let mut xDiff: i16 = 0i16;
        if (IsContest()) != 0 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(2i16);
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
            if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                & 1i32)
                != 0
            {
                let __p1 =
                    (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2);
                (__p1).write((((((__p1).read()) as i32) & (-2i32)) as i16));
            } else {
                let __p2 =
                    (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2);
                (__p2).write((((((__p2).read()) as i32) | 1i32) as i16));
            }
        } else {
            if ((((((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                & 1i32)
                == 0i32
            {
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(1i16);
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                    (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                        .wrapping_neg()) as i16),
                );
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
                if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(2))
                .read()) as i32)
                    & 1i32)
                    != 0
                {
                    let __p3 = (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2);
                    (__p3).write((((((__p3).read()) as i32) & (-2i32)) as i16));
                } else {
                    let __p4 = (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2);
                    (__p4).write((((((__p4).read()) as i32) | 1i32) as i16));
                }
            }
        }
        attackerX = {
            let __v5 =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(9))
            .write(__v5);
            __v5
        };
        attackerY = {
            let __v6 =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                    as i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(__v6);
            __v6
        };
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
            && ((IsBattlerSpriteVisible(
                ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
            )) != 0)
        {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                0u8,
                &raw mut targetX,
                &raw mut targetY,
            );
        } else {
            targetX =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i16);
            targetY =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i16);
        }
        targetX = {
            let __v7 = ((((targetX) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(__v7);
            __v7
        };
        targetY = {
            let __v8 = ((((targetY) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .write(__v8);
            __v8
        };
        if ((targetX) as i32) >= ((attackerX) as i32) {
            xDiff = ((((targetX) as i32).wrapping_sub(((attackerX) as i32))) as i16);
        } else {
            xDiff = ((((attackerX) as i32).wrapping_sub(((targetX) as i32))) as i16);
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(MathUtil_Mul16(
            xDiff,
            MathUtil_Inv16(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    & (-2i32)) as i16),
            ),
        ));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(MathUtil_Mul16(
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read(),
            ((((0.5f32) as f32) * ((256i32) as f32)) as i16),
        ));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        if ((targetY) as i32) >= ((attackerY) as i32) {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(
                ((((MathUtil_Mul16(
                    ((((targetY) as i32).wrapping_sub(((attackerY) as i32))) as i16),
                    MathUtil_Inv16(
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read(),
                    ),
                )) as i32)
                    & (-2i32)) as i16),
            );
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(8))
            .write(
                ((((MathUtil_Mul16(
                    ((((attackerY) as i32).wrapping_sub(((targetY) as i32))) as i16),
                    MathUtil_Inv16(
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .read(),
                    ),
                )) as i32)
                    | 1i32) as i16),
            );
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
            .read()) as i32)
            & 128i32)
            != 0
        {
            let __p9 =
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4);
            (__p9).write((((((__p9).read()) as i32) ^ 128i32) as i16));
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as i32)
                >= 64i32
            {
                let mut var: u16 = ((((GetBattlerSpriteSubpriority(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(4))
                        .read()) as i32)
                            .wrapping_sub(64i32),
                    )) as u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((var) as i16));
            } else {
                let mut var: u16 = ((((GetBattlerSpriteSubpriority(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                )) as i32)
                    .wrapping_sub(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(4))
                        .read()) as i32),
                    )) as u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((var) as i16));
            }
        } else {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as i32)
                >= 64i32
            {
                let mut var: u16 = ((((GetBattlerSpriteSubpriority(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(4))
                        .read()) as i32)
                            .wrapping_sub(64i32),
                    )) as u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((var) as i16));
            } else {
                let mut var: u16 = ((((GetBattlerSpriteSubpriority(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                )) as i32)
                    .wrapping_sub(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(4))
                        .read()) as i32),
                    )) as u16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((var) as i16));
            }
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            < 3i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(3i16);
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AirCutterProjectileStep1));
    }
}
pub(crate) unsafe extern "C" fn AnimVoidLines(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 0u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((256i32).wrapping_add(
                ((IndexOfSpritePaletteTag(
                    (((&raw const sVoidLinesSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut())
                    .wrapping_add(2)
                    .cast::<u16>())
                    .read(),
                )) as i32)
                    .wrapping_mul(16i32),
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimVoidLines_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimVoidLines_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut id: u16 = 0u16;
        let mut val: u16 = 0u16;
        let mut i: i32 = 0i32;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 2i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            id = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16);
            val = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((8i32).wrapping_add(((id) as i32))) as isize))
            .read();
            {
                i = 8i32;
                'l1: loop {
                    if !(i < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(((i).wrapping_add(((id) as i32))) as isize))
                        .write(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    (((i).wrapping_add(((id) as i32))).wrapping_add(1i32)) as isize,
                                ))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((id) as i32).wrapping_add(15i32)) as isize))
            .write(val);
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 24i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimCoinThrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r6: i16 = 0i16;
        let mut r7: i16 = 0i16;
        let mut var: u16 = 0u16;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        r6 = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
            as i16);
        r7 = ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
            as i32)
            .wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        r6 = ((((r6) as i32).wrapping_add(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32),
        )) as i16);
        var = ArcTan2Neg(
            ((((r6) as i32)
                .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                as i16),
            ((((r7) as i32)
                .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                as i16),
        );
        var = ((((var) as i32).wrapping_sub(16384i32)) as u16);
        TrySetSpriteRotScale(sprite, 0u8, 256i16, 256i16, var);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(r6);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(r7);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(InitAnimLinearTranslationWithSpeedAndPos));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimFallingCoin(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write((-16i16));
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFallingCoin_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFallingCoin_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(128i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>())
            .write((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(5i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 126i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((crate::c::div_i32((((__p3).read()) as i32), 2i32)) as i16));
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == 2i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBulletSeed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
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
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
        StoreSpriteCallbackInData6(sprite, Some(AnimBulletSeed_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimBulletSeed_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut rand: u16 = 0u16;
        let mut ptr: *mut i16 = core::ptr::null_mut();
        PlaySE12WithPanning(166u16, BattleAnimAdjustPanning(63i8));
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        ptr = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    ((ptr).wrapping_offset(((i).wrapping_sub(7i32)) as isize)).write(0i16);
                }
                i = (i).wrapping_add(1);
            }
        }
        rand = Random2();
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write((((65524i32).wrapping_sub((((rand) as i32) & 7i32))) as i16));
        rand = Random2();
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write((((crate::c::rem_i32(((rand) as i32), 160i32)).wrapping_add(160i32)) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBulletSeed_Step2));
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
    }
}
pub(crate) unsafe extern "C" fn AnimBulletSeed_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>())
            .write((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8) as i16));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            & 1i32)
            != 0
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
        ));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 126i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(((crate::c::div_i32((((__p3).read()) as i32), 2i32)) as i16));
            if (({
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                let __t5 = ((__p4).read()).wrapping_add(1);
                (__p4).write(__t5);
                __t5
            }) as i32)
                == 1i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRazorWindTornado(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 0u8);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(16i32)) as i16));
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteInCircle));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimViceGripPincer(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut startXOffset: i16 = 32i16;
        let mut startYOffset: i16 = (-32i16);
        let mut endXOffset: i16 = 16i16;
        let mut endYOffset: i16 = (-16i16);
        if ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0 {
            startXOffset = (-32i16);
            startYOffset = 32i16;
            endXOffset = (-16i16);
            endYOffset = 16i16;
            StartSpriteAnim(sprite, 1u8);
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((startXOffset) as i32))) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(((startYOffset) as i32))) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((endXOffset) as i32))) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((endYOffset) as i32))) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(AnimViceGripPincer_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimViceGripPincer_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGuillotinePincer(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut startXOffset: i16 = 32i16;
        let mut startYOffset: i16 = (-32i16);
        let mut endXOffset: i16 = 16i16;
        let mut endYOffset: i16 = (-16i16);
        if ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0 {
            startXOffset = (-32i16);
            startYOffset = 32i16;
            endXOffset = (-16i16);
            endYOffset = 16i16;
            StartSpriteAnim(
                sprite,
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            );
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((startXOffset) as i32))) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(((startYOffset) as i32))) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((endXOffset) as i32))) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((endYOffset) as i32))) as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimGuillotinePincer_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimGuillotinePincer_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((AnimTranslateLinear(sprite)) != 0)
            && ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
        {
            SeekSpriteAnim(sprite, 0u8);
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(2i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write((-2i16));
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write((((((__p3).read()) as i32) ^ 1i32) as i16));
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p4).write((((((__p4).read()) as i32) ^ 1i32) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimGuillotinePincer_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGuillotinePincer_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write((((((__p1).read()) as i32) ^ 1i32) as i16));
        if (({
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 51i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
            StartSpriteAnim(
                sprite,
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    ^ 1i32) as u8),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimGuillotinePincer_Step3));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGuillotinePincer_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GrowAndGrayscale(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
        PrepareBattlerSpriteForRotScale(spriteId, 1u8);
        SetSpriteRotScale(spriteId, 208i16, 208i16, 0u16);
        SetGrayscaleOrOriginalPalette(
            ((((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as i32)
                .wrapping_add(16i32)) as u16),
            0u8,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(80i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_GrowAndGrayscale_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_GrowAndGrayscale_Step(taskId: u8) {
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
            let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
            ResetSpriteRotScale(spriteId);
            SetGrayscaleOrOriginalPalette(
                ((((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(5),
                    4,
                    4,
                    false,
                ) as u16) as i32)
                    .wrapping_add(16i32)) as u16),
                1u8,
            );
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Minimize(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        (((task).wrapping_add(8)).cast::<i16>()).write(((spriteId) as i16));
        PrepareBattlerSpriteForRotScale(spriteId, 0u8);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(256i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i16),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_Minimize_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Minimize_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                if ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    == 0i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 3i32))
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 6i32)
                {
                    CreateMinimizeSprite(task, taskId);
                }
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(40i32)) as i16));
                SetSpriteRotScale(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                    0u16,
                );
                SetBattlerSpriteYOffsetFromYScale(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 32i32
                {
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    == 0i32
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        == 3i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(3i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(256i16);
                        SetSpriteRotScale(
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                            0u16,
                        );
                        SetBattlerSpriteYOffsetFromYScale(
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(2i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (({
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 32i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p9).write((((((__p9).read()) as i32).wrapping_add(2i32)) as i16));
                let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p10).write((((((__p10).read()) as i32).wrapping_sub(80i32)) as i16));
                SetSpriteRotScale(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                    0u16,
                );
                SetBattlerSpriteYOffsetFromYScale(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 32i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                ResetSpriteRotScale((((((task).wrapping_add(8)).cast::<i16>()).read()) as u8));
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(0i16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateMinimizeSprite(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut matrixNum: u16 = 0u16;
        let mut spriteId: i16 = CloneBattlerSpriteWithBlend(0u8);
        if ((spriteId) as i32) >= 0i32 {
            if (({
                let __v1 = ((AllocOamMatrix()) as u16);
                matrixNum = __v1;
                __v1
            }) as i32)
                == 255i32
            {
                DestroySpriteWithActiveSheet(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                );
            } else {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    2,
                    2,
                    (1u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    0,
                    2,
                    (3u32) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(44),
                    7,
                    1,
                    (1u8) as i32,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(3),
                    1,
                    5,
                    ((matrixNum) as u32) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(67))
                .write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                                as i32),
                        )) as u8),
                );
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p2).write(((__p2).read()).wrapping_add(1));
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p3).write(((__p3).read()).wrapping_add(1));
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(16i16);
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
                .write(6i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(ClonedMinizeSprite_Step));
                SetSpriteRotScale(
                    ((spriteId) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                    0u16,
                );
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(1),
                    0,
                    2,
                    (1u32) as i32,
                );
                CalcCenterToCornerVec(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(1),
                        6,
                        2,
                        false,
                    ) as u32) as u8),
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(3),
                        6,
                        2,
                        false,
                    ) as u32) as u8),
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(1),
                        0,
                        2,
                        false,
                    ) as u32) as u8),
                );
            }
        }
    }
}
pub(crate) unsafe extern "C" fn ClonedMinizeSprite_Step(sprite: *mut u8) {
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
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            DestroySpriteWithActiveSheet(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Splash(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read())
            as i32)
            == 0i32
        {
            DestroyAnimVisualTask(taskId);
        } else {
            let mut spriteId: u8 = GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            );
            (((task).wrapping_add(8)).cast::<i16>()).write(((spriteId) as i16));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            PrepareAffineAnimInTaskData(
                task,
                spriteId,
                ((&raw const gSplashEffectAffineAnimCmds)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_Splash_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Splash_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                RunAffineAnimFromTaskData(task);
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(3i32)) as i16));
                let __p3 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 7i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                RunAffineAnimFromTaskData(task);
                let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 7i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    != 0i32
                {
                    let __p11 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>();
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(2i32)) as i16));
                    let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p12).write((((((__p12).read()) as i32).wrapping_sub(2i32)) as i16));
                } else {
                    let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if !((RunAffineAnimFromTaskData(task)) != 0) {
                    if (({
                        let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t15 = ((__p14).read()).wrapping_sub(1);
                        (__p14).write(__t15);
                        __t15
                    }) as i32)
                        == 0i32
                    {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .write(0i16);
                        DestroyAnimVisualTask(taskId);
                    } else {
                        PrepareAffineAnimInTaskData(
                            task,
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
                            ((&raw const gSplashEffectAffineAnimCmds)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>(),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GrowAndShrink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        PrepareAffineAnimInTaskData(
            task,
            spriteId,
            ((&raw const gGrowAndShrinkAffineAnimCmds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_GrowAndShrink_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_GrowAndShrink_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((RunAffineAnimFromTaskData(task)) != 0) {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBreathPuff(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            StartSpriteAnim(sprite, 0u8);
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_add(32i32)) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(64i16);
        } else {
            StartSpriteAnim(sprite, 1u8);
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_sub(32i32)) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-64i16));
        }
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(52i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPoint));
    }
}
pub(crate) unsafe extern "C" fn AnimAngerMark(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        if !(((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0) {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            let __p1 =
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 2u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 3u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 8i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(8i16);
        }
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ThrashMoveMonHorizontal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        (((task).wrapping_add(8)).cast::<i16>()).write(((spriteId) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        PrepareAffineAnimInTaskData(
            task,
            spriteId,
            ((&raw const gThrashMoveMonAffineAnimCmds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ThrashMoveMonHorizontal_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ThrashMoveMonHorizontal_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((RunAffineAnimFromTaskData(task)) != 0) {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ThrashMoveMonVertical(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(4i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(7i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(3i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(2i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ThrashMoveMonVertical_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ThrashMoveMonVertical_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 2i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
            (__p3).write(((__p3).read()).wrapping_add(1));
            if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32)
                & 1i32)
                != 0
            {
                let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32),
                    )) as i16),
                );
            } else {
                let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32),
                    )) as i16),
                );
            }
        }
        'l1: {
            let __sw6 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32);
            if __sw6 == 0i32 {
                let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p7).write(
                    (((((__p7).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t9 = ((__p8).read()).wrapping_sub(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(14i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                }
                break 'l1;
            }
            if __sw6 == 1i32 {
                let __p10 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p10).write(
                    (((((__p10).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t12 = ((__p11).read()).wrapping_sub(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    == 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(7i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(2i16);
                }
                break 'l1;
            }
            if __sw6 == 2i32 {
                let __p13 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>();
                (__p13).write(
                    (((((__p13).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t15 = ((__p14).read()).wrapping_sub(1);
                    (__p14).write(__t15);
                    __t15
                }) as i32)
                    == 0i32
                {
                    if (({
                        let __p16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        let __t17 = ((__p16).read()).wrapping_sub(1);
                        (__p16).write(__t17);
                        __t17
                    }) as i32)
                        != 0i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(7i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    } else {
                        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)
                            & 1i32)
                            != 0i32
                        {
                            let __p18 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(34)
                            .cast::<i16>();
                            (__p18).write(
                                (((((__p18).read()) as i32).wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                        .read()) as i32),
                                )) as i16),
                            );
                        }
                        DestroyAnimVisualTask(taskId);
                    }
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SketchDrawMon(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut params = crate::ffi::Align4([0u8; 12]);
        let mut i: i16 = 0i16;
        (((task).wrapping_add(8)).cast::<i16>()).write(
            ((((GetBattlerYCoordWithElevation(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                as i32)
                .wrapping_add(32i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8),
        );
        if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
            as i32)
            == 1i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
            (((&raw mut params).cast::<u8>()).cast::<*mut u8>())
                .write(((67108884i32) as usize as *mut u16).cast::<u8>());
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                .write(((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16));
            (((&raw mut params).cast::<u8>()).cast::<*mut u8>())
                .write(((67108888i32) as usize as *mut u16).cast::<u8>());
        }
        {
            i = (((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32).wrapping_sub(64i32))
                as i16);
            'l1: loop {
                if !(((i) as i32) <= (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if ((i) as i32) >= 0i32 {
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32)
                                .wrapping_add(240i32)) as u16),
                        );
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i32)
                                .wrapping_add(240i32)) as u16),
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        (((&raw mut params).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(2724200449u32);
        (((&raw mut params).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut params).cast::<u8>()).wrapping_add(9)).write(0u8);
        ScanlineEffect_SetParams(
            (&raw mut params)
                .cast::<u8>()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_SketchDrawMon_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SketchDrawMon_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 20i32
                {
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 3i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            & 3i32) as i16),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        (((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32).wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                                as i32),
                        )) as i16),
                    );
                    'l2: {
                        let __sw7 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                            .read()) as i32);
                        if __sw7 == 0i32 {
                            break 'l2;
                        }
                        if __sw7 == 1i32 {
                            let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                            (__p8).write((((((__p8).read()) as i32).wrapping_sub(2i32)) as i16));
                            break 'l2;
                        }
                        if __sw7 == 2i32 {
                            let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                            (__p9).write((((((__p9).read()) as i32).wrapping_add(1i32)) as i16));
                            break 'l2;
                        }
                        if __sw7 == 3i32 {
                            let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                            (__p10).write((((((__p10).read()) as i32).wrapping_add(1i32)) as i16));
                            break 'l2;
                        }
                    }
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        >= 0i32
                    {
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32) as isize,
                            ))
                        .write(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                                as u16),
                        );
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                                as i32) as isize,
                        ))
                        .write(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                                as u16),
                        );
                    }
                    if (({
                        let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        let __t12 = ((__p11).read()).wrapping_add(1);
                        (__p11).write(__t12);
                        __t12
                    }) as i32)
                        >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32)
                    {
                        (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                        DestroyAnimVisualTask(taskId);
                    }
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPencil(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_sub(16i32)) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerYCoordWithElevation(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                as i32)
                .wrapping_add(16i32)) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(16i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(2i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((BattleAnimAdjustPanning(63i8)) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimPencil_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimPencil_Step(sprite: *mut u8) {
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
                    > 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    crate::c::bf_write(
                        (sprite).wrapping_add(62),
                        2,
                        1,
                        ((!((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16)
                            != 0)) as u16) as i32,
                    );
                }
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 16i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((({
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    > 3i32)
                    && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32))
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p9 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p9).write((((((__p9).read()) as i32).wrapping_sub(1i32)) as i16));
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    if crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                        10i32,
                    ) == 0i32
                    {
                        PlaySE12WithPanning(
                            205u16,
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                .read()) as i8),
                        );
                    }
                }
                let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p11).write(
                    (((((__p11).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    > 31i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        (((64i32).wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32),
                        )) as i16),
                    );
                    let __p12 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p12).write((((((__p12).read()) as i32).wrapping_mul((-1i32))) as i16));
                } else {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        <= (-32i32)
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                            (((-64i32).wrapping_sub(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
                                    .read()) as i32),
                            )) as i16),
                        );
                        let __p13 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        (__p13).write((((((__p13).read()) as i32).wrapping_mul((-1i32))) as i16));
                    }
                }
                ((sprite).wrapping_add(36).cast::<i16>())
                    .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read());
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p14 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t16 = ((__p15).read()).wrapping_add(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    > 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    crate::c::bf_write(
                        (sprite).wrapping_add(62),
                        2,
                        1,
                        ((!((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16)
                            != 0)) as u16) as i32,
                    );
                }
                if (({
                    let __p17 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t18 = ((__p17).read()).wrapping_add(1);
                    (__p17).write(__t18);
                    __t18
                }) as i32)
                    > 16i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBlendThinRing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut r4: u8 = 0u8;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        r4 = ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
            .read()) as i32)
            ^ 1i32) as u8);
        if ((IsDoubleBattle()) != 0)
            && ((IsBattlerSpriteVisible(((((battler) as i32) ^ 2i32) as u8))) != 0)
        {
            SetAverageBattlerPositions(battler, r4, &raw mut x, &raw mut y);
            if ((r4) as i32) == 0i32 {
                r4 = GetBattlerSpriteCoord(battler, 0u8);
            } else {
                r4 = GetBattlerSpriteCoord(battler, 2u8);
            }
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                let __p1 = ((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>();
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_sub(
                        (((x) as i32).wrapping_sub(((r4) as i32))).wrapping_sub(
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as i32),
                        ),
                    )) as i16),
                );
            } else {
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .write(((((x) as i32).wrapping_sub(((r4) as i32))) as i16));
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSpriteOnMonPos));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimHyperVoiceRing_WaitEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            FreeSpriteOamMatrix(sprite);
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHyperVoiceRing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut startX: i16 = 0i16;
        let mut startY: i16 = 0i16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        let mut yCoordType: u8 = 0u8;
        let mut battler1: u8 = 0u8;
        let mut battler2: u8 = 0u8;
        let mut xCoordType: u8 = 0u8;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read())
            as i32)
            == 0i32
        {
            battler1 = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
            battler2 = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        } else {
            battler1 = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
            battler2 = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        }
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
            .read())
            != 0)
        {
            xCoordType = 0u8;
            yCoordType = 1u8;
        } else {
            xCoordType = 2u8;
            yCoordType = 3u8;
        }
        if ((GetBattlerSide(battler1)) as i32) != 0i32 {
            startX = ((((GetBattlerSpriteCoord(battler1, xCoordType)) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16);
            if (IsBattlerSpriteVisible(((((battler2) as i32) ^ 2i32) as u8))) != 0 {
                ((sprite).wrapping_add(67)).write(
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                            .wrapping_offset((((battler2) as i32) ^ 2i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(67))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
            } else {
                ((sprite).wrapping_add(67)).write(
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                            .wrapping_offset(((battler2) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(67))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
            }
        } else {
            startX = ((((GetBattlerSpriteCoord(battler1, xCoordType)) as i32).wrapping_sub(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16);
            if (!((IsContest()) != 0))
                && ((IsBattlerSpriteVisible(((((battler1) as i32) ^ 2i32) as u8))) != 0)
            {
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler1) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(32)
                .cast::<i16>())
                .read()) as i32)
                    < ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                            .wrapping_offset((((battler1) as i32) ^ 2i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .read()) as i32)
                {
                    ((sprite).wrapping_add(67)).write(
                        ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                .wrapping_offset((((battler1) as i32) ^ 2i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(67))
                        .read()) as i32)
                            .wrapping_add(1i32)) as u8),
                    );
                } else {
                    ((sprite).wrapping_add(67)).write(
                        ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                .wrapping_offset(((battler1) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(67))
                        .read()) as i32)
                            .wrapping_sub(1i32)) as u8),
                    );
                }
            } else {
                ((sprite).wrapping_add(67)).write(
                    ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                            .wrapping_offset(((battler1) as i32) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(67))
                    .read()) as i32)
                        .wrapping_sub(1i32)) as u8),
                );
            }
        }
        startY = ((((GetBattlerSpriteCoord(battler1, yCoordType)) as i32).wrapping_add(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32),
        )) as i16);
        if (!((IsContest()) != 0))
            && ((IsBattlerSpriteVisible(((((battler2) as i32) ^ 2i32) as u8))) != 0)
        {
            SetAverageBattlerPositions(
                battler2,
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
                    .read()) as u8),
                &raw mut x,
                &raw mut y,
            );
        } else {
            x = ((GetBattlerSpriteCoord(battler2, xCoordType)) as i16);
            y = ((GetBattlerSpriteCoord(battler2, yCoordType)) as i16);
        }
        if (GetBattlerSide(battler2)) != 0 {
            x = ((((x) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16);
        } else {
            x = ((((x) as i32).wrapping_sub(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16);
        }
        y = ((((y) as i32).wrapping_add(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as i32),
        )) as i16);
        ((sprite).wrapping_add(32).cast::<i16>()).write({
            let __v1 = startX;
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(__v1);
            __v1
        });
        ((sprite).wrapping_add(34).cast::<i16>()).write({
            let __v2 = startY;
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(__v2);
            __v2
        });
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(x);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(y);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        InitAnimLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimHyperVoiceRing_WaitEnd));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimUproarRing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index: u8 = IndexOfSpritePaletteTag(10203u16);
        if ((index) as i32) != 255i32 {
            BlendPalette(
                ((((256i32).wrapping_add(((index) as i32).wrapping_mul(16i32))).wrapping_add(1i32))
                    as u16),
                15u16,
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                    .read()) as u8),
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .read()) as u16),
            );
        }
        StartSpriteAffineAnim(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSpriteOnMonPos));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r1: i16 = 0i16;
        InitSpritePosToAnimAttacker(sprite, 0u8);
        r1 = ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
            != 0i32
        {
            (-160i32)
        } else {
            160i32
        }) as i16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(896i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(r1);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSoftBoiledEgg_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut add: i16 = 0i16;
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_sub(((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 8)))
                as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >> 8) as i16),
        );
        let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(32i32)) as i16));
        add = ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
            != 0i32
        {
            (-160i32)
        } else {
            160i32
        }) as i16);
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p3).write((((((__p3).read()) as i32).wrapping_add(((add) as i32))) as i16));
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 0i32 {
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p5 = (sprite).wrapping_add(32).cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            StartSpriteAffineAnim(sprite, 1u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimSoftBoiledEgg_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 19i32
        {
            StartSpriteAffineAnim(sprite, 2u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimSoftBoiledEgg_Step3));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            StartSpriteAffineAnim(sprite, 1u8);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 0i32
            {
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
                .write(Some(AnimSoftBoiledEgg_Step3_Callback1));
            } else {
                crate::c::bf_write(
                    (sprite).wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                        .wrapping_add(32i32)) as u16) as i32,
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimSoftBoiledEgg_Step4));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step3_Callback1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_sub(2i32)) as i16));
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 9i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            SetGpuReg(80u8, 16192u16);
            SetGpuReg(
                82u8,
                ((0i32 | ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16) as i32))
                    as u16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimSoftBoiledEgg_Step3_Callback2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step3_Callback2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if crate::c::rem_i32(
            (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t2 = (__p1).read();
                (__p1).write(((__p1).read()).wrapping_add(1));
                __t2
            }) as i32),
            3i32,
        ) == 0i32
        {
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            SetGpuReg(
                82u8,
                ((((16i32)
                    .wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                    << 8)
                    | (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                    as u16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimSoftBoiledEgg_Step4));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step4(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 0i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimSoftBoiledEgg_Step4_Callback));
            } else {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(DestroyAnimSprite));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSoftBoiledEgg_Step4_Callback(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AttackerStretchAndDisappear(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        (((task).wrapping_add(8)).cast::<i16>()).write(((spriteId) as i16));
        PrepareAffineAnimInTaskData(
            task,
            spriteId,
            ((&raw const gStretchAttackerAffineAnimCmds)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_AttackerStretchAndDisappear_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AttackerStretchAndDisappear_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if !((RunAffineAnimFromTaskData(task)) != 0) {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ExtremeSpeedImpact(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(3i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write((-1i16));
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(8i16);
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(1i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write((-8i16));
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(1u8)) as i16));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ExtremeSpeedImpact_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ExtremeSpeedImpact_Step(taskId: u8) {
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
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32),
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>();
                        (__p7).write((((((__p7).read()) as i32).wrapping_add(6i32)) as i16));
                    } else {
                        let __p8 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>();
                        (__p8).write((((((__p8).read()) as i32).wrapping_sub(6i32)) as i16));
                    }
                    if (({
                        let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        let __t10 = ((__p9).read()).wrapping_add(1);
                        (__p9).write(__t10);
                        __t10
                    }) as i32)
                        > 4i32
                    {
                        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 1i32)
                            != 0
                        {
                            let __p11 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                                    .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>();
                            (__p11).write((((((__p11).read()) as i32).wrapping_sub(6i32)) as i16));
                        }
                        let __p12 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p12).write(((__p12).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                    let __t14 = ((__p13).read()).wrapping_sub(1);
                    (__p13).write(__t14);
                    __t14
                }) as i32)
                    != 0i32
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                } else {
                    let __p15 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p15).write(((__p15).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p16 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p16).write(
                    (((((__p16).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i32),
                    )) as i16),
                );
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ExtremeSpeedMonReappear(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(14i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(2i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ExtremeSpeedMonReappear_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ExtremeSpeedMonReappear_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if ((((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) == 0i32)
            && ((({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32))
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            if ((({
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                & 1i32)
                != 0
            {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
            }
            if (({
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                let __t6 = ((__p5).read()).wrapping_add(1);
                (__p5).write(__t6);
                __t6
            }) as i32)
                >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
            {
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SpeedDust(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(4i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i16),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_SpeedDust_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SpeedDust_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_add(1i32)
                            & 1i32) as i16),
                    );
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        > 20i32
                    {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            == 0i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                .write(0i16);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                                .write(1i16);
                        } else {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                                .write(2i16);
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                if (({
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 20i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(1i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                break 'l1;
            }
        }
        'l2: {
            let __sw8 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw8 == 0i32 {
                if (({
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > 4i32
                {
                    let mut spriteId: u8 = 0u8;
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    spriteId = CreateSprite(
                        (&raw const gSpeedDustSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                        0u8,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((taskId) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(13i16);
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(
                            (((((((&raw const gSpeedDustPosTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32) as isize
                                    * 2,
                            ))
                            .cast::<i8>())
                            .read()) as i16),
                        );
                        ((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .write(
                            ((((((((&raw const gSpeedDustPosTable).cast::<u8>().cast_mut())
                                .cast::<u8>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32) as isize
                                    * 2,
                            ))
                            .cast::<i8>())
                            .wrapping_offset(1))
                            .read()) as i16),
                        );
                        let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13);
                        (__p11).write(((__p11).read()).wrapping_add(1));
                        if (({
                            let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                            let __t13 = ((__p12).read()).wrapping_add(1);
                            (__p12).write(__t13);
                            __t13
                        }) as i32)
                            > 3i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .write(0i16);
                            if (({
                                let __p14 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                                let __t15 = ((__p14).read()).wrapping_add(1);
                                (__p14).write(__t15);
                                __t15
                            }) as i32)
                                > 5i32
                            {
                                let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                                (__p16).write(((__p16).read()).wrapping_add(1));
                            }
                        }
                    }
                }
                break 'l2;
            }
            if __sw8 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l2;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpeedDust(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u16) as i32,
        );
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_LoadMusicNotesPals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut paletteNums = crate::ffi::Align4([0u8; 3]);
        ((&raw mut paletteNums).cast::<u8>()).write(IndexOfSpritePaletteTag(10206u16));
        {
            i = 1i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut paletteNums).cast::<u8>()).wrapping_offset((i) as isize))
                        .write(AllocSpritePalette((((10000i32).wrapping_sub(i)) as u16)));
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
            .wrapping_add(380)
            .cast::<*mut u16>())
        .write(
            (AllocZeroed((((crate::c::div_i32(4096i32, 2i32)).wrapping_mul(4i32)) as u32)))
                .cast::<u16>(),
        );
        LZDecompressWram(
            ((&raw mut gBattleAnimSpritePal_MusicNotes2).cast::<u32>()).cast::<u32>(),
            (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .read())
            .cast::<u8>(),
        );
        {
            i = 0i32;
            'l3: loop {
                if !(i < 3i32) {
                    break 'l3;
                }
                'l4: {
                    LoadPalette(
                        ((((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                            .wrapping_add(380)
                            .cast::<*mut u16>())
                        .read())
                        .wrapping_offset(((i).wrapping_mul(32i32)) as isize))
                        .cast::<u8>(),
                        (((256i32).wrapping_add(
                            (((((&raw mut paletteNums).cast::<u8>()).wrapping_offset((i) as isize))
                                .read()) as i32)
                                .wrapping_mul(16i32),
                        )) as u16),
                        32u16,
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            Free(
                (((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                    .wrapping_add(380)
                    .cast::<*mut u16>())
                .read())
                .cast::<u8>(),
            );
            ((((&raw mut gMonSpritesGfxPtr).cast::<*mut u8>()).read())
                .wrapping_add(380)
                .cast::<*mut u16>())
            .write(core::ptr::null_mut());
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FreeMusicNotesPals(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        {
            i = 0i32;
            'l1: loop {
                if !(i < 3i32) {
                    break 'l1;
                }
                'l2: {
                    FreeSpritePaletteByTag(
                        ((((&raw const sMusicNotePaletteTagsTable)
                            .cast::<u8>()
                            .cast_mut()
                            .cast::<u16>())
                        .cast::<u16>())
                        .wrapping_offset((i) as isize))
                        .read(),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn SetMusicNotePalette(sprite: *mut u8, a: u8, b: u8) {
    unsafe {
        let mut sprite = sprite;
        let mut a = a;
        let mut b = b;
        let mut tile: u8 = ((if (((b) as i32) & 1i32) != 0 {
            32i32
        } else {
            0i32
        }) as u8);
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(((tile) as i32).wrapping_add(((a) as i32).wrapping_mul(4i32))))
                as u16) as i32,
        );
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            4,
            4,
            ((IndexOfSpritePaletteTag(
                ((((&raw const sMusicNotePaletteTagsTable)
                    .cast::<u8>()
                    .cast_mut()
                    .cast::<u16>())
                .cast::<u16>())
                .wrapping_offset((((b) as i32) >> 1) as isize))
                .read(),
            )) as u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn AnimHealBellMusicNote(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 0u8);
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
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        SetMusicNotePalette(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                .read()) as u8),
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
                .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimMagentaHeart(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            InitSpritePosToAnimAttacker(sprite, 0u8);
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            8i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 8) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(7i32)
                & 255i32) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write((((((__p3).read()) as i32).wrapping_sub(128i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 60i32 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FakeOut(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut win0h: u16 = ((if (IsContest()) != 0 { 152i32 } else { 240i32 }) as u16);
        let mut win0v: u16 = 0u16;
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(win0h);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(160u16);
        SetGpuReg(64u8, ((&raw mut gBattle_WIN0H).cast::<u16>()).read());
        SetGpuReg(68u8, ((&raw mut gBattle_WIN0V).cast::<u16>()).read());
        SetGpuReg(72u8, 16159u16);
        SetGpuReg(74u8, 16191u16);
        SetGpuReg(80u8, 200u16);
        SetGpuReg(84u8, 16u16);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((win0v) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((win0h) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_FakeOut_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FakeOut_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(13i32)) as i16));
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(13i32)) as i16));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            >= ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_FakeOut_Step2));
        } else {
            ((&raw mut gBattle_WIN0H).cast::<u16>()).write(
                ((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
                    << 8)
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FakeOut_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 5i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .write(136i16);
            SetGpuReg(80u8, 136u16);
            BlendPalettes(
                GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8),
                16u8,
                32767u16,
            );
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32)
                > 4i32
            {
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                SetGpuReg(72u8, 16191u16);
                SetGpuReg(74u8, 16191u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(84u8, 0u16);
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StretchTargetUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(1u8);
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                GetAnimBattlerSpriteId(1u8),
                ((&raw const sAffineAnims_StretchBattlerUp)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(4i16);
        } else {
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
            if !((RunAffineAnimFromTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StretchAttackerUp(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 1i32
        {
            PrepareAffineAnimInTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
                GetAnimBattlerSpriteId(0u8),
                ((&raw const sAffineAnims_StretchBattlerUp)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(4i16);
        } else {
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
            if !((RunAffineAnimFromTaskData(
                ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            )) != 0)
            {
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
pub(crate) unsafe extern "C" fn AnimRedHeartProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(95i16);
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRedHeartProjectile_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRedHeartProjectile_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        14i16,
                    )) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(4i32)
                    & 255i32) as i16),
            );
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimParticleBurst(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            ));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    .wrapping_add(3i32)
                    & 255i32) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 100i32
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                        2i32,
                    )) as u16) as i32,
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 120i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRedHeartRising(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>()).write(160i16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(AnimRedHeartRising_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRedHeartRising_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut y: i16 = 0i16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u16)
                as i32)
                >> 8)
                .wrapping_neg()) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
            4i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                .wrapping_add(3i32)
                & 255i32) as i16),
        );
        y = ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
            as i16);
        if ((y) as i32) <= 72i32 {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                    2i32,
                )) as u16) as i32,
            );
            if ((y) as i32) <= 64i32 {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HeartsBackground(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        SetAnimBgAttribute(1u8, 4u8, 3u8);
        SetAnimBgAttribute(1u8, 0u8, 0u8);
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 3u8, 1u8);
        }
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gBattleAnimBgImage_Attract).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        AnimLoadCompressedBgTilemapHandleContest(
            (&raw mut animBg).cast::<u8>(),
            (((&raw mut gBattleAnimBgTilemap_Attract).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
        );
        LoadCompressedPalette(
            ((&raw mut gBattleAnimBgPalette_Attract).cast::<u32>()).cast::<u32>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_HeartsBackground_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_HeartsBackground_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32)
                        == 16i32
                    {
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 141i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(16i16);
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32)
                        == 0i32
                    {
                        let __p12 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p12).write(((__p12).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                );
                let __p13 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ScaryFace(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        SetAnimBgAttribute(1u8, 4u8, 1u8);
        SetAnimBgAttribute(1u8, 0u8, 0u8);
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 3u8, 1u8);
        }
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        if (IsContest()) != 0 {
            AnimLoadCompressedBgTilemapHandleContest(
                (&raw mut animBg).cast::<u8>(),
                (((&raw mut gBattleAnimBgTilemap_ScaryFaceContest).cast::<u32>()).cast::<u32>())
                    .cast::<u8>(),
                0u32,
            );
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 1i32
            {
                AnimLoadCompressedBgTilemapHandleContest(
                    (&raw mut animBg).cast::<u8>(),
                    (((&raw mut gBattleAnimBgTilemap_ScaryFacePlayer).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                    0u32,
                );
            } else {
                AnimLoadCompressedBgTilemapHandleContest(
                    (&raw mut animBg).cast::<u8>(),
                    (((&raw mut gBattleAnimBgTilemap_ScaryFaceOpponent).cast::<u32>())
                        .cast::<u32>())
                    .cast::<u8>(),
                    0u32,
                );
            }
        }
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gBattleAnimBgImage_ScaryFace).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        LoadCompressedPalette(
            ((&raw mut gBattleAnimBgPalette_ScaryFace).cast::<u32>()).cast::<u32>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_ScaryFace_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ScaryFace_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if (({
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 2i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32)
                        == 14i32
                    {
                        let __p5 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if (({
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    == 21i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(14i16);
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 2i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32)
                        == 0i32
                    {
                        let __p12 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p12).write(((__p12).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(1u32);
                ClearBattleAnimBg(2u32);
                let __p13 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p13).write(((__p13).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 4i32 {
                __fall = true;
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimOrbitFast(sprite: *mut u8) {
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
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimOrbitFast_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimOrbitFast_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            >= 64i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                <= 191i32)
        {
            ((sprite).wrapping_add(67)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_add(1i32)) as u8),
            );
        } else {
            ((sprite).wrapping_add(67)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_sub(1i32)) as u8),
            );
        }
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
        'l1: {
            let __sw1 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32);
            if __sw1 == 1i32 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(1024i32)) as i16));
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(256i32)) as i16));
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(2i16);
                    return;
                }
                break 'l1;
            }
            if __sw1 == 0i32 {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(1024i32)) as i16));
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p7).write((((((__p7).read()) as i32).wrapping_add(256i32)) as i16));
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                }
                break 'l1;
            }
        }
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimOrbitScatter(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(Sin(
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
            10i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(Cos(
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
            7i16,
        ));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimOrbitScatter_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimOrbitScatter_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
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
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpitUpOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_add(1));
            __t4
        }) as i32)
            >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpitUpOrb(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(Sin(
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
            10i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(Cos(
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
            7i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSpitUpOrb_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimEyeSparkle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimEyeSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimEyeSparkle_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimAngel(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var0: i16 = 0i16;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
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
        }
        let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p3).write(((__p3).read()).wrapping_add(1));
        var0 = (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(10i32)
            & 255i32) as i16);
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((((Sin(var0, 80i16)) as i32) >> 8) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 80i32 {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                (((crate::c::div_i32(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    2i32,
                ))
                .wrapping_add((((Cos(var0, 80i16)) as i32) >> 8))) as i16),
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 90i32 {
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p4).write(((__p4).read()).wrapping_add(1));
            let __p5 = (sprite).wrapping_add(36).cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_sub(crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    2i32,
                ))) as i16),
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 100i32 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPinkHeart_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
            5i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                2i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                .wrapping_add(3i32)
                & 255i32) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > 20i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                    2i32,
                )) as u16) as i32,
            );
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > 30i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPinkHeart(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            ));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    .wrapping_add(3i32)
                    & 255i32) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 70i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimPinkHeart_Step));
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
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                    .write(((crate::c::rem_i32(((Random2()) as i32), 180i32)) as i16));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDevil(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 0i32
        {
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
            StartSpriteAnim(sprite, 0u8);
            ((sprite).wrapping_add(67)).write(((((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))).wrapping_sub(1i32)) as u8));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        }
        let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((crate::c::rem_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(4i32),
                256i32,
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32) < 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            (((30i32).wrapping_sub(crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                4i32,
            ))) as i16),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            (((10i32).wrapping_sub(crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                8i32,
            ))) as i16),
        ));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > 128i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 0i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write((-1i16));
        }
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                < 0i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(1i16);
        }
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p4).write(((__p4).read()).wrapping_add(1));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            < 10i32)
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 80i32)
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
        } else {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            > 90i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFurySwipes(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
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
            StartSpriteAnim(
                sprite,
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as u8),
            );
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
        } else {
            if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMovementWaves(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
            .read())
            != 0)
        {
            DestroyAnimSprite(sprite);
        } else {
            if !(((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0) {
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
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        3u8,
                    )) as i16),
                );
            }
            if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read())
                != 0)
            {
                let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_add(32i32)) as i16));
            } else {
                let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_sub(32i32)) as i16));
            }
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            StartSpriteAnim(
                sprite,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimMovementWaves_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMovementWaves_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            if ({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) != 0
            {
                StartSpriteAnim(
                    sprite,
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u8),
                );
            } else {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_UproarDistortion(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
        );
        PrepareAffineAnimInTaskData(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
            spriteId,
            ((&raw const sAffineAnims_UproarDistortion)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_UproarDistortion_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_UproarDistortion_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunAffineAnimFromTaskData(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        )) != 0)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimJaggedMusicNote(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 =
            ((if !(((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0) {
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
            } else {
                ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
            }) as u8);
        if ((GetBattlerSide(battler)) as i32) == 1i32 {
            let __p1 =
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 2u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 3u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 3) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 3) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((crate::c::div_i32(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    << 3),
                8i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((crate::c::div_i32(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    << 3),
                8i32,
            )) as i16),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16) as i32,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimJaggedMusicNote_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimJaggedMusicNote_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >> 3) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 3) as i16),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 16i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPerishSongMusicNote2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                (((120i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            SetGrayscaleOrOriginalPalette(
                ((((crate::c::bf_read((sprite).wrapping_add(5), 4, 4, false) as u16) as i32)
                    .wrapping_add(16i32)) as u16),
                0u8,
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(80i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPerishSongMusicNote(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index: u16 = 0u16;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            ((sprite).wrapping_add(32).cast::<i16>()).write(120i16);
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                (((crate::c::div_i32(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                    2i32,
                ))
                .wrapping_sub(15i32)) as i16),
            );
            StartSpriteAnim(
                sprite,
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as u8),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(120i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                2i32,
            )) as i16),
        );
        index = ((((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            .wrapping_mul(3i32))
        .wrapping_add(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
        ) & 255i32) as u16);
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(10i32)) as i16));
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(((index) as i16), 100i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(((Sin(((index) as i16), 10i16)) as i32)))
            .wrapping_add(
                ((Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    4i16,
                )) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimPerishSongMusicNote_Step1));
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            SetSpritePrimaryCoordsFromSecondaryCoords(sprite);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(5i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            StartSpriteAffineAnim(sprite, 1u8);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPerishSongMusicNote_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 10i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimPerishSongMusicNote_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPerishSongMusicNote_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read());
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            > 48i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                > 0i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    .wrapping_sub(5i32)) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(((__p3).read()).wrapping_add(1));
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32) > 3i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((crate::c::rem_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                    2i32,
                )) as u16) as i32,
            );
            DestroyAnimSprite(sprite);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 4i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGuardRing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 1u32) != 0)
            && ((IsBattlerSpriteVisible(
                ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
            )) != 0)
        {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                0u8,
                (sprite).wrapping_add(32).cast::<i16>(),
                (sprite).wrapping_add(34).cast::<i16>(),
            );
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(40i32)) as i16));
            StartSpriteAffineAnim(sprite, 1u8);
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(40i32)) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(13i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(72i32))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsFuryCutterHitRight(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(
            ((((((((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i32)
                & 1i32) as i16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetFuryCutterHitCount(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(
            ((((((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(16))
                .read()) as i16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
