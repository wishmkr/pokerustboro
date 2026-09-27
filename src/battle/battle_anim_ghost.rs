//! Translated from `src/battle_anim_ghost.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAffineAnim_ConfuseRayBallBounce sAffineAnims_ConfuseRayBallBounce gConfuseRayBallBounceSpriteTemplate gConfuseRayBallSpiralSpriteTemplate sAffineAnim_ShadowBall sAffineAnims_ShadowBall gShadowBallSpriteTemplate sAnim_Lick sAnims_Lick gLickSpriteTemplate sAffineAnim_Unused sAffineAnims_Unused gDestinyBondWhiteShadowSpriteTemplate gCurseNailSpriteTemplate gCurseGhostSpriteTemplate gNightmareDevilSpriteTemplate sAnim_GrudgeFlame sAnims_GrudgeFlame gGrudgeFlameSpriteTemplate sMonMoveCircularSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_ghost::*;

unsafe extern "C" {
    static mut gAnimCustomPanning: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattle_WIN0H: u8;
    static mut gBattle_WIN0V: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gPaletteFade: u8;
    static mut gPlttBufferFaded: u8;
    static mut gPlttBufferUnfaded: u8;
    static mut gScanlineEffect: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocSpritePalette(a0: u16) -> u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn ClearGpuRegBits(a0: u8, a1: u16);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CpuSet(a0: *mut u8, a1: *mut u8, a2: u32);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimSpriteAndDisableBlend(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroySpriteWithActiveSheet(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn InitAnimLinearTranslationWithSpeed(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn PlaySE(a0: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn ResetSpriteRotScale(a0: u8);
    fn ScanlineEffect_InitWave(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetGpuRegBits(a0: u8, a1: u16);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateSpriteLinearFixedPoint(a0: *mut u8);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimConfuseRayBallBounce(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
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
        InitAnimLinearTranslationWithSpeed(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimConfuseRayBallBounce_Step1));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(16i16);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(
            82u8,
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimConfuseRayBallBounce_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r0: i16 = 0i16;
        let mut r2: i16 = 0i16;
        UpdateConfuseRayBallBlend(sprite);
        if (AnimTranslateLinear(sprite)) != 0 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimConfuseRayBallBounce_Step2));
            return;
        }
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    10i16,
                )) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    15i16,
                )) as i32),
            )) as i16),
        );
        r2 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read();
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(5i32)
                & 255i32) as i16),
        );
        r0 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read();
        if (((r2) as i32) != 0i32) && (((r2) as i32) <= 196i32) {
            return;
        }
        if ((r0) as i32) <= 0i32 {
            return;
        }
        PlaySE12WithPanning(
            196u16,
            ((((&raw mut gAnimCustomPanning).cast::<u8>()).read()) as i8),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimConfuseRayBallBounce_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r2: i16 = 0i16;
        let mut r0: i16 = 0i16;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        AnimTranslateLinear(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((Sin(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    10i16,
                )) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((Cos(
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    15i16,
                )) as i32),
            )) as i16),
        );
        r2 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read();
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(5i32)
                & 255i32) as i16),
        );
        r0 = ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read();
        if (((r2) as i32) == 0i32) || (((r2) as i32) > 196i32) {
            if ((r0) as i32) > 0i32 {
                PlaySE(196u16);
            }
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            == 0i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(DestroyAnimSpriteAndDisableBlend));
        } else {
            UpdateConfuseRayBallBlend(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn UpdateConfuseRayBallBlend(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            > 255i32
        {
            if (({
                let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 269i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            }
            return;
        }
        if ((({
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            let __t4 = (__p3).read();
            (__p3).write(((__p3).read()).wrapping_add(1));
            __t4
        }) as i32)
            & 255i32)
            == 0i32
        {
            let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p5).write((((((__p5).read()) as i32) & 65280i32) as i16));
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                & 256i32)
                != 0i32
            {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p6).write(((__p6).read()).wrapping_add(1));
            } else {
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
                (__p7).write(((__p7).read()).wrapping_sub(1));
            }
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                ) << 8)
                    | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)) as u16),
            );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                == 0i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    == 16i32)
            {
                let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
                (__p8).write((((((__p8).read()) as i32) ^ 256i32) as i16));
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                == 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(256i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimConfuseRayBallSpiral(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimConfuseRayBallSpiral_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimConfuseRayBallSpiral_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut temp1: u16 = 0u16;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            32i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            8i16,
        ));
        temp1 = (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_sub(65i32))
            as u16);
        if ((temp1) as i32) <= 130i32 {
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(19i32)
                & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(80i32)) as i16));
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p3).write((((((__p3).read()) as i32).wrapping_add(1i32)) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 61i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_NightShadeClone(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 4096u16);
        spriteId = GetAnimBattlerSpriteId(0u8);
        PrepareBattlerSpriteForRotScale(spriteId, 1u8);
        SetSpriteRotScale(spriteId, 128i16, 128i16, 0u16);
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
            2,
            1,
            (0u16) as i32,
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(128i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(16i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_NightShadeClone_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_NightShadeClone_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(1i32)) as i16));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .read()) as i32)
            == 3i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(1i32)) as i16));
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p3).write((((((__p3).read()) as i32).wrapping_sub(1i32)) as i16));
            SetGpuReg(
                82u8,
                (((((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    << 8)
                    | ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(2))
                    .read()) as i32)) as u16),
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                != 9i32
            {
                return;
            }
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_NightShadeClone_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_NightShadeClone_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            > 0i32
        {
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(1i32)) as i16));
            return;
        }
        spriteId = GetAnimBattlerSpriteId(0u8);
        let __p2 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(8i32)) as i16));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            <= 255i32
        {
            SetSpriteRotScale(
                spriteId,
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read(),
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read(),
                0u16,
            );
        } else {
            ResetSpriteRotScale(spriteId);
            DestroyAnimVisualTask(taskId);
            SetGpuReg(80u8, 0u16);
            SetGpuReg(82u8, 0u16);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimShadowBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut oldPosX: i16 = ((sprite).wrapping_add(32).cast::<i16>()).read();
        let mut oldPosY: i16 = ((sprite).wrapping_add(34).cast::<i16>()).read();
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((crate::c::div_i32(
                (((oldPosX) as i32)
                    .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32))
                    << 4),
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) << 1),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::div_i32(
                (((oldPosY) as i32)
                    .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32))
                    << 4),
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) << 1),
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimShadowBall_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimShadowBall_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        >> 4) as i16),
                );
                let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(1i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    > 0i32
                {
                    break 'l1;
                }
                let __p5 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p5).write((((((__p5).read()) as i32).wrapping_add(1i32)) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p6).write((((((__p6).read()) as i32).wrapping_sub(1i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    > 0i32
                {
                    break 'l1;
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        3u8,
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                    ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
                    ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((crate::c::div_i32(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)
                            .wrapping_sub(
                                ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32),
                            ) << 4),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                    ((crate::c::div_i32(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_sub(
                                ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32),
                            ) << 4),
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
                );
                let __p7 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p7).write((((((__p7).read()) as i32).wrapping_add(1i32)) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p8).write(
                    (((((__p8).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32),
                    )) as i16),
                );
                let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p9).write(
                    (((((__p9).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        >> 4) as i16),
                );
                let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p10).write((((((__p10).read()) as i32).wrapping_sub(1i32)) as i16));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    > 0i32
                {
                    break 'l1;
                }
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
                let __p11 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p11).write((((((__p11).read()) as i32).wrapping_add(1i32)) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                DestroySpriteAndMatrix(sprite);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimLick(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimLick_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimLick_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r5: u8 = 0u8;
        let mut r6: u8 = 0u8;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            if !((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) != 0) {
                crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            }
            'l1: {
                let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
                let __matched = __sw1 == 0i32 || __sw1 == 1i32;
                if !__matched {
                    r6 = 1u8;
                    break 'l1;
                }
                if __sw1 == 0i32 {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        == 2i32
                    {
                        r5 = 1u8;
                    }
                    break 'l1;
                }
                if __sw1 == 1i32 {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        == 4i32
                    {
                        r5 = 1u8;
                    }
                    break 'l1;
                }
            }
            if (r5) != 0 {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                        ^ 1i32) as u16) as i32,
                );
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p2).write(((__p2).read()).wrapping_add(1));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    == 5i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
            } else {
                if (r6) != 0 {
                    DestroyAnimSprite(sprite);
                } else {
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_NightmareClone(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(CloneBattlerSpriteWithBlend(1u8));
        if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) < 0i32 {
            DestroyAnimVisualTask(taskId);
            return;
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(15i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(2i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(
            82u8,
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))
                as u16),
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .write(80i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write((-144i16));
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(112i16);
        } else {
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(144i16);
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write((-112i16));
        }
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(0i16);
        StoreSpriteCallbackInData6(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
            ),
            Some(SpriteCallbackDummy),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPoint));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_NightmareClone_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_NightmareClone_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(1i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        & 3i32) as i16),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == 1i32
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        > 0i32
                    {
                        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        (__p3).write((((((__p3).read()) as i32).wrapping_sub(1i32)) as i16));
                    }
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    == 3i32
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        <= 15i32
                    {
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p4).write((((((__p4).read()) as i32).wrapping_add(1i32)) as i16));
                    }
                }
                SetGpuReg(
                    82u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)) as u16),
                );
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    != 16i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        != 0i32)
                {
                    break 'l1;
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    <= 80i32
                {
                    break 'l1;
                }
                DestroySpriteWithActiveSheet(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    <= 1i32
                {
                    break 'l1;
                }
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                (__p7).write((((((__p7).read()) as i32).wrapping_add(1i32)) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SpiteTargetShadow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(0i16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_SpiteTargetShadow_Step1));
        (((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read()).unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SpiteTargetShadow_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut startLine: i16 = 0i16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut position: u8 =
            GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((AllocSpritePalette(10097u16)) as i16));
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                    as i32)
                    == 255i32)
                    || (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        == 15i32)
                {
                    DestroyAnimVisualTask(taskId);
                } else {
                    (((task).wrapping_add(8)).cast::<i16>())
                        .write(CloneBattlerSpriteWithBlend(1u8));
                    if (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) < 0i32 {
                        FreeSpritePaletteByTag(10097u16);
                        DestroyAnimVisualTask(taskId);
                    } else {
                        let mut mask2: i16 = 0i16;
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            4,
                            4,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                                as u16) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(1),
                            2,
                            2,
                            (0u32) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(5),
                            2,
                            2,
                            (3u16) as i32,
                        );
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(62),
                            2,
                            1,
                            (crate::c::bf_read(
                                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                    .cast::<*mut u8>())
                                .read())
                                .wrapping_offset(
                                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
                                        as isize
                                        * 4,
                                ))
                                .wrapping_add(0),
                                0,
                                1,
                                false,
                            ) as u16) as i32,
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(16i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                            .write(((GetAnimBattlerSpriteId(1u8)) as i16));
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
                            (((((crate::c::bf_read(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(13))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(5),
                                4,
                                4,
                                false,
                            ) as u16) as i32)
                                .wrapping_add(16i32))
                            .wrapping_mul(16i32)) as i16),
                        );
                        if ((position) as i32) == 1i32 {
                            let mut mask: u16 = 512u16;
                            mask2 = ((mask) as i16);
                        } else {
                            let mut mask: u16 = 1024u16;
                            mask2 = ((mask) as i16);
                        }
                        ClearGpuRegBits(0u8, ((mask2) as u16));
                        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                        (__p2).write(((__p2).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        .wrapping_add(16i32))
                    .wrapping_mul(16i32)) as i16),
                );
                'l2: loop {
                    'l3: {
                        'l4: loop {
                            'l5: {
                                CpuSet(
                                    ((((&raw mut gPlttBufferUnfaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            ((((((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(4))
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .cast::<u8>(),
                                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                        .wrapping_offset(
                                            ((((((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(14))
                                            .read())
                                                as i32)
                                                as isize,
                                        ))
                                    .cast::<u8>(),
                                    (67108864u32
                                        | (crate::c::div_u32(
                                            32u32,
                                            ((crate::c::div_i32(32i32, 8i32)) as u32),
                                        ) & 2097151u32)),
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
                BlendPalette(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16),
                    16u16,
                    10u8,
                    15373u16,
                );
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 2i32 {
                startLine = (((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>())
                .read()) as i32)
                    .wrapping_add(
                        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>())
                        .read()) as i32),
                    ))
                .wrapping_sub(32i32)) as i16);
                if ((startLine) as i32) < 0i32 {
                    startLine = 0i16;
                }
                if ((position) as i32) == 1i32 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
                        ((ScanlineEffect_InitWave(
                            ((startLine) as u8),
                            ((((startLine) as i32).wrapping_add(64i32)) as u8),
                            2u8,
                            6u8,
                            0u8,
                            4u8,
                            1u8,
                        )) as i16),
                    );
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
                        ((ScanlineEffect_InitWave(
                            ((startLine) as u8),
                            ((((startLine) as i32).wrapping_add(64i32)) as u8),
                            2u8,
                            6u8,
                            0u8,
                            8u8,
                            1u8,
                        )) as i16),
                    );
                }
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                (__p4).write(((__p4).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((position) as i32) == 1i32 {
                    SetGpuReg(80u8, 16194u16);
                } else {
                    SetGpuReg(80u8, 16196u16);
                }
                SetGpuReg(82u8, 4096u16);
                let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                (__p5).write(((__p5).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((position) as i32) == 1i32 {
                    SetGpuRegBits(0u8, 512u16);
                } else {
                    SetGpuRegBits(0u8, 1024u16);
                }
                ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                    .write(Some(AnimTask_SpiteTargetShadow_Step2));
                let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if !__matched {
                let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SpiteTargetShadow_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 1i32) as i16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                ((crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32),
                    18i32,
                )) as i16),
            );
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32) == 1i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
                (((16i32).wrapping_sub(crate::c::div_i32(
                    ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32),
                    18i32,
                ))) as i16),
            );
        }
        SetGpuReg(
            82u8,
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                << 8)
                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32))
                as u16),
        );
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32) == 128i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(0i16);
            ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
                .write(Some(AnimTask_SpiteTargetShadow_Step3));
            (((task).cast::<Option<unsafe extern "C" fn(u8)>>()).read()).unwrap_unchecked()(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SpiteTargetShadow_Step3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut rank: u8 =
            GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32);
            if __sw1 == 0i32 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
                    .write(((GetAnimBattlerSpriteId(1u8)) as i16));
                if ((rank) as i32) == 1i32 {
                    ClearGpuRegBits(0u8, 512u16);
                } else {
                    ClearGpuRegBits(0u8, 1024u16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                BlendPalette(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as u16),
                    16u16,
                    0u8,
                    15373u16,
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    2,
                    1,
                    (1u16) as i32,
                );
                DestroySpriteWithActiveSheet(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                ));
                FreeSpritePaletteByTag(10097u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                if ((rank) as i32) == 1i32 {
                    SetGpuRegBits(0u8, 512u16);
                } else {
                    SetGpuRegBits(0u8, 1024u16);
                }
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15);
        (__p2).write(((__p2).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AnimDestinyBondWhiteShadow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler1X: i16 = 0i16;
        let mut battler1Y: i16 = 0i16;
        let mut battler2X: i16 = 0i16;
        let mut battler2Y: i16 = 0i16;
        let mut yDiff: i16 = 0i16;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler1X =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16);
            battler1Y = ((((GetBattlerSpriteCoord(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                1u8,
            )) as i32)
                .wrapping_add(28i32)) as i16);
            battler2X =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i16);
            battler2Y =
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i32)
                    .wrapping_add(28i32)) as i16);
        } else {
            battler1X =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i16);
            battler1Y =
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i32)
                    .wrapping_add(28i32)) as i16);
            battler2X =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                    as i16);
            battler2Y = ((((GetBattlerSpriteCoord(
                ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                1u8,
            )) as i32)
                .wrapping_add(28i32)) as i16);
        }
        yDiff = ((((battler2Y) as i32).wrapping_sub(((battler1Y) as i32))) as i16);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((((battler1X) as i32).wrapping_mul(16i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((battler1Y) as i32).wrapping_mul(16i32)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((crate::c::div_i32(
                (((battler2X) as i32).wrapping_sub(((battler1X) as i32))).wrapping_mul(16i32),
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((crate::c::div_i32(
                ((yDiff) as i32).wrapping_mul(16i32),
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(battler2X);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(battler2Y);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                2i32,
            )) as i16),
        );
        crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
        ((sprite).wrapping_add(32).cast::<i16>()).write(battler1X);
        ((sprite).wrapping_add(34).cast::<i16>()).write(battler1Y);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimDestinyBondWhiteShadow_Step));
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn AnimDestinyBondWhiteShadow_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) != 0 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) >> 4) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    >> 4) as i16),
            );
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 0i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_DestinyBondWhiteShadow(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        let mut battler: i16 = 0i16;
        let mut spriteId: u8 = 0u8;
        let mut baseX: i16 = 0i16;
        let mut baseY: i16 = 0i16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 4096u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(16i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        baseX = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
            as i16);
        baseY =
            GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8);
        if !((IsContest()) != 0) {
            {
                battler = 0i16;
                'l1: loop {
                    if !(((battler) as i32) < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((((battler) as i32)
                            != ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32))
                            && (((battler) as i32)
                                != (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read())
                                    as i32)
                                    ^ 2i32)))
                            && ((IsBattlerSpriteVisible(((battler) as u8))) != 0)
                        {
                            spriteId = CreateSprite(
                                (&raw const gDestinyBondWhiteShadowSpriteTemplate)
                                    .cast::<u8>()
                                    .cast_mut(),
                                baseX,
                                baseY,
                                55u8,
                            );
                            if ((spriteId) as i32) != 64i32 {
                                x = ((GetBattlerSpriteCoord(((battler) as u8), 2u8)) as i16);
                                y = GetBattlerSpriteCoordAttr(((battler) as u8), 3u8);
                                (((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .write(((((baseX) as i32) << 4) as i16));
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(1))
                                .write(((((baseY) as i32) << 4) as i16));
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(
                                    ((crate::c::div_i32(
                                        (((x) as i32).wrapping_sub(((baseX) as i32)) << 4),
                                        ((((((&raw mut gBattleAnimArgs).cast::<i16>())
                                            .cast::<i16>())
                                        .wrapping_offset(1))
                                        .read()) as i32),
                                    )) as i16),
                                );
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .write(
                                    ((crate::c::div_i32(
                                        (((y) as i32).wrapping_sub(((baseY) as i32)) << 4),
                                        ((((((&raw mut gBattleAnimArgs).cast::<i16>())
                                            .cast::<i16>())
                                        .wrapping_offset(1))
                                        .read()) as i32),
                                    )) as i16),
                                );
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(4))
                                .write(
                                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                                        .wrapping_offset(1))
                                    .read(),
                                );
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(5))
                                .write(x);
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(6))
                                .write(y);
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(28)
                                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                                .write(Some(AnimDestinyBondWhiteShadow_Step));
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                                    (((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(12))
                                    .read()) as i32)
                                        .wrapping_add(13i32))
                                        as isize,
                                ))
                                .write(((spriteId) as i16));
                                let __p1 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                                (__p1).write(((__p1).read()).wrapping_add(1));
                            }
                        }
                    }
                    battler = (battler).wrapping_add(1);
                }
            }
        } else {
            spriteId = CreateSprite(
                (&raw const gDestinyBondWhiteShadowSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                baseX,
                baseY,
                55u8,
            );
            if ((spriteId) as i32) != 64i32 {
                x = 48i16;
                y = 40i16;
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(((((baseX) as i32) << 4) as i16));
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(((((baseY) as i32) << 4) as i16));
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(
                    ((crate::c::div_i32(
                        (((x) as i32).wrapping_sub(((baseX) as i32)) << 4),
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
                );
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(
                    ((crate::c::div_i32(
                        (((y) as i32).wrapping_sub(((baseY) as i32)) << 4),
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
                );
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                        .read(),
                );
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(x);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(6))
                .write(y);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimDestinyBondWhiteShadow_Step));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13))
                    .write(((spriteId) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(1i16);
            }
        }
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_DestinyBondWhiteShadow_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_DestinyBondWhiteShadow_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    == 0i32
                {
                    if (({
                        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                        let __t3 = ((__p2).read()).wrapping_add(1);
                        (__p2).write(__t3);
                        __t3
                    }) as i32)
                        > 1i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                        (__p4).write(((__p4).read()).wrapping_add(1));
                        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            & 1i32)
                            != 0
                        {
                            if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                                .read()) as i32)
                                < 16i32
                            {
                                let __p5 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                                (__p5).write(((__p5).read()).wrapping_add(1));
                            }
                        } else {
                            if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                .read())
                                != 0
                            {
                                let __p6 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                                (__p6).write(((__p6).read()).wrapping_sub(1));
                            }
                        }
                        SetGpuReg(
                            82u8,
                            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                .read()) as i32)
                                << 8)
                                | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                                    .read()) as i32)) as u16),
                        );
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            >= 24i32
                        {
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7))
                                .write(0i16);
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                .write(1i16);
                        }
                    }
                }
                if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) != 0 {
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                } else {
                    if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) != 0 {
                        let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        if (((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                            != 0
                        {
                            let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                            (__p12).write(((__p12).read()).wrapping_sub(1));
                        }
                    } else {
                        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32)
                            < 16i32
                        {
                            let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                            (__p13).write(((__p13).read()).wrapping_add(1));
                        }
                    }
                    SetGpuReg(
                        82u8,
                        (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32)
                            << 8)
                            | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                                .read()) as i32)) as u16),
                    );
                    if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read())
                        as i32)
                        == 0i32)
                        && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                            as i32)
                            == 16i32)
                    {
                        {
                            i = 0u16;
                            'l2: loop {
                                if !(((i) as i32)
                                    < ((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(12))
                                    .read()) as i32))
                                {
                                    break 'l2;
                                }
                                'l3: {
                                    DestroySprite(
                                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(
                                                    (((i) as i32).wrapping_add(13i32)) as isize,
                                                ))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ),
                                    );
                                }
                                i = (i).wrapping_add(1);
                            }
                        }
                        let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p14).write(((__p14).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                    let __t16 = ((__p15).read()).wrapping_add(1);
                    (__p15).write(__t16);
                    __t16
                }) as i32)
                    > 0i32
                {
                    let __p17 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CurseStretchingBlackBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut startX: i16 = 0i16;
        let mut startY: i16 = 0i16;
        let mut leftDistance: i16 = 0i16;
        let mut topDistance: i16 = 0i16;
        let mut bottomDistance: i16 = 0i16;
        let mut rightDistance: i16 = 0i16;
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        SetGpuReg(72u8, 16191u16);
        SetGpuReg(74u8, 16159u16);
        SetGpuReg(80u8, 200u16);
        SetGpuReg(84u8, 16u16);
        if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32)
            || ((IsContest()) != 0)
        {
            startX = 40i16;
        } else {
            startX = 200i16;
        }
        ((&raw mut gBattle_WIN0H).cast::<u16>())
            .write((((((startX) as i32) << 8) | ((startX) as i32)) as u16));
        startY = 40i16;
        ((&raw mut gBattle_WIN0V).cast::<u16>())
            .write((((((startY) as i32) << 8) | ((startY) as i32)) as u16));
        leftDistance = startX;
        rightDistance = (((240i32).wrapping_sub(((startX) as i32))) as i16);
        topDistance = startY;
        bottomDistance = 72i16;
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(leftDistance);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(rightDistance);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(topDistance);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(bottomDistance);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(startX);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(startY);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_CurseStretchingBlackBg_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_CurseStretchingBlackBg_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut step: i16 = 0i16;
        let mut leftDistance: i16 = 0i16;
        let mut rightDistance: i16 = 0i16;
        let mut topDistance: i16 = 0i16;
        let mut bottomDistance: i16 = 0i16;
        let mut startX: i16 = 0i16;
        let mut startY: i16 = 0i16;
        let mut left: u16 = 0u16;
        let mut right: u16 = 0u16;
        let mut top: u16 = 0u16;
        let mut bottom: u16 = 0u16;
        let mut selectedPalettes: u16 = 0u16;
        step = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read();
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        leftDistance = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read();
        rightDistance = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read();
        topDistance = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read();
        bottomDistance = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read();
        startX = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read();
        startY = ((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .read();
        if ((step) as i32) < 16i32 {
            left = ((((startX) as f32)
                - ((((((leftDistance) as f32) * ((0.0625f32) as f32)) as f32) * ((step) as f32))
                    as f32)) as u16);
            right = ((((startX) as f32)
                + ((((((rightDistance) as f32) * ((0.0625f32) as f32)) as f32) * ((step) as f32))
                    as f32)) as u16);
            top = ((((startY) as f32)
                - ((((((topDistance) as f32) * ((0.0625f32) as f32)) as f32) * ((step) as f32))
                    as f32)) as u16);
            bottom = ((((startY) as f32)
                + ((((((bottomDistance) as f32) * ((0.0625f32) as f32)) as f32) * ((step) as f32))
                    as f32)) as u16);
        } else {
            left = 0u16;
            right = 240u16;
            top = 0u16;
            bottom = 112u16;
            selectedPalettes = ((GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8)) as u16);
            BeginNormalPaletteFade(((selectedPalettes) as u32), 0i8, 16u8, 16u8, 0u16);
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_CurseStretchingBlackBg_Step2));
        }
        ((&raw mut gBattle_WIN0H).cast::<u16>())
            .write((((((left) as i32) << 8) | ((right) as i32)) as u16));
        ((&raw mut gBattle_WIN0V).cast::<u16>())
            .write((((((top) as i32) << 8) | ((bottom) as i32)) as u16));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_CurseStretchingBlackBg_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
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
pub(crate) unsafe extern "C" fn AnimCurseNail(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut xDelta: i16 = 0i16;
        let mut xDelta2: i16 = 0i16;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            xDelta = 24i16;
            xDelta2 = (-2i16);
            crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
        } else {
            xDelta = (-24i16);
            xDelta2 = 2i16;
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((xDelta) as i32))) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(xDelta2);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(60i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimCurseNail_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimCurseNail_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var0: u16 = 0u16;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        } else {
            let __p2 = (sprite).wrapping_add(36).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            var0 = ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_add(7i32))
                as u16);
            if ((var0) as i32) > 14i32 {
                let __p3 = (sprite).wrapping_add(32).cast::<i16>();
                (__p3).write(
                    (((((__p3).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                crate::c::bf_write(
                    (sprite).wrapping_add(4),
                    0,
                    10,
                    ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                        .wrapping_add(8i32)) as u16) as i32,
                );
                if (({
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 3i32
                {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(WaitAnimForDuration));
                    StoreSpriteCallbackInData6(sprite, Some(AnimCurseNail_Step2));
                } else {
                    (((sprite).wrapping_add(46)).cast::<i16>()).write(40i16);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimCurseNail_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            SetGpuReg(80u8, 16192u16);
            SetGpuReg(82u8, 16u16);
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                < 2i32
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_add(1));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                SetGpuReg(
                    82u8,
                    (((16i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                    ) | (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                        .read()) as i32)
                        << 8)) as u16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    == 16i32
                {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimCurseNail_End));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimCurseNail_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        ((&raw mut gBattle_WIN0H).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_WIN0V).cast::<u16>()).write(0u16);
        DestroyAnimSprite(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimGhostStatusSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut coeffB: u16 = 0u16;
        let mut coeffA: u16 = 0u16;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            12i16,
        ));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32).wrapping_neg())
                    as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(6i32)
                & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(256i32)) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >> 8)
                .wrapping_neg()) as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
        (__p2).write(((__p2).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            == 1i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(1291i16);
            SetGpuReg(80u8, 16192u16);
            SetGpuReg(
                82u8,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
            );
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                > 30i32
            {
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p3).write(((__p3).read()).wrapping_add(1));
                coeffB = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                    .read()) as i32)
                    >> 8) as u16);
                coeffA = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                    .read()) as i32)
                    & 255i32) as u16);
                if (({
                    let __t4 = (coeffB).wrapping_add(1);
                    coeffB = __t4;
                    __t4
                }) as i32)
                    > 16i32
                {
                    coeffB = 16u16;
                }
                coeffA = (coeffA).wrapping_sub(1);
                if (((coeffA) as i16) as i32) < 0i32 {
                    coeffA = 0u16;
                }
                SetGpuReg(
                    82u8,
                    (((((coeffB) as i32) << 8) | ((coeffA) as i32)) as u16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                    .write((((((coeffB) as i32) << 8) | ((coeffA) as i32)) as i16));
                if (((coeffB) as i32) == 16i32) && (((coeffA) as i32) == 0i32) {
                    crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimGhostStatusSprite_Step));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGhostStatusSprite_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(80u8, 0u16);
        SetGpuReg(82u8, 0u16);
        DestroyAnimSprite(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GrudgeFlames(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(16i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
            ((GetBattlerYCoordWithElevation(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
            (((crate::c::div_i32(
                ((GetBattlerSpriteCoordAttr(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32),
                2i32,
            ))
            .wrapping_add(8i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i32)
                .wrapping_sub(2i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(16i16);
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 4096u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(0i16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_GrudgeFlames_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_GrudgeFlames_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0u16;
                    'l2: loop {
                        if !(((i) as i32) < 6i32) {
                            break 'l2;
                        }
                        'l3: {
                            spriteId = CreateSprite(
                                (&raw const gGrudgeFlameSpriteTemplate)
                                    .cast::<u8>()
                                    .cast_mut(),
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9))
                                    .read(),
                                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                                    .read(),
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as u8),
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
                                .write(
                                    ((((GetBattlerSide(
                                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                                    )) as i32)
                                        == 0i32) as i16),
                                );
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(2))
                                .write(((((i) as i32).wrapping_mul(42i32) & 255i32) as i16));
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(3))
                                .write(
                                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11))
                                        .read(),
                                );
                                ((((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(46))
                                .cast::<i16>())
                                .wrapping_offset(5))
                                .write(((((i) as i32).wrapping_mul(6i32)) as i16));
                                let __p2 =
                                    (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                                (__p2).write(((__p2).read()).wrapping_add(1));
                            }
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    & 1i32)
                    != 0
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        < 14i32
                    {
                        let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                } else {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        > 4i32
                    {
                        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        (__p7).write(((__p7).read()).wrapping_sub(1));
                    }
                }
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 14i32)
                    && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        == 4i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                SetGpuReg(
                    82u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)) as u16),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > 30i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p11 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                if ((({
                    let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    & 1i32)
                    != 0
                {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        > 0i32
                    {
                        let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                        (__p14).write(((__p14).read()).wrapping_sub(1));
                    }
                } else {
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        < 16i32
                    {
                        let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                    }
                }
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 0i32)
                    && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        == 16i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(1i16);
                    let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                SetGpuReg(
                    82u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)) as u16),
                );
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    == 0i32
                {
                    let __p17 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p17).write(((__p17).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimGrudgeFlame(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut index: u16 = 0u16;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(2i32)) as i16));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            (__p2).write((((((__p2).read()) as i32).wrapping_sub(2i32)) as i16));
        }
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
        ));
        index = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            .wrapping_sub(65i32)) as u16);
        if ((index) as i32) < 127i32 {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as i32)
                    .wrapping_add(1i32)) as u16) as i32,
            );
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u16) as i32,
            );
        }
        let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p4).write(((__p4).read()).wrapping_add(1));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_mul(8i32)
                & 255i32) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read(),
            7i16,
        ));
        if (((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .read())
            != 0
        {
            let __p5 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7);
            (__p5).write(((__p5).read()).wrapping_sub(1));
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimMonMoveCircular(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
            ))
            .read()) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(128i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(10i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimMonMoveCircular_Step));
        let __p1 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                as isize
                * 68,
        ))
        .wrapping_add(34)
        .cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimMonMoveCircular_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) != 0 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(Sin(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(Cos(
                (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
            ));
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 255i32 {
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write((((((__p3).read()) as i32).wrapping_sub(256i32)) as i16));
            }
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            let __p4 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_sub(8i32)) as i16));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(DestroySpriteAndMatrix));
        }
    }
}
