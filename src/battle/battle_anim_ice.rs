//! Translated from `src/battle_anim_ice.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_Unused sAnims_Unused sUnusedIceCrystalThrowSpriteTemplate sAnim_IceCrystalLargeChunk sAnim_IceCrystalLarge sAnim_IceCrystalSmall sAnim_Snowball sAnim_BlizzardIceCrystal sAnim_SmallBubblePair sAnims_IceCrystalLargeChunk sAnims_IceCrystalLarge sAnims_IceCrystalSmall sAnims_Snowball sAnims_BlizzardIceCrystal gAnims_SmallBubblePair sAffineAnim_IceCrystalSpiralInwardLarge sAffineAnims_IceCrystalSpiralInwardLarge gIceCrystalSpiralInwardLarge gIceCrystalSpiralInwardSmall sAffineAnim_IceBeamInnerCrystal sAffineAnims_IceBeamInnerCrystal gIceBeamInnerCrystalSpriteTemplate gIceBeamOuterCrystalSpriteTemplate sAffineAnim_IceCrystalHit sAffineAnims_IceCrystalHit gIceCrystalHitLargeSpriteTemplate gIceCrystalHitSmallSpriteTemplate gSwirlingSnowballSpriteTemplate gBlizzardIceCrystalSpriteTemplate gPowderSnowSnowballSpriteTemplate sAnim_IceGroundSpike sAnims_IceGroundSpike gIceGroundSpikeSpriteTemplate sAnim_Cloud sAnims_Cloud gMistCloudSpriteTemplate gSmogCloudSpriteTemplate sHazeBlendAmounts gMistBallSpriteTemplate sMistBlendAmounts gPoisonGasCloudSpriteTemplate sHailCoordData sAffineAnim_HailParticle_0 sAffineAnim_HailParticle_1 sAffineAnim_HailParticle_2 sAffineAnim_WeatherBallIceDown sAffineAnims_HailParticle sAffineAnims_WeatherBallIceDown gHailParticleSpriteTemplate gWeatherBallIceDownSpriteTemplate sAnim_IceBallChunk_0 sAnim_IceBallChunk_1 sAnims_IceBallChunk sAffineAnim_IceBallChunk_0 sAffineAnim_IceBallChunk_1 sAffineAnim_IceBallChunk_2 sAffineAnim_IceBallChunk_3 sAffineAnim_IceBallChunk_4 sAffineAnims_IceBallChunk gIceBallChunkSpriteTemplate gIceBallImpactShardSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_ice::*;

