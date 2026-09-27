//! Translated from `src/battle_anim_fire.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_FireSpiralSpread_0 sAnim_FireSpiralSpread_1 sAnims_FireSpiralSpread gFireSpiralInwardSpriteTemplate gFireSpreadSpriteTemplate sAnim_LargeFlame sAnims_LargeFlame sAnim_FirePlume sAnims_FirePlume sAffineAnim_LargeFlame sAffineAnims_LargeFlame gLargeFlameSpriteTemplate gLargeFlameScatterSpriteTemplate gFirePlumeSpriteTemplate sUnusedEmberFirePlumeSpriteTemplate sAnim_UnusedSmallEmber sAnims_UnusedSmallEmber sUnusedSmallEmberSpriteTemplate sAffineAnim_SunlightRay sAffineAnims_SunlightRay gSunlightRaySpriteTemplate sAnim_BasicFire gAnims_BasicFire gEmberSpriteTemplate gEmberFlareSpriteTemplate gBurnFlameSpriteTemplate gFireBlastRingSpriteTemplate sAnim_FireBlastCross sAnims_FireBlastCross sAffineAnim_Unused_0 sAffineAnim_Unused_1 sAffineAnims_Unused gFireBlastCrossSpriteTemplate gFireSpiralOutwardSpriteTemplate gWeatherBallFireDownSpriteTemplate gEruptionLaunchRockSpriteTemplate sEruptionLaunchRockSpeeds gEruptionFallingRockSpriteTemplate sAnim_WillOWispOrb_0 sAnim_WillOWispOrb_1 sAnim_WillOWispOrb_2 sAnim_WillOWispOrb_3 sAnims_WillOWispOrb gWillOWispOrbSpriteTemplate sAnim_WillOWispFire sAnims_WillOWispFire gWillOWispFireSpriteTemplate sShakeDirsPattern0 sShakeDirsPattern1
#[allow(unused_imports)]
use crate::data::battle_anim_fire::*;

