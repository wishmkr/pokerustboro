//! Translated from `src/battle_anim_dark.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnusedBagStealSpriteTemplate sAffineAnim_Bite_0 sAffineAnim_Bite_1 sAffineAnim_Bite_2 sAffineAnim_Bite_3 sAffineAnim_Bite_4 sAffineAnim_Bite_5 sAffineAnim_Bite_6 sAffineAnim_Bite_7 gAffineAnims_Bite gSharpTeethSpriteTemplate gClampJawSpriteTemplate sAffineAnim_TearDrop_0 sAffineAnim_TearDrop_1 sAffineAnims_TearDrop gTearDropSpriteTemplate sAnim_ClawSlash_0 sAnim_ClawSlash_1 sAnims_ClawSlash gClawSlashSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_dark::*;

unsafe extern "C" {
    static mut gAnimMoveTurn: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattle_BG2_Y: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gContestResources: u8;
    static mut gEnemyParty: u8;
    static mut gMetalShineGfx: u8;
    static mut gMetalShinePalette: u8;
    static mut gMetalShineTilemap: u8;
    static mut gPlayerParty: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemap(a0: u32, a1: *mut u8);
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn ClearBattleAnimBg(a0: u32);
    fn CreateInvisibleSpriteCopy(a0: i32, a1: u8, a2: i32) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn FillPalette(a0: u16, a1: u16, a2: u16);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetGpuReg(a0: u8) -> u16;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitSpriteDataForLinearTranslation(a0: *mut u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn MoveBattlerSpriteToBG(a0: u8, a1: u8, a2: u8);
    fn ResetBattleAnimBg(a0: u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetGrayscaleOrOriginalPalette(a0: u16, a1: u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AttackerFadeToInvisible(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: i32 = 0i32;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((cmd).cast::<i16>()).read());
        battler = ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(16i16);
        SetGpuReg(82u8, 16u16);
        if ((GetBattlerSpriteBGPriorityRank(((battler) as u8))) as i32) == 1i32 {
            SetGpuReg(80u8, 16194u16);
        } else {
            SetGpuReg(80u8, 16196u16);
        }
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_AttackerFadeToInvisible_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AttackerFadeToInvisible_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut blendA: u8 = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            >> 8) as u8);
        let mut blendB: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8) as i32)
        {
            blendA = (blendA).wrapping_add(1);
            blendB = (blendB).wrapping_sub(1);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((((((blendA) as i32) << 8) | ((blendB) as i32)) as i16));
            SetGpuReg(
                82u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            if ((blendA) as i32) == 16i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                DestroyAnimVisualTask(taskId);
            }
        } else {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_AttackerFadeFromInvisible(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((cmd).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(4096i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_AttackerFadeFromInvisible_Step));
        SetGpuReg(
            82u8,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimTask_AttackerFadeFromInvisible_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut blendA: u8 = ((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            >> 8) as u8);
        let mut blendB: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as u8);
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u8) as i32)
        {
            blendA = (blendA).wrapping_sub(1);
            blendB = (blendB).wrapping_add(1);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((((((blendA) as i32) << 8) | ((blendB) as i32)) as i16));
            SetGpuReg(
                82u8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            if ((blendA) as i32) == 0i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimVisualTask(taskId);
            }
        } else {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_InitAttackerFadeFromInvisible(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        SetGpuReg(82u8, 4096u16);
        if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
            as i32)
            == 1i32
        {
            SetGpuReg(80u8, 16194u16);
        } else {
            SetGpuReg(80u8, 16196u16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedBagSteal(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(126i16);
        InitSpriteDataForLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_neg()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                .wrapping_neg()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write((-40i16));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimUnusedBagSteal_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimUnusedBagSteal_Step(sprite: *mut u8) {
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
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 8) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 0i32
        {
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
            let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p5).write(((__p5).read()).wrapping_sub(1));
        }
        let __p6 = (sprite).wrapping_add(38).cast::<i16>();
        (__p6).write(
            (((((__p6).read()) as i32).wrapping_add(
                ((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
                )) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(3i32)
                & 255i32) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            > 127i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p7).write((((((__p7).read()) as i32).wrapping_add(20i32)) as i16));
            let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p8).write(((__p8).read()).wrapping_add(1));
        }
        if (({
            let __p9 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t10 = ((__p9).read()).wrapping_sub(1);
            (__p9).write(__t10);
            __t10
        }) as i32)
            == 0i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBite(sprite: *mut u8) {
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
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(4).cast::<i16>()).read()) as u8),
        );
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBite_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimBite_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >> 8) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimBite_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBite_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                >> 8) as i16),
        );
        if (({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t4 = ((__p3).read()).wrapping_sub(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 0i32
        {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTearDrop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: u8 = 0u8;
        let mut xOffset: i8 = 0i8;
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        xOffset = 20i8;
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(4i32)) as u16) as i32,
        );
        'l1: {
            let __sw1 = ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 5u8)) as i32).wrapping_sub(8i32))
                        as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 2u8)) as i32).wrapping_add(8i32))
                        as i16),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 5u8)) as i32).wrapping_sub(14i32))
                        as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 2u8)) as i32).wrapping_add(16i32))
                        as i16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 4u8)) as i32).wrapping_add(8i32))
                        as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 2u8)) as i32).wrapping_add(8i32))
                        as i16),
                );
                StartSpriteAffineAnim(sprite, 1u8);
                xOffset = (-20i8);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 4u8)) as i32).wrapping_add(14i32))
                        as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((GetBattlerSpriteCoordAttr(battler, 2u8)) as i32).wrapping_add(16i32))
                        as i16),
                );
                StartSpriteAffineAnim(sprite, 1u8);
                xOffset = (-20i8);
                break 'l1;
            }
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(32i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                .wrapping_add(((xOffset) as i32))) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(12i32))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-12i16));
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTearDrop_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTearDrop_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            DestroySpriteAndMatrix(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoveAttackerMementoShadow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut i: u16 = 0u16;
        let mut pos: u8 = 0u8;
        let mut var0: i32 = 0i32;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(31i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((GetBattlerSpriteCoordAttr(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                2u8,
            )) as i32)
                .wrapping_sub(7i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
            .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                .wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
                )
                << 8) as i16),
        );
        pos = GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
            .write(((((pos) as i32).wrapping_sub(32i32)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((((pos) as i32).wrapping_add(32i32)) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write((-12i16));
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write((-64i16));
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) == 1i32 {
            GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                .write(((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16));
            SetGpuReg(80u8, 16194u16);
            FillPalette(
                0u16,
                (((0i32).wrapping_add(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108886i32) as usize as *mut u16).cast::<u8>());
            var0 = 2i32;
            if !((IsContest()) != 0) {
                let __p1 = (&raw mut gBattle_BG2_X).cast::<u16>();
                (__p1).write((((((__p1).read()) as i32).wrapping_add(240i32)) as u16));
            }
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                .write(((((&raw mut gBattle_BG2_Y).cast::<u16>()).read()) as i16));
            SetGpuReg(80u8, 16196u16);
            FillPalette(0u16, 144u16, 32u16);
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108890i32) as usize as *mut u16).cast::<u8>());
            var0 = 4i32;
            if !((IsContest()) != 0) {
                let __p2 = (&raw mut gBattle_BG1_X).cast::<u16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_add(240i32)) as u16));
            }
        }
        (((&raw mut scanlineParams).cast::<u8>())
            .wrapping_add(4)
            .cast::<u32>())
        .write(2724200449u32);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(8)).write(1u8);
        (((&raw mut scanlineParams).cast::<u8>()).wrapping_add(9)).write(0u8);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(16i16);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        SetAllBattlersSpritePriority(3u8);
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 112i32) {
                    break 'l1;
                }
                'l2: {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as u16),
                    );
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
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
        SetGpuReg(74u8, ((16128i32 | (var0 ^ 63i32)) as u16));
        SetGpuReg(72u8, 16191u16);
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32))
                as u16),
        );
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(160u16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_MoveAttackerMementoShadow_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MoveAttackerMementoShadow_Step(taskId: u8) {
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
                    if ((({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        & 1i32)
                        != 0
                    {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            as i32)
                            != 12i32
                        {
                            let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                            (__p6).write(((__p6).read()).wrapping_add(1));
                        }
                    } else {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            != 8i32
                        {
                            let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                            (__p7).write(((__p7).read()).wrapping_sub(1));
                        }
                    }
                    SetGpuReg(
                        82u8,
                        (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            << 8)
                            | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                .read()) as i32)) as u16),
                    );
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                        as i32)
                        == 12i32)
                        && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            == 8i32)
                    {
                        let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p9).write((((((__p9).read()) as i32).wrapping_sub(8i32)) as i16));
                DoMementoShadowEffect(task);
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                {
                    let __p10 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p10).write(((__p10).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p11).write((((((__p11).read()) as i32).wrapping_sub(8i32)) as i16));
                DoMementoShadowEffect(task);
                let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                (__p12).write((((((__p12).read()) as i32).wrapping_add(4i32)) as i16));
                let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                (__p13).write((((((__p13).read()) as i32).wrapping_sub(4i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read(),
                    );
                }
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32)) as u16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32)
                    == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                        as i32)
                {
                    let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoveTargetMementoShadow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        let mut x: u8 = 0u8;
        let mut i: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((IsContest()) as i32) == 1i32 {
                    ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                    ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                    SetGpuReg(72u8, 16191u16);
                    SetGpuReg(74u8, 16191u16);
                    DestroyAnimVisualTask(taskId);
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                        ((GetBattlerSpriteBGPriorityRank(
                            ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        )) as i16),
                    );
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        == 1i32
                    {
                        SetGpuReg(80u8, 16194u16);
                        let __p2 = (&raw mut gBattle_BG2_X).cast::<u16>();
                        (__p2).write((((((__p2).read()) as i32).wrapping_add(240i32)) as u16));
                    } else {
                        SetGpuReg(80u8, 16196u16);
                        let __p3 = (&raw mut gBattle_BG1_X).cast::<u16>();
                        (__p3).write((((((__p3).read()) as i32).wrapping_add(240i32)) as u16));
                    }
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 1i32
                {
                    GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                        .write(((((&raw mut gBattle_BG1_Y).cast::<u16>()).read()) as i16));
                    FillPalette(
                        0u16,
                        (((0i32).wrapping_add(
                            (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                                .wrapping_mul(16i32),
                        )) as u16),
                        32u16,
                    );
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                        .write(((((&raw mut gBattle_BG2_Y).cast::<u16>()).read()) as i16));
                    FillPalette(0u16, 144u16, 32u16);
                }
                SetAllBattlersSpritePriority(3u8);
                let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
                    ((((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        1u8,
                    )) as i32)
                        .wrapping_add(31i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i32)
                        .wrapping_sub(7i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        .wrapping_sub(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                                as i32),
                        )
                        << 8) as i16),
                );
                x = GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((((x) as i32).wrapping_sub(4i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((((x) as i32).wrapping_add(4i32)) as i16));
                if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write((-12i16));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write((-64i16));
                }
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read());
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(12i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(8i16);
                let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 1i32
                {
                    (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                        .write(((67108886i32) as usize as *mut u16).cast::<u8>());
                } else {
                    (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                        .write(((67108890i32) as usize as *mut u16).cast::<u8>());
                }
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 112i32) {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32)
                                    .wrapping_add((159i32).wrapping_sub(((i) as i32))))
                                    as u16),
                            );
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(1920))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read()) as i32)
                                    .wrapping_add((159i32).wrapping_sub(((i) as i32))))
                                    as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
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
                let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 1i32
                {
                    SetGpuReg(74u8, 16189u16);
                } else {
                    SetGpuReg(74u8, 16187u16);
                }
                SetGpuReg(72u8, 16191u16);
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32)) as u16),
                );
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(160u16);
                (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                SetGpuReg(82u8, 2060u16);
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(AnimTask_MoveTargetMementoShadow_Step));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MoveTargetMementoShadow_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read(),
                    );
                }
                DoMementoShadowEffect(task);
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    .wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32),
                    )
                    < 64i32
                {
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14);
                    (__p4).write((((((__p4).read()) as i32).wrapping_sub(4i32)) as i16));
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                    (__p5).write((((((__p5).read()) as i32).wrapping_add(4i32)) as i16));
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(1i16);
                }
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32)) as u16),
                );
                let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(8i32)) as i16));
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read(),
                    );
                }
                DoMementoShadowEffect(task);
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    == ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32))
                    && ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) != 0)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
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
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            != 0
                        {
                            let __p11 =
                                (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                            (__p11).write(((__p11).read()).wrapping_sub(1));
                        }
                    } else {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            < 16i32
                        {
                            let __p12 =
                                (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                            (__p12).write(((__p12).read()).wrapping_add(1));
                        }
                    }
                    SetGpuReg(
                        82u8,
                        (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            << 8)
                            | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                .read()) as i32)) as u16),
                    );
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                        as i32)
                        == 0i32)
                        && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as i32)
                            == 16i32)
                    {
                        let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p13).write(((__p13).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                (__p14).write(((__p14).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                SetGpuReg(72u8, 16191u16);
                SetGpuReg(74u8, 16191u16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn DoMementoShadowEffect(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut var0: i32 = 0i32;
        let mut var1: i32 = 0i32;
        let mut var2: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut var4: i32 = 0i32;
        var2 = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            .wrapping_sub(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16);
        if ((var2) as i32) != 0i32 {
            var0 = crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32),
                ((var2) as i32),
            );
            var1 = (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                as i32)
                << 8);
            {
                i = 0i16;
                'l1: loop {
                    if !(((i) as i32)
                        < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
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
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                .read()) as i32)
                                .wrapping_sub(((i) as i32).wrapping_sub(159i32)))
                                as u16),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            {
                i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read();
                'l3: loop {
                    if !(((i) as i32)
                        <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32))
                    {
                        break 'l3;
                    }
                    'l4: {
                        if ((i) as i32) >= 0i32 {
                            let mut var3: i16 = (((var1 >> 8).wrapping_sub(((i) as i32))) as i16);
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(
                                ((((var3) as i32).wrapping_add(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(10))
                                    .read()) as i32),
                                )) as u16),
                            );
                        }
                        var1 = (var1).wrapping_add(var0);
                    }
                    i = (i).wrapping_add(1);
                }
            }
            var4 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                .wrapping_sub(((i) as i32).wrapping_sub(159i32));
            {
                i = i;
                'l5: loop {
                    if !(((i) as i32)
                        < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32))
                    {
                        break 'l5;
                    }
                    'l6: {
                        if ((i) as i32) >= 0i32 {
                            (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                .wrapping_offset(
                                    (((((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(20))
                                        .read()) as i32)
                                        as isize
                                        * 1920,
                                ))
                            .cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                            .write(((var4) as u16));
                            var4 = (var4).wrapping_sub(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            var4 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                .wrapping_add(159i32);
            {
                i = 0i16;
                'l7: loop {
                    if !(((i) as i32) < 112i32) {
                        break 'l7;
                    }
                    'l8: {
                        ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                            .wrapping_offset(((i) as i32) as isize))
                        .write(((var4) as u16));
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(((var4) as u16));
                        var4 = (var4).wrapping_sub(1);
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SetAllBattlersSpritePriority(priority: u8) {
    unsafe {
        let mut priority = priority;
        let mut i: u16 = 0u16;
        {
            i = 0u16;
            'l1: loop {
                if !(((i) as i32) < 4i32) {
                    break 'l1;
                }
                'l2: {
                    let mut spriteId: u8 = GetAnimBattlerSpriteId(((i) as u8));
                    if ((spriteId) as i32) != 255i32 {
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(5),
                            2,
                            2,
                            ((priority) as u16) as i32,
                        );
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_InitMementoShadow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut toBG2: u8 = ((if (((GetBattlerSpriteBGPriorityRank(
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
        )) as i32)
            ^ 1i32)
            != 0
        {
            1i32
        } else {
            0i32
        }) as u8);
        MoveBattlerSpriteToBG(
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
            toBG2,
            1u8,
        );
        crate::c::bf_write(
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
            (0u16) as i32,
        );
        if (IsBattlerSpriteVisible(
            ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
        )) != 0
        {
            MoveBattlerSpriteToBG(
                ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
                ((((toBG2) as i32) ^ 1i32) as u8),
                1u8,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                            as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MementoHandleBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut toBG2: u8 = ((if (((GetBattlerSpriteBGPriorityRank(
            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
        )) as i32)
            ^ 1i32)
            != 0
        {
            1i32
        } else {
            0i32
        }) as u8);
        ResetBattleAnimBg(toBG2);
        if (IsBattlerSpriteVisible(
            ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32) as u8),
        )) != 0
        {
            ResetBattleAnimBg(((((toBG2) as i32) ^ 1i32) as u8));
        }
        DestroyAnimVisualTask(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimClawSlash(sprite: *mut u8) {
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
        StartSpriteAnim(
            sprite,
            ((((cmd).wrapping_add(4).cast::<i16>()).read()) as u8),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MetallicShine(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut species: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut newSpriteId: u8 = 0u8;
        let mut paletteNum: u16 = 0u16;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut priorityChanged: u32 = 0u32;
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16189u16);
        SetGpuRegBits(0u8, 32768u16);
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 3080u16);
        SetAnimBgAttribute(1u8, 4u8, 0u8);
        SetAnimBgAttribute(1u8, 0u8, 0u8);
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 3u8, 1u8);
        }
        if ((IsDoubleBattle()) != 0) && (!((IsContest()) != 0)) {
            if (((GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 3i32)
                || (((GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as i32)
                    == 0i32)
            {
                if ((IsBattlerSpriteVisible(
                    ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
                        as u8),
                )) as i32)
                    == 1i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    ^ 2i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            false,
                        ) as u16)
                            .wrapping_sub(1)) as i32,
                    );
                    SetAnimBgAttribute(1u8, 4u8, 1u8);
                    priorityChanged = 1u32;
                }
            }
        }
        if (IsContest()) != 0 {
            species = ((((((&raw mut gContestResources).cast::<*mut u8>()).read())
                .wrapping_add(24)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read();
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                species = ((GetMonData2(
                    ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            } else {
                species = ((GetMonData2(
                    ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                        ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    as isize,
                            ))
                        .read()) as i32) as isize
                            * 100,
                    ),
                    11i32,
                )) as u16);
            }
        }
        spriteId = GetAnimBattlerSpriteId(0u8);
        newSpriteId = CreateInvisibleSpriteCopy(
            ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32),
            spriteId,
            ((species) as i32),
        );
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        AnimLoadCompressedBgTilemap(
            (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
            (((&raw mut gMetalShineTilemap).cast::<u32>()).cast::<u32>()).cast::<u8>(),
        );
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gMetalShineGfx).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        LoadCompressedPalette(
            ((&raw mut gMetalShinePalette).cast::<u32>()).cast::<u32>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(
            (((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_neg())
            .wrapping_add(96i32)) as u16),
        );
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(
            (((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_neg())
            .wrapping_add(32i32)) as u16),
        );
        paletteNum = (((16i32).wrapping_add(
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
                4,
                4,
                false,
            ) as u16) as i32),
        )) as u16);
        if ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) == 0i32 {
            SetGrayscaleOrOriginalPalette(paletteNum, 0u8);
        } else {
            BlendPalette(
                (((0i32).wrapping_add(((paletteNum) as i32).wrapping_mul(16i32))) as u16),
                16u16,
                11u8,
                ((((cmd).wrapping_add(4).cast::<i16>()).read()) as u16),
            );
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((newSpriteId) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((cmd).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((priorityChanged) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_MetallicShine_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MetallicShine_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut paletteNum: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
        let __p2 = (&raw mut gBattle_BG1_X).cast::<u16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(4i32)) as u16));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 128i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            let __p3 = (&raw mut gBattle_BG1_X).cast::<u16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(128i32)) as u16));
            let __p4 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11);
            (__p4).write(((__p4).read()).wrapping_add(1));
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11))
            .read()) as i32)
                == 2i32
            {
                spriteId = GetAnimBattlerSpriteId(0u8);
                paletteNum = (((16i32).wrapping_add(
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(5),
                        4,
                        4,
                        false,
                    ) as u16) as i32),
                )) as u16);
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    == 0i32
                {
                    SetGrayscaleOrOriginalPalette(paletteNum, 1u8);
                }
                DestroySprite(
                    ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .read()) as i32) as isize
                            * 68,
                    ),
                );
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                );
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as i32)
                    == 1i32
                {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32)
                                    ^ 2i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(5),
                        2,
                        2,
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                        as i32)
                                        ^ 2i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            false,
                        ) as u16)
                            .wrapping_add(1)) as i32,
                    );
                }
            } else {
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11))
                .read()) as i32)
                    == 3i32
                {
                    ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
                    ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
                    SetGpuReg(72u8, 16191u16);
                    SetGpuReg(74u8, 16191u16);
                    if !((IsContest()) != 0) {
                        SetAnimBgAttribute(1u8, 3u8, 0u8);
                    }
                    SetGpuReg(0u8, ((((GetGpuReg(0u8)) as i32) ^ 32768i32) as u16));
                    SetGpuReg(80u8, 0u16);
                    SetGpuReg(82u8, 0u16);
                    DestroyAnimVisualTask(taskId);
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SetGrayscaleOrOriginalPal(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut spriteId: u8 = 0u8;
        let mut battler: u8 = 0u8;
        let mut calcSpriteId: u8 = 0u8;
        let mut position: u8 = 0u8;
        'l1: {
            let __sw1 = ((((cmd).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 3i32
                || __sw1 == 4i32
                || __sw1 == 5i32
                || __sw1 == 6i32
                || __sw1 == 7i32;
            if __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 {
                spriteId = GetAnimBattlerSpriteId(((((cmd).cast::<i16>()).read()) as u8));
                break 'l1;
            }
            if __sw1 == 4i32 {
                position = 0u8;
                calcSpriteId = 1u8;
                break 'l1;
            }
            if __sw1 == 5i32 {
                position = 2u8;
                calcSpriteId = 1u8;
                break 'l1;
            }
            if __sw1 == 6i32 {
                position = 1u8;
                calcSpriteId = 1u8;
                break 'l1;
            }
            if __sw1 == 7i32 {
                position = 3u8;
                calcSpriteId = 1u8;
                break 'l1;
            }
            if !__matched {
                spriteId = 255u8;
                break 'l1;
            }
        }
        if (calcSpriteId) != 0 {
            battler = GetBattlerAtPosition(position);
            if (IsBattlerSpriteVisible(battler)) != 0 {
                spriteId = (((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read();
            } else {
                spriteId = 255u8;
            }
        }
        if ((spriteId) as i32) != 255i32 {
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
                ((((cmd).wrapping_add(2).cast::<i16>()).read()) as u8),
            );
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetIsDoomDesireHitTurn(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((&raw mut gAnimMoveTurn).cast::<u8>()).read()) as i32) < 2i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        }
        if ((((&raw mut gAnimMoveTurn).cast::<u8>()).read()) as i32) == 2i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
