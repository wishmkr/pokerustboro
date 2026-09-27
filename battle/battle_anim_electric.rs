//! Translated from `src/battle_anim_electric.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_Lightning sAnims_Lightning gLightningSpriteTemplate sAffineAnim_UnusedSpinningFist sAffineAnims_UnusedSpinningFist sUnusedSpinningFistSpriteTemplate sAnim_UnusedCirclingShock sAnims_UnusedCirclingShock sUnusedCirclingShockSpriteTemplate gSparkElectricitySpriteTemplate gZapCannonBallSpriteTemplate sAffineAnim_FlashingSpark sAffineAnims_FlashingSpark gZapCannonSparkSpriteTemplate sAnim_ThunderboltOrb sAnims_ThunderboltOrb sAffineAnim_ThunderboltOrb sAffineAnims_ThunderboltOrb gThunderboltOrbSpriteTemplate gSparkElectricityFlashingSpriteTemplate gElectricitySpriteTemplate gElectricBoltSegmentSpriteTemplate gThunderWaveSpriteTemplate sElectricChargingParticleCoordOffsets sAnim_ElectricChargingParticles_0 sAnim_ElectricChargingParticles_1 sAnims_ElectricChargingParticles gElectricChargingParticlesSpriteTemplate sAffineAnim_GrowingElectricOrb_0 sAffineAnim_GrowingElectricOrb_1 sAffineAnim_GrowingElectricOrb_2 sAffineAnims_GrowingElectricOrb gGrowingChargeOrbSpriteTemplate sAnim_ElectricPuff sAnims_ElectricPuff gElectricPuffSpriteTemplate gVoltTackleOrbSlideSpriteTemplate sAnim_VoltTackleBolt_0 sAnim_VoltTackleBolt_1 sAnim_VoltTackleBolt_2 sAnim_VoltTackleBolt_3 sAnims_VoltTackleBolt sAffineAnim_VoltTackleBolt sAffineAnims_VoltTackleBolt gVoltTackleBoltSpriteTemplate gGrowingShockWaveOrbSpriteTemplate gShockWaveProgressingBoltSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_electric::*;

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gOamMatrices: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimSpriteAfterTimer(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateSpriteInCircle(a0: *mut u8);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimLightning(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimLightning_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimLightning_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedSpinningFist(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimUnusedSpinningFist_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedSpinningFist_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedCirclingShock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_sub(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
            );
        } else {
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteInCircle));
    }
}
pub(crate) unsafe extern "C" fn AnimSparkElectricity(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        let mut matrixNum: u32 = 0u32;
        let mut sineVal: i16 = 0i16;
        'l1: {
            let __sw1 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                .wrapping_offset(4))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 1i32 || !__matched {
                battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
                break 'l1;
            }
            if __sw1 == 2i32 {
                if !((IsBattlerSpriteVisible(
                    ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                        as u8),
                )) != 0)
                {
                    battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
                } else {
                    battler = ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                        ^ 2i32) as u8);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if (IsBattlerSpriteVisible(
                    ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                        as u8),
                )) != 0
                {
                    battler = ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                        ^ 2i32) as u8);
                } else {
                    battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
                }
                break 'l1;
            }
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read())
            as i32)
            == 0i32
        {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((GetBattlerSpriteCoord(battler, 0u8)) as i16));
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((GetBattlerSpriteCoord(battler, 1u8)) as i16));
        } else {
            ((sprite).wrapping_add(32).cast::<i16>())
                .write(((GetBattlerSpriteCoord(battler, 2u8)) as i16));
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((GetBattlerSpriteCoord(battler, 3u8)) as i16));
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    as isize,
            ))
            .read()) as i32)
                .wrapping_mul(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )
                >> 8) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_add(64i32)) as isize,
            ))
            .read()) as i32)
                .wrapping_mul(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )
                >> 8) as i16),
        );
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
            .read()) as i32)
            & 1i32)
            != 0
        {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((GetBattlerSpriteBGPriority(battler)) as i32).wrapping_add(1i32)) as u16) as i32,
            );
        }
        matrixNum = (crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32);
        sineVal = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32) as isize,
        ))
        .read();
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .cast::<i16>())
        .write({
            let __v2 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_add(64i32)) as isize,
            ))
            .read();
            ((((&raw mut gOamMatrices).cast::<u8>())
                .wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<i16>())
            .write(__v2);
            __v2
        });
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(2)
        .cast::<i16>())
        .write(sineVal);
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i16>())
        .write(((((sineVal) as i32).wrapping_neg()) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(DestroyAnimSpriteAfterTimer));
    }
}
pub(crate) unsafe extern "C" fn AnimZapCannonSpark(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(6))
                    .read()) as i32)
                        .wrapping_mul(4i32),
                )) as u16) as i32,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimZapCannonSpark_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimZapCannonSpark_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    )) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    )) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )
                    & 255i32) as i16),
            );
            if !((crate::c::rem_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
                3i32,
            )) != 0)
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
            }
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimThunderboltOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == (-1i32)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read());
        }
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_sub(1));
            __t4
        }) as i32)
            <= 0i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimThunderboltOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((IsContest()) != 0)
            || (((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                == 0i32)
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimThunderboltOrb_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSparkElectricityFlashing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as i32)
            & 32768i32)
            != 0
        {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        }
        if ((IsContest()) != 0) || (((GetBattlerSide(battler)) as i32) == 0i32) {
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 2u8)) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 3u8)) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .read()) as i32)
                & 32767i32) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(6))
                    .read()) as i32)
                        .wrapping_mul(4i32),
                )) as u16) as i32,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSparkElectricityFlashing_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimSparkElectricityFlashing_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )
                & 255i32) as i16),
        );
        if crate::c::rem_i32(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
        ) == 0i32
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
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            __t2
        }) as i32)
            <= 0i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimElectricity(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 0u8);
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32)
                        .wrapping_mul(4i32),
                )) as u16) as i32,
        );
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            == 1i32
        {
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
        } else {
            if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .read()) as i32)
                == 2i32
            {
                crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (16u32) as i32);
            }
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ElectricBolt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i32)
                .wrapping_add(
                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
                )) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32),
                )) as i16),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_ElectricBolt_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ElectricBolt_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut r8: u16 = 0u16;
        let mut r2: u16 = 0u16;
        let mut r12: i16 = 0i16;
        let mut spriteId: u8 = 0u8;
        let mut r7: u8 = 0u8;
        let mut sp: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        let mut x: i16 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read();
        let mut y: i16 = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        if !((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read())
            != 0)
        {
            r8 = 0u16;
            r2 = 1u16;
            r12 = 16i16;
        } else {
            r12 = 16i16;
            r8 = 8u16;
            r2 = 4u16;
        }
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as i32);
            if __sw1 == 0i32 {
                r12 = ((((r12) as i32).wrapping_mul(1i32)) as i16);
                spriteId = CreateSprite(
                    (&raw const gElectricBoltSegmentSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    x,
                    ((((y) as i32).wrapping_add(((r12) as i32))) as i16),
                    2u8,
                );
                r7 = (r7).wrapping_add(1);
                break 'l1;
            }
            if __sw1 == 2i32 {
                r12 = ((((r12) as i32).wrapping_mul(2i32)) as i16);
                r8 = ((((r8) as i32).wrapping_add(((r2) as i32))) as u16);
                spriteId = CreateSprite(
                    (&raw const gElectricBoltSegmentSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    x,
                    ((((y) as i32).wrapping_add(((r12) as i32))) as i16),
                    2u8,
                );
                r7 = (r7).wrapping_add(1);
                break 'l1;
            }
            if __sw1 == 4i32 {
                r12 = ((((r12) as i32).wrapping_mul(3i32)) as i16);
                r8 = ((((r8) as i32).wrapping_add(((r2) as i32).wrapping_mul(2i32))) as u16);
                spriteId = CreateSprite(
                    (&raw const gElectricBoltSegmentSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    x,
                    ((((y) as i32).wrapping_add(((r12) as i32))) as i16),
                    2u8,
                );
                r7 = (r7).wrapping_add(1);
                break 'l1;
            }
            if __sw1 == 6i32 {
                r12 = ((((r12) as i32).wrapping_mul(4i32)) as i16);
                r8 = ((((r8) as i32).wrapping_add(((r2) as i32).wrapping_mul(3i32))) as u16);
                spriteId = CreateSprite(
                    (&raw const gElectricBoltSegmentSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    x,
                    ((((y) as i32).wrapping_add(((r12) as i32))) as i16),
                    2u8,
                );
                r7 = (r7).wrapping_add(1);
                break 'l1;
            }
            if __sw1 == 8i32 {
                r12 = ((((r12) as i32).wrapping_mul(5i32)) as i16);
                spriteId = CreateSprite(
                    (&raw const gElectricBoltSegmentSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    x,
                    ((((y) as i32).wrapping_add(((r12) as i32))) as i16),
                    2u8,
                );
                r7 = (r7).wrapping_add(1);
                break 'l1;
            }
            if __sw1 == 10i32 {
                DestroyAnimVisualTask(taskId);
                return;
            }
        }
        if (r7) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                ((((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    false,
                ) as u16) as i32)
                    .wrapping_add(((r8) as i32))) as u16) as i32,
            );
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(((sp) as i16));
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read())
            .unwrap_unchecked()(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        }
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AnimElectricBoltSegment(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            crate::c::bf_write((sprite).wrapping_add(1), 6, 2, (2u32) as i32);
            crate::c::bf_write((sprite).wrapping_add(3), 6, 2, (0u32) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(1), 6, 2, (0u32) as i32);
            crate::c::bf_write((sprite).wrapping_add(3), 6, 2, (1u32) as i32);
        }
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 15i32
        {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimThunderWave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
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
        spriteId = CreateSprite(
            (&raw const gThunderWaveSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(32i32))
                as i16),
            ((sprite).wrapping_add(34).cast::<i16>()).read(),
            ((sprite).wrapping_add(67)).read(),
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                false,
            ) as u16) as i32)
                .wrapping_add(8i32)) as u16) as i32,
        );
        let __p3 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
        (__p3).write(((__p3).read()).wrapping_add(1));
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimThunderWave_Step));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimThunderWave_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimThunderWave_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 3i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        }
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 51i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ElectricChargingParticles(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ElectricChargingParticles_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ElectricChargingParticles_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
            if (({
                let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
            {
                let mut spriteId: u8 = 0u8;
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(0i16);
                spriteId = CreateSprite(
                    (&raw const gElectricChargingParticlesSpriteTemplate)
                        .cast::<u8>()
                        .cast_mut(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                    2u8,
                );
                if ((spriteId) as i32) != 64i32 {
                    let mut sprite: *mut u8 = ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68);
                    let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                    (__p3).write(
                        (((((__p3).read()) as i32).wrapping_add(
                            (((((((&raw const sElectricChargingParticleCoordOffsets)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                    .read()) as i32) as isize
                                    * 2,
                            ))
                            .cast::<i8>())
                            .read()) as i32),
                        )) as i16),
                    );
                    let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p4).write(
                        (((((__p4).read()) as i32).wrapping_add(
                            ((((((((&raw const sElectricChargingParticleCoordOffsets)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                    .read()) as i32) as isize
                                    * 2,
                            ))
                            .cast::<i8>())
                            .wrapping_offset(1))
                            .read()) as i32),
                        )) as i16),
                    );
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(
                        (((40i32).wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                                as i32)
                                .wrapping_mul(5i32),
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                        .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                        .write(((taskId) as i16));
                    InitAnimLinearTranslation(sprite);
                    StoreSpriteCallbackInData6(sprite, Some(AnimElectricChargingParticles));
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(RunStoredCallbackWhenAnimEnds));
                    if (({
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        > 15i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
                    }
                    if (({
                        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                        let __t8 = ((__p7).read()).wrapping_add(1);
                        (__p7).write(__t8);
                        __t8
                    }) as i32)
                        >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            as i32)
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(0i16);
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            as i32)
                            <= 5i32
                        {
                            let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                            (__p9).write(((__p9).read()).wrapping_add(1));
                        }
                    }
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    (__p10).write(((__p10).read()).wrapping_add(1));
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    (__p11).write(((__p11).read()).wrapping_sub(1));
                }
            }
        } else {
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 0i32
            {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimElectricChargingParticles_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimElectricChargingParticles(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimElectricChargingParticles_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimGrowingChargeOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        }
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimElectricPuff(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimVoltTackleOrbSlide(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAffineAnim(sprite, 1u8);
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(16i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimVoltTackleOrbSlide_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimVoltTackleOrbSlide_Step(sprite: *mut u8) {
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
                    > 40i32
                {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p5 = (sprite).wrapping_add(32).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as i16),
                );
                let __p6 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>();
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as i16),
                );
                if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(80i32)) as u16) as i32)
                    > 400i32
                {
                    DestroySpriteAndMatrix(sprite);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_VoltTackleAttackerReappear(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((GetAnimBattlerSpriteId(0u8)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write((-32i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(2i16);
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(32i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write((-2i16));
                }
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read());
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        ((((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                                    .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            false,
                        ) as u16) as i32)
                            ^ 1i32) as u16) as i32,
                    );
                    if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) != 0
                    {
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                        (__p5).write(
                            (((((__p5).read()) as i32).wrapping_add(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                    .read()) as i32),
                            )) as i16),
                        );
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write(
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                        );
                    } else {
                        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        ((((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                                    .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            false,
                        ) as u16) as i32)
                            ^ 1i32) as u16) as i32,
                    );
                    if (({
                        let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t10 = ((__p9).read()).wrapping_add(1);
                        (__p9).write(__t10);
                        __t10
                    }) as i32)
                        == 8i32
                    {
                        let __p11 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p11).write(((__p11).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_VoltTackleBolt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
                    ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                        as i32)
                        == 0i32
                    {
                        1i32
                    } else {
                        (-1i32)
                    }) as i16),
                );
                'l2: {
                    let __sw2 = (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                        as i32);
                    let __matched = __sw2 == 0i32 || __sw2 == 4i32;
                    if __sw2 == 0i32 {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                            ((GetBattlerSpriteCoord(
                                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                                2u8,
                            )) as i16),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                            ((GetBattlerSpriteCoord(
                                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                                3u8,
                            )) as i16),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32)
                                .wrapping_mul(128i32))
                            .wrapping_add(120i32)) as i16),
                        );
                        break 'l2;
                    }
                    if __sw2 == 4i32 {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                            (((120i32).wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as i32)
                                    .wrapping_mul(128i32),
                            )) as i16),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                            ((GetBattlerSpriteCoord(
                                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                                3u8,
                            )) as i16),
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                            ((((GetBattlerSpriteCoord(
                                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                                2u8,
                            )) as i32)
                                .wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                        .read()) as i32)
                                        .wrapping_mul(32i32),
                                )) as i16),
                        );
                        break 'l2;
                    }
                    if !__matched {
                        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32)
                            & 1i32)
                            != 0i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .write(256i16);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                .write((-16i16));
                        } else {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .write((-16i16));
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                .write(256i16);
                        }
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            == 1i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                                (((80i32).wrapping_sub(
                                    (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                        .read()) as i32)
                                        .wrapping_mul(10i32),
                                )) as i16),
                            );
                        } else {
                            let mut temp: u16 = 0u16;
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                    .read()) as i32)
                                    .wrapping_mul(10i32))
                                .wrapping_add(40i32)) as i16),
                            );
                            temp = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
                                .read()) as u16);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                    .read(),
                            );
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                .write(((temp) as i16));
                        }
                    }
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write((-1i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(3i16);
                }
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    if ((CreateVoltTackleBolt(task, taskId)) != 0)
                        || ((CreateVoltTackleBolt(task, taskId)) != 0)
                    {
                        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateVoltTackleBolt(task: *mut u8, taskId: u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut spriteId: u8 = CreateSprite(
            (&raw const gVoltTackleBoltSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read(),
            35u8,
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
            .write(7i16);
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32) < 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(3i16);
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32) > 3i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        }
        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as i16),
        );
        if ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 1i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)))
            || ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                == (-1i32))
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)))
        {
            return 1u8;
        } else {
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimVoltTackleBolt(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 12i32
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
pub(crate) unsafe extern "C" fn AnimGrowingShockWaveOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
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
                StartSpriteAffineAnim(sprite, 2u8);
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    DestroySpriteAndMatrix(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShockWaveProgressingBolt(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        3u8,
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(4i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(
                    ((crate::c::div_i32(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32)
                            .wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                            ),
                        5i32,
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(7i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write((-1i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                    .write(((BattleAnimAdjustPanning((-64i8))) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                    .write(((BattleAnimAdjustPanning(63i8)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
                    ((crate::c::div_i32(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i32)
                            .wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                                    .read()) as i32),
                            ),
                        3i32,
                    )) as i16),
                );
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if (CreateShockWaveBoltSprite(task, taskId)) != 0 {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            == 5i32
                        {
                            (((task).wrapping_add(8)).cast::<i16>()).write(3i16);
                        } else {
                            let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                            (__p5).write(((__p5).read()).wrapping_add(1));
                        }
                    }
                }
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) != 0 {
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) != 0 {
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                }
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 4i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(4i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(68i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(68i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(4i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(7i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                            .write((-1i16));
                    }
                    if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) != 0
                    {
                        (((task).wrapping_add(8)).cast::<i16>()).write(4i16);
                    } else {
                        (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) != 0 {
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                    (__p10).write(((__p10).read()).wrapping_sub(1));
                } else {
                    (((task).wrapping_add(8)).cast::<i16>()).write(1i16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateShockWaveBoltSprite(task: *mut u8, taskId: u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut spriteId: u8 = CreateSprite(
            (&raw const gShockWaveProgressingBoltSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read(),
            35u8,
        );
        if ((spriteId) as i32) != 64i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(4),
                0,
                10,
                ((((crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(4),
                    0,
                    10,
                    false,
                ) as u16) as i32)
                    .wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as u16) as i32,
            );
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                )) as i16),
            );
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                < 0i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(7i16);
            }
            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                > 7i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            }
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
            .write(3i16);
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32) == 0i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > 0i32)
        {
            let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                        as i32),
                )) as i16),
            );
            PlaySE12WithPanning(
                118u16,
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i8),
            );
        }
        if ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            < 0i32)
            && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32)))
            || ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > 0i32)
                && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)))
        {
            let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            (__p4).write(((__p4).read()).wrapping_add(1));
            let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read()) as i32),
                )) as i16),
            );
            return 1u8;
        } else {
            let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p6).write(
                (((((__p6).read()) as i32).wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                        .wrapping_mul(8i32),
                )) as i16),
            );
            return 0u8;
        }
        #[allow(unreachable_code)]
        {
            return 0u8;
        }
    }
}
pub(crate) unsafe extern "C" fn AnimShockWaveProgressingBolt(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 12i32
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
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShockWaveLightning(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(
                    ((((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        1u8,
                    )) as i32)
                        .wrapping_add(32i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read());
                'l2: loop {
                    if !(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        > 16i32)
                    {
                        break 'l2;
                    }
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                    (__p2).write((((((__p2).read()) as i32).wrapping_sub(32i32)) as i16));
                }
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                    ((((GetBattlerSpriteSubpriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as i32)
                        .wrapping_sub(2i32)) as i16),
                );
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
                    if (CreateShockWaveLightningSprite(task, taskId)) != 0 {
                        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
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
pub(crate) unsafe extern "C" fn CreateShockWaveLightningSprite(task: *mut u8, taskId: u8) -> u8 {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut spriteId: u8 = CreateSprite(
            (&raw const gLightningSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as u8),
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimShockWaveLightning));
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
            .write(10i16);
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
            >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
        {
            return 1u8;
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(32i32)) as i16));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn AnimShockWaveLightning(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            let __p1 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
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
            (__p1).write(((__p1).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