unsafe extern "C" {
    static mut gAnimCustomPanning: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn AnimTravelDiagonally(a0: *mut u8);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitAnimLinearTranslationWithSpeed(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn PrepareEruptAnimTaskData(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: i16, a5: i16, a6: u16);
    fn ResetSpriteRotScale(a0: u8);
    fn SetAnimSpriteInitialXOffset(a0: *mut u8, a1: i16);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateSpriteInGrowingCircle(a0: *mut u8);
    fn TranslateSpriteLinear(a0: *mut u8);
    fn TranslateSpriteLinearFixedPoint(a0: *mut u8);
    fn UpdateEruptAnimTask(a0: *mut u8) -> u8;
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimFireSpiralInward(sprite: *mut u8) {
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
pub(crate) unsafe extern "C" fn AnimFireSpread(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPoint));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimFirePlume(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
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
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
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
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .read(),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimLargeFlame_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimLargeFlame(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
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
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .read(),
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
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimLargeFlame_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimLargeFlame_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            < ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
        }
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
        {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedSmallEmber(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
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
            ((sprite).wrapping_add(67)).write(8u8);
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimUnusedSmallEmber_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedSmallEmber_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                > 10000i32
            {
                ((sprite).wrapping_add(67)).write(1u8);
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            >> 8),
                    )) as i16),
            ));
            ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32)
                            >> 8),
                    )) as i16),
            ));
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 255i32 {
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(256i32)) as i16));
            } else {
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(256i32)) as i16));
                }
            }
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p5).write(((__p5).read()).wrapping_sub(1));
        } else {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSunlight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(0i16);
        ((sprite).wrapping_add(34).cast::<i16>()).write(0i16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(60i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(140i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(80i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimEmberFlare(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
            == ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))
            && ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                == ((GetBattlerAtPosition(2u8)) as i32))
                || (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                    == ((GetBattlerAtPosition(3u8)) as i32)))
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTravelDiagonally));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimBurnFlame(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
            (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                .wrapping_neg()) as i16),
        );
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                .wrapping_neg()) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTravelDiagonally));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimFireRing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFireRing_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimFireRing_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateFireRingCircleOffset(sprite);
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 18i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(25i16);
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
            .write(Some(AnimFireRing_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFireRing_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimFireRing_Step3));
            (((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read())
            .unwrap_unchecked()(sprite);
        } else {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        28i16,
                    )) as i32),
                )) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                        28i16,
                    )) as i32),
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_add(20i32)
                    & 255i32) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFireRing_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateFireRingCircleOffset(sprite);
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 31i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateFireRingCircleOffset(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            28i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            28i16,
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_add(20i32)
                & 255i32) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimFireCross(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinear));
    }
}
pub(crate) unsafe extern "C" fn AnimFireSpiralOutward(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(AnimFireSpiralOutward_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimFireSpiralOutward_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFireSpiralOutward_Step2));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimFireSpiralOutward_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 8) as i16),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 8) as i16),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(10i32)
                & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(208i32)) as i16));
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_sub(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == (-1i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_EruptionLaunchRocks(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
            .write(((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        PrepareBattlerSpriteForRotScale(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            0u8,
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_EruptionLaunchRocks_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_EruptionLaunchRocks_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32;
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                PrepareEruptAnimTaskData(
                    task,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    256i16,
                    256i16,
                    224i16,
                    512i16,
                    32u16,
                );
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if ((({
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
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
                        .write(3i16);
                    } else {
                        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(36)
                        .cast::<i16>())
                        .write((-3i16));
                    }
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    != 0i32
                {
                    if (({
                        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        let __t8 = ((__p7).read()).wrapping_add(1);
                        (__p7).write(__t8);
                        __t8
                    }) as i32)
                        > 4i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                        let __p9 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(34)
                        .cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                if !((UpdateEruptAnimTask(task)) != 0) {
                    SetBattlerSpriteYOffsetFromYScale(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as u8),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p10 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    > 4i32
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        != 0i32
                    {
                        PrepareEruptAnimTaskData(
                            task,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            224i16,
                            512i16,
                            384i16,
                            240i16,
                            6u16,
                        );
                    } else {
                        PrepareEruptAnimTaskData(
                            task,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            224i16,
                            512i16,
                            384i16,
                            192i16,
                            6u16,
                        );
                    }
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if !((UpdateEruptAnimTask(task)) != 0) {
                    CreateEruptionLaunchRocks(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as u8),
                        taskId,
                        6u8,
                    );
                    let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                if (({
                    let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t16 = ((__p15).read()).wrapping_add(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if ((({
                        let __p17 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t18 = ((__p17).read()).wrapping_add(1);
                        (__p17).write(__t18);
                        __t18
                    }) as i32)
                        & 1i32)
                        != 0
                    {
                        let __p19 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p19).write((((((__p19).read()) as i32).wrapping_add(3i32)) as i16));
                    } else {
                        let __p20 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p20).write((((((__p20).read()) as i32).wrapping_sub(3i32)) as i16));
                    }
                }
                if (({
                    let __p21 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t22 = ((__p21).read()).wrapping_add(1);
                    (__p21).write(__t22);
                    __t22
                }) as i32)
                    > 24i32
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        != 0i32
                    {
                        PrepareEruptAnimTaskData(
                            task,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            384i16,
                            240i16,
                            256i16,
                            256i16,
                            8u16,
                        );
                    } else {
                        PrepareEruptAnimTaskData(
                            task,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            384i16,
                            192i16,
                            256i16,
                            256i16,
                            8u16,
                        );
                    }
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        let __p23 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p23).write((((((__p23).read()) as i32).wrapping_sub(3i32)) as i16));
                    }
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p24 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p24).write(((__p24).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                __fall = true;
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    != 0i32
                {
                    let __p25 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>();
                    (__p25).write(((__p25).read()).wrapping_sub(1));
                }
                if !((UpdateEruptAnimTask(task)) != 0) {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read());
                    ResetSpriteRotScale(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as u8),
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p26 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p26).write(((__p26).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
            if !__matched {
                __fall = true;
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateEruptionLaunchRocks(
    spriteId: u8,
    taskId: u8,
    activeSpritesIdx: u8,
) {
    unsafe {
        let mut spriteId = spriteId;
        let mut taskId = taskId;
        let mut activeSpritesIdx = activeSpritesIdx;
        let mut i: u16 = 0u16;
        let mut j: u16 = 0u16;
        let mut sign: i8 = 0i8;
        let mut y: u16 = GetEruptionLaunchRockInitialYPos(spriteId);
        let mut x: u16 = ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .read()) as u16);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            x = ((((x) as i32).wrapping_sub(12i32)) as u16);
            sign = 1i8;
        } else {
            x = ((((x) as i32).wrapping_add(16i32)) as u16);
            sign = (-1i8);
        }
        {
            i = 0u16;
            j = 0u16;
            'l1: loop {
                if !(((i) as i32) <= 6i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = CreateSprite(
                        (&raw const gEruptionLaunchRockSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        ((x) as i16),
                        ((y) as i16),
                        2u8,
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
                                    (((j) as i32).wrapping_mul(4i32)).wrapping_add(64i32),
                                )) as u16) as i32,
                        );
                        if (({
                            let __t1 = (j).wrapping_add(1);
                            j = __t1;
                            __t1
                        }) as i32)
                            >= 5i32
                        {
                            j = 0u16;
                        }
                        InitEruptionLaunchRockCoordData(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            (((((((((&raw const sEruptionLaunchRockSpeeds)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_mul(((sign) as i32)))
                                as i16),
                            ((((((&raw const sEruptionLaunchRockSpeeds)
                                .cast::<u8>()
                                .cast_mut())
                            .cast::<u8>())
                            .wrapping_offset(((i) as i32) as isize * 4))
                            .cast::<i16>())
                            .wrapping_offset(1))
                            .read(),
                        );
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
                        .write(((activeSpritesIdx) as i16));
                        let __p2 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(((activeSpritesIdx) as i32) as isize);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimEruptionLaunchRock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        UpdateEruptionLaunchRockPos(sprite);
        if (crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0 {
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
pub(crate) unsafe extern "C" fn GetEruptionLaunchRockInitialYPos(spriteId: u8) -> u16 {
    unsafe {
        let mut spriteId = spriteId;
        let mut y: i16 = (((((((((&raw mut gSprites).cast::<u8>())
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
            ))
        .wrapping_add(
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(41)
            .cast::<i8>())
            .read()) as i32),
        )) as i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            y = ((((y) as i32).wrapping_add(74i32)) as i16);
        } else {
            y = ((((y) as i32).wrapping_add(44i32)) as i16);
        }
        return ((y) as u16);
    }
}
pub(crate) unsafe extern "C" fn InitEruptionLaunchRockCoordData(
    sprite: *mut u8,
    speedX: i16,
    speedY: i16,
) {
    unsafe {
        let mut sprite = sprite;
        let mut speedX = speedX;
        let mut speedY = speedY;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as u16) as i32)
                .wrapping_mul(8i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as u16) as i32)
                .wrapping_mul(8i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((speedX) as i32).wrapping_mul(8i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((speedY) as i32).wrapping_mul(8i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn UpdateEruptionLaunchRockPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut extraLaunchSpeed: i32 = 0i32;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 2i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p3).write(((__p3).read()).wrapping_add(1));
            extraLaunchSpeed = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .read()) as u16) as i32)
                .wrapping_mul(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as u16) as i32),
                );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p4).write((((((__p4).read()) as i32).wrapping_add(extraLaunchSpeed)) as i16));
        }
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p5).write(
            (((((__p5).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 3) as i16),
        );
        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p6).write(
            (((((__p6).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 3) as i16),
        );
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-8i32))
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 248i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-8i32)))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 120i32)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimEruptionFallingRock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(4))
                    .read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16) as i32,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimEruptionFallingRock_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimEruptionFallingRock_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    != 0i32
                {
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                    return;
                }
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p4 = (sprite).wrapping_add(34).cast::<i16>();
                (__p4).write((((((__p4).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                {
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                    );
                    let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if ((({
                        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __t9 = ((__p8).read()).wrapping_add(1);
                        (__p8).write(__t9);
                        __t9
                    }) as i32)
                        & 1i32)
                        != 0i32
                    {
                        ((sprite).wrapping_add(38).cast::<i16>()).write((-3i16));
                    } else {
                        ((sprite).wrapping_add(38).cast::<i16>()).write(3i16);
                    }
                }
                if (({
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    > 16i32
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWillOWispOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                InitSpritePosToAnimAttacker(sprite, 0u8);
                StartSpriteAnim(
                    sprite,
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as u8),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                        .read(),
                );
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(4i16);
                } else {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-4i16));
                }
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((GetBattlerSpriteBGPriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as u16) as i32,
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p3).write((((((__p3).read()) as i32).wrapping_add(192i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32
                {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                            .read()) as i32)
                            >> 8)
                            .wrapping_neg()) as i16),
                    );
                } else {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            >> 8) as i16),
                    );
                }
                ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(4i32)
                        & 255i32) as i16),
                );
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(4i32)
                        & 255i32) as i16),
                );
                if (({
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == 31i32
                {
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
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(256i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                        .write(((sprite).wrapping_add(32).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            2u8,
                        )) as i16),
                    );
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                        .write(((sprite).wrapping_add(34).cast::<i16>()).read());
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            3u8,
                        )) as i16),
                    );
                    InitAnimLinearTranslationWithSpeed(sprite);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimWillOWispOrb_Step));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWillOWispOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut initialData5: i16 = 0i16;
        let mut newData5: i16 = 0i16;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(36).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        16i16,
                    )) as i32),
                )) as i16),
            );
            initialData5 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read();
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    .wrapping_add(4i32)
                    & 255i32) as i16),
            );
            newData5 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read();
            if (((((initialData5) as i32) == 0i32) || (((initialData5) as i32) > 196i32))
                && (((newData5) as i32) > 0i32))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    == 0i32)
            {
                PlaySE12WithPanning(
                    144u16,
                    ((((&raw mut gAnimCustomPanning).cast::<u8>()).read()) as i8),
                );
            }
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWillOWispFire(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !(((((sprite).wrapping_add(46)).cast::<i16>()).read()) != 0) {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(1i32)) as i16));
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(384i32)) as i16));
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p3).write((((((__p3).read()) as i32).wrapping_add(160i32)) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 8) as i16),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8) as i16),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_add(7i32)
                & 255i32) as i16),
        );
        if !((IsContest()) != 0) {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 64i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 195i32)
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((GetBattlerSpriteBGPriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((((GetBattlerSpriteBGPriority(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                    )) as i32)
                        .wrapping_add(1i32)) as u16) as i32,
                );
            }
        } else {
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 64i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 195i32)
            {
                ((sprite).wrapping_add(67)).write(29u8);
            } else {
                ((sprite).wrapping_add(67)).write(31u8);
            }
        }
        if (({
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            > 20i32
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 30i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoveHeatWaveTargets(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
            ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32
            {
                1i32
            } else {
                (-1i32)
            }) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((((IsBattlerSpriteVisible(
                ((((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
            )) as i32)
                .wrapping_add(1i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
            .write(((GetAnimBattlerSpriteId(1u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(3u8)) as i16));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_MoveHeatWaveTargets_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MoveHeatWaveTargets_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16),
                );
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    >= 2i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(2i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                            .write((-2i16));
                    }
                }
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    'l2: loop {
                        if !(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                                    (((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(3))
                                    .read()) as i32)
                                        .wrapping_add(14i32))
                                        as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32)
                                    .wrapping_add(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(11))
                                        .read()) as i32),
                                    )) as i16),
                            );
                        }
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == 16i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
                    let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    >= 5i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p12).write(((__p12).read()).wrapping_add(1));
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(2i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                            .write((-2i16));
                    }
                }
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    'l4: loop {
                        if !(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                                    (((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(3))
                                    .read()) as i32)
                                        .wrapping_add(14i32))
                                        as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32)
                                    .wrapping_add(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(11))
                                        .read()) as i32),
                                    )) as i16),
                            );
                        }
                        let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p13).write(((__p13).read()).wrapping_add(1));
                    }
                }
                if (({
                    let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t15 = ((__p14).read()).wrapping_add(1);
                    (__p14).write(__t15);
                    __t15
                }) as i32)
                    == 96i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
                    let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p17 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                (__p17).write(
                    (((((__p17).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16),
                );
                if (({
                    let __p18 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t19 = ((__p18).read()).wrapping_add(1);
                    (__p18).write(__t19);
                    __t19
                }) as i32)
                    >= 2i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p20 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    (__p20).write(((__p20).read()).wrapping_add(1));
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(2i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                            .write((-2i16));
                    }
                }
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    'l6: loop {
                        if !(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32))
                        {
                            break 'l6;
                        }
                        'l7: {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                                    (((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(3))
                                    .read()) as i32)
                                        .wrapping_add(14i32))
                                        as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32)
                                    .wrapping_add(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(11))
                                        .read()) as i32),
                                    )) as i16),
                            );
                        }
                        let __p21 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p21).write(((__p21).read()).wrapping_add(1));
                    }
                }
                if (({
                    let __p22 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t23 = ((__p22).read()).wrapping_add(1);
                    (__p22).write(__t23);
                    __t23
                }) as i32)
                    == 16i32
                {
                    let __p24 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p24).write(((__p24).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    'l8: loop {
                        if !(((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32))
                        {
                            break 'l8;
                        }
                        'l9: {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                                    (((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(3))
                                    .read()) as i32)
                                        .wrapping_add(14i32))
                                        as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(0i16);
                        }
                        let __p25 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p25).write(((__p25).read()).wrapping_add(1));
                    }
                }
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendBackground(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        BlendPalette(
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            16u16,
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as u16),
        );
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShakeTargetInPattern(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut dir: i8 = 0i8;
        let mut spriteId: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
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
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read(),
            );
        }
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
        ))
        .read();
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            == 0i32
        {
            dir = ((((&raw const sShakeDirsPattern0)
                .cast::<u8>()
                .cast_mut()
                .cast::<i8>())
            .cast::<i8>())
            .wrapping_offset(
                (crate::c::rem_i32(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32),
                    10i32,
                )) as isize,
            ))
            .read();
        } else {
            dir = ((((&raw const sShakeDirsPattern1)
                .cast::<u8>()
                .cast_mut()
                .cast::<i8>())
            .cast::<i8>())
            .wrapping_offset(
                (crate::c::rem_i32(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32),
                    10i32,
                )) as isize,
            ))
            .read();
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 1i32
        {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_mul(((dir) as i32))
                    < 0i32
                {
                    (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(((dir) as i32)))
                    .wrapping_neg()
                } else {
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(((dir) as i32))
                }) as i16),
            );
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_mul(((dir) as i32))) as i16),
            );
        }
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
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
