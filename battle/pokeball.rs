//! Translated from `src/pokeball.c` by tools/rustport/c2rs.py, then reviewed.
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

// Data tables (translate with cdata.py): gBallSpriteSheets gBallSpritePalettes sBallOamData sBallAnimSeq3 sBallAnimSeq5 sBallAnimSeq4 sBallAnimSeq6 sBallAnimSeq0 sBallAnimSeq1 sBallAnimSeq2 sBallAnimSequences sAffineAnim_BallRotate_0 sAffineAnim_BallRotate_Right sAffineAnim_BallRotate_Left sAffineAnim_BallRotate_3 sAffineAnim_BallRotate_4 sAffineAnim_BallRotate gBallSpriteTemplates
#[allow(unused_imports)]
use crate::data::pokeball::*;

unsafe extern "C" {
    static mut gActiveBattler: u8;
    static mut gBattleSpritesDataPtr: u8;
    static mut gBattleTypeFlags: u8;
    static mut gBattlerPartyIndexes: u8;
    static mut gBattlerSpriteIds: u8;
    static mut gBattlerTarget: u8;
    static mut gDoingBattleAnim: u8;
    static mut gEnemyParty: u8;
    static mut gHealthboxSpriteIds: u8;
    static mut gMPlayInfo_BGM: u8;
    static mut gMain: u8;
    static mut gOpenPokeballGfx: u8;
    static mut gPlayerParty: u8;
    static mut gSineTable: u8;
    static mut gSprites: u8;
    static mut gTasks: u8;
    fn AnimTranslateLinear(a0: *mut u8) -> u8;
    fn AnimateBallOpenParticles(a0: u8, a1: u8, a2: u8, a3: u8, a4: u8) -> u8;
    fn AnimateSprite(a0: *mut u8);
    fn ChangeSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn Cos(a0: i16, a1: i16) -> i16;
    fn CreateInvisibleSpriteWithCallback(a0: Option<unsafe extern "C" fn(*mut u8)>) -> u8;
    fn CreateSprite(a0: *mut u8, a1: i16, a2: i16, a3: u8) -> u8;
    fn CreateTask(a0: Option<unsafe extern "C" fn(u8)>, a1: u8) -> u8;
    fn DestroySprite(a0: *mut u8);
    fn DestroySpriteAndFreeResources(a0: *mut u8);
    fn DestroyTask(a0: u8);
    fn DoMonFrontSpriteAnimation(a0: *mut u8, a1: u16, a2: u8, a3: u8);
    fn FreeOamMatrix(a0: u8);
    fn FreeSpriteOamMatrix(a0: *mut u8);
    fn FreeSpritePaletteByTag(a0: u16);
    fn FreeSpriteTilesByTag(a0: u16);
    fn GetBattlerAtPosition(a0: u8) -> u8;
    fn GetBattlerPosition(a0: u8) -> u8;
    fn GetBattlerSide(a0: u8) -> u8;
    fn GetBattlerSpriteCoord(a0: u8, a1: u8) -> u8;
    fn GetMonData2(a0: *mut u8, a1: i32) -> u32;
    fn GetSpriteTileStartByTag(a0: u16) -> u16;
    fn InitAnimArcTranslation(a0: *mut u8);
    fn IsBGMPlaying() -> u8;
    fn IsCryPlayingOrClearCrySongs() -> u8;
    fn IsDoubleBattle() -> u8;
    fn ItemIdToBallId(a0: u16) -> u8;
    fn LZDecompressVram(a0: *mut u32, a1: *mut u8);
    fn LaunchBallFadeMonTask(a0: u8, a1: u8, a2: u32, a3: u8) -> u8;
    fn LoadCompressedSpritePaletteUsingHeap(a0: *mut u8) -> u8;
    fn LoadCompressedSpriteSheetUsingHeap(a0: *mut u8) -> u8;
    fn PlayCry_ByMode(a0: u16, a1: i8, a2: u8);
    fn PlayCry_ReleaseDouble(a0: u16, a1: i8, a2: u8);
    fn PlaySE(a0: u16);
    fn ShouldPlayNormalMonCry(a0: *mut u8) -> u32;
    fn Sin(a0: i16, a1: i16) -> i16;
    fn SpriteCB_OpponentMonFromBall(a0: *mut u8);
    fn SpriteCB_PlayerMonFromBall(a0: *mut u8);
    fn SpriteCallbackDummy(a0: *mut u8);
    fn StartSpriteAffineAnim(a0: *mut u8, a1: u8);
    fn StartSpriteAnim(a0: *mut u8, a1: u8);
    fn StopCryAndClearCrySongs();
    fn TaskDummy(a0: u8);
    fn TranslateAnimHorizontalArc(a0: *mut u8) -> u8;
    fn m4aMPlayAllStop();
    fn m4aMPlayStop(a0: *mut u8);
    fn m4aMPlayVolumeControl(a0: *mut u8, a1: u16, a2: u16);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoPokeballSendOutAnimation(pan: i16, kindOfThrow: u8) -> u8 {
    unsafe {
        let mut pan = pan;
        let mut kindOfThrow = kindOfThrow;
        let mut taskId: u8 = 0u8;
        ((&raw mut gDoingBattleAnim).cast::<u8>()).write(1u8);
        crate::c::bf_write(
            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                .wrapping_add(4)
                .cast::<*mut u8>())
            .read())
            .wrapping_offset(
                ((((&raw mut gActiveBattler).cast::<u8>()).read()) as i32) as isize * 12,
            ))
            .wrapping_add(0),
            3,
            1,
            (1u8) as i32,
        );
        taskId = CreateTask(Some(Task_DoPokeballSendOutAnim), 5u8);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(pan);
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((kindOfThrow) as i16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((((&raw mut gActiveBattler).cast::<u8>()).read()) as i16));
        return 0u8;
    }
}
pub(crate) unsafe extern "C" fn Task_DoPokeballSendOutAnim(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut throwCaseId: u16 = 0u16;
        let mut battler: u8 = 0u8;
        let mut itemId: u16 = 0u16;
        let mut ballId: u16 = 0u16;
        let mut ballSpriteId: u8 = 0u8;
        let mut notSendOut: u8 = 0u8;
        if (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as i32)
            == 0i32
        {
            let __p1 = ((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>();
            (__p1).write(((__p1).read()).wrapping_add(1));
            return;
        }
        throwCaseId = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u16);
        battler = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        if ((GetBattlerSide(battler)) as i32) != 0i32 {
            itemId = ((GetMonData2(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                38i32,
            )) as u16);
        } else {
            itemId = ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                38i32,
            )) as u16);
        }
        ballId = ((ItemIdToBallId(itemId)) as u16);
        LoadBallGfx(((ballId) as u8));
        ballSpriteId = CreateSprite(
            (((&raw const gBallSpriteTemplates).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 24),
            32i16,
            80i16,
            29u8,
        );
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(128i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(0i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((throwCaseId) as i16));
        'l1: {
            let __sw2 = ((throwCaseId) as i32);
            let __matched = __sw2 == 255i32 || __sw2 == 254i32;
            if __sw2 == 255i32 {
                ((&raw mut gBattlerTarget).cast::<u8>()).write(battler);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .write(24i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .write(68i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_PlayerMonSendOut_1));
                break 'l1;
            }
            if __sw2 == 254i32 {
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(32)
                .cast::<i16>())
                .write(((GetBattlerSpriteCoord(battler, 0u8)) as i16));
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(34)
                .cast::<i16>())
                .write(
                    ((((GetBattlerSpriteCoord(battler, 1u8)) as i32).wrapping_add(24i32)) as i16),
                );
                ((&raw mut gBattlerTarget).cast::<u8>()).write(battler);
                (((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .write(0i16);
                ((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_OpponentMonSendOut));
                break 'l1;
            }
            if !__matched {
                ((&raw mut gBattlerTarget).cast::<u8>()).write(GetBattlerAtPosition(1u8));
                notSendOut = 1u8;
                break 'l1;
            }
        }
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i16));
        if !((notSendOut) != 0) {
            DestroyTask(taskId);
            return;
        }
        (((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .write(34i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(
            ((GetBattlerSpriteCoord(((&raw mut gBattlerTarget).cast::<u8>()).read(), 0u8)) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(
            ((((GetBattlerSpriteCoord(((&raw mut gBattlerTarget).cast::<u8>()).read(), 1u8))
                as i32)
                .wrapping_sub(16i32)) as i16),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write((-40i16));
        InitAnimArcTranslation(
            ((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((ballSpriteId) as i32) as isize * 68),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((ballSpriteId) as i32) as isize * 68))
        .wrapping_add(6)
        .cast::<u16>())
        .write(((taskId) as u16));
        ((((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((((&raw mut gBattlerTarget).cast::<u8>()).read()) as i16));
        ((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
            .cast::<Option<unsafe extern "C" fn(u8)>>())
        .write(Some(TaskDummy));
        PlaySE(61u16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (TranslateAnimHorizontalArc(sprite)) != 0 {
            let mut ballId: u16 = 0u16;
            let mut taskId: u8 = ((((sprite).wrapping_add(6).cast::<u16>()).read()) as u8);
            let mut opponentBattler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .read()) as u8);
            let mut noOfShakes: u8 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .read()) as u8);
            StartSpriteAnim(sprite, 1u8);
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
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
            ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ballId = ((ItemIdToBallId(GetBattlerPokeballItemId(opponentBattler))) as u16);
            AnimateBallOpenParticles(
                ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8),
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(5i32))
                    as u8),
                1u8,
                28u8,
                ((ballId) as u8),
            );
            (((sprite).wrapping_add(46)).cast::<i16>()).write(
                ((LaunchBallFadeMonTask(0u8, opponentBattler, 14u32, ((ballId) as u8))) as i16),
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                .write(((opponentBattler) as i16));
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                .write(((noOfShakes) as i16));
            DestroyTask(taskId);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BallThrow_ReachMon));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_ReachMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_BallThrow_StartShrinkMon));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_StartShrinkMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            let __t2 = ((__p1).read()).wrapping_add(1);
            (__p1).write(__t2);
            __t2
        }) as i32)
            == 10i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BallThrow_ShrinkMon));
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
                2u8,
            );
            AnimateSprite(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ),
            );
            ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_ShrinkMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 11i32
        {
            PlaySE(60u16);
        }
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            StartSpriteAnim(sprite, 2u8);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BallThrow_Close));
        } else {
            let __p2 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(96i32)) as i16));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                            as i32) as isize,
                    ))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_neg()
                    >> 8) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_Close(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
            (__p1).write(((__p1).read()).wrapping_add(1));
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                == 1i32
            {
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(32i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                let __p2 = (sprite).wrapping_add(34).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(((Cos(0i16, 32i16)) as i32))) as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Cos(
                        0i16,
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_BallThrow_FallToGround));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_FallToGround(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r5: u8 = 0u8;
        'l1: {
            let __sw1 = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                as i32)
                & 255i32);
            if __sw1 == 0i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                let __p2 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add((4i32).wrapping_add(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            >> 8),
                    ))) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    >= 64i32
                {
                    let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
                    (__p3).write((((((__p3).read()) as i32).wrapping_sub(10i32)) as i16));
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p4).write((((((__p4).read()) as i32).wrapping_add(257i32)) as i16));
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                        as i32)
                        >> 8)
                        == 4i32
                    {
                        r5 = 1u8;
                    }
                    'l2: {
                        let __sw5 = (((((((sprite).wrapping_add(46)).cast::<i16>())
                            .wrapping_offset(3))
                        .read()) as i32)
                            >> 8);
                        let __matched = __sw5 == 1i32 || __sw5 == 2i32 || __sw5 == 3i32;
                        if __sw5 == 1i32 {
                            PlaySE(56u16);
                            break 'l2;
                        }
                        if __sw5 == 2i32 {
                            PlaySE(57u16);
                            break 'l2;
                        }
                        if __sw5 == 3i32 {
                            PlaySE(58u16);
                            break 'l2;
                        }
                        if !__matched {
                            PlaySE(59u16);
                            break 'l2;
                        }
                    }
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                ((sprite).wrapping_add(38).cast::<i16>()).write(
                    ((((Cos(
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read(),
                    )) as i32)
                        .wrapping_neg()) as i16),
                );
                let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p6).write(
                    (((((__p6).read()) as i32).wrapping_sub((4i32).wrapping_add(
                        (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                            as i32)
                            >> 8),
                    ))) as i16),
                );
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    <= 0i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p7).write((((((__p7).read()) as i32) & 65280i32) as i16));
                }
                break 'l1;
            }
        }
        if (r5) != 0 {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            let __p8 = (sprite).wrapping_add(34).cast::<i16>();
            (__p8).write(
                (((((__p8).read()) as i32).wrapping_add(((Cos(64i16, 32i16)) as i32))) as i16),
            );
            ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
                == 0i32
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ReleaseMonFromBall));
            } else {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_BallThrow_StartShakes));
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(1i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_StartShakes(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read()) as i32)
            == 31i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
            crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
            StartSpriteAffineAnim(sprite, 1u8);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_BallThrow_Shake));
            PlaySE(23u16);
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_Shake(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        'l1: {
            let __sw1 = (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                as i32)
                & 255i32);
            let __matched =
                __sw1 == 0i32 || __sw1 == 2i32 || __sw1 == 1i32 || __sw1 == 3i32 || __sw1 == 4i32;
            if __sw1 == 0i32 || __sw1 == 2i32 {
                let __p2 = (sprite).wrapping_add(36).cast::<i16>();
                (__p2).write(
                    (((((__p2).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                let __p3 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p3).write(
                    (((((__p3).read()) as i32).wrapping_add(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                            as i32),
                    )) as i16),
                );
                crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    > 3i32)
                    || (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                        as i32)
                        < (-3i32))
                {
                    let __p4 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p4).write(((__p4).read()).wrapping_add(1));
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                let __p5 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p5).write(((__p5).read()).wrapping_add(1));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == 1i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)).wrapping_neg()) as i16));
                    let __p6 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p6).write(((__p6).read()).wrapping_add(1));
                    crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (0u8) as i32);
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        < 0i32
                    {
                        ChangeSpriteAffineAnim(sprite, 2u8);
                    } else {
                        ChangeSpriteAffineAnim(sprite, 1u8);
                    }
                } else {
                    crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                let __p7 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                (__p7).write((((((__p7).read()) as i32).wrapping_add(256i32)) as i16));
                if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).read())
                    as i32)
                    >> 8)
                    == ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ReleaseMonFromBall));
                } else {
                    if (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as i32)
                        == 4i32)
                        && ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3))
                            .read()) as i32)
                            >> 8)
                            == 3i32)
                    {
                        ((sprite)
                            .wrapping_add(28)
                            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                        .write(Some(SpriteCB_BallThrow_StartCaptureMon));
                        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                    } else {
                        let __p8 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                        (__p8).write(((__p8).read()).wrapping_add(1));
                        crate::c::bf_write((sprite).wrapping_add(44), 7, 1, (1u8) as i32);
                    }
                }
                break 'l1;
            }
            if __sw1 == 4i32 || !__matched {
                let __p9 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
                (__p9).write(((__p9).read()).wrapping_add(1));
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                    as i32)
                    == 31i32
                {
                    ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
                    let __p10 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3);
                    (__p10).write((((((__p10).read()) as i32) & 65280i32) as i16));
                    StartSpriteAffineAnim(sprite, 3u8);
                    if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                        as i32)
                        < 0i32
                    {
                        StartSpriteAffineAnim(sprite, 2u8);
                    } else {
                        StartSpriteAffineAnim(sprite, 1u8);
                    }
                    PlaySE(23u16);
                }
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn Task_PlayCryWhenReleasedFromBall(taskId: u8) {
    unsafe {
        let mut taskId = taskId;
        let mut wantedCry: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(2))
        .read()) as u8);
        let mut pan: i8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(1))
        .read()) as i8);
        let mut species: u16 = (((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .read()) as u16);
        let mut battler: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(3))
        .read()) as u8);
        let mut monSpriteId: u8 = ((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(4))
        .read()) as u8);
        let mut mon: *mut u8 = ((((((((((((&raw mut gTasks).cast::<u8>())
            .wrapping_offset(((taskId) as i32) as isize * 40))
        .wrapping_add(8))
        .cast::<i16>())
        .wrapping_offset(5))
        .read()) as i32)
            << 16)
            | (((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .read()) as u16) as i32)) as u32) as usize as *mut u8);
        'l1: {
            let __sw1 = ((((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .read()) as i32);
            let __matched = __sw1 == 0i32
                || __sw1 == 1i32
                || __sw1 == 2i32
                || __sw1 == 20i32
                || __sw1 == 3i32
                || __sw1 == 30i32
                || __sw1 == 31i32
                || __sw1 == 32i32;
            let mut __fall = false;
            if __sw1 == 0i32 || !__matched {
                __fall = true;
                if (crate::c::bf_read(
                    (((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((monSpriteId) as i32) as isize * 68))
                    .wrapping_add(63),
                    5,
                    1,
                    false,
                ) as u16)
                    != 0
                {
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15))
                    .write(((((wantedCry) as i32).wrapping_add(1i32)) as i16));
                }
                break 'l1;
            }
            if __sw1 == 1i32 {
                __fall = true;
                if ShouldPlayNormalMonCry(mon) == 1u32 {
                    PlayCry_ByMode(species, pan, 0u8);
                } else {
                    PlayCry_ByMode(species, pan, 11u8);
                }
                crate::c::bf_write(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(1),
                    6,
                    1,
                    (0u8) as i32,
                );
                DestroyTask(taskId);
                break 'l1;
            }
            if __sw1 == 2i32 {
                __fall = true;
                StopCryAndClearCrySongs();
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(3i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .write(20i16);
                break 'l1;
            }
            if __sw1 == 20i32 {
                __fall = true;
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    == 0i32
                {
                    if ShouldPlayNormalMonCry(mon) == 1u32 {
                        PlayCry_ReleaseDouble(species, pan, 1u8);
                    } else {
                        PlayCry_ReleaseDouble(species, pan, 12u8);
                    }
                    crate::c::bf_write(
                        ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .wrapping_add(4)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_offset(((battler) as i32) as isize * 12))
                        .wrapping_add(1),
                        6,
                        1,
                        (0u8) as i32,
                    );
                    DestroyTask(taskId);
                } else {
                    let __p2 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p2).write(((__p2).read()).wrapping_sub(1));
                }
                break 'l1;
            }
            if __sw1 == 3i32 {
                __fall = true;
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .write(6i16);
                ((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15))
                .write(30i16);
                break 'l1;
            }
            if __sw1 == 30i32 {
                __fall = true;
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    != 0i32
                {
                    let __p3 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p3).write(((__p3).read()).wrapping_sub(1));
                    break 'l1;
                }
                let __p4 = (((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(15);
                (__p4).write(((__p4).read()).wrapping_add(1));
            }
            if __fall || __sw1 == 31i32 {
                __fall = true;
                if !((IsCryPlayingOrClearCrySongs()) != 0) {
                    StopCryAndClearCrySongs();
                    ((((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10))
                    .write(3i16);
                    let __p5 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(15);
                    (__p5).write(((__p5).read()).wrapping_add(1));
                }
                break 'l1;
            }
            if __sw1 == 32i32 {
                __fall = true;
                if ((((((((&raw mut gTasks).cast::<u8>())
                    .wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
                .cast::<i16>())
                .wrapping_offset(10))
                .read()) as i32)
                    != 0i32
                {
                    let __p6 = (((((&raw mut gTasks).cast::<u8>())
                        .wrapping_offset(((taskId) as i32) as isize * 40))
                    .wrapping_add(8))
                    .cast::<i16>())
                    .wrapping_offset(10);
                    (__p6).write(((__p6).read()).wrapping_sub(1));
                    break 'l1;
                }
                if ShouldPlayNormalMonCry(mon) == 1u32 {
                    PlayCry_ReleaseDouble(species, pan, 0u8);
                } else {
                    PlayCry_ReleaseDouble(species, pan, 11u8);
                }
                crate::c::bf_write(
                    ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(4)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_offset(((battler) as i32) as isize * 12))
                    .wrapping_add(1),
                    6,
                    1,
                    (0u8) as i32,
                );
                DestroyTask(taskId);
                break 'l1;
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReleaseMonFromBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8);
        let mut ballId: u32 = 0u32;
        StartSpriteAnim(sprite, 1u8);
        ballId = ((ItemIdToBallId(GetBattlerPokeballItemId(battler))) as u32);
        AnimateBallOpenParticles(
            ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8),
            ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(5i32))
                as u8),
            1u8,
            28u8,
            ((ballId) as u8),
        );
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            ((LaunchBallFadeMonTask(
                1u8,
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8),
                14u32,
                ((ballId) as u8),
            )) as i16),
        );
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(HandleBallAnimEnd));
        if (crate::c::bf_read(
            ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
            1,
            1,
            false,
        ) as u8)
            != 0
        {
            let mut mon: *mut u8 = core::ptr::null_mut();
            let mut species: u16 = 0u16;
            let mut pan: i8 = 0i8;
            let mut wantedCryCase: u16 = 0u16;
            let mut taskId: u8 = 0u8;
            if ((GetBattlerSide(battler)) as i32) != 0i32 {
                mon = ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                );
                pan = 25i8;
            } else {
                mon = ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                );
                pan = (-25i8);
            }
            species = ((GetMonData2(mon, 11i32)) as u16);
            if (((((battler) as i32) == ((GetBattlerAtPosition(0u8)) as i32))
                || (((battler) as i32) == ((GetBattlerAtPosition(1u8)) as i32)))
                && ((IsDoubleBattle()) != 0))
                && ((crate::c::bf_read(
                    (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0)
            {
                if ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 64u32) != 0)
                    && ((((&raw mut gBattleTypeFlags).cast::<u32>()).read() & 2u32) != 0)
                {
                    if (IsBGMPlaying()) != 0 {
                        m4aMPlayStop((&raw mut gMPlayInfo_BGM).cast::<u8>());
                    }
                } else {
                    m4aMPlayVolumeControl((&raw mut gMPlayInfo_BGM).cast::<u8>(), 65535u16, 128u16);
                }
            }
            if (!((IsDoubleBattle()) != 0))
                || (!((crate::c::bf_read(
                    (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0))
            {
                wantedCryCase = 0u16;
            } else {
                if (((battler) as i32) == ((GetBattlerAtPosition(0u8)) as i32))
                    || (((battler) as i32) == ((GetBattlerAtPosition(1u8)) as i32))
                {
                    wantedCryCase = 1u16;
                } else {
                    wantedCryCase = 2u16;
                }
            }
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(1),
                6,
                1,
                (1u8) as i32,
            );
            taskId = CreateTask(Some(Task_PlayCryWhenReleasedFromBall), 3u8);
            (((((&raw mut gTasks).cast::<u8>()).wrapping_offset(((taskId) as i32) as isize * 40))
                .wrapping_add(8))
            .cast::<i16>())
            .write(((species) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(((pan) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(2))
            .write(((wantedCryCase) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(3))
            .write(((battler) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(4))
            .write(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i16),
            );
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(5))
            .write(((((mon) as usize as u32) >> 16) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(6))
            .write((((mon) as usize as u32) as i16));
            ((((((&raw mut gTasks).cast::<u8>())
                .wrapping_offset(((taskId) as i32) as isize * 40))
            .wrapping_add(8))
            .cast::<i16>())
            .wrapping_offset(15))
            .write(0i16);
        }
        StartSpriteAffineAnim(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
            1u8,
        );
        if ((GetBattlerSide(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8),
        )) as i32)
            == 1i32
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_OpponentMonFromBall));
        } else {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_PlayerMonFromBall));
        }
        AnimateSprite(
            ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32) as isize,
                ))
                .read()) as i32) as isize
                    * 68,
            ),
        );
        ((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                    as isize,
            ))
            .read()) as i32) as isize
                * 68,
        ))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(4096i16);
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_StartCaptureMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        crate::c::bf_write((sprite).wrapping_add(44), 6, 1, (1u8) as i32);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_BallThrow_CaptureMon));
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(3)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(0i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
    }
}
pub(crate) unsafe extern "C" fn HandleBallAnimEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut affineAnimEnded: u8 = 0u8;
        let mut battler: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8);
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
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ),
                0u8,
            );
            affineAnimEnded = 1u8;
        } else {
            let __p1 = (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p1).write((((((__p1).read()) as i32).wrapping_sub(288i32)) as i16));
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                    (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 68,
                ))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    >> 8) as i16),
            );
        }
        if ((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
            && ((affineAnimEnded) != 0)
        {
            let mut i: i32 = 0i32;
            let mut doneBattlers: i32 = 0i32;
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                (((((&raw mut gBattlerSpriteIds).cast::<u8>())
                    .wrapping_offset(((battler) as i32) as isize))
                .read()) as i32) as isize
                    * 68,
            ))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
            crate::c::bf_write(
                ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                    .wrapping_add(4)
                    .cast::<*mut u8>())
                .read())
                .wrapping_offset(((battler) as i32) as isize * 12))
                .wrapping_add(0),
                3,
                1,
                (0u8) as i32,
            );
            FreeSpriteOamMatrix(sprite);
            DestroySprite(sprite);
            {
                doneBattlers = 0i32;
                i = 0i32;
                'l1: loop {
                    if !(i < 4i32) {
                        break 'l1;
                    }
                    'l2: {
                        if ((crate::c::bf_read(
                            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset((i) as isize * 12))
                            .wrapping_add(0),
                            3,
                            1,
                            false,
                        ) as u8) as i32)
                            == 0i32
                        {
                            doneBattlers = (doneBattlers).wrapping_add(1);
                        }
                    }
                    i = (i).wrapping_add(1);
                }
            }
            if doneBattlers == 4i32 {
                {
                    i = 0i32;
                    'l3: loop {
                        if !(i < 12i32) {
                            break 'l3;
                        }
                        'l4: {
                            FreeBallGfx(((i) as u8));
                        }
                        i = (i).wrapping_add(1);
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_BallThrow_CaptureMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut battler: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8);
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
            == 40i32
        {
            return;
        } else {
            if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read()) as i32)
                == 95i32
            {
                ((&raw mut gDoingBattleAnim).cast::<u8>()).write(0u8);
                m4aMPlayAllStop();
                PlaySE(371u16);
            } else {
                if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as i32)
                    == 315i32
                {
                    FreeOamMatrix(
                        ((crate::c::bf_read(
                            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                                (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                    ((((((sprite).wrapping_add(46)).cast::<i16>())
                                        .wrapping_offset(6))
                                    .read()) as i32) as isize,
                                ))
                                .read()) as i32) as isize
                                    * 68,
                            ))
                            .wrapping_add(3),
                            1,
                            5,
                            false,
                        ) as u32) as u8),
                    );
                    DestroySprite(
                        ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
                            (((((&raw mut gBattlerSpriteIds).cast::<u8>()).wrapping_offset(
                                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6))
                                    .read()) as i32) as isize,
                            ))
                            .read()) as i32) as isize
                                * 68,
                        ),
                    );
                    DestroySpriteAndFreeResources(sprite);
                    if (crate::c::bf_read(
                        ((&raw mut gMain).cast::<u8>()).wrapping_add(1081),
                        1,
                        1,
                        false,
                    ) as u8)
                        != 0
                    {
                        crate::c::bf_write(
                            ((((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                                .wrapping_add(4)
                                .cast::<*mut u8>())
                            .read())
                            .wrapping_offset(((battler) as i32) as isize * 12))
                            .wrapping_add(0),
                            3,
                            1,
                            (0u8) as i32,
                        );
                    }
                }
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerMonSendOut_1(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        (((sprite).wrapping_add(46)).cast::<i16>()).write(25i16);
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
            ((GetBattlerSpriteCoord(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8),
                2u8,
            )) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).write(
            ((((GetBattlerSpriteCoord(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u8),
                3u8,
            )) as i32)
                .wrapping_add(24i32)) as i16),
        );
        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write((-30i16));
        ((sprite).wrapping_add(6).cast::<u16>()).write(
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as u16),
        );
        InitAnimArcTranslation(sprite);
        ((sprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_PlayerMonSendOut_2));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PlayerMonSendOut_2(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r6: u32 = 0u32;
        let mut r7: u32 = 0u32;
        if (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            >> 8)
            & 255i32)
            >= 35i32)
            && (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                as i32)
                >> 8)
                & 255i32)
                < 80i32)
        {
            let mut r4: i16 = 0i16;
            if (((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32) & 65280i32) == 0i32 {
                r6 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    & 1i32) as u32);
                r7 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    & 1i32) as u32);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                            as i32),
                        3i32,
                    ) & (-2i32)) as u32)
                        | r6) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((crate::c::div_i32(
                        ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                            as i32),
                        3i32,
                    ) & (-2i32)) as u32)
                        | r7) as i16),
                );
                StartSpriteAffineAnim(sprite, 4u8);
            }
            r4 = (((sprite).wrapping_add(46)).cast::<i16>()).read();
            AnimTranslateLinear(sprite);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write(
                (((((__p1).read()) as i32).wrapping_add(crate::c::div_i32(
                    ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32),
                    3i32,
                ))) as i16),
            );
            let __p2 = (sprite).wrapping_add(38).cast::<i16>();
            (__p2).write(
                (((((__p2).read()) as i32).wrapping_add(
                    ((Sin(
                        (((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7))
                            .read()) as i32)
                            >> 8)
                            & 255i32) as i16),
                        ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read(),
                    )) as i32),
                )) as i16),
            );
            let __p3 = (sprite).wrapping_add(6).cast::<u16>();
            (__p3).write((((((__p3).read()) as i32).wrapping_add(256i32)) as u16));
            if crate::c::rem_i32(
                (((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32) >> 8),
                3i32,
            ) != 0i32
            {
                (((sprite).wrapping_add(46)).cast::<i16>()).write(r4);
            } else {
                (((sprite).wrapping_add(46)).cast::<i16>())
                    .write(((((r4) as i32).wrapping_sub(1i32)) as i16));
            }
            if ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                as i32)
                >> 8)
                & 255i32)
                >= 80i32
            {
                r6 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                    as i32)
                    & 1i32) as u32);
                r7 = ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                    as i32)
                    & 1i32) as u32);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                    ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read())
                        as i32)
                        .wrapping_mul(3i32)
                        & (-2i32)) as u32)
                        | r6) as i16),
                );
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).write(
                    ((((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read())
                        as i32)
                        .wrapping_mul(3i32)
                        & (-2i32)) as u32)
                        | r7) as i16),
                );
            }
        } else {
            if (TranslateAnimHorizontalArc(sprite)) != 0 {
                let __p4 = (sprite).wrapping_add(32).cast::<i16>();
                (__p4).write(
                    (((((__p4).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                let __p5 = (sprite).wrapping_add(34).cast::<i16>();
                (__p5).write(
                    (((((__p5).read()) as i32)
                        .wrapping_add(((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32)))
                        as i16),
                );
                ((sprite).wrapping_add(38).cast::<i16>()).write(0i16);
                ((sprite).wrapping_add(36).cast::<i16>()).write(0i16);
                ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).write(
                    ((((((sprite).wrapping_add(6).cast::<u16>()).read()) as i32) & 255i32) as i16),
                );
                (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
                if (((IsDoubleBattle()) != 0)
                    && ((crate::c::bf_read(
                        (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                            .wrapping_add(8)
                            .cast::<*mut u8>())
                        .read())
                        .wrapping_add(9),
                        0,
                        1,
                        false,
                    ) as u8)
                        != 0))
                    && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                        as i32)
                        == ((GetBattlerAtPosition(2u8)) as i32))
                {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ReleaseMon2FromBall));
                } else {
                    ((sprite)
                        .wrapping_add(28)
                        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                    .write(Some(SpriteCB_ReleaseMonFromBall));
                }
                StartSpriteAffineAnim(sprite, 0u8);
            }
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReleaseMon2FromBall(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (({
            let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
            let __t2 = (__p1).read();
            (__p1).write(((__p1).read()).wrapping_add(1));
            __t2
        }) as i32)
            > 24i32
        {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ReleaseMonFromBall));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_OpponentMonSendOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = ((sprite).wrapping_add(46)).cast::<i16>();
        (__p1).write(((__p1).read()).wrapping_add(1));
        if (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32) > 15i32 {
            (((sprite).wrapping_add(46)).cast::<i16>()).write(0i16);
            if (((IsDoubleBattle()) != 0)
                && ((crate::c::bf_read(
                    (((((&raw mut gBattleSpritesDataPtr).cast::<*mut u8>()).read())
                        .wrapping_add(8)
                        .cast::<*mut u8>())
                    .read())
                    .wrapping_add(9),
                    0,
                    1,
                    false,
                ) as u8)
                    != 0))
                && (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read())
                    as i32)
                    == ((GetBattlerAtPosition(3u8)) as i32))
            {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ReleaseMon2FromBall));
            } else {
                ((sprite)
                    .wrapping_add(28)
                    .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
                .write(Some(SpriteCB_ReleaseMonFromBall));
            }
        }
    }
}
pub(crate) unsafe extern "C" fn AnimateBallOpenParticlesForPokeball(
    x: u8,
    y: u8,
    kindOfStars: u8,
    subpriority: u8,
) -> u8 {
    unsafe {
        let mut x = x;
        let mut y = y;
        let mut kindOfStars = kindOfStars;
        let mut subpriority = subpriority;
        return AnimateBallOpenParticles(x, y, kindOfStars, subpriority, 0u8);
    }
}
pub(crate) unsafe extern "C" fn LaunchBallFadeMonTaskForPokeball(
    unFadeLater: u8,
    spritePalNum: u8,
    selectedPalettes: u32,
) -> u8 {
    unsafe {
        let mut unFadeLater = unFadeLater;
        let mut spritePalNum = spritePalNum;
        let mut selectedPalettes = selectedPalettes;
        return LaunchBallFadeMonTask(unFadeLater, spritePalNum, selectedPalettes, 0u8);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreatePokeballSpriteToReleaseMon(
    monSpriteId: u8,
    monPalNum: u8,
    x: u8,
    y: u8,
    oamPriority: u8,
    subpriority: u8,
    delay: u8,
    fadePalettes: u32,
    species: u16,
) {
    unsafe {
        let mut monSpriteId = monSpriteId;
        let mut monPalNum = monPalNum;
        let mut x = x;
        let mut y = y;
        let mut oamPriority = oamPriority;
        let mut subpriority = subpriority;
        let mut delay = delay;
        let mut fadePalettes = fadePalettes;
        let mut species = species;
        let mut spriteId: u8 = 0u8;
        LoadCompressedSpriteSheetUsingHeap(
            ((&raw const gBallSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadCompressedSpritePaletteUsingHeap(
            ((&raw const gBallSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        spriteId = CreateSprite(
            ((&raw const gBallSpriteTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((x) as i16),
            ((y) as i16),
            subpriority,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((monSpriteId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(5))
        .write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .read(),
        );
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(6))
        .write(
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .read(),
        );
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .write(((x) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .write(((y) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(7))
        .write(((species) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((delay) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((monPalNum) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((fadePalettes) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((fadePalettes >> 16) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((oamPriority) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_PokeballReleaseMon));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(62),
            2,
            1,
            (1u16) as i32,
        );
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_PokeballReleaseMon(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            let mut subpriority: u8 = 0u8;
            let mut spriteId: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
            let mut monPalNum: u8 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
            let mut selectedPalettes: u32 = (((((((((sprite).wrapping_add(46)).cast::<i16>())
                .wrapping_offset(3))
            .read()) as u16) as i32)
                | ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as u16) as i32)
                    << 16)) as u32);
            if ((((sprite).wrapping_add(67)).read()) as i32) != 0i32 {
                subpriority =
                    ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32)) as u8);
            } else {
                subpriority = 0u8;
            }
            StartSpriteAnim(sprite, 1u8);
            AnimateBallOpenParticlesForPokeball(
                ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8),
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(5i32))
                    as u8),
                ((crate::c::bf_read((sprite).wrapping_add(5), 2, 2, false) as u16) as u8),
                subpriority,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((LaunchBallFadeMonTaskForPokeball(1u8, monPalNum, selectedPalettes)) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_ReleasedMonFlyOut));
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (0u16) as i32,
            );
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
                1u8,
            );
            AnimateSprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((spriteId) as i32) as isize * 68),
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(4096i16);
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).write(0i16);
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_ReleasedMonFlyOut(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut emergeAnimFinished: u8 = 0u8;
        let mut atFinalPosition: u8 = 0u8;
        let mut monSpriteId: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        let mut x: u16 = 0u16;
        let mut y: u16 = 0u16;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            crate::c::bf_write((sprite).wrapping_add(62), 2, 1, (1u16) as i32);
        }
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((monSpriteId) as i32) as isize * 68),
                0u8,
            );
            emergeAnimFinished = 1u8;
        }
        x = (((crate::c::div_i32(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
                .wrapping_sub(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
            .wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            ),
            128i32,
        ))
        .wrapping_add(((((sprite).wrapping_add(32).cast::<i16>()).read()) as i32)))
            as u16);
        y = (((crate::c::div_i32(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read()) as i32)
                .wrapping_sub(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
            .wrapping_mul(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32),
            ),
            128i32,
        ))
        .wrapping_add(((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32)))
            as u16);
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(32)
        .cast::<i16>())
        .write(((x) as i16));
        ((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((monSpriteId) as i32) as isize * 68))
        .wrapping_add(34)
        .cast::<i16>())
        .write(((y) as i16));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read()) as i32)
            < 128i32
        {
            let mut sine: i16 = (((crate::c::div_i32(
                ((((((&raw mut gSineTable).cast::<i16>()).cast::<i16>()).wrapping_offset(
                    (((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7)).read())
                        as u8) as i32) as isize,
                ))
                .read()) as i32),
                8i32,
            ))
            .wrapping_neg()) as i16);
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(7);
            (__p1).write((((((__p1).read()) as i32).wrapping_add(4i32)) as i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(sine);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(sine);
        } else {
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(32)
            .cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read());
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(34)
            .cast::<i16>())
            .write(((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(6)).read());
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(36)
            .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(0i16);
            atFinalPosition = 1u8;
        }
        if (((crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0)
            && ((emergeAnimFinished) != 0))
            && ((atFinalPosition) != 0)
        {
            if ((((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(7))
            .read()) as i32)
                == 412i32
            {
                DoMonFrontSpriteAnimation(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((monSpriteId) as i32) as isize * 68),
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((monSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read()) as u16),
                    1u8,
                    0u8,
                );
            } else {
                DoMonFrontSpriteAnimation(
                    ((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((monSpriteId) as i32) as isize * 68),
                    ((((((((&raw mut gSprites).cast::<u8>())
                        .wrapping_offset(((monSpriteId) as i32) as isize * 68))
                    .wrapping_add(46))
                    .cast::<i16>())
                    .wrapping_offset(7))
                    .read()) as u16),
                    0u8,
                    0u8,
                );
            }
            DestroySpriteAndFreeResources(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn CreateTradePokeballSprite(
    monSpriteId: u8,
    monPalNum: u8,
    x: u8,
    y: u8,
    oamPriority: u8,
    subPriority: u8,
    delay: u8,
    fadePalettes: u32,
) -> u8 {
    unsafe {
        let mut monSpriteId = monSpriteId;
        let mut monPalNum = monPalNum;
        let mut x = x;
        let mut y = y;
        let mut oamPriority = oamPriority;
        let mut subPriority = subPriority;
        let mut delay = delay;
        let mut fadePalettes = fadePalettes;
        let mut spriteId: u8 = 0u8;
        LoadCompressedSpriteSheetUsingHeap(
            ((&raw const gBallSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        LoadCompressedSpritePaletteUsingHeap(
            ((&raw const gBallSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>(),
        );
        spriteId = CreateSprite(
            ((&raw const gBallSpriteTemplates).cast::<u8>().cast_mut()).cast::<u8>(),
            ((x) as i16),
            ((y) as i16),
            subPriority,
        );
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(((monSpriteId) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(((delay) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(2))
        .write(((monPalNum) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(3))
        .write(((fadePalettes) as i16));
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(4))
        .write(((fadePalettes >> 16) as i16));
        crate::c::bf_write(
            (((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
                .wrapping_add(5),
            2,
            2,
            ((oamPriority) as u16) as i32,
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_TradePokeball));
        return spriteId;
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TradePokeball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 0i32
        {
            let mut subpriority: u8 = 0u8;
            let mut monSpriteId: u8 = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
            let mut monPalNum: u8 =
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as u8);
            let mut selectedPalettes: u32 = (((((((((sprite).wrapping_add(46)).cast::<i16>())
                .wrapping_offset(3))
            .read()) as u16) as i32)
                | ((((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(4)).read())
                    as u16) as i32)
                    << 16)) as u32);
            if ((((sprite).wrapping_add(67)).read()) as i32) != 0i32 {
                subpriority =
                    ((((((sprite).wrapping_add(67)).read()) as i32).wrapping_sub(1i32)) as u8);
            } else {
                subpriority = 0u8;
            }
            StartSpriteAnim(sprite, 1u8);
            AnimateBallOpenParticlesForPokeball(
                ((((sprite).wrapping_add(32).cast::<i16>()).read()) as u8),
                ((((((sprite).wrapping_add(34).cast::<i16>()).read()) as i32).wrapping_sub(5i32))
                    as u8),
                ((crate::c::bf_read((sprite).wrapping_add(5), 2, 2, false) as u16) as u8),
                subpriority,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((LaunchBallFadeMonTaskForPokeball(1u8, monPalNum, selectedPalettes)) as i16),
            );
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_TradePokeballSendOff));
            StartSpriteAffineAnim(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((monSpriteId) as i32) as isize * 68),
                2u8,
            );
            AnimateSprite(
                ((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((monSpriteId) as i32) as isize * 68),
            );
            ((((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1))
            .write(0i16);
        } else {
            let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
            (__p1).write(((__p1).read()).wrapping_sub(1));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TradePokeballSendOff(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut monSpriteId: u8 = 0u8;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read()) as i32)
            == 11i32
        {
            PlaySE(60u16);
        }
        monSpriteId = (((((sprite).wrapping_add(46)).cast::<i16>()).read()) as u8);
        if (crate::c::bf_read(
            (((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(63),
            5,
            1,
            false,
        ) as u16)
            != 0
        {
            StartSpriteAnim(sprite, 2u8);
            crate::c::bf_write(
                (((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((monSpriteId) as i32) as isize * 68))
                .wrapping_add(62),
                2,
                1,
                (1u16) as i32,
            );
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_TradePokeballEnd));
        } else {
            let __p2 = (((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(46))
            .cast::<i16>())
            .wrapping_offset(1);
            (__p2).write((((((__p2).read()) as i32).wrapping_add(96i32)) as i16));
            ((((&raw mut gSprites).cast::<u8>())
                .wrapping_offset(((monSpriteId) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
            .write(
                ((((((((((&raw mut gSprites).cast::<u8>())
                    .wrapping_offset(((monSpriteId) as i32) as isize * 68))
                .wrapping_add(46))
                .cast::<i16>())
                .wrapping_offset(1))
                .read()) as i32)
                    .wrapping_neg()
                    >> 8) as i16),
            );
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_TradePokeballEnd(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        if (crate::c::bf_read((sprite).wrapping_add(63), 4, 1, false) as u16) != 0 {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
pub(crate) unsafe extern "C" fn DestroySpriteAndFreeResources_Ball(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        DestroySpriteAndFreeResources(sprite);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn StartHealthboxSlideIn(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut healthboxSprite: *mut u8 = ((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i32) as isize
                * 68,
        );
        (((healthboxSprite).wrapping_add(46)).cast::<i16>()).write(5i16);
        ((((healthboxSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
        ((healthboxSprite).wrapping_add(36).cast::<i16>()).write(115i16);
        ((healthboxSprite).wrapping_add(38).cast::<i16>()).write(0i16);
        ((healthboxSprite)
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_HealthboxSlideIn));
        if ((GetBattlerSide(battler)) as i32) != 0i32 {
            (((healthboxSprite).wrapping_add(46)).cast::<i16>()).write(
                (((((((healthboxSprite).wrapping_add(46)).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((((healthboxSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(
                ((((((((healthboxSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1))
                    .read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((healthboxSprite).wrapping_add(36).cast::<i16>()).write(
                ((((((healthboxSprite).wrapping_add(36).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
            ((healthboxSprite).wrapping_add(38).cast::<i16>()).write(
                ((((((healthboxSprite).wrapping_add(38).cast::<i16>()).read()) as i32)
                    .wrapping_neg()) as i16),
            );
        }
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((healthboxSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32) as isize
                * 68,
        ))
        .wrapping_add(28)
        .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .read())
        .unwrap_unchecked()(((&raw mut gSprites).cast::<u8>()).wrapping_offset(
            ((((((healthboxSprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(5)).read())
                as i32) as isize
                * 68,
        ));
        if ((GetBattlerPosition(battler)) as i32) == 2i32 {
            ((healthboxSprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_HealthboxSlideInDelayed));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HealthboxSlideInDelayed(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32)
            == 20i32
        {
            ((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).write(0i16);
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCB_HealthboxSlideIn));
        }
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HealthboxSlideIn(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let __p1 = (sprite).wrapping_add(36).cast::<i16>();
        (__p1).write(
            (((((__p1).read()) as i32)
                .wrapping_sub((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32)))
                as i16),
        );
        let __p2 = (sprite).wrapping_add(38).cast::<i16>();
        (__p2).write(
            (((((__p2).read()) as i32).wrapping_sub(
                ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as i32),
            )) as i16),
        );
        if (((((sprite).wrapping_add(36).cast::<i16>()).read()) as i32) == 0i32)
            && (((((sprite).wrapping_add(38).cast::<i16>()).read()) as i32) == 0i32)
        {
            ((sprite)
                .wrapping_add(28)
                .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
            .write(Some(SpriteCallbackDummy));
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DoHitAnimHealthboxEffect(battler: u8) {
    unsafe {
        let mut battler = battler;
        let mut spriteId: u8 = 0u8;
        spriteId = CreateInvisibleSpriteWithCallback(Some(SpriteCB_HitAnimHealthoxEffect));
        (((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(46))
        .cast::<i16>())
        .write(1i16);
        ((((((&raw mut gSprites).cast::<u8>())
            .wrapping_offset(((spriteId) as i32) as isize * 68))
        .wrapping_add(46))
        .cast::<i16>())
        .wrapping_offset(1))
        .write(
            (((((&raw mut gHealthboxSpriteIds).cast::<u8>())
                .wrapping_offset(((battler) as i32) as isize))
            .read()) as i16),
        );
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((spriteId) as i32) as isize * 68))
            .wrapping_add(28)
            .cast::<Option<unsafe extern "C" fn(*mut u8)>>())
        .write(Some(SpriteCB_HitAnimHealthoxEffect));
    }
}
pub(crate) unsafe extern "C" fn SpriteCB_HitAnimHealthoxEffect(sprite: *mut u8) {
    unsafe {
        let mut sprite = sprite;
        let mut r1: u8 =
            ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(1)).read()) as u8);
        ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((r1) as i32) as isize * 68))
            .wrapping_add(38)
            .cast::<i16>())
        .write((((sprite).wrapping_add(46)).cast::<i16>()).read());
        (((sprite).wrapping_add(46)).cast::<i16>()).write(
            (((((((sprite).wrapping_add(46)).cast::<i16>()).read()) as i32).wrapping_neg()) as i16),
        );
        let __p1 = (((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2);
        (__p1).write(((__p1).read()).wrapping_add(1));
        if ((((((sprite).wrapping_add(46)).cast::<i16>()).wrapping_offset(2)).read()) as i32)
            == 21i32
        {
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((r1) as i32) as isize * 68))
                .wrapping_add(36)
                .cast::<i16>())
            .write(0i16);
            ((((&raw mut gSprites).cast::<u8>()).wrapping_offset(((r1) as i32) as isize * 68))
                .wrapping_add(38)
                .cast::<i16>())
            .write(0i16);
            DestroySprite(sprite);
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn LoadBallGfx(ballId: u8) {
    unsafe {
        let mut ballId = ballId;
        let mut var: u16 = 0u16;
        if ((GetSpriteTileStartByTag(
            (((((&raw const gBallSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        )) as i32)
            == 65535i32
        {
            LoadCompressedSpriteSheetUsingHeap(
                (((&raw const gBallSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((ballId) as i32) as isize * 8),
            );
            LoadCompressedSpritePaletteUsingHeap(
                (((&raw const gBallSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>())
                    .wrapping_offset(((ballId) as i32) as isize * 8),
            );
        }
        'l1: {
            let __sw1 = ((ballId) as i32);
            let __matched = __sw1 == 6i32 || __sw1 == 10i32 || __sw1 == 11i32;
            if __sw1 == 6i32 || __sw1 == 10i32 || __sw1 == 11i32 {
                break 'l1;
            }
            if !__matched {
                var = GetSpriteTileStartByTag(
                    (((((&raw const gBallSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                        .wrapping_offset(((ballId) as i32) as isize * 8))
                    .wrapping_add(6)
                    .cast::<u16>())
                    .read(),
                );
                LZDecompressVram(
                    ((&raw mut gOpenPokeballGfx).cast::<u32>()).cast::<u32>(),
                    (((100729088i32).wrapping_add(((var) as i32).wrapping_mul(32i32))) as usize
                        as *mut u8),
                );
                break 'l1;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn FreeBallGfx(ballId: u8) {
    unsafe {
        let mut ballId = ballId;
        FreeSpriteTilesByTag(
            (((((&raw const gBallSpriteSheets).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 8))
            .wrapping_add(6)
            .cast::<u16>())
            .read(),
        );
        FreeSpritePaletteByTag(
            (((((&raw const gBallSpritePalettes).cast::<u8>().cast_mut()).cast::<u8>())
                .wrapping_offset(((ballId) as i32) as isize * 8))
            .wrapping_add(4)
            .cast::<u16>())
            .read(),
        );
    }
}
pub(crate) unsafe extern "C" fn GetBattlerPokeballItemId(battler: u8) -> u16 {
    unsafe {
        let mut battler = battler;
        if ((GetBattlerSide(battler)) as i32) == 0i32 {
            return ((GetMonData2(
                ((&raw mut gPlayerParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                38i32,
            )) as u16);
        } else {
            return ((GetMonData2(
                ((&raw mut gEnemyParty).cast::<u8>()).wrapping_offset(
                    ((((((&raw mut gBattlerPartyIndexes).cast::<u16>()).cast::<u16>())
                        .wrapping_offset(((battler) as i32) as isize))
                    .read()) as i32) as isize
                        * 100,
                ),
                38i32,
            )) as u16);
        }
        #[allow(unreachable_code)]
        {
            return 0u16;
        }
    }
}
