//! Translated from `src/battle_anim_rock.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_FlyingRock_0 sAnim_FlyingRock_1 sAnim_FlyingRock_2 sAnims_FlyingRock gFallingRockSpriteTemplate gRockFragmentSpriteTemplate gSwirlingDirtSpriteTemplate sAffineAnim_Whirlpool sAffineAnims_Whirlpool gWhirlpoolSpriteTemplate gFireSpinSpriteTemplate gFlyingSandCrescentSpriteTemplate sFlyingSandSubsprites sFlyingSandSubspriteTable sAnim_Rock_Biggest sAnim_Rock_Bigger sAnim_Rock_Big sAnim_Rock_Small sAnim_Rock_Smaller sAnim_Rock_Smallest sAnims_BasicRock gAncientPowerRockSpriteTemplate gRolloutMudSpriteTemplate gRolloutRockSpriteTemplate gRockTombRockSpriteTemplate sAffineAnim_BasicRock_0 sAffineAnim_BasicRock_1 sAffineAnims_BasicRock gRockBlastRockSpriteTemplate gRockScatterSpriteTemplate gTwisterRockSpriteTemplate gWeatherBallRockDownSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_rock::*;

unsafe extern "C" {
    static mut gAnimDisableStructPtr: u8;
    static mut gAnimMoveDmg: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimBgImage_Sandstorm: u8;
    static mut gBattleAnimBgTilemap_Sandstorm: u8;
    static mut gBattleAnimSpritePal_FlyingDirt: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG1_Y: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimLoadCompressedBgGfx(a0: u32, a1: *mut u32, a2: u32);
    fn AnimLoadCompressedBgTilemapHandleContest(a0: *mut u8, a1: *mut u8, a2: u32);
    fn AnimateSprite(a0: *mut u8);
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn ClearBattleAnimBg(a0: u32);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn FindTaskIdByFunc(a0: Option<unsafe extern "C" fn(u8)>) -> u8;
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattleAnimBg1Data(a0: *mut u8);
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitSpriteDataForLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsContest() -> u8;
    fn LoadCompressedPalette(a0: *mut u32, a1: u16, a2: u16);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn SetAnimBgAttribute(a0: u8, a1: u8, a2: u8);
    fn SetAverageBattlerPositions(a0: u8, a1: u8, a2: *mut i16, a3: *mut i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSubspriteTables(a0: *mut u8, a1: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn TranslateAnimSpriteToTargetMonLocation(a0: *mut u8);
    fn TranslateSpriteInEllipse(a0: *mut u8);
    fn TranslateSpriteLinearFixedPoint(a0: *mut u8);
    fn UpdateAnimBg3ScreenSize(a0: u8);
}

pub(crate) unsafe extern "C" fn AnimFallingRock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            != 0i32
        {
            SetAverageBattlerPositions(
                ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                0u8,
                (sprite).wrapping_add(32).cast::<i16>(),
                (sprite).wrapping_add(34).cast::<i16>(),
            );
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(14i32)) as i16));
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                .read()) as u8),
        );
        AnimateSprite(sprite);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(4i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(16i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-70i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        StoreSpriteCallbackInData6(sprite, Some(AnimFallingRock_Step));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteInEllipse));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimFallingRock_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(192i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(4i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(32i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((-24i16));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteInEllipse));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimRockFragment(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5))
                .read()) as u8),
        );
        AnimateSprite(sprite);
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as i32),
            )) as i16),
        );
        InitSpriteDataForLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(TranslateSpriteLinearFixedPoint));
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
    }
}
pub(crate) unsafe extern "C" fn AnimParticleInVortex(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).read())
            as i32)
            == 0i32
        {
            InitSpritePosToAnimAttacker(sprite, 0u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 0u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
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
        .write(Some(AnimParticleInVortex_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimParticleInVortex_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(38).cast::<i16>()).write(
            (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                >> 8)
                .wrapping_neg()) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
        ));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )
                & 255i32) as i16),
        );
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
pub unsafe extern "C" fn AnimTask_LoadSandstormBackground(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: i32 = 0i32;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        var0 = 0i32;
        SetGpuReg(80u8, 16194u16);
        SetGpuReg(82u8, 4096u16);
        SetAnimBgAttribute(1u8, 4u8, 1u8);
        SetAnimBgAttribute(1u8, 0u8, 0u8);
        if !((IsContest()) != 0) {
            SetAnimBgAttribute(1u8, 3u8, 1u8);
        }
        ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
        ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
        SetGpuReg(20u8, ((&raw mut gBattle_BG1_X).cast::<u16>()).read());
        SetGpuReg(22u8, ((&raw mut gBattle_BG1_Y).cast::<u16>()).read());
        GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
        AnimLoadCompressedBgGfx(
            (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
            ((&raw mut gBattleAnimBgImage_Sandstorm).cast::<u32>()).cast::<u32>(),
            (((((&raw mut animBg).cast::<u8>())
                .wrapping_add(10)
                .cast::<u16>())
            .read()) as u32),
        );
        AnimLoadCompressedBgTilemapHandleContest(
            (&raw mut animBg).cast::<u8>(),
            (((&raw mut gBattleAnimBgTilemap_Sandstorm).cast::<u32>()).cast::<u32>()).cast::<u8>(),
            0u32,
        );
        LoadCompressedPalette(
            ((&raw mut gBattleAnimSpritePal_FlyingDirt).cast::<u32>()).cast::<u32>(),
            (((0i32).wrapping_add(
                (((((&raw mut animBg).cast::<u8>()).wrapping_add(8)).read()) as i32)
                    .wrapping_mul(16i32),
            )) as u16),
            32u16,
        );
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) != 0)
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32)
        {
            var0 = 1i32;
        }
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((var0) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_LoadSandstormBackground_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_LoadSandstormBackground_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut animBg = crate::ffi::Align4([0u8; 16]);
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            let __p1 = (&raw mut gBattle_BG1_X).cast::<u16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_add((-6i32))) as u16));
        } else {
            let __p2 = (&raw mut gBattle_BG1_X).cast::<u16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(6i32)) as u16));
        }
        let __p3 = (&raw mut gBattle_BG1_Y).cast::<u16>();
        (__p3).write((((((__p3).read()) as i32).wrapping_add((-1i32))) as u16));
        'l1: {
            let __sw4 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(12))
            .read()) as i32);
            if __sw4 == 0i32 {
                if (({
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p7 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32)
                        == 7i32
                    {
                        let __p8 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p8).write(((__p8).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw4 == 1i32 {
                if (({
                    let __p9 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    let __t10 = ((__p9).read()).wrapping_add(1);
                    (__p9).write(__t10);
                    __t10
                }) as i32)
                    == 101i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .write(7i16);
                    let __p11 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(12);
                    (__p11).write(((__p11).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw4 == 2i32 {
                if (({
                    let __p12 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    let __t13 = ((__p12).read()).wrapping_add(1);
                    (__p12).write(__t13);
                    __t13
                }) as i32)
                    == 4i32
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(0i16);
                    let __p14 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11);
                    (__p14).write(((__p14).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32),
                        ) << 8)
                            | ((((((((&raw mut gTasks).cast::<u8>())
                                .wrapping_offset(((taskId) as i32) as isize * 40))
                            .wrapping_add(8))
                            .cast::<i16>())
                            .wrapping_offset(11))
                            .read()) as i32)) as u16),
                    );
                    if ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(11))
                    .read()) as i32)
                        == 0i32
                    {
                        let __p15 = (((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(12);
                        (__p15).write(((__p15).read()).wrapping_add(1));
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(11))
                        .write(0i16);
                    }
                }
                break 'l1;
            }
            if __sw4 == 3i32 {
                GetBattleAnimBg1Data((&raw mut animBg).cast::<u8>());
                ClearBattleAnimBg(
                    (((((&raw mut animBg).cast::<u8>()).wrapping_add(9)).read()) as u32),
                );
                let __p16 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(12);
                (__p16).write(((__p16).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw4 == 4i32 {
                if !((IsContest()) != 0) {
                    SetAnimBgAttribute(1u8, 3u8, 0u8);
                }
                ((&raw mut gBattle_BG1_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG1_Y).cast::<u16>()).write(0u16);
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                SetAnimBgAttribute(1u8, 4u8, 1u8);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimFlyingSandCrescent(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .read()) as i32)
                != 0i32)
                && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    != 0i32)
            {
                ((sprite).wrapping_add(32).cast::<i16>()).write(304i16);
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_neg()) as i16),
                    );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(1i16);
                crate::c::bf_write((sprite).wrapping_add(3), 1, 5, (8u32) as i32);
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write((-64i16));
            }
            ((sprite).wrapping_add(34).cast::<i16>())
                .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
            SetSubspriteTables(
                sprite,
                ((&raw const sFlyingSandSubspriteTable)
                    .cast::<u8>()
                    .cast_mut())
                .cast::<u8>(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p3).write(
                (((((__p3).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32),
                )) as i16),
            );
            let __p4 = (sprite).wrapping_add(36).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            let __p5 = (sprite).wrapping_add(38).cast::<i16>();
            (__p5).write(
                (((((__p5).read()) as i32).wrapping_add(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        >> 8),
                )) as i16),
            );
            let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p6).write((((((__p6).read()) as i32) & 255i32) as i16));
            let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p7).write((((((__p7).read()) as i32) & 255i32) as i16));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 0i32
            {
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                    > 272i32
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(DestroyAnimSprite));
                }
            } else {
                if ((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32))
                    < (-32i32)
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(DestroyAnimSprite));
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRaiseSprite(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u8),
        );
        InitSpritePosToAnimAttacker(sprite, 0u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Rollout(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: u16 = 0u16;
        let mut var1: u16 = 0u16;
        let mut var2: u16 = 0u16;
        let mut var3: u16 = 0u16;
        let mut rolloutCounter: u8 = 0u8;
        let mut pan1: i16 = 0i16;
        let mut pan2: i16 = 0i16;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        var0 = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
            as u16);
        var1 =
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_add(24i32)) as u16);
        var2 = ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
            as u16);
        var3 = ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
            as i32)
            .wrapping_add(24i32)) as u16);
        if (((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
            == ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32)
        {
            var3 = var1;
        }
        rolloutCounter = GetRolloutCounter();
        if ((rolloutCounter) as i32) == 1i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(32i16);
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).write(
                (((48i32).wrapping_sub(((rolloutCounter) as i32).wrapping_mul(8i32))) as i16),
            );
        }
        (((task).wrapping_add(8)).cast::<i16>()).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(1i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(
            (((crate::c::div_i32(
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32),
                8i32,
            ))
            .wrapping_sub(1i32)) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
            .write(((((var0) as i32).wrapping_mul(8i32)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3))
            .write(((((var1) as i32).wrapping_mul(8i32)) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(
            ((crate::c::div_i32(
                (((var2) as i32).wrapping_sub(((var0) as i32))).wrapping_mul(8i32),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).write(
            ((crate::c::div_i32(
                (((var3) as i32).wrapping_sub(((var1) as i32))).wrapping_mul(8i32),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        pan1 = ((BattleAnimAdjustPanning((-64i8))) as i16);
        pan2 = ((BattleAnimAdjustPanning(63i8)) as i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(pan1);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((crate::c::div_i32(
                ((pan2) as i32).wrapping_sub(((pan1) as i32)),
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8)).read()) as i32),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
            .write(((rolloutCounter) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
            .write(((GetAnimBattlerSpriteId(0u8)) as i16));
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_Rollout_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Rollout_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 = core::ptr::null_mut();
        task = ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_sub(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        >> 3) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        >> 3) as i16),
                );
                if (({
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t5 = ((__p4).read()).wrapping_add(1);
                    (__p4).write(__t5);
                    __t5
                }) as i32)
                    == 10i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(20i16);
                    let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p6).write(((__p6).read()).wrapping_add(1));
                }
                PlaySE12WithPanning(
                    162u16,
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as i8),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
                    let __t8 = ((__p7).read()).wrapping_sub(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    == 0i32
                {
                    let __p9 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t11 = ((__p10).read()).wrapping_sub(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    != 0i32
                {
                    let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6);
                    (__p12).write(
                        (((((__p12).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                                as i32),
                        )) as i16),
                    );
                    let __p13 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7);
                    (__p13).write(
                        (((((__p13).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                                as i32),
                        )) as i16),
                    );
                } else {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).write(0i16);
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).write(0i16);
                    let __p14 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p14).write(((__p14).read()).wrapping_add(1));
                }
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(36)
                .cast::<i16>())
                .write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        >> 3) as i16),
                );
                ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ))
                .wrapping_add(38)
                .cast::<i16>())
                .write(
                    ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        >> 3) as i16),
                );
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p15 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                (__p15).write(
                    (((((__p15).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                let __p16 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                (__p16).write(
                    (((((__p16).read()) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(5)).read())
                            as i32),
                    )) as i16),
                );
                if (({
                    let __p17 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                    let __t18 = ((__p17).read()).wrapping_add(1);
                    (__p17).write(__t18);
                    __t18
                }) as i32)
                    >= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                        as i32)
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).write(0i16);
                    CreateRolloutDirtSprite(task);
                    let __p19 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13);
                    (__p19).write(
                        (((((__p19).read()) as i32).wrapping_add(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                                as i32),
                        )) as i16),
                    );
                    PlaySE12WithPanning(
                        175u16,
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read())
                            as i8),
                    );
                }
                if (({
                    let __p20 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                    let __t21 = ((__p20).read()).wrapping_sub(1);
                    (__p20).write(__t21);
                    __t21
                }) as i32)
                    == 0i32
                {
                    let __p22 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p22).write(((__p22).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 4i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read()) as i32)
                    == 0i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn CreateRolloutDirtSprite(task: *mut u8) {
    unsafe {
        let mut task = task;
        let mut spriteTemplate: *mut u8 = core::ptr::null_mut();
        let mut tileOffset: i32 = 0i32;
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32);
            let __matched =
                __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32 || __sw1 == 4i32 || __sw1 == 5i32;
            if __sw1 == 1i32 {
                spriteTemplate = (&raw const gRolloutMudSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut();
                tileOffset = 0i32;
                break 'l1;
            }
            if __sw1 == 2i32 || __sw1 == 3i32 {
                spriteTemplate = (&raw const gRolloutRockSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut();
                tileOffset = 80i32;
                break 'l1;
            }
            if __sw1 == 4i32 {
                spriteTemplate = (&raw const gRolloutRockSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut();
                tileOffset = 64i32;
                break 'l1;
            }
            if __sw1 == 5i32 {
                spriteTemplate = (&raw const gRolloutRockSpriteTemplate)
                    .cast::<u8>()
                    .cast_mut();
                tileOffset = 48i32;
                break 'l1;
            }
            if !__matched {
                return;
            }
        }
        x = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read()) as i32) >> 3)
            as u16);
        y = ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32) >> 3)
            as u16);
        x = ((((x) as i32).wrapping_add(
            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read()) as i32)
                .wrapping_mul(4i32),
        )) as u16);
        spriteId = CreateSprite(spriteTemplate, ((x) as i16), ((y) as i16), 35u8);
        if ((spriteId) as i32) != 64i32 {
            (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .write(18i16);
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(
                ((((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                    as i32)
                    .wrapping_mul(20i32))
                .wrapping_add(((x) as i32)))
                .wrapping_add(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        .wrapping_mul(3i32),
                )) as i16),
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((y) as i16));
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(
                (((-16i32).wrapping_sub(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                        .wrapping_mul(2i32),
                )) as i16),
            );
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
                    .wrapping_add(tileOffset)) as u16) as i32,
            );
            InitAnimArcTranslation(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11);
            (__p2).write(((__p2).read()).wrapping_add(1));
        }
        let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12);
        (__p3).write((((((__p3).read()) as i32).wrapping_mul((-1i32))) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimRolloutParticle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            let mut taskId: u8 = FindTaskIdByFunc(Some(AnimTask_Rollout_Step));
            if ((taskId) as i32) != 255i32 {
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(11);
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
            DestroySprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn GetRolloutCounter() -> u8 {
    unsafe {
        let mut retVal: u8 = ((((crate::c::bf_read(
            (((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(17),
            4,
            4,
            false,
        ) as u8) as i32)
            .wrapping_sub(
                ((crate::c::bf_read(
                    (((&raw mut gAnimDisableStructPtr).cast::<*mut u8>()).read()).wrapping_add(17),
                    0,
                    4,
                    false,
                ) as u8) as i32),
            )) as u8);
        let mut var0: u8 = ((((retVal) as i32).wrapping_sub(1i32)) as u8);
        if ((var0) as i32) > 4i32 {
            retVal = 1u8;
        }
        return retVal;
    }
}
pub(crate) unsafe extern "C" fn AnimRockTomb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u8),
        );
        ((sprite).wrapping_add(36).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_sub(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(3i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRockTomb_Step));
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
    }
}
pub(crate) unsafe extern "C" fn AnimRockTomb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (0u16) as i32);
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            != 0i32
        {
            ((sprite).wrapping_add(38).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                    )) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(
                (((((__p1).read()) as i32)
                    .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p2).write(((__p2).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 0i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            }
        } else {
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                let __t4 = ((__p3).read()).wrapping_sub(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 0i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRockBlastRock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            StartSpriteAffineAnim(sprite, 1u8);
        }
        TranslateAnimSpriteToTargetMonLocation(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimRockScatter(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                .read()) as u8),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRockScatter_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRockScatter_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(8i32)) as i16));
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p3).write(
            (((((__p3).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32),
            )) as i16),
        );
        let __p4 = (sprite).wrapping_add(36).cast::<i16>();
        (__p4).write(
            (((((__p4).read()) as i32).wrapping_add(crate::c::div_i32(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
                40i32,
            ))) as i16),
        );
        let __p5 = (sprite).wrapping_add(38).cast::<i16>();
        (__p5).write(
            (((((__p5).read()) as i32).wrapping_sub(
                ((Sin(
                    (((sprite).wrapping_add(46)).cast::<i16>()).read(),
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                )) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 140i32 {
            DestroyAnimSprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_GetSeismicTossDamageLevel(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((&raw mut gAnimMoveDmg).cast::<i32>()).read() < 33i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(0i16);
        }
        if ((((&raw mut gAnimMoveDmg).cast::<i32>()).read()) as u32).wrapping_sub(33u32) < 33u32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(1i16);
        }
        if ((&raw mut gAnimMoveDmg).cast::<i32>()).read() > 65i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
                .write(2i16);
        }
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoveSeismicTossBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            UpdateAnimBg3ScreenSize(0u8);
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(200i16);
        }
        let __p1 = (&raw mut gBattle_BG3_Y).cast::<u16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32),
                10i32,
            ))) as u16),
        );
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_sub(3i32)) as i16));
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 120i32
        {
            UpdateAnimBg3ScreenSize(1u8);
            DestroyAnimVisualTask(taskId);
        }
        let __p3 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p3).write(((__p3).read()).wrapping_add(1));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SeismicTossBgAccelerateDownAtEnd(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            UpdateAnimBg3ScreenSize(0u8);
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((((&raw mut gBattle_BG3_Y).cast::<u16>()).read()) as i16));
        }
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32).wrapping_add(80i32)) as i16));
        let __p3 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p3).write((((((__p3).read()) as i32) & 255i32) as i16));
        ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(
            ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                .wrapping_add(
                    ((Cos(
                        4i16,
                        ((((((&raw mut gTasks).cast::<u8>())
                            .wrapping_offset(((taskId) as i32) as isize * 40))
                        .wrapping_add(8))
                        .cast::<i16>())
                        .wrapping_offset(1))
                        .read(),
                    )) as i32),
                )) as u16),
        );
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            == 4095i32
        {
            ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
            UpdateAnimBg3ScreenSize(1u8);
            DestroyAnimVisualTask(taskId);
        }
    }
}