unsafe extern "C" {
    static mut gAnimDisableStructPtr: u8;
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimFogTilemap: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattlerPositions: u8;
    static mut gFogPalette: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gWeatherFogHorizontalTiles: u8;
    fn AnimFastTranslateLinear(a0: *mut u8) -> u8;
    fn AnimLoadCompressedBgTilemapHandleContest(a0: *mut u8, a1: *mut u8, a2: u32);
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ClearBattleAnimBg(a0: u32);
    fn ConvertPosDataToTranslateLinearData(a0: *mut u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitAnimFastLinearTranslationWithSpeed(a0: *mut u8);
    fn InitAnimFastLinearTranslationWithSpeedAndPos(a0: *mut u8);
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitAnimLinearTranslationWithSpeed(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LoadBgTiles(a0: u8, a1: *mut u8, a2: u16, a3: u16) -> u16;
    fn LoadPalette(a0: *mut u8, a1: u16, a2: u16);
    fn Random2() -> u16;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn TranslateAnimSpriteToTargetMonLocation(a0: *mut u8);
    fn TranslateSpriteInGrowingCircle(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimUnusedIceCrystalThrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut targetX: i16 = 0i16;
        let mut targetY: i16 = 0i16;
        let mut attackerX: i16 = 0i16;
        let mut attackerY: i16 = 0i16;
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(7i32)) as u16) as i32,
        );
        targetX = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
            as i16);
        targetY = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
            as i16);
        attackerX =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16);
        attackerY =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                .wrapping_add(((attackerX) as i32))) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                .wrapping_add(((targetX) as i32))) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as i32)
                .wrapping_add(((attackerY) as i32))) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .read()) as i32)
                .wrapping_add(((targetY) as i32))) as i16),
        );
        ConvertPosDataToTranslateLinearData(sprite);
        {
            'l1: loop {
                if !(((((targetX) as i32) >= (-32i32)) && (((targetX) as i32) <= 272i32))
                    && ((((targetY) as i32) >= (-32i32)) && (((targetY) as i32) <= 192i32)))
                {
                    break 'l1;
                }
                'l2: {}
                targetX = ((((targetX) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16);
                targetY = ((((targetY) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16);
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_neg()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_neg()) as i16),
        );
        {
            'l3: loop {
                if !(((((attackerX) as i32) >= (-32i32)) && (((attackerX) as i32) <= 272i32))
                    && ((((attackerY) as i32) >= (-32i32)) && (((attackerY) as i32) <= 192i32)))
                {
                    break 'l3;
                }
                'l4: {}
                attackerX = ((((attackerX) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16);
                attackerY = ((((attackerY) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16);
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(attackerX);
        ((sprite).wrapping_add(34).cast::<i16>()).write(attackerY);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(attackerX);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(targetX);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(attackerY);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(targetY);
        ConvertPosDataToTranslateLinearData(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimUnusedIceCrystalThrow_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedIceCrystalThrow_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read());
            ((sprite).wrapping_add(38).cast::<i16>())
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                    )) as i32),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                    )) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )
                    & 255i32) as i16),
            );
            let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_sub(1));
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimIcePunchSwirlingParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(60i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(9i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(30i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-512i16));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteInGrowingCircle));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimIceBeamParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
    }
}
pub(crate) unsafe extern "C" fn AnimIceEffectParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            InitSpritePosToAnimTarget(sprite, 1u8);
        } else {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                1u8,
                (sprite).wrapping_add(32).cast::<i16>(),
                (sprite).wrapping_add(34).cast::<i16>(),
            );
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                    (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                        .wrapping_neg()) as i16),
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
        }
        StoreSpriteCallbackInData6(sprite, Some(AnimFlickerIceEffectParticle));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimFlickerIceEffectParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32) ^ 1i32)
                as u16) as i32,
        );
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(1i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 20i32 {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut tempDataHolder = crate::ffi::Align4([0u8; 16]);
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
            .read())
            != 0)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32),
                    )) as i16),
            );
        } else {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                1u8,
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2),
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4),
            );
        }
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        }
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut tempDataHolder).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(
                            ((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        InitAnimFastLinearTranslationWithSpeed(sprite);
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p3).write((((((__p3).read()) as i32) ^ 1i32) as i16));
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p4).write((((((__p4).read()) as i32) ^ 1i32) as i16));
        'l3: loop {
            if !((1i32) != 0) {
                break 'l3;
            }
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
            AnimFastTranslateLinear(sprite);
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
                break 'l3;
            }
        }
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
        {
            i = 0i32;
            'l4: loop {
                if !(i < 8i32) {
                    break 'l4;
                }
                'l5: {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(
                            (((&raw mut tempDataHolder).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(InitAnimFastLinearTranslationWithSpeedAndPos));
        StoreSpriteCallbackInData6(sprite, Some(AnimSwirlingSnowball_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut tempVar: i16 = 0i16;
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(128i16);
        tempVar = ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
            as i32)
            != 0i32
        {
            20i32
        } else {
            (-20i32)
        }) as i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            tempVar,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            15i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSwirlingSnowball_Step2));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut tempVar: i16 = 0i16;
        tempVar = ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
            as i32)
            != 0i32
        {
            20i32
        } else {
            (-20i32)
        }) as i16);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            <= 31i32
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((Sin((((sprite).wrapping_add(46)).cast::<i16>()).read(), tempVar)) as i32)
                    .wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((Cos((((sprite).wrapping_add(46)).cast::<i16>()).read(), 15i16)) as i32)
                    .wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(16i32)
                    & 255i32) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(1i32)) as i16));
        } else {
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
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimSwirlingSnowball_End));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingSnowball_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        AnimFastTranslateLinear(sprite);
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
            > 256i32)
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                < (-16i32)))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                > 256i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                < (-16i32))
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMoveParticleBeyondTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: i32 = 0i32;
        let mut tempDataHolder = crate::ffi::Align4([0u8; 16]);
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        if !((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read())
            != 0)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        } else {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                1u8,
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2),
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4),
            );
        }
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
        }
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        InitAnimFastLinearTranslationWithSpeed(sprite);
        {
            i = 0i32;
            'l1: loop {
                if !(i < 8i32) {
                    break 'l1;
                }
                'l2: {
                    (((&raw mut tempDataHolder).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(
                            ((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p4).write((((((__p4).read()) as i32) ^ 1i32) as i16));
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p5).write((((((__p5).read()) as i32) ^ 1i32) as i16));
        'l3: loop {
            if !((1i32) != 0) {
                break 'l3;
            }
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
            AnimFastTranslateLinear(sprite);
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
                break 'l3;
            }
        }
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
        ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
        {
            i = 0i32;
            'l4: loop {
                if !(i < 8i32) {
                    break 'l4;
                }
                'l5: {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset((i) as isize))
                        .write(
                            (((&raw mut tempDataHolder).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWiggleParticleTowardsTarget));
    }
}
pub(crate) unsafe extern "C" fn AnimWiggleParticleTowardsTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        AnimFastTranslateLinear(sprite);
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        }
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )
                & 255i32) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 1i32 {
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
}
pub(crate) unsafe extern "C" fn AnimWaveFromCenterOfTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                == 0i32
            {
                InitSpritePosToAnimTarget(sprite, 0u8);
            } else {
                SetAverageBattlerPositions(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    0u8,
                    (sprite).wrapping_add(32).cast::<i16>(),
                    (sprite).wrapping_add(34).cast::<i16>(),
                );
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                        (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32)
                            .wrapping_neg()) as i16),
                    );
                }
                let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                (__p1).write(
                    (((((__p1).read()) as i32).wrapping_add(
                        (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32),
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
        } else {
            if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitSwirlingFogAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut tempVar: i16 = 0i16;
        let mut battler: u8 = 0u8;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read())
            as i32)
            == 0i32
        {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                .read()) as i32)
                == 0i32
            {
                InitSpritePosToAnimAttacker(sprite, 0u8);
            } else {
                SetAverageBattlerPositions(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                    (sprite).wrapping_add(32).cast::<i16>(),
                    (sprite).wrapping_add(34).cast::<i16>(),
                );
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    let __p1 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p1).write(
                        (((((__p1).read()) as i32).wrapping_sub(
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as i32),
                        )) as i16),
                    );
                } else {
                    let __p2 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p2).write(
                        (((((__p2).read()) as i32).wrapping_add(
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as i32),
                        )) as i16),
                    );
                }
                let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
                );
            }
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                .read()) as i32)
                == 0i32
            {
                InitSpritePosToAnimTarget(sprite, 0u8);
            } else {
                SetAverageBattlerPositions(
                    ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    0u8,
                    (sprite).wrapping_add(32).cast::<i16>(),
                    (sprite).wrapping_add(34).cast::<i16>(),
                );
                if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_sub(
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as i32),
                        )) as i16),
                    );
                } else {
                    let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p5).write(
                        (((((__p5).read()) as i32).wrapping_add(
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as i32),
                        )) as i16),
                    );
                }
                let __p6 = (sprite).wrapping_add(34).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
                );
            }
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(((battler) as i16));
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
            .read()) as i32)
            == 0i32)
            || (!((IsDoubleBattle()) != 0))
        {
            tempVar = 32i16;
        } else {
            tempVar = 64i16;
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(tempVar);
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p7 = (sprite).wrapping_add(34).cast::<i16>();
            (__p7).write((((((__p7).read()) as i32).wrapping_add(8i32)) as i16));
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(64i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSwirlingFogAnim));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSwirlingFogAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    )) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        (-6i16),
                    )) as i32),
                )) as i16),
            );
            if (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32)
                .wrapping_sub(64i32)) as u16) as i32)
                <= 127i32
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((GetBattlerSpriteBGPriority(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u8),
                    )) as u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((((GetBattlerSpriteBGPriority(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as u8),
                    )) as i32)
                        .wrapping_add(1i32)) as u16) as i32,
                );
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(3i32)
                    & 255i32) as i16),
            );
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HazeScrollingFog(taskId: u8) {
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
        LoadBgTiles(
            (((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read(),
            (&raw mut gWeatherFogHorizontalTiles).cast::<u8>(),
            2048u16,
            (((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read(),
        );
        AnimLoadCompressedBgTilemapHandleContest(
            (&raw mut animBg).cast::<u8>(),
            (((&raw mut gBattleAnimFogTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
        );
        LoadPalette(
            (((&raw mut gFogPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_HazeScrollingFog_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_HazeScrollingFog_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add((-1i32))) as u16));
        let __p2 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(0i32)) as u16));
        'l1: {
            let __sw3 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .read()) as i32);
            let mut __fall = false;
            if __sw3 == 0i32 {
                __fall = true;
                if (({
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(9);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(
                        ((((((&raw const sHazeBlendAmounts).cast::<u8>().cast_mut()).cast::<u8>())
                            .wrapping_offset(
                                ((((((((&raw mut gTasks).cast::<u8>())
                                    .wrapping_offset(((taskId) as i32) as isize * 40))
                                .wrapping_add(8))
                                .cast::<i16>())
                                .wrapping_offset(9))
                                .read()) as i32) as isize,
                            ))
                        .read()) as i16),
                    );
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
                        == 9i32
                    {
                        let __p7 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p7).write(((__p7).read()).wrapping_add(1));
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
            if __sw3 == 1i32 {
                __fall = true;
                if (({
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 81i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(9i16);
                    let __p10 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 2i32 {
                __fall = true;
                if (({
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p13 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p13).write(((__p13).read()).wrapping_sub(1));
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
                        let __p14 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p14).write(((__p14).read()).wrapping_add(1));
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
            if __sw3 == 3i32 {
                __fall = true;
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(1u32);
                ClearBattleAnimBg(2u32);
                let __p15 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p15).write(((__p15).read()).wrapping_add(1));
            }
            if __fall || __sw3 == 4i32 {
                __fall = true;
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimThrowMistBall(sprite: *mut u8) {
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateAnimSpriteToTargetMonLocation));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MistBallFog(taskId: u8) {
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
        LoadBgTiles(
            (((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read(),
            (&raw mut gWeatherFogHorizontalTiles).cast::<u8>(),
            2048u16,
            (((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read(),
        );
        AnimLoadCompressedBgTilemapHandleContest(
            (&raw mut animBg).cast::<u8>(),
            (((&raw mut gBattleAnimFogTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
        );
        LoadPalette(
            (((&raw mut gFogPalette).cast::<u16>()).cast::<u16>()).cast::<u8>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write((-1i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_MistBallFog_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MistBallFog_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32),
            )) as u16),
        );
        let __p2 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(0i32)) as u16));
        'l1: {
            let __sw3 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .read()) as i32);
            let mut __fall = false;
            if __sw3 == 0i32 {
                __fall = true;
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(9);
                (__p4).write((((((__p4).read()) as i32).wrapping_add(1i32)) as i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .write(
                    ((((((&raw const sMistBlendAmounts).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(9))
                            .read()) as i32) as isize,
                        ))
                    .read()) as i16),
                );
                SetGpuReg(
                    82u8,
                    ((((17i32).wrapping_sub(
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
                    == 5i32
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
                break 'l1;
            }
            if __sw3 == 1i32 {
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
                    == 81i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(5i16);
                    let __p8 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw3 == 2i32 {
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
                    (__p11).write((((((__p11).read()) as i32).wrapping_sub(1i32)) as i16));
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
            if __sw3 == 3i32 {
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
            if __fall || __sw3 == 4i32 {
                __fall = true;
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn InitPoisonGasCloudAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        if ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
            as i32)
            < ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write((-32768i16));
        }
        if ((((((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
        ))
        .read()) as i32)
            & 1i32)
            == 0i32
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                as i32)
                & 32768i32)
                != 0)
                && (((((((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32)
                    & 1i32)
                    == 0i32)
            {
                ((sprite).wrapping_add(67)).write(
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((GetAnimBattlerSpriteId(1u8)) as i32) as isize * 68))
                    .wrapping_add(67))
                    .read()) as i32)
                        .wrapping_add(1i32)) as u8),
                );
            }
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            != 0
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32),
                    )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(4))
                        .read()) as i32),
                    )) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(
                (((((__p1).read()) as i32)
                    | (((GetBattlerSpriteBGPriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as i32)
                        << 8)) as i16),
            );
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32),
                    )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(4))
                        .read()) as i32),
                    )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write(
                (((((__p2).read()) as i32)
                    | (((GetBattlerSpriteBGPriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as i32)
                        << 8)) as i16),
            );
        }
        if (IsContest()) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1i16);
            ((sprite).wrapping_add(67)).write(128u8);
        }
        InitAnimLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(MovePoisonGasCloud));
    }
}
pub(crate) unsafe extern "C" fn MovePoisonGasCloud(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut value: i32 = 0i32;
        'l1: {
            let __sw1 = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                as i32)
                & 255i32);
            if __sw1 == 0i32 {
                AnimTranslateLinear(sprite);
                value = ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read()) as i32);
                let __p2 = (sprite).wrapping_add(36).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add((value >> 4))) as i16));
                if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_sub(8i32) & 255i32) as i16),
                    );
                } else {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_add(8i32) & 255i32) as i16),
                    );
                }
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) <= 0i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(80i16);
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            0u8,
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                        .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                    let __p3 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32),
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                        .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                            .wrapping_add(29i32)) as i16),
                    );
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    if (IsContest()) != 0 {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                            .write(80i16);
                    } else {
                        if ((((((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            & 1i32)
                            != 0i32
                        {
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .write(204i16);
                        } else {
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .write(80i16);
                        }
                    }
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    value = ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32) as isize,
                        ))
                    .read()) as i32);
                    ((sprite).wrapping_add(36).cast::<i16>()).write(((value >> 3) as i16));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_add(2i32) & 255i32) as i16),
                    );
                    InitAnimLinearTranslation(sprite);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                AnimTranslateLinear(sprite);
                value = ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32) as isize,
                ))
                .read()) as i32);
                let __p5 = (sprite).wrapping_add(36).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add((value >> 3))) as i16));
                let __p6 = (sprite).wrapping_add(38).cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32)
                                .wrapping_add(64i32)) as isize,
                        ))
                        .read()) as i32)
                            .wrapping_mul((-3i32))
                            >> 8),
                    )) as i16),
                );
                if !((IsContest()) != 0) {
                    let mut var0: u16 = ((((((((sprite).wrapping_add(46)).cast::<i16>())
                        .wrapping_offset(5))
                    .read()) as i32)
                        .wrapping_sub(64i32)) as u16);
                    if ((var0) as i32) <= 127i32 {
                        crate::c::bf_write(
                            (sprite).wrapping_add(5),
                            2,
                            2,
                            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32)
                                >> 8) as u16) as i32,
                        );
                    } else {
                        crate::c::bf_write(
                            (sprite).wrapping_add(5),
                            2,
                            2,
                            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32)
                                >> 8)
                                .wrapping_add(1i32)) as u16) as i32,
                        );
                    }
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_add(4i32) & 255i32) as i16),
                    );
                } else {
                    let mut var0: u16 = ((((((((sprite).wrapping_add(46)).cast::<i16>())
                        .wrapping_offset(5))
                    .read()) as i32)
                        .wrapping_sub(64i32)) as u16);
                    if ((var0) as i32) <= 127i32 {
                        ((sprite).wrapping_add(67)).write(128u8);
                    } else {
                        ((sprite).wrapping_add(67)).write(140u8);
                    }
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            .wrapping_sub(4i32) & 255i32) as i16),
                    );
                }
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) <= 0i32 {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(768i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write({
                        let __p7 = (sprite).wrapping_add(32).cast::<i16>();
                        let __v8 = (((((__p7).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32),
                        )) as i16);
                        (__p7).write(__v8);
                        __v8
                    });
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write({
                        let __p9 = (sprite).wrapping_add(34).cast::<i16>();
                        let __v10 = (((((__p9).read()) as i32).wrapping_add(
                            ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32),
                        )) as i16);
                        (__p9).write(__v10);
                        __v10
                    });
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                            .wrapping_add(4i32)) as i16),
                    );
                    if (IsContest()) != 0 {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                            .write((-16i16));
                    } else {
                        if ((((((&raw mut gBattlerPositions).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32)
                            & 1i32)
                            != 0i32
                        {
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .write(256i16);
                        } else {
                            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                                .write((-16i16));
                        }
                    }
                    let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                    ((sprite).wrapping_add(36).cast::<i16>()).write({
                        let __v12 = 0i16;
                        ((sprite).wrapping_add(38).cast::<i16>()).write(__v12);
                        __v12
                    });
                    InitAnimLinearTranslationWithSpeed(sprite);
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (AnimTranslateLinear(sprite)) != 0 {
                    if ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) & 1u32)
                        != 0
                    {
                        FreeOamMatrix(
                            ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32)
                                as u8),
                        );
                        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                    }
                    DestroySprite(sprite);
                    let __p13 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
                    (__p13).write(((__p13).read()).wrapping_sub(1));
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Hail(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_Hail2));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Hail2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 2i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == 0i32
                {
                    if (GenerateHailParticle(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as u8),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as u8),
                        taskId,
                        1u8,
                    )) != 0
                    {
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                    if (({
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t7 = ((__p6).read()).wrapping_add(1);
                        (__p6).write(__t7);
                        __t7
                    }) as i32)
                        == ((crate::c::div_u32(12u32, 4u32)) as i32)
                    {
                        if (({
                            let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                            let __t9 = ((__p8).read()).wrapping_add(1);
                            (__p8).write(__t9);
                            __t9
                        }) as i32)
                            == ((crate::c::div_u32(40u32, 4u32)) as i32)
                        {
                            let __p10 = ((task).wrapping_add(8)).cast::<i16>();
                            (__p10).write(((__p10).read()).wrapping_add(1));
                        } else {
                            let __p11 = ((task).wrapping_add(8)).cast::<i16>();
                            (__p11).write(((__p11).read()).wrapping_sub(1));
                        }
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                    }
                } else {
                    let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    (__p12).write(((__p12).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GenerateHailParticle(
    hailStructId: u8,
    affineAnimNum: u8,
    taskId: u8,
    c: u8,
) -> u8 {
    unsafe {
        let mut hailStructId = hailStructId;
        let mut affineAnimNum = affineAnimNum;
        let mut taskId = taskId;
        let mut c = c;
        let mut id: u8 = 0u8;
        let mut battlerX: i16 = 0i16;
        let mut battlerY: i16 = 0i16;
        let mut spriteX: i16 = 0i16;
        let mut shouldSpawnImpactEffect: u8 = 0u8;
        let mut r#type: i8 = ((crate::c::bf_read(
            ((((&raw const sHailCoordData).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((hailStructId) as i32) as isize * 4))
            .wrapping_add(3),
            4,
            4,
            true,
        ) as i32) as i8);
        if ((r#type) as i32) != 2i32 {
            id = GetBattlerAtPosition(
                ((crate::c::bf_read(
                    ((((&raw const sHailCoordData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((hailStructId) as i32) as isize * 4))
                    .wrapping_add(2),
                    4,
                    8,
                    true,
                ) as i32) as u8),
            );
            if (IsBattlerSpriteVisible(id)) != 0 {
                shouldSpawnImpactEffect = 1u8;
                battlerX = ((GetBattlerSpriteCoord(id, 2u8)) as i16);
                battlerY = ((GetBattlerSpriteCoord(id, 3u8)) as i16);
                'l1: {
                    let __sw1 = ((r#type) as i32);
                    if __sw1 == 0i32 {
                        battlerX = ((((battlerX) as i32).wrapping_sub(crate::c::div_i32(
                            ((GetBattlerSpriteCoordAttr(id, 1u8)) as i32),
                            6i32,
                        ))) as i16);
                        battlerY = ((((battlerY) as i32).wrapping_sub(crate::c::div_i32(
                            ((GetBattlerSpriteCoordAttr(id, 0u8)) as i32),
                            6i32,
                        ))) as i16);
                        break 'l1;
                    }
                    if __sw1 == 1i32 {
                        battlerX = ((((battlerX) as i32).wrapping_add(crate::c::div_i32(
                            ((GetBattlerSpriteCoordAttr(id, 1u8)) as i32),
                            6i32,
                        ))) as i16);
                        battlerY = ((((battlerY) as i32).wrapping_add(crate::c::div_i32(
                            ((GetBattlerSpriteCoordAttr(id, 0u8)) as i32),
                            6i32,
                        ))) as i16);
                        break 'l1;
                    }
                }
            } else {
                battlerX = ((crate::c::bf_read(
                    ((((&raw const sHailCoordData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((hailStructId) as i32) as isize * 4))
                    .wrapping_add(0),
                    0,
                    10,
                    true,
                ) as i32) as i16);
                battlerY = ((crate::c::bf_read(
                    ((((&raw const sHailCoordData).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((hailStructId) as i32) as isize * 4))
                    .wrapping_add(1),
                    2,
                    10,
                    true,
                ) as i32) as i16);
            }
        } else {
            battlerX = ((crate::c::bf_read(
                ((((&raw const sHailCoordData).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((hailStructId) as i32) as isize * 4))
                .wrapping_add(0),
                0,
                10,
                true,
            ) as i32) as i16);
            battlerY = ((crate::c::bf_read(
                ((((&raw const sHailCoordData).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((hailStructId) as i32) as isize * 4))
                .wrapping_add(1),
                2,
                10,
                true,
            ) as i32) as i16);
        }
        spriteX = ((((battlerX) as i32).wrapping_sub(crate::c::div_i32(
            ((battlerY) as i32).wrapping_add(8i32),
            2i32,
        ))) as i16);
        id = CreateSprite(
            (&raw const gHailParticleSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            spriteX,
            (-8i16),
            18u8,
        );
        if ((id) as i32) == 64i32 {
            return 0u8;
        } else {
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68),
                affineAnimNum,
            );
            (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .write(((shouldSpawnImpactEffect) as i16));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(battlerX);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(battlerY);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((affineAnimNum) as i16));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((taskId) as i16));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((id) as i32) as isize * 68))
                .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((c) as i16));
            return 1u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHailBegin(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32))
            && (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32))
        {
            return;
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 1i32)
            && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 0i32)
        {
            spriteId = CreateSprite(
                (&raw const gIceCrystalHitLargeSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((sprite).wrapping_add(67)).read(),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(((spriteId) as i16));
            if ((spriteId) as i32) != 64i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimHailContinue));
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(7))
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
            }
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            DestroySprite(sprite);
        } else {
            let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize,
            );
            (__p3).write(((__p3).read()).wrapping_sub(1));
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHailContinue(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 20i32
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize,
            );
            (__p3).write(((__p3).read()).wrapping_sub(1));
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn InitIceBallAnim(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut animNum: u8 = (((((crate::c::bf_read(
            (((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(17),
            4,
            4,
            false,
        ) as u8) as i32)
            .wrapping_sub(
                ((crate::c::bf_read(
                    (((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(17),
                    0,
                    4,
                    false,
                ) as u8) as i32),
            ))
        .wrapping_sub(1i32)) as u8);
        if ((animNum) as i32) > 4i32 {
            animNum = 4u8;
        }
        StartSpriteAffineAnim(sprite, animNum);
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimThrowIceBall));
    }
}
pub(crate) unsafe extern "C" fn AnimThrowIceBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((TranslateAnimHorizontalArc(sprite)) != 0) {
            return;
        }
        StartSpriteAnim(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn InitIceBallParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut randA: i16 = 0i16;
        let mut randB: i16 = 0i16;
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(8i32)) as u16) as i32,
        );
        InitSpritePosToAnimTarget(sprite, 1u8);
        randA = (((((Random2()) as i32) & 255i32).wrapping_add(256i32)) as i16);
        randB = ((((Random2()) as i32) & 511i32) as i16);
        if ((randB) as i32) > 255i32 {
            randB = (((256i32).wrapping_sub(((randB) as i32))) as i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(randA);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(randB);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimIceBallParticle));
    }
}
pub(crate) unsafe extern "C" fn AnimIceBallParticle(sprite: *mut u8) {
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
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            & 1i32)
            != 0
        {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8)
                    .wrapping_neg()) as i16),
            );
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8) as i16),
            );
        }
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8) as i16),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 21i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetIceBallCounter(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut arg: u8 =
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8);
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .wrapping_offset(((arg) as i32) as isize))
        .write(
            (((((crate::c::bf_read(
                (((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(17),
                4,
                4,
                false,
            ) as u8) as i32)
                .wrapping_sub(
                    ((crate::c::bf_read(
                        (((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read())
                            .wrapping_add(17),
                        0,
                        4,
                        false,
                    ) as u8) as i32),
                ))
            .wrapping_sub(1i32)) as i16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
