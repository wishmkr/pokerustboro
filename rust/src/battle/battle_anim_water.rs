//! Translated from `src/battle_anim_water.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnusedWater_Gfx sUnusedWater sAnim_RainDrop sAnims_RainDrop gRainDropSpriteTemplate sAffineAnim_WaterBubbleProjectile sAffineAnims_WaterBubbleProjectile sAnim_WaterBubbleProjectile sAnims_WaterBubbleProjectile gWaterBubbleProjectileSpriteTemplate sAnim_AuroraBeamRing_0 sAnim_AuroraBeamRing_1 sAnims_AuroraBeamRing sAffineAnim_AuroraBeamRing sAffineAnims_AuroraBeamRing gAuroraBeamRingSpriteTemplate sAnim_WaterMudOrb gAnims_WaterMudOrb gHydroPumpOrbSpriteTemplate gMudShotOrbSpriteTemplate gSignalBeamRedOrbSpriteTemplate gSignalBeamGreenOrbSpriteTemplate sAnim_FlamethrowerFlame sAnims_FlamethrowerFlame gFlamethrowerFlameSpriteTemplate gPsywaveRingSpriteTemplate sAffineAnim_HydroCannonCharge sAffineAnim_HydroCannonBeam sAffineAnims_HydroCannonCharge sAffineAnims_HydroCannonBeam gHydroCannonChargeSpriteTemplate gHydroCannonBeamSpriteTemplate sAnim_WaterBubble sAnim_WaterGunDroplet gAnims_WaterBubble sAnims_WaterGunDroplet gWaterGunProjectileSpriteTemplate gWaterGunDropletSpriteTemplate gSmallBubblePairSpriteTemplate gSmallDriftingBubblesSpriteTemplate gSmallWaterOrbSpriteTemplate sAnim_WaterPulseBubble_0 sAnim_WaterPulseBubble_1 sAnim_WeatherBallWaterDown sAnims_WaterPulseBubble sAnims_WeatherBallWaterDown sAffineAnim_WaterPulseRingBubble_0 sAffineAnim_WaterPulseRingBubble_1 sAffineAnim_WeatherBallWaterDown sAffineAnims_WaterPulseRingBubble sAffineAnims_WeatherBallWaterDown gWaterPulseBubbleSpriteTemplate gWaterPulseRingBubbleSpriteTemplate gWeatherBallWaterDownSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_water::*;

