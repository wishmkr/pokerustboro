//! Translated from `src/battle_anim_psychic.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): sAffineAnim_PsychUpSpiral sAffineAnims_PsychUpSpiral gPsychUpSpiralSpriteTemplate gLightScreenWallSpriteTemplate gReflectWallSpriteTemplate gMirrorCoatWallSpriteTemplate gBarrierWallSpriteTemplate gMagicCoatWallSpriteTemplate sAnim_ReflectSparkle sAnims_ReflectSparkle gReflectSparkleSpriteTemplate sAnim_SpecialScreenSparkle sAnims_SpecialScreenSparkle gSpecialScreenSparkleSpriteTemplate gGoldRingSpriteTemplate sAnim_BentSpoon_0 sAnim_BentSpoon_1 sAnims_BentSpoon gBentSpoonSpriteTemplate sAnim_QuestionMark sAnims_QuestionMark sAffineAnim_QuestionMark sAffineAnims_QuestionMark gQuestionMarkSpriteTemplate sAffineAnim_MeditateStretchAttacker sAffineAnim_Teleport gImprisonOrbSpriteTemplate gRedXSpriteTemplate sAffineAnim_SkillSwapOrb_0 sAffineAnim_SkillSwapOrb_1 sAffineAnim_SkillSwapOrb_2 sAffineAnim_SkillSwapOrb_3 sAffineAnims_SkillSwapOrb gSkillSwapOrbSpriteTemplate sAffineAnim_LusterPurgeCircle sAffineAnims_LusterPurgeCircle gLusterPurgeCircleSpriteTemplate sAffineAnim_PsychoBoostOrb_0 sAffineAnim_PsychoBoostOrb_1 sAffineAnims_PsychoBoostOrb gPsychoBoostOrbSpriteTemplate
#[allow(unused_imports)]
use crate::data::battle_anim_psychic::*;

