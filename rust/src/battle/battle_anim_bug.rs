//! Translated from `src/battle_anim_bug.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAffineAnim_MegahornHorn_0 sAffineAnim_MegahornHorn_1 sAffineAnim_MegahornHorn_2 sAffineAnims_MegahornHorn gMegahornHornSpriteTemplate sAffineAnim_LeechLifeNeedle_0 sAffineAnim_LeechLifeNeedle_1 sAffineAnim_LeechLifeNeedle_2 sAffineAnims_LeechLifeNeedle gLeechLifeNeedleSpriteTemplate gWebThreadSpriteTemplate gStringWrapSpriteTemplate sAffineAnim_SpiderWeb sAffineAnims_SpiderWeb gSpiderWebSpriteTemplate gLinearStingerSpriteTemplate gPinMissileSpriteTemplate gIcicleSpearSpriteTemplate sAffineAnim_TailGlowOrb sAffineAnims_TailGlowOrb gTailGlowOrbSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_bug::*;

unsafe extern "C" {
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ArcTan2Neg(a0: i16, a1: i16) -> u16;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoord2(a0: u8, a1: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitAnimLinearTranslationWithSpeed(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn IsContest() -> u8;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn TrySetSpriteRotScale(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: u16);
}

pub(crate) unsafe extern "C" fn AnimMegahornHorn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (IsContest()) != 0 {
            StartSpriteAffineAnim(sprite, 2u8);
            ((cmd).wrapping_add(4).cast::<i16>()).write(
                ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
            ((cmd).cast::<i16>())
                .write(((((((cmd).cast::<i16>()).read()) as i32).wrapping_neg()) as i16));
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32
            {
                StartSpriteAffineAnim(sprite, 1u8);
                ((cmd).wrapping_add(2).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
                ((cmd).wrapping_add(4).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
                ((cmd).wrapping_add(6).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
                ((cmd).cast::<i16>())
                    .write(((((((cmd).cast::<i16>()).read()) as i32).wrapping_neg()) as i16));
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((((cmd).cast::<i16>()).read()) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimLeechLifeNeedle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (IsContest()) != 0 {
            ((cmd).cast::<i16>())
                .write(((((((cmd).cast::<i16>()).read()) as i32).wrapping_neg()) as i16));
            StartSpriteAffineAnim(sprite, 2u8);
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32
            {
                ((cmd).wrapping_add(2).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
                ((cmd).cast::<i16>())
                    .write(((((((cmd).cast::<i16>()).read()) as i32).wrapping_neg()) as i16));
            }
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((((cmd).cast::<i16>()).read()) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
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
pub(crate) unsafe extern "C" fn AnimTranslateWebThread(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (IsContest()) != 0 {
            let __p1 = (cmd).wrapping_add(4).cast::<i16>();
            (__p1).write(((crate::c::div_i32((((__p1).read()) as i32), 2i32)) as i16));
        }
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        if !((((cmd).wrapping_add(8).cast::<i16>()).read()) != 0) {
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
        InitAnimLinearTranslationWithSpeed(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTranslateWebThread_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTranslateWebThread_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            DestroyAnimSprite(sprite);
            return;
        }
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_add(13i32)
                & 255i32) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimStringWrap(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        SetAverageBattlerPositions(
            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
            0u8,
            (sprite).wrapping_add(32).cast::<i16>(),
            (sprite).wrapping_add(34).cast::<i16>(),
        );
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
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
        let __p3 = (sprite).wrapping_add(34).cast::<i16>();
        (__p3).write(
            (((((__p3).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(8i32)) as i16));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimStringWrap_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimStringWrap_Step(sprite: *mut u8) {
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
pub(crate) unsafe extern "C" fn AnimSpiderWeb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 16u16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(16i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSpiderWeb_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSpiderWeb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            < 20i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if ((({
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t3 = (__p2).read();
                (__p2).write(((__p2).read()).wrapping_add(1));
                __t3
            }) as i32)
                & 1i32)
                != 0
            {
                let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_sub(1));
                SetGpuReg(
                    82u8,
                    ((((16i32).wrapping_sub(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    ) << 8)
                        | (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32))
                        as u16),
                );
                if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimSpiderWeb_End));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpiderWeb_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimTranslateStinger(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut lVarX: i16 = 0i16;
        let mut lVarY: i16 = 0i16;
        let mut rot: u16 = 0u16;
        if (IsContest()) != 0 {
            ((cmd).wrapping_add(4).cast::<i16>()).write(
                ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
        } else {
            if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
                ((cmd).wrapping_add(4).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
                ((cmd).wrapping_add(2).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
                ((cmd).wrapping_add(6).cast::<i16>()).write(
                    ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
            }
        }
        if (!((IsContest()) != 0))
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32))
        {
            if (((GetBattlerPosition(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                == 0i32)
                || (((GetBattlerPosition(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as i32)
                    == 1i32)
            {
                let __p1 = (cmd).wrapping_add(4).cast::<i16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
                let __p2 = (cmd).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
            }
        }
        InitSpritePosToAnimAttacker(sprite, 1u8);
        lVarX = ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
            as i32)
            .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
            as i16);
        lVarY = ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
            as i32)
            .wrapping_add(((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)))
            as i16);
        rot = ArcTan2Neg(
            ((((lVarX) as i32)
                .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                as i16),
            ((((lVarY) as i32)
                .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                as i16),
        );
        rot = ((((rot) as i32).wrapping_sub(16384i32)) as u16);
        TrySetSpriteRotScale(sprite, 0u8, 256i16, 256i16, rot);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(lVarX);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(lVarY);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimMissileArc(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        InitSpritePosToAnimAttacker(sprite, 1u8);
        if (GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) != 0 {
            ((cmd).wrapping_add(4).cast::<i16>()).write(
                ((((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMissileArc_Step));
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn AnimMissileArc_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        } else {
            let mut tempData = crate::ffi::Align4([0u8; 16]);
            let mut x2: i16 = 0i16;
            let mut y2: i16 = 0i16;
            let mut i: i32 = 0i32;
            {
                i = 0i32;
                'l1: loop {
                    if !(i < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        (((&raw mut tempData).cast::<i16>()).wrapping_offset((i) as isize)).write(
                            ((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            x2 = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                as i16);
            y2 = ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                as i16);
            if !((TranslateAnimHorizontalArc(sprite)) != 0) {
                let mut rotation: u16 = ArcTan2Neg(
                    (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    .wrapping_sub(((x2) as i32))) as i16),
                    (((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    .wrapping_sub(((y2) as i32))) as i16),
                );
                rotation = ((((rotation) as i32).wrapping_sub(16384i32)) as u16);
                TrySetSpriteRotScale(sprite, 0u8, 256i16, 256i16, rotation);
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 8i32) {
                            break 'l3;
                        }
                        'l4: {
                            ((((sprite).wrapping_add(46)).cast::<i16>())
                                .wrapping_offset((i) as isize))
                            .write(
                                (((&raw mut tempData).cast::<i16>()).wrapping_offset((i) as isize))
                                    .read(),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTailGlowOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    3u8,
                )) as i32)
                    .wrapping_add(18i32)) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i32)
                    .wrapping_add(18i32)) as i16),
            );
        }
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
