//! Translated from `src/battle_anim_fight.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sUnusedHumanoidFootSpriteTemplate sAnim_Fist sAnim_FootWide sAnim_FootTall sAnim_HandLeft sAnim_HandRight sAnims_HandsAndFeet gKarateChopSpriteTemplate gJumpKickSpriteTemplate gFistFootSpriteTemplate gFistFootRandomPosSpriteTemplate gCrossChopHandSpriteTemplate gSlidingKickSpriteTemplate sAffineAnim_SpinningHandOrFoot sAffineAnims_SpinningHandOrFoot gSpinningHandOrFootSpriteTemplate sAffineAnim_MegaPunchKick sAffineAnims_MegaPunchKick gMegaPunchKickSpriteTemplate gStompFootSpriteTemplate gDizzyPunchDuckSpriteTemplate gBrickBreakWallSpriteTemplate gBrickBreakWallShardSpriteTemplate sAffineAnim_SuperpowerOrb sAffineAnims_SuperpowerOrb gSuperpowerOrbSpriteTemplate gSuperpowerRockSpriteTemplate gSuperpowerFireballSpriteTemplate gArmThrustHandSpriteTemplate sAnim_RevengeSmallScratch_0 sAnim_RevengeSmallScratch_1 sAnim_RevengeSmallScratch_2 sAnims_RevengeSmallScratch gRevengeSmallScratchSpriteTemplate sAnim_RevengeBigScratch_0 sAnim_RevengeBigScratch_1 sAnim_RevengeBigScratch_2 sAnims_RevengeBigScratch gRevengeBigScratchSpriteTemplate sAffineAnim_FocusPunchFist sAffineAnims_FocusPunchFist gFocusPunchFistSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_fight::*;

