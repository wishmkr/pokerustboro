//! Translated from `src/battle_anim_dragon.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_OutrageOverheatFire_0 sAnims_OutrageOverheatFire gOutrageFlameSpriteTemplate sAnim_DragonBreathFire_0 sAnim_DragonBreathFire_1 sAnims_DragonBreathFire sAffineAnim_DragonBreathFire_0 sAffineAnim_DragonBreathFire_1 sAffineAnims_DragonBreathFire gDragonBreathFireSpriteTemplate sAnim_DragonRageFirePlume sAnims_DragonRageFirePlume gDragonRageFirePlumeSpriteTemplate sAnim_DragonRageFire sAnims_DragonRageFire sAffineAnim_DragonRageFire_0 sAffineAnim_DragonRageFire_1 sAffineAnims_DragonRageFire gDragonRageFireSpitSpriteTemplate gDragonDanceOrbSpriteTemplate gOverheatFlameSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_dragon::*;

#[unsafe(link_section = "ewram_data")]
pub(crate) static mut sUnusedOverheatData: crate::ffi::Align4<[u8; 14]> =
    crate::ffi::Align4([0; 14]);

unsafe extern "C" {
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattlerAttacker: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gTasks: u8;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn SetAnimSpriteInitialXOffset(a0: *mut u8, a1: i16);
    fn SetSpriteCoordsToAnimAttackerCoords(a0: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateSpriteLinearAndFlicker(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimOutrageFlame(sprite: *mut u8) {
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
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_sub(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((cmd).wrapping_add(6).cast::<i16>()).write(
                ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
            ((cmd).wrapping_add(8).cast::<i16>()).write(
                ((((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
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
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearAndFlicker));
    }
}
pub(crate) unsafe extern "C" fn StartDragonFireTranslation(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        SetSpriteCoordsToAnimAttackerCoords(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_sub(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_sub(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)))
                    as i16),
            );
        } else {
            let __p5 = (sprite).wrapping_add(32).cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(((((cmd).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p6 = (sprite).wrapping_add(34).cast::<i16>();
            (__p6).write(
                (((((__p6).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p7).write(
                (((((__p7).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p8).write(
                (((((__p8).read()) as i32)
                    .wrapping_add(((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            StartSpriteAnim(sprite, 1u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    }
}
pub(crate) unsafe extern "C" fn AnimDragonRageFirePlume(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i16),
            );
        }
        SetAnimSpriteInitialXOffset(sprite, ((cmd).wrapping_add(2).cast::<i16>()).read());
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add(((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    }
}
pub(crate) unsafe extern "C" fn AnimDragonFireToTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            StartSpriteAffineAnim(sprite, 1u8);
        }
        StartDragonFireTranslation(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimDragonDanceOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut r5: u16 = 0u16;
        let mut r0: u16 = 0u16;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((cmd).cast::<i16>()).read());
        r5 = ((GetBattlerSpriteCoordAttr(((&raw mut gBattlerAttacker).cast::<u8>()).read(), 0u8))
            as u16);
        r0 = ((GetBattlerSpriteCoordAttr(((&raw mut gBattlerAttacker).cast::<u8>()).read(), 1u8))
            as u16);
        if ((r5) as i32) > ((r0) as i32) {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((crate::c::div_i32(((r5) as i32), 2i32)) as i16));
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((crate::c::div_i32(((r0) as i32), 2i32)) as i16));
        }
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
        ));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimDragonDanceOrb_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimDragonDanceOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32),
                        )
                        & 255i32) as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                ));
                ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                ));
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 5i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        <= 15i32)
                        && ((({
                            let __p4 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                            let __t5 = ((__p4).read()).wrapping_add(1);
                            (__p4).write(__t5);
                            __t5
                        }) as i32)
                            > 15i32)
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                            .write(16i16);
                    }
                }
                if (({
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t7 = ((__p6).read()).wrapping_add(1);
                    (__p6).write(__t7);
                    __t7
                }) as i32)
                    > 60i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        .wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32),
                        )
                        & 255i32) as i16),
                );
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    <= 149i32)
                    && ((({
                        let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                        let __v10 = (((((__p9).read()) as i32).wrapping_add(8i32)) as i16);
                        (__p9).write(__v10);
                        __v10
                    }) as i32)
                        > 149i32)
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(150i16);
                }
                ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                ));
                ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                ));
                if (({
                    let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    > 5i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        <= 15i32)
                        && ((({
                            let __p13 =
                                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                            let __t14 = ((__p13).read()).wrapping_add(1);
                            (__p13).write(__t14);
                            __t14
                        }) as i32)
                            > 15i32)
                    {
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                            .write(16i16);
                    }
                }
                if (({
                    let __p15 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    let __t16 = ((__p15).read()).wrapping_add(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    > 20i32
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DragonDanceWaver(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut i: u16 = 0u16;
        let mut y: u8 = 0u8;
        if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
            as i32)
            == 1i32
        {
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108884i32) as usize as *mut u16).cast::<u8>());
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
        } else {
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108888i32) as usize as *mut u16).cast::<u8>());
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                .write(((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16));
        }
        (((&raw mut scanlineParams).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(2724200449u32);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(9)).write(0u8);
        y = GetBattlerYCoordWithElevation(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
            .write(((((y) as i32).wrapping_sub(32i32)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
            .write(((((y) as i32).wrapping_add(32i32)) as i16));
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) < 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        }
        {
            i = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
            'l1: loop {
                if !(((i) as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as u16),
                    );
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
        ScanlineEffect_SetParams(
            (&raw mut scanlineParams)
                .cast::<u8>()
                .cast::<crate::c::Rec4<12>>()
                .read_unaligned(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_DragonDanceWaver_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DragonDanceWaver_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        == 3i32
                    {
                        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                UpdateDragonDanceScanlineEffect(task);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    > 60i32
                {
                    let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                UpdateDragonDanceScanlineEffect(task);
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                    if (({
                        let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
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
                UpdateDragonDanceScanlineEffect(task);
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                let __p15 = ((task).wrapping_add(8)).cast::<i16>();
                (__p15).write(((__p15).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateDragonDanceScanlineEffect(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut sineIndex: u16 =
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as u16);
        let mut i: u16 = 0u16;
        {
            i = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as u16);
            'l1: loop {
                if !(((i) as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32))
                {
                    break 'l1;
                }
                'l2: {
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20)).read())
                            as i32) as isize
                            * 1920,
                    ))
                    .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((sineIndex) as i32) as isize))
                        .read()) as i32)
                            .wrapping_mul(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32),
                            )
                            >> 7)
                            .wrapping_add(
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as i32),
                            )) as u16),
                    );
                    sineIndex = ((((sineIndex) as i32).wrapping_add(8i32) & 255i32) as u16);
                }
                i = (i).wrapping_add(1);
            }
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(9i32)
                & 255i32) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimOverheatFlame(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut i: i32 = 0i32;
        let mut yAmplitude: i32 = crate::c::div_i32(
            ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32).wrapping_mul(3i32),
            5i32,
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(Cos(
            ((cmd).wrapping_add(2).cast::<i16>()).read(),
            ((cmd).wrapping_add(4).cast::<i16>()).read(),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(Sin(
            ((cmd).wrapping_add(2).cast::<i16>()).read(),
            ((yAmplitude) as i16),
        ));
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    .wrapping_mul(((((cmd).cast::<i16>()).read()) as i32)),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    .wrapping_mul(((((cmd).cast::<i16>()).read()) as i32)),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimOverheatFlame_Step));
        {
            i = 0i32;
            'l1: loop {
                if !(i < 7i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut sUnusedOverheatData).cast::<u8>().cast::<u16>()).cast::<u16>())
                        .wrapping_offset((i) as isize))
                    .write(
                        ((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset((i) as isize))
                        .read()) as u16),
                    );
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimOverheatFlame_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                10i32,
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
                10i32,
            )) as i16),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
