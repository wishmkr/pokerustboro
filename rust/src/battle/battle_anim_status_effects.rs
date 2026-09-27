//! Translated from `src/battle_anim_status_effects.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_FlickeringOrb sAnims_FlickeringOrb sFlickeringOrbSpriteTemplate sFlickeringOrbFlippedSpriteTemplate sAnim_WeatherBallNormal sAnims_WeatherBallNormal gWeatherBallUpSpriteTemplate gWeatherBallNormalDownSpriteTemplate sAnim_SpinningSparkle sAnims_SpinningSparkle gSpinningSparkleSpriteTemplate sFlickeringFootSpriteTemplate sAnim_FlickeringImpact_0 sAnim_FlickeringImpact_1 sAnim_FlickeringImpact_2 sAnims_FlickeringImpact sFlickeringImpactSpriteTemplate sAnim_FlickeringShrinkOrb sAnims_FlickeringShrinkOrb sAffineAnim_FlickeringShrinkOrb sAffineAnims_FlickeringShrinkOrb sFlickeringShrinkOrbSpriteTemplate sFrozenIceCubeSubsprites sFrozenIceCubeSubspriteTable sFrozenIceCubeSpriteTemplate sFlashingCircleImpactSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_status_effects::*;

unsafe extern "C" {
    static mut gAnimScriptActive: u8;
    static mut gAnimScriptCallback: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimPaletteTable: u8;
    static mut gBattleAnimPicTable: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattleAnims_StatusConditions: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gPlttBufferFaded: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn BlendPalette(a0: u16, a1: u16, a2: u8, a3: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitStatsChangeAnimation(a0: u8);
    fn IsContest() -> u8;
    fn LaunchBattleAnimation(a0: *mut *mut u8, a1: u16, a2: u8);
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut u8) -> u8;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
}