unsafe extern "C" {
    static mut gAnimMoveTurn: u8;
    static mut gBasicHitSplatSpriteTemplate: u8;
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG3_X: u8;
    static mut gBattle_BG3_Y: u8;
    static mut gBattlerAttacker: u8;
    static mut gBattlerPositions: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn AnimTranslateLinear_WithFollowup(a0: *mut u8);
    fn AnimTravelDiagonally(a0: *mut u8);
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn FreeOamMatrix(a0: u8);
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriority(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn InitAnimLinearTranslation(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsContest() -> u8;
    fn LoadPointerFromVars(a0: i16, a1: i16) -> *mut u8;
    fn Random2() -> u16;
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn SetAnimSpriteInitialXOffset(a0: *mut u8, a1: i16);
    fn SetGpuReg(a0: u8, a1: u16);
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartAnimLinearTranslation(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StorePointerInVars(a0: *mut i16, a1: *mut i16, a2: *mut u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn UpdateAnimBg3ScreenSize(a0: u8);
    fn WaitAnimForDuration(a0: *mut u8);
}

pub(crate) unsafe extern "C" fn AnimUnusedHumanoidFoot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetAnimSpriteInitialXOffset(
            sprite,
            (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read(),
        );
        let __p1 = (sprite).wrapping_add(34).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(15i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimSlideHandOrFootToTarget(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7))
            .read()) as i32)
            == 1i32)
            && (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32)
        {
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
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6))
                .read()) as u8),
        );
        ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(6)).write(0i16);
        AnimTravelDiagonally(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimJumpKick(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
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
        AnimSlideHandOrFootToTarget(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimBasicFistOrFoot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4))
                .read()) as u8),
        );
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read())
            as i32)
            == 0i32
        {
            InitSpritePosToAnimAttacker(sprite, 1u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 1u8);
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimFistOrFootRandomPos(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        let mut xMod: i16 = 0i16;
        let mut yMod: i16 = 0i16;
        let mut x: i16 = 0i16;
        let mut y: i16 = 0i16;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
        } else {
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
        }
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            < 0i32
        {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .write(((crate::c::rem_i32(((Random2()) as i32), 5i32)) as i16));
        }
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as u8),
        );
        ((sprite).wrapping_add(32).cast::<i16>())
            .write(((GetBattlerSpriteCoord(battler, 2u8)) as i16));
        ((sprite).wrapping_add(34).cast::<i16>())
            .write(((GetBattlerSpriteCoord(battler, 3u8)) as i16));
        xMod =
            ((crate::c::div_i32(((GetBattlerSpriteCoordAttr(battler, 1u8)) as i32), 2i32)) as i16);
        yMod =
            ((crate::c::div_i32(((GetBattlerSpriteCoordAttr(battler, 0u8)) as i32), 4i32)) as i16);
        x = ((crate::c::rem_i32(((Random2()) as i32), ((xMod) as i32))) as i16);
        y = ((crate::c::rem_i32(((Random2()) as i32), ((yMod) as i32))) as i16);
        if (((Random2()) as i32) & 1i32) != 0 {
            x = ((((x) as i32).wrapping_mul((-1i32))) as i16);
        }
        if (((Random2()) as i32) & 1i32) != 0 {
            y = ((((y) as i32).wrapping_mul((-1i32))) as i16);
        }
        if ((((((&raw mut gBattlerPositions).cast::<u8>())
            .wrapping_offset(((battler) as i32) as isize))
        .read()) as i32)
            & 1i32)
            == 0i32
        {
            y = ((((y) as i32).wrapping_sub(16i32)) as i16);
        }
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write((((((__p1).read()) as i32).wrapping_add(((x) as i32))) as i16));
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write((((((__p2).read()) as i32).wrapping_add(((y) as i32))) as i16));
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(
            ((CreateSprite(
                (&raw mut gBasicHitSplatSpriteTemplate).cast::<u8>(),
                ((sprite).wrapping_add(32).cast::<i16>()).read(),
                ((sprite).wrapping_add(34).cast::<i16>()).read(),
                ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_add(1i32)) as u8),
            )) as i16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            != 64i32
        {
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ),
                0u8,
            );
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                    as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimFistOrFootRandomPos_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimFistOrFootRandomPos_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                != 64i32
            {
                FreeOamMatrix(
                    ((crate::c::bf_read(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                                .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(3),
                        1,
                        5,
                        false,
                    ) as u32) as u8),
                );
                DestroySprite(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32) as isize
                        * 68,
                ));
            }
            DestroyAnimSprite(sprite);
        } else {
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimCrossChopHand(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(30i16);
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_sub(20i32))
                    as i16),
            );
        } else {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(20i32))
                    as i16),
            );
            crate::c::bf_write((sprite).wrapping_add(63), 0, 1, (1u16) as i32);
        }
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(20i32))
                as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(StartAnimLinearTranslation));
        StoreSpriteCallbackInData6(sprite, Some(AnimCrossChopHand_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimCrossChopHand_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 11i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)
                    .wrapping_sub(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)
                    .wrapping_sub(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(8i16);
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write(
                (((((__p3).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write(
                (((((__p4).read()) as i32)
                    .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                    as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(StartAnimLinearTranslation));
            StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSlidingKick(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i32) ^ 2i32)
            == ((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i32))
            && (((GetBattlerPosition(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32)
                < 2i32)
        {
            let __p1 = ((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_mul((-1i32))) as i16));
        }
        InitSpritePosToAnimTarget(sprite, 1u8);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
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
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        InitAnimLinearTranslation(sprite);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(5)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSlidingKick_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSlidingKick_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((AnimTranslateLinear(sprite)) != 0) {
            let __p1 = (sprite).wrapping_add(38).cast::<i16>();
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(
                    ((Sin(
                        ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                            as i32)
                            >> 8) as i16),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    )) as i32),
                )) as i16),
            );
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                )) as i16),
            );
        } else {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSpinningKickOrPunch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
        StartSpriteAnim(
            sprite,
            ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                .read()) as u8),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(AnimSpinningKickOrPunchFinish));
    }
}
pub(crate) unsafe extern "C" fn AnimSpinningKickOrPunchFinish(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        StartSpriteAffineAnim(sprite, 0u8);
        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(20i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimStompFoot(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        InitSpritePosToAnimTarget(sprite, 1u8);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimStompFoot_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimStompFoot_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == (-1i32)
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(6i16);
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
            StoreSpriteCallbackInData6(sprite, Some(AnimStompFoot_End));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimStompFoot_End(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(15i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(WaitAnimForDuration));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimDizzyPunchDuck(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            InitSpritePosToAnimTarget(sprite, 1u8);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read(),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read(),
            );
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32),
                )) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    >> 8) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read(),
            ));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    .wrapping_add(3i32)
                    & 255i32) as i16),
            );
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 100i32
            {
                crate::c::bf_write(
                    (sprite).wrapping_add(62),
                    2,
                    1,
                    ((crate::c::rem_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32),
                        2i32,
                    )) as u16) as i32,
                );
            }
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                > 120i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBrickBreakWall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
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
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2))
                    .read()) as i32),
            )) as i16),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(4)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBrickBreakWall_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBrickBreakWall_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (({
                    let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_sub(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    == 0i32
                {
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        == 0i32
                    {
                        DestroyAnimSprite(sprite);
                    } else {
                        let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p4).write(((__p4).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t6 = ((__p5).read()).wrapping_add(1);
                    (__p5).write(__t6);
                    __t6
                }) as i32)
                    > 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p7).write(((__p7).read()).wrapping_add(1));
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        & 1i32)
                        != 0
                    {
                        ((sprite).wrapping_add(36).cast::<i16>()).write(2i16);
                    } else {
                        ((sprite).wrapping_add(36).cast::<i16>()).write((-2i16));
                    }
                }
                if (({
                    let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t9 = ((__p8).read()).wrapping_sub(1);
                    (__p8).write(__t9);
                    __t9
                }) as i32)
                    == 0i32
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBrickBreakWallShard(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32),
                    )) as i16),
            );
        } else {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 0u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(2))
                        .read()) as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 1u8))
                    as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32),
                    )) as i16),
            );
        }
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(1))
                    .read()) as i32)
                        .wrapping_mul(16i32),
                )) as u16) as i32,
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        'l1: {
            let __sw1 = ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                .wrapping_offset(1))
            .read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32 || __sw1 == 3i32;
            if __sw1 == 0i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write((-3i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write((-3i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(3i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write((-3i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write((-3i16));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
                break 'l1;
            }
            if __sw1 == 3i32 {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(3i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(3i16);
                break 'l1;
            }
            if !__matched {
                DestroyAnimSprite(sprite);
                return;
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimBrickBreakWallShard_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimBrickBreakWallShard_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(32).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32),
            )) as i16),
        );
        let __p2 = (sprite).wrapping_add(34).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            )) as i16),
        );
        if (({
            let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t4 = ((__p3).read()).wrapping_add(1);
            (__p3).write(__t4);
            __t4
        }) as i32)
            > 40i32
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSuperpowerOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattlerAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattlerAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((&raw mut gBattleAnimTarget).cast::<u8>()).read()) as i16));
        } else {
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((((&raw mut gBattleAnimAttacker).cast::<u8>()).read()) as i16));
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(12i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(8i16);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSuperpowerOrb_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimSuperpowerOrb_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 180i32
        {
            SetGpuReg(80u8, 0u16);
            (((sprite).wrapping_add(46)).cast::<i16>()).write(16i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((sprite).wrapping_add(32).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((GetBattlerSpriteCoord(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u8),
                    2u8,
                )) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                .write(((sprite).wrapping_add(34).cast::<i16>()).read());
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
                ((GetBattlerSpriteCoord(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u8),
                    3u8,
                )) as i16),
            );
            InitAnimLinearTranslation(sprite);
            StoreSpriteCallbackInData6(sprite, Some(DestroySpriteAndMatrix));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimTranslateLinear_WithFollowup));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSuperpowerRock(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite).wrapping_add(32).cast::<i16>())
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((sprite).wrapping_add(34).cast::<i16>()).write(120i16);
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        StorePointerInVars(
            (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4),
            (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5),
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 8) as usize
                as *mut u8),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        crate::c::bf_write(
            (sprite).wrapping_add(4),
            0,
            10,
            ((((crate::c::bf_read((sprite).wrapping_add(4), 0, 10, false) as u16) as i32)
                .wrapping_add(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as i32)
                        .wrapping_mul(4i32),
                )) as u16) as i32,
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimSuperpowerRock_Step1));
    }
}
pub(crate) unsafe extern "C" fn AnimSuperpowerRock_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut var0: *mut u8 = core::ptr::null_mut();
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) != 0i32 {
            var0 = LoadPointerFromVars(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
            );
            var0 = (var0).wrapping_offset(
                (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    .wrapping_neg()) as isize
                    * 1,
            );
            StorePointerInVars(
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4),
                (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5),
                var0,
            );
            var0 = ((((var0) as usize as i32) >> 8) as usize as *mut u8);
            ((sprite).wrapping_add(34).cast::<i16>()).write((((var0) as usize as i32) as i16));
            if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-8i32) {
                DestroyAnimSprite(sprite);
            } else {
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p1).write(((__p1).read()).wrapping_sub(1));
            }
        } else {
            let mut pos0: i16 =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16);
            let mut pos1: i16 =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16);
            let mut pos2: i16 =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                    as i16);
            let mut pos3: i16 =
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                    as i16);
            (((sprite).wrapping_add(46)).cast::<i16>())
                .write(((((pos2) as i32).wrapping_sub(((pos0) as i32))) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                .write(((((pos3) as i32).wrapping_sub(((pos1) as i32))) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
                .write(((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32) << 4) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                .write(((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) << 4) as i16));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimSuperpowerRock_Step2));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSuperpowerRock_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut edgeX: u16 = 0u16;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_add((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_add(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
                >> 4) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                >> 4) as i16),
        );
        edgeX = ((((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32).wrapping_add(8i32))
            as u16);
        if ((((edgeX) as i32) > 256i32)
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < (-8i32)))
            || (((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) > 120i32)
        {
            DestroyAnimSprite(sprite);
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSuperpowerFireball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = 0u8;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattlerAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattlerAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
            battler = ((&raw mut gBattleAnimTarget).cast::<u8>()).read();
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimAttacker).cast::<u8>()).read()))
                    as u16) as i32,
            );
        } else {
            battler = ((&raw mut gBattleAnimAttacker).cast::<u8>()).read();
            crate::c::bf_write(
                (sprite).wrapping_add(5),
                2,
                2,
                ((GetBattlerSpriteBGPriority(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
                    as u16) as i32,
            );
        }
        if (IsContest()) != 0 {
            crate::c::bf_write(
                (sprite).wrapping_add(3),
                1,
                5,
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) | 8u32) as i32,
            );
        } else {
            if ((GetBattlerSide(battler)) as i32) == 0i32 {
                crate::c::bf_write(
                    (sprite).wrapping_add(3),
                    1,
                    5,
                    ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) | 24u32)
                        as i32,
                );
            }
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(16i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
            .write(((sprite).wrapping_add(32).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write(((GetBattlerSpriteCoord(battler, 2u8)) as i16));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
            .write(((sprite).wrapping_add(34).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4))
            .write(((GetBattlerSpriteCoord(battler, 3u8)) as i16));
        InitAnimLinearTranslation(sprite);
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimTranslateLinear_WithFollowup));
    }
}
pub(crate) unsafe extern "C" fn AnimArmThrustHit_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
            == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AnimArmThrustHit(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut turn: u8 = 0u8;
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimTarget).cast::<u8>()).read(), 3u8))
                as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2))
            .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read(),
        );
        turn = ((&raw mut gAnimMoveTurn).cast::<u8>()).read();
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            turn = (turn).wrapping_add(1);
        }
        if (((turn) as i32) & 1i32) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    .wrapping_neg()) as i16),
            );
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
        StartSpriteAnim(
            sprite,
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8),
        );
        ((sprite).wrapping_add(36).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read());
        ((sprite).wrapping_add(38).cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read());
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimArmThrustHit_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimRevengeScratch(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(2)).read())
            as i32)
            == 0i32
        {
            InitSpritePosToAnimAttacker(sprite, 0u8);
        } else {
            InitSpritePosToAnimTarget(sprite, 0u8);
        }
        if (IsContest()) != 0 {
            StartSpriteAnim(sprite, 2u8);
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                StartSpriteAnim(sprite, 1u8);
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimFocusPunchFist(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    .wrapping_add(40i32)
                    & 255i32) as i16),
            );
            ((sprite).wrapping_add(36).cast::<i16>()).write(Sin(
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read(),
                2i16,
            ));
            if (({
                let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
                let __t2 = ((__p1).read()).wrapping_add(1);
                (__p1).write(__t2);
                __t2
            }) as i32)
                > 40i32
            {
                DestroyAnimSprite(sprite);
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MoveSkyUppercutBg(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            let __matched = __sw1 == 0i32 || __sw1 == 1i32 || __sw1 == 2i32;
            if __sw1 == 0i32 {
                UpdateAnimBg3ScreenSize(0u8);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8))
                    .write((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read());
                let __p2 = ((task).wrapping_add(8)).cast::<i16>();
                (__p2).write(((__p2).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(8);
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == (-1i32)
                {
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 || !__matched {
                let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
                (__p6).write((((((__p6).read()) as i32).wrapping_add(1280i32)) as i16));
                break 'l1;
            }
        }
        let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
        (__p7).write((((((__p7).read()) as i32).wrapping_add(2816i32)) as i16));
        if ((GetBattlerSide(((&raw mut gBattleAnimTarget).cast::<u8>()).read())) as i32) == 0i32 {
            let __p8 = (&raw mut gBattle_BG3_X).cast::<u16>();
            (__p8).write(
                (((((__p8).read()) as i32).wrapping_add(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                        as i32)
                        >> 8),
                )) as u16),
            );
        } else {
            let __p9 = (&raw mut gBattle_BG3_X).cast::<u16>();
            (__p9).write(
                (((((__p9).read()) as i32).wrapping_sub(
                    (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9)).read())
                        as i32)
                        >> 8),
                )) as u16),
            );
        }
        let __p10 = (&raw mut gBattle_BG3_Y).cast::<u16>();
        (__p10).write(
            (((((__p10).read()) as i32).wrapping_add(
                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read()) as i32)
                    >> 8),
            )) as u16),
        );
        let __p11 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(9);
        (__p11).write((((((__p11).read()) as i32) & 255i32) as i16));
        let __p12 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10);
        (__p12).write((((((__p12).read()) as i32) & 255i32) as i16));
        if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(7)).read())
            as i32)
            == (-1i32)
        {
            ((&raw mut gBattle_BG3_X).cast::<u16>()).write(0u16);
            ((&raw mut gBattle_BG3_Y).cast::<u16>()).write(0u16);
            UpdateAnimBg3ScreenSize(1u8);
            DestroyAnimVisualTask(taskId);
        }
    }
}
