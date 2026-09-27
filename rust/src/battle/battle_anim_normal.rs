//! Translated from `src/battle_anim_normal.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAnim_ConfusionDuck_0 sAnim_ConfusionDuck_1 sAnims_ConfusionDuck gConfusionDuckSpriteTemplate gSimplePaletteBlendSpriteTemplate gComplexPaletteBlendSpriteTemplate sAnim_CirclingSparkle sAnims_CirclingSparkle sCirclingSparkleSpriteTemplate gShakeMonOrPlatformSpriteTemplate sAffineAnim_HitSplat_0 sAffineAnim_HitSplat_1 sAffineAnim_HitSplat_2 sAffineAnim_HitSplat_3 sAffineAnims_HitSplat gBasicHitSplatSpriteTemplate gHandleInvertHitSplatSpriteTemplate gWaterHitSplatSpriteTemplate gRandomPosHitSplatSpriteTemplate gMonEdgeHitSplatSpriteTemplate gCrossImpactSpriteTemplate gFlashingHitSplatSpriteTemplate gPersistHitSplatSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_normal::*;

unsafe extern "C" {
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlersCount: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gPaletteFade: u8;
    static mut gSpriteCoordOffsetX: u8;
    static mut gSpriteCoordOffsetY: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn BeginNormalPaletteFade(a0: u32, a1: i8, a2: u8, a3: u8, a4: u16) -> u8;
    fn BlendPalettes(a0: u32, a1: u8, a2: u16);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimSpriteAfterTimer(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlePalettesMask(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8, a5: u8, a6: u8) -> u32;
    fn GetBattlerSide(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn InvertPlttBuffer(a0: u32);
    fn IsContest() -> u8;
    fn Random2() -> u16;
    fn RunStoredCallbackWhenAffineAnimEnds(a0: *mut u8);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TintPlttBuffer(a0: u32, a1: i8, a2: i8, a3: i8);
    fn TranslateSpriteInGrowingCircle(a0: *mut u8);
    fn UnfadePlttBuffer(a0: u32);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimConfusionDuck(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((cmd).wrapping_add(6).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
            StartSpriteAnim(sprite, 1u8);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimConfusionDuck_Step));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimConfusionDuck_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(36).cast::<i16>()).write(Cos(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            30i16,
        ));
        ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
            (((sprite).wrapping_add(46)).cast::<i16>()).read(),
            10i16,
        ));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16) as i32) < 128i32 {
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (1u16) as i32);
        } else {
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (3u16) as i32);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            ) & 255i32) as i16),
        );
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSimplePaletteBlend(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(((cmd).cast::<i16>()).read());
        BeginNormalPaletteFade(
            selectedPalettes,
            ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i8),
            ((((cmd).wrapping_add(4).cast::<i16>()).read()) as u8),
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
            ((((cmd).wrapping_add(8).cast::<i16>()).read()) as u16),
        );
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSimplePaletteBlend_Step));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn UnpackSelectedBattlePalettes(selector: i16) -> u32 {
    unsafe {
        let mut selector = selector;
        let mut battleBackground: u8 = ((((selector) as i32) & 1i32) as u8);
        let mut attacker: u8 = (((((selector) as i32) >> 1) & 1i32) as u8);
        let mut target: u8 = (((((selector) as i32) >> 2) & 1i32) as u8);
        let mut attackerPartner: u8 = (((((selector) as i32) >> 3) & 1i32) as u8);
        let mut targetPartner: u8 = (((((selector) as i32) >> 4) & 1i32) as u8);
        let mut anim1: u8 = (((((selector) as i32) >> 5) & 1i32) as u8);
        let mut anim2: u8 = (((((selector) as i32) >> 6) & 1i32) as u8);
        return GetBattlePalettesMask(
            battleBackground,
            attacker,
            target,
            attackerPartner,
            targetPartner,
            anim1,
            anim2,
        );
    }
}
pub(crate) unsafe extern "C" fn AnimSimplePaletteBlend_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimComplexPaletteBlend(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut selectedPalettes: u32 = 0u32;
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
            .write(((cmd).wrapping_add(12).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
            .write(((cmd).cast::<i16>()).read());
        selectedPalettes = UnpackSelectedBattlePalettes(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
        );
        BlendPalettes(
            selectedPalettes,
            ((((cmd).wrapping_add(8).cast::<i16>()).read()) as u8),
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u16),
        );
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimComplexPaletteBlend_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimComplexPaletteBlend_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut selectedPalettes: u32 = 0u32;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 0i32 {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            return;
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 0i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimComplexPaletteBlend_Step2));
            return;
        }
        selectedPalettes = UnpackSelectedBattlePalettes(
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
        );
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            & 256i32)
            != 0
        {
            BlendPalettes(
                selectedPalettes,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as u8),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as u16),
            );
        } else {
            BlendPalettes(
                selectedPalettes,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8),
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as u16),
            );
        }
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32) ^ 256i32) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                & 255i32) as i16),
        );
        let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p3).write(((__p3).read()).wrapping_sub(1));
    }
}
pub(crate) unsafe extern "C" fn AnimComplexPaletteBlend_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut selectedPalettes: u32 = 0u32;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            selectedPalettes = UnpackSelectedBattlePalettes(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read(),
            );
            BlendPalettes(selectedPalettes, 0u8, 0u16);
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimCirclingSparkle(sprite: *mut u8) {
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
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(10i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(8i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(40i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(112i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendColorCycle(taskId: u8) {
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
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(0i16);
        BlendColorCycle(
            taskId,
            0u8,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_BlendColorCycleLoop));
    }
}
pub(crate) unsafe extern "C" fn BlendColorCycle(
    taskId: u8,
    startBlendAmount: u8,
    targetBlendAmount: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut startBlendAmount = startBlendAmount;
        let mut targetBlendAmount = targetBlendAmount;
        let mut selectedPalettes: u32 = UnpackSelectedBattlePalettes(
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .read(),
        );
        BeginNormalPaletteFade(
            selectedPalettes,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i8),
            startBlendAmount,
            targetBlendAmount,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u16),
        );
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8);
        (__p2).write((((((__p2).read()) as i32) ^ 1i32) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BlendColorCycleLoop(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut startBlendAmount: u8 = 0u8;
        let mut targetBlendAmount: u8 = 0u8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                > 0i32
            {
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .read())
                    != 0)
                {
                    startBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                    targetBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8);
                } else {
                    startBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8);
                    targetBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 1i32
                {
                    targetBlendAmount = 0u8;
                }
                BlendColorCycle(taskId, startBlendAmount, targetBlendAmount);
            } else {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendColorCycleExclude(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut battler: i32 = 0i32;
        let mut selectedPalettes: u32 = 0u32;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((cmd).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(0i16);
        {
            battler = 0i32;
            'l1: loop {
                if !(battler < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32)) {
                    break 'l1;
                }
                'l2: {
                    if (battler != ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32))
                        && (battler
                            != ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32))
                    {
                        selectedPalettes = (selectedPalettes
                            | ((crate::c::shl_i32(1i32, (((battler).wrapping_add(16i32)) as u32)))
                                as u32));
                    }
                }
                battler = (battler).wrapping_add(1);
            }
        }
        if ((((cmd).cast::<i16>()).read()) as i32) == 1i32 {
            selectedPalettes = (selectedPalettes | 14u32);
        }
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .write(((selectedPalettes >> 16) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(10))
        .write(((selectedPalettes & 255u32) as i16));
        BlendColorCycleExclude(
            taskId,
            0u8,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_BlendColorCycleExcludeLoop));
    }
}
pub(crate) unsafe extern "C" fn BlendColorCycleExclude(
    taskId: u8,
    startBlendAmount: u8,
    targetBlendAmount: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut startBlendAmount = startBlendAmount;
        let mut targetBlendAmount = targetBlendAmount;
        let mut selectedPalettes: u32 = ((((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(9))
        .read()) as u16) as i32)
            << 16)
            | (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(10))
            .read()) as u16) as i32)) as u32);
        BeginNormalPaletteFade(
            selectedPalettes,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i8),
            startBlendAmount,
            targetBlendAmount,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u16),
        );
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8);
        (__p2).write((((((__p2).read()) as i32) ^ 1i32) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BlendColorCycleExcludeLoop(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut startBlendAmount: u8 = 0u8;
        let mut targetBlendAmount: u8 = 0u8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                > 0i32
            {
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .read())
                    != 0)
                {
                    startBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                    targetBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8);
                } else {
                    startBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8);
                    targetBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 1i32
                {
                    targetBlendAmount = 0u8;
                }
                BlendColorCycleExclude(taskId, startBlendAmount, targetBlendAmount);
            } else {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_BlendColorCycleByTag(taskId: u8) {
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
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(0i16);
        BlendColorCycleByTag(
            taskId,
            0u8,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_BlendColorCycleByTagLoop));
    }
}
pub(crate) unsafe extern "C" fn BlendColorCycleByTag(
    taskId: u8,
    startBlendAmount: u8,
    targetBlendAmount: u8,
) {
    unsafe {
        let mut taskId = taskId;
        let mut startBlendAmount = startBlendAmount;
        let mut targetBlendAmount = targetBlendAmount;
        let mut paletteIndex: u8 = IndexOfSpritePaletteTag(
            (((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .read()) as u16),
        );
        BeginNormalPaletteFade(
            ((crate::c::shl_i32(1i32, ((((paletteIndex) as i32).wrapping_add(16i32)) as u32)))
                as u32),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i8),
            startBlendAmount,
            targetBlendAmount,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as u16),
        );
        let __p1 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_sub(1));
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8);
        (__p2).write((((((__p2).read()) as i32) ^ 1i32) as i16));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_BlendColorCycleByTagLoop(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut startBlendAmount: u8 = 0u8;
        let mut targetBlendAmount: u8 = 0u8;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            if ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as i32)
                > 0i32
            {
                if !((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .read())
                    != 0)
                {
                    startBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                    targetBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8);
                } else {
                    startBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(4))
                    .read()) as u8);
                    targetBlendAmount = ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(3))
                    .read()) as u8);
                }
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2))
                .read()) as i32)
                    == 1i32
                {
                    targetBlendAmount = 0u8;
                }
                BlendColorCycleByTag(taskId, startBlendAmount, targetBlendAmount);
            } else {
                DestroyAnimVisualTask(taskId);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_FlashAnimTagWithColor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut paletteIndex: u8 = 0u8;
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(((cmd).wrapping_add(10).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((cmd).wrapping_add(12).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((cmd).cast::<i16>()).read());
        paletteIndex = IndexOfSpritePaletteTag(((((cmd).cast::<i16>()).read()) as u16));
        BeginNormalPaletteFade(
            ((crate::c::shl_i32(1i32, ((((paletteIndex) as i32).wrapping_add(16i32)) as u32)))
                as u32),
            0i8,
            ((((cmd).wrapping_add(8).cast::<i16>()).read()) as u8),
            ((((cmd).wrapping_add(8).cast::<i16>()).read()) as u8),
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u16),
        );
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_FlashAnimTagWithColor_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FlashAnimTagWithColor_Step1(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = 0u32;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            > 0i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
            return;
        }
        if (crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0
        {
            return;
        }
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            == 0i32
        {
            ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_FlashAnimTagWithColor_Step2));
            return;
        }
        selectedPalettes = ((crate::c::shl_i32(
            1i32,
            ((((IndexOfSpritePaletteTag(
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(7))
                .read()) as u16),
            )) as i32)
                .wrapping_add(16i32)) as u32),
        )) as u32);
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i32)
            & 256i32)
            != 0
        {
            BeginNormalPaletteFade(
                selectedPalettes,
                0i8,
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
                .wrapping_offset(4))
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(3))
                .read()) as u16),
            );
        } else {
            BeginNormalPaletteFade(
                selectedPalettes,
                0i8,
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(6))
                .read()) as u8),
                ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(5))
                .read()) as u16),
            );
        }
        let __p2 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1);
        (__p2).write((((((__p2).read()) as i32) ^ 256i32) as i16));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .write(
            ((((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .read()) as i32)
                & 255i32) as i16),
        );
        let __p3 = (((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2);
        (__p3).write(((__p3).read()).wrapping_sub(1));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_FlashAnimTagWithColor_Step2(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut selectedPalettes: u32 = 0u32;
        if !((crate::c::bf_read(
            ((&raw mut gPaletteFade).cast::<u8>()).wrapping_add(7),
            7,
            1,
            false,
        ) as u16)
            != 0)
        {
            selectedPalettes = ((crate::c::shl_i32(
                1i32,
                ((((IndexOfSpritePaletteTag(
                    ((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read()) as u16),
                )) as i32)
                    .wrapping_add(16i32)) as u32),
            )) as u32);
            BeginNormalPaletteFade(selectedPalettes, 0i8, 0u8, 0u8, 0u16);
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_InvertScreenColor(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut selectedPalettes: u32 = 0u32;
        let mut attackerBattler: u8 = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        let mut targetBattler: u8 = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        if (((((cmd).cast::<i16>()).read()) as i32) & 256i32) != 0 {
            selectedPalettes = GetBattlePalettesMask(1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8);
        }
        if (((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) & 256i32) != 0 {
            selectedPalettes = (selectedPalettes
                | ((crate::c::shl_i32(65536i32, ((attackerBattler) as u32))) as u32));
        }
        if (((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) & 256i32) != 0 {
            selectedPalettes = (selectedPalettes
                | ((crate::c::shl_i32(65536i32, ((targetBattler) as u32))) as u32));
        }
        InvertPlttBuffer(selectedPalettes);
        DestroyAnimVisualTask(taskId);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_TintPalettes(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        let mut attackerBattler: u8 = 0u8;
        let mut targetBattler: u8 = 0u8;
        let mut paletteIndex: u8 = 0u8;
        let mut selectedPalettes: u32 = 0u32;
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
            .wrapping_offset(2))
            .write(((cmd).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write(((cmd).wrapping_add(10).cast::<i16>()).read());
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .write(((cmd).wrapping_add(12).cast::<i16>()).read());
        }
        let __p1 = ((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        attackerBattler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        targetBattler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            & 256i32)
            != 0
        {
            selectedPalettes = 65535u32;
        }
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as i32)
            & 1i32)
            != 0
        {
            paletteIndex = IndexOfSpritePaletteTag(
                ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                        .wrapping_offset(((attackerBattler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(20)
                .cast::<*mut u8>())
                .read())
                .wrapping_add(2)
                .cast::<u16>())
                .read(),
            );
            selectedPalettes = (selectedPalettes
                | ((crate::c::shl_i32(1i32, ((paletteIndex) as u32)) << 16) as u32));
        }
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            & 256i32)
            != 0
        {
            selectedPalettes = (selectedPalettes
                | ((crate::c::shl_i32(1i32, ((attackerBattler) as u32)) << 16) as u32));
        }
        if (((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as i32)
            & 256i32)
            != 0
        {
            selectedPalettes = (selectedPalettes
                | ((crate::c::shl_i32(1i32, ((targetBattler) as u32)) << 16) as u32));
        }
        TintPlttBuffer(
            selectedPalettes,
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .read()) as i8),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as i8),
            ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(7))
            .read()) as i8),
        );
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
            UnfadePlttBuffer(selectedPalettes);
            DestroyAnimVisualTask(taskId);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimShakeMonOrBattlePlatforms(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((((((cmd).cast::<i16>()).read()) as i32).wrapping_neg()) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        'l1: {
            let __sw1 = ((((cmd).wrapping_add(6).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                StoreSpriteCallbackInData6(
                    sprite,
                    core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut u8)>>(
                        ((&raw mut gBattle_BG3_X).cast::<u16>()).cast::<u8>(),
                    ),
                );
                break 'l1;
            }
            if __sw1 == 1i32 {
                StoreSpriteCallbackInData6(
                    sprite,
                    core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut u8)>>(
                        ((&raw mut gBattle_BG3_Y).cast::<u16>()).cast::<u8>(),
                    ),
                );
                break 'l1;
            }
            if __sw1 == 2i32 {
                StoreSpriteCallbackInData6(
                    sprite,
                    core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut u8)>>(
                        ((&raw mut gSpriteCoordOffsetX).cast::<i16>()).cast::<u8>(),
                    ),
                );
                break 'l1;
            }
            if !__matched {
                StoreSpriteCallbackInData6(
                    sprite,
                    core::mem::transmute::<_, Option<unsafe extern "C" fn(*mut u8)>>(
                        ((&raw mut gSpriteCoordOffsetY).cast::<i16>()).cast::<u8>(),
                    ),
                );
                break 'l1;
            }
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                | (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    << 16)) as usize as *mut u16)
                .read()) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5))
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 2i32)
            || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 3i32)
        {
            AnimShakeMonOrBattlePlatforms_UpdateCoordOffsetEnabled();
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimShakeMonOrBattlePlatforms_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimShakeMonOrBattlePlatforms_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut i: u8 = 0u8;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32) > 0i32
        {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_sub(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                > 0i32
            {
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                (__p2).write(((__p2).read()).wrapping_sub(1));
            } else {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read());
                let __p3 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                    .read()) as i32)
                    | (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        << 16)) as usize as *mut u16);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32),
                    )) as u16),
                );
                (((sprite).wrapping_add(46)).cast::<i16>()).write(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_neg())
                        as i16),
                );
            }
        } else {
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                | (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                    as i32)
                    << 16)) as usize as *mut u16)
                .write(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as u16),
                );
            if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 2i32)
                || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == 3i32)
            {
                {
                    i = 0u8;
                    'l1: loop {
                        if !(((i) as i32)
                            < ((((&raw mut gBattlersCount).cast::<u8>()).read()) as i32))
                        {
                            break 'l1;
                        }
                        'l2: {
                            crate::c::bf_write(
                                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                        .wrapping_offset(((i) as i32) as isize))
                                    .read()) as i32) as isize
                                        * 68,
                                ))
                                .wrapping_add(62),
                                1,
                                1,
                                (0u16) as i32,
                            );
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimShakeMonOrBattlePlatforms_UpdateCoordOffsetEnabled() {
    unsafe {
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            1,
            1,
            (0u16) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(62),
            1,
            1,
            (0u16) as i32,
        );
        if ((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32) == 2i32 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                1,
                1,
                (1u16) as i32,
            );
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                1,
                1,
                (1u16) as i32,
            );
        } else {
            if ((((cmd).wrapping_add(8).cast::<i16>()).read()) as i32) == 0i32 {
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
                    1,
                    1,
                    (1u16) as i32,
                );
            } else {
                crate::c::bf_write(
                    (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                            ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32) as isize,
                        ))
                        .read()) as i32) as isize
                            * 68,
                    ))
                    .wrapping_add(62),
                    1,
                    1,
                    (1u16) as i32,
                );
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ShakeBattlePlatforms(taskId: u8) {
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
        .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(8))
        .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        ((&raw mut gBattle_BG3_X).cast::<u16>()).write(((((cmd).cast::<i16>()).read()) as u16));
        ((&raw mut gBattle_BG3_Y).cast::<u16>())
            .write(((((cmd).wrapping_add(2).cast::<i16>()).read()) as u16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(AnimTask_ShakeBattlePlatforms_Step));
        (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .read())
        .unwrap_unchecked()(taskId);
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ShakeBattlePlatforms_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as i32)
            == 0i32
        {
            if ((((&raw mut gBattle_BG3_X).cast::<u16>()).read()) as i32)
                == (((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .read()) as i32)
            {
                ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                    (((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as i32)
                        .wrapping_neg()) as u16),
                );
            } else {
                ((&raw mut gBattle_BG3_X).cast::<u16>()).write(
                    (((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .read()) as u16),
                );
            }
            if ((((&raw mut gBattle_BG3_Y).cast::<u16>()).read()) as i32)
                == ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_neg()
            {
                ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
            } else {
                ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(
                    ((((((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_neg()) as u16),
                );
            }
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(8))
                .read(),
            );
            if (({
                let __p1 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(2);
                let __t2 = ((__p1).read()).wrapping_sub(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                == 0i32
            {
                ((&raw mut gBattle_BG3_X).cast::<u16>()).write(0u16);
                ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
                DestroyAnimVisualTask(taskId);
            }
        } else {
            let __p3 = (((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3);
            (__p3).write(((__p3).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimHitSplatBasic(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
        );
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 1u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 1u8);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimHitSplatPersistent(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
        );
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 1u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 1u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(8).cast::<i16>()).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSpriteAfterTimer));
    }
}
pub(crate) unsafe extern "C" fn AnimHitSplatHandleInvert(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32)
            && (!((IsContest()) != 0))
        {
            ((cmd).wrapping_add(2).cast::<i16>()).write(
                ((((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
            );
        }
        AnimHitSplatBasic(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimHitSplatRandom(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).wrapping_add(2).cast::<i16>()).read()) as i32) == (-1i32) {
            ((cmd).wrapping_add(2).cast::<i16>()).write(((((Random2()) as i32) & 3i32) as i16));
        }
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(2).cast::<i16>()).read()) as u8),
        );
        if ((((cmd).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 0u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 0u8);
        }
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((crate::c::rem_i32(((Random2()) as i32), 48i32)).wrapping_sub(24i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32)
                .wrapping_add((crate::c::rem_i32(((Random2()) as i32), 24i32)).wrapping_sub(12i32)))
                as i16),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimHitSplatOnMonEdge(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((GetAnimBattlerSpriteId(((((cmd).cast::<i16>()).read()) as u8))) as i16));
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(32)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(36)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
            ))
            .wrapping_add(34)
            .cast::<i16>())
            .read()) as i32)
                .wrapping_add(
                    ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>())
                    .read()) as i32),
                )) as i16),
        );
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((cmd).wrapping_add(2).cast::<i16>()).read());
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((cmd).wrapping_add(4).cast::<i16>()).read());
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
        );
        StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAffineAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimCrossImpact(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 1u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 1u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>())
            .write(((cmd).wrapping_add(6).cast::<i16>()).read());
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
    }
}
pub(crate) unsafe extern "C" fn AnimFlashingHitSplat(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut cmd: *mut u8 =
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).cast::<u8>();
        StartSpriteAffineAnim(
            sprite,
            ((((cmd).wrapping_add(6).cast::<i16>()).read()) as u8),
        );
        if ((((cmd).wrapping_add(4).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimAttacker(sprite, 1u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 1u8);
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFlashingHitSplat_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFlashingHitSplat_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write(
            (sprite).wrapping_add(62),
            2,
            1,
            ((((crate::c::bf_read((sprite).wrapping_add(62), 2, 1, false) as u16) as i32) ^ 1i32)
                as u16) as i32,
        );
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 12i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