unsafe extern "C" {
    static mut gBattleAnimArgs: u8;
    static mut gBattleAnimAttacker: u8;
    static mut gBattleAnimTarget: u8;
    static mut gBattle_BG1_X: u8;
    static mut gBattle_BG2_X: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gPlttBufferFaded: u8;
    static mut gScanlineEffect: u8;
    static mut gScanlineEffectRegBuffers: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AllocOamMatrix() -> u8;
    fn BattleAnimAdjustPanning(a0: i8) -> i8;
    fn CalcCenterToCornerVec(a0: *mut u8, a1: u8, a2: u8, a3: u8);
    fn ChangeSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn CloneBattlerSpriteWithBlend(a0: u8) -> i16;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn DestroyAnimSprite(a0: *mut u8);
    fn DestroyAnimVisualTask(a0: u8);
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndMatrix(a0: *mut u8);
    fn DestroySpriteWithActiveSheet(a0: *mut u8);
    fn FreeOamMatrix(a0: u8);
    fn GetAnimBattlerSpriteId(a0: u8) -> u8;
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteBGPriorityRank(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetBattlerSpriteCoordAttr(a0: u8, a1: u8) -> i16;
    fn GetBattlerYCoordWithElevation(a0: u8) -> u8;
    fn IndexOfSpritePaletteTag(a0: u16) -> u8;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn InitSpriteAffineAnim(a0: *mut u8);
    fn InitSpritePosToAnimAttacker(a0: *mut u8, a1: u8);
    fn InitSpritePosToAnimTarget(a0: *mut u8, a1: u8);
    fn IsBattlerSpriteVisible(a0: u8) -> u8;
    fn IsContest() -> u8;
    fn IsDoubleBattle() -> u8;
    fn MoveBattlerSpriteToBG(a0: u8, a1: u8, a2: u8);
    fn PlaySE12WithPanning(a0: u16, a1: i8);
    fn PrepareAffineAnimInTaskData(a0: *mut u8, a1: u8, a2: *mut u8);
    fn ResetBattleAnimBg(a0: u8);
    fn ResetSpriteRotScale(a0: u8);
    fn RunAffineAnimFromTaskData(a0: *mut u8) -> u8;
    fn RunStoredCallbackWhenAnimEnds(a0: *mut u8);
    fn ScanlineEffect_SetParams(a0: crate::c::Rec4<12>);
    fn SetBattlerSpriteYOffsetFromOtherYScale(a0: u8, a1: u8);
    fn SetGpuReg(a0: u8, a1: u16);
    fn SetSpriteRotScale(a0: u8, a1: i16, a2: i16, a3: u16);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StoreSpriteCallbackInData6(a0: *mut u8, a1: Option<unsafe extern "C" fn(*mut u8)>);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
}

pub(crate) unsafe extern "C" fn AnimDefensiveWall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut isContest: u8 = IsContest();
        if (((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 0i32)
            || ((isContest) != 0)
        {
            crate::c::bf_write((sprite).wrapping_add(5), 2, 2, (2u16) as i32);
            ((sprite).wrapping_add(67)).write(200u8);
        }
        if !((isContest) != 0) {
            let mut battlerCopy: u8 = 0u8;
            let mut battler: u8 = {
                let __v1 = GetBattlerAtPosition(1u8);
                battlerCopy = __v1;
                __v1
            };
            let mut rank: u8 = GetBattlerSpriteBGPriorityRank(battler);
            let mut var0: i32 = 1i32;
            let mut toBG_2: u8 = (((((rank) as i32) ^ var0) != 0i32) as u8);
            if (IsBattlerSpriteVisible(battler)) != 0 {
                MoveBattlerSpriteToBG(battler, toBG_2, 0u8);
            }
            battler = ((((battlerCopy) as i32) ^ 2i32) as u8);
            if (IsBattlerSpriteVisible(battler)) != 0 {
                MoveBattlerSpriteToBG(battler, ((((toBG_2) as i32) ^ var0) as u8), 0u8);
            }
        }
        if (!((isContest) != 0)) && ((IsDoubleBattle()) != 0) {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                == 0i32
            {
                ((sprite).wrapping_add(32).cast::<i16>()).write(72i16);
                ((sprite).wrapping_add(34).cast::<i16>()).write(80i16);
            } else {
                ((sprite).wrapping_add(32).cast::<i16>()).write(176i16);
                ((sprite).wrapping_add(34).cast::<i16>()).write(40i16);
            }
        } else {
            if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                (((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).write(
                    (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32)
                        .wrapping_neg()) as i16),
                );
            }
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    0u8,
                )) as i32)
                    .wrapping_add(
                        (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32),
                    )) as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((((GetBattlerSpriteCoord(
                    ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                    1u8,
                )) as i32)
                    .wrapping_add(
                        ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32),
                    )) as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((256i32).wrapping_add(
                ((IndexOfSpritePaletteTag(
                    ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                        .wrapping_offset(2))
                    .read()) as u16),
                )) as i32)
                    .wrapping_mul(16i32),
            )) as i16),
        );
        if (isContest) != 0 {
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(9i32)) as i16));
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimDefensiveWall_Step2));
            (((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .read())
            .unwrap_unchecked()(sprite);
        } else {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimDefensiveWall_Step1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 = GetBattlerAtPosition(1u8);
        if !((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) != 0) {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(1i16);
            return;
        }
        if (IsBattlerSpriteVisible(battler)) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        battler = ((((battler) as i32) ^ 2i32) as u8);
        if (IsBattlerSpriteVisible(battler)) != 0 {
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimDefensiveWall_Step2));
        (((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(sprite);
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(
            82u8,
            ((((16i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            ) << 8)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)) as u16),
        );
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 13i32
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimDefensiveWall_Step3));
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            (__p1).write(((__p1).read()).wrapping_add(1));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step3(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut color: u16 = 0u16;
        let mut startOffset: u16 = 0u16;
        let mut i: i32 = 0i32;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 2i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            startOffset = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u16);
            color = ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((startOffset) as i32).wrapping_add(8i32)) as isize))
            .read();
            {
                i = 8i32;
                'l1: loop {
                    if !(i > 0i32) {
                        break 'l1;
                    }
                    'l2: {
                        ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                            .wrapping_offset((((startOffset) as i32).wrapping_add(i)) as isize))
                        .write(
                            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                                .wrapping_offset(
                                    ((((startOffset) as i32).wrapping_add(i)).wrapping_sub(1i32))
                                        as isize,
                                ))
                            .read(),
                        );
                    }
                    i = (i).wrapping_sub(1);
                }
            }
            ((((&raw mut gPlttBufferFaded).cast::<u16>()).cast::<u16>())
                .wrapping_offset((((startOffset) as i32).wrapping_add(1i32)) as isize))
            .write(color);
            if (({
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                let __t4 = ((__p3).read()).wrapping_add(1);
                (__p3).write(__t4);
                __t4
            }) as i32)
                == 16i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(AnimDefensiveWall_Step4));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step4(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        SetGpuReg(
            82u8,
            ((((16i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32),
            ) << 8)
                | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)) as u16),
        );
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
            let __t2 = ((__p1).read()).wrapping_sub(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == (-1i32)
        {
            if !((IsContest()) != 0) {
                let mut battlerCopy: u8 = 0u8;
                let mut battler: u8 = {
                    let __v3 = GetBattlerAtPosition(1u8);
                    battlerCopy = __v3;
                    __v3
                };
                if (IsBattlerSpriteVisible(battler)) != 0 {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
                battler = ((((battlerCopy) as i32) ^ 2i32) as u8);
                if (IsBattlerSpriteVisible(battler)) != 0 {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                                .wrapping_offset(((battler) as i32) as isize))
                            .read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (0u16) as i32,
                    );
                }
            }
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(AnimDefensiveWall_Step5));
        }
    }
}
pub(crate) unsafe extern "C" fn AnimDefensiveWall_Step5(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if !((IsContest()) != 0) {
            let mut battlerCopy: u8 = 0u8;
            let mut battler: u8 = {
                let __v1 = GetBattlerAtPosition(1u8);
                battlerCopy = __v1;
                __v1
            };
            let mut rank: u8 = GetBattlerSpriteBGPriorityRank(battler);
            let mut var0: i32 = 1i32;
            let mut toBG2: u8 = (((((rank) as i32) ^ var0) != 0i32) as u8);
            if (IsBattlerSpriteVisible(battler)) != 0 {
                ResetBattleAnimBg(toBG2);
            }
            battler = ((((battlerCopy) as i32) ^ 2i32) as u8);
            if (IsBattlerSpriteVisible(battler)) != 0 {
                ResetBattleAnimBg(((((toBG2) as i32) ^ var0) as u8));
            }
        }
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(DestroyAnimSprite));
    }
}
pub(crate) unsafe extern "C" fn AnimWallSparkle(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) == 0i32 {
            let mut ignoreOffsets: u32 =
                ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(3))
                    .read()) as u32);
            let mut respectMonPicOffsets: u8 = 0u8;
            if !((ignoreOffsets) != 0) {
                respectMonPicOffsets = 1u8;
            }
            if (!((IsContest()) != 0)) && ((IsDoubleBattle()) != 0) {
                if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                    == 0i32
                {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        (((72i32).wrapping_sub(
                            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                                as i32),
                        )) as i16),
                    );
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(80i32)) as i16),
                    );
                } else {
                    ((sprite).wrapping_add(32).cast::<i16>()).write(
                        (((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read())
                            as i32)
                            .wrapping_add(176i32)) as i16),
                    );
                    ((sprite).wrapping_add(34).cast::<i16>()).write(
                        ((((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(1))
                        .read()) as i32)
                            .wrapping_add(40i32)) as i16),
                    );
                }
            } else {
                if ((((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>())
                    .wrapping_offset(2))
                .read()) as i32)
                    == 0i32
                {
                    InitSpritePosToAnimAttacker(sprite, respectMonPicOffsets);
                } else {
                    InitSpritePosToAnimTarget(sprite, respectMonPicOffsets);
                }
            }
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
        } else {
            if ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
                || ((crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0)
            {
                DestroySpriteAndMatrix(sprite);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimBentSpoon(sprite: *mut u8) {
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
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) != 0i32 {
            StartSpriteAnim(sprite, 1u8);
            let __p1 = (sprite).wrapping_add(32).cast::<i16>();
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(40i32)) as i16));
            let __p2 = (sprite).wrapping_add(34).cast::<i16>();
            (__p2).write((((((__p2).read()) as i32).wrapping_add(10i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write((-1i16));
        } else {
            let __p3 = (sprite).wrapping_add(32).cast::<i16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(40i32)) as i16));
            let __p4 = (sprite).wrapping_add(34).cast::<i16>();
            (__p4).write((((((__p4).read()) as i32).wrapping_sub(10i32)) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(1i16);
        }
        StoreSpriteCallbackInData6(sprite, Some(DestroyAnimSprite));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimQuestionMark(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut x: i16 = ((crate::c::div_i32(
            ((GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32),
            2i32,
        )) as i16);
        let mut y: i16 = ((crate::c::div_i32(
            ((GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i32),
            (-2i32),
        )) as i16);
        if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32) == 1i32 {
            x = ((((x) as i32).wrapping_neg()) as i16);
        }
        ((sprite).wrapping_add(32).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i32)
                .wrapping_add(((x) as i32))) as i16),
        );
        ((sprite).wrapping_add(34).cast::<i16>()).write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i32)
                .wrapping_add(((y) as i32))) as i16),
        );
        if ((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32) < 16i32 {
            ((sprite).wrapping_add(34).cast::<i16>()).write(16i16);
        }
        StoreSpriteCallbackInData6(sprite, Some(AnimQuestionMark_Step1));
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(RunStoredCallbackWhenAnimEnds));
    }
}
pub(crate) unsafe extern "C" fn AnimQuestionMark_Step1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (1u32) as i32);
        ((sprite).wrapping_add(16).cast::<*mut *mut u8>()).write(
            ((&raw const sAffineAnims_QuestionMark)
                .cast::<u8>()
                .cast_mut()
                .cast::<*mut u8>())
            .cast::<*mut u8>(),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
        InitSpriteAffineAnim(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimQuestionMark_Step2));
    }
}
pub(crate) unsafe extern "C" fn AnimQuestionMark_Step2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    FreeOamMatrix(
                        ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
                    );
                    crate::c::bf_write((sprite).wrapping_add(1), 0, 2, (0u32) as i32);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(18i16);
                    let __p2 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p2).write(((__p2).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    let __t4 = ((__p3).read()).wrapping_sub(1);
                    (__p3).write(__t4);
                    __t4
                }) as i32)
                    == (-1i32)
                {
                    DestroyAnimSprite(sprite);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_MeditateStretchAttacker(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        (((task).wrapping_add(8)).cast::<i16>()).write(((spriteId) as i16));
        PrepareAffineAnimInTaskData(
            task,
            spriteId,
            ((&raw const sAffineAnim_MeditateStretchAttacker)
                .cast::<u8>()
                .cast_mut())
            .cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_MeditateStretchAttacker_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_MeditateStretchAttacker_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        if !((RunAffineAnimFromTaskData(
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40),
        )) != 0)
        {
            DestroyAnimVisualTask(taskId);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_Teleport(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        let mut spriteId: u8 = GetAnimBattlerSpriteId(0u8);
        (((task).wrapping_add(8)).cast::<i16>()).write(((spriteId) as i16));
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(
            ((if ((GetBattlerSide(((&raw mut gBattleAnimAttacker).cast::<u8>()).read())) as i32)
                != 0i32
            {
                4i32
            } else {
                8i32
            }) as i16),
        );
        PrepareAffineAnimInTaskData(
            task,
            (((((task).wrapping_add(8)).cast::<i16>()).read()) as u8),
            ((&raw const sAffineAnim_Teleport).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_Teleport_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_Teleport_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 =
                ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32);
            if __sw1 == 0i32 {
                RunAffineAnimFromTaskData(task);
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 19i32
                {
                    let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
                    != 0i32
                {
                    let __p5 = (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(38)
                    .cast::<i16>();
                    (__p5).write((((((__p5).read()) as i32).wrapping_sub(8i32)) as i16));
                    let __p6 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                } else {
                    crate::c::bf_write(
                        (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize
                                * 68,
                        ))
                        .wrapping_add(62),
                        2,
                        1,
                        (1u16) as i32,
                    );
                    ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                        (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32) as isize * 68,
                    ))
                    .wrapping_add(32)
                    .cast::<i16>())
                    .write(272i16);
                    ResetSpriteRotScale((((((task).wrapping_add(8)).cast::<i16>()).read()) as u8));
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ImprisonOrbs(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut var0: u16 = 0u16;
        let mut var1: u16 = 0u16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).write(16i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
            ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                as i16),
        );
        var0 = ((crate::c::div_i32(
            ((GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 1u8))
                as i32),
            3i32,
        )) as u16);
        var1 = ((crate::c::div_i32(
            ((GetBattlerSpriteCoordAttr(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 0u8))
                as i32),
            3i32,
        )) as u16);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
            ((if ((var0) as i32) > ((var1) as i32) {
                ((var0) as i32)
            } else {
                ((var1) as i32)
            }) as i16),
        );
        SetGpuReg(80u8, 16192u16);
        SetGpuReg(82u8, 16u16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_ImprisonOrbs_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ImprisonOrbs_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: u16 = 0u16;
        let mut spriteId: u8 = 0u8;
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
                    > 8i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    spriteId = CreateSprite(
                        (&raw const gImprisonOrbSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                        0u8,
                    );
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(
                        (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32)
                            .wrapping_add(8i32)) as isize,
                    ))
                    .write(((spriteId) as i16));
                    if ((spriteId) as i32) != 64i32 {
                        'l2: {
                            let __sw4 = ((((((task).wrapping_add(8)).cast::<i16>())
                                .wrapping_offset(2))
                            .read()) as i32);
                            if __sw4 == 0i32 {
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(36)
                                .cast::<i16>())
                                .write(
                                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                                        .read(),
                                );
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(38)
                                .cast::<i16>())
                                .write(
                                    ((((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(12))
                                    .read()) as i32)
                                        .wrapping_neg())
                                        as i16),
                                );
                                break 'l2;
                            }
                            if __sw4 == 1i32 {
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(36)
                                .cast::<i16>())
                                .write(
                                    ((((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(12))
                                    .read()) as i32)
                                        .wrapping_neg())
                                        as i16),
                                );
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(38)
                                .cast::<i16>())
                                .write(
                                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                                        .read(),
                                );
                                break 'l2;
                            }
                            if __sw4 == 2i32 {
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(36)
                                .cast::<i16>())
                                .write(
                                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                                        .read(),
                                );
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(38)
                                .cast::<i16>())
                                .write(
                                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12))
                                        .read(),
                                );
                                break 'l2;
                            }
                            if __sw4 == 3i32 {
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(36)
                                .cast::<i16>())
                                .write(
                                    ((((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(12))
                                    .read()) as i32)
                                        .wrapping_neg())
                                        as i16),
                                );
                                ((((&raw mut gSprites).cast::<u8>())
                                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                                .wrapping_add(38)
                                .cast::<i16>())
                                .write(
                                    ((((((((task).wrapping_add(8)).cast::<i16>())
                                        .wrapping_offset(12))
                                    .read()) as i32)
                                        .wrapping_neg())
                                        as i16),
                                );
                                break 'l2;
                            }
                        }
                    }
                    if (({
                        let __p5 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t6 = ((__p5).read()).wrapping_add(1);
                        (__p5).write(__t6);
                        __t6
                    }) as i32)
                        == 5i32
                    {
                        let __p7 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p7).write(((__p7).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    & 1i32)
                    != 0
                {
                    let __p8 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3);
                    (__p8).write(((__p8).read()).wrapping_sub(1));
                } else {
                    let __p9 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4);
                    (__p9).write(((__p9).read()).wrapping_add(1));
                }
                SetGpuReg(
                    82u8,
                    (((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        << 8)
                        | ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)) as u16),
                );
                if (({
                    let __p10 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t11 = ((__p10).read()).wrapping_add(1);
                    (__p10).write(__t11);
                    __t11
                }) as i32)
                    == 32i32
                {
                    {
                        i = 8u16;
                        'l3: loop {
                            if !(((i) as i32) < 13i32) {
                                break 'l3;
                            }
                            'l4: {
                                if ((((((task).wrapping_add(8)).cast::<i16>())
                                    .wrapping_offset(((i) as i32) as isize))
                                .read()) as i32)
                                    != 64i32
                                {
                                    DestroySprite(
                                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                            ((((((task).wrapping_add(8)).cast::<i16>())
                                                .wrapping_offset(((i) as i32) as isize))
                                            .read())
                                                as i32)
                                                as isize
                                                * 68,
                                        ),
                                    );
                                }
                            }
                            i = (i).wrapping_add(1);
                        }
                    }
                    let __p12 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p12).write(((__p12).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                let __p13 = ((task).wrapping_add(8)).cast::<i16>();
                (__p13).write(((__p13).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetGpuReg(82u8, 0u16);
                SetGpuReg(80u8, 0u16);
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimRedX_Step(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            > (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_sub(10i32)
        {
            crate::c::bf_write(
                (sprite).wrapping_add(62),
                2,
                1,
                ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    & 1i32) as u16) as i32,
            );
        }
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
        {
            DestroyAnimSprite(sprite);
        }
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
    }
}
pub(crate) unsafe extern "C" fn AnimRedX(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 0i32 {
            ((sprite).wrapping_add(32).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 2u8))
                    as i16),
            );
            ((sprite).wrapping_add(34).cast::<i16>()).write(
                ((GetBattlerSpriteCoord(((&raw mut gBattleAnimAttacker).cast::<u8>()).read(), 3u8))
                    as i16),
            );
        }
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).wrapping_offset(1)).read(),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(AnimRedX_Step));
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_SkillSwap(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        if (IsContest()) != 0 {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 1i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write((-10i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        5u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        5u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        2u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(10i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        4u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        3u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        4u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        3u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
            }
        } else {
            if (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32) == 1i32
            {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write((-10i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        4u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        2u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        4u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        2u8,
                    )) as i32)
                        .wrapping_add(8i32)) as i16),
                );
            } else {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).write(10i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        5u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        3u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        5u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(
                    ((((GetBattlerSpriteCoordAttr(
                        ((&raw mut gBattleAnimTarget).cast::<u8>()).read(),
                        3u8,
                    )) as i32)
                        .wrapping_sub(8i32)) as i16),
                );
            }
        }
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(6i16);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>()).write(Some(AnimTask_SkillSwap_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_SkillSwap_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: u8 = 0u8;
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
                    > 6i32
                {
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).write(0i16);
                    spriteId = CreateSprite(
                        (&raw const gSkillSwapOrbSpriteTemplate)
                            .cast::<u8>()
                            .cast_mut(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read(),
                        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read(),
                        0u8,
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
                        .write(
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read(),
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(4))
                        .write(
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read(),
                        );
                        ((((((&raw mut gSprites).cast::<u8>())
                            .wrapping_offset(((spriteId) as i32) as isize * 68))
                        .wrapping_add(46))
                        .cast::<i16>())
                        .wrapping_offset(5))
                        .write(
                            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read(),
                        );
                        InitAnimArcTranslation(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                        );
                        StartSpriteAffineAnim(
                            ((&raw mut gSprites).cast::<u8>())
                                .wrapping_offset(((spriteId) as i32) as isize * 68),
                            ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2))
                                .read()) as i32)
                                & 3i32) as u8),
                        );
                    }
                    if (({
                        let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2);
                        let __t5 = ((__p4).read()).wrapping_add(1);
                        (__p4).write(__t5);
                        __t5
                    }) as i32)
                        == 12i32
                    {
                        let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                        (__p6).write(((__p6).read()).wrapping_add(1));
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (({
                    let __p7 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t8 = ((__p7).read()).wrapping_add(1);
                    (__p7).write(__t8);
                    __t8
                }) as i32)
                    > 17i32
                {
                    DestroyAnimVisualTask(taskId);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimSkillSwapOrb(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            FreeOamMatrix(
                ((crate::c::bf_read((sprite).wrapping_add(3), 1, 5, false) as u32) as u8),
            );
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn AnimTask_ExtrasensoryDistortion(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut i: i16 = 0i16;
        let mut yOffset: u8 = 0u8;
        let mut scanlineParams = crate::ffi::Align4([0u8; 12]);
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        yOffset = GetBattlerYCoordWithElevation(((&raw mut gBattleAnimTarget).cast::<u8>()).read());
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14))
            .write(((((yOffset) as i32).wrapping_sub(32i32)) as i16));
        'l1: {
            let __sw1 =
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(2i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(5i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(64i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((((yOffset) as i32).wrapping_add(32i32)) as i16));
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(2i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(5i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(192i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((((yOffset) as i32).wrapping_add(32i32)) as i16));
                break 'l1;
            }
            if __sw1 == 2i32 {
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).write(4i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).write(4i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(0i16);
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15))
                    .write(((((yOffset) as i32).wrapping_add(32i32)) as i16));
                break 'l1;
            }
        }
        if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as i32) < 0i32 {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(0i16);
        }
        if ((GetBattlerSpriteBGPriorityRank(((&raw mut gBattleAnimTarget).cast::<u8>()).read()))
            as i32)
            == 1i32
        {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                .write(((((&raw mut gBattle_BG1_X).cast::<u16>()).read()) as i16));
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108884i32) as usize as *mut u16).cast::<u8>());
        } else {
            ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10))
                .write(((((&raw mut gBattle_BG2_X).cast::<u16>()).read()) as i16));
            (((&raw mut scanlineParams).cast::<u8>()).cast::<*mut u8>())
                .write(((67108888i32) as usize as *mut u16).cast::<u8>());
        }
        {
            i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read();
            'l2: loop {
                if !(((i) as i32)
                    <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read())
                        as i32)
                        .wrapping_add(64i32))
                {
                    break 'l2;
                }
                'l3: {
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
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_ExtrasensoryDistortion_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_ExtrasensoryDistortion_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut sineIndex: i16 = 0i16;
        let mut i: i16 = 0i16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                sineIndex = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read();
                i = ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read();
                'l2: loop {
                    if !(((i) as i32)
                        <= ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read())
                            as i32))
                    {
                        break 'l2;
                    }
                    let mut var2: i16 = ((crate::c::shr_i32(
                        ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>())
                            .wrapping_offset(((sineIndex) as i32) as isize))
                        .read()) as i32),
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(12)).read())
                            as u32),
                    )) as i16);
                    if ((var2) as i32) > 0i32 {
                        var2 = ((((var2) as i32).wrapping_add(
                            (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                                as i32)
                                & 3i32),
                        )) as i16);
                    } else {
                        if ((var2) as i32) < 0i32 {
                            var2 = ((((var2) as i32).wrapping_sub(
                                (((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1))
                                    .read()) as i32)
                                    & 3i32),
                            )) as i16);
                        }
                    }
                    ((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).cast::<u16>())
                        .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32)
                            .wrapping_add(((var2) as i32))) as u16),
                    );
                    (((((&raw mut gScanlineEffectRegBuffers).cast::<u8>()).wrapping_offset(1920))
                        .cast::<u16>())
                    .wrapping_offset(((i) as i32) as isize))
                    .write(
                        ((((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(10)).read())
                            as i32)
                            .wrapping_add(((var2) as i32))) as u16),
                    );
                    sineIndex = ((((sineIndex) as i32).wrapping_add(
                        ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(11)).read())
                            as i32),
                    )) as i16);
                    i = (i).wrapping_add(1);
                }
                if (({
                    let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                    let __t3 = ((__p2).read()).wrapping_add(1);
                    (__p2).write(__t3);
                    __t3
                }) as i32)
                    > 23i32
                {
                    let __p4 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                (((&raw mut gScanlineEffect).cast::<u8>()).wrapping_add(21)).write(3u8);
                let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                (__p5).write(((__p5).read()).wrapping_add(1));
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
pub unsafe extern "C" fn AnimTask_TransparentCloneGrowAndShrink(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut spriteId: i16 = 0i16;
        let mut matrixNum: i16 = 0i16;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        matrixNum = ((AllocOamMatrix()) as i16);
        if ((matrixNum) as i32) == 255i32 {
            DestroyAnimVisualTask(taskId);
            return;
        }
        spriteId = CloneBattlerSpriteWithBlend(
            (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
        );
        if ((spriteId) as i32) < 0i32 {
            FreeOamMatrix(((matrixNum) as u8));
            DestroyAnimVisualTask(taskId);
            return;
        }
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCallbackDummy));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
            0,
            2,
            (3u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
            1,
            5,
            ((matrixNum) as u32) as i32,
        );
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(44),
            7,
            1,
            (1u8) as i32,
        );
        let __p1 = (((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(67);
        (__p1).write(((__p1).read()).wrapping_add(1));
        SetSpriteRotScale(((spriteId) as u8), 256i16, 256i16, 0u16);
        CalcCenterToCornerVec(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                6,
                2,
                false,
            ) as u32) as u8),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(3),
                6,
                2,
                false,
            ) as u32) as u8),
            ((crate::c::bf_read(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(1),
                0,
                2,
                false,
            ) as u32) as u8),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).write(
            ((GetAnimBattlerSpriteId(
                (((((&raw mut gBattleAnimArgs).cast::<i16>()).cast::<i16>()).read()) as u8),
            )) as i16),
        );
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).write(matrixNum);
        ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).write(spriteId);
        ((task).cast::<Option<unsafe extern "C" fn(u8)>>())
            .write(Some(AnimTask_TransparentCloneGrowAndShrink_Step));
    }
}
pub(crate) unsafe extern "C" fn AnimTask_TransparentCloneGrowAndShrink_Step(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut task: *mut u8 =
            ((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40);
        'l1: {
            let __sw1 = (((((task).wrapping_add(8)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                let __p2 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p2).write((((((__p2).read()) as i32).wrapping_add(4i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                    (((256i32).wrapping_sub(
                        (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                                as i32) as isize,
                        ))
                        .read()) as i32)
                            >> 1),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
                    0u16,
                );
                SetBattlerSpriteYOffsetFromOtherYScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 48i32
                {
                    let __p3 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p3).write(((__p3).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p4 = (((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1);
                (__p4).write((((((__p4).read()) as i32).wrapping_sub(4i32)) as i16));
                ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).write(
                    (((256i32).wrapping_sub(
                        (((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                            ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read())
                                as i32) as isize,
                        ))
                        .read()) as i32)
                            >> 1),
                    )) as i16),
                );
                SetSpriteRotScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
                    ((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(2)).read(),
                    0u16,
                );
                SetBattlerSpriteYOffsetFromOtherYScale(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as u8),
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(13)).read()) as u8),
                );
                if ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
                    == 0i32
                {
                    let __p5 = ((task).wrapping_add(8)).cast::<i16>();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                DestroySpriteWithActiveSheet(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(15)).read()) as i32)
                        as isize
                        * 68,
                ));
                let __p6 = ((task).wrapping_add(8)).cast::<i16>();
                (__p6).write(((__p6).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 3i32 {
                FreeOamMatrix(
                    ((((((task).wrapping_add(8)).cast::<i16>()).wrapping_offset(14)).read()) as u8),
                );
                DestroyAnimVisualTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimPsychoBoost(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(32).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        0u8,
                    )) as i16),
                );
                ((sprite).wrapping_add(34).cast::<i16>()).write(
                    ((GetBattlerSpriteCoord(
                        ((&raw mut gBattleAnimAttacker).cast::<u8>()).read(),
                        1u8,
                    )) as i16),
                );
                if (IsContest()) != 0 {
                    let __p2 = (sprite).wrapping_add(34).cast::<i16>();
                    (__p2).write((((((__p2).read()) as i32).wrapping_add(12i32)) as i16));
                }
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(8i16);
                SetGpuReg(80u8, 16192u16);
                SetGpuReg(
                    82u8,
                    ((((16i32).wrapping_sub(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                    ) << 8)
                        | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32)) as u16),
                );
                let __p3 = ((sprite).wrapping_add(46)).cast::<i16>();
                (__p3).write(((__p3).read()).wrapping_add(1));
                break 'l1;
            }
            if __sw1 == 1i32 {
                if (crate::c::bf_read((sprite).wrapping_add(63), 5, 1, false) as u16) != 0 {
                    PlaySE12WithPanning(203u16, BattleAnimAdjustPanning((-64i8)));
                    ChangeSpriteAffineAnim(sprite, 1u8);
                    let __p4 = ((sprite).wrapping_add(46)).cast::<i16>();
                    (__p4).write(((__p4).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 2i32 {
                if (({
                    let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
                    let __t6 = (__p5).read();
                    (__p5).write(((__p5).read()).wrapping_add(1));
                    __t6
                }) as i32)
                    > 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(0i16);
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
                    (__p7).write(((__p7).read()).wrapping_sub(1));
                    SetGpuReg(
                        82u8,
                        ((((16i32).wrapping_sub(
                            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32),
                        ) << 8)
                            | ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                                .read()) as i32)) as u16),
                    );
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        == 0i32
                    {
                        let __p8 = ((sprite).wrapping_add(46)).cast::<i16>();
                        (__p8).write(((__p8).read()).wrapping_add(1));
                        crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
                    }
                }
                let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p9).write((((((__p9).read()) as i32).wrapping_add(896i32)) as i16));
                let __p10 = (sprite).wrapping_add(38).cast::<i16>();
                (__p10).write(
                    (((((__p10).read()) as i32).wrapping_sub(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            >> 8),
                    )) as i16),
                );
                let __p11 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p11).write((((((__p11).read()) as i32) & 255i32) as i16));
                break 'l1;
            }
            if __sw1 == 3i32 {
                SetGpuReg(80u8, 0u16);
                SetGpuReg(82u8, 0u16);
                DestroyAnimSprite(sprite);
                break 'l1;
            }
        }
    }
}