pub(crate) unsafe extern "C" fn Task_FlashingCircleImpacts(battler: u8, red: u8) -> u8 {
    unsafe {
        let mut battler = battler;
        let mut red = red;
        let mut battlerSpriteId: u8 = (((&raw mut gBattlerSpriteIds).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read();
        let mut taskId: u8 = CreateTask(Some(Task_UpdateFlashingCircleImpacts), 10u8);
        let mut spriteId: u8 = 0u8;
        let mut i: u8 = 0u8;
        LoadCompressedSpriteSheetUsingHeap(
            ((&raw mut gBattleAnimPicTable).cast::<u8>()).wrapping_offset(1088),
        );
        LoadCompressedSpritePaletteUsingHeap(
            ((&raw mut gBattleAnimPaletteTable).cast::<u8>()).wrapping_offset(1088),
        );
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((battler) as i16));
        if (red) != 0 {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(31i16);
            {
                i = 0u8;
                'l1: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l1;
                    }
                    'l2: {
                        spriteId = CreateSprite(
                            (&raw const sFlashingCircleImpactSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            ((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read(),
                            ((((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                            .wrapping_add(34)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_add(32i32)) as i16),
                            0u8,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(51i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write((-256i16));
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        if ((i) as i32) > 4i32 {
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(21i16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(31744i16);
            {
                i = 0u8;
                'l3: loop {
                    if !(((i) as i32) < 10i32) {
                        break 'l3;
                    }
                    'l4: {
                        spriteId = CreateSprite(
                            (&raw const sFlashingCircleImpactSpriteTemplate)
                                .cast::<u8>()
                                .cast_mut(),
                            ((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                            .wrapping_add(32)
                            .cast::<i16>())
                            .read(),
                            ((((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((battlerSpriteId) as i32) as isize * 68))
                            .wrapping_add(34)
                            .cast::<i16>())
                            .read()) as i32)
                                .wrapping_sub(32i32)) as i16),
                            0u8,
                        );
                        (((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .write(((((i) as i32).wrapping_mul(51i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(256i16);
                        crate::c::bf_write(
                            (((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(62),
                            2,
                            1,
                            (1u16) as i32,
                        );
                        if ((i) as i32) > 4i32 {
                            ((((((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68))
                            .wrapping_add(46))
                            .cast::<i16>())
                            .wrapping_offset(6))
                            .write(21i16);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(1i16);
        return taskId;
    }
}
pub(crate) unsafe extern "C" fn Task_UpdateFlashingCircleImpacts(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == 2i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(0i16);
            BlendPalette(
                (((256i32).wrapping_add(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                16u16,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as u16),
            );
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i32)
                == 0i32
            {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p1).write(((__p1).read()).wrapping_add(1));
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    > 8i32
                {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    (__p2).write((((((__p2).read()) as i32) ^ 1i32) as i16));
                }
            } else {
                let mut var: u16 = ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as u16);
                let __p3 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p3).write(((__p3).read()).wrapping_sub(1));
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4))
                .read()) as i32)
                    < 0i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .write(((var) as i16));
                    let __p4 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(5);
                    (__p4).write((((((__p4).read()) as i32) ^ 1i32) as i16));
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        == 2i32
                    {
                        DestroyTask(taskId);
                    }
                }
            }
        } else {
            let __p6 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p6).write(((__p6).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlashingCircleImpact(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            == 0i32
        {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimFlashingCircleImpact_Step));
            AnimFlashingCircleImpact_Step(sprite);
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlashingCircleImpact_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            32i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            8i16,
        ));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) < 128i32 {
            ((sprite).wrapping_add(67)).write(29u8);
        } else {
            ((sprite).wrapping_add(67)).write(31u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(8i32)
                & 255i32) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >> 8),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write(((__p3).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 52i32
        {
            if (((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0 {
                DestroySpriteAndFreeResources(sprite);
            } else {
                DestroySprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FrozenIceCube(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: i16 =
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_sub(32i32)) as i16);
        let mut y: i16 =
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_sub(36i32)) as i16);
        let mut spriteId: u8 = 0u8;
        if (IsContest()) != 0 {
            x = ((((x) as i32).wrapping_sub(6i32)) as i16);
        }
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 4096u16);
        spriteId = CreateSprite(
            (&raw const sFrozenIceCubeSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            x,
            y,
            4u8,
        );
        if ((GetSpriteTileStartByTag(10010u16)) as i32) == 65535i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        SetSubspriteTables(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((&raw const sFrozenIceCubeSubspriteTable)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((spriteId) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_FrozenIceCube_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 10i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_FrozenIceCube_Step2));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
        } else {
            let mut var: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8);
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(((var) as i32)) << 8) | ((var) as i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut palIndex: u8 = IndexOfSpritePaletteTag(10010u16);
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1);
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 13i32
        {
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2);
            (__p3).write(((__p3).read()).wrapping_add(1));
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                == 3i32
            {
                let mut temp: u16 = 0u16;
                temp = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                    .wrapping_offset(
                        (((256i32).wrapping_add(((palIndex) as i32).wrapping_mul(16i32)))
                            .wrapping_add(13i32)) as isize,
                    ))
                .read();
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    (((256i32).wrapping_add(((palIndex) as i32).wrapping_mul(16i32)))
                        .wrapping_add(13i32)) as isize,
                ))
                .write(
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                        (((256i32).wrapping_add(((palIndex) as i32).wrapping_mul(16i32)))
                            .wrapping_add(14i32)) as isize,
                    ))
                    .read(),
                );
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    (((256i32).wrapping_add(((palIndex) as i32).wrapping_mul(16i32)))
                        .wrapping_add(14i32)) as isize,
                ))
                .write(
                    ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                        (((256i32).wrapping_add(((palIndex) as i32).wrapping_mul(16i32)))
                            .wrapping_add(15i32)) as isize,
                    ))
                    .read(),
                );
                ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                    (((256i32).wrapping_add(((palIndex) as i32).wrapping_mul(16i32)))
                        .wrapping_add(15i32)) as isize,
                ))
                .write(temp);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(0i16);
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3);
                (__p4).write(((__p4).read()).wrapping_add(1));
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as i32)
                    == 3i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .write(0i16);
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(0i16);
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        == 2i32
                    {
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(9i16);
                        ((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .write(Some(AnimTask_FrozenIceCube_Step3));
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step3(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == (-1i32)
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_FrozenIceCube_Step4));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
        } else {
            let mut var: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as u8);
            SetGpuReg(
                82u8,
                ((((16i32).wrapping_sub(((var) as i32)) << 8) | ((var) as i32)) as u16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FrozenIceCube_Step4(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            == 37i32
        {
            let mut spriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as u8);
            FreeSpriteOamMatrix(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
        } else {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                == 39i32
            {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StatsChange(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut goesDown: u16 = 0u16;
        let mut animStatId: i16 = 0i16;
        let mut sharply: u16 = 0u16;
        'l1: {
            let __sw1 = ((((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(8)
                .cast::<*mut u8>())
            .read())
            .cast::<u16>())
            .read()) as i32);
            let __matched = __sw1 == 15i32
                || __sw1 == 16i32
                || __sw1 == 17i32
                || __sw1 == 18i32
                || __sw1 == 19i32
                || __sw1 == 20i32
                || __sw1 == 21i32
                || __sw1 == 22i32
                || __sw1 == 23i32
                || __sw1 == 24i32
                || __sw1 == 25i32
                || __sw1 == 26i32
                || __sw1 == 27i32
                || __sw1 == 28i32
                || __sw1 == 39i32
                || __sw1 == 40i32
                || __sw1 == 41i32
                || __sw1 == 42i32
                || __sw1 == 43i32
                || __sw1 == 44i32
                || __sw1 == 45i32
                || __sw1 == 46i32
                || __sw1 == 47i32
                || __sw1 == 48i32
                || __sw1 == 49i32
                || __sw1 == 50i32
                || __sw1 == 51i32
                || __sw1 == 52i32
                || __sw1 == 55i32
                || __sw1 == 56i32
                || __sw1 == 57i32
                || __sw1 == 58i32;
            if __sw1 == 15i32 {
                goesDown = 0u16;
                animStatId = 0i16;
                break 'l1;
            }
            if __sw1 == 16i32 {
                goesDown = 0u16;
                animStatId = 1i16;
                break 'l1;
            }
            if __sw1 == 17i32 {
                goesDown = 0u16;
                animStatId = 3i16;
                break 'l1;
            }
            if __sw1 == 18i32 {
                goesDown = 0u16;
                animStatId = 5i16;
                break 'l1;
            }
            if __sw1 == 19i32 {
                goesDown = 0u16;
                animStatId = 6i16;
                break 'l1;
            }
            if __sw1 == 20i32 {
                goesDown = 0u16;
                animStatId = 2i16;
                break 'l1;
            }
            if __sw1 == 21i32 {
                goesDown = 0u16;
                animStatId = 4i16;
                break 'l1;
            }
            if __sw1 == 22i32 {
                goesDown = 1u16;
                animStatId = 0i16;
                break 'l1;
            }
            if __sw1 == 23i32 {
                goesDown = 1u16;
                animStatId = 1i16;
                break 'l1;
            }
            if __sw1 == 24i32 {
                goesDown = 1u16;
                animStatId = 3i16;
                break 'l1;
            }
            if __sw1 == 25i32 {
                goesDown = 1u16;
                animStatId = 5i16;
                break 'l1;
            }
            if __sw1 == 26i32 {
                goesDown = 1u16;
                animStatId = 6i16;
                break 'l1;
            }
            if __sw1 == 27i32 {
                goesDown = 1u16;
                animStatId = 2i16;
                break 'l1;
            }
            if __sw1 == 28i32 {
                goesDown = 1u16;
                animStatId = 4i16;
                break 'l1;
            }
            if __sw1 == 39i32 {
                goesDown = 0u16;
                animStatId = 0i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 40i32 {
                goesDown = 0u16;
                animStatId = 1i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 41i32 {
                goesDown = 0u16;
                animStatId = 3i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 42i32 {
                goesDown = 0u16;
                animStatId = 5i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 43i32 {
                goesDown = 0u16;
                animStatId = 6i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 44i32 {
                goesDown = 0u16;
                animStatId = 2i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 45i32 {
                goesDown = 0u16;
                animStatId = 4i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 46i32 {
                goesDown = 1u16;
                animStatId = 0i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 47i32 {
                goesDown = 1u16;
                animStatId = 1i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 48i32 {
                goesDown = 1u16;
                animStatId = 3i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 49i32 {
                goesDown = 1u16;
                animStatId = 5i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 50i32 {
                goesDown = 1u16;
                animStatId = 6i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 51i32 {
                goesDown = 1u16;
                animStatId = 2i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 52i32 {
                goesDown = 1u16;
                animStatId = 4i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 55i32 {
                goesDown = 0u16;
                animStatId = 255i16;
                sharply = 0u16;
                break 'l1;
            }
            if __sw1 == 56i32 {
                goesDown = 0u16;
                animStatId = 255i16;
                sharply = 1u16;
                break 'l1;
            }
            if __sw1 == 57i32 {
                goesDown = 1u16;
                animStatId = 255i16;
                sharply = 0u16;
                break 'l1;
            }
            if __sw1 == 58i32 {
                goesDown = 1u16;
                animStatId = 255i16;
                sharply = 1u16;
                break 'l1;
            }
            if !__matched {
                DestroyAnimVisualTask(taskId);
                return;
            }
        }
        (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(((goesDown) as i16));
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
            .write(animStatId);
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
            .write(((sharply) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(InitStatsChangeAnimation));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LaunchStatusAnimation(battler: u8, statusAnimId: u8) {
    unsafe {
        let mut battler = battler;
        let mut statusAnimId = statusAnimId;
        let mut taskId: u8 = 0u8;
        ((&raw mut gBattleAnimAttacker).cast::<u8>()).write(battler);
        ((&raw mut gBattleAnimTarget).cast::<u8>()).write(battler);
        LaunchBattleAnimation(
            ((&raw mut gBattleAnims_StatusConditions).cast::<*mut u8>()).cast::<*mut u8>(),
            ((statusAnimId) as u16),
            0u8,
        );
        taskId = CreateTask(Some(Task_DoStatusAnimation), 10u8);
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((battler) as i16));
    }
}
pub(crate) unsafe extern "C" fn Task_DoStatusAnimation(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((&raw mut gAnimScriptCallback).cast::<Option<unsafe extern "C" fn()>>()).read())
            .unwrap_unchecked()();
        if !((((&raw mut gAnimScriptActive).cast::<u8>()).read()) != 0) {
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32) as isize
                        * 12,
                ))
                .wrapping_add(0),
                4,
                1,
                (0u8) as i32,
            );
            DestroyTask(taskId);
        }
    }
}