unsafe extern "C" {
    static mut gAnimVisualTaskCount: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimBackgroundImageMuddyWater_Pal: u8;
    static mut gBattleAnimBgImage_Surf: u8;
    static mut gBattleAnimBgPalette_Surf: u8;
    static mut gBattleAnimBgTilemap_SurfContest: u8;
    static mut gBattleAnimBgTilemap_SurfOpponent: u8;
    static mut gBattleAnimBgTilemap_SurfPlayer: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gEnemyParty: u8;
    static mut gPlayerParty: u8;
    static mut gPlttBufferFaded: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    static mut gWaterHitSplatSpriteTemplate: u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemap(a0: u32, a1: *mut u8);
    fn AnimLoadCompressedBgTilemapHandleContest(a0: *mut u8, a1: *mut u8, a2: u32);
    fn AnimTask_HorizontalShake(a0: u8);
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn ClearBattleAnimBg(a0: u32);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteSubpriority(a0: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsContest() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn PrepareBattlerSpriteForRotScale(a0: u8, a1: u8);
    fn PrepareEruptAnimTaskData(a0: *mut u8, a1: u8, a2: i16, a3: i16, a4: i16, a5: i16, a6: u16);
    fn Random2() -> u16;
    fn ResetSpriteRotScale(a0: u8);
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn ScanlineEffect_Stop();
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetBattlerSpriteYOffsetFromYScale(a0: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn UpdateEruptAnimTask(a0: *mut u8) -> u8;
    fn WaitAnimForDuration(a0: *mut u8);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_CreateRaindrops(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut x: u8 = 0u8;
        let mut y: u8 = 0u8;
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
        }
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if crate::c::rem_i32(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32),
        ) == 1i32
        {
            x = ((crate::c::rem_i32(((Random2()) as i32), 240i32)) as u8);
            y = ((crate::c::rem_i32(((Random2()) as i32), crate::c::div_i32(160i32, 2i32))) as u8);
            CreateSprite(
                (&raw const gRainDropSpriteTemplate).cast::<u8>().cast_mut(),
                ((x) as i16),
                ((y) as i16),
                4u8,
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
            .wrapping_offset(3))
            .read()) as i32)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRainDrop(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRainDrop_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRainDrop_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            <= 13i32
        {
            let __p3 = (sprite).wrapping_add(36).cast::<i16>();
            (__p3).write(((__p3).read()).wrapping_add(1));
            let __p4 = (sprite).wrapping_add(38).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_add(4i32)) as i16));
        }
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut spriteId: u8 = 0u8;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_sub(
                        (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    3u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    2u8,
                )) as i32)
                    .wrapping_add(
                        (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    3u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        }
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
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
        spriteId = CreateInvisibleSpriteWithCallback(Some(SpriteCallbackDummy));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(((spriteId) as i16));
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((Sin(
                    (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(4))
                    .read()) as u8) as i16),
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                        .read(),
                )) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_sub(
                ((Cos(
                    (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(4))
                    .read()) as u8) as i16),
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                        .read(),
                )) as i32),
            )) as i16),
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            (((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u8) as i32)
                .wrapping_mul(256i32)) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWaterBubbleProjectile_Step1));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut otherSpriteId: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u8);
        let mut timer: u8 = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        let mut trigIndex: u16 = ((((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(1i16);
        AnimTranslateLinear(sprite);
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((Sin(
                    ((((trigIndex) as i32) >> 8) as i16),
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .read(),
                )) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((Cos(
                    ((((trigIndex) as i32) >> 8) as i16),
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read(),
                )) as i32),
            )) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(
            ((((trigIndex) as i32).wrapping_add(
                ((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32),
            )) as i16),
        );
        if (({
            let __t3 = (timer).wrapping_sub(1);
            timer = __t3;
            __t3
        }) as i32)
            != 0i32
        {
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((otherSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((timer) as i16));
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimWaterBubbleProjectile_Step2));
            DestroySprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((otherSpriteId) as i32) as isize * 68),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (0u8) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(AnimWaterBubbleProjectile_Step3));
    }
}
pub(crate) unsafe extern "C" fn AnimWaterBubbleProjectile_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(10i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    }
}
pub(crate) unsafe extern "C" fn AnimAuroraBeamRings(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut unkArg: i16 = 0i16;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            unkArg = ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                .wrapping_offset(2))
            .read()) as i32)
                .wrapping_neg()) as i16);
        } else {
            unkArg = ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                .wrapping_offset(2))
            .read();
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((unkArg) as i32))) as i16),
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
        .write(Some(AnimAuroraBeamRings_Step));
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimAuroraBeamRings_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16) as i32)
            == 65535i32
        {
            StartSpriteAnim(sprite, 1u8);
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
        }
        if (AnimTranslateLinear(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_RotateAuroraRingColors(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            (((256i32)
                .wrapping_add(((IndexOfSpritePaletteTag(10140u16)) as i32).wrapping_mul(16i32)))
                as i16),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_RotateAuroraRingColors_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_RotateAuroraRingColors_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i32 = 0i32;
        let mut palIndex: u16 = 0u16;
        if (({
            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 3i32
        {
            let mut rgbBuffer: u16 = 0u16;
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .write(0i16);
            palIndex = ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                .wrapping_add(1i32)) as u16);
            rgbBuffer = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(((palIndex) as i32) as isize))
            .read();
            {
                i = 1i32;
                'l1: loop {
                    if !(i < 8i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((palIndex) as i32).wrapping_add(i)).wrapping_sub(1i32)) as isize,
                            ))
                        .write(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset((((palIndex) as i32).wrapping_add(i)) as isize))
                            .read(),
                        );
                    }
                    i = (i).wrapping_add(1);
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((palIndex) as i32).wrapping_add(7i32)) as isize))
            .write(rgbBuffer);
        }
        if (({
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(11);
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as i32)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimToTargetInSinWave(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut retArg: u16 = 0u16;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
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
            ((crate::c::div_i32(
                53760i32,
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        retArg = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as u16);
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            > 127i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                .write((((((retArg) as i32).wrapping_sub(127i32)).wrapping_mul(256i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                .write(((((retArg) as i32).wrapping_mul(256i32)) as i16));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimToTargetInSinWave_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimToTargetInSinWave_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (AnimTranslateLinear(sprite)) != 0 {
            DestroyAnimSprite(sprite);
        }
        let __p1 = (sprite).wrapping_add(38).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((Sin(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        >> 8) as i16),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
                )) as i32),
            )) as i16),
        );
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
            .wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )
            >> 8)
            > 127i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32),
                )) as i16),
            );
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_StartSinAnimTimer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_RunSinAnimTimer));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_RunSinAnimTimer(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).write(
            ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .read()) as i32)
                .wrapping_add(3i32)
                & 255i32) as i16),
        );
        if (({
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 0i32
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHydroCannonCharge(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut priority: u8 = 0u8;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write((-10i16));
        priority =
            GetBattlerSpriteSubpriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read());
        if !((IsContest()) != 0) {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32
            {
                ((sprite).wrapping_add(36).cast::<i16>()).write(10i16);
                ((sprite).wrapping_add(67)).write(((((priority) as i32).wrapping_add(2i32)) as u8));
            } else {
                ((sprite).wrapping_add(36).cast::<i16>()).write((-10i16));
                ((sprite).wrapping_add(67)).write(((((priority) as i32).wrapping_sub(2i32)) as u8));
            }
        } else {
            ((sprite).wrapping_add(36).cast::<i16>()).write((-10i16));
            ((sprite).wrapping_add(67)).write(((((priority) as i32).wrapping_add(2i32)) as u8));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimHydroCannonCharge_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimHydroCannonCharge_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHydroCannonBeam(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut respectMonPicOffsets: u8 = 0u8;
        let mut coordType: u8 = 0u8;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
            == ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
        {
            let __p1 = ((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
            if (((GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32)
                || (((GetBattlerPosition(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as i32)
                    == 1i32)
            {
                let __p2 = ((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>();
                (__p2).write((((((__p2).read()) as i32).wrapping_mul((-1i32))) as i16));
            }
        }
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
            .read()) as i32)
            & 65280i32)
            == 0i32
        {
            respectMonPicOffsets = 1u8;
        } else {
            respectMonPicOffsets = 0u8;
        }
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
            .read()) as u8) as i32)
            == 0i32
        {
            coordType = 3u8;
        } else {
            coordType = 1u8;
        }
        InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
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
            ((((GetBattlerSpriteCoord(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                coordType,
            )) as i32)
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
pub(crate) unsafe extern "C" fn AnimWaterGunDroplet(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
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
pub(crate) unsafe extern "C" fn AnimSmallBubblePair(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            != 0i32
        {
            InitSpritePosToAnimTarget(sprite, 1u8);
        } else {
            InitSpritePosToAnimAttacker(sprite, 1u8);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSmallBubblePair_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSmallBubblePair_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(11i32)
                & 255i32) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            4i16,
        ));
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write((((((__p1).read()) as i32).wrapping_add(48i32)) as i16));
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                >> 8)
                .wrapping_neg()) as i16),
        );
        if (({
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
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
pub unsafe extern "C" fn AnimTask_CreateSurfWave(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut taskId2: u8 = 0u8;
        let mut x: *mut u16 = core::ptr::null_mut();
        let mut y: *mut u16 = core::ptr::null_mut();
        x = (&raw mut gBattle_BG1_X).cast::<u16>();
        y = (&raw mut gBattle_BG1_Y).cast::<u16>();
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        SetAnimBgAttribute(1u8, 4u8, 1u8);
        SetAnimBgAttribute(1u8, 0u8, 1u8);
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 3u8, 1u8);
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 1i32
            {
                AnimLoadCompressedBgTilemap(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                    (((&raw mut gBattleAnimBgTilemap_SurfOpponent).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                );
            } else {
                AnimLoadCompressedBgTilemap(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                    (((&raw mut gBattleAnimBgTilemap_SurfPlayer).cast::<u32>()).cast::<u32>())
                        .cast::<u8>(),
                );
            }
        } else {
            AnimLoadCompressedBgTilemapHandleContest(
                (&raw mut animBg).cast::<u8>(),
                (((&raw mut gBattleAnimBgTilemap_SurfContest).cast::<u32>()).cast::<u32>())
                    .cast::<u8>(),
                1u32,
            );
        }
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gBattleAnimBgImage_Surf).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            LoadCompressedPalette(
                ((&raw mut gBattleAnimBgPalette_Surf).cast::<u32>()).cast::<u32>(),
                (((0i32).wrapping_add(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
        } else {
            LoadCompressedPalette(
                ((&raw mut gBattleAnimBackgroundImageMuddyWater_Pal).cast::<u32>()).cast::<u32>(),
                (((0i32).wrapping_add(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16),
                32u16,
            );
        }
        taskId2 = CreateTask(
            Some(AnimTask_SurfWaveScanlineEffect),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(7))
            .read()) as i32)
                .wrapping_add(1i32)) as u8),
        );
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(15))
        .write(((taskId2) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(0i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(4096i16);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(4096i16);
        if (IsContest()) != 0 {
            (x).write(65456u16);
            (y).write(65488u16);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(2i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(1i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(0i16);
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 1i32
            {
                (x).write(65312u16);
                (y).write(256u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write(2i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write((-1i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId2) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(1i16);
            } else {
                (x).write(0u16);
                (y).write(65488u16);
                (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .write((-2i16));
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(1i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId2) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .write(0i16);
            }
        }
        SetGpuReg(20u8, (x).read());
        SetGpuReg(22u8, (y).read());
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId2) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 0i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(48i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(112i16);
        } else {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(0i16);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId2) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(1i16);
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_CreateSurfWave_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_CreateSurfWave_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        let mut i: u8 = 0u8;
        let mut rgbBuffer: u16 = 0u16;
        let mut BGptrX: *mut u16 = (&raw mut gBattle_BG1_X).cast::<u16>();
        let mut BGptrY: *mut u16 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (BGptrX).write(
            (((((BGptrX).read()) as i32).wrapping_add(
                (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32),
            )) as u16),
        );
        (BGptrY).write(
            (((((BGptrY).read()) as i32).wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
            )) as u16),
        );
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
            )) as i16),
        );
        if (({
            let __p2 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5);
            let __t3 = ((__p2).read()).wrapping_add(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 4i32
        {
            rgbBuffer = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    (((0i32).wrapping_add(
                        (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                            .wrapping_mul(16i32),
                    ))
                    .wrapping_add(7i32)) as isize,
                ))
            .read();
            {
                i = 6u8;
                'l1: loop {
                    if !(((i) as i32) != 0i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset(
                                ((((0i32).wrapping_add(
                                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read())
                                        as i32)
                                        .wrapping_mul(16i32),
                                ))
                                .wrapping_add(1i32))
                                .wrapping_add(((i) as i32)))
                                    as isize,
                            ))
                        .write(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                (((((0i32).wrapping_add(
                                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read())
                                        as i32)
                                        .wrapping_mul(16i32),
                                ))
                                .wrapping_add(1i32))
                                .wrapping_add(((i) as i32)))
                                .wrapping_sub(1i32)) as isize,
                            ))
                            .read(),
                        );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>()).wrapping_offset(
                (((0i32).wrapping_add(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                        .wrapping_mul(16i32),
                ))
                .wrapping_add(1i32)) as isize,
            ))
            .write(rgbBuffer);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(0i16);
        }
        if (({
            let __p4 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6);
            let __t5 = ((__p4).read()).wrapping_add(1);
            (__p4).write(__t5);
            __t5
        }) as i32)
            > 1i32
        {
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(0i16);
            if (({
                let __p6 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3);
                let __t7 = ((__p6).read()).wrapping_add(1);
                (__p6).write(__t7);
                __t7
            }) as i32)
                <= 13i32
            {
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as i32)
                        | ((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(3))
                            .read()) as i32),
                        ) << 8)) as i16),
                );
                let __p8 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p8).write(((__p8).read()).wrapping_add(1));
            }
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .read()) as i32)
                > 54i32
            {
                let __p9 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(4);
                (__p9).write(((__p9).read()).wrapping_sub(1));
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .write(
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as i32)
                        | ((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(4))
                            .read()) as i32),
                        ) << 8)) as i16),
                );
            }
        }
        if !((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32) as isize
                * 40,
        ))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            & 31i32)
            != 0)
        {
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .read()) as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    & 31i32) as i16),
            );
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_CreateSurfWave_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_CreateSurfWave_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut BGptrX: *mut u16 = (&raw mut gBattle_BG1_X).cast::<u16>();
        let mut BGptrY: *mut u16 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            ClearBattleAnimBg(1u32);
            ClearBattleAnimBg(2u32);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if !((IsContest()) != 0) {
                SetAnimBgAttribute(1u8, 3u8, 0u8);
            }
            (BGptrX).write(0u16);
            (BGptrY).write(0u16);
            SetGpuReg(80u8, 0u16);
            SetGpuReg(82u8, 0u16);
            ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .read()) as i32) as isize
                    * 40,
            ))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write((-1i16));
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SurfWaveScanlineEffect(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i16 = 0i16;
        let mut params = crate::ffi::Align4([0u8; 12]);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                {
                    i = 0i16;
                    'l2: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32))
                        {
                            break 'l2;
                        }
                        'l3: {
                            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write({
                                let __v2 = ((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(2))
                                .read()) as u16);
                                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(1920))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(__v2);
                                __v2
                            });
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read();
                    'l4: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32))
                        {
                            break 'l4;
                        }
                        'l5: {
                            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write({
                                let __v3 = ((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(1))
                                .read()) as u16);
                                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(1920))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(__v3);
                                __v3
                            });
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read();
                    'l6: loop {
                        if !(((i) as i32) < 160i32) {
                            break 'l6;
                        }
                        'l7: {
                            ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                            .write({
                                let __v4 = ((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(2))
                                .read()) as u16);
                                (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                                    .wrapping_offset(1920))
                                .cast::<u16>())
                                .wrapping_offset(((i) as i32) as isize))
                                .write(__v4);
                                __v4
                            });
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                    == 0i32
                {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write({
                        let __v5 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                            .read()) as u16);
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(__v5);
                        __v5
                    });
                } else {
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write({
                        let __v6 = ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                            .read()) as u16);
                        (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>())
                            .wrapping_offset(1920))
                        .cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                        .write(__v6);
                        __v6
                    });
                }
                (((&raw mut params).cast::<u8>()).cast::<*mut u8>())
                    .write(((67108946i32) as usize as *mut u16).cast::<u8>());
                (((&raw mut params).cast::<u8>())
                    .wrapping_add(4)
                    .cast::<u32>())
                .write(2724200449u32);
                (((&raw mut params).cast::<u8>()).wrapping_add(8)).write(1u8);
                (((&raw mut params).cast::<u8>()).wrapping_add(9)).write(0u8);
                ScanlineEffect_SetParams(
                    (&raw mut params)
                        .cast::<u8>()
                        .cast::<crate::c::Rec4<12>>()
                        .read_unaligned(),
                );
                let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                (__p7).write(((__p7).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    == 0i32
                {
                    if (({
                        let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        let __t9 = ((__p8).read()).wrapping_sub(1);
                        (__p8).write(__t9);
                        __t9
                    }) as i32)
                        <= 0i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                        let __p10 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p10).write(((__p10).read()).wrapping_add(1));
                    }
                } else {
                    if (({
                        let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                        let __t12 = ((__p11).read()).wrapping_add(1);
                        (__p11).write(__t12);
                        __t12
                    }) as i32)
                        > 111i32
                    {
                        let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p13).write(((__p13).read()).wrapping_add(1));
                    }
                }
                {
                    i = 0i16;
                    'l8: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32))
                        {
                            break 'l8;
                        }
                        'l9: {
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
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read();
                    'l10: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32))
                        {
                            break 'l10;
                        }
                        'l11: {
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
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read();
                    'l12: loop {
                        if !(((i) as i32) < 160i32) {
                            break 'l12;
                        }
                        'l13: {
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
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                {
                    i = 0i16;
                    'l14: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4))
                                .read()) as i32))
                        {
                            break 'l14;
                        }
                        'l15: {
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
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read();
                    'l16: loop {
                        if !(((i) as i32)
                            < ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5))
                                .read()) as i32))
                        {
                            break 'l16;
                        }
                        'l17: {
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
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                {
                    i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read();
                    'l18: loop {
                        if !(((i) as i32) < 160i32) {
                            break 'l18;
                        }
                        'l19: {
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
                                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                    .read()) as u16),
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    == (-1i32)
                {
                    ScanlineEffect_Stop();
                    DestroyTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSmallDriftingBubbles(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut randData: i16 = 0i16;
        let mut randData2: i16 = 0i16;
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(8i32)) as u16) as i32,
        );
        InitSpritePosToAnimTarget(sprite, 1u8);
        randData = (((((Random2()) as i32) & 255i32) | 256i32) as i16);
        randData2 = ((((Random2()) as i32) & 511i32) as i16);
        if ((randData2) as i32) > 255i32 {
            randData2 = (((256i32).wrapping_sub(((randData2) as i32))) as i16);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(randData);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(randData2);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSmallDriftingBubbles_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSmallDriftingBubbles_Step(sprite: *mut u8) {
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
pub unsafe extern "C" fn AnimTask_WaterSpoutLaunch(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
            .write(((GetWaterSpoutPowerForAnim()) as i16));
        PrepareBattlerSpriteForRotScale(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
            0u8,
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_WaterSpoutLaunch_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_WaterSpoutLaunch_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
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
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    if ((({
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
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
                        let __p7 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(34)
                        .cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
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
                if ((UpdateEruptAnimTask(task)) as i32) == 0i32 {
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
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    let __p8 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p8).write(((__p8).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                if (({
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    > 4i32
                {
                    PrepareEruptAnimTaskData(
                        task,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as u8),
                        224i16,
                        512i16,
                        384i16,
                        224i16,
                        8u16,
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    let __p11 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                if ((UpdateEruptAnimTask(task)) as i32) == 0i32 {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    let __p12 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                __fall = true;
                CreateWaterSpoutLaunchDroplets(task, taskId);
                let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                (__p13).write(((__p13).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 5i32 {
                __fall = true;
                if (({
                    let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    let __t15 = ((__p14).read()).wrapping_add(1);
                    (__p14).write(__t15);
                    __t15
                }) as i32)
                    > 1i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                    if ((({
                        let __p16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                        let __t17 = ((__p16).read()).wrapping_add(1);
                        (__p16).write(__t17);
                        __t17
                    }) as i32)
                        & 1i32)
                        != 0
                    {
                        let __p18 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p18).write((((((__p18).read()) as i32).wrapping_add(2i32)) as i16));
                    } else {
                        let __p19 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(38)
                        .cast::<i16>();
                        (__p19).write((((((__p19).read()) as i32).wrapping_sub(2i32)) as i16));
                    }
                    if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        == 10i32
                    {
                        PrepareEruptAnimTaskData(
                            task,
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                                as u8),
                            384i16,
                            224i16,
                            256i16,
                            256i16,
                            8u16,
                        );
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                        let __p20 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p20).write(((__p20).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                __fall = true;
                let __p21 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(34)
                .cast::<i16>();
                (__p21).write(((__p21).read()).wrapping_sub(1));
                if ((UpdateEruptAnimTask(task)) as i32) == 0i32 {
                    ResetSpriteRotScale(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as u8),
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(34)
                    .cast::<i16>())
                    .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read());
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
                    let __p22 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p22).write(((__p22).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 7i32 {
                __fall = true;
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn GetWaterSpoutPowerForAnim() -> u8 {
    unsafe {
        let mut i: u8 = 0u8;
        let mut hp: u16 = 0u16;
        let mut maxhp: u16 = 0u16;
        let mut partyIndex: u16 = 0u16;
        let mut slot: *mut u8 = core::ptr::null_mut();
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            partyIndex = ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
            .read();
            slot = ((&raw mut gPlayerParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100);
            maxhp = ((GetMonData2(slot, 58i32)) as u16);
            hp = ((GetMonData2(slot, 57i32)) as u16);
            maxhp = ((crate::c::div_i32(((maxhp) as i32), 4i32)) as u16);
        } else {
            partyIndex = ((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                .wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
            .read();
            slot = ((&raw mut gEnemyParty).cast::<u8>())
                .wrapping_offset(((partyIndex) as i32) as isize * 100);
            maxhp = ((GetMonData2(slot, 58i32)) as u16);
            hp = ((GetMonData2(slot, 57i32)) as u16);
            maxhp = ((crate::c::div_i32(((maxhp) as i32), 4i32)) as u16);
        }
        {
            i = 0u8;
            'l1: loop {
                if !(((i) as i32) < 3i32) {
                    break 'l1;
                }
                'l2: {
                    if ((hp) as i32)
                        < ((maxhp) as i32).wrapping_mul(((i) as i32).wrapping_add(1i32))
                    {
                        return i;
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        return 3u8;
    }
}
pub(crate) unsafe extern "C" fn CreateWaterSpoutLaunchDroplets(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut i: i16 = 0i16;
        let mut attackerCoordX: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16);
        let mut attackerCoordY: i16 =
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16);
        let mut trigIndex: i16 = 172i16;
        let mut subpriority: u8 =
            ((((GetBattlerSpriteSubpriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                as i32)
                .wrapping_sub(1i32)) as u8);
        let mut increment: i16 = (((4i32).wrapping_sub(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
        )) as i16);
        let mut spriteId: u8 = 0u8;
        if ((increment) as i32) <= 0i32 {
            increment = 1i16;
        }
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) < 20i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const gSmallWaterOrbSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        attackerCoordX,
                        attackerCoordY,
                        subpriority,
                    );
                    if ((spriteId) as i32) != 64i32 {
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .write(i);
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(((((attackerCoordX) as i32).wrapping_mul(16i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(3))
                        .write(((((attackerCoordY) as i32).wrapping_mul(16i32)) as i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(Cos(trigIndex, 64i16));
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(Sin(trigIndex, 64i16));
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
                        .write(2i16);
                        if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            & 1i32)
                            != 0
                        {
                            AnimSmallWaterOrb(
                                ((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                            );
                        }
                        let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        (__p1).write(((__p1).read()).wrapping_add(1));
                    }
                    trigIndex = ((((trigIndex) as i32)
                        .wrapping_add(((increment) as i32).wrapping_mul(2i32)))
                        as i16);
                    trigIndex = ((((trigIndex) as i32) & 255i32) as i16);
                }
                i = ((((i) as i32).wrapping_add(((increment) as i32))) as i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSmallWaterOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            let mut __fall = false;
            if __sw1 == 0i32 {
                __fall = true;
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        (crate::c::rem_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32),
                            6i32,
                        ))
                        .wrapping_mul(3i32),
                    )) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        (crate::c::rem_i32(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32),
                            3i32,
                        ))
                        .wrapping_mul(3i32),
                    )) as i16),
                );
                let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 1i32 {
                __fall = true;
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                (__p5).write(
                    (((((__p5).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        >> 4) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        >> 4) as i16),
                );
                if (((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) < (-8i32))
                    || (((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) > 248i32))
                    || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-8i32)))
                    || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 120i32)
                {
                    let __p7 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize
                            * 40,
                    ))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32) as isize,
                    );
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                    DestroySprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_WaterSpoutRain(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
            .write(((GetWaterSpoutPowerForAnim()) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(136i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(40i16);
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(16i16);
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(80i16);
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(98i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_add(49i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
            (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                .wrapping_mul(5i32))
            .wrapping_add(5i32)) as i16),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_WaterSpoutRain_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_WaterSpoutRain_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut taskId2: u8 = 0u8;
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 2i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    CreateWaterSpoutRainDroplet(task, taskId);
                }
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                    as i32)
                    != 0i32)
                    && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                        as i32)
                        == 0i32)
                {
                    (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(1i16);
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                        .write(0i16);
                    ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                        .write(12i16);
                    taskId2 = CreateTask(Some(AnimTask_HorizontalShake), 80u8);
                    if ((taskId2) as i32) != 255i32 {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId2) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .read())
                        .unwrap_unchecked()(taskId2);
                        let __p4 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                    (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(3i16);
                    taskId2 = CreateTask(Some(AnimTask_HorizontalShake), 80u8);
                    if ((taskId2) as i32) != 255i32 {
                        (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId2) as i32) as isize * 40))
                        .cast::<Option<unsafe extern "C" fn(u8)>>())
                        .read())
                        .unwrap_unchecked()(taskId2);
                        let __p5 = (&raw mut gAnimVisualTaskCount).cast::<u8>();
                        (__p5).write(((__p5).read()).wrapping_add(1));
                    }
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(1i16);
                }
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                        as i32)
                {
                    let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWaterSpoutRainDroplet(task: *mut u8, taskId: u8) {
    unsafe {
        let mut task = task;
        let mut taskId = taskId;
        let mut yPosArg: u16 = (((((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
            .wrapping_offset(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32)
                    as isize,
            ))
        .read()) as i32)
            .wrapping_add(3i32)
            >> 4)
            .wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as u16);
        let mut spriteId: u8 = CreateSprite(
            (&raw const gSmallWaterOrbSpriteTemplate)
                .cast::<u8>()
                .cast_mut(),
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read(),
            0i16,
            0u8,
        );
        if ((spriteId) as i32) != 64i32 {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimWaterSpoutRain));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((yPosArg) as i16));
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
            .write(9i16);
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
        (__p2).write(((__p2).read()).wrapping_add(1));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32)
                .wrapping_add(39i32)
                & 255i32) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            (((crate::c::rem_i32(
                ((1103515245i32).wrapping_mul(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
                ))
                .wrapping_add(12345i32),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            ))
            .wrapping_add(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
            )) as i16),
        );
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSpoutRain(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            let __p1 = (sprite).wrapping_add(34).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                >= ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
            {
                ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((CreateSprite(
                        (&raw mut gWaterHitSplatSpriteTemplate).cast::<u8>(),
                        ((sprite).wrapping_add(32).cast::<i16>()).read(),
                        ((sprite).wrapping_add(34).cast::<i16>()).read(),
                        1u8,
                    )) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    != 64i32
                {
                    StartSpriteAffineAnim(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32) as isize
                                * 68,
                        ),
                        3u8,
                    );
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(6))
                    .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read());
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(AnimWaterSpoutRainHit));
                }
                DestroySprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSpoutRainHit(sprite: *mut u8) {
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
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32)
                    ^ 1i32) as u16) as i32,
            );
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 12i32
            {
                let __p5 = (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize
                        * 40,
                ))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize,
                );
                (__p5).write(((__p5).read()).wrapping_sub(1));
                FreeOamMatrix(
                    ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                );
                DestroySprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_WaterSport(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(
            ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32
            {
                1i32
            } else {
                (-1i32)
            }) as i16),
        );
        if (IsContest()) != 0 {
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                .wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                        .wrapping_mul(8i32),
                )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(
            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                .wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                        .wrapping_mul(8i32),
                )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write((-32i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_WaterSport_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_WaterSport_Step(taskId: u8) {
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
            if __sw1 == 0i32 {
                CreateWaterSportDroplet(task);
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                    != 0i32
                {
                    let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                CreateWaterSportDroplet(task);
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_add(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    > 16i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                CreateWaterSportDroplet(task);
                let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(6i32),
                    )) as i16),
                );
                if !((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >= (-16i32))
                    && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        <= 256i32))
                {
                    if (({
                        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                        let __t8 = ((__p7).read()).wrapping_add(1);
                        (__p7).write(__t8);
                        __t8
                    }) as i32)
                        > 2i32
                    {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(1i16);
                        (((task).wrapping_add(8)).cast::<i16>()).write(6i16);
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    } else {
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                        let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p9).write(((__p9).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                CreateWaterSportDroplet(task);
                let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p10).write(
                    (((((__p10).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(2i32),
                    )) as i16),
                );
                if (({
                    let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t12 = ((__p11).read()).wrapping_add(1);
                    (__p11).write(__t12);
                    __t12
                }) as i32)
                    > 7i32
                {
                    let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p13).write(((__p13).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                CreateWaterSportDroplet(task);
                let __p14 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5);
                (__p14).write(
                    (((((__p14).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            .wrapping_mul(6i32),
                    )) as i16),
                );
                if !((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >= (-16i32))
                    && (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        <= 256i32))
                {
                    let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
                    (__p15).write(((__p15).read()).wrapping_add(1));
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p16 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p16).write(((__p16).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 5i32 {
                CreateWaterSportDroplet(task);
                let __p17 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p17).write(
                    (((((__p17).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
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
                    > 7i32
                {
                    (((task).wrapping_add(8)).cast::<i16>()).write(2i16);
                }
                break 'l1;
            }
            if __sw1 == 6i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32)
                    == 0i32
                {
                    let __p20 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p20).write(((__p20).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if !__matched {
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateWaterSportDroplet(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spriteId: u8 = 0u8;
        if (({
            let __p1 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            > 1i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
            spriteId = CreateSprite(
                (&raw const gSmallWaterOrbSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read(),
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read(),
                10u8,
            );
            if ((spriteId) as i32) != 64i32 {
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(16i16);
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(2))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read());
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(4))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read());
                ((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(5))
                .write(((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read());
                InitAnimArcTranslation(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68),
                );
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimWaterSportDroplet));
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                (__p3).write(((__p3).read()).wrapping_add(1));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSportDroplet(sprite: *mut u8) {
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
            (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((Random2()) as i32) & 31i32).wrapping_sub(16i32))
                    .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((Random2()) as i32) & 31i32).wrapping_sub(16i32))
                    .wrapping_add(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
                .write(((!(((Random2()) as i32) & 7i32)) as i16));
            InitAnimArcTranslation(sprite);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimWaterSportDroplet_Step));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterSportDroplet_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u16 = 0u16;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            {
                i = 0u16;
                'l1: loop {
                    if !(((i) as i32) < 16i32) {
                        break 'l1;
                    }
                    'l2: {
                        if core::mem::transmute::<_, usize>(
                            ((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 40))
                            .cast::<Option<unsafe extern "C" fn(u8)>>())
                            .read(),
                        ) == (AnimTask_WaterSport_Step as *const () as usize)
                        {
                            ((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(10))
                            .write(1i16);
                            let __p1 = (((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((i) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(8);
                            (__p1).write(((__p1).read()).wrapping_sub(1));
                            DestroySprite(sprite);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterPulseBubble(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
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
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWaterPulseBubble_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimWaterPulseBubble_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32),
                10i32,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )
                & 255i32) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
        ));
        if (({
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t3 = ((__p2).read()).wrapping_sub(1);
            (__p2).write(__t3);
            __t3
        }) as i32)
            == 0i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimWaterPulseRingBubble(sprite: *mut u8) {
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
                >> 7) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 7) as i16),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_sub(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            == 0i32
        {
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimWaterPulseRing(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimAttacker(sprite, 1u8);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimWaterPulseRing_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimWaterPulseRing_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut xDiff: i32 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .read()) as i32)
            .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32));
        let mut yDiff: i32 = ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .read()) as i32)
            .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32));
        ((sprite).wrapping_add(36).cast::<i16>()).write(
            ((crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(xDiff),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            ((crate::c::div_i32(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_mul(yDiff),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            )) as i16),
        );
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            CreateWaterPulseRingBubbles(sprite, xDiff, yDiff);
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
        let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p3).write(((__p3).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn CreateWaterPulseRingBubbles(
    sprite: *mut u8,
    xDiff: i32,
    yDiff: i32,
) {
    unsafe {
        let mut sprite = sprite;
        let mut xDiff = xDiff;
        let mut yDiff = yDiff;
        let mut combinedX: i16 = 0i16;
        let mut combinedY: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut something: i16 = 0i16;
        let mut unusedVar: i16 = 1i16;
        let mut randomSomethingY: i16 = 0i16;
        let mut randomSomethingX: i16 = 0i16;
        let mut spriteId: u8 = 0u8;
        something = ((crate::c::div_i32(
            (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
            2i32,
        )) as i16);
        combinedX = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
            as i16);
        combinedY = ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
            .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
            as i16);
        if yDiff < 0i32 {
            unusedVar = ((((unusedVar) as i32).wrapping_mul((-1i32))) as i16);
        }
        randomSomethingY = ((((yDiff).wrapping_add(crate::c::rem_i32(((Random2()) as i32), 10i32)))
            .wrapping_sub(5i32)) as i16);
        randomSomethingX = (((((xDiff).wrapping_neg())
            .wrapping_add(crate::c::rem_i32(((Random2()) as i32), 10i32)))
        .wrapping_sub(5i32)) as i16);
        {
            i = 0i16;
            'l1: loop {
                if !(((i) as i32) <= 0i32) {
                    break 'l1;
                }
                'l2: {
                    spriteId = CreateSprite(
                        (&raw const gWaterPulseRingBubbleSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        combinedX,
                        ((((combinedY) as i32).wrapping_add(((something) as i32))) as i16),
                        130u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(20i16);
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(randomSomethingY);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(67))
                    .write(
                        ((((GetBattlerSpriteSubpriority(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        )) as i32)
                            .wrapping_sub(1i32)) as u8),
                    );
                    if ((randomSomethingX) as i32) < 0i32 {
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(((((randomSomethingX) as i32).wrapping_neg()) as i16));
                    } else {
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(randomSomethingX);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
        {
            i = 0i16;
            'l3: loop {
                if !(((i) as i32) <= 0i32) {
                    break 'l3;
                }
                'l4: {
                    spriteId = CreateSprite(
                        (&raw const gWaterPulseRingBubbleSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        combinedX,
                        ((((combinedY) as i32).wrapping_sub(((something) as i32))) as i16),
                        130u8,
                    );
                    (((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .write(20i16);
                    ((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .write(randomSomethingY);
                    ((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((spriteId) as i32) as isize * 68))
                    .wrapping_add(67))
                    .write(
                        ((((GetBattlerSpriteSubpriority(
                            ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        )) as i32)
                            .wrapping_sub(1i32)) as u8),
                    );
                    if ((randomSomethingX) as i32) > 0i32 {
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(((((randomSomethingX) as i32).wrapping_neg()) as i16));
                    } else {
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(2))
                        .write(randomSomethingX);
                    }
                }
                i = (i).wrapping_add(1);
            }
        }
    }
}
