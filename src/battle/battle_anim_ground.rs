//! Translated from `src/battle_anim_ground.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAffineAnim_Bonemerang sAffineAnim_SpinningBone sAffineAnims_Bonemerang sAffineAnims_SpinningBone gBonemerangSpriteTemplate gSpinningBoneSpriteTemplate gSandAttackDirtSpriteTemplate sAnim_MudSlapMud sAnims_MudSlapMud gMudSlapMudSpriteTemplate gMudsportMudSpriteTemplate gDirtPlumeSpriteTemplate gDirtMoundSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_ground::*;

unsafe extern "C" {
    static mut gAnimMovePower: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoord2(a0: u8, a1: u8) -> u8;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn Random2() -> u16;
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimBonemerangProjectile(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-40i16));
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBonemerangProjectile_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBonemerangProjectile_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
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
            (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(40i16);
            InitAnimArcTranslation(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimBonemerangProjectile_End));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBonemerangProjectile_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBoneHitProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimDirtScatter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut targetXPos: u8 = 0u8;
        let mut targetYPos: u8 = 0u8;
        let mut xOffset: i16 = 0i16;
        let mut yOffset: i16 = 0i16;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        targetXPos =
            GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8);
        targetYPos =
            GetBattlerSpriteCoord2(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8);
        xOffset = ((((Random2()) as i32) & 31i32) as i16);
        yOffset = ((((Random2()) as i32) & 31i32) as i16);
        if ((xOffset) as i32) > 16i32 {
            xOffset = (((16i32).wrapping_sub(((xOffset) as i32))) as i16);
        }
        if ((yOffset) as i32) > 16i32 {
            yOffset = (((16i32).wrapping_sub(((yOffset) as i32))) as i16);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((((targetXPos) as i32).wrapping_add(((xOffset) as i32))) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((targetYPos) as i32).wrapping_add(((yOffset) as i32))) as i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    }
}
pub(crate) unsafe extern "C" fn AnimMudSportDirt(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16).wrapping_add(1))
                as i32,
        );
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    3u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32)
                    > 0i32
                {
                    1i32
                } else {
                    (-1i32)
                }) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimMudSportDirtRising));
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimMudSportDirtFalling));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMudSportDirtRising(sprite: *mut u8) {
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
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        }
        let __p4 = (sprite).wrapping_add(34).cast::<i16>();
        (__p4).write((((((__p4).read()) as i32).wrapping_sub(4i32)) as i16));
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-4i32) {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMudSportDirtFalling(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (sprite).wrapping_add(38).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
                if ((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) >= 0i32 {
                    ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 0i32
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
                    if (({
                        let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                        let __t7 = ((__p6).read()).wrapping_add(1);
                        (__p6).write(__t7);
                        __t7
                    }) as i32)
                        == 10i32
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
pub unsafe extern "C" fn AnimTask_DigDownMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_DigBounceMovement));
        } else {
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_DigEndBounceMovementSetInvisible));
        }
        (((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read()).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DigBounceMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut y: u8 = 0u8;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                    .write(((GetAnimBattlerSpriteId(0u8)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                    ((GetBattlerSpriteBGPriorityRank(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                    == 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                        .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                        .write(((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                        .write(((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                        .write(((((&raw mut gBattle_BG2_Y).cast::<u16>()).read()) as i16));
                }
                y = GetBattlerYCoordWithElevation(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((((y) as i32).wrapping_sub(32i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((((y) as i32).wrapping_add(32i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                    < 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(0i16);
                }
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetDigScanlineEffect(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                );
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_add(6i32)
                        & 127i32) as i16),
                );
                if (({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    > 2i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        .wrapping_add(
                            (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                                .wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                        .read()) as i32)
                                        as isize,
                                ))
                            .read()) as i32)
                                >> 4),
                        )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                    == 1i32
                {
                    ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i32)
                            .wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            )) as u16),
                    );
                } else {
                    ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i32)
                            .wrapping_sub(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                    .read()) as i32),
                            )) as u16),
                    );
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    > 63i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        (((120i32).wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                                as i32),
                        )) as i16),
                    );
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                        as i32)
                        == 1i32
                    {
                        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32)
                                .wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                        .read()) as i32),
                                )) as u16),
                        );
                    } else {
                        ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32)
                                .wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                        .read()) as i32),
                                )) as u16),
                        );
                    }
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(
                        (((272i32).wrapping_sub(
                            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read()) as i32),
                        )) as i16),
                    );
                    let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p7).write(((__p7).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                (__p8).write(((__p8).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                DestroyAnimVisualTask(taskId);
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DigEndBounceMovementSetInvisible(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
        .write(0i16);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write(0i16);
        if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
            as i32)
            == 1i32
        {
            ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        } else {
            ((&raw mut gBattle_BG2_Y).cast::<u16>()).write(0u16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DigUpMovement(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_DigSetVisibleUnderground));
        } else {
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_DigRiseUpFromHole));
        }
        (((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read()).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DigSetVisibleUnderground(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                    .write(((GetAnimBattlerSpriteId(0u8)) as i16));
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (0u16) as i32,
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    (((160i32).wrapping_sub(
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(34)
                        .cast::<i16>())
                        .read()) as i32),
                    )) as i16),
                );
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DigRiseUpFromHole(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut y: u8 = 0u8;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                    .write(((GetAnimBattlerSpriteId(0u8)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                    ((GetBattlerSpriteBGPriorityRank(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                    == 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                        .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                        .write(((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16));
                }
                y = GetBattlerYCoordWithElevation(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((((y) as i32).wrapping_sub(32i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((((y) as i32).wrapping_add(32i32)) as i16));
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                SetDigScanlineEffect(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as u8),
                    0i16,
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                );
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(96i16);
                let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_sub(8i32)) as i16));
                if ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .read()) as i32)
                    == 0i32
                {
                    (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                    let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetDigScanlineEffect(useBG1: u8, y: i16, endY: i16) {
    unsafe {
        let mut useBG1 = useBG1;
        let mut y = y;
        let mut endY = endY;
        let mut bgX: i16 = 0i16;
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        if ((useBG1) as i32) == 1i32 {
            bgX = ((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16);
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108884i32) as usize as *mut u16).cast::<u8>());
        } else {
            bgX = ((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16);
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108888i32) as usize as *mut u16).cast::<u8>());
        }
        if ((y) as i32) < 0i32 {
            y = 0i16;
        }
        'l1: loop {
            if !(((y) as i32) < ((endY) as i32)) {
                break 'l1;
            }
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                .wrapping_offset(((y) as i32) as isize))
            .write(((bgX) as u16));
            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .wrapping_offset(((y) as i32) as isize))
            .write(((bgX) as u16));
            y = (y).wrapping_add(1);
        }
        'l2: loop {
            if !(((y) as i32) < 160i32) {
                break 'l2;
            }
            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                .wrapping_offset(((y) as i32) as isize))
            .write(((((bgX) as i32).wrapping_add(240i32)) as u16));
            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                .cast::<u16>())
            .wrapping_offset(((y) as i32) as isize))
            .write(((((bgX) as i32).wrapping_add(240i32)) as u16));
            y = (y).wrapping_add(1);
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
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimDirtPlumeParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u16 = 0u16;
        let mut xOffset: i16 = 0i16;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as u16);
        } else {
            battler = ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as u16);
        }
        xOffset = 24i16;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read())
            as i32)
            == 1i32
        {
            xOffset = ((((xOffset) as i32).wrapping_mul((-1i32))) as i16);
            let __p1 =
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((battler) as u8), 2u8)) as i32)
                .wrapping_add(((xOffset) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerYCoordWithElevation(((battler) as u8))) as i32).wrapping_add(30i32))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimDirtPlumeParticle_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimDirtPlumeParticle_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDigDirtMound(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            (((((GetBattlerSpriteCoord(battler, 0u8)) as i32).wrapping_sub(16i32)).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_mul(32i32),
            )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerYCoordWithElevation(battler)) as i32).wrapping_add(32i32)) as i16),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(8i32),
                )) as u16) as i32,
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_HorizontalShake(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read())
            as i32)
            != 0i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write({
                let __v1 = ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_add(3i32)) as i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(__v1);
                __v1
            });
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write({
                let __v2 = (((crate::c::div_i32(
                    ((((&raw mut gAnimMovePower).cast::<u16>()).read()) as i32),
                    10i32,
                ))
                .wrapping_add(3i32)) as i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(__v2);
                __v2
            });
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        'l1: {
            let __sw3 =
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32);
            let __matched = __sw3 == 5i32 || __sw3 == 4i32;
            if __sw3 == 5i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                    .write(((((&raw mut gBattle_BG3_X).cast::<u16>()).read()) as i16));
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(AnimTask_ShakePlatforms));
                break 'l1;
            }
            if __sw3 == 4i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(0i16);
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 4i32) {
                            break 'l2;
                        }
                        'l3: {
                            if (IsBattlerSpriteVisible(((i) as u8))) != 0 {
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                                    ((9i32).wrapping_add(
                                        ((((((task).wrapping_add(8)).cast::<i16>())
                                            .wrapping_offset(13))
                                        .read()) as i32),
                                    )) as isize,
                                ))
                                .write(
                                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i16),
                                );
                                let __p4 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13);
                                (__p4).write(((__p4).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(AnimTask_ShakeBattlers));
                break 'l1;
            }
            if !__matched {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(
                    ((GetAnimBattlerSpriteId(
                        (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
                    )) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read()) as i32)
                    == 255i32
                {
                    DestroyAnimVisualTask(taskId);
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(1i16);
                    ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(AnimTask_ShakeBattlers));
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ShakePlatforms(taskId: u8) {
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
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        == 0i32
                    {
                        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32)
                                .wrapping_add(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(15))
                                    .read()) as i32),
                                )) as u16),
                        );
                    } else {
                        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32)
                                .wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(15))
                                    .read()) as i32),
                                )) as u16),
                        );
                    }
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                        let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        == 0i32
                    {
                        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32)
                                .wrapping_add(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(14))
                                    .read()) as i32),
                                )) as u16),
                        );
                    } else {
                        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32)
                                .wrapping_sub(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(14))
                                    .read()) as i32),
                                )) as u16),
                        );
                    }
                    if (({
                        let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t11 = ((__p10).read()).wrapping_add(1);
                        (__p10).write(__t11);
                        __t11
                    }) as i32)
                        == 4i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        if (({
                            let __p12 =
                                (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                            let __t13 = ((__p12).read()).wrapping_sub(1);
                            (__p12).write(__t13);
                            __t13
                        }) as i32)
                            == 0i32
                        {
                            let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                            (__p14).write(((__p14).read()).wrapping_add(1));
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                        as u16),
                );
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ShakeBattlers(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
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
                    SetBattlersXOffsetForShake(task);
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                        (__p6).write(((__p6).read()).wrapping_sub(1));
                        let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t9 = ((__p8).read()).wrapping_add(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    SetBattlersXOffsetForShake(task);
                    if (({
                        let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t11 = ((__p10).read()).wrapping_add(1);
                        (__p10).write(__t11);
                        __t11
                    }) as i32)
                        == 4i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        if (({
                            let __p12 =
                                (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                            let __t13 = ((__p12).read()).wrapping_sub(1);
                            (__p12).write(__t13);
                            __t13
                        }) as i32)
                            == 0i32
                        {
                            let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                            (__p14).write(((__p14).read()).wrapping_add(1));
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                                .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                ((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(((9i32).wrapping_add(((i) as i32))) as isize))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(36)
                            .cast::<i16>())
                            .write(0i16);
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetBattlersXOffsetForShake(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut i: u16 = 0u16;
        let mut xOffset: i16 = 0i16;
        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) & 1i32)
            == 0i32
        {
            xOffset = (((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32),
                2i32,
            ))
            .wrapping_add(
                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                    & 1i32),
            )) as i16);
        } else {
            xOffset = (((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32),
                2i32,
            ))
            .wrapping_neg()) as i16);
        }
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32)
                    < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>())
                            .wrapping_offset(((9i32).wrapping_add(((i) as i32))) as isize))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .write(xOffset);
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_IsPowerOver99(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .write(((((((&raw mut gAnimMovePower).cast::<u16>()).read()) as i32) > 99i32) as i16));
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_PositionFissureBgOnBattler(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut newTask: *mut u8 = core::ptr::null_mut();
        let mut battler: u8 = ((if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
            .read()) as i32)
            & 1i32)
            != 0
        {
            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
        } else {
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
        }) as u8);
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) > 1i32 {
            battler = ((((battler) as i32) ^ 2i32) as u8);
        }
        newTask = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((CreateTask(
                Some(WaitForFissureCompletion),
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as u8),
            )) as i32) as isize
                * 40,
        );
        ((((newTask).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(
            (((32i32).wrapping_sub(((GetBattlerSpriteCoord(battler, 2u8)) as i32)) & 511i32)
                as i16),
        );
        ((((newTask).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
            (((64i32).wrapping_sub(((GetBattlerSpriteCoord(battler, 3u8)) as i32)) & 255i32)
                as i16),
        );
        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
            ((((((newTask).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
        );
        ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(
            ((((((newTask).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
        );
        ((((newTask).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn WaitForFissureCompletion(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            ((&raw mut gBattle_BG3_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
            DestroyTask(taskId);
        } else {
            ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as u16),
            );
            ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as u16),
            );
        }
    }
}
