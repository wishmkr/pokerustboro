//! Translated from `src/battle_anim_flying.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gEllipticalGustSpriteTemplate sAffineAnim_GustToTarget sAffineAnims_GustToTarget gGustToTargetSpriteTemplate sAffineAnim_AirWaveCrescent sAffineAnims_AirWaveCrescent gAirWaveCrescentSpriteTemplate sAffineAnim_FlyBallUp sAffineAnims_FlyBallUp sAffineAnim_FlyBallAttack_0 sAffineAnim_FlyBallAttack_1 sAffineAnims_FlyBallAttack gFlyBallUpSpriteTemplate gFlyBallAttackSpriteTemplate sAnim_FallingFeather_0 sAnim_FallingFeather_1 sAnims_FallingFeather gFallingFeatherSpriteTemplate sUnusedBubbleThrowSpriteTemplate sAnim_WhirlwindLines sAnims_WhirlwindLines gWhirlwindLineSpriteTemplate sAffineAnim_BounceBallShrink sAffineAnims_BounceBallShrink gBounceBallShrinkSpriteTemplate sAffineAnim_BounceBallLand sAffineAnims_BounceBallLand gBounceBallLandSpriteTemplate sAffineAnim_DiveBall sAffineAnims_DiveBall gDiveBallSpriteTemplate sAnim_Unused sAnims_Unused gDiveWaterSplashSpriteTemplate gSprayWaterDropletSpriteTemplate sUnusedFlashingLightSpriteTemplate gSkyAttackBirdSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_flying::*;

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gFlashingHitSplatSpriteTemplate: u8;
    static mut gOamMatrices: u8;
    static mut gPlttBufferFaded: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSpriteAndAnimate(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsContest() -> u8;
    fn Random2() -> u16;
    fn ResetSpriteRotScale_PreserveAffine(a0: *mut u8);
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn SeekSpriteAnim(a0: *mut u8, a1: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimSpriteToTargetMonLocation(a0: *mut u8);
    fn TrySetSpriteRotScale(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: u16);
}

pub(crate) unsafe extern "C" fn AnimEllipticalGust(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 0u8);
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(20i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(191i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimEllipticalGust_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimEllipticalGust_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            32i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            8i16,
        ));
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(5i32)) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32) & 255i32) as i16));
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 71i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AnimateGustTornadoPalette(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((IndexOfSpritePaletteTag(10009u16)) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_AnimateGustTornadoPalette_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AnimateGustTornadoPalette_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut data2: u8 = 0u8;
        let mut temp: u16 = 0u16;
        let mut i: i32 = 0i32;
        let mut base: i32 = 0i32;
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
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            data2 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            temp = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((256i32).wrapping_add(((data2) as i32).wrapping_mul(16i32))).wrapping_add(8i32))
                    as isize,
            ))
            .read();
            i = 7i32;
            base = ((data2) as i32).wrapping_mul(16i32);
            'l1: loop {
                'l2: {
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                        ((((base).wrapping_add(256i32)).wrapping_add(1i32)).wrapping_add(i))
                            as isize,
                    ))
                    .write(
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                (((base).wrapping_add(256i32)).wrapping_add(i)) as isize,
                            ))
                        .read(),
                    );
                    i = (i).wrapping_sub(1);
                }
                if !(i > 0i32) {
                    break 'l1;
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((base).wrapping_add(256i32)).wrapping_add(1i32)) as isize))
            .write(temp);
        }
        if (({
            let __p3 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_sub(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 0i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGustToTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(3))
                    .read()) as i32),
                )) as i16),
        );
        InitAnimLinearTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(AnimGustToTarget_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimGustToTarget_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimAirWaveCrescent(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        if (IsContest()) != 0 {
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
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
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
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read())
            as i32)
            == 0i32
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32),
                )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
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
        SeekSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                .read()) as u8),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimFlyBallUp(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFlyBallUp_Step));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn AnimFlyBallUp_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (sprite).wrapping_add(38).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_sub(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
        }
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            < (-32i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlyBallAttack(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(272i16);
            ((sprite).wrapping_add(34).cast::<i16>()).write((-32i16));
            StartSpriteAffineAnim(sprite, 1u8);
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write((-32i16));
            ((sprite).wrapping_add(34).cast::<i16>()).write((-32i16));
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
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
        .write(Some(AnimFlyBallAttack_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFlyBallAttack_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        AnimTranslateLinear(sprite);
        if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16)
            as i32)
            >> 8)
            > 200i32
        {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write((((((__p2).read()) as i32) & 255i32) as i16));
        }
        if ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
            < (-32i32))
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                > 272i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                > 160i32)
        {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyAnimSpriteAfterTimer(sprite: *mut u8) {
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
            if ((crate::c::bf_read((sprite).wrapping_add(1), 0, 2, false) as u32) & 1u32) != 0 {
                FreeOamMatrix(
                    ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                );
                crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
            }
            DestroySprite(sprite);
            let __p3 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFallingFeather(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        let mut matrixNum: u8 = 0u8;
        let mut sinIndex: u8 = 0u8;
        let mut spriteCoord: i16 = 0i16;
        let mut data: *mut u8 = (((sprite).wrapping_add(46)).cast::<i16>()).cast::<u8>();
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as i32)
            & 256i32)
            != 0
        {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(battler, 0u8)) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        spriteCoord = ((GetBattlerSpriteCoord(battler, 1u8)) as i16);
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((spriteCoord) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((data).wrapping_add(8).cast::<u16>())
            .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 8) as u16));
        crate::c::bf_write(
            (data).wrapping_add(14),
            1,
            15,
            ((((spriteCoord) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
                    .read()) as i32),
            )) as u16) as i32,
        );
        crate::c::bf_write((data).wrapping_add(0), 2, 1, (1u16) as i32);
        ((data).wrapping_add(2).cast::<u16>()).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                & 255i32) as u16),
        );
        ((data).wrapping_add(10).cast::<u16>()).write(
            (((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                >> 8)
                & 255i32) as u16),
        );
        ((data).wrapping_add(4).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((data).wrapping_add(6).cast::<u16>()).write(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u16),
        );
        ((((data).wrapping_add(12)).cast::<u8>()).cast::<u16>()).write(
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                .read()) as u16),
        );
        if (((((data).wrapping_add(2).cast::<u16>()).read()) as i32) >= 64i32)
            && (((((data).wrapping_add(2).cast::<u16>()).read()) as i32) <= 191i32)
        {
            if !((IsContest()) != 0) {
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((((GetBattlerSpriteBGPriority(battler)) as i32).wrapping_add(1i32)) as u16)
                        as i32,
                );
            } else {
                crate::c::bf_write(
                    (sprite).wrapping_add(5),
                    2,
                    2,
                    ((GetBattlerSpriteBGPriority(battler)) as u16) as i32,
                );
            }
            crate::c::bf_write((data).wrapping_add(14), 0, 1, (0u16) as i32);
            if !((((((data).wrapping_add(4).cast::<i16>()).read()) as i32) & 32768i32) != 0) {
                crate::c::bf_write(
                    (sprite).wrapping_add(63),
                    0,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
                ((sprite).wrapping_add(42)).write(
                    ((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false) as u16) as u8),
                );
                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
            }
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(battler)) as u16) as i32,
            );
            crate::c::bf_write((data).wrapping_add(14), 0, 1, (1u16) as i32);
            if (((((data).wrapping_add(4).cast::<i16>()).read()) as i32) & 32768i32) != 0 {
                crate::c::bf_write(
                    (sprite).wrapping_add(63),
                    0,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
                ((sprite).wrapping_add(42)).write(
                    ((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false) as u16) as u8),
                );
                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
            }
        }
        crate::c::bf_write(
            (data).wrapping_add(0),
            4,
            4,
            ((((((data).wrapping_add(2).cast::<u16>()).read()) as i32) >> 6) as u16) as i32,
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                ((((data).wrapping_add(2).cast::<u16>()).read()) as i32) as isize,
            ))
            .read()) as i32)
                .wrapping_mul((((((data).wrapping_add(12)).cast::<u8>()).read()) as i32))
                >> 8) as i16),
        );
        matrixNum = ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8);
        sinIndex = (((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg()
            >> 1)
            .wrapping_add(((((data).wrapping_add(10).cast::<u16>()).read()) as i32)))
            as u8);
        spriteCoord = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
            .wrapping_offset(((sinIndex) as i32) as isize))
        .read();
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .cast::<i16>())
        .write({
            let __v1 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset((((sinIndex) as i32).wrapping_add(64i32)) as isize))
            .read();
            ((((&raw mut gOamMatrices).cast::<u8>())
                .wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<i16>())
            .write(__v1);
            __v1
        });
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(2)
        .cast::<i16>())
        .write(spriteCoord);
        ((((&raw mut gOamMatrices).cast::<u8>())
            .wrapping_offset(((matrixNum) as i32) as isize * 8))
        .wrapping_add(4)
        .cast::<i16>())
        .write(((((spriteCoord) as i32).wrapping_neg()) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFallingFeather_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFallingFeather_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut matrixNum: u8 = 0u8;
        let mut sinIndex: u8 = 0u8;
        let mut sinVal: i16 = 0i16;
        let mut data: *mut u8 = (((sprite).wrapping_add(46)).cast::<i16>()).cast::<u8>();
        if (crate::c::bf_read((data).wrapping_add(0), 0, 1, false) as u16) != 0 {
            if crate::c::rem_i32(
                (({
                    let __t1 = (crate::c::bf_read((data).wrapping_add(1), 0, 8, false) as u16);
                    crate::c::bf_write(
                        (data).wrapping_add(1),
                        0,
                        8,
                        ((crate::c::bf_read((data).wrapping_add(1), 0, 8, false) as u16)
                            .wrapping_sub(1)) as i32,
                    );
                    __t1
                }) as i32),
                256i32,
            ) == 0i32
            {
                crate::c::bf_write((data).wrapping_add(0), 0, 1, (0u16) as i32);
                crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
            }
        } else {
            'l1: {
                let __sw2 = crate::c::div_i32(
                    ((((data).wrapping_add(2).cast::<u16>()).read()) as i32),
                    64i32,
                );
                if __sw2 == 0i32 {
                    if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                        as i32)
                        == 1i32
                    {
                        crate::c::bf_write((data).wrapping_add(0), 3, 1, (1u16) as i32);
                        crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                        crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                    } else {
                        if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                            as i32)
                            == 3i32
                        {
                            crate::c::bf_write(
                                (data).wrapping_add(0),
                                1,
                                1,
                                ((((crate::c::bf_read((data).wrapping_add(0), 1, 1, false) as u16)
                                    as i32)
                                    ^ 1i32) as u16) as i32,
                            );
                            crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                            crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                        } else {
                            if (crate::c::bf_read((data).wrapping_add(0), 3, 1, false) as u16) != 0
                            {
                                crate::c::bf_write(
                                    (sprite).wrapping_add(63),
                                    0,
                                    1,
                                    ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as i32)
                                        ^ 1i32) as u16) as i32,
                                );
                                ((sprite).wrapping_add(42)).write(
                                    ((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as u8),
                                );
                                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
                                if (crate::c::bf_read((data).wrapping_add(0), 2, 1, false) as u16)
                                    != 0
                                {
                                    if !((IsContest()) != 0) {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_sub(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_add(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    } else {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            let __p3 = (sprite).wrapping_add(67);
                                            (__p3).write(
                                                (((((__p3).read()) as i32).wrapping_sub(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            let __p4 = (sprite).wrapping_add(67);
                                            (__p4).write(
                                                (((((__p4).read()) as i32).wrapping_add(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    }
                                }
                                crate::c::bf_write((data).wrapping_add(0), 3, 1, (0u16) as i32);
                            }
                        }
                    }
                    crate::c::bf_write((data).wrapping_add(0), 4, 4, (0u16) as i32);
                    break 'l1;
                }
                if __sw2 == 1i32 {
                    if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                        as i32)
                        == 0i32
                    {
                        crate::c::bf_write((data).wrapping_add(0), 3, 1, (1u16) as i32);
                        crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                        crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                    } else {
                        if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                            as i32)
                            == 2i32
                        {
                            crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                            crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                        } else {
                            if (crate::c::bf_read((data).wrapping_add(0), 3, 1, false) as u16) != 0
                            {
                                crate::c::bf_write(
                                    (sprite).wrapping_add(63),
                                    0,
                                    1,
                                    ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as i32)
                                        ^ 1i32) as u16) as i32,
                                );
                                ((sprite).wrapping_add(42)).write(
                                    ((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as u8),
                                );
                                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
                                if (crate::c::bf_read((data).wrapping_add(0), 2, 1, false) as u16)
                                    != 0
                                {
                                    if !((IsContest()) != 0) {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_sub(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_add(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    } else {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            let __p5 = (sprite).wrapping_add(67);
                                            (__p5).write(
                                                (((((__p5).read()) as i32).wrapping_sub(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            let __p6 = (sprite).wrapping_add(67);
                                            (__p6).write(
                                                (((((__p6).read()) as i32).wrapping_add(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    }
                                }
                                crate::c::bf_write((data).wrapping_add(0), 3, 1, (0u16) as i32);
                            }
                        }
                    }
                    crate::c::bf_write((data).wrapping_add(0), 4, 4, (1u16) as i32);
                    break 'l1;
                }
                if __sw2 == 2i32 {
                    if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                        as i32)
                        == 3i32
                    {
                        crate::c::bf_write((data).wrapping_add(0), 3, 1, (1u16) as i32);
                        crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                        crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                    } else {
                        if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                            as i32)
                            == 1i32
                        {
                            crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                            crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                        } else {
                            if (crate::c::bf_read((data).wrapping_add(0), 3, 1, false) as u16) != 0
                            {
                                crate::c::bf_write(
                                    (sprite).wrapping_add(63),
                                    0,
                                    1,
                                    ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as i32)
                                        ^ 1i32) as u16) as i32,
                                );
                                ((sprite).wrapping_add(42)).write(
                                    ((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as u8),
                                );
                                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
                                if (crate::c::bf_read((data).wrapping_add(0), 2, 1, false) as u16)
                                    != 0
                                {
                                    if !((IsContest()) != 0) {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_sub(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_add(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    } else {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            let __p7 = (sprite).wrapping_add(67);
                                            (__p7).write(
                                                (((((__p7).read()) as i32).wrapping_sub(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            let __p8 = (sprite).wrapping_add(67);
                                            (__p8).write(
                                                (((((__p8).read()) as i32).wrapping_add(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    }
                                }
                                crate::c::bf_write((data).wrapping_add(0), 3, 1, (0u16) as i32);
                            }
                        }
                    }
                    crate::c::bf_write((data).wrapping_add(0), 4, 4, (2u16) as i32);
                    break 'l1;
                }
                if __sw2 == 3i32 {
                    if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                        as i32)
                        == 2i32
                    {
                        crate::c::bf_write((data).wrapping_add(0), 3, 1, (1u16) as i32);
                    } else {
                        if (((crate::c::bf_read((data).wrapping_add(0), 4, 4, false) as u16) as u8)
                            as i32)
                            == 0i32
                        {
                            crate::c::bf_write(
                                (data).wrapping_add(0),
                                1,
                                1,
                                ((((crate::c::bf_read((data).wrapping_add(0), 1, 1, false) as u16)
                                    as i32)
                                    ^ 1i32) as u16) as i32,
                            );
                            crate::c::bf_write((data).wrapping_add(0), 0, 1, (1u16) as i32);
                            crate::c::bf_write((data).wrapping_add(1), 0, 8, (0u16) as i32);
                        } else {
                            if (crate::c::bf_read((data).wrapping_add(0), 3, 1, false) as u16) != 0
                            {
                                crate::c::bf_write(
                                    (sprite).wrapping_add(63),
                                    0,
                                    1,
                                    ((((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as i32)
                                        ^ 1i32) as u16) as i32,
                                );
                                ((sprite).wrapping_add(42)).write(
                                    ((crate::c::bf_read((sprite).wrapping_add(63), 0, 1, false)
                                        as u16) as u8),
                                );
                                crate::c::bf_write((sprite).wrapping_add(63), 2, 1, (1u16) as i32);
                                crate::c::bf_write((sprite).wrapping_add(63), 4, 1, (0u16) as i32);
                                if (crate::c::bf_read((data).wrapping_add(0), 2, 1, false) as u16)
                                    != 0
                                {
                                    if !((IsContest()) != 0) {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_sub(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            crate::c::bf_write(
                                                (sprite).wrapping_add(5),
                                                2,
                                                2,
                                                ((crate::c::bf_read(
                                                    (sprite).wrapping_add(5),
                                                    2,
                                                    2,
                                                    false,
                                                )
                                                    as u16)
                                                    .wrapping_add(1))
                                                    as i32,
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    } else {
                                        if !((crate::c::bf_read(
                                            (data).wrapping_add(14),
                                            0,
                                            1,
                                            false,
                                        ) as u16)
                                            != 0)
                                        {
                                            let __p9 = (sprite).wrapping_add(67);
                                            (__p9).write(
                                                (((((__p9).read()) as i32).wrapping_sub(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        } else {
                                            let __p10 = (sprite).wrapping_add(67);
                                            (__p10).write(
                                                (((((__p10).read()) as i32).wrapping_add(12i32))
                                                    as u8),
                                            );
                                            crate::c::bf_write(
                                                (data).wrapping_add(14),
                                                0,
                                                1,
                                                ((((crate::c::bf_read(
                                                    (data).wrapping_add(14),
                                                    0,
                                                    1,
                                                    false,
                                                )
                                                    as u16)
                                                    as i32)
                                                    ^ 1i32)
                                                    as u16)
                                                    as i32,
                                            );
                                        }
                                    }
                                }
                                crate::c::bf_write((data).wrapping_add(0), 3, 1, (0u16) as i32);
                            }
                        }
                    }
                    crate::c::bf_write((data).wrapping_add(0), 4, 4, (3u16) as i32);
                    break 'l1;
                }
            }
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((data).wrapping_add(12)).cast::<u8>()).wrapping_offset(
                    ((crate::c::bf_read((data).wrapping_add(0), 1, 1, false) as u16) as i32)
                        as isize,
                ))
                .read()) as i32)
                    .wrapping_mul(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                            ((((data).wrapping_add(2).cast::<u16>()).read()) as i32) as isize,
                        ))
                        .read()) as i32),
                    )
                    >> 8) as i16),
            );
            matrixNum = ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8);
            sinIndex =
                (((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg() >> 1)
                    .wrapping_add(((((data).wrapping_add(10).cast::<u16>()).read()) as i32)))
                    as u8);
            sinVal = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                .wrapping_offset(((sinIndex) as i32) as isize))
            .read();
            ((((&raw mut gOamMatrices).cast::<u8>())
                .wrapping_offset(((matrixNum) as i32) as isize * 8))
            .cast::<i16>())
            .write({
                let __v11 = ((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                    .wrapping_offset((((sinIndex) as i32).wrapping_add(64i32)) as isize))
                .read();
                ((((&raw mut gOamMatrices).cast::<u8>())
                    .wrapping_offset(((matrixNum) as i32) as isize * 8))
                .wrapping_add(6)
                .cast::<i16>())
                .write(__v11);
                __v11
            });
            ((((&raw mut gOamMatrices).cast::<u8>())
                .wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(2)
            .cast::<i16>())
            .write(sinVal);
            ((((&raw mut gOamMatrices).cast::<u8>())
                .wrapping_offset(((matrixNum) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<i16>())
            .write(((((sinVal) as i32).wrapping_neg()) as i16));
            let __p12 = (data).wrapping_add(8).cast::<u16>();
            (__p12).write(
                (((((__p12).read()) as i32)
                    .wrapping_add(((((data).wrapping_add(6).cast::<u16>()).read()) as i32)))
                    as u16),
            );
            ((sprite).wrapping_add(34).cast::<i16>())
                .write(((((((data).wrapping_add(8).cast::<u16>()).read()) as i32) >> 8) as i16));
            if (((((data).wrapping_add(4).cast::<i16>()).read()) as i32) & 32768i32) != 0 {
                ((data).wrapping_add(2).cast::<u16>()).write(
                    ((((((data).wrapping_add(2).cast::<u16>()).read()) as i32).wrapping_sub(
                        (((((data).wrapping_add(4).cast::<i16>()).read()) as i32) & 32767i32),
                    ) & 255i32) as u16),
                );
            } else {
                ((data).wrapping_add(2).cast::<u16>()).write(
                    ((((((data).wrapping_add(2).cast::<u16>()).read()) as i32).wrapping_add(
                        (((((data).wrapping_add(4).cast::<i16>()).read()) as i32) & 32767i32),
                    ) & 255i32) as u16),
                );
            }
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                >= ((crate::c::bf_read((data).wrapping_add(14), 1, 15, false) as u16) as i32)
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(DestroyAnimSpriteAfterTimer));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedBubbleThrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(5),
            2,
            2,
            ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                as u16) as i32,
        );
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
pub(crate) unsafe extern "C" fn AnimWhirlwindLine(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut offset: u16 = 0u16;
        let mut mult: u8 = 0u8;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            InitSpritePosToAnimAttacker(sprite, 0u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 0u8);
        }
        if ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
            .read()) as i32)
            == 0i32)
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32))
            || ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as i32)
                == 1i32)
                && (((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                    == 0i32))
        {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        }
        SeekSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u8),
        );
        let __p2 = (sprite).wrapping_add(32).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(32i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(3276i16);
        offset = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
            .read()) as u16);
        mult = 12u8;
        let __p3 = (sprite).wrapping_add(36).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32)
                .wrapping_add(((mult) as i32).wrapping_mul(((offset) as i32))))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(((offset) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWhirlwindLine_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimWhirlwindLine_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
        if (({
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 6i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            StartSpriteAnim(sprite, 0u8);
        }
        if (({
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            let __t5 = ((__p4).read()).wrapping_sub(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            == (-1i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DrillPeckHitSplats(task: u8) {
    unsafe {
        let mut task = task;
        if !((crate::c::rem_i32(
            (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((task) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32),
            32i32,
        )) != 0)
        {
            let __p1 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(Sin(
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((task) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read(),
                (-13i16),
            ));
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).write(
                Cos(
                    (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((task) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read(),
                    (-13i16),
                ),
            );
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .write(1i16);
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .write(3i16);
            CreateSpriteAndAnimate(
                (&raw mut gFlashingHitSplatSpriteTemplate).cast::<u8>(),
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16),
                3u8,
            );
        }
        let __p2 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((task) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((task) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            > 255i32
        {
            DestroyAnimVisualTask(task);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBounceBallShrink(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                InitSpritePosToAnimAttacker(sprite, 1u8);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBounceBallLand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        1u8,
                    )) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_neg())
                        .wrapping_sub(32i32)) as i16),
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_add(10i32)) as i16));
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p5 = (sprite).wrapping_add(38).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_sub(10i32)) as i16));
                if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                    < (-32i32)
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDiveBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimDiveBall_Step1));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((GetAnimBattlerSpriteId(0u8)) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimDiveBall_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
                > (-32i32)
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (sprite).wrapping_add(38).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            >> 8),
                    )) as i16),
                );
            } else {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t5 = (__p4).read();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    __t5
                }) as i32)
                    > 20i32
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimDiveBall_Step2));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDiveBall_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32))
            > (-32i32)
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        }
        if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) > 0i32 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDiveWaterSplash(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut matrixNum: u32 = 0u32;
        let mut t1: i32 = 0i32;
        let mut t2: i32 = 0i32;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if !(((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0) {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            0u8,
                        )) as i16),
                    );
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                            1u8,
                        )) as i16),
                    );
                } else {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            0u8,
                        )) as i16),
                    );
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((GetBattlerSpriteCoord(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                            1u8,
                        )) as i16),
                    );
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(512i16);
                TrySetSpriteRotScale(
                    sprite,
                    0u8,
                    256i16,
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    0u16,
                );
                let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    <= 11i32
                {
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p3).write((((((__p3).read()) as i32).wrapping_sub(40i32)) as i16));
                } else {
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(40i32)) as i16));
                }
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p5).write(((__p5).read()).wrapping_add(1));
                TrySetSpriteRotScale(
                    sprite,
                    0u8,
                    256i16,
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                    0u16,
                );
                matrixNum = (crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32);
                t1 = 15616i32;
                t2 = (crate::c::div_i32(
                    t1,
                    ((((((&raw mut gOamMatrices).cast::<u8>())
                        .wrapping_offset(((matrixNum) as i32) as isize * 8))
                    .wrapping_add(6)
                    .cast::<i16>())
                    .read()) as i32),
                ))
                .wrapping_add(1i32);
                if t2 > 128i32 {
                    t2 = 128i32;
                }
                t2 = crate::c::div_i32((64i32).wrapping_sub(t2), 2i32);
                ((sprite).wrapping_add(38).cast::<i16>()).write(((t2) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    == 24i32
                {
                    ResetSpriteRotScale_PreserveAffine(sprite);
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSprayWaterDroplet(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut v1: i32 = (511i32 & ((Random2()) as i32));
        let mut v2: i32 = (127i32 & ((Random2()) as i32));
        if (crate::c::rem_i32(v1, 2i32)) != 0 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write((((736i32).wrapping_add(v1)) as i16));
        } else {
            (((sprite).wrapping_add(46)).cast::<i16>()).write((((736i32).wrapping_sub(v1)) as i16));
        }
        if (crate::c::rem_i32(v2, 2i32)) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((896i32).wrapping_add(v2)) as i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write((((896i32).wrapping_sub(v2)) as i16));
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) != 0 {
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read())
            as i32)
            == 0i32
        {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(32i32)) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i32)
                    .wrapping_add(32i32)) as i16),
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSprayWaterDroplet_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSprayWaterDroplet_Step(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
        let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p5).write((((((__p5).read()) as i32).wrapping_sub(32i32)) as i16));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 0i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        }
        if (({
            let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t7 = ((__p6).read()).wrapping_add(1);
            (__p6).write(__t7);
            __t7
        }) as i32)
            == 31i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedFlashingLight(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(64i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimUnusedFlashingLight_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedFlashingLight_Step(sprite: *mut u8) {
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
                    > 8i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    crate::c::bf_write(
                        (sprite).wrapping_add(62),
                        2,
                        1,
                        ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16)
                            as i32)
                            ^ 1i32) as u16) as i32,
                    );
                    if ((({
                        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        > 5i32)
                        && ((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0)
                    {
                        let __p6 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                DestroyAnimSprite(sprite);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSkyAttackBird(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut rotation: u16 = 0u16;
        let mut posx: i16 = ((sprite).wrapping_add(32).cast::<i16>()).read();
        let mut posy: i16 = ((sprite).wrapping_add(34).cast::<i16>()).read();
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((crate::c::div_i32(
                (((posx) as i32)
                    .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                    << 4),
                12i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::div_i32(
                (((posy) as i32)
                    .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32))
                    << 4),
                12i32,
            )) as i16),
        );
        rotation = ArcTan2Neg(
            ((((posx) as i32)
                .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                as i16),
            ((((posy) as i32)
                .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                as i16),
        );
        rotation = ((((rotation) as i32).wrapping_sub(16384i32)) as u16);
        TrySetSpriteRotScale(sprite, 1u8, 256i16, 256i16, rotation);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSkyAttackBird_Step));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimSkyAttackBird_Step(sprite: *mut u8) {
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
        if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 285i32)
            || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-45i32)))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 157i32))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-45i32))
        {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SetAttackerVisibility(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        } else {
            let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        DestroyAnimVisualTask(taskId);
    }
}
